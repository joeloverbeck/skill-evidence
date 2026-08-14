---
name: grill-with-docs
description: Run grilling with domain-modeling; create or update domain docs only when terms or ADR-worthy decisions crystallize.
disable-model-invocation: true
---

Use the `grilling` skill with `domain-modeling` as its companion (Claude aliases: `/grilling` and `/domain-modeling`).

- Before premise verification, questions, or execution, state one visible line: `Grilling shape: interview|adjudication|documented process`. Do no task work before that line.
- `grilling` owns the decision protocol: verify premises, collapse branches, ask only outcome-changing questions, recap, then execute reversible ratified work.
- Domain-model only for live term or decision work. Check the governing context; update `CONTEXT.md` or ADRs only when one crystallizes.
- Otherwise, say explicitly in the final recap that no domain-doc update was needed.
