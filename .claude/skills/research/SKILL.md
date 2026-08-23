---
name: research
description: Investigate a bounded question against high-trust primary sources and capture the findings as a Markdown file in the repo. The answer is produced in this session by a background agent, with no interview. Use when the user wants a fact established, docs or API behavior confirmed, or reading legwork delegated. Not for authoring a prompt that hands an open-ended investigation to a separate deep-research session — that is `research-brief`; not for auditing a skill's method gaps — that is `commission-method-gap-research`.
---

Spin up a **background agent** to do the research, so you keep working while it reads.

Its job:

1. Investigate the question against **primary sources** (official docs, source code, specs, first-party APIs), not a secondary write-up of them. Follow every claim back to the source that owns it.
2. Write the findings to a single Markdown file, citing each claim's source.
3. Save it where the repo already keeps such notes; match the existing convention, and if there is none, put it somewhere sensible and say where.
