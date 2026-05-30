# HPHYS0209 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production implementation
- Static: no production kernel/runtime publication behavior changes were applied
  in HPHYS0209.
- Static: implementation scope is contract + test + adjudication evidence for
  the near-closed `ProfileWPStore` lane.

## Workspace validation gates
- Ran: `cargo fmt --check` -> pass
- Ran: `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- Ran: `cargo test --workspace` -> pass
- Ran: `cargo deny check` -> pass (exit `0`, warnings only)
- Ran: logs under `/tmp/hphys0209_20260530T171007Z/gates/`

## Contract-derived targeted tests
- Ran: `cargo test -p openwepp --test hphys0209_profilewp_adjudication_contract`
  -> pass
- Ran: `cargo test -p openwepp-runner hphys0209_` -> pass
- Ran: logs under `/tmp/hphys0209_20260530T171007Z/tests/`

## Residual-lane adjudication evidence
- Ran: generated focused residual summary from source cohort semantic reports:
  - source reports:
    `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic/H*.semantic.json`
  - generated summary:
    `/tmp/hphys0209_20260530T171007Z/parity/reports/hphys0209_profilewp_focus_summary.json`
- Ran: focused metrics:
  - `ProfileWPStore`: `1/39` fail hillslopes (`H7` only)
  - `ProfileDepth`: `0/39` fail hillslopes
  - `ProfilePorosityCap`: `0/39` fail hillslopes
- Ran: isolated failing lane (`H7`) remained stable:
  - `ProfileWPStore` mean abs diff: `1.669863102645198`
  - `ProfileWPStore` fail_count: `1461/1461` rows (H7 only)
  - evidence file:
    `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic/H7.semantic.json`
