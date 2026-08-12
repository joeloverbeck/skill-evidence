# Deterministic Checks

## T1

All six repositories retained HEAD `04166ca72f4359314bfffcbed5663a131603c93a`, the same five
modified paths, no stash, and scoped diff SHA-256
`cbfcec98ce8d55df23ed31f827fa3df597b3e556cae04a54ed814b9cf3d4e1e0` before and after execution.
File SHA-256 values were identical across arms and runs. Candidate reports were internally
identity-consistent, but run 2 chose patch identity
`ab7db7f9163aeaf3e7bad13bf0ae735cb7650c260c03c0a90a3dbd3357ed2c53` while runs 1 and 3 chose
the deterministic scoped-diff hash above.

## T2-T4

Each paired repository retained its exact reviewed HEAD, remained clean, held no stash, and had
byte-identical files across arms after execution.

## T5

Both repositories retained HEAD `98674fc4acb647d466cd23787a4a723e9c65c387`, the same two dirty
paths, no stash, and authorized scoped-diff SHA-256
`2b317e4323231d04da566ff3dd3df3a78bff2a0e9f606a9b7a653a1f233847da`. The unrelated sentinel
remained SHA-256 `f641f022503420433a082e885647810297b74db84e34a743976893e73e7e20cc` and was absent from the
candidate report.
