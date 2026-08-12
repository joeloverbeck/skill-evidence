**Category:** `enhancement`  
**State:** `wontfix`

The requested preview command is not implemented, but the accepted retirement-reach ADR explicitly considered and rejected it. Retirement reach is already knowable from the gate projection’s live reach bound combined with the claim receipt’s frozen coverage. Adding `skills evolution close --preview` would expand the consumer-facing command surface without adding information.

Codebase summary: the supplied fixture contains no command implementation to reuse or duplicate. The relevant domain documentation establishes that evolution-close authorization derives from immutable evidence and that existing artifacts already expose the retirement reach. This is a prior-rejected enhancement, not an already-implemented feature.