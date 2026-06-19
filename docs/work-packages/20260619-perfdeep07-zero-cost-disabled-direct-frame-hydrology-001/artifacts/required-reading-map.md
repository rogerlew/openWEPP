# PERFDEEP07 Required Reading Map

Status: complete for HOLD disposition.
Evidence mode: Static/Ran.

## Core Package and Process Authority

Read before production edits:

- `AGENTS.md` (`9043` bytes).
- `docs/work-packages/AGENTS.md` (`10235` bytes).
- `docs/specifications/science-contracts/AGENTS.md` (`5585` bytes).
- `crates/AGENTS.md` (`4450` bytes).
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/package.md`
  (`14765` bytes before execution amendment).
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/prompts/active/perfdeep07_kickoff_agent_prompt.md`
  (`5140` bytes).

## PERFDEEP06 and Runtime Authority

Read or consulted:

- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/worker-handoff.md`
  (`2373` bytes).
- `docs/architecture/array-native-runtime-specification.md` (`42145`
  bytes).
- `docs/decisions/0025-array-native-hillslope-day-frame.md` (`6487`
  bytes).

PERFDEEP06 artifacts established the direct-frame API direction, but
PERFDEEP07 stopped before direct-frame implementation because the ordered P0
disabled-path gate failed.

## PERFDEEP05 Profile Evidence

Read or consulted:

- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05_disposition.md`
  (`2475` bytes).
- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05-profile.md`
  (`3782` bytes).

The relevant control values were PERFDEEP05 default-disabled `701.95 s`,
reference baseline `669.97 s`, and P0 threshold `676.67 s`.

## Source Inventory Inspected

Inspected source relevant to disabled-path cost and retained edits:

- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`
- `crates/openwepp-kernel-contract/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/perfdeep02_frame_roundtrip.rs`
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_scheduler_pl_activation.rs`

Read-only side investigations covered runner registry construction,
persistent scheduler indexed execution, H2637 run command shape, and prior
PERFDEEP timing artifacts.

## Conditional Reading

No canonical `SC-*` contract, physics invariant, output meaning, or diagnostic
authority change was made. The deeper contract-authoring procedure was not
entered for this HOLD run.
