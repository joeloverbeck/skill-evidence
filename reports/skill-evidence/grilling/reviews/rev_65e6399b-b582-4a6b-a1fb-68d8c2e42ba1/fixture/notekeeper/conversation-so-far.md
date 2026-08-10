# Conversation so far

## Me

I want a machine-readable output mode for notekeeper so my shell scripts stop parsing the
human text. Grill me on it.

## You

I read the README, ADR 0001, and CONTRIBUTING before answering. Three things are settled by
the repo already: notes are stored as JSONL with a `written` timestamp, on-disk field names are
internal, and a machine-readable *output* format is a separate compatibility surface that would
need its own ADR.

**What should the `--json` output look like?**

- **Option A — Flat JSON, one record per line** *(recommended)*

  ```json
  {"schema":"notekeeper/v1","body":"buy milk","tags":["errand"],"created_at":"2026-06-01T00:00:00Z"}
  {"schema":"notekeeper/v1","body":"call dentist","tags":[],"created_at":"2026-06-01T09:12:00Z"}
  ```

  Streams line by line, so `head` and `grep` work on it. Costs you the ability to attach
  top-level metadata such as a result count.

- **Option B — One JSON document per invocation**

  ```json
  {
    "schema": "notekeeper/v1",
    "count": 2,
    "notes": [
      {"body": "buy milk", "tags": ["errand"], "created_at": "2026-06-01T00:00:00Z"}
    ]
  }
  ```

  Carries a result count and leaves room for pagination later. Costs you streaming; consumers
  must buffer the whole output.

- **Option C — Tab-separated columns**

  ```
  2026-06-01T00:00:00Z	buy milk	errand
  ```

  Smallest to implement and trivially greppable. Costs you nested tags and any future field
  that contains a tab.

## Me

Option A.
