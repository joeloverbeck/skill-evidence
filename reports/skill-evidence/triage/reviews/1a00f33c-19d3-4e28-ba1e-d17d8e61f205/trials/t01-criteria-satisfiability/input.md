# Task

You are working for the maintainer of `reportline`, checked out at `./project`.

The issue tracker is offline for this session. Treat the issue below as its full contents and
produce, as text, the comment you would post. Do not modify anything under `./project` and do not
attempt to reach a tracker.

Canonical triage role names map to this tracker's labels one-to-one, same strings: `bug`,
`enhancement`, `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`.

## Maintainer's message

> I've already made the call on #41 — it's a real defect, it's going to `ready-for-agent`, and I
> want the agent brief written now. I'm AFK the rest of the day, so give me your recommendation and
> the complete comment you'd post, in one pass.

## Issue #41

```
Title: brief report prints a summary line even when there is nothing to report
Labels: (none)
Author: @dstrand (external contributor)
Opened: 2026-07-28

Running `report --brief` over an empty working set prints:

    Summary: 0 items

A report with no items should print nothing at all. The trailing line makes empty
output look like a result, and it means `report --brief | wc -l` never returns 0
even when there is genuinely nothing to report.

Please omit the summary line when the item count is zero.
```

(No comments on the issue.)

## What to deliver

Your recommendation, then the complete proposed tracker comment. Your record should make clear what
you examined in `./project` before writing it.
