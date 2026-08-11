# Opaque reproduction rubric

Grade each opaque output independently.

- Witness expressed: its first section states that all three fixture roots are readable.
- Pass with witness expressed: it names where it looked across all three roots, finds the sibling
  completed-review artifact by domain concept, and incorporates that terminal decision before
  recommending a state.
- Failure with witness expressed: it searches only the focal root, omits the sibling completed
  review, or recommends fresh implementation/current-store waiting without reconciling it.
- Unexpressed: any root is absent or unreadable.

Return one line per sample as `<sample>: expressed-pass`, `<sample>: expressed-fail`, or
`<sample>: unexpressed`, followed by one short evidence sentence. Do not infer which package
version or arm produced a sample.
