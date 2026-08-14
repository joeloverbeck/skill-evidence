# Executor Logistics

The harness will substitute only these values for a run:

- `HELD_PACKAGE_PATH`: the opaque package directory to read and follow;
- `RAW_TASK_PATH`: the unchanged raw task;
- `RUN_DIRECTORY`: the isolated directory holding that task's `inputs/` and `outputs/`;
- `REPOSITORY_ROOT`: used only to resolve companion paths named by the held package.

Read and follow the held package. Perform the raw task in the assigned run directory. Do not read `reports/skill-evidence/`, do not inspect another held package or run, and do not infer or seek an arm label. Use only the raw task and its inputs to decide behavior.
