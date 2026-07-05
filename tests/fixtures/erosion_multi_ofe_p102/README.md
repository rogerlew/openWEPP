# erosion_multi_ofe_p102 — E.3 multi-OFE Wave-1 chain fixture

Real 2-OFE disturbed-forest hillslope from the WSHED-W7DC01 substrate
(`insensible-aliquot` H102): OFE-1 "forest high sev fire" over OFE-2
"forest moderate sev fire" (same loam texture family, differing
severity/erodibility). Climate truncated from the original 50-year
CLIGEN series to years 1-10 (`Years simulated` rewritten accordingly).

Exercises (ROADMAP §E.3 / SC-SED-001 rev 44 `INV-SED-016`):
- the per-lane per-OFE seed slicing and the inter-OFE hourly erosion
  handoff (`qout_h`/`qsout_h`/fractions/continuity end state);
- the `param.for:249-390` shear/transport coefficient continuity;
- the EXIT-scoped chain EVENT with chain-aggregated `tdet`/`tdep` and
  the telescoped intake closure `Σ S_h(exit) = Σ_lanes(tdet − tdep)`;
- the D5 manifest lift (`erod14_qin_sediment_coupled = true`,
  `wave1-hourly-sediment-coupled-handoff`).

The §4a soil-contrast observable (per-OFE particle classes from each
OFE's own soil) is exercised by the in-test texture-modified variant
(OFE-2 coarsened), following the `dff_ws2` in-test soil-edit pattern.

Sidecars (`pmetpara`/`frost`/`snow`/`wepp_ui`) are the standard
single-hillslope set shared with `erosion_single_ofe_p61`.
