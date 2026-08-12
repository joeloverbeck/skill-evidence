# Review event write boundary

Every review event write must first construct and validate a `PreparedReviewEvent`, then pass that
value to a separate append function. This boundary is mandatory. Unused extension hooks remain
forbidden unless the current task requires one.
