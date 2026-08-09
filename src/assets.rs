//! The skill packages and schemas this crate ships, and the command that writes
//! them into a repository.
//!
//! The lifecycle machinery is useless without the four skill packages that
//! drive it, and those packages were duplicated by hand across repositories for
//! as long as the machinery was. They travel here instead: compiled into the
//! binary, rendered with the host's own names, and written out on request.
//!
//! Two tokens vary per repository — the binary an operator types and the Cargo
//! package that provides it — so the shipped copies carry `{{command}}`,
//! `{{cargo_package}}`, and `{{namespace}}` where those appear. Nothing else in
//! a package is host-specific; a package that needed a third token would be a
//! sign the package, not the template, has the wrong boundary.

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::{Error, Host, refusal, unsafe_failure};

/// One shipped file: where it goes, relative to a repository root, and what it
/// says before the host's names are substituted in.
struct Asset {
    relative_path: &'static str,
    template: &'static str,
}

/// Pairs an installed skill file with the shipped copy that fills it.
///
/// The two live at different paths — `assets/skills/…` here, `.claude/skills/…`
/// in a host — and writing both by hand is how one of them drifts. Each macro
/// derives the source from the destination, so the list below names each file
/// once.
macro_rules! skill_asset {
    ($path:literal) => {
        Asset {
            relative_path: concat!(".claude/skills/", $path),
            template: include_str!(concat!("../assets/skills/", $path)),
        }
    };
}

/// The same, for the versioned contracts under `schemas/skill-evidence/`.
macro_rules! schema_asset {
    ($path:literal) => {
        Asset {
            relative_path: concat!("schemas/skill-evidence/", $path),
            template: include_str!(concat!("../assets/schemas/", $path)),
        }
    };
}

/// Everything this crate installs.
///
/// Listed rather than walked: a build that silently shipped one file fewer
/// because a directory listing changed would be indistinguishable from a
/// working one until an operator hit the gap.
const ASSETS: &[Asset] = &[
    skill_asset!("skill-evidence-capture/SKILL.md"),
    skill_asset!("skill-evidence-capture/agents/openai.yaml"),
    skill_asset!("skill-evolution/SKILL.md"),
    skill_asset!("skill-evolution/agents/openai.yaml"),
    skill_asset!("skill-evolution/references/authorized-review.md"),
    skill_asset!("skill-evolution-status/SKILL.md"),
    skill_asset!("skill-evolution-status/agents/openai.yaml"),
    skill_asset!("method-gap-research-status/SKILL.md"),
    skill_asset!("method-gap-research-status/agents/openai.yaml"),
    skill_asset!("method-gap-research-status/references/lineage-reconstruction.md"),
    skill_asset!("method-gap-research-status/references/selection-rules.md"),
    schema_asset!("event.v1.schema.json"),
    schema_asset!("gate-status.v1.schema.json"),
];

/// Files from packages this crate once installed and can now withdraw.
///
/// Retired assets are deliberately separate from [`ASSETS`]: retaining their
/// last-shipped templates must never make them reachable from `install`.
const RETIRED_ASSETS: &[Asset] = &[
    Asset {
        relative_path: ".claude/skills/legacy-skill-decontamination/SKILL.md",
        template: include_str!("../assets/retired-skills/legacy-skill-decontamination/SKILL.md"),
    },
    Asset {
        relative_path: ".claude/skills/legacy-skill-decontamination/agents/openai.yaml",
        template: include_str!(
            "../assets/retired-skills/legacy-skill-decontamination/agents/openai.yaml"
        ),
    },
    Asset {
        relative_path: ".claude/skills/legacy-skill-decontamination/references/eligible-run.md",
        template: include_str!(
            "../assets/retired-skills/legacy-skill-decontamination/references/eligible-run.md"
        ),
    },
];

/// The skill packages [`ASSETS`] installs, in the order they first appear.
///
/// Derived from the asset list rather than restated, so a package added there
/// gets its discovery link without a second edit here.
fn skill_package_names() -> Vec<&'static str> {
    package_names(ASSETS)
}

/// The retired package names, derived from their retained files rather than
/// maintained as a second registry.
fn retired_skill_package_names() -> Vec<&'static str> {
    package_names(RETIRED_ASSETS)
}

fn package_names(assets: &[Asset]) -> Vec<&'static str> {
    let mut names = Vec::new();
    for asset in assets {
        let Some(rest) = asset.relative_path.strip_prefix(".claude/skills/") else {
            continue;
        };
        let name = rest.split('/').next().expect("a split always yields one");
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

/// What one `install` did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InstallReceipt {
    pub schema_version: u8,
    /// Files this run created.
    pub written: Vec<String>,
    /// Files already byte-identical to what this crate ships.
    pub unchanged: Vec<String>,
    /// Files that exist and differ. Empty unless `force` was set, because a
    /// run that found any of these without `force` refuses instead.
    pub replaced: Vec<String>,
    /// Discovery links under `.agents/skills/` this run created. Empty on a
    /// platform without symlinks, where discovery falls back to
    /// `.claude/skills/` directly.
    pub linked: Vec<String>,
    /// Retired packages still present in the repository. `install` reports
    /// them but never removes them.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub orphaned_packages: Vec<String>,
}

/// One path `withdraw` deliberately retained and why it was outside the
/// operation's authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedPath {
    pub path: String,
    pub reason: String,
}

/// What one [`withdraw`] did.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WithdrawalReceipt {
    pub schema_version: u8,
    /// Retired files removed because they still matched the last shipped copy.
    pub removed_files: Vec<String>,
    /// Removed files that differed and therefore required `force`.
    pub forced_files: Vec<String>,
    /// Empty directories removed below a retired package, including the
    /// package directory itself.
    pub removed_directories: Vec<String>,
    /// Correct discovery links removed from `.agents/skills/`.
    pub removed_links: Vec<String>,
    /// Paths left in place because the crate cannot prove it wrote them.
    pub retained: Vec<RetainedPath>,
}

/// Writes the shipped packages and schemas into `root`.
///
/// Refuses rather than overwrites when a file exists and differs, so a
/// repository that has edited its own copy finds out instead of losing it.
/// `force` converts those into replacements.
pub fn install(root: &Path, host: &Host, force: bool) -> Result<InstallReceipt, Error> {
    let rendered: Vec<(PathBuf, String, &str)> = ASSETS
        .iter()
        .map(|asset| {
            (
                root.join(asset.relative_path),
                render(asset.template, host),
                asset.relative_path,
            )
        })
        .collect();

    let mut conflicts = Vec::new();
    let mut written = Vec::new();
    let mut unchanged = Vec::new();
    let mut replaced = Vec::new();

    for (destination, contents, relative_path) in &rendered {
        match read_existing(destination)? {
            None => written.push((*relative_path).to_owned()),
            Some(existing) if &existing == contents => unchanged.push((*relative_path).to_owned()),
            Some(_) => {
                replaced.push((*relative_path).to_owned());
                conflicts.push((*relative_path).to_owned());
            }
        }
    }

    if !conflicts.is_empty() && !force {
        return Err(refusal(format!(
            "{} installed file(s) differ from the ones this crate ships: {}. Re-run with --force to replace them.",
            conflicts.len(),
            conflicts.join(", ")
        )));
    }

    // Every decision is made before the first byte lands, so a refusal above
    // leaves the tree exactly as it found it.
    for (destination, contents, relative_path) in &rendered {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                unsafe_failure(format!(
                    "Could not create {} for {relative_path}: {error}",
                    parent.display()
                ))
            })?;
        }
        fs::write(destination, contents)
            .map_err(|error| unsafe_failure(format!("Could not write {relative_path}: {error}")))?;
    }

    let linked = install_discovery_links(root)?;
    let mut orphaned_packages = Vec::new();
    for name in retired_skill_package_names() {
        let package = root.join(".claude/skills").join(name);
        let link = root.join(".agents/skills").join(name);
        if path_exists_without_following(&package)? || path_exists_without_following(&link)? {
            orphaned_packages.push(name.to_owned());
        }
    }

    Ok(InstallReceipt {
        schema_version: 1,
        written,
        unchanged,
        replaced,
        linked,
        orphaned_packages,
    })
}

/// Removes every retired package this crate can still identify.
///
/// Every file and discovery-link decision is made before the first removal.
/// A differing retired file therefore refuses atomically unless `force` is
/// set. `force` does not widen the operation to paths the crate never shipped.
pub fn withdraw(root: &Path, host: &Host, force: bool) -> Result<WithdrawalReceipt, Error> {
    let expected_paths: HashSet<PathBuf> = RETIRED_ASSETS
        .iter()
        .map(|asset| PathBuf::from(asset.relative_path))
        .collect();
    let mut removed_files = Vec::new();
    let mut forced_files = Vec::new();
    let mut retained = Vec::new();
    let mut conflicts = Vec::new();
    let mut package_directories = Vec::new();

    let mut safe_packages = HashSet::new();
    if safe_directory_boundary(root, ".claude/skills", &mut retained)? {
        for name in retired_skill_package_names() {
            if inspect_foreign_package_entries(
                root,
                name,
                &expected_paths,
                &mut retained,
                &mut package_directories,
            )? {
                safe_packages.insert(name);
            }
        }
    }

    for asset in RETIRED_ASSETS {
        let package_name = asset
            .relative_path
            .strip_prefix(".claude/skills/")
            .and_then(|path| path.split('/').next())
            .expect("every retired asset belongs to a retired skill package");
        if !safe_packages.contains(package_name) {
            continue;
        }
        let destination = root.join(asset.relative_path);
        let metadata = match fs::symlink_metadata(&destination) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(unsafe_failure(format!(
                    "Could not inspect {} before withdrawal: {error}",
                    asset.relative_path
                )));
            }
        };
        if !metadata.file_type().is_file() {
            retained.push(RetainedPath {
                path: asset.relative_path.to_owned(),
                reason: "expected retired path is not a regular file shipped by this crate"
                    .to_owned(),
            });
            continue;
        }

        let existing = fs::read(&destination).map_err(|error| {
            unsafe_failure(format!(
                "Could not read {} to compare it before withdrawal: {error}",
                asset.relative_path
            ))
        })?;
        if existing == render(asset.template, host).as_bytes() {
            removed_files.push(asset.relative_path.to_owned());
        } else if force {
            removed_files.push(asset.relative_path.to_owned());
            forced_files.push(asset.relative_path.to_owned());
        } else {
            conflicts.push(asset.relative_path.to_owned());
        }
    }

    let mut links_to_remove = Vec::new();
    if safe_directory_boundary(root, ".agents/skills", &mut retained)? {
        for name in retired_skill_package_names() {
            let relative = PathBuf::from(".agents/skills").join(name);
            let link = root.join(&relative);
            let expected = PathBuf::from("../../.claude/skills").join(name);
            match fs::read_link(&link) {
                Ok(destination) if destination == expected => links_to_remove.push(relative),
                Ok(destination) => retained.push(RetainedPath {
                    path: portable_path(&relative),
                    reason: format!(
                        "discovery link points to {} instead of {}",
                        destination.display(),
                        expected.display()
                    ),
                }),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => match fs::symlink_metadata(&link) {
                    Ok(_) => retained.push(RetainedPath {
                        path: portable_path(&relative),
                        reason: "discovery path is not a link created by this crate".to_owned(),
                    }),
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(error) => {
                        return Err(unsafe_failure(format!(
                            "Could not inspect {} before withdrawal: {error}",
                            portable_path(&relative)
                        )));
                    }
                },
            }
        }
    }

    retained.sort_by(|left, right| (&left.path, &left.reason).cmp(&(&right.path, &right.reason)));
    if !conflicts.is_empty() {
        return Err(refusal(format!(
            "{} retired installed file(s) differ from the last copies this crate shipped: {}. Re-run with --force to withdraw them.",
            conflicts.len(),
            conflicts.join(", ")
        )));
    }

    for relative in &removed_files {
        fs::remove_file(root.join(relative))
            .map_err(|error| unsafe_failure(format!("Could not remove {relative}: {error}")))?;
    }

    let mut removed_links = Vec::new();
    for relative in links_to_remove {
        fs::remove_file(root.join(&relative)).map_err(|error| {
            unsafe_failure(format!(
                "Could not remove discovery link {}: {error}",
                portable_path(&relative)
            ))
        })?;
        removed_links.push(portable_path(&relative));
    }

    let removed_directories = remove_empty_package_directories(root, package_directories)?;

    Ok(WithdrawalReceipt {
        schema_version: 1,
        removed_files,
        forced_files,
        removed_directories,
        removed_links,
        retained,
    })
}

fn inspect_foreign_package_entries(
    root: &Path,
    package_name: &str,
    expected_paths: &HashSet<PathBuf>,
    retained: &mut Vec<RetainedPath>,
    package_directories: &mut Vec<PathBuf>,
) -> Result<bool, Error> {
    let package = root.join(".claude/skills").join(package_name);
    let metadata = match fs::symlink_metadata(&package) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(unsafe_failure(format!(
                "Could not inspect retired package {}: {error}",
                portable_relative(root, &package)
            )));
        }
    };
    if !metadata.file_type().is_dir() {
        retained.push(RetainedPath {
            path: portable_relative(root, &package),
            reason: "retired package path is not a directory created by this crate".to_owned(),
        });
        return Ok(false);
    }

    let mut pending = vec![package];
    while let Some(directory) = pending.pop() {
        package_directories.push(directory.clone());
        let entries = fs::read_dir(&directory).map_err(|error| {
            unsafe_failure(format!(
                "Could not inspect retired package directory {}: {error}",
                portable_relative(root, &directory)
            ))
        })?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                unsafe_failure(format!(
                    "Could not inspect an entry under {}: {error}",
                    portable_relative(root, &directory)
                ))
            })?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                unsafe_failure(format!(
                    "Could not inspect {} before withdrawal: {error}",
                    portable_relative(root, &path)
                ))
            })?;
            if metadata.file_type().is_dir() {
                pending.push(path);
                continue;
            }

            let relative = path
                .strip_prefix(root)
                .expect("a traversed retired-package path stays below root");
            if !expected_paths.contains(relative) {
                retained.push(RetainedPath {
                    path: portable_path(relative),
                    reason: "path was not shipped by this crate".to_owned(),
                });
            }
        }
    }
    Ok(true)
}

fn safe_directory_boundary(
    root: &Path,
    relative: &str,
    retained: &mut Vec<RetainedPath>,
) -> Result<bool, Error> {
    let path = root.join(relative);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_dir() => Ok(true),
        Ok(_) => {
            retained.push(RetainedPath {
                path: relative.to_owned(),
                reason: "withdrawal boundary is not a directory created by this crate".to_owned(),
            });
            Ok(false)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(unsafe_failure(format!(
            "Could not inspect withdrawal boundary {relative}: {error}"
        ))),
    }
}

fn remove_empty_package_directories(
    root: &Path,
    mut directories: Vec<PathBuf>,
) -> Result<Vec<String>, Error> {
    directories.sort_by(|left, right| {
        right
            .components()
            .count()
            .cmp(&left.components().count())
            .then_with(|| left.cmp(right))
    });

    let mut removed = Vec::new();
    for directory in directories {
        match fs::remove_dir(&directory) {
            Ok(()) => removed.push(portable_relative(root, &directory)),
            Err(error) if error.kind() == std::io::ErrorKind::DirectoryNotEmpty => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(unsafe_failure(format!(
                    "Could not remove empty retired-package directory {}: {error}",
                    portable_relative(root, &directory)
                )));
            }
        }
    }
    Ok(removed)
}

fn portable_relative(root: &Path, path: &Path) -> String {
    portable_path(
        path.strip_prefix(root)
            .expect("an asset operation reports only paths below root"),
    )
}

fn portable_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn path_exists_without_following(path: &Path) -> Result<bool, Error> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(unsafe_failure(format!(
            "Could not inspect {}: {error}",
            path.display()
        ))),
    }
}

/// Points `.agents/skills/<name>` at each installed package.
///
/// A package is only half-installed without its link: `.claude/skills/` is
/// where the file lives and `.agents/skills/` is where a non-Claude client
/// looks, and the lifecycle itself searches both (see `skill_key`). Returns the
/// links this call created; an existing correct link is left alone and not
/// reported.
fn install_discovery_links(root: &Path) -> Result<Vec<String>, Error> {
    let mut created = Vec::new();
    for name in skill_package_names() {
        let link = root.join(".agents/skills").join(name);
        // Relative, and relative to the link's own directory: an absolute
        // target would break the moment the repository is cloned elsewhere.
        let destination = PathBuf::from("../../.claude/skills").join(name);

        match fs::read_link(&link) {
            Ok(existing) if existing == destination => continue,
            Ok(_) => {
                fs::remove_file(&link).map_err(|error| {
                    unsafe_failure(format!(
                        "Could not replace the discovery link {}: {error}",
                        link.display()
                    ))
                })?;
            }
            // Not a link: either absent, or something else entirely. Creating
            // the link below reports the difference either way.
            Err(_) => {}
        }

        if let Some(parent) = link.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                unsafe_failure(format!(
                    "Could not create {} for discovery links: {error}",
                    parent.display()
                ))
            })?;
        }
        match symlink(&destination, &link) {
            Ok(()) => created.push(format!(".agents/skills/{name}")),
            Err(error) if error.kind() == std::io::ErrorKind::Unsupported => return Ok(Vec::new()),
            Err(error) => {
                return Err(unsafe_failure(format!(
                    "Could not create the discovery link {}: {error}",
                    link.display()
                )));
            }
        }
    }
    Ok(created)
}

#[cfg(unix)]
fn symlink(destination: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(destination, link)
}

/// Windows needs a privilege for directory symlinks that an ordinary user does
/// not have, so discovery there is left to `.claude/skills/` alone rather than
/// failing an otherwise complete install.
#[cfg(not(unix))]
fn symlink(_destination: &Path, _link: &Path) -> std::io::Result<()> {
    Err(std::io::Error::from(std::io::ErrorKind::Unsupported))
}

/// Substitutes the host's names into one shipped file.
fn render(template: &str, host: &Host) -> String {
    template
        .replace("{{cargo_package}}", &host.cargo_package)
        .replace("{{command}}", &host.command)
        .replace("{{namespace}}", &host.namespace)
}

/// The file's current contents, or `None` when it is not there yet.
fn read_existing(path: &Path) -> Result<Option<String>, Error> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(Some(contents)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(unsafe_failure(format!(
            "Could not read {} to compare it: {error}",
            path.display()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn host() -> Host {
        Host {
            namespace: "demo".to_owned(),
            command: "demo".to_owned(),
            cargo_package: "demo-cli".to_owned(),
            skills_directory: PathBuf::from("/nonexistent/.claude/skills"),
        }
    }

    fn markdown_link_targets(contents: &str) -> Vec<&str> {
        let mut targets = Vec::new();
        let mut remainder = contents;
        while let Some(start) = remainder.find("](") {
            remainder = &remainder[start + 2..];
            let Some(end) = remainder.find(')') else {
                break;
            };
            let target = &remainder[..end];
            if !target.is_empty()
                && !target.starts_with('#')
                && !target.contains("://")
                && !target.starts_with("mailto:")
            {
                targets.push(target);
            }
            remainder = &remainder[end + 1..];
        }
        targets
    }

    fn backticked_package_relative_paths(contents: &str) -> Vec<&str> {
        contents
            .split('`')
            .enumerate()
            .filter_map(|(index, span)| {
                (index % 2 == 1 && (span.starts_with("references/") || span.starts_with("../")))
                    .then_some(span)
            })
            .collect()
    }

    fn resolve_package_relative_path(source: &str, target: &str) -> Option<String> {
        let mut components: Vec<&str> = source.split('/').collect();
        components.pop();

        for component in target.split('/') {
            match component {
                "" | "." => {}
                ".." => {
                    components.pop()?;
                }
                component => components.push(component),
            }
        }

        Some(components.join("/"))
    }

    #[test]
    fn every_shipped_markdown_reference_resolves_within_shipped_assets() {
        let shipped_paths: HashSet<&str> = ASSETS.iter().map(|asset| asset.relative_path).collect();
        let evolution = ASSETS
            .iter()
            .find(|asset| asset.relative_path == ".claude/skills/skill-evolution/SKILL.md")
            .expect("the shipped Skill Evolution package");
        assert_eq!(
            backticked_package_relative_paths(evolution.template),
            vec!["references/authorized-review.md"]
        );
        let authorized_review = ASSETS
            .iter()
            .find(|asset| {
                asset.relative_path
                    == ".claude/skills/skill-evolution/references/authorized-review.md"
            })
            .expect("the shipped authorized-review reference");
        assert!(backticked_package_relative_paths(authorized_review.template).is_empty());

        let mut unresolved = Vec::new();
        for asset in ASSETS
            .iter()
            .filter(|asset| asset.relative_path.ends_with(".md"))
        {
            let targets = markdown_link_targets(asset.template)
                .into_iter()
                .chain(backticked_package_relative_paths(asset.template));
            for target in targets {
                match resolve_package_relative_path(asset.relative_path, target) {
                    Some(path) if shipped_paths.contains(path.as_str()) => {}
                    _ => unresolved.push(format!("{} -> {target}", asset.relative_path)),
                }
            }
        }

        assert_eq!(unresolved, Vec::<String>::new());
    }

    #[test]
    fn every_shipped_file_renders_without_leaving_a_placeholder() {
        for asset in ASSETS {
            let rendered = render(asset.template, &host());
            assert!(
                !rendered.contains("{{"),
                "{} still holds a placeholder after rendering",
                asset.relative_path
            );
        }
    }

    #[test]
    fn install_writes_every_asset_and_reports_them() {
        let root = tempfile::tempdir().expect("temporary repository root");
        let receipt = install(root.path(), &host(), false).expect("install into an empty root");
        assert_eq!(receipt.written.len(), ASSETS.len());
        assert!(receipt.unchanged.is_empty());
        assert!(receipt.replaced.is_empty());
        for asset in ASSETS {
            assert!(
                root.path().join(asset.relative_path).is_file(),
                "{} was not written",
                asset.relative_path
            );
        }
    }

    #[test]
    fn every_skill_package_gets_a_relative_discovery_link() {
        let root = tempfile::tempdir().expect("temporary repository root");
        let receipt = install(root.path(), &host(), false).expect("install into an empty root");
        let names = skill_package_names();
        assert_eq!(names.len(), 4, "the four packages this crate ships");

        #[cfg(unix)]
        {
            assert_eq!(receipt.linked.len(), names.len());
            for name in names {
                assert_eq!(
                    fs::read_link(root.path().join(".agents/skills").join(name))
                        .expect("read discovery link"),
                    PathBuf::from("../../.claude/skills").join(name)
                );
            }
        }
        #[cfg(not(unix))]
        assert!(receipt.linked.is_empty());
    }

    #[test]
    fn reinstalling_leaves_a_correct_discovery_link_alone() {
        let root = tempfile::tempdir().expect("temporary repository root");
        install(root.path(), &host(), false).expect("first install");
        let receipt = install(root.path(), &host(), false).expect("second install");
        assert!(
            receipt.linked.is_empty(),
            "a link that already points where it should is not re-created"
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_discovery_link_pointing_somewhere_else_is_repaired() {
        let root = tempfile::tempdir().expect("temporary repository root");
        install(root.path(), &host(), false).expect("first install");
        let link = root.path().join(".agents/skills/skill-evolution");
        fs::remove_file(&link).expect("remove the correct link");
        std::os::unix::fs::symlink("../../elsewhere", &link).expect("point it elsewhere");

        let receipt = install(root.path(), &host(), false).expect("second install");
        assert_eq!(
            receipt.linked,
            vec![".agents/skills/skill-evolution".to_owned()]
        );
        assert_eq!(
            fs::read_link(&link).expect("read repaired link"),
            PathBuf::from("../../.claude/skills/skill-evolution")
        );
    }

    #[test]
    fn reinstalling_the_same_tree_reports_every_file_unchanged() {
        let root = tempfile::tempdir().expect("temporary repository root");
        install(root.path(), &host(), false).expect("first install");
        let receipt = install(root.path(), &host(), false).expect("second install");
        assert_eq!(receipt.unchanged.len(), ASSETS.len());
        assert!(receipt.written.is_empty());
    }

    #[test]
    fn install_refuses_an_edited_file_and_leaves_it_alone() {
        let root = tempfile::tempdir().expect("temporary repository root");
        install(root.path(), &host(), false).expect("first install");
        let edited = root.path().join(".claude/skills/skill-evolution/SKILL.md");
        fs::write(&edited, "Locally edited.\n").expect("edit an installed file");

        let error = install(root.path(), &host(), false).expect_err("refuse the edited file");
        assert_eq!(error.class(), crate::ErrorClass::Refusal);
        assert!(error.to_string().contains("--force"));
        assert_eq!(
            fs::read_to_string(&edited).expect("read back"),
            "Locally edited.\n",
            "a refusal must not have written anything"
        );
    }

    #[test]
    fn force_replaces_an_edited_file_and_names_it() {
        let root = tempfile::tempdir().expect("temporary repository root");
        install(root.path(), &host(), false).expect("first install");
        let edited = root.path().join(".claude/skills/skill-evolution/SKILL.md");
        fs::write(&edited, "Locally edited.\n").expect("edit an installed file");

        let receipt = install(root.path(), &host(), true).expect("force past the edit");
        assert_eq!(
            receipt.replaced,
            vec![".claude/skills/skill-evolution/SKILL.md".to_owned()]
        );
        assert_ne!(
            fs::read_to_string(&edited).expect("read back"),
            "Locally edited.\n"
        );
    }

    #[test]
    fn a_host_installs_its_own_names_not_the_authoring_repositorys() {
        let root = tempfile::tempdir().expect("temporary repository root");
        install(root.path(), &host(), false).expect("install");
        let skill = fs::read_to_string(
            root.path()
                .join(".claude/skills/skill-evidence-capture/SKILL.md"),
        )
        .expect("read the installed skill");
        assert!(skill.contains("`demo skills evidence`"));
        assert!(skill.contains("cargo run --locked -p demo-cli"));
        assert!(!skill.contains("playbench"));
    }
}
