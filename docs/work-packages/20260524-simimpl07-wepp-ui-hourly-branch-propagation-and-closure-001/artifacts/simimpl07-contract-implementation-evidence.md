# simimpl07 contract implementation evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- SIMIMPL03 contract authority targets for SIMMODE closure were implemented in
  production runner flow:
  - `SC-SYSTEM-001` `INV-SYSTEM-019`
  - `SC-WATBAL-001` `INV-WATBAL-019` and `OBL-WATBAL-P-007`
  - `SC-INFILE-WEPPUI-001` `C-WUI-005`, `G-WUI-008`, `G-WUI-009`, `WUI-E-005`
- Runner manifest now exports canonical mode-propagation provenance subtree:
  - `mode_selection.wepp_ui.requested`
  - `mode_selection.wepp_ui.effective`
  - `mode_selection.wepp_ui.selected_lane`
  - `mode_selection.wepp_ui.mode_divergence`
  - `mode_selection.wepp_ui.guard_id = "WUI-E-005"`
- Parsed `wepp_ui` results are now consumed in both sidecar resolution paths
  instead of parse-only discard behavior:
  - legacy sidecar discovery path
  - runfile sidecar override path
- Effective-mode to lane closure is now typed and deterministic:
  - `ui_run=0 -> daily`
  - `ui_run=1 -> hourly`
  - other values or tuple inconsistency hard-fail via mode-selection guard.

## Ran
- Production implementation changes verified in:
  - `crates/openwepp-runner/src/lib.rs`
- Contract-derived test activation change verified in:
  - `crates/openwepp-runner/tests/simimpl04_wepp_ui_mode_closure_contract.rs`
