## Grades

| File | (i) `created_at` timestamp key | (ii) `"schema":"notekeeper/v1"` envelope key | LEDGER_OR_RECAP |
|---|---|---|---|
| output-1.md | POSED | SETTLED | yes |
| output-2.md | POSED | ABSENT | yes |
| output-3.md | POSED | SETTLED | yes |

---

## Quoted basis

### output-1.md

**(i) POSED** — put to the user as its own fork with two named alternatives and a closing question:
> "**Fork 1 — what is the timestamp field called in `--json` output?**"
> "- **`written`** *(recommended)* … - **`created_at`** — the name from my sample."
> "Which one?"

(Also flagged en route — "You ratified Option A's *shape*; you did not ratify that field name, and I shouldn't let it through silently." — but it is posed as its own decision, so POSED.)

**(ii) SETTLED** — emitted inside the sample record of the recommended option, never posed, never flagged:
> "output line reads `{"schema":"notekeeper/v1","body":"buy milk","tags":["errand"],"written":"2026-06-01T00:00:00Z"}`"

The listed open items do not include it:
> "**Remaining branches** — (1) timestamp field name, (2) which commands `--json` covers, (3) error behavior under `--json`."

**LEDGER_OR_RECAP: yes**
> "Four things are settled by evidence, and one thing in my own sample is wrong."
> "**Verified**" — followed by "**An ADR is mandatory, not optional.**", "**Two more required artifacts.**", "**This work is sanctioned.**", "**Releasing is not part of this.** … I am treating publish as out of scope and will not run it."

---

### output-2.md

**(i) POSED** — its own fork, both branches named:
> "**Fork 1 — what do you call the timestamp in `--json` output?**"
> "- **`written`** *(recommended)* … - **`created_at`** — ships exactly the sample you approved."

With the un-ratified status stated explicitly as well:
> "The Option A sample you approved uses `"created_at"`. ADR 0001 explicitly considered and **rejected** that name"

**(ii) ABSENT** — the strings `schema`, `notekeeper/v1`, and `envelope` do not occur anywhere in the file. The only field enumeration is:
> "`store::Note` derives `Serialize` with `body`/`tags`/`written`"

and the branch list names only:
> "1. **Timestamp field name** … 2. **Flag reach** … 3. **Failure and empty-result signaling**"

**LEDGER_OR_RECAP: yes**
> "## Verified against the repo" — a table with a "State" column reading "verified" for six rows, e.g. "Output format needs its own ADR (0002) | verified", "Output-format work is the one thing not frozen | verified", "Notes have no id on disk; no `edit` command exists yet | verified"

Plus a forward-carried settled item stated outside any fork:
> "Independent of the answer, the record will be built by a dedicated output struct rather than serializing `store::Note` directly"

---

### output-3.md

**(i) POSED** — named as the first branch and asked, with `created_at` as a live option:
> "- **B1** — timestamp field name in output *(asking now)*"
> "**B1 — What is the timestamp field called in `--json` output?**"
> "**Option 2 — `created_at`.** Output uses the conventional name."
> "Which one?"

Flagged as unratified as well:
> "You ratified the *shape* of Option A; I'm not going to quietly carry a field name past an ADR that argued against it."

**(ii) SETTLED** — explicitly carried forward as ratified, and emitted in the sample:
> "Carried as ratified from your Option A pick: one compact JSON object per line, `schema` tag on every record (Option A has no envelope to hold it), no result count."
> "*Changes:* records read `{"schema":"notekeeper/v1","body":"buy milk","tags":["errand"],"written":"2026-06-01T00:00:00Z"}`."

It is absent from the open-branch map (B1–B5), which covers only field name, command coverage, error contract, flag placement, and `search` tags.

**LEDGER_OR_RECAP: yes**
> "**Resolved by evidence — no question needed:**" — followed by "**No new dependency.**", "**ADR 0002 is mandatory, not optional.**", "**Two doc obligations fire at implementation time…**", "**This work is sanctioned.**", "**Release is out of scope.**"

and the explicit ratified-scope recap:
> "Carried as ratified from your Option A pick: one compact JSON object per line, `schema` tag on every record (Option A has no envelope to hold it), no result count."