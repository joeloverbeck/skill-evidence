Read the repo. Here's what the evidence already settles, so we don't spend the grill on it:

**Resolved — no question needed**

| Premise | Evidence | State |
|---|---|---|
| Nothing to break either way | README:11–12 "There is no machine-readable output mode"; `src/main.rs:12-17` has no format flag | verified — greenfield |
| The work is in scope | `docs/status.md:5` "feature-frozen apart from output-format work" | verified |
| Either spelling needs a new ADR before shipping | ADR 0001:23–25 — a machine-readable *output* format "would need its own ADR"; CONTRIBUTING:4–5 | verified — requirement, not a fork |
| No dependency cost either way | `serde_json` already direct (Cargo.toml:10); ADR 0001:18–20; clap 4 derive does both trivially | verified |
| Either costs a README line + `## Unreleased` entry now | CONTRIBUTING:3, CONTRIBUTING:8 | verified |

So the spelling is a pure surface-design call. One fork actually decides it.

---

**The fork: is JSON the only machine-readable format notekeeper will ever emit?**

**Option A — `--json` (my recommendation).**
- *Changes:* a bool flag on `list` and `search`. Human output stays unnamed and informal.
- *Benefit:* smallest surface, which CONTRIBUTING:3 explicitly demands. And the non-obvious one: `--format` forces you to **name** the current human output (`text`? `plain`?), which under CONTRIBUTING:4–5 promotes today's incidental stdout into a shipped format that downstream scripts may parse and that you'd then need an ADR to change. `--json` adds one compatibility surface; `--format` adds two.
- *Cost:* if a second format ever lands, you carry `--json` forever as an alias beside `--format` — you can't drop it, per CONTRIBUTING:4–5.
- *Rejected by:* you naming a concrete second format you expect within a release or two.

**Option B — `--format=<fmt>`.**
- *Changes:* a `ValueEnum` on `list`/`search` with a default variant for today's output.
- *Benefit:* absorbs csv/ndjson/table later with zero new flags.
- *Cost:* blesses the human text output as a named, ADR-governed format on day one; wordier for the 95% case.
- *Rejected by:* JSON being the terminal state — which is what `docs/status.md`'s "feature-frozen" posture and ADR 0001's stated minimalism both suggest.

Note the asymmetry: retrofitting `--format` onto `--json` leaves a permanent two-spellings wart, but adding `--json` as a shorthand *alias* to `--format` later breaks nothing. That cuts slightly toward B — I still land on A because of the naming cost above.

**Answer with a concrete second format, or "none foreseen."** I'll take "none foreseen" as ratifying `--json`.

One fork I'm recording as open rather than asking, since you scoped this to the flag: the ADR-0002 payload shape — specifically whether JSON output reuses the on-disk field name `written`, which ADR 0001:23 currently classifies as *internal*. Exposing it makes it public. That's the decision with real teeth; the flag name isn't.