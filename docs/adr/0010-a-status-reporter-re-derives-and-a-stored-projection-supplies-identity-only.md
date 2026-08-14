# A status reporter re-derives, and a stored projection supplies identity only

Status: accepted (2026-08-14, commit `07ba195`)

Every gate answer a status reporter prints is derived from `events.jsonl` at the moment it prints.
The `gate-status.json` sitting beside that stream is read for one thing and is otherwise not
consulted.

**The stored projection supplies target identity, and only when the stream cannot.**
`read_gate_projection` (`src/status_reports.rs:860`) returns a `serde_json::Value` consumed at
exactly one site — `src/status_reports.rs:341`, for `target_repo_relative_path`, and only after
`validated_event_target_path` has failed to identify the target from the events themselves.
`tests/skill_status_cli.rs:2045` pins it: the fixture writes a projection carrying
`"state": "deliberately_stale_fixture"`, and the reporter is required to identify the store and
ignore everything else in the file.

**The degenerate fixtures are the proof of that rule, not a gap in coverage.** The projections in
`fixtures/skill-evidence/status-reporters-v1/reports/skill-evidence/{game-alpha,game-beta}/gate-status.json`
are the literal bytes `{"stale":"projection"}` and are excluded from schema validation deliberately.
`tests/lifecycle_compatibility.rs:250` states the ground: *"their job is to prove the status
reporters re-derive instead of trusting whatever is on disk. They are supposed to fail validation."*
A projection is derived and regenerable, so it is not the surface the forward-only rule protects —
`events.jsonl` is.

**There is therefore no typed read-back of the projection to design.** A deserializer for
`gate-status.json` would be machinery for a read that does not happen, and it would have to tolerate
precisely the bytes two frozen fixtures exist to make unreadable. The tolerance requirement and the
absence of a reader are the same fact seen from two sides.

## Why

An architecture review generated 2026-08-13 against `main @ 9cbc923`
(`architecture-review-20260813T111751.html`; a session artifact, not retained in this tree) carried a
candidate titled *"The reporter reads the gate twice."* It diagrammed `gate-status.json` re-read as a
`Value`, flowing through `string_field` / `.get()` into `skill_evolution_status`; it listed
*"projection read back typed, not raw"* among the candidate's wins; and it posed as the card's one
open design question: *"A typed read-back cannot be strict serde: `status-reporters-v1` deliberately
holds `{"stale":"projection"}`, and every projection ever written must still parse. The reader has to
be tolerant by design."*

That path does not exist. `string_field` and `optional_string_field`
(`src/status_reports.rs:1288`, `:1292`) are applied to recorded-event payloads —
`let payload = &event.raw["payload"]` at `:1212` — and never to a projection. Two unrelated untyped
reads were drawn as one arrow, and a design problem was derived from the conflation.

The review read the tree accurately and cited every line it used. What it could not find was this
decision, because the decision was written down nowhere it would look: not in the nine ADRs, not in
[`../principles/`](../principles/), not in [`../../CONTEXT.md`](../../CONTEXT.md). It survived in a
comment inside a compatibility test and in the contents of two fixture files. A load-bearing rule
reachable only by reading the fixtures that enforce it will be re-litigated by every careful reviewer
who does not happen to open them, and the cost of the first occurrence is already paid.

### The rest of the card, checked at `07ba195` and left alone

The same review will be run again, so what was examined and declined is recorded here with it. Each
item below is real; none of them is reachable by a consumer, which is the test
[`../principles/mission-and-scope.md`](../principles/mission-and-scope.md) applies.

- **The run-group rule has two implementations that cannot disagree.** `src/gate.rs:1166` skips every
  event whose `target_content_hash` is not the current hash (`:1171`) before deduplicating on
  `same_run_group`. `src/status_reports.rs:1246` deduplicates on `(hash, run_group)` across all
  events and reads the current hash's row at `:1275`. Restricted to the current hash these are the
  same computation. They also belong to different commands: `summarize_current_evidence` is called
  only from `:219`, inside `method_gap_research_inventory`, which publishes the per-hash
  `observed_target_hashes` breakdown that `gate::derive` does not produce at all. The reporter's
  tally is a superset the gate does not offer, and the current-hash figure falls out of rows it has
  already built. Both sides are pinned independently — `src/gate.rs:1740`,
  `tests/skill_status_cli.rs:496` — and `src/gate.rs:1164` records the measurement behind the shared
  premise: 1251 use records across the three consumer repositories and this one, every
  `(hash, run group)` pair distinct.
- **`GateStatus.state` is a `String`, matched by prefix.** `src/status_reports.rs:516` reads
  `starts_with("quarantined_")`, which selects `quarantined_eligible` and
  `quarantined_pending_cooldown` and nothing else in the closed enum at
  `assets/schemas/gate-status.v1.schema.json:87`. It is correct, and typing the field would
  break the Rust API for output that is byte-identical on the wire.
- **`/payload/authorizing_rule` is hand-pointed on both sides of the seam**, at `src/gate.rs:1054`
  and `src/status_reports.rs:454`. One JSON path, two readers, no divergence available to them.
- **The counterfactual derivation is tested, though not directly.**
  `tests/skill_status_cli.rs:1724` drives the `omitted += 1` branch at `src/status_reports.rs:446`
  end to end. What it lacks is a unit test, because `src/status_reports.rs` carries no in-crate tests
  — so every assertion about the counterfactual pays a tempdir and a process spawn.

The card's architectural observation stands and is not disputed: `src/status_reports.rs:16` still
imports nine crate-private items, now split across two modules, and
`ValidatedStream::without_event` (`src/gate.rs:514`) is `pub(crate)` with exactly one caller in the
tree — `src/status_reports.rs:435`. Deepening the gate derivation gave the gate a seam; it did not
reduce the reporter's reach into internals, and one of the items it exposed exists solely to serve
the reporter. That is recorded as accepted rather than unnoticed.

## Considered options

**Deserialize the projection into a tolerant typed struct.** Rejected. It is the review's own
proposal and it builds a reader for a read this repository decided not to perform. Every field it
would type is already derived from the stream microseconds earlier; the only field actually consumed
is a path used when derivation cannot name the target, which is exactly the case where the
projection's other fields are least trustworthy. Tolerance would additionally have to accept
`{"stale":"projection"}`, so the deserializer's contract would be that it extracts nothing
guaranteed — which is what `.get()` on a `Value` already says, more honestly and in one line.

**Type `GateStatus.state` as an enum and drop the prefix match.** Rejected. `GateStatus` is `pub`,
so the change breaks every consumer reading `status.state` as a string while producing identical
serialized bytes — the shape [`../principles/consumer-contract.md`](../principles/consumer-contract.md)
exists to catch, in the direction Cargo SemVer does flag but where the compensation is nil. The
prefix match is correct against the current enum and the enum is closed and published.

**Move the per-hash tally into the gate derivation.** Rejected. It reverses the dependency that makes
the current arrangement cheap: the gate needs one number, the method-gap inventory needs a row per
hash, and the inventory derives the gate's number from its rows for free. Relocating it would put a
reporting breakdown inside the derivation to remove a duplication that cannot produce a wrong answer.

**Give `src/status_reports.rs` in-crate tests and leave the structure alone.** Rejected for now, and
the closest of the alternatives. The testability gap is genuine and the cost is real, but it is this
repository's own velocity rather than a consumer's bottleneck, and
[`../principles/mission-and-scope.md`](../principles/mission-and-scope.md) declines to price that as
priority — an improvement gains none from being obviously correct. It reopens the moment a
consumer-facing defect has to be fixed in that file.

**File the card's items as issues and leave the decision unwritten.** Rejected. An issue records that
something was observed and authorizes nothing, and an open issue proposing work this decision
declines invites its own closure by doing the work. It also leaves the actual gap — an unwritten
decision — exactly where it was.

**Record the decision and the rejection, and change no code.** Chosen. The one thing the occurrence
proved was missing is a written decision; the items the review proposed are, on inspection, changes
no consumer can feel. Writing the record is the whole of the warranted work.

## Consequences

- **Documentation only.** No Rust API, command surface, recorded-event shape, published schema, or
  installed package byte moves, and no fixture corpus is touched. Under
  [`../releasing.md`](../releasing.md) this is not a release and nothing is owed to withdrawal.
- **No ADR is superseded.**
  [`0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md`](0002-blocked-no-valid-test-retires-its-evidence-from-the-gate.md)
  governs what the live projection *says*; this decision governs who reads it back, and the two do
  not meet. The projection remains the honest published statement of the gate — it is simply not the
  channel a reporter in the same process uses to learn it.
- **A reporter may not start trusting the projection quietly.** Any future read of
  `gate-status.json` beyond target identity contradicts this record and the two fixtures, and
  `tests/skill_status_cli.rs:2045` fails first.
- **`CONTEXT.md` gains no term.** *Qualifying use* (`:26`) and *run group* (`:72`) already carry the
  meanings used above. "Counterfactual gate" stays uncoined: it names one branch at
  `src/status_reports.rs:437` with a single caller, and coining it would give a private mechanism the
  standing of lifecycle vocabulary.
- **What could reopen this.** A divergence a consumer can see between a number
  `skills evolution-status` prints and the same-named field in `gate-status.json`. A gate state whose
  name begins `quarantined_` that is not a quarantined state, which would make the prefix match
  wrong rather than merely untyped. A gate derivation that grows a per-hash tally for its own
  reasons, which would turn the method-gap reporter's rows from a superset into a copy. Or a reporter
  that must describe a store it cannot re-derive — a projection whose stream is unreadable, or a
  cross-repository read — which would make identity-only insufficient rather than incorrect, and
  would be the first case where a typed reader has a question to answer.
