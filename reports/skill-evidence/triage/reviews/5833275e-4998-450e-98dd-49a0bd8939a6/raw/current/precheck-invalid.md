# Invalid precheck batch

All three outputs are retained, but none enters the behavioral comparison. The executor wrapper
said not to read "any other evidence/review material," which conflicted with the frozen raw
artifact `inputs/reproduction/repos/playbench/reports/triage-review.md`. Two executors treated the
required fixture as forbidden and one did not. The frozen plan, raw task, artifacts, witness, and
rubric were not changed. The complete three-run current arm must be rerun with a wrapper that bars
only the repository's real evidence store and explicitly permits every sanitized raw fixture.
