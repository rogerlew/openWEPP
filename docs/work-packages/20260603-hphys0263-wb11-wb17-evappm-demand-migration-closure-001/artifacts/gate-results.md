# Gate Results

Status: completed

Evidence mode: ran

Ran: All commands below were executed locally in `/home/workdir/openWEPP`.

## Contract-First Gate

- `cargo test -p openwepp-runner hphys0263_wb11_seed_uses_evappm_branch_when_pmetpara_selects_pmet -- --nocapture`
  - Result: failed before production implementation.
  - Failure class: expected missing migrated PMET diagnostics, including
    `missing pmet.etorc_mm`.
  - Use: red pre-implementation contract gate.

## Focused Gates

- `cargo fmt --check`
  - Result: passed.
- `cargo test -p openwepp-runner hphys026 -- --nocapture`
  - Result: passed; 5 tests passed.
- `cargo test -p openwepp --test parser_runtime_seam_integration climate_runtime_projection_parity_hillslope_vs_watershed_adapter_path -- --nocapture`
  - Result: passed after adding watershed per-hillslope `deglat` and `elevm`
    projection.

## Diagnostic Runs

- `python docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/hphys0263_diagnostics.py --out-root /tmp/hphys0263_20260603T064818Z`
  - Result: completed, but invalid for PMET-branch closure.
  - Finding: runfile-sidecar mode did not discover default `pmetpara.txt`, so
    H1/H7/H39 selected `iflget=1`.
- `python docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/hphys0263_diagnostics.py --out-root /tmp/hphys0263_20260603T070159Z`
  - Result: failed intentionally through typed missing-symbol handling.
  - Finding: PMET branch required `canhgt`; management projection computed it
    but did not publish it to the scalar runtime surface.
- `python docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/hphys0263_diagnostics.py --out-root /tmp/hphys0263_20260603T070311Z`
  - Result: passed.
  - Targeted classification: H1/H7/H39 all
    `EVAPPM_MIGRATED_BRANCH_OBSERVED`.
  - Report root: `/tmp/hphys0263_20260603T070311Z/reports`.

## Full Workspace Gates

- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed.
- `cargo test --workspace`
  - Result: passed.
  - Note: one HPARITY test remained ignored as configured by the suite.
- `cargo deny check`
  - Result: passed.
  - Existing warnings: duplicate crate versions for `getrandom`, `hashbrown`,
    and `twox-hash`; license-not-encountered warnings for `ISC` and
    `Unicode-DFS-2016`.

## Full H1..H39 Metrics

- Semantic pass: `0/39`.
- `Ep`: `pass=0/39`, `total_failures=56148`, `mean_abs_diff_mean=1.669180`,
  `max_abs_diff=7.778863`.
- `Total-Soil`: `pass=0/39`, `total_failures=55878`,
  `mean_abs_diff_mean=149.417133`, `max_abs_diff=611.810480`.
- `SoilWaterTotal`: `pass=0/39`, `total_failures=55878`,
  `mean_abs_diff_mean=149.417133`, `max_abs_diff=611.810480`.
- `Dp`: `pass=0/39`, `total_failures=35444`,
  `mean_abs_diff_mean=0.150037`, `max_abs_diff=0.244800`.
- `latqcc`: `pass=0/39`, `total_failures=40342`,
  `mean_abs_diff_mean=0.675258`, `max_abs_diff=14.760000`.
- `Q`: `pass=0/39`, `total_failures=2986`,
  `mean_abs_diff_mean=0.925027`, `max_abs_diff=194.715728`.
- `RM`: `pass=0/39`, `total_failures=10678`,
  `mean_abs_diff_mean=2.301802`, `max_abs_diff=204.850510`.
- `Snow-Water`: `pass=0/39`, `total_failures=24137`,
  `mean_abs_diff_mean=58.195696`, `max_abs_diff=562.470000`.
