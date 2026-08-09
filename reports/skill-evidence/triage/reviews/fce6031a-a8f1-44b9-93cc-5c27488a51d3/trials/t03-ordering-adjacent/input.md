# Trial t03: ordinary bug verification

The maintainer asks: "Triage issue #22 and recommend the next state. Do not mutate the tracker
until I answer."

- Labels: `bug`, `needs-triage`.
- Claim: `skills inspect --json` emits plain text rather than JSON when the requested skill does
  not exist.
- Code snapshot: success responses use the JSON serializer; the not-found branch writes a human
  sentence directly.
- Reproduction snapshot: invoking the documented missing-skill case returns exit code 1 and the
  bytes `skill not found` on stdout; JSON parsing fails.
- No equivalent out-of-scope decision or existing repair exists.

Perform the held workflow through its first maintainer wait. Preserve the distinction between code
inspection and claim verification.
