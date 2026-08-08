//! The repository this crate is running inside.
//!
//! Everything the lifecycle writes to a caller is either neutral or branded.
//! Neutral output — the event stream, the gate projection — is identified by
//! `schema_version` and reads the same in every repository, which is why the
//! recorded evidence under `reports/skill-evidence/` needed no migration when
//! this crate left its first host. Branded output is the receipts a command
//! prints, the recovery instructions it gives, and the skill packages it
//! installs, all of which name a repository and a binary that this crate cannot
//! know.
//!
//! One fact here is not about output at all. The lifecycle refuses to let a
//! workflow review its own skill package, and deciding that means knowing where
//! *the running binary's* repository keeps its skills — not where the `--root`
//! under audit keeps its. A crate compiled into someone else's dependency graph
//! cannot infer that from its own source location, so the host states it.
//!
//! [`Host`] is where a repository states all of it, once.

use std::path::{Path, PathBuf};

/// One repository's identity to this crate.
///
/// The fields are public and there is no constructor, so adding one here is a
/// compile error at every host rather than a silent default. There are no
/// invariants to protect: each field is a name this crate repeats back, and a
/// wrong one is visible in the first line of output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    /// Prefixes every schema identity in a receipt, e.g. `playbench` in
    /// `playbench.skill-evidence.record.v1`.
    pub namespace: String,
    /// The binary an operator actually types. Usually, but not necessarily,
    /// the same word as `namespace`.
    pub command: String,
    /// The Cargo package that provides `command`, as `cargo run -p` wants it.
    /// Installed skill packages quote it in their runnable examples.
    pub cargo_package: String,
    /// Where *this* repository keeps its own skill packages.
    ///
    /// Resolve it however the host resolves it — typically from the host's own
    /// `CARGO_MANIFEST_DIR` at compile time. It is deliberately not derived
    /// from the audited `--root`:
    /// `lifecycle_self_target_checks_canonical_paths_instead_of_basenames`
    /// pins that a same-named package in the audited tree is a different
    /// package, not a self-target.
    pub skills_directory: PathBuf,
}

impl Host {
    /// The schema identity on a `hash` receipt.
    #[must_use]
    pub fn hash_schema(&self) -> String {
        format!("{}.skill-evidence.hash.v1", self.namespace)
    }

    /// The schema identity on a `record` receipt.
    #[must_use]
    pub fn record_schema(&self) -> String {
        format!("{}.skill-evidence.record.v1", self.namespace)
    }

    /// The schema identity on a method-gap research inventory.
    #[must_use]
    pub fn method_gap_inventory_schema(&self) -> String {
        format!("{}.method-gap-research-status.inventory/v1", self.namespace)
    }

    /// The package this repository would be running when it drives
    /// `operator_skill_name`, used to refuse a workflow that targets itself.
    #[must_use]
    pub fn operator_skill(&self, operator_skill_name: &str) -> PathBuf {
        self.skills_directory.join(operator_skill_name)
    }

    /// Where this repository keeps its own skill packages.
    #[must_use]
    pub fn skills_directory(&self) -> &Path {
        &self.skills_directory
    }

    /// Renders the sentence that tells an operator how to finish an operation
    /// that stopped part-way.
    ///
    /// This is the half of an [`Error`](crate::Error) the library deliberately
    /// does not write: it knows the action, and only the host knows what to call
    /// the binary that performs it.
    #[must_use]
    pub fn recovery_instruction(&self, recovery: Recovery) -> String {
        match recovery {
            Recovery::RederiveGate => format!(
                "Run `{} skills evidence derive --target <skill-dir>`.",
                self.command
            ),
        }
    }
}

/// The operator action that finishes an operation this crate could not finish.
///
/// An [`Error`](crate::Error) carrying one of these is not merely describing a
/// failure — it is naming work the caller must still do, and the caller's own
/// output surface is responsible for saying so in its own words. See
/// [`Host::recovery_instruction`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Recovery {
    /// The evidence event was durably appended, but the gate projection beside
    /// it was not replaced. The stream is intact and the projection is stale
    /// until it is derived again.
    RederiveGate,
}
