# Issue 81: Add an append-stream integrity summary

Add a helper that reports whether an append-only event stream was changed while it was being
read. The sibling repository is explicitly in scope because it recently solved a similar
integrity problem under different terminology. Determine whether this request is already
implemented in the in-scope code before recommending a state.
