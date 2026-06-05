# Snow Publication Lifecycle Localization

Status: complete
Evidence mode: static + ran

## Finding

Static/Ran: The same-day snow publication lifecycle is now guarded from the
runoff reconciliation producer through WB13 consumption, but H1..H39 semantic
parity remains `0/39`. Therefore HPHYS0291 closes the lifecycle defect and
routes continuation upstream/downstream of publication rather than changing
WB13 `RM` math again.

## Producer Lifecycle

- Static: runoff reconciliation publishes both `snow.post_winter_rain_m` and
  `snow.routed_melt_m` through a named lifecycle helper.
- Static: the helper range-checks finite non-negative values and writes them to
  the flux surface for same-day consumption.
- Static: WB13 requires both snow publication terms from the flux surface.
- Static: trace/localization fields for these lifecycle symbols read from the
  flux surface only, not from flux-preferred state fallback.
- Ran: source-level contract tests and runner regression tests passed.

## Residual Localization

- Ran: H1/H7/H39 WB13-phase rows show `RM` is exactly the producer-owned
  snow publication flux in millimeters plus irrigation/post-winter rain where
  applicable.
- Ran: H39 2014-146 preserves the HPHYS0290 warm-rain/no-snow case:
  `post_winter_rain_m=0.002620000`, `routed_melt_m=0.000000000`,
  `RM=2.620000 mm`.
- Ran: H1/H7/H39 still show material `Snow-Water`, `Q`, `RM`, and
  `Total-Soil` residuals after publication hardening.

## Continuation Focus

Static/Ran: The next package should diagnose baseline-authoritative snow/liquid
partitioning before WB13 publication:

- snowpack retention/release state entering runoff reconciliation,
- winter/contin `wmelt` and rain-on-snow routing into runoff versus same-pass
  infiltration,
- infiltration/runoff partition capacity during high-melt days,
- downstream storage coupling only after producer fluxes and partitioning are
  proven coherent.

Do not reopen WB13 `RM` inference. WB13 is now a consumer of explicit producer
fluxes and should remain fail-closed.
