# Contract-Test Implementation Evidence

Status: completed

Evidence mode: Static + Ran

Static:

- Added contract-derived diagnostic runner
  `docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py`.
- The runner implements `SC-EVAP-001#INV-EVAP-023` and
  `SC-WATBAL-001#INV-WATBAL-051` evidence collection:
  - builds `openwepp-cli-hill`,
  - runs H1/H7/H39 with `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=130`,
  - identifies first candidate/baseline WAT rows where
    `|candidate Ep - baseline Ep| > 0.05 mm`,
  - joins WAT `Ep`, `Total-Soil`, `SoilWaterTotal`, `Dp`, `latqcc`, `Q`,
    `RM`, and `Snow-Water`,
  - classifies WB17/SWU identity surfaces (`pmet_ep_m`, `Etp`, `Ep`, `ΣUi`,
    `Ws`, storage-to-threshold layers),
  - runs the full H1..H39 semantic comparator suite.

Ran:

- `/workdir/wepppy/.venv/bin/python -m py_compile docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py`
- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py --run-root /tmp/hphys0265_20260603T151958Z --trace-max-days 130`
- `/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0265-longer-season-ep-divergence-localization-closure-001/artifacts/hphys0265_diagnostics.py --run-root /tmp/hphys0265_20260603T151958Z --trace-max-days 130 --skip-full-suite`
