# line count governance checklist

Status: checked through M-E1

Evidence mode: Ran

## M-E1

Ran `wc -l` over touched Rust/test files.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | 1239 | OK. |
| `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs` | 1453 | OK. |
| `crates/openwepp-runner/src/hillslope/intake_lane_setup/lane_setup_helpers.rs` | 214 | OK. |
| `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs` | 331 | OK. |
| `crates/openwepp-runner/src/hillslope/tests03/per_ofe_state.rs` | 291 | OK. |
| HPHYS0319/0320 authority tests | 149-231 | OK; stale exact-version pins removed narrowly. |

No touched Rust source file crossed the 2000-line warning threshold or
3000-line non-exempt threshold.

## M-E0

Ran `wc -l` over touched Rust/test/Cargo files and touched contracts/index.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `tests/integration/mofe01_per_ofe_state_contract.rs` | 301 | OK |
| `tests/integration/mofe01_inter_ofe_route_contract.rs` | 41 | OK |
| `Cargo.toml` | 616 | OK |
| `SC-RUNOFFPART-001.md` | 997 | Existing large contract authority; updated narrowly. |
| `SC-WATBAL-001.md` | 2484 | Existing large contract authority; updated narrowly. |
| `SC-SYSTEM-001.md` | 1024 | Existing large contract authority; updated narrowly. |
| `docs/specifications/science-contracts/index.md` | 67 | OK |

No touched Rust source file crossed a line-count threshold. No production Rust
source file was edited.

## M-D

M-D edited work-package artifacts only. No Rust source line-count governance
was triggered.

`mofe-per-ofe-state-architecture.md` is 277 lines after M-D and carries the
required architecture, citation, and M-E breakdown content.

## M-C2

M-C2 edited work-package artifacts only. No Rust source line-count governance
was triggered.

## M-C

M-C edited work-package artifacts only. No Rust source line-count governance was
triggered.

## M-B

Ran `wc -l` over touched production/test Rust files and touched contracts/artifacts.

| File group | Lines | Disposition |
| --- | ---: | --- |
| `03_kernel_support_00_support_helpers.rs` | 417 | OK |
| `hydrology_phase_runoff_reconciliation.rs` | 1255 | OK |
| `state_access.rs` | 1911 | OK, below 2000-line warning threshold |
| `scheduler_seed_and_runtime.rs` | 1890 | OK, below 2000-line warning threshold |
| `publication_wb11_seed.rs` | 557 | OK |
| M-B integration tests touched/added | 41-1175 | OK |
| `SC-WATBAL-001.md` | 2456 | Existing large contract authority; updated narrowly, not a Rust source line-count violation |

No touched Rust source file crossed the 2000-line warning threshold or 3000-line non-exempt threshold.

## M-A

Ran `wc -l` over the three M-A deliverables after edits:

| Artifact | Lines |
| --- | ---: |
| `characterization-openwepp-multi-ofe.md` | 104 |
| `legacy-per-ofe-closure-calibration.md` | 101 |
| `mofe-routing-port-scope.md` | 170 |

The routing scope artifact is longer because it carries file:line citations required by increment M-A. No production source line-count governance was triggered.
