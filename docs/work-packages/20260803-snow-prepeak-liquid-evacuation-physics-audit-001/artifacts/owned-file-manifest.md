# Owned File Manifest

Status: `amended after review and verification precheck`

Evidence mode: `Static: original prospective manifest plus retrospective amendments`

## Writable Paths

- `docs/work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/`
- `docs/work-packages/README.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- `target/snow_prepeak_liquid_evacuation_physics_audit/`
- `target/snow_prepeak_liquid_evacuation_physics_audit_v2/`
- `target/snow_prepeak_liquid_evacuation_physics_audit_v3/`
- `/tmp/openwepp-snow-prepeak-audit-invalid-rain-label-v1-20260803/`

The two versioned target paths are a post-review amendment, not part of the
original prospective manifest. Review required clean non-overwriting reruns;
v2 is retained as rejected binary-reference-confounded evidence and v3 is the
accepted same-binary execution. The original manifest should have named a
bounded versioned namespace family.

The bounded `/tmp` recovery path is a verification-precheck amendment, not part
of the original prospective manifest. The orchestrator moved an invalidated
rain-label run there before creating the accepted evidence namespaces; its 206
files remain rejected custody evidence and are identified by the rejected-run
entry in `execution-receipt.json`. The original manifest should also have
authorized this exact recovery namespace before that move.

Only the orchestrator may write. Investigator, reviewer, and verifier agents
are read-only.

## Protected Paths

Everything outside the seven entries above is protected. In particular,
production Rust, canonical science contracts, tests, fixtures, observations,
selectors, defaults, parameters, and the frozen predecessor evidence are
read-only. Protected source/tree identities are recorded in
`audit-freeze-v3.json` and must be reproduced at terminal verification.
