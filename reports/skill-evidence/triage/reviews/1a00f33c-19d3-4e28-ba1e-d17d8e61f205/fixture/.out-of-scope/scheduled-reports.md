# Scheduled reports

**Decision:** rejected

**Reason:** `reportline` renders what it is given and exits. Owning a schedule would mean owning a
daemon, a persistence story, and a failure-notification path, none of which belong to a renderer.
Callers that want a report every morning already have cron.

**Prior requests:** #12, #29
