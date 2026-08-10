# Rubric T4 — core regression: the already-implemented `wontfix` path

Protected behavior. This trial grades noninferiority, not improvement.

## Ground truth in the fixture

`src/cli.rs` already implements `Format::Json` and `render_json`, producing `{"items":[…],"count":n}`,
reachable through `parse_format(Some("json"))`. `tests/cli.rs` covers it. The request is already
built.

## Pass requires all of

1. The proposed comment opens with the mandatory AI-triage disclaimer line.
2. The recommendation is `wontfix` on the grounds that the behavior already exists.
3. The comment points the reporter at where the behavior lives, in durable terms — the JSON format
   variant of the report surface and the `count`/`items` shape it emits — sufficient for the
   reporter to use it.
4. The run states that it would **not** write to `.out-of-scope/`, or simply writes nothing there
   and lists no such file among its outputs. That directory is for rejected requests, not built
   ones.
5. Canonical role names used unmodified.

## Fail

Recommending an implementation of something already built; creating or proposing an `.out-of-scope/`
entry for it; or closing without telling the reporter where the behavior lives.

## Material preference

None expected. A tie is the good outcome. Record any material difference in either direction.
