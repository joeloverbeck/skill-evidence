# TDD evidence — `load_profile`

Workspace root (`$W` below, always absolute):
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T5-B-run1`

## Seams under test, written down before any test was written

| Seam | Authority | Status |
| --- | --- | --- |
| `profiles.load_profile(payload: dict) -> dict`, called with a plain dict | `TASK.md` → "Requirement for `load_profile(payload: dict) -> dict`" names the module, the function, the signature, and the error type | Ratified — no seam question raised |
| `profiles.ProfileError`, observed as the raised type and via `str(err)` | `TASK.md` → "`ProfileError` also lives in `profiles.py`" and "the message on the raised error names the missing field" | Ratified |

The task document is the authoritative requirement here, so the seams were already agreed and I did
not need to ask. No other seam is touched: the tests import only these two public names, and nothing
reaches into module internals, private helpers, or a side channel.

## Command key

`PYT <target>` = `python3 -m pytest <target> -q`

`PYT` and `$W` are defined once here for this whole evidence document. Expanded literally, the
whole-suite position reads:

```
python3 -m pytest /home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T5-B-run1/test_profiles.py -q
```

(verified by replaying it from a different working directory: `2 passed`).

Every command position below is `PYT` applied to an absolute target, so each one replays on its own
from any working directory (pytest's prepend import mode puts the test file's directory on
`sys.path`, which is what resolves `import profiles`). The green position is listed separately from
the red even where the command text is identical.

---

## Cycle 1 — a payload with no `email` is rejected, and the error names the field

- **Behaviour:** rejection and field-naming as one behaviour, per `TASK.md` ("an error that does not
  name the field does not satisfy this requirement, and naming a field without rejecting does not
  either").
- **Seam authority:** `TASK.md`, requirement section (row 1 and 2 of the seam table).
- **Test file:** `$W/test_profiles.py`
- **Test selector:** `test_load_profile_rejects_payload_missing_email_and_names_the_field`
- **Observed public entry point:** `from profiles import ProfileError, load_profile`; call
  `load_profile({"id": "u-17", "display_name": "Ada Lovelace"})`; observation through
  `pytest.raises(ProfileError)` and `str(excinfo.value)`. Public interface only.

**Red — structural (preceding observation):**

`PYT $W/test_profiles.py::test_load_profile_rejects_payload_missing_email_and_names_the_field`

- Intended failure: `profiles.py` is empty, so the import of the two public names fails and the
  behaviour cannot exist yet.
- Observed failure: `ImportError: cannot import name 'ProfileError' from 'profiles'` (collection
  error). Matches intended.

**Red — behavioural (the red for this row):** after adding only `ProfileError` and a `load_profile`
that returns its argument, so that the failure is about behaviour rather than missing symbols:

`PYT $W/test_profiles.py::test_load_profile_rejects_payload_missing_email_and_names_the_field`

- Intended failure: the payload is not rejected, so no `ProfileError` is raised.
- Observed failure: `Failed: DID NOT RAISE <class 'profiles.ProfileError'>`. Matches intended.

**Minimal implementation:** an unconditional
`raise ProfileError("profile payload is missing required field: 'email'")`. This is deliberately the
least code that passes this one test — the `"email" not in payload` guard was **not** written here,
because no test yet demanded it. Cycle 2 triangulates it in.

**Green:**

`PYT $W/test_profiles.py::test_load_profile_rejects_payload_missing_email_and_names_the_field`

- Result: `1 passed`.

---

## Cycle 2 — the rejection is scoped to the missing field

- **Behaviour:** a payload that carries `email` is not rejected, and a dict comes back.
- **Seam authority:** `TASK.md`, same requirement sentence — the qualifier in "A payload with **no**
  `email` key is rejected", plus the signature `load_profile(payload: dict) -> dict`.
- **Authority limit, recorded honestly:** `TASK.md` fixes *that* a dict is returned but not *what is
  in it*. The test therefore asserts only `isinstance(result, dict)` and does not invent a
  return-content contract. This cycle is the one place I went past the literal rejection sentence;
  the justification is that after cycle 1 the suite was equally well satisfied by code that rejects
  *every* payload, which the "with no `email` key" qualifier rules out. Same ratified seam, new case.
- **Test file:** `$W/test_profiles.py`
- **Test selector:** `test_load_profile_accepts_payload_that_has_an_email`
- **Observed public entry point:** `load_profile({"id": "u-17", "display_name": "Ada Lovelace",
  "email": "ada@example.com"})` return value. Public interface only.

**Red:**

`PYT $W/test_profiles.py::test_load_profile_accepts_payload_that_has_an_email`

- Intended failure: cycle 1's unconditional raise rejects a payload that does have an email.
- Observed failure: `profiles.ProfileError: profile payload is missing required field: 'email'`.
  Matches intended.

**Minimal implementation:** guard the raise with `if "email" not in payload:` and return `payload`.

**Green:**

`PYT $W/test_profiles.py::test_load_profile_accepts_payload_that_has_an_email`

- Result: `1 passed`.

Whole-suite confirmation: `PYT $W/test_profiles.py` → `2 passed`.

---

## Discriminating-golden / sensitivity checks

The expected value `"email"` is a literal taken from the requirement, not recomputed the way the
code computes it, so the assertion is not tautological. To confirm the golden actually discriminates,
each rejected implementation below was substituted into a throwaway copy of `profiles.py` (in
`$W/.sensitivity-probe/`, alongside a copy of the test file; directory deleted afterwards) and the
suite was re-run.

| Probe | Substituted implementation | Rejected alternative it represents | Observed |
| --- | --- | --- | --- |
| A | `raise ProfileError("invalid profile payload")` | rejects but names no field | cycle-1 test **failed** |
| B | `return {"error": "missing email"}` | names the field but does not reject | cycle-1 test **failed** |
| C | `raise ProfileError("missing required field: 'display_name'")` | names the wrong field | cycle-1 test **failed** |
| D | final code, message rewritten to `invalid profile payload` | regression of the naming half only | cycle-1 test **failed**, cycle-2 test passed |
| E | final code, guard rewritten to `if False:` | regression of the rejection half only | cycle-1 test **failed**, cycle-2 test passed |

The payload in the cycle-1 test deliberately carries *other* fields (`id`, `display_name`), which is
what gives probes A and C somewhere to go wrong: an implementation that echoes a field it can see, or
stays generic, cannot pass. Probes D and E each break exactly one half of the single required
behaviour and each is caught, and neither is masked by the other test.

## Pre-review reconciliation

Scoped change: `profiles.py` and `test_profiles.py` were both empty (0 bytes) at the start of this
task, so the scoped diff is "empty file → current content" and every collected test is a changed
test. No git command was run to derive this. Inventory derived from the repository's native test
structure:

`python3 -m pytest $W/test_profiles.py --collect-only -q`

Inventory (2 collected):

1. `test_profiles.py::test_load_profile_rejects_payload_missing_email_and_names_the_field`
2. `test_profiles.py::test_load_profile_accepts_payload_that_has_an_email`

Reconciliation rows:

| Test file | Selector | Observed public entry point | Seam authority | TDD evidence row |
| --- | --- | --- | --- | --- |
| `$W/test_profiles.py` | `test_load_profile_rejects_payload_missing_email_and_names_the_field` | `load_profile(dict)` raising `ProfileError`, read via `str(err)` | `TASK.md` requirement section | Cycle 1 |
| `$W/test_profiles.py` | `test_load_profile_accepts_payload_that_has_an_email` | `load_profile(dict)` return value | `TASK.md` requirement section (scope qualifier + `-> dict` signature) | Cycle 2 |

Bidirectional comparison: inventory − reconciliation = ∅; reconciliation − inventory = ∅. Both
differences empty. No test enters through a module, helper, mutable global, or side channel that the
authority did not ratify, and each row proves the public behaviour it claims rather than a private
invariant. Coverage check passes, so the evidence rows above are reconciled.

## Review re-entry rows

None. `/code-review` was not run as part of this task, so there are no findings marked
`TDD re-entry required: yes` and the `Findings fixed during review` ledger is empty; the
bidirectional comparison against appended review-reentry rows is vacuous on both sides. Any later
finding gets a **new** appended row keyed by review pass, axis, and finding — cycles 1 and 2 above
must not be rewritten or extended to absorb one.

## Anti-pattern self-check

- **Mocking:** none. There is no system boundary in this function — no I/O, clock, or network — so
  per `mocking.md` there was nothing legitimate to mock, and no internal collaborator was faked.
- **Implementation coupling:** none. Both tests would survive any rewrite of `load_profile` that
  keeps the two public behaviours.
- **Tautology:** avoided; expected values are literals from the requirement.
- **Horizontal slicing:** avoided. Two vertical slices, each one test → one minimal implementation;
  the cycle-2 test was written only after cycle 1 was green, and it was written *because* cycle 1's
  green exposed the unscoped raise.

## Final state

`profiles.py` — `ProfileError(Exception)`, and `load_profile` raising it with the message
`profile payload is missing required field: 'email'` when `"email" not in payload`, otherwise
returning the payload. Suite: `PYT $W/test_profiles.py` → `2 passed`.
