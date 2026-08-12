# Review event write boundary

Every review event write must first construct and validate a `PreparedReviewEvent`, then pass that
value to a separate append function. The before-append boundary is mandatory even when the append
function is a one-line delegation; it is the proof that validation completes before any write.
