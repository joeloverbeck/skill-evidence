## Verdict on `reports/audit-findings.md`

**1 of 4 findings survives.** Three are contradicted by the repo — and in two cases the audit recommends something the repo already decided against, in writing.

---

### F1 — Changelog generated at release time → **Reject (premise false)**

The claim is backwards. `docs/release-process.md:5-6` step 1 *moves* existing entries out of `## Unreleased` into a dated section — it generates nothing. The practice the audit recommends is already the written policy: `CONTRIBUTING.md:7-8` says "Every user-visible change gets an entry under `## Unreleased` at the time of the change, not at release time." `CHANGELOG.md:3-6` shows two hand-written entries sitting there now. Recommendation is a no-op.

### F2 — Timestamp field is `created_at`, rename to `written` → **Reject (contradicted, direction inverted)**

The field is *already* `written` (`src/store.rs:7`, consumed at `src/main.rs:25,29,34`). `created_at` appears nowhere in the repo outside this report. ADR 0001 (`docs/adr/0001-storage-format.md:13-14`) records the exact opposite decision: `written` was chosen **over** `created_at`, because a note can be edited and `written` records the original writing. The supporting reasoning is also confused — RFC 3339 governs the timestamp *value* format, not the field *name*, and the value is already RFC 3339 UTC.

### F3 — Pretty-printing needs a new dependency → **Reject (false, and the fix violates policy)**

`serde_json` is already a direct dependency (`Cargo.toml:10`) and ships `to_string_pretty`. ADR 0001:18-20 states this verbatim: "Pretty-printing needs no additional crate; `serde_json::to_string_pretty` covers it." Worse, acting on the recommendation would breach ADR 0001:21-22 — "Adding a runtime dependency requires a superseding ADR." Adopting this finding means doing harm, not nothing.

### F4 — Publish step has no stop condition → **Adopt, and it is worse than reported**

The narrow claim is exactly right and survives the obvious rebuttal. Step 4 *is* a gate, but F4 says "no gate that can halt a bad release **after preflight passes**" — and `docs/release-process.md:10-12` runs `./scripts/publish.sh` unconditionally once it does, against an explicitly irreversible action ("A published version can never be unpublished, edited, or reused").

The audit understated it. `scripts/preflight.sh` is three `echo` statements — it prints `PREFLIGHT OK` unconditionally with no check behind "checking changelog..." or "checking version consistency...". The sole gate guarding a permanent, unrecallable upload **cannot fail**. Recommending a confirmation before upload is correct; the stub preflight is the sharper defect and the audit missed it.

---

### On the source itself

The three rejected findings are each contradicted by one of the two governance documents — F1 by `CONTRIBUTING.md`, F2 and F3 both by ADR 0001, which addresses them by name. The pattern says the reviewer read `src/`, `README.md`, and `docs/release-process.md` but never opened `docs/adr/` or `CONTRIBUTING.md`. Treat this report as unreviewed against project decisions: its one real finding is worth acting on, but nothing here should be adopted on the report's authority alone.

**Collapsed disposition:** reject F1, F2, F3; adopt F4, scoped to *both* the missing confirmation gate and the no-op `preflight.sh`.

Verdict only, as asked — no files changed, nothing staged. Say the word if you want F4 turned into a fix or a ticket.

*Closeout: read-only adjudication of a report; no product code touched, so the repo's lint/typecheck/test completion contract does not apply. Evidence base is complete — all twelve non-skill repo files read in full, so coverage is exhaustive rather than sampled. No premise was left unavailable or pending. One command (a confirmatory `grep` for `created_at`) was denied by permissions; it was redundant given full-file coverage, and F2's disposition does not rest on it.*