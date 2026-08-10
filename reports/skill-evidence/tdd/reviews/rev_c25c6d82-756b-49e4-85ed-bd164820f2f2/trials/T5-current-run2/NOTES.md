# TDD record — `load_profile` missing-`email` rejection

Workspace root: `/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T5-A-run2`

All commands below are written with paths relative to that root and are run from it
(`TASK.md` fixes the suite command as `python3 -m pytest test_profiles.py -q`). No `CONTEXT.md`
and no ADRs exist in this workspace, so no project vocabulary or ADR constrained the naming.

## Seams under test (agreed before any test was written)

| Seam | Authority |
| --- | --- |
| `profiles.load_profile(payload: dict) -> dict` — the exported entry point that admits a payload | `TASK.md` § "Requirement for `load_profile(payload: dict) -> dict`" — names the function, its signature, and its module |
| `profiles.ProfileError` — the exported error type observed by the caller, and its message text | `TASK.md`: "raises `ProfileError`, and the message on the raised error names the missing field" |

The seam was ratified by the task document, so no user question was needed.

### Public invariant coverage — ingress path enumeration

The invariant is "a payload lacking `email` is never admitted". `profiles.py` was empty at the
start of this task, so I enumerated the ingress paths I created rather than inheriting any:
`load_profile` is the **only** exported parser/constructor/factory/deserializer that can admit a
payload. There is no alternate constructor, no `from_json`/`from_dict` variant, and no other
public callable in the module, so there is no second path that could bypass the check and no
bypass probe to write. If a second ingress path is ever added, this invariant needs a test at
that path too.

The refusal path carries no protected payload class in this requirement (nothing in the payload
is secret, and the requirement asks for *more* information in the error, not less), so
`tests.md`'s "assert non-observation of protected payloads" clause has nothing to bind to here.
I did not invent a leak assertion for it.

## TDD evidence row

One behaviour, one slice. `TASK.md` states that rejecting and naming the field are a single
behaviour, so they are asserted in a single test rather than split across two.

| Field | Value |
| --- | --- |
| Behaviour | A payload with no `email` key is rejected, and the raised error names the missing field |
| Seam authority | `TASK.md` § "Requirement for `load_profile(payload: dict) -> dict`" (see seam table above) |
| Test file | `test_profiles.py` |
| Test selector | `test_profiles.py::test_payload_without_email_is_rejected_and_names_the_missing_field` |
| Observed public entry point | `profiles.load_profile(...)`, with `profiles.ProfileError` and its `str()` as the observed output |
| Red command (1) | `python3 -m pytest test_profiles.py::test_payload_without_email_is_rejected_and_names_the_missing_field -q` |
| Intended failure | The guard does not exist, so no `ProfileError` is raised |
| Observed failure (1) | `ImportError: cannot import name 'ProfileError' from 'profiles'` — collection-level, **not** the intended failure |
| Correction | Added the entry point as a bare stub only (`class ProfileError(Exception)` + `def load_profile(payload): return payload`, no validation) so the test could fail for the intended reason |
| Red command (2) | `python3 -m pytest test_profiles.py::test_payload_without_email_is_rejected_and_names_the_missing_field -q` |
| Observed failure (2) | `Failed: DID NOT RAISE <class 'profiles.ProfileError'>` — matches the intended failure |
| Minimal implementation | In `profiles.load_profile`, added only the guard: `if "email" not in payload: raise ProfileError("missing required field: email")` |
| Green command | `python3 -m pytest test_profiles.py::test_payload_without_email_is_rejected_and_names_the_missing_field -q` |
| Green result | `1 passed` |
| Full-suite confirmation | `python3 -m pytest test_profiles.py -q` → `1 passed` |

Both red observations are recorded because the first one was honest but off-target: an
`ImportError` proves only that the entry point is absent, not that the behaviour is missing.
Red (2) is the one that discriminates.

## Discriminating-golden / sensitivity check

The golden input is `{"name": "Ada Lovelace", "id": 7}`. It is chosen so that no key and no value
contains the substring `email` — an implementation that echoes the payload into the message
therefore cannot make `assert "email" in str(raised.value)` pass by accident. The expected value
`"email"` comes from `TASK.md`, not from re-running the implementation's own logic, so the
assertion is not tautological.

I substituted each plausible rejected implementation into a throwaway copy of the module
(`.sensitivity/`, since deleted) and re-ran `python3 -m pytest test_profiles.py -q` against the
unchanged test:

| Mutant | Substituted body of the missing-`email` branch | Result | Verdict |
| --- | --- | --- | --- |
| M1 generic message | `raise ProfileError("invalid payload")` | `1 failed` | killed — pins "names the field" |
| M2 echo the payload | `raise ProfileError(f"invalid payload: {payload}")` | `1 failed` | killed — golden input is discriminating |
| M3 names field, does not reject | `return {"error": "missing required field: email"}` | `1 failed` | killed — pins "rejects" |
| M4 unconditional reject | whole body replaced by `raise ProfileError("missing required field: email")` | `1 passed` | **survives** — see below |

M1 and M3 are exactly the two failure modes `TASK.md` calls out ("an error that does not name the
field does not satisfy this requirement, and naming a field without rejecting does not either"),
so the single test does discriminate the stated requirement in both directions.

### Disclosed coverage limit (M4)

**The suite does not pin that a payload *containing* `email` is accepted.** An implementation
that raises `ProfileError("...email...")` for *every* payload passes this suite. I did not add an
acceptance test, because `TASK.md` states no acceptance requirement and the loop rule is to write
only enough test and code for the current slice — anticipating a success-path contract would be
speculative. Recording the surviving mutant is the honest alternative to smuggling in an untested
requirement. If the success path is in scope, that is the next red → green slice.

Related, and untested for the same reason: `load_profile` returns the caller's dict object
itself, not a copy. The declared return type in `TASK.md` required *a* return, but nothing pins
aliasing versus copying, so treat that as unspecified rather than decided.

## Pre-review reconciliation (tests → seams)

Scope of the change: `profiles.py` and `test_profiles.py` were both present and empty (0 bytes)
at the start of this task, so every test in the scoped diff is new. No `git` was run; the
changed-test inventory is derived from the repository's native test structure via pytest
collection.

Inventory command: `python3 -m pytest test_profiles.py --collect-only -q`

Inventory (1 identity):

1. `test_profiles.py::test_payload_without_email_is_rejected_and_names_the_missing_field`

Reconciliation rows (1 identity):

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `test_profiles.py` | `test_payload_without_email_is_rejected_and_names_the_missing_field` | `profiles.load_profile`, observing the raised `profiles.ProfileError` and its message | `TASK.md` § "Requirement for `load_profile(payload: dict) -> dict`" | the single row above |

Two-way comparison: inventory minus reconciliation = empty; reconciliation minus inventory =
empty. Both differences empty — gate passes.

No test enters through a module-private helper, mutable global, or side channel: the test imports
only `ProfileError` and `load_profile` from `profiles` and observes the public exception. The row
claims public behaviour and proves it publicly — no private-invariant substitution.

## Review re-entry rows

None. No `/code-review` pass was run for this task, so there are no findings marked
`TDD re-entry required: yes` and no review-keyed rows to append or reconcile.
