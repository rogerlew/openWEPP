# WSHEDIMPL12 Follow-On Package Specs

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static
Execution-ready follow-on package specifications derived from WSHEDIMPL11
handoff immediate next actions.

### WSHEDIMPL13 - Active-Lane 15-Function Parity Migration
- Proposed package id:
  - `20260527-wshedimpl13-active-lane-15-function-parity-migration-001`
- Primary blocker:
  - `GAP-SYSTEM-007` (`SC-SYSTEM-001`)
  - `GAP-IMPOUND-006` (`SC-IMPOUND-001`)
- Objective:
  - migrate full active-lane WS12 structure-family parity from legacy
    min-controller composition (`impint/imphnw/impflo`) beyond current reduced
    coefficient family projection.
- Required reading:
  - `AGENTS.md`, `docs/codex_exec_plans.md`,
    `docs/specifications/science-contract-authoring-procedure.md`,
    `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
    `SC-IMPOUND-001`, `SC-SYSTEM-001`,
    `/workdir/wepp-forest_260430_baseline/src/impint.for`,
    `/workdir/wepp-forest_260430_baseline/src/imphnw.for`,
    `/workdir/wepp-forest_260430_baseline/src/impflo.for`.
- Intended write set:
  - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
  - `crates/openwepp-watershed-orchestrator/src/lib.rs`
  - `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
  - canonical contracts/index rows for gap closure notes.
- Contract-first sequencing:
  1. contract amendments (if needed),
  2. contract-derived vectors,
  3. pre-implementation contract gate,
  4. production kernel edits.
- Validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### WSHEDIMPL14 - Baseline-Authoritative Watershed Comparator Lane
- Proposed package id:
  - `20260527-wshedimpl14-watershed-baseline-authoritative-end-to-end-comparator-lane-001`
- Primary blocker:
  - `GAP-SYSTEM-005` (`SC-SYSTEM-001`)
- Objective:
  - establish a baseline-authoritative `openwepp-cli-watershed` comparator lane
    that validates topology dispatch, branch execution, and output publication
    against baseline authority.
- Required reading:
  - `AGENTS.md`, `docs/codex_exec_plans.md`,
    `SC-SYSTEM-001`, `SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-SED-001`,
    watershed CLI contract tests,
    `/workdir/wepp-forest_260430_baseline` watershed routines.
- Intended write set:
  - `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`
  - comparator harness modules/artifacts under `tests/integration/**` as needed
  - package artifacts for comparator evidence and disposition.
- Contract-first sequencing:
  1. contract lane/acceptance authority confirmation,
  2. contract-derived comparator vectors,
  3. pre-implementation gate,
  4. harness/integration edits.
- Validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### WSHEDIMPL15 - Channel Sediment Process-Parity Migration
- Proposed package id:
  - `20260527-wshedimpl15-watershed-channel-sediment-process-parity-migration-001`
- Primary blockers:
  - `GAP-SYSTEM-008` (`SC-SYSTEM-001`)
  - `GAP-ROUTE-009` (`SC-ROUTE-001`)
  - `GAP-SED-006` (`SC-SED-001`)
- Objective:
  - migrate full watershed channel sediment process families
    (`chnero/chnrt/detach`) with typed guards and baseline-authoritative
    process lineage; remove remaining publication-only surrogate behavior.
- Required reading:
  - `AGENTS.md`, `docs/codex_exec_plans.md`,
    `SC-ROUTE-001`, `SC-SED-001`, `SC-SYSTEM-001`,
    `/workdir/wepp-forest_260430_baseline/src/chnero.for`,
    `/workdir/wepp-forest_260430_baseline/src/chnrt.for`,
    `/workdir/wepp-forest_260430_baseline/src/detach.for`.
- Intended write set:
  - `crates/openwepp-watershed-orchestrator/src/lib.rs` (or extracted module)
  - watershed sediment contract/integration tests
  - canonical contract/index closure notes for sediment blocker rows.
- Contract-first sequencing:
  1. contract authority updates/alias continuity,
  2. contract-derived vectors,
  3. pre-implementation gate,
  4. production kernel migration.
- Validation gates:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

### Sequencing
Recommended execution order:
1. `WSHEDIMPL13`
2. `WSHEDIMPL14`
3. `WSHEDIMPL15`
4. follow-on hold-lift/disposition rerun package after the three closures.

## Ran
- not run
