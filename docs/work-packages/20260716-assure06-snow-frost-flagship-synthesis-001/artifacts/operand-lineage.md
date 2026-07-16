# ASSURE-06 Operand Lineage

Evidence class: Static

| Reported quantity | Units/support | Authoritative source | Reconstruction | Rejected aliases |
| --- | --- | --- | --- | --- |
| Phase observations scored | rows | Jennings `rows_scored` and exact scorer | Sum the four confusion-matrix cells for either formulation; separately retain read, skipped, and scored counts | rows read or skipped represented as scored; station count |
| Phase accuracy | fraction of scored rows | Jennings Harder-Pomeroy and fixed-threshold confusion matrices | `(RR + SS) / (RR + RS + SR + SS)` for each formulation, retaining all eight integer cells | station threshold score; mixed-phase fraction; snowpack agreement |
| Station threshold bias/MAE | degrees Celsius across 6,883 stations | Jennings `threshold_summary` | Direct aggregate with station count retained | hourly classification accuracy |
| Humidity contrast | degrees Celsius | Jennings `humidity_threshold_contrast` | High-RH minus low-RH observed and predicted threshold means | pooled temperature threshold |
| Snow profile labels and physical diagnostics | correlated cells and daily pairs across ten named surfaces | current-default site summaries and profiles | Retain per-site paired count, fail/marginal/pass/strong counts, density KGE, peak/meltout offsets, and depth-SWE slope ratio; pooled counts are secondary | independent-trial success rate; all-cell counts including forcing-limited or unavailable cells |
| Snow paired observations | daily paired rows per surface | current-default site summaries | Sum only named site `paired_count` values | trace-row count; observation-manifest total rows |
| Residual families | robust cells | post-partition decomposition | Sum density, depth-density geometry, and timing counts to 15 | forcing-limited magnitude labels; package activation decision |
| Frost-tube residual | meters at matched dates | non-SNOTEL site metrics | Site-specific observed-minus-modeled residual statistics | snow-depth residual; zero-isotherm upper-bound margin |
| Temperature-isotherm exceedance | count of evaluated dates | non-SNOTEL site metrics | Site-specific exceedance count divided by declared bound count when displayed | frost-tube residual count; total matched temperature rows |
| Snow-control failure | paired dates | non-SNOTEL site metrics | Site-specific failed controls divided by paired snow controls | absence of paired snow observations interpreted as a pass |
| Phase conservation | meters per active precipitation row | activation trace | `rain + snow - active precipitation`, maximum absolute residual | phase-classification error |
| Snow storage closure | millimeters at two selected OFE-days | content-identified production WAT operand log | Recalculate `prior Snow-Water + P - RM - current Snow-Water` for accumulation and release rows | physical Snow-Depth; snowbench state; all-row closure |
| Frozen-soil storage closure | millimeters at two selected OFE-days | content-identified production WAT operand log | Recalculate prior liquid plus frozen storage, input, sink, and current liquid plus frozen storage in retained arithmetic order | frost depth; freeze/thaw transfer treated as external water; all-row closure |

Every claim-bearing table or figure must use these operands through strict result
bindings. Derived percentages must retain their integer numerator and
denominator. Ordinal profile scores may appear only with their rubric meaning,
correlated-cell boundary, and site-resolved physical diagnostics; they may not
be graphed as universal model accuracy.
