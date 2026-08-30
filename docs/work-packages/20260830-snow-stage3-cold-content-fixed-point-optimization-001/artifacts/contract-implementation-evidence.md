# Contract implementation evidence

Status: `PASS`

Evidence mode: `Static + Ran`

`SC-SNOWENERGY-001` advanced from version 27 to 28 for the implementation and
to review-correction version 29. Version 28 added
`INV-SNOWENERGY-054`. It binds a failed authentic finalization rebuild to the
existing support-scaled, discrete-guarded Stage 3 contraction and requires one
guarded provisional Picard stabilization crossing before finalization is
retried. It retains authentic final LSE/boundary operands, converged soil,
unchanged `TOL-SNOWENERGY-003`, the 96-iteration cap, exact-floor policy, and
authentic-map-only publication.

Runtime implementation is in
`covered_fixed_point_finalization_stage3_iterate_v1` and
the stateful `CoveredFinalizationStabilizationV1` seam. Contract registry and
all compiler-discovered SnowEnergy consumers are advanced to version 29.

Version 29 resolves RA-001/RB-003 without changing runtime tolerances or
physics: candidate density is copied bitwise and never interpolated, its
difference remains an exact convergence failure, and the canonical branch,
guard, invariant, and test-vector tables now bind finalization restart and the
exactly-once stabilization seam. Compiler-discovered consumers are pinned to
version 29.

Ran on the terminal generation-37 worktree:
`bash tools/release/check_science_contract_admission.sh --base-ref be40a9435 --worktree`
returned `A0_ADMITTED`, 49 contracts, four science surfaces, authority digest
`ce2befbdb7214be8194f01d3f8645663ce916a232ff476cc21692986034dad1a`.
