# WSHEDPLAN01 Gap Assessment

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static

### Gap register

| Gap ID | Severity | Statement | Impact | Candidate write set |
|---|---|---|---|---|
| WSHED-GAP-001 | blocking | WS11 contract authority requires legacy-equivalent branch lineage and explicitly deauthorizes gain-factor surrogate routing, but current `run_channel_node` still uses simplified gain/storage equations (`routing_gain`, `creams_gain`, `wave_storage`, `mc_storage`) instead of baseline wave-routing branch families. | Contract/runtime divergence for core channel routing physics. | `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`, `crates/openwepp-watershed-orchestrator/src/lib.rs`, `tests/integration/ws11_*` |
| WSHED-GAP-002 | blocking | `ipeak=3` and `ipeak>=4` branches do not implement baseline `wshchr/chrqin` time-step/segment routing state (`q1`, `qin`, `qlat`, `c0..c4`). | No baseline-authoritative KW/MC routing parity for watershed channels. | `crates/openwepp-watershed-orchestrator/src/lib.rs`, runtime seam projection surfaces, comparator harness/tests |
| WSHED-GAP-003 | high | Channel runoff generation and transmission-loss process lineage (`wshcqi` + `wshirs` + `wshrun` case semantics) is not migrated; watershed CLI currently assembles from latest-event pass payload + minimal globals. | Inflow/runoff/duration semantics are materially simplified vs baseline controller. | `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`, `crates/openwepp-watershed-orchestrator/src/lib.rs`, `runtime_inputs.rs` |
| WSHED-GAP-004 | blocking | Channel sediment routing stack (`chnero`, `chnrt`, `detach`) has no production migration path in watershed orchestrator. | Watershed sediment process parity and related output families cannot close. | new watershed-channel-sediment module(s), `crates/openwepp-watershed-orchestrator/src/lib.rs`, contracts/tests |
| WSHED-GAP-005 | blocking | WS12 authority requires RK4 continuity, adaptive timestep retry, and regime-transition handling (`imphnw`/`impflo`/`impmai` lineage); current impoundment kernel uses a simplified one-step update and reduced outflow composition. | Impoundment hydraulic parity is incomplete under declared WS12 authority. | `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`, `crates/openwepp-watershed-orchestrator/src/lib.rs`, WS12 tests/comparators |
| WSHED-GAP-006 | high | Parser/runtime seam does not project full impoundment coefficient/structure families into production symbols; WS12 tests manually inject coefficient payload (`a,b,c,d,e,ha,ht,hlm,a0,a1,a2,l0,l1,l2`). | Runtime depends on synthetic test seeding rather than parser-authoritative projection. | `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`, `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`, `openwepp-cli-watershed.rs` |
| WSHED-GAP-007 | blocking | Watershed output writer is intentionally blocked with `OWSOUT-E-004`; no parquet outputs are emitted. | Watershed runtime cannot publish non-placeholder interchange outputs. | `crates/openwepp-watershed-output/src/writers.rs`, row-model builders, watershed CLI integration tests |
| WSHED-GAP-008 | high | Test coverage is mainly synthetic finite/non-negative assertions; there is no end-to-end `openwepp-cli-watershed` execution lane proving baseline-authoritative routing/impoundment/parquet semantics. | High risk of false-positive closure claims from partial tests. | `tests/integration/*watershed*`, fixture packs, comparator tooling lanes |
| WSHED-GAP-009 | medium | Package scaffolding cited non-existent baseline file `chndet.for`; correct channel-detachment authority path is `detach.for` (used via `chnrt`). | Documentation provenance friction and onboarding ambiguity. | package metadata/prompt deps, baseline map artifacts |

### Readiness verdict
- Watershed execution scaffolding readiness: `PARTIAL-READY`.
- Watershed process-physics parity readiness: `HOLD`.
- Watershed parquet publication readiness: `HOLD`.

### Recommendation for audit 20260526 finding 4.1 follow-up
- Keep finding 4.1 as historically correct, but add a re-anchoring note because
  refactor005 moved line anchors.
- Publish a lightweight audit amendment that:
  1. re-anchors old `03_kernel_support.rs` references to split files,
  2. records that `SC-SED-001` `GAP-SED-005` closure text lagged code and
     should remain synchronized in future closure chains.

## Ran
- Static evidence extraction commands (`rg`, `sed`, `nl`) over watershed
  orchestrator/runtime/output/runner surfaces, baseline Fortran routines, and
  relevant science contracts listed in `wshedplan01-current-surface-inventory.md`
  and `wshedplan01-baseline-routine-map.md`.
