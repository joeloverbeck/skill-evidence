# ADR-0002: Report item titles are not truncated

**Status:** accepted

**Decision:** `reportline` renders an item title in full, at whatever length the item carries. No
rendering surface truncates, elides, or abbreviates a title.

**Reason:** titles are the only field a consumer can use to identify an item across two runs. A
truncated title collides with every other title sharing its prefix, and the collision is silent —
the consumer sees two identical rows and cannot tell whether the underlying items differ. Terminal
width is the terminal's problem to solve, not the renderer's.

**Consequences:** callers that need a fixed-width display wrap or scroll; they do not ask the
renderer to shorten. A future decision to truncate must supersede this record.
