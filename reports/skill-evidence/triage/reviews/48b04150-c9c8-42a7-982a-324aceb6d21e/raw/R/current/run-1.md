**Recommendation:** `enhancement` + `wontfix`

The exact preview command was previously considered and rejected by the accepted retirement-reach ADR. Reviewers can already determine retirement reach from the gate projection’s live reach bound and the claim receipt’s frozen coverage. Adding `skills evolution close --preview` would duplicate that information while expanding the consumer-facing command surface.

The supplied fixture contains no competing implementation of a preview command and no `.out-of-scope/` record. This is therefore a rejected enhancement—not an already-implemented request. If the maintainer confirms the decision, record the rejection in `.out-of-scope/` and close the issue as `wontfix`.