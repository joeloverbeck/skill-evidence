# Retain retired package templates so withdrawal can prove its authority

Status: accepted (2026-08-09, GitHub [#3](https://github.com/joeloverbeck/skill-evidence/issues/3))

A retired skill package remains in each consumer because `install` only writes the current asset
set. The crate will retain the last-shipped template for every file in every package it retires,
separate from the installed asset set, and expose a deliberate `withdraw` operation. `install`
reports retired packages it finds but never removes them. `withdraw` renders the retained
templates with the consumer's `Host`, compares the installed files byte-for-byte, and removes the
package only within that proof.

Legacy Skill Decontamination is the first retirement entry. Its three templates are the exact
blobs last shipped before commit `7034bd4` removed its writer package. The retirement set is
permanent and package-granular: entries accumulate, are never installed, and are never dropped on
the assumption that every consumer has caught up.

## Why

The only observed pressure is whole-package retirement. Playbench had to delete Legacy Skill
Decontamination by hand, and even that left the empty package and `scripts/` directories behind.
The narrow answer must therefore remove files, the package's correct discovery link, and empty
directories below the package root without inventing partial-file retirement machinery.

Authority depends on the retained template rather than a package name. Templates contain host
tokens, so only rendering the last shipped source for the current `Host` can distinguish a
pristine installed file from a locally edited one. Every file and link decision is made before the
first removal. A differing shipped file refuses the whole operation unless `--force` is present;
even then, foreign files, foreign links, and symlinked package or boundary paths remain outside the
crate's authority.

## Considered options

**Retain only retired package names.** Rejected. A name identifies where to look but cannot prove
which rendered bytes the crate wrote. Removing everything below the named directory would erase
consumer-owned files and could not mirror `install`'s refusal discipline.

**Retain author-time content hashes.** Rejected. The shipped templates are rendered with open-ended
host names, so no hash computed in this repository can identify every consumer's installed bytes.
A hash of the rendered file would still require retaining enough source to render it.

**Persist an install manifest in each consumer.** Rejected. It introduces a new compatibility and
state surface in every consumer, cannot reach packages stranded before the manifest existed, and
generalizes beyond the one whole-package retirement actually observed.

**Retain the last templates in a retirement set compiled separately from installed assets.**
Chosen. It reaches the existing orphan retroactively, proves authority for host-rendered files,
and is structurally unreachable from `install`.

## Consequences

- `assets::withdraw` and `skills evidence withdraw` are new public surfaces. This release takes the
  `0.3.0` minor version so consumers opt into them deliberately.
- `InstallReceipt` optionally serializes `orphaned_packages`; ordinary install remains a
  side-effect-free preview with respect to retirement.
- A withdrawal receipt names removed files, forced files, removed directories, removed discovery
  links, and every retained path with its reason. Refusal remains exit code `3`; a mid-operation
  I/O failure remains unsafe failure `1`.
- A foreign file legitimately keeps its containing retired package directory alive. A later run
  completes withdrawal after the consumer resolves that file.
- No recorded-event type, schema, reader, writer, fixture, or evidence-store path changes.
- Retiring or renaming an installed package remains a breaking installed-asset change. Withdrawal
  removes the manual work for consumers that have upgraded far enough to carry the relevant
  retirement entry; it does not make the surface change additive.
