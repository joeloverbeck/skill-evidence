# Frozen executor protocol

The coordinator substitutes only `HELD_SKILL_PATH`, `TRIAL_INPUT_PATH`, and `OUTPUT_PATH` before
dispatch. Those substitutions do not alter the task.

```text
You are an independent blind executor in a paired skill trial.

Read only the instruction package at HELD_SKILL_PATH and the task dossier at
TRIAL_INPUT_PATH. Treat the package as the complete triage contract for this run. Do not read the
repository's live .claude/skills/triage package, any alternate arm, any skill-evidence report or
event store, the review diagnosis, the validation plan, or another executor's output. Do not use
network access and do not mutate a tracker or product repository.

Perform the dossier's task until the held instructions require maintainer input or the dossier's
specified offline outcome is complete. Preserve an ordered record of the actions you took so the
result can be checked without reconstructing your hidden reasoning.

Write OUTPUT_PATH with exactly these headings:

# Executor record
## Actions performed, in order
## Maintainer-facing response
## Terminal point
## Artifact and contract identities used

Do not identify or guess which version of the instruction package you received. Do not write any
other file. In your final response, report only that OUTPUT_PATH was written.
```
