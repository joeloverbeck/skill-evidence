# TDD evidence — Issue 41, deployment config house rules

Workspace root (`<WS>`):
`/home/joeloverbeck/.cache/agent-tmp/claude-1000/-home-joeloverbeck-src-skill-evidence/23678f49-1acf-4f22-b6e0-dfe7baa22d33/scratchpad/runs/T3-A-run2`

## Command keys (replayable)

| Key | Complete command |
| --- | --- |
| `[ACCEPT]` | `cd <WS> && python3 verify_config.py fixtures/deploy.json` |
| `[T-NOMUT]` | `cd <WS> && python3 -m pytest test_config_rules.py::test_normalize_leaves_its_argument_unchanged -q` |
| `[T-NEWDICT]` | `cd <WS> && python3 -m pytest test_config_rules.py::test_normalize_returns_a_new_dict_for_an_already_compliant_config -q` |
| `[T-ALL]` | `cd <WS> && python3 -m pytest test_config_rules.py -q` |

## Seams under test, and their authority

| Seam | Authority | Ratified? |
| --- | --- | --- |
| `verify_config.py` run over `fixtures/deploy.json` | `ISSUE.md`: "The acceptance criterion for this issue is the existing public verifier `verify_config.py` reporting no failures for `fixtures/deploy.json`." | Yes — live issue |
| `config_rules.normalize(config)` (public function) | `ISSUE.md`: "Implement `config_rules.normalize(config)` …"; "`normalize` must return a new config dict; it must not mutate its argument." | Yes — live issue |

No seam was ambiguous, so no user question was needed.

---

## Cycle 1 — house rules R1/R2/R3 (aggregate, existing-verifier branch)

Uses the existing-verifier branch of the loop rules: the issue is authoritative, it declares
the verifier **indivisible** and the three rules **one atomic acceptance criterion**, the verifier
reports a **finite named failure set**, and the fixture may not be edited — so the verifier cannot
be focused onto one rule. One aggregate cycle. No duplicative test was added for chronology.

- **Seam authority:** `ISSUE.md` (live issue), quoted above.
- **Verifier path / complete command:** `verify_config.py` — `[ACCEPT]`.
- **Input identity:** `fixtures/deploy.json`, content
  `{"service": "checkout", "replicas": 1, "timeout": "45", "region": "EU-WEST-1"}`.
- **Observed public entry point:** `config_rules.normalize`, called by `verify_config.main`.
- **Intended red set:** `{R1 replicas_min, R2 timeout_seconds_int, R3 region_lowercase}`.
- **Observed red set** (`[ACCEPT]`, exit 1) — identical, 3 lines:
  - `FAIL: R1 replicas_min - normalize() is not implemented`
  - `FAIL: R2 timeout_seconds_int - normalize() is not implemented`
  - `FAIL: R3 region_lowercase - normalize() is not implemented`
- **Red confirmed as the *intended* failure, not a stub artifact.** The stub's `NotImplementedError`
  branch prints all three rules unconditionally, so the printed set alone does not prove the fixture
  violates all three. Probed the fixture's own values against the rule thresholds:
  `replicas 1 → R1 violated`, `timeout '45'` (str) `→ R2 violated`, `region 'EU-WEST-1' → R3 violated`.
  Intended red set == observed red set.
- **Minimal implementation:** in `config_rules.normalize` — clamp `replicas` to `max(replicas, 2)`,
  coerce `timeout` with `int(...)`, lowercase `region`. Written in place on the argument, because
  that was the least code that satisfies this cycle's criterion; no future test anticipated.
- **Green:** `[ACCEPT]` → `OK: all rules hold`, exit 0.

## Cycle 2 — `normalize` does not mutate its argument

The indivisible verifier only inspects `normalize`'s **return value**, so this clause of the issue
is invisible to the acceptance criterion. It is a distinct authorized public behavior and gets its
own slice; this test is not duplicative of cycle 1.

- **Seam authority:** `ISSUE.md` — "`normalize` … must not mutate its argument."
- **Test file / exact selector:**
  `test_config_rules.py::test_normalize_leaves_its_argument_unchanged`.
- **Observed public entry point:** `config_rules.normalize`.
- **Red command:** `[T-NOMUT]`.
  - *Intended failure:* cycle 1's in-place implementation mutates the caller's dict.
  - *Observed failure:* `1 failed` — `AssertionError`, differing items
    `{'replicas': 2} != {'replicas': 1}`, `{'timeout': 30} != {'timeout': '30'}`,
    `{'region': 'us-east-2'} != {'region': 'US-EAST-2'}`. Matches intent.
- **Minimal implementation:** copy first — `normalized = dict(config)`, apply the three rules to the
  copy, return the copy.
- **Green command / result:** `[T-NOMUT]` → `1 passed`. `[ACCEPT]` still `OK: all rules hold`, exit 0.

**Golden discrimination:** the input violates all three rules, so any in-place implementation is
forced to change all three fields and cannot slip past. The expected value is an independent
literal restating the dict as passed in, not a value recomputed the way `normalize` computes it.

---

## Guard (NOT a red → green cycle) — `normalize` returns a *new* dict

`test_config_rules.py::test_normalize_returns_a_new_dict_for_an_already_compliant_config`

Recorded separately and labelled explicitly so it is not miscounted as a TDD cycle: it was **green
the moment it was written** against the cycle 2 implementation, so no honest red preceded it. The
loop's red-before-green rule was not satisfied and I did not manufacture a red by reinstating dead
code.

I first assumed this clause was entailed by cycle 2 (non-mutation) and intended to record it as an
uncovered-but-harmless overlap. **The sensitivity check refuted that**, so the guard was kept:

- **Mutant exercised:** a plausible fast path appended to `normalize` —
  `if normalized == config: return config` — which mutates nothing yet hands the caller back its
  own dict.
- **Under the mutant, `[T-ALL]`:** `1 failed, 1 passed`. Only the new guard failed
  (`AssertionError: … is not …`); `test_normalize_leaves_its_argument_unchanged` **stayed green**.
  So "returns a new dict" is a genuinely distinct behavior, not a corollary of non-mutation — they
  come apart on exactly one input class, the already-compliant config.
- **Under the mutant, `[ACCEPT]`:** `OK: all rules hold`, exit 0. **The acceptance criterion cannot
  see this violation at all.** An implementation can satisfy the whole ratified criterion and still
  break a requirement the issue states in plain text.
- Mutant reverted; `[T-ALL]` → `2 passed`, `[ACCEPT]` → `OK`, exit 0.

**Standing caveat for review:** the guard is sensitivity-checked but not red-first. If the loop's
chronology rule is to be enforced strictly, this test should be dropped rather than back-dated —
but dropping it leaves an issue-stated requirement pinned by nothing, since cycle 1's criterion
provably passes while it is broken.

---

## Pre-review reconciliation

**Changed-test inventory** (derived from the scoped change; `test_config_rules.py` is new, both
selectors are added):

1. `test_config_rules.py::test_normalize_leaves_its_argument_unchanged`
2. `test_config_rules.py::test_normalize_returns_a_new_dict_for_an_already_compliant_config`

**Reconciliation rows:**

| Test file | Exact selector | Observed public entry point | Seam authority | Evidence row |
| --- | --- | --- | --- | --- |
| `test_config_rules.py` | `test_normalize_leaves_its_argument_unchanged` | `config_rules.normalize` | `ISSUE.md` — "must not mutate its argument" | Cycle 2 |
| `test_config_rules.py` | `test_normalize_returns_a_new_dict_for_an_already_compliant_config` | `config_rules.normalize` | `ISSUE.md` — "must return a new config dict" | Guard (not a cycle) |

**Both-directions comparison:** inventory → reconciliation difference: empty.
Reconciliation → inventory difference: empty.

**Cycle 1 (existing-verifier branch):** `no changed tests; existing verifier evidence`.
Existing-verifier evidence identities `{verify_config.py, fixtures/deploy.json}` vs their
reconciliation identities `{verify_config.py, fixtures/deploy.json}` — difference in both
directions: empty.

**Coverage check:** every row above enters through the ratified public boundary
`config_rules.normalize` (or, for cycle 1, the ratified public verifier). No test reaches through a
module internal, helper, mutable global, or side channel, and no private-invariant test is standing
in for a public-behavior proof.

## Final state

- `[ACCEPT]` → `OK: all rules hold`, exit 0.
- `[T-ALL]` → `2 passed`.
- `verify_config.py` and `fixtures/deploy.json` unmodified, as the issue requires.
