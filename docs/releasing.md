# Releasing, and bringing consumers forward

Procedure, not law. The rules this procedure serves are in
[`principles/consumer-contract.md`](principles/consumer-contract.md); when the two disagree, the
principles win and this document is what gets fixed.

## What is actually at stake

Recorded evidence in the consuming repositories, as of 2026-08-09. A dated snapshot, not a figure
to maintain — it drifts every time a consumer records anything, and the order of magnitude is the
part that matters.

| Consumer | Evidence stores | Recorded events | How it depends |
|---|---|---|---|
| `playbench` | 32 | 1038 | `skill-evidence = "0.1"`; host command `playbench` |
| `mundifold` | 16 | 118 | `skill-evidence = "0.1.2"`; host command `mundifold` |
| `what-we-bring-home` | 7 | 18 | `skill-evidence = "0.1"`; host command `cargo run --locked -p developer-tools --` |

Over a thousand append-only events that no release can regenerate. That is the number to have in
mind when deciding whether a change is really additive.

## 1. Decide the version from the surfaces you touched

| You changed | Version consequence |
|---|---|
| Private helpers, tests, docs, fixtures (additively) | patch |
| Added a public API item, a subcommand, an optional flag, a new installed package | patch or minor; minor if a consumer would want to opt in deliberately |
| Removed or renamed anything public, changed a flag or exit code, retired or renamed an installed package | **minor while `0.x`, major after `1.0.0`** |
| Anything at all about the shape of a recorded event | Stop. Go to §2 before choosing a number. |

While the crate is `0.x`, Cargo treats the minor position as the breaking one: `0.1.0` ↔ `0.1.1`
compatible, `0.1.0` ↔ `0.2.0` not.

Do not cut `1.0.0` as a milestone. Cut it when the recorded-event shape has stopped moving.

## 2. If you touched the recorded-event shape

Every one of these must hold before the release is allowed to exist:

- [ ] Every new field is **optional**, with a defined meaning when absent.
- [ ] No existing field became required, changed type, or narrowed its accepted values.
- [ ] The reader still accepts every shape this crate has ever written — including the two
      decontamination event types, whose writer is retired and whose readers are not.
- [ ] `cargo test --all-targets --locked` passes, which includes replaying the frozen corpora in
      `fixtures/skill-evidence/*-v1/` and validating them against the published schemas.
- [ ] No fixture was edited to make any of the above pass.

If a change cannot satisfy these, it needs a new schema version whose reader handles both, not a
looser interpretation of "additive".

## 3. Pre-release checks

```console
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
cargo package --list          # confirm the shipped file set is what you expect
cargo publish --dry-run
```

`cargo package --list` matters more than it looks: the four skill packages and two schemas under
`assets/` are contract, and a consumer that receives the library without them has machinery and
no operator for it.

CI runs the last two on every push, diffing the file list against
`.github/expected-package-files.txt`. When the shipped set legitimately changes — a new skill
package, a new schema — that file is what you update; it carries no comments because it is
compared byte-for-byte against `cargo package --list` output.

## 4. Publish and tag

```console
cargo publish
git tag vX.Y.Z && git push --tags
```

crates.io versions are immutable. A bad release is yanked, never replaced — so the dry run is not
a formality.

## 5. Write the release note

It exists so a consumer can decide whether to upgrade and knows what to do afterward. It must
name, explicitly:

- **Breaking Rust API changes** — what a consumer's build will complain about.
- **Command-surface changes** — renamed flags, removed subcommands, changed exit-code meanings.
  These break operators and skill packages without breaking `cargo build`.
- **Installed-package changes** — additions, and above all **retirements or renames**, with the
  exact retired package names, the minimum crate version that can withdraw them, and the exact
  `.claude/skills/` and `.agents/skills/` directories an older consumer must delete manually for
  each affected package. See §7.
- **Schema changes** — and the reason they satisfy §2.

## 6. Bring a consumer forward

From the consumer's repository root, on a clean tree:

```console
# 1. Take the new version.
cargo update -p skill-evidence          # or edit the pinned version first

# 2. Confirm it still builds and the consumer's own suites pass.
cargo test --all-targets --locked

# 3. Check the asset install without --force. On a consumer with any differing
#    installed file, this is expected to refuse atomically with exit 3. It writes
#    nothing and names every differing file.
<host-command> skills evidence install --root .

# 4. Read the refusal's file list before re-running. Files that differ because
#    the crate changed are the upgrade; files that differ because you edited
#    them locally are not. Resolve the second kind first, then:
<host-command> skills evidence install --root . --force

# 5. Withdraw packages the install receipt named as retired. Without --force,
#    a locally edited retired file refuses the whole operation and removes nothing.
<host-command> skills evidence withdraw --root .

# 6. Review what actually changed.
git diff

# 7. Re-derive each evidence store and confirm history is untouched.
git status --short reports/skill-evidence/    # events.jsonl must not appear
```

Step 3 is the useful one. Because `install` decides every write before the first byte lands,
running it *without* `--force` writes nothing when files differ. Its exit-3 refusal is the expected
result in that case, and the refusal's file list identifies exactly which files the upgrade would
change. It also reports any retired package still present, but never removes one under any flag.

Step 5 is deliberately separate. `withdraw` compares every retired file with the last copy this
crate shipped and decides every removal before the first one occurs. A difference refuses the
whole operation; after confirming that the differing file is an edited copy of a retired file the
crate shipped, re-run with `--force`. A foreign file or discovery link stays in place even under
`--force`, and the receipt names why it was retained.

Step 7 is the one that matters. A projection may legitimately change; `events.jsonl` may not.

Host commands: `playbench` for playbench, `mundifold` for mundifold, and
`cargo run --locked -p developer-tools --` for what-we-bring-home.

## 7. Withdrawing retired packages

`skills evidence install` only writes and reports. It never removes, including under `--force`.
`skills evidence withdraw` is its deliberate inverse for whole packages this crate has retired.

The crate permanently retains the last-shipped templates for every retired package. That lets
`withdraw` render them with the consumer's host names, compare each installed file byte-for-byte,
and remove only what the crate can prove it shipped. It also removes the correct
`.agents/skills/` discovery link and empty directories below the retired package, including the
package directory itself. The `.claude/skills/` and `.agents/skills/` roots are never removed.

This is not hypothetical. Retiring `legacy-skill-decontamination` cost playbench a hand-written
deletion of 228 lines across three files, inside its own migration commit `8cbc1573`. **And that
was not the end of it**: `git rm` removes tracked files but not the directories holding them, so
two empty directories survived, untracked and therefore invisible to `git status`, until they were
found and `rmdir`-ed on 2026-08-08.

That history fixes the release rule: a retirement is still a breaking installed-asset change, its
templates stay in the retirement set permanently, and the release note tells consumers to run:

```console
<host-command> skills evidence withdraw --root .
```

For a consumer pinned to a version that predates `withdraw` or the relevant retirement entry, the
manual fallback remains explicit and package-granular:

```console
rm -rf .claude/skills/<retired-package> .agents/skills/<retired-package>
```

Deleting individual files is not an adequate fallback: it can reproduce the empty-directory
orphan that motivated this command.

## 8. A consumer that stays behind

Being behind is not a defect. A consumer may pin an old version indefinitely and nothing about
that obliges anyone.

A consumer that *vendors* a copy is a different matter — it has left the contract entirely, gets
no compatibility guarantee, and a defect reported against its line numbers is a report about that
fork until it is reproduced against a published version. Which consumers resolve a published
version is recorded once, in the table under *What is actually at stake*.
