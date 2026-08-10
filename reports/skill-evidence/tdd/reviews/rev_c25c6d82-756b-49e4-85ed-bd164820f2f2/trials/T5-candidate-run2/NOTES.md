# TDD record — `load_profile` rejects a payload with no `email`

Workspace root (`$W`) for every command below:

```
/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T5-B-run2
```

## Seams under test, written down before any test

| Seam | Authority | Status |
| --- | --- | --- |
| `profiles.load_profile(payload: dict) -> dict` — module-level public function | `TASK.md`, "Requirement for `load_profile(payload: dict) -> dict`" | Ratified by the task statement; no question to the user needed |
| `profiles.ProfileError` — public exception type, observed via the type raised and its `str()` message | `TASK.md`: "`ProfileError` also lives in `profiles.py`" and "the message on the raised error names the missing field" | Ratified by the task statement |

The task fixes the interface, so the seam was already agreed and I did not ask.
No test is written at any other boundary: nothing imports a private name, no
monkeypatching, no side channel.

## TDD evidence row

One behaviour, one slice. `TASK.md` states that rejecting and naming the field
are **one** behaviour, so both halves live in a single test rather than two.

| Field | Value |
| --- | --- |
| Seam authority | `TASK.md` — requirement for `load_profile`, which names both `load_profile` and `ProfileError` as the public interface |
| Test file | `test_profiles.py` |
| Exact test selector | `test_profiles.py::test_payload_without_email_is_rejected_with_error_naming_the_missing_field` |
| Observed public entry point | `load_profile({"name": "Ada Lovelace", "age": 36})`, imported as `from profiles import ProfileError, load_profile`; the refusal is observed as the raised `ProfileError` and its `str(excinfo.value)` message |
| Red command | `cd $W && python3 -m pytest test_profiles.py::test_payload_without_email_is_rejected_with_error_naming_the_missing_field -q` |
| Intended red failure | The named public entry point does not exist yet (`profiles.py` was empty), so the test cannot reach a passing state |
| Observed red failure | `ERROR collecting test_profiles.py` — `ImportError: cannot import name 'ProfileError' from 'profiles'`, raised at `test_profiles.py:3`. `1 error in 0.01s`. Matches the intent |
| Minimal implementation | `profiles.py`: `ProfileError(Exception)`, plus a single `if "email" not in payload: raise ProfileError("missing required field: 'email'")` guard in `load_profile` |
| Green command | `cd $W && python3 -m pytest test_profiles.py::test_payload_without_email_is_rejected_with_error_naming_the_missing_field -q` |
| Green result | `1 passed in 0.00s` |
| Green command (full suite) | `cd $W && python3 -m pytest test_profiles.py -q` |
| Full-suite result | `1 passed in 0.00s` |

Each command position above is a complete runnable command that replays on its
own; no position back-references another.

## Sensitivity check — is the golden discriminating?

The red was an `ImportError`. That proves the test runs against absent code, but
it does **not** prove the assertion can tell a correct implementation from a
wrong one — an `ImportError` red would look identical for a much weaker test.
`method/tests.md` requires that a golden which would still pass after the
forbidden behaviour is substituted must be replaced, so I substituted each
forbidden behaviour that `TASK.md` names and confirmed the test fails on both.

Replay: copy `test_profiles.py` into a scratch dir alongside the mutant
`profiles.py` below, then `cd <scratch dir> && python3 -m pytest test_profiles.py -q`.
(The scratch dir was deleted afterwards; the real files were never mutated.)

| Mutant | Forbidden behaviour, quoting `TASK.md` | Mutant body | Result |
| --- | --- | --- | --- |
| A | "an error that does not name the field does not satisfy this requirement" | `if "email" not in payload: raise ProfileError("invalid payload")` | **Killed** — `AssertionError` on `"email" in str(excinfo.value)`, `where ProfileError('invalid payload')`. `1 failed` |
| B | "naming a field without rejecting does not either" | `if "email" not in payload: return {"error": "missing required field: 'email'"}` | **Killed** — `Failed: DID NOT RAISE <class 'profiles.ProfileError'>`. `1 failed` |

Both halves of the single behaviour are therefore load-bearing in the assertion.

The expected value is independent, not recomputed the way the code computes it:
the literal `"email"` comes from the requirement text, not from the payload or
from any expression over the implementation. The test never reads
`profiles.py`'s message constant.

## Public invariant coverage — ingress enumeration

`method/tests.md` requires enumerating every exported entry point that can
create or admit the value, and probing each path that could bypass validation.
Enumerated from the built module rather than by eye:

```
cd $W && python3 -c "import profiles; print(sorted(n for n, v in vars(profiles).items() if not n.startswith('_')))"
→ ['ProfileError', 'load_profile']
```

`load_profile` is the only exported path that admits a payload; `ProfileError`
is the refusal type, not an admitting path. There is exactly one ingress, so
there is no second path that could bypass the `email` check and no bypass probe
to add. If a parser, factory, or deserializer is added later, this invariant
needs a test at that path too.

**Refusal-path non-observation:** `tests.md` also asks that refusal paths assert
non-observation of protected payloads. My judgement is that this does not bite
here — the requirement designates no field as secret, and the test payload holds
no protected data — so I asserted only the refusal and the field name. Flagging
it rather than silently dropping it: the requirement's phrase "without
inspecting the payload" is a caller-ergonomics constraint, not a secrecy one, so
if a protected-field class is intended, this test does not cover it.

## Pre-review reconciliation

Changed-test inventory derived from the repository's native test structure.
There is no VCS history to diff against in this workspace and I am barred from
running git, so the scope is established another way: `test_profiles.py` was
0 bytes at task start (verified before editing), so every collected test in it
is new in this change.

```
cd $W && python3 -m pytest test_profiles.py --collect-only -q
→ test_profiles.py::test_payload_without_email_is_rejected_with_error_naming_the_missing_field
→ 1 test collected in 0.00s
```

One row per changed test selector — no file- or class-level summarising:

| Test file | Exact selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `test_profiles.py` | `test_payload_without_email_is_rejected_with_error_naming_the_missing_field` | `profiles.load_profile` called with a dict; `profiles.ProfileError` type and its `str()` message | `TASK.md` requirement for `load_profile` | The single row above |

Both-direction difference check:

- Inventory identities not in reconciliation: **none**.
- Reconciliation identities not in inventory: **none**.

Both differences are empty, so the gate passes. The row's claim is public
behaviour observed through the ratified boundary, not a private invariant — the
test imports only the two public names and observes only the raised exception
and its message.

## Review re-entry rows

None. No `/code-review` pass has run against this change, so there are no
findings marked `TDD re-entry required: yes` and no review-reentry rows to
reconcile. If a review runs later, each actionable finding gets its own appended
row keyed by pass, axis, and finding — never an edit to the row above.

## Mocking

No mocks, fakes, or patches. `method/mocking.md` limits mocking to system
boundaries; this slice touches no external API, database, clock, randomness, or
filesystem, and mocking anything here would mean mocking code I control.

## Deliberate scope boundary in `profiles.py`

`load_profile`'s accepted-payload path raises `NotImplementedError` rather than
returning a value. No test authorizes what a loaded profile is, and guessing
would decide unratified questions — copy vs. alias of the input dict,
normalisation, defaults — as untested, speculative code. Per "only enough code
to pass it", the module contains just the rejection guard, and the unimplemented
path fails loudly instead of silently returning `None` against the `-> dict`
annotation the task specifies. The next slice replaces that line.
