# HPHYS0214 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Implementation scope
- Static: no production kernel/runtime code edits were performed in HPHYS0214.
- Static: package scope is integrated readjudication synthesis from upstream
  completed packages plus fresh gate execution.

## Workspace validation gates
- Ran: `cargo fmt --check` -> pass.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- Ran: `cargo test --workspace` -> pass.
- Ran: `cargo deny check` -> pass (warnings only, exit `0`).
- Ran: logs under `/tmp/hphys0214_20260531T004200Z/gates/`.

## Targeted verification checks
- Ran:
  `cargo test -p openwepp --test hphys0208_fc_threshold_coupled_residual_contract`
  -> pass.
- Ran:
  `cargo test -p openwepp --test hphys0209_profilewp_adjudication_contract`
  -> pass.
- Ran:
  `cargo test -p openwepp-runner hphys0213_`
  -> pass.
- Ran: logs under `/tmp/hphys0214_20260531T004200Z/tests/`.

## Integrated diagnostics outputs
- Ran: generated integrated family summary with fail-count and mean-abs-diff
  deltas (HPHYS0212 -> HPHYS0213), confidence-tier labels, and contract-status
  classes:
  - `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.json`
  - `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.tsv`
- Ran: key summary signals:
  - H5 blocker transition: `HKERNEL-WB12-STORAGE-E-003` present in HPHYS0212
    and absent in HPHYS0213.
  - Remaining integrated blockers:
    - `ProfileFCStore`: `27/39`
    - `Dp`: `39/39`
    - `latqcc`: `39/39`
    - `Total-Soil`: `39/39`
    - `SoilWaterTotal`: `39/39`
