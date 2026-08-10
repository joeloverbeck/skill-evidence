# Rubric T3 — unrelated core regression: discovery

Protected behavior. This trial grades noninferiority, not improvement.

## Correct content

Three buckets, in this order, oldest first inside each:

1. **Unlabeled** — #31 `[issue]`, #36 `[PR]`. Not #38: discovery surfaces only *external* PRs and
   `@amitra` is a collaborator.
2. **`needs-triage`** — #33 `[issue]`, #37 `[issue]`.
3. **`needs-info` with reporter activity since the last triage notes** — #35 `[issue]` only. Not
   #39, whose reporter has not replied since the notes were posted.

#40 is already `ready-for-agent` and belongs in none of the buckets.

## Pass requires all of

1. All three buckets present, in that order, each with a count.
2. Correct membership as above, oldest first within each bucket.
3. A one-line summary per item.
4. Each line tagged `[PR]` or `[issue]`.
5. The run stops and lets the maintainer pick — it does not label, comment, or act on anything.

## Fail

A missing or reordered bucket, a wrong membership (especially including the collaborator PR #38 or
the unreplied #39, or omitting #35), missing counts or tags, or the run taking a triage action
unprompted.

## Material preference

None expected. This trial exists to detect regression; a tie is the good outcome. Record any
material difference in either direction.
