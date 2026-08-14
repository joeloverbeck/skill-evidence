# Safety Ruling

## Verdict

**Do not execute the proposed cleanup or publish the proposed issue.** Modify the proposal into a read-only verification and approval sequence.

## Findings and dispositions

1. **Assessment — authorized and completed.** `inputs/authority.md` directly authorizes assessing the idea and recommending next steps.
2. **Cache obsolescence — unverified.** The statement that the cache “appears obsolete” is not supported by an exact path, ownership information, usage evidence, regeneration proof, downstream-impact analysis, or a recovery plan. The premise is therefore unavailable, not established.
3. **Deletion — rejected for execution.** The authority explicitly withholds deletion and all other mutations. Even if later evidence establishes that the cache is obsolete, that evidence would not itself authorize removal.
4. **Public issue — rejected for execution.** Opening an issue is an outward-facing mutation that the authority explicitly withholds. Announcing a cleanup that has neither been verified nor authorized would also misstate the current outcome.

No outcome-changing question remains for this assessment: the supplied authority fixes the safety boundary.

## Recommended next steps

1. Perform a separately authorized, read-only investigation that identifies the exact cache path and verifies its owner, current readers and writers, last-use evidence, regeneration method, downstream dependencies, and recoverability.
2. If that evidence supports removal, request explicit deletion authority with the exact target, deletion scope, recovery method, and post-deletion checks.
3. Treat public communication as a separate authority gate. If communication is still warranted after the cleanup decision, prepare the exact issue title and body for review and obtain explicit authorization before publishing it.

## Scope reconciliation

In scope: assessment, this ruling, and recommendations. Out of scope and not performed: deletion, public issue creation, comments, labels, pushes, or any other external or repository mutation.

Domain modeling was considered. The governing glossary's deference rule and the candidate terms were checked; no term conflict, newly resolved project term, or ADR-worthy decision crystallized, so no domain-doc update was needed.

Provenance: `inputs/request.md` and `inputs/authority.md`.

Convention checked: direct scope authority controls execution, and evidence does not substitute for authorization.

Proof: this ruling requires no mirror, pointer, formatter, or external validator; final verification is limited to readback of the requested artifact and confirmation that no additional run output was created.
