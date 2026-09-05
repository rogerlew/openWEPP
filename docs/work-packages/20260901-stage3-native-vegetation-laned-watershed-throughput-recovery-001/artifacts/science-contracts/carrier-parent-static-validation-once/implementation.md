# Carrier parent-static validation-once implementation disposition

Evidence mode: `Static + Ran`

Canonical authority remains
`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
version 30. The authority is valid but its production implementation remains
`NOT_IMPLEMENTED`.

## Result

The bounded implementation attempt was rejected and fully reverted. A
per-stack validation slot cannot establish the contract's one-parent custody:
the authentic adaptive terminal, batch, single, and canonical subslab paths
construct multiple stacks and deep-clone their LSE and surface configuration
objects. Pointer-bound plans created inside those stacks therefore represent
different allocations and cannot truthfully produce the required `1/52/52`
parent/map audit.

A source-real implementation would have to create a stable plan at
`execute_covered_real_v11_parent_with_evidence`, borrow it through the complete
adaptive terminal/batch/single/subslab executor graph, authenticate every
deep-cloned stack against that live parent lineage, and mint the forcing proof
from `prepared.0.lse_forcing` before the ordinary/native split. The V8
structural joins would still have to occur only at the exact checks they
replace. That is materially broader than the selected bounded increment.

The temporary implementation also could not satisfy the executable parity and
poison obligations with thin wrappers: authentic disposition/path attribution,
parent-lifetime counts, restart proof rejection, and dynamic/solver/output
poisons require real production-boundary instrumentation. No fabricated audit,
fallback, weakened test, or partial production path was retained.

## Reversion evidence

Ran:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo fmt --all -- --check
env RUST_MIN_STACK=67108864 nix develop -c cargo check -p openwepp-hillslope-orchestrator
git diff --check
```

All passed. A production-source residue search for the temporary carrier plan,
forcing proof, resident map proof, test-audit module, and validation-once entry
points returned no matches. Both temporary modules are absent. The contract-
derived v30 tests remain expected-red on the intentionally absent production
surfaces.

## Retention disposition

`REJECTED_BEFORE_BENCHMARK`. The implementation never reached an authoritative
source shape, so no release performance result was run and no savings are
claimed. The earlier `103059 us` attribution remains only an upper-bound target,
not retained evidence.
