# simimpl07 mode propagation integration map

Status: complete
Evidence mode: Static + Ran
Date: 2026-05-24

## Static
- Integration closure chain is now explicit and typed:
  - parse `wepp_ui` (`requested`, `effective`, `mode_divergence`, warnings),
  - validate and normalize mode tuple via `build_mode_selection_provenance`,
  - map effective mode to lane identity (`daily`/`hourly`),
  - pass selected lane into scheduler/kernel lifecycle execution,
  - publish manifest mode-selection provenance at
    `mode_selection.wepp_ui.*`.
- Legacy sidecar discovery path mapping:
  - sidecar presence establishes `requested_hourly_seepage`,
  - parser result is consumed and warnings are surfaced,
  - mode-selection provenance is built and propagated.
- Runfile override path mapping:
  - `requested_hourly_seepage` uses `.run` flag (`sidecar_overrides.wepp_ui`),
  - parser result is consumed regardless of sidecar file registration,
  - mode-selection provenance is built and propagated.
- Guard closure behavior:
  - invalid `requested`/`effective` domain,
  - inconsistent divergence tuple,
  - unsupported lane mapping,
  all fail typed at `surface = "mode_selection"` with `WUI-E-005`.

## Ran
- Verified integration points in `crates/openwepp-runner/src/lib.rs`:
  - sidecar parse and provenance build paths,
  - lane injection into execution lifecycle,
  - manifest mode-selection publication.
