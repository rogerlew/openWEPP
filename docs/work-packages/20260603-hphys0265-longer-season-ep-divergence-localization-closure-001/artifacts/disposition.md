# HPHYS0265 Disposition

Status: completed/HOLD

Evidence mode: Static + Ran

## Decision

HOLD.

## Rationale

The package achieved the HPHYS0264 next-focus objective: it localized the first
large longer-season `Ep` divergence for H1/H7/H39 and separated PMET/WB17
identity ownership from storage/snow/lateral context.

At each first divergence:

- PMET seam closes: `pmet_ep_m = Etp`.
- WB17/SWU publication identities close: `Ep = ΣUi`, aggregate `Ui = ΣUi`, and
  `Ws = Ep/Etp`.
- SWU stress is active with four stress-limited layers.
- Same-day storage, snow/runoff, lateral-flow, or runoff-melt context is
  already material.

Therefore, no baseline-authoritative WB17/SWU production defect was identified.
Per `SC-EVAP-001#GAP-EVAP-011` and `SC-WATBAL-001#INV-WATBAL-051`, production
patching would be heuristic and is not allowed.

## Continuation Recommendation

Scaffold the next package around root-zone layer storage and snow/runoff/lateral
coupling before revisiting SWU math. The high-value target is explaining why
the first-divergence days have stress-limited layers despite PMET demand closure:

- H1/H7 start with `Total-Soil`/`SoilWaterTotal` about `6.7-7.0 mm` drier than
  baseline and lower `latqcc`.
- H39 is `24.9 mm` wetter in aggregate but still has stress-limited lower root
  layers and much lower `latqcc`.
- H7 also has candidate runoff/melt input (`RM=0.074545 mm`) when baseline has
  zero.

Focus likely belongs to layer distribution, frost/snow/runoff timing, and
lateral withdrawal/storage projection lineage, not aggregate WB13 publication
or PMET/WB17 seam wiring.
