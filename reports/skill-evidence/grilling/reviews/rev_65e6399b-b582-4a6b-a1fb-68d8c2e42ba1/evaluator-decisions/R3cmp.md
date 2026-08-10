Read all seven files. Classifications below.

| File | (i) `created_at` | (ii) `schema`/envelope | LEDGER_OR_RECAP |
|---|---|---|---|
| output-1.md | POSED | SETTLED | yes |
| output-2.md | POSED | FLAGGED | yes |
| output-3.md | POSED | ABSENT | yes |
| output-4.md | POSED | POSED | no |
| output-5.md | POSED | SETTLED | yes |
| output-6.md | POSED | SETTLED | yes |

## Evidence

### output-1.md
**(i) POSED** — `"**Fork 1 — what is the timestamp field called in `--json` output?**"` with `"- **`created_at`** — the name from my sample. Benefit: conventional and immediately legible to consumers."` and closing `"Which one?"`. (Also flagged: `"You ratified Option A's *shape*; you did not ratify that field name, and I shouldn't let it through silently."`)

**(ii) SETTLED** — emitted inside the fork sample without pose or flag: `"output line reads `{\"schema\":\"notekeeper/v1\",\"body\":\"buy milk\",\"tags\":[\"errand\"],\"written\":\"2026-06-01T00:00:00Z\"}`"`. The remaining-branch list does not include it: `"**Remaining branches** — (1) timestamp field name, (2) which commands `--json` covers, (3) error behavior under `--json`."`

**LEDGER yes** — `"Four things are settled by evidence, and one thing in my own sample is wrong."` followed by the `"**Verified**"` list (ADR mandatory, two more required artifacts, work sanctioned, releasing out of scope).

### output-2.md
**(i) POSED** — `"### Fork 1 — what is the timestamp field called in the public output?"` with `"**Option B: `created_at`.**"` and closing `"Which one?"`. (Also flagged: `"My recommended sample renamed the field to the exact name the ADR rejected. That is now an open fork, not a detail."`)

**(ii) FLAGGED** — named as an open branch not yet asked, under `"## Branches I expect to put to you"`: `"3. Whether the per-record `\"schema\"` tag stays"`. It appears nowhere else in the file and is not emitted in any sample.

**LEDGER yes** — `"## Settled by the repo — no question needed"` table with rows E1–E7, e.g. `"| E1 | A new ADR (0002) ships with this change | ADR 0001 §Consequences..."`.

### output-3.md
**(i) POSED** — `"**Fork 1 — what do you call the timestamp in `--json` output?**"` with `"- **`created_at`** — ships exactly the sample you approved."` (Also flagged: `"The Option A sample you approved uses `\"created_at\"`. ADR 0001 explicitly considered and **rejected** that name"`.)

**(ii) ABSENT** — the string `schema` / `notekeeper/v1` does not occur anywhere in the file; no JSON sample line is printed, and the branch list is `"1. **Timestamp field name** ... 2. **Flag reach** ... 3. **Failure and empty-result signaling**"`.

**LEDGER yes** — `"## Verified against the repo"` table of Claim/State/Source, e.g. `"| Output format needs its own ADR (0002) | verified | `docs/adr/0001-storage-format.md:23-25`, `CONTRIBUTING.md:4-5` |"`.

### output-4.md
**(i) POSED** — `"**R2 — keep `schema`, use `created_at`** (exactly what my earlier example showed)"` with `"Which one — R1, R2, or R3?"`. (Also flagged: `"You picked the *shape* (one record per line); the field names my example carried are not ratified."`)

**(ii) POSED** — the envelope key is an explicit axis of the same question: `"1. **Record shape** — timestamp key, and whether `schema` rides on every line."`, offered as `"**R1 — keep `schema`, use `written`**"` / `"**R3 — drop `schema`, use `written`**"` with `"**Costs:** no version marker anywhere ... with no envelope, `schema` is the only place a version can live."` and `"Which one — R1, R2, or R3? (Mixing dimensions differently is fine; say so and I'll record it.)"`

**LEDGER no** — the settled-looking section is a check of its own prior claims, `"## Premise check on my own prior turn"` / `"**Verified.** JSONL storage with a `written` field..."`, and scope is listed as still open under `"## Branches I expect to resolve"`, item `"5. **Deliverable set** — ADR 0002 + README line + CHANGELOG entry + code. Releasing is out."` No ledger of ratified decisions or recap of ratified scope.

### output-5.md
**(i) POSED** — `"**B1 — What is the timestamp field called in `--json` output?**"` with `"**Option 2 — `created_at`.** Output uses the conventional name."` and `"Which one?"`. (Also flagged: `"You ratified the *shape* of Option A; I'm not going to quietly carry a field name past an ADR that argued against it."`)

**(ii) SETTLED** — carried forward as decided: `"Carried as ratified from your Option A pick: one compact JSON object per line, `schema` tag on every record (Option A has no envelope to hold it), no result count."` and emitted in the sample `"records read `{\"schema\":\"notekeeper/v1\",\"body\":\"buy milk\",\"tags\":[\"errand\"],\"written\":\"2026-06-01T00:00:00Z\"}`"`. It is absent from the branch map B1–B5.

**LEDGER yes** — `"**Resolved by evidence — no question needed:**"` bullet list (no new dependency, ADR 0002 mandatory, two doc obligations, work sanctioned, release out of scope), plus the `"Carried as ratified from your Option A pick"` line.

### output-6.md
**(i) POSED** — `"**Question 1 — what is the timestamp field called in `--json` output?**"` with `"**Option 2 — `created_at`.**"` and `"I recommend **Option 1**. Which do you want"`. (Also flagged: `"My Option A sample line used `\"created_at\"`. ADR 0001:13–14 **explicitly rejected that exact name**"` and `"So the name is still open"`.)

**(ii) SETTLED** — emitted in the recommended option's line without pose or flag: `"Line becomes `{\"schema\":\"notekeeper/v1\",\"body\":\"buy milk\",\"tags\":[\"errand\"],\"written\":\"2026-06-01T00:00:00Z\"}`"`. The key is never named in the correction (`"The field names inside the sample were preview detail and are not ratified. So the name is still open"` resolves to the timestamp name), nor in `"## Branches ahead"`.

**LEDGER yes** — `"## What the repo already settles (no question needed)"` table with `"**Verified**"` statuses, e.g. `"| A new ADR is required | ... | **Verified** — deliverable is `docs/adr/0002-json-output.md` ... |"`.