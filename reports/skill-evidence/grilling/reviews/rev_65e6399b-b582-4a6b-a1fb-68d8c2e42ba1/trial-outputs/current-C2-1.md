Read the process doc, both scripts' relevant one (`preflight.sh`), and all three files steps 1–3 touch. `git log`/`git status` were denied, so tag history is **unavailable** — noted below.

## Mode: documented process

You've ratified the boundary (steps 1–3, nothing past). I'm not re-opening it. Verifying scope and naming branches before I mutate anything.

**Premises checked**

| Claim | State |
|---|---|
| `CHANGELOG.md` has a populated `## Unreleased` | verified — two entries (`--limit` flag; case-insensitive `--tag` fix) |
| `Cargo.toml` version to bump | verified — `version = "0.3.1"`, line 3 |
| `docs/status.md` has a version badge line | verified — line 3, `Current released version: 0.3.1` |
| Dated-section format | verified — `## 0.3.1 - 2026-05-02`, so `## X.Y.Z - YYYY-MM-DD`; today is 2026-08-10 |
| Git tags / prior bump in flight | **unavailable** — Bash denied |

**Branches, and how each resolves**

1. **Version number** — unspecified by you *and* by the doc ("the version being released" is an input it never supplies). This is the one real fork; it lands in three files. → question below.
2. Changelog date → evidence-resolved: `2026-08-10`, existing format.
3. Which status.md line is "the badge" → evidence-resolved: line 3.
4. `## Unreleased` disposition → evidence-resolved: heading stays, entries removed (doc: "leave `## Unreleased` empty"; CONTRIBUTING says entries land there per-change, so the heading must survive).

**Two disclosures, neither needing your approval**

- The doc says "Do not skip step 4." Your cap at 3 skips it. I'm honoring your cap: step 4 is a read-only gate protecting the irreversible step 5, and since 5 isn't running, skipping 4 leaves nothing unsafe — just a tree with the version bumped and unvalidated. Flagging, not re-litigating.
- Separately: `scripts/preflight.sh` is a stub — it `echo`s `PREFLIGHT OK` unconditionally with no actual checks. So step 4 wouldn't have caught anything anyway. Out of scope to fix; you may want to know before a future real release.

---

## The one question: what version?

**A — `0.4.0` (my recommendation).** The Unreleased block contains a new user-visible flag (`--limit`), not just fixes. This matches the repo's own precedent: `0.3.0` was "Added `search`" (feature → minor), `0.3.1` was a panic fix (fix → patch). Cost: publishes a feature release on a project `status.md` calls "feature-frozen apart from output-format work." Rejected if you consider `--limit` too small to consume a minor.

**B — `0.3.2`.** Consistent with the feature-frozen posture. Cost: understates a new flag — anyone pinned to `~0.3` gets a surface change in a patch, which is exactly what your own `0.3.0` avoided. Rejected by the changelog's own precedent.

**C — `1.0.0`.** A stability commitment. Cost: promises compatibility nothing here supports — README states there's still no machine-readable output mode, and CONTRIBUTING treats output formats as an unsettled compatibility surface. Rejected by the maintenance posture in `status.md`.

Say **A**, **B**, **C**, or give me a different number, and I'll execute steps 1–3 — changelog section dated `2026-08-10`, `Cargo.toml`, `docs/status.md` line 3 — and stop there.