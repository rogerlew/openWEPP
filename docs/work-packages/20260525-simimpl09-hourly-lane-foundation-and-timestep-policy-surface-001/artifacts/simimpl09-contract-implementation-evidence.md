# simimpl09 contract implementation evidence

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- SIMIMPL09 production implementation closes hourly lane foundation through a
  typed timestep-policy surface and adapter-boundary provenance in the runner
  manifest.
- Authority alignment preserved with canonical SIMMODE/SIMCONS invariants:
  - `SC-WATBAL-001` `INV-WATBAL-019`, `INV-WATBAL-021`
  - `SC-SYSTEM-001` `INV-SYSTEM-019`, `INV-SYSTEM-021`
  - `SC-INFILE-WEPPUI-001` `D-WUI-005`, `G-WUI-008`, `WUI-E-005`
- Adopt-only intake posture from SIMIMPL08 is encoded in production provenance:
  - adapter boundary records `adopt_profile = "SIMIMPL08-adopt-only"`,
  - `reject_surfaces_excluded = true`,
  - `defer_surfaces_excluded = true`.
- Reject/defer surfaces remain excluded from runtime intake:
  - no `wbk09_hourly_qcap_policy` integration,
  - no env-toggle identity/probe overlays,
  - no deferred route/pass adapter surfaces.

## Ran
- Production implementation verified in:
  - `crates/openwepp-runner/src/lib.rs`
- Contract authority probes reviewed via:
  - `rg -n "INV-WATBAL-019|INV-WATBAL-021|INV-SYSTEM-019|INV-SYSTEM-021|D-WUI-005|G-WUI-008|WUI-E-005" docs/specifications/science-contracts/contracts/SC-WATBAL-001.md docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md`
