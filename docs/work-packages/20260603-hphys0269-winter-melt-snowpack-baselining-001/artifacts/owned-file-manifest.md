# Owned-File Manifest

Status: completed/HOLD
Evidence mode: static

Static: files touched by HPHYS0269.

| Path | Rationale |
|---|---|
| `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | Added HPHYS0269 snowpack retained-rain/signed-melt authority and aliases. |
| `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | Added WB closure invariant consuming retained-rain/raw-melt snow lineage. |
| `docs/specifications/science-contracts/index.md` | Registered updated SC versions and HPHYS0269 invariant references. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs` | Implemented active snow retained-rain/raw-melt/daily redistribution slice. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs` | Wired retained-rain liquid forcing, runoff snow term, and hourly trace publication. |
| `crates/openwepp-runner/src/hillslope/mod.rs` | Extended HPHYS trace schema v8 and added retained-rain/raw-melt closure fields. |
| `tests/integration/clim05_snow_runtime_kernel_contract.rs` | Added contract-derived retained-rain and signed-melt redistribution tests. |
| `docs/work-packages/README.md` | Updated HPHYS0269 status and continuation posture. |
| `docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/**` | Package artifacts, prompt, diagnostics script, evidence, and disposition. |

Static: files intentionally not modified.

- `SC-RUNOFFPART-001.md`: runoff publication semantics were expressible through
  existing WB snow-term and did not require a new canonical invariant in this
  slice.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`:
  no forcing-construction mismatch was proven for this slice.
