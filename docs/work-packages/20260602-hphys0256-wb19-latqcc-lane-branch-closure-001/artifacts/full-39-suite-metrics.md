# Full 39 Suite Metrics

Status: completed/HOLD

Evidence mode: ran

- Ran: `/workdir/wepppy/.venv/bin/python docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_diagnostics.py --run-root /tmp/hphys0256_20260603T003117Z`.
- Ran: semantic pass remains `0/39`.
- Ran: metrics are unchanged from HPHYS0254 because the run manifest selected
  the hourly lane (`selected_lane: hourly`, `effective_mode: hourly`,
  `substep_count: 24`).

## Measure Summary

- Ran: `Ep`: pass `0/39`, fail count `56391`, mean abs diff `1.700230`,
  max abs diff `7.779928`.
- Ran: `Total-Soil`: pass `0/39`, fail count `56941`, mean abs diff
  `167.165068`, max abs diff `618.513538`.
- Ran: `SoilWaterTotal`: pass `0/39`, fail count `56941`, mean abs diff
  `167.165068`, max abs diff `618.513538`.
- Ran: `Dp`: pass `0/39`, fail count `41028`, mean abs diff `0.172796`,
  max abs diff `0.240000`.
- Ran: `latqcc`: pass `0/39`, fail count `39871`, mean abs diff `0.805148`,
  max abs diff `28.005815`.
- Ran: `Q`: pass `0/39`, fail count `2986`, mean abs diff `0.925027`,
  max abs diff `194.715728`.
- Ran: `RM`: pass `0/39`, fail count `10678`, mean abs diff `2.301802`,
  max abs diff `204.850510`.
- Ran: `Snow-Water`: pass `0/39`, fail count `24137`, mean abs diff
  `58.195696`, max abs diff `562.470000`.
