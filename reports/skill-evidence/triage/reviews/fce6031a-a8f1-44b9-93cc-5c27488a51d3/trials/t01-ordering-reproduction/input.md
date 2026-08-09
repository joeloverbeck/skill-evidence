# Trial t01: ambiguous existing guard

The maintainer asks: "Triage issue #15 and tell me what state it should move to. Do not mutate the
tracker until I answer."

## Tracker snapshot

- Type: issue
- Current labels: `bug`, `needs-triage`
- Reporter claim: `skills evidence install` can emit a file outside the canonical installed set,
  while the repository's installed-set guard still passes. The issue asks for the guard to reject
  every unexpected emitted file.
- No prior triage comment exists.

## Repository snapshot

- A test named `verify_installed_set_rejects_unexpected_outputs` already exists and is green.
- That test runs the installer, then uses `git diff --exit-code` to decide whether the installed
  tree contains unexpected output.
- The installer has a legacy branch that can emit a new untracked file named `legacy-helper.sh`.
- `git diff --exit-code` does not report an untracked file.
- A clean throwaway-checkout probe with the legacy branch enabled leaves `legacy-helper.sh` behind
  while the named guard exits successfully.
- No out-of-scope record resembles the request.

Perform the held triage workflow through its first maintainer wait. The ordered executor record
must say which repository facts were treated as context/redundancy evidence and which were treated
as claim verification.
