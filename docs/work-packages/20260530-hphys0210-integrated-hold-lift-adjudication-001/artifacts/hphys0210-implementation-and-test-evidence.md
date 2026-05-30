# HPHYS0210 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Implementation scope
- Static: no production kernel/runtime code edits were performed in HPHYS0210.
- Static: package scope is integrated adjudication evidence synthesis from
  upstream completed packages plus fresh workspace gate execution.

## Workspace validation gates
- Ran: `cargo fmt --check` -> pass.
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- Ran: `cargo test --workspace` -> pass.
- Ran: `cargo deny check` -> pass (warnings only, exit `0`).
- Ran: logs under `/tmp/hphys0210_20260530T194829Z/gates/`.

## Targeted integration verification
- Ran:
  `cargo test -p openwepp --test hphys0208_fc_threshold_coupled_residual_contract`
  -> pass.
- Ran:
  `cargo test -p openwepp --test hphys0209_profilewp_adjudication_contract`
  -> pass.
- Ran: logs under `/tmp/hphys0210_20260530T194829Z/tests/`.

## Integrated diagnostics outputs
- Ran: generated integrated family summary with fail-count and mean-abs-diff
  deltas vs HPHYS0207, plus confidence-tier labels and contract-status classes:
  - `/tmp/hphys0210_20260530T194829Z/diagnostics/hphys0210_integrated_family_summary.json`
  - `/tmp/hphys0210_20260530T194829Z/diagnostics/hphys0210_integrated_family_summary.tsv`
- Ran: key summary signals:
  - Closed/corroborated:
    - `ProfileDepth` `0/39`
    - `ProfilePorosityCap` `0/39`
  - Bounded near-closed:
    - `ProfileWPStore` `1/39` (`H7`)
  - Open coupled blockers:
    - `ProfileFCStore` `27/39`
    - `Dp` `39/39` (`+39.9689` mean-abs-diff delta vs HPHYS0207)
    - `latqcc` `39/39` (`+89.6728` mean-abs-diff delta vs HPHYS0207)
    - `Total-Soil` `39/39`
    - `SoilWaterTotal` `39/39`
