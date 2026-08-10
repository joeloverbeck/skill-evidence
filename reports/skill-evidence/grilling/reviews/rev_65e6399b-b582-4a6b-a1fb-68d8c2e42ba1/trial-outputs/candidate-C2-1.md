Bash was denied for the git/date probe, so git history is unavailable — I'm treating `CHANGELOG.md`'s `## Unreleased` as the authoritative record of user-visible changes, which `CONTRIBUTING.md:7-8` explicitly warrants ("maintained by hand… at the time of the change, not at release time").

## Premise check

| Premise | State |
|---|---|
| `docs/release-process.md` steps 1–3 | **Verified** — 1: move `## Unreleased` into a dated section; 2: `Cargo.toml` version; 3: version line at top of `docs/status.md` |
| Current version `0.3.1` | **Verified** — `Cargo.toml:3`, `CHANGELOG.md:8`, `docs/status.md:3` all agree |
| Unreleased content | **Verified** — one feature (`--limit`), one fix (case-insensitive `--tag`) |
| `--limit` documented per `CONTRIBUTING.md:3` | **Verified** — `README.md:8` already lists it; no gap to close |
| Release date `2026-08-10` | **Derived** from session context; `date` probe was denied, so not independently confirmed |
| Next version number | **Unresolved** — nothing in the repo names it |

Two things you should see before I touch anything:

**Step 3 leaves a claim that isn't true yet.** `docs/status.md:3` reads "Current released version: 0.3.1". Step 3 sets it to the new version — but since we stop before step 5, nothing is actually published. The repo will assert a release that hasn't happened. That's inherent to the doc's design (it assumes 4–5 follow immediately), not a mistake in your boundary. Say the word if you'd rather I leave `status.md` alone; otherwise I follow the doc.

**The doc says "Do not skip step 4"** (`docs/release-process.md:3`). I'm honoring your boundary and skipping it. The cost is low: `preflight.sh` is a pre-publish validation gate, and `publish.sh` — the irreversible one — is also excluded. Nothing gets uploaded. Noting it as a disclosed deviation, not reopening it.

## The one fork I can't resolve from evidence

**What version number?** The doc says "the version being released" but never says how to pick it, and you didn't name one.

- **`0.4.0` (recommended).** Matches the project's own history: `0.3.0` added `search` (feature → minor), `0.3.1` fixed a panic (fix → patch). Unreleased contains a new flag, so semver says minor. Cost: consumes the 0.4 line. Rejected if you don't treat 0.x minors as feature bumps.
- **`0.3.2`.** Valid if you hold 0.x entirely in patch space. Cost: contradicts the `0.3.0` precedent above and hides a feature addition from anyone reading version numbers.
- **Something else** — `1.0.0`, or a number you've already picked.

Whichever you pick gets written into `CHANGELOG.md`, `Cargo.toml`, and `docs/status.md` as `## <version> - 2026-08-10`. Confirm the version (and correct the date if 2026-08-10 is wrong) and I'll execute steps 1–3 and stop.