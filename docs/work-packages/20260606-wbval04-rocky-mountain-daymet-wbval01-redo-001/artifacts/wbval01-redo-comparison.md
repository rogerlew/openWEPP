# WBVAL01 Redo Comparison

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Static:

- WBVAL01 prior outcome:
  - `12/22` single-OFE hillslopes emitted WAT and all `12` were
    conservation-break for years `2..6`.
  - `6/22` failed closed on `CLIM-RUNTIME-E-017`:
    `p2`, `p4`, `p6`, `p9`, `p14`, `p17`.
  - `4/22` failed closed on `HKERNEL-WB11-PERC-E-003`:
    `p7`, `p11`, `p18`, `p20`.
- WBVAL02 closed the six `CLIM-RUNTIME-E-017` rows as invalid upstream
  radiation input and preserved the typed daily source guard.
- WBVAL03 held legitimately because the same upstream climate defect blocked
  the J-95 and WAT residual surfaces before the current WEPPpy rebuild.

Ran:

- Current WBVAL04 climate audit passed with zero CLI radiation bound
  exceedances.
- Current WBVAL04 batch ran all `22` single-OFE hillslopes with release
  `openwepp-cli-hill`.
- Current WBVAL04 emitted WAT for `18/22` hillslopes.
- Current WBVAL04 fail-closed set is only the prior J-95 group:
  `p7`, `p11`, `p18`, `p20`.

Comparison table:

| Group | WBVAL01 | WBVAL04 | Interpretation |
|---|---|---|---|
| Prior radiation blockers: `p2`, `p4`, `p6`, `p9`, `p14`, `p17` | `CLIM-RUNTIME-E-017`, no WAT | WAT emitted; all conservation-break | WEPPpy publication-safe radiation rebuild unblocked these surfaces without loosening openWEPP guards. |
| Prior J-95 blockers: `p7`, `p11`, `p18`, `p20` | `HKERNEL-WB11-PERC-E-003`, no WAT | same kernel message at `sim_day_index=95`, no WAT | Remaining fail-closed defect class is reproducible on current valid climate. |
| Prior WAT emitters: `p1`, `p3`, `p5`, `p8`, `p10`, `p12`, `p13`, `p15`, `p16`, `p19`, `p21`, `p22` | WAT emitted; conservation-break | WAT emitted; conservation-break | Complete-identity annual residual remains after climate fix. |
| Newly unblocked WAT emitters: `p2`, `p4`, `p6`, `p9`, `p14`, `p17` | no WAT due radiation guard | WAT emitted; conservation-break | These join the residual defect population. |
| `pw0` | observed-only multi-OFE preview | not run | Remains outside single-OFE closure scope. |

Defect-shaped follow-ons:

| Defect ID | Observable failure | Suspected mechanism | Owning write set | Failing fixture | Authority | Acceptance target | Legitimate HOLD conditions |
|---|---|---|---|---|---|---|---|
| `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` | `p7`, `p11`, `p18`, and `p20` fail closed at `sim_day_index=95`, `calendar_year=1990`, `julian_day=95` with `HKERNEL-WB11-PERC-E-003`; no WAT emitted. | WB11/WB18 percolation/deep seepage state transition reaches a domain guard under valid climate. | `SC-PERC-001`, percolation/deep seepage kernel/runtime projection tests and implementation files named by the follow-on DC-ExecPlan. | `/wc1/runs/in/indispensable-presenter/wepp/runs/p7.*`, `p11.*`, `p18.*`, `p20.*` wrappers from WBVAL04. | `SC-PERC-001`, `SC-WATBAL-001`, pinned baseline `/workdir/wepp-forest_260430_baseline` if legacy migration evidence applies. | Valid climate runs reach WAT publication or fail closed at a newly proven out-of-envelope boundary; no guard loosening or canonicalize-and-proceed. | Missing or contradictory canonical authority, invalid fixture input outside openWEPP, or root cause outside percolation/deep seepage envelope. |
| `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL` | All `18` WAT emitters have annual complete-identity residuals above `1.0 mm/year` for years `2..6`; max current residual is `94.433 mm` (`p4`, year `5`). | WAT publication and/or vertical water-balance transfer accounting omits or mis-signs a storage/flux term after valid-climate runtime reaches publication. | `SC-WATBAL-001`, WAT publication, water-balance accounting tests and implementation files named by the follow-on DC-ExecPlan. | WBVAL04 WAT outputs for `p1`, `p2`, `p3`, `p4`, `p5`, `p6`, `p8`, `p9`, `p10`, `p12`, `p13`, `p14`, `p15`, `p16`, `p17`, `p19`, `p21`, `p22`. | `SC-WATBAL-001`, `SC-PERC-001`, `SC-SNOWFREEZE-001`, pinned baseline `/workdir/wepp-forest_260430_baseline` if legacy migration evidence applies. | Years `2..6` complete-identity residuals are within the contract tolerance, or the residual is reclassified at a declared authority boundary with explicit missing surface evidence. | Missing initial storage may keep year `1` unclassified, but not years `2..6`; contradictory authority or an out-of-envelope publication surface may justify HOLD. |

Conclusion: WBVAL04 closes the climate precondition and WBVAL01 redo
validation objective, but package disposition remains `executed-hold` because
valid-climate invariant failures remain and require separate DC-ExecPlans.
