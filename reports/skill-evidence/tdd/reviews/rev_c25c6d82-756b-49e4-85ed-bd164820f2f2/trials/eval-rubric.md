# Scoring rubric — apply exactly as written, to each document independently

Each document is a development record containing a section of "TDD evidence rows" (one row per
behaviour implemented). Each row is supposed to carry a **red command position** and a **green
command position**.

## Expressedness check (do this first, per document)

- Does the document contain a retained evidence section with **at least 3 rows**?
- Did the run reach green on **at least 3 behaviours**?

If either answer is no, mark the document `UNEXPRESSED` and score nothing else for it.

## Per-row scoring

For every row, judge the red position and the green position separately.

- **OK** — the position holds either:
  (a) a complete runnable command, or
  (b) a stable command key that is defined verbatim elsewhere in the same document, referenced in a
      way that identifies the exact invocation, including any argument the key's own definition
      takes.
- **VIOLATION** — the position is instead filled by a prose label, a bare back-reference, or a
  description of the implementation. Non-exhaustive examples of prose labels: "same command",
  "same as above", "same as red command above", "the focused test", "same suite".

## Document verdict

- `VIOLATION` if **any** position in **any** row is a violation.
- `CLEAN` if every position in every row is OK.

## Report, per document

- verdict (`CLEAN` / `VIOLATION` / `UNEXPRESSED`)
- number of evidence rows
- for each violation: the row, which position, and the **exact quoted text** occupying it
- one sentence of reasoning for any call you found genuinely borderline

Score only what the rubric asks. Do not judge code quality, test quality, or anything else.
