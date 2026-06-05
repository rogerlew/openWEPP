# Disposition

Status: executed-hold
Evidence mode: Static/Ran

## Decision

HPHYS0289 is executed and held for continuation.

## Completed

- Contract authority amended for WB13 `RM`/`Snow-Water` publication lineage.
- Kernel publishes `snow.routed_melt_m`.
- WB13 `RM` consumes routed melt instead of raw precipitation plus SWE-delta proxy.
- Behavioral tests cover snow-active routed melt, missing routed melt, warm-rain/no-snow, flux-over-state shadowing, and negative routed melt.
- Full H1..H39 runtime completes and semantic metrics are recorded.
- Dual review findings are dispositioned.

## Hold Rationale

Full H1..H39 semantic pass remains `0/39`. `RM` fail count improved by `765`, but mean absolute `RM` residual worsened by `0.010391`, and `Q`/`Snow-Water` did not move. Target traces show WB13 still needs an explicit post-winter rain publication surface rather than inferring post-winter rain from raw `prcp` and snow-state activity.

## Continuation Recommendation

Scaffold HPHYS0290 for explicit post-winter `rain(iplane)` publication/consumption. Scope should trace `contin.for` rain clearing/restoration, `winter.for` residual rain release into `wmelt`, and WB13 `RM` publication rows including H39 2014-146.

## Evidence

Ran:

- Final gates: `/tmp/hphys0289_final_broad_gates_20260605T001506Z.log`
- Full suite root: `/tmp/hphys0289_full_release_current_20260605T000159Z`
- Target trace root: `/tmp/hphys0289_target_traces_current_20260605T000516Z`
