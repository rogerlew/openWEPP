# Pre-implementation contract gate

Status: `SUPERSEDED` for temporal-floor claims; rerun required

Static: SnowEnergy v22, SnowFreeze v140, CoupledTime v7, registry, package
authority, kernel-profile checklist, and contract-derived oracle are coherent.

Ran: `nix develop --command cargo nextest run --test
snow_stage3_adaptive_compositional_contract` — 4 passed, 0 failed on the
pre-production contract/test tree. `cargo fmt --all -- --check` and
`git diff --check` passed. Existing workspace warning baseline was observed;
the new contract test introduced no warning after correction.

The recorded run predates the 2026-08-27 60-second owner amendment. It remains
historical evidence for unchanged conservation/custody/phase/topology/receipt/
rollback/fail-closed clauses only. Its exact-grid, floor, attempt-count,
event-tick, and floor-dependent performance results are invalid for current
admission and must not be reported as current PASS evidence. No amended rerun
had occurred at this pre-implementation gate; subsequent focused amended-floor
results are recorded separately in `implementation-test-evidence.md` and do
not alter this artifact's superseded status.
