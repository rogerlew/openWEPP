# HPHYS0220 Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

## Scope
- Execute coupled residual diagnostics for HPHYS0218 vs HPHYS0219.
- Execute static WB19 source-lineage audit against baseline `watbal.for`.
- Publish remediation-planning outcome with explicit next package scope.

## Ran coupled-delta findings
- `Dp` mean residual improved for **39/39 hillslopes** (HPHYS0219 vs HPHYS0218).
- `latqcc` mean residual regressed for **39/39 hillslopes**.
- `Total-Soil` mean residual regressed for **39/39 hillslopes**.
- `SoilWaterTotal` mean residual regressed for **39/39 hillslopes**.
- Correlations across 39 hillslopes:
  - `corr(ΔDp, Δlatqcc) = -0.9997641396512593`
  - `corr(ΔDp, ΔTotal-Soil) = -0.9007897054173599`
  - `corr(Δlatqcc, ΔTotal-Soil) = 0.895086530846899`

## Static lineage findings
- Baseline WB19 (`watbal.for`) includes additional lateral-flow coupling
  surfaces not represented in current openWEPP WB19 kernels:
  - water-yield coupling:
    `watyld = avpora - (avfca + (1.0-avcoca))`
  - saturated-depth state update:
    `fcdep = fcdep - (latqcc/watyld)`
  - `unsdep` recomputation from updated `fcdep`
  - explicit `drfc`-threshold usage in layer withdrawal loops and water-table
    positioning logic.
- Current openWEPP WB19 logic implements `drfc`-threshold withdrawals but does
  not currently model baseline `avcoca`/`watyld`/`fcdep` coupling surfaces.

## Interpretation
- The deterministic 39/39 inverse directional shift strongly indicates a
  structural flux-partition tradeoff under current simplified WB19 coupling.
- Coefficient-family correction (`coca`) was necessary but not sufficient for
  integrated residual closure.
