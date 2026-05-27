# Worker Handoff

Status: complete
Evidence mode: static+ran
Date: 2026-05-27

## Static
- WSHED06 is complete for scoped WS11 channel sediment publication-family
  closure.
- WS10 channel production writeback now emits:
  - `ws10_channel_{id}_qsed`
  - `ws10_channel_{id}_tc`
- WSHED03 WS11 sediment publication vector is active and passing.
- Residual `chnero/chnrt/detach` process-parity migration remains open and is
  explicitly retained as non-promotable.

### Immediate next actions
- Execute `WSHED07`: RK4/adaptive regime-transition impoundment migration.
- Execute `WSHED08`: watershed output row-model/parquet writer activation.
- Execute `WSHED09`: full validation/comparator rerun and hold-lift decision.

### Watch-items
- Preserve fail-closed typed guard posture for sediment payload intake and
  publication; do not add silent defaulting/clamping.
- Do not claim full channel sediment process parity until
  `chnero/chnrt/detach` branch families are migrated and validated.

## Ran
- Validation commands captured in `gate-results.md`.
