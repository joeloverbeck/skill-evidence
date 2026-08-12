**Category:** `enhancement`  
**State recommendation:** `wontfix`

The accepted retirement-reach ADR explicitly considered and rejected a preview command beside the irreversible close. Retirement reach is already determinable from immutable evidence: the gate projection provides the live reach bound, while the claim receipt freezes coverage. Adding `skills evolution close --preview` would duplicate that information and expand the consumer-facing command surface without improving the authorization model.

Codebase summary: the supplied fixture contains no implementation of the proposed command and no `.out-of-scope/` record. Its domain documentation confirms that the underlying preview information already exists through the gate projection and claim receipt, and the governing ADR records a durable decision against adding this command.