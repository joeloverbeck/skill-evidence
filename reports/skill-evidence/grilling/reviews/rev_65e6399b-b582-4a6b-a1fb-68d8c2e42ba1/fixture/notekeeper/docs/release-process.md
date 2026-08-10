# Release process

Follow these steps in order. Do not skip step 4.

1. Move everything under `## Unreleased` in `CHANGELOG.md` into a new dated section for the
   version being released, and leave `## Unreleased` empty.
2. Set the new version in `Cargo.toml`.
3. Update the version badge line at the top of `docs/status.md`.
4. Run `./scripts/preflight.sh`. It must print `PREFLIGHT OK`. If it does not, stop.
5. Run `./scripts/publish.sh`. This uploads the crate to the public registry.
   **A published version can never be unpublished, edited, or reused.** The version number is
   permanent once this step runs.
