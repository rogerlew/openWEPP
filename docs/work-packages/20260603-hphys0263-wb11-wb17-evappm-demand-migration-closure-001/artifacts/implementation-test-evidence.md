# Implementation/Test Evidence

Status: completed

Evidence mode: static + ran

Static:

- `crates/openwepp-runner/src/hillslope/mod.rs`
  - Preserves the existing Priestley-Taylor branch when `iflget == 1`.
  - Selects migrated PMET demand when `pmetpara.mode.iflget != 1`.
  - Computes and publishes `pmet.etorc_mm`, `pmet.rn_mj_m2`, `pmet.fwv_m_s`,
    `pmet.rhd_pct`, `pmet.kcbadj`, `pmet.kcbcon`, `pmet.etke`, `pmet.etkr`,
    `pmet.etks`, `pmet.tew_mm`, `pmet.rew_mm`, `pmet.wfevp_mm`,
    `pmet.taw_mm`, `pmet.raw_mm`, `pmet.wftrp_mm`, `pmet.es_m`, and
    `pmet.ep_m`.
  - Uses `pmet.ep_m` as `wb11_et_demand` for PMET-mode WB17/SWU consumption.
  - Derives missing runtime `radpot` from pinned `sunmap` horizontal potential
    radiation using `deglat` and calendar date.
  - Restores default `pmetpara.txt` discoverability in runfile-sidecar mode.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs`
  publishes `deglat` and `elevm` for EVAPPM radiation/pressure lineage.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
  publishes the already-computed baseline-derived `canhgt` scalar for EVAPPM.
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs` publishes
  matching per-hillslope `deglat` and `elevm` symbols to preserve adapter
  parity.
- `docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/hphys0263_diagnostics.py`
  runs the full 39-hillslope suite and classifies H1/H7/H39 PMET migration
  evidence.

Ran:

- `cargo test -p openwepp-runner hphys026 -- --nocapture`
  - Result: passed, `5 passed`.
- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0263-wb11-wb17-evappm-demand-migration-closure-001/artifacts/hphys0263_diagnostics.py --run-root /tmp/hphys0263_20260603T070311Z`
  - Result: passed.
  - H1/H7/H39 classification: `EVAPPM_MIGRATED_BRANCH_OBSERVED`.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed.
- `cargo test --workspace`
  - Result: passed.
- `cargo deny check`
  - Result: passed with existing duplicate/unmatched-license warnings.
