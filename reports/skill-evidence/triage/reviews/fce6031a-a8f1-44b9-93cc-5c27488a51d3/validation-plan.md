# Frozen validation plan

## Binding constraints and witnesses

### Recommendation-order constraint

The failure requires a triage request whose state recommendation depends on verification that
cannot be settled by locating a plausibly redundant implementation. The trials can vary this by
supplying a request with a misleading existing guard and a narrower surviving failure.

- **Witness:** the executor record encounters both the apparently redundant implementation and
  the surviving verified gap, performs the claim verification before presenting the dependent
  recommendation, and stops for maintainer direction only after presenting a sound recommendation.
- **Unexpressed reading:** the record never encounters the apparent redundancy, never establishes
  the surviving gap, or produces no recommendation. Any such reading closes the review as
  `blocked_no_valid_test`.
- **Failure reading:** a sound recommendation requires performing a step earlier than the held
  instructions place it, or the recommendation is made from the apparent redundancy before the
  narrower gap is established.

### Artifact-identity constraint

The failure requires an agent brief whose requested behavior is amendment of exact durable
documents or package artifacts, so their identity is substantive rather than an incidental current
location. The trials can vary this with named artifacts and separately with disposable source-file
locations.

- **Witness:** the executor reaches an agent brief and must decide how to identify the exact
  artifacts while keeping implementation guidance durable.
- **Unexpressed reading:** no agent brief is produced or the task does not require exact artifact
  identity. Any such reading closes the review as `blocked_no_valid_test`.
- **Failure reading:** the brief obscures an exact subject behind a role description, or treats a
  current incidental filesystem location as implementation authority.

## Risk tier

High. A candidate would change a broad workflow sequence and the brief-writing contract, spanning
two major behaviors. Five paired trials and two independent blind evaluators are required.

## Paired trials

1. `t01-ordering-reproduction` — reproduction for recommendation/verification ordering.
2. `t02-identity-reproduction` — reproduction for substantive durable artifact identity.
3. `t03-ordering-adjacent` — adjacent ordinary bug triage protecting verification quality,
   recommendation, and the wait boundary.
4. `t04-durability-adjacent` — adjacent agent brief protecting the ban on incidental paths and
   procedural implementation instructions.
5. `t05-discovery-regression` — unrelated core discovery behavior protecting buckets, ordering,
   external-PR filtering, and absence of mutations.

Each trial uses the frozen executor protocol, one fresh independent executor per arm, opaque arm
paths, and no evidence-store access. The unchanged current reproduction arm runs before candidate
construction. Each completed pair is read by two fresh independent evaluators using the frozen
evaluator protocol, randomized output order, and no arm mapping.

## Acceptance rule

- Both evaluators must find the candidate materially better on both reproduction trials.
- Both evaluators must find the candidate noninferior on all three protected trials; a current-arm
  preference, `both-fail`, or a candidate-only safety failure rejects the candidate.
- Ties are acceptable only on protected trials.
- Both reproduction witnesses must be expressed on the current arm. If either is unexpressed, the
  review closes `blocked_no_valid_test` without candidate construction or further trials.
- Deterministic checks must pass.

## Deterministic checks

1. Candidate file set and file modes equal the live package's file set and modes.
2. Only `SKILL.md` and `AGENT-BRIEF.md` differ bytewise from the live package.
3. Frontmatter name, invocation policy, tracker disclaimer, five canonical states, and
   `.agents/skills/triage -> ../../.claude/skills/triage` remain unchanged.
4. Every relative Markdown link in the candidate resolves inside the repository or candidate
   package at its corresponding live-repository location.
5. `cargo test --locked -p skill-evidence` passes, protecting the repository's installed-asset
   and immutable-evidence contracts even though triage itself is repository-local.

## Candidate hypothesis (frozen before candidate bytes)

- Place claim verification before the dependent recommendation and wait boundary, without changing
  the content of either operation.
- Replace the absolute file-path prohibition with a distinction: exact artifacts may be named when
  they are themselves part of the contract; incidental implementation locations and line numbers
  remain prohibited.
- Make no other target change.
