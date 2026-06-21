# Required Reading Map

Status: executed-hold.
Evidence mode: Static + Ran.

## Core Reading

| Path | Purpose |
|---|---|
| `AGENTS.md` | Repository-wide governance, validation, and root directives. |
| `docs/work-packages/AGENTS.md` | Work-package execution, gate, review, and closure rules. |
| `docs/specifications/science-contracts/AGENTS.md` | Contract-first and science-authority rules for kernel/publication work. |
| `docs/standards/prompt-wording-guidance.md` | Kickoff prompt wording and subagent authorization requirements. |
| `docs/architecture/array-native-runtime-specification.md` | Canonical R6 publication operand ledger authority. |
| `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md` | Parent R6 cutover objective and acceptance gates. |
| `docs/work-packages/20260621-r6-direct-publication-cutover-001/artifacts/worker-handoff.md` | Source of the five first-actionable items for R6B. |
| `docs/work-packages/20260621-r6a-run-bound-direct-publication-frame-001/package.md` | Direct publication frame and consumer-path predecessor. |
| `docs/work-packages/20260621-r6b-direct-publication-parity-manifest-cutover-001/package.md` | This package contract. |
| `crates/AGENTS.md` | Rust crate-local implementation guidance. |
| `tests/AGENTS.md` | Test-local implementation guidance. |

## Conditional Reading

| Trigger | Path |
|---|---|
| Any `SC-*` amendment. | `docs/specifications/science-contract-authoring-procedure.md` |
| Any `SC-*` amendment. | `docs/specifications/science-contracts/kernel-process-contract-profile.md` |
| Any `SC-*` amendment. | `docs/specifications/science-contracts/index.md` |
| Manifest/provenance authority change. | `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md` |
| Water-balance publication/reconstruction authority change. | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` |
| Sediment/HBP/PASS publication authority change. | `docs/specifications/science-contracts/contracts/SC-SED-001.md` |

## On-Demand Source Reading

| Area | Paths |
|---|---|
| Direct runtime frame and capture. | `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`, `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**` |
| Runner cutover path. | `crates/openwepp-runner/src/api.rs`, `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`, `crates/openwepp-runner/src/hillslope/**` |
| Output writers and metadata. | `crates/openwepp-hillslope-output/src/**`, `crates/openwepp-legacy-bridge/src/hbp.rs` |
| Tests and fixtures. | `crates/openwepp-runner/tests/**`, `tests/integration/**`, `tests/fixtures/**` |

## Execution Source Reading

| Path | Finding |
|---|---|
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | Cutover candidate calls `build_direct_publication_artifacts`, creates `DirectRunFrame::skeleton`, seeds geometry, then runs publication capture. |
| `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` | `DirectRunFrame`, `DirectLaneFrame`, and `DirectDayFrame` skeletons provide zero/default publication inputs; erosion authority is absent. |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | Direct HBP/WAT/PASS/loss projection consumers exist, but they read the skeleton-populated `DirectRunPublicationFrame`. |
| `crates/openwepp-hillslope-output/src/**` | WAT/PASS production writers are compatibility-fed until the cutover gate passes. |

## Budget

Core pre-edit reading excluding this new package is approximately `119494`
bytes. Disposition: `WARN`. The package keeps detailed source and contract
files conditional/on-demand to avoid loading unrelated authority before the
touched mechanism is known.
