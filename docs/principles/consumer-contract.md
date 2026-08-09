# Consumer Contract

Status: accepted constitutional principle

This repository is depended upon by repositories that record evidence they cannot regenerate.
That is the whole reason this document exists, and it is the one place where a mistake here is
not recoverable by a consumer pinning an older version.

## Three surfaces, not one

A change to this repository can reach a consumer through three independent surfaces. They have
different failure modes and different protections, and conflating them is the mistake this
document exists to prevent.

| Surface | What it is | How a consumer recovers from a bad change |
|---|---|---|
| **Rust API** | `Host`, the lifecycle functions, `cli::SkillsArgs` / `run` / `Exit`, `assets::install`, the `cli` feature | Pin the previous version. Compile-time breakage; loud and immediate. |
| **Installed assets** | The four skill packages under `.claude/skills/`, their `.agents/skills/` links, the two schemas under `schemas/skill-evidence/` | Re-run the installer from the previous version, or restore from git. Recoverable, but see *the installer never removes* below. |
| **Recorded evidence** | `reports/skill-evidence/<skill>/events.jsonl`, append-only, accumulating in every consumer | **Not recoverable.** The data is already written. Pinning an older version rolls back the reader, not the history. |

Cargo's SemVer protects the first surface only. It says nothing about the other two. Treating a
version bump as sufficient protection is the specific error this document forbids.

## Versioning the Rust API

Ordinary Cargo SemVer. While the crate is `0.x`, Cargo treats the minor position as the breaking
one: `0.1.0` and `0.1.1` are compatible, `0.1.0` and `0.2.0` are not.

- A change that breaks a consumer's build takes a minor bump while `0.x`, a major bump after
  `1.0.0`.
- A change to the CLI surface — a removed subcommand, a renamed flag, a changed exit code —
  is a breaking change even though `cargo build` may not notice. Operators and skill packages
  invoke that surface by name.
- The published exit-code meanings (`0` success, `1` unsafe failure, `3` refusal) are contract.
  Hosts map them onto process exit codes; changing what they mean silently changes what a
  consumer's automation concludes.

`1.0.0` is not a goal. It is a statement that the recorded-evidence shape has stopped moving,
and it should not be cut before that is true.

## The forward-only evidence rule

**A consumer's recorded evidence is immutable and this repository may never invalidate it.**

Concretely, for any change that touches the shape of a recorded event:

1. **Additive and optional, or a new schema version.** A new field is optional, with a defined
   meaning when absent. A field may not become required, change type, or narrow its accepted
   values within a schema version.
2. **The reader accepts every shape this repository ever wrote.** Including shapes written by
   workflows that have since been retired. A retired writer does not retire its readers.
3. **A new schema version does not orphan the old one.** `event.v2` means the reader handles v1
   and v2, and says which it is reading. It does not mean v1 streams stop being valid.
4. **The frozen corpora are the proof.** Every change to a schema or to recorded-event handling
   is validated against `fixtures/skill-evidence/*-v1/`, which holds real streams covering all
   seven v1 event types. Those fixtures are never edited to make a change pass. If a change
   cannot keep them valid, the change is wrong, not the fixtures.

The rule binds hardest where it is least visible. Adding an optional flag and an optional serde
field is a compatible change by every Cargo rule and every code review instinct — and it is
exactly the change that invalidates history if the optionality is got wrong on the read side.

## The installer never removes

`skills evidence install` writes packages and refuses to clobber a locally edited file. It has
no removal mode under any flag. **Removing a retired package is a separate, deliberate authority
transition:** `skills evidence withdraw` acts only on the permanent retirement set and never on
the current installed set. A consumer opts into withdrawal explicitly after an upgrade; install
may report the orphan, but cannot remove it.

This has already happened once, and the cleanup fell to the consumer: retiring the decontamination
package required a hand-written deletion of 228 lines inside playbench's own migration commit,
because nothing upstream could reach it. Even that was not enough: `git rm` removes tracked files
but not the directories holding them, so two empty directories survived the very commit meant to
remove them — untracked, therefore invisible to `git status`, and found only by a later inspection
that went looking. It is a real property of the distribution mechanism, not a hypothetical.

Withdrawal closes that gap without weakening the installed-asset boundary:

- Retiring or renaming an installed package is a **breaking change to the installed-asset
  surface**. The release must name the retired package and the minimum version whose retirement
  set can withdraw it; consumers pinned below that version still require the exact manual removal
  path.
- Adding a package is additive. Renaming one is not — it is a remove plus an add, and it strands
  the old directory until the consumer deliberately withdraws it.
- A consumer upgrade is not complete when `cargo update` succeeds. See
  [`../releasing.md`](../releasing.md).
- The retirement set retains each retired file's last-shipped template permanently. Withdrawal
  renders it for the consumer's host, compares byte-for-byte, and refuses before the first removal
  if a shipped path differs. An explicit force may remove that differing shipped path; it never
  authorizes removal of a foreign file, link, or package.
- Withdrawal removes a retired package's proven files, correct discovery link, and empty package
  directories. It never touches the `.claude/skills/` or `.agents/skills/` roots and never reaches
  `reports/skill-evidence/`.

The retirement set is warranted because consumers accumulated an observed orphan, not because a
general uninstall facility would be tidy. A current package remains outside withdrawal: the crate
cannot remove a consumer-created package, selectively remove a live package, or forget a
retirement on the assumption that all consumers have caught up.

## Shipped packages carry instructions, not executables

**An installed skill package carries instructions, not executables.** A script inside a shipped
package makes its runtime a dependency of every consumer's tree — arriving with a crate upgrade,
installed by the same command that installs the instructions, and, per *the installer never
removes* above, unavailable to withdrawal while the package remains current. The consumer did not
choose that runtime and cannot remove it through this crate without the package itself first being
retired.

The shipped packages already satisfy this: no executable bit on any file, no script of any
language. Writing the rule down changes nothing that ships today. What it changes is that the
next package wanting a script argues with this clause rather than with someone's recollection
of an ADR.

A private skill in this repository is a different surface. It is not installed, it reaches no
consumer, and this clause does not reach it — a scope boundary, not a permission.

## What is contract and what is internal

Contract: the public Rust API, the `cli` feature boundary, the mounted command surface and its
exit codes, the installed skill packages, the published schemas, and the recorded-event shape.

Internal: everything else, including module layout, private helpers, the reference binary's own
argument parsing, the test suites, and the fixture directory structure.

A consumer that reaches into internals has taken on the risk of that reach. A consumer that
relies on contract has not.

## Upgrade authority runs one way

A consumer's need is evidence. It can motivate a bounded change here, and a real downstream
failure is among the strongest evidence this repository can receive.

It does not follow that a consumer's schedule governs this repository. Specifically:

- A downstream deadline does not authorize a change that breaks the other consumers.
- A consumer may stay on an older version indefinitely. Being behind is not a defect, and this
  repository does not chase consumers to upgrade.
- A consumer that vendors a copy has left the contract. It receives no compatibility guarantee,
  and a defect reported against a vendored fork's line numbers is a report about that fork until
  it is reproduced against a published version.

## Toward outside users

The crate is published, so strangers may depend on it. The obligations accepted toward them are:
the published schemas and recorded-event shape follow the forward-only rule above; breaking
changes take a version bump that Cargo will honor; the README states what the crate is for.

Not accepted: a deprecation window, a maintained changelog as an obligation rather than a
courtesy, a support commitment, backports, or feature work motivated by an outside request. If
outside users appear, that is new evidence and this section is what gets amended.
