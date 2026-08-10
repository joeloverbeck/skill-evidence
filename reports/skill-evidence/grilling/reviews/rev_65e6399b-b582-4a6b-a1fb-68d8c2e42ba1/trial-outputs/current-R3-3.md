I've read the repo: README, CONTRIBUTING, ADR 0001, `src/main.rs`, `src/store.rs`, `Cargo.toml`, `CHANGELOG.md`, `docs/status.md`, `docs/release-process.md`, and both scripts. Findings before the next fork.

## Verification against the repo

**Resolved by evidence — no question needed:**

- **No new dependency.** `serde_json` is already a direct dep (`Cargo.toml:10`), and ADR 0001 names it as the serialization path. Compact per-line output needs nothing new. ADR 0001's "adding a runtime dependency requires a superseding ADR" clause is not triggered.
- **ADR 0002 is mandatory, not optional.** ADR 0001 line 23–25 states a machine-readable output format "is a separate compatibility surface and would need its own ADR." CONTRIBUTING line 4–5 repeats it. The deliverable is ADR + code, not code alone.
- **Two doc obligations fire at implementation time, not release time.** CONTRIBUTING line 3: every new flag needs a line in the README command list. CONTRIBUTING line 7–8: a `## Unreleased` CHANGELOG entry at the time of the change.
- **This work is sanctioned.** `docs/status.md:5` — feature-frozen *apart from output-format work*.
- **Release is out of scope.** `docs/release-process.md:11` and `scripts/publish.sh` — publishing is permanent and irreversible. I will not touch version numbers, `preflight.sh`, or `publish.sh` unless you separately ask.

**Contradicted premise — my own prior turn:**

My Option A sample used `"created_at"`. ADR 0001 line 13–14 deliberately rejected that name: the field is `written`, "chosen over `created_at` because a note can be edited, and `written` records the original writing." You ratified the *shape* of Option A; I'm not going to quietly carry a field name past an ADR that argued against it. That's the first fork below.

**Current-state facts that shape the remaining forks:**

- `search` prints timestamp + body only; `list` prints timestamp + body + tags (`src/main.rs:29,34`). The two commands do not have a common record shape today.
- `add` prints `added note at <ts>` (`src/main.rs:25`).
- Errors are `.expect("write note")` (`src/main.rs:24`) — a panic to stderr, exit 101. There is no error contract to inherit.

## Branch map

- **B1** — timestamp field name in output *(asking now)*
- **B2** — which commands get `--json` (does `add` count?)
- **B3** — error and exit-code contract in JSON mode
- **B4** — flag placement: `notekeeper --json list` vs `notekeeper list --json`
- **B5** — whether `search` records carry `tags`, matching `list`

Carried as ratified from your Option A pick: one compact JSON object per line, `schema` tag on every record (Option A has no envelope to hold it), no result count.

---

**B1 — What is the timestamp field called in `--json` output?**

**Option 1 — `written` (recommended).** Output mirrors the on-disk name.
- *Changes:* records read `{"schema":"notekeeper/v1","body":"buy milk","tags":["errand"],"written":"2026-06-01T00:00:00Z"}`. No mapping layer in the serializer.
- *Benefit:* ADR 0001's naming argument is about the concept, not the medium — if `created_at` was wrong on disk because notes can be edited, it is wrong on the wire for the same reason. One name for one concept across the whole project.
- *Cost:* it puts an internal name on a permanent public surface. ADR 0001 explicitly reserved the option to keep the two decoupled, and you'd be spending that option. If storage ever renames the field, the output must keep emitting `written` regardless.
- *Rejected if:* you expect to rename the storage field, or you want the output layer to be provably a mapping rather than a passthrough.

**Option 2 — `created_at`.** Output uses the conventional name.
- *Changes:* the serializer gets an explicit rename, and ADR 0002 documents disk→wire as a translation table.
- *Benefit:* consumers get the name they expect from JSON tooling, and the decoupling ADR 0001 anticipated is enforced structurally — you cannot leak a storage rename by accident.
- *Cost:* two names for one concept, and the name you'd be shipping is the one ADR 0001 examined and rejected on semantic grounds. Anyone reading both documents hits the contradiction.
- *Rejected if:* you agree with ADR 0001 that "created" is the wrong word for this value.

**Option 3 — `written`, plus ADR 0002 freezing output names independently.** Same bytes as Option 1, but ADR 0002 states the output names are frozen on their own terms and a future storage rename does not propagate.
- *Changes:* nothing in the code versus Option 1; one extra paragraph in the ADR.
- *Benefit:* keeps the semantically correct name and still buys back the decoupling, at documentation cost only.
- *Cost:* the guarantee is prose, not structure — a careless `#[derive(Serialize)]` reuse could still leak a rename.
- *Rejected if:* you want the decoupling enforced by a type, not a promise.

Which one?