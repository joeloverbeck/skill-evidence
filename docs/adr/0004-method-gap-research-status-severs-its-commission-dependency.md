# method-gap-research-status severs its dependency on the commission package

Status: accepted (2026-08-08, GitHub [#8](https://github.com/joeloverbeck/skill-evidence/issues/8))

`method-gap-research-status` is one of the four packages this crate installs, and two of its steps
hard-depend on [`commission-method-gap-research`](../../.claude/skills/commission-method-gap-research/SKILL.md),
which the crate never ships, never installs, and never version-checks. Step 2 is a mandatory
*read completely* of two of that skill's references; step 4's only non-empty terminal emits a
`$commission-method-gap-research` invocation the skill forbids shortening. For `playbench`,
`mundifold`, and `what-we-bring-home` the links resolve because all three hand-copied that skill.
For a consumer installing from crates.io they do not resolve and cannot be made to. **The package
becomes self-contained**: the decision rules step 2 actually consumes are inlined into the
package's own `references/`, and step 4 emits its census-only mandate as a mandate rather than as
an invocation of a package that may not exist. `commission-method-gap-research` stays a private
hand-copied skill, outside this crate's contract.

## Why

**The shipped surface promises something the distribution channel cannot deliver.**
[`../principles/mission-and-scope.md`](../principles/mission-and-scope.md) states the primary
outcome as running the lifecycle *"without hand-copying any of it"*. This package is runnable only
by hand-copying, which inverts that outcome for the one package that names it.

**There is a live defect that does not depend on distribution at all.** Step 2 orders both
references read *completely*. Roughly fifty of those lines instruct building a claim-instrument
map, mining candidate areas, and selecting external vantages — work this skill's own hard
boundaries forbid it in terms: *"never semantically audit every target or build a claim-instrument
map. The target-specific commission owns that work."* That misfires today in every repository
where the file resolves, which is all four. Severing the dependency is therefore a correctness
repair, not only a packaging one.

**The gap is far smaller than the link suggests, and this is what makes the decision cheap.** The
two references total 277 lines. Measured against the already-shipped `references/selection-rules.md`,
most of what step 2 consumes is already there: the concern-routing table is mirrored section for
section, the eight `commission now` cadence signals map essentially one-to-one onto the ordered
positive signals, and the unversioned-cadence proxy survives as *"Commit count is not
meaningful-version count."* What is genuinely absent is roughly twenty-five to thirty-five lines —
the six lineage states, the discriminators for what counts as overlapping work, the
lineage-discovery scoping rules, and the false-gap rejection list that eligibility gate 4 needs and
nothing shipped states. Everything else is either already present or commission-side work the
selector may not do.

**The code already drew this boundary; only the reference did not follow it.** The selector half is
lifecycle machinery — `method_gap_research_inventory` reads event streams through the shared
skill-evidence authority, `Host` mints the `method-gap-research-status.inventory/v1` projection
namespace, the CLI compiles the subcommand, and a frozen fixture golden pins the output. The
commission half touches none of it: its package contains no reference to `skill-evidence`,
`events.jsonl`, or gate status, no reference to `method-gap-research-status`, and none of the
`{{command}}` / `{{cargo_package}}` / `{{namespace}}` tokens a shipped package is rendered with.
Its premise is the lifecycle; its mechanism is not.

**This defect was already on the record.**
[ADR 0003](0003-no-new-instrument-for-conformance-only-evidence.md) §*Considered options* recorded
it while withdrawing an objection built on it: *"the missing package is a real pre-existing
cross-reference defect worth its own issue, not a reason to decline this."* #8 is that issue and
this ADR is its answer.

## Considered options

**(a) Sever the dependency.** Chosen. Inline into the package's own `references/` only the decision
rules step 2 consumes, and reword step 4's terminal to carry the mandate without naming a package.
It matches the boundary the code already drew, adds no runtime dependency, strands nothing, and is
the only option that *structurally* cannot overwrite a consumer's localized copy of the commission
skill — the crate never ships that package, so the installer never touches it.

**(b) Ship `commission-method-gap-research` as a fifth package.** The most direct answer to the
dependency, and the one with real evidence behind it: the commission is genuinely used, carrying 47
recorded events in `playbench` and 1 in `mundifold`. Rejected on three counts. It needs
host-specific content no template token expresses — one paragraph justifies its Node validator
against each host's own constitution, which the consumers have genuinely localized (`mundifold`
cites `technical-boundaries.md` §Domain sovereignty and *"world or character semantics"*;
`playbench` cites P30 and *"game semantics"*) — and [`../../src/assets.rs`](../../src/assets.rs)
states the test itself: *"a package that needed a third token would be a sign the package, not the
template, has the wrong boundary."* It would make Node 20+ a runtime dependency of a Rust crate's
installed surface for every consumer — the constraint that
[`../principles/consumer-contract.md`](../principles/consumer-contract.md) §*Shipped packages carry
instructions, not executables* now states. And it would break all three consumers' next install until
`--force`, where `--force` overwrites `mundifold`'s correct localization with no uninstall to undo
it. The observed pressure is for the *skill*, not for its *distribution*: all three consumers
already obtained it by hand-copy and none is blocked.

**(c) Declare the dependency without satisfying it.** Cheapest and honest about the requirement,
but it converts a broken happy path into a documented dead end while leaving a shipped package a
public consumer still cannot complete. It also leaves the live read-completely defect untouched.

**(d) Retire `method-gap-research-status` from the shipped set.** One piece of evidence favours it:
the selector has **zero recorded uses in any of the four repositories**, and that absence is
genuine rather than a by-design consequence of being read-only — its equally read-only sibling
`skill-evolution-status` has 39 events captured in `playbench` alone, recorded externally by
`skill-evidence-capture`. Rejected anyway, because the installer has no uninstall:
[`../principles/consumer-contract.md`](../principles/consumer-contract.md) makes retiring an
installed package a breaking change to the installed-asset surface, and
[`../releasing.md`](../releasing.md) §7 requires naming the exact directories each consumer must
delete by hand. Retirement strands the package in every consumer forever and discards working Rust
machinery and a frozen fixture golden to do it.

## Consequences

- **The inlined rules get a second home and can drift from the commission's classifier silently.**
  This is the accepted cost of (a). Nothing detects that drift today and nothing is being built to;
  the mitigation is that the inlined subset is scoped to what a read-only selector needs rather
  than copied wholesale, so there is less surface to drift.
- **A link-integrity test now constrains every shipped package.** No installed asset may cite a
  file outside the installed-asset set, in either citation style the packages use — markdown links
  and backticked package-relative paths. This is the guardrail whose absence let the defect ship:
  nothing in the repository could previously detect a shipped file citing an unshipped one.
- **Editing the package changes its content hash**, which resets `qualifying_uses_on_current_hash`
  for this skill. Verified impact is nil — no repository holds a `method-gap-research-status`
  evidence stream to reset.
- **No consumer's copy of the commission skill is touched.** It is not an installed asset, so the
  installer does not see it, and `mundifold`'s localization is safe by construction rather than by
  care.
- **A patch release, and consumers install with `--force`.** [`../releasing.md`](../releasing.md)
  §1: an installed package's content changed, none was added or retired, no public API and no
  recorded-event shape moved. §2 does not engage; §7's orphan problem does not engage.
- **The commission stays outside the contract.** It remains a private hand-copy in the repositories
  that want it, with no version relationship to this crate and no route to obtain it from a
  release. That is now the deliberate state rather than an unnoticed one.
- **What could reopen this.** A consumer that needs the commission itself distributed rather than
  hand-copied, which would mean solving the constitutional-paragraph and Node-runtime problems
  (b) foundered on — not merely re-weighing them. Or evidence that the inlined subset is
  insufficient for a real census, which would be an argument for widening the inline, not for
  restoring the link.
