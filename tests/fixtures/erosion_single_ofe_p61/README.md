# erosion_single_ofe_p61 — single-OFE Wave-1 erosion regression fixture

Single-OFE hillslope with real climate that produces Wave-1
sediment-continuity **erosion events** through the direct-production
runtime (SC-SED-001 1b-C). Used to regression-guard that the enabled
single-OFE erosion solve produces nonzero sediment.

Provenance: `/wc1/runs/as/assisted-weakness/wepp/runs/p61.*`
(operator-supplied, 2026-07-04). Legacy WEPP `H61.ebe.dat` reports 4
erosion events; the dominant event (12.5 mm runoff, Sed.Del 4.2 kg/m)
clears the Wave-1 `passby` gate. The `frost.txt` is a generic default
(the source run had none); `p61.run` is the openWEPP TOML runfile wrapper.

E.1 note (2026-07-04): openWEPP publishes `tdet`/`tdep` in **total kg**
(scaled by the slope-file `fwidth = 724.3 m`, the legacy `sedseg.for`
pass-file convention); the per-width value is `tdet / fwidth`. When
comparing to `H61.ebe.dat`, `Sed.Del` is per-width **delivery** (kg/m,
export at the toe), not detachment — compare it to the exported load,
not `tdet` (the 1b-C "~5×" note conflated those cut-points).
