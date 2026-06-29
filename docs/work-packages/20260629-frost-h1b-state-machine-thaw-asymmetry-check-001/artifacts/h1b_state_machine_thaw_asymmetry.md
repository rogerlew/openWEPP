# H1b State-Machine Thaw-Asymmetry Check

Evidence class: Static + Ran.

## Determination

Verdict: `NARROW-EDGE`.

The top-down thaw path is present and branch-3 top retreat is visible in the H1b cells through `thdp` growth. True warm/material branch-3 days with neither `frdp` retreat nor `thdp` advance are rare in the full Sleepers scan.

Named blocking term: No branch guard blocks top retreat in the H1b cells. The apparent stall is the `frdp` bottom-extent detector staying fixed while branch 3 grows `thdp`, the surface-thawed cap; residual persistence is lower frozen-domain persistence, not absent top-front thaw.

## Static Code Path

Classification: `PRESENT`.

A top-down thaw path exists. `select_frost_branch` chooses branch 3 when signed surface flux is positive over an existing frozen column, `apply_active_frost_thaw_step` calls `thaw_fine_top_with_resistance_feedback`, and `thaw_fine_top_step` reduces fine-layer `slfsd_m`/`slsic_m` from the surface downward.

`frdp` is the bottom extent of the frozen domain. `thdp` is the thawed surface cap above a frozen segment. Top-front retreat may increase `thdp` without reducing `frdp` on the same day.

## H1b Cells

| Cell | Residual d | Warm material d | Branch-3 d | No `frdp` retreat d | With `thdp` retreat d | Max `frdp` m | Max `thdp` m | Blocking term |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| site2_sleepers_w9_hardwood_vt:1995:thaw | 18 | 6 | 6 | 6 | 6 | 0.136855 | 0.00997345 | none: branch 3 grows thdp while frdp bottom extent remains fixed |
| site2_sleepers_w9_hardwood_vt:2010:thaw | 50 | 6 | 6 | 4 | 4 | 0.105814 | 0.00989751 | none: branch 3 grows thdp while frdp bottom extent remains fixed |

## Generalization Scan

| Site | Branch-3 warm/material d | `frdp` retreat d | No `frdp` retreat d | No `frdp` + `thdp` retreat d | Neither retreat d | Neither fraction |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| site1_sleepers_south_field_vt | 288 | 247 | 41 | 32 | 9 | 0.031 |
| site2_sleepers_w9_hardwood_vt | 282 | 250 | 32 | 26 | 6 | 0.021 |

Aggregate:

- Branch-3 warm/material days: `570`.
- `frdp` retreat days: `497`.
- No-`frdp` days with `thdp` retreat: `58`.
- Days with neither `frdp` retreat nor `thdp` retreat: `15` (`0.026` of eligible).

## Routing

`proceed-to-ratification-with-bounded-residual-note`.

`GAP-SNOWFREEZE-002` remains open for the snow-persistence and snow-free wet-heat/Qwet routes, but the H1b state-machine structural gap is not a blocker for ratification.
