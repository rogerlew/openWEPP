# Worker Handoff

Status: T-B executed; T-C ready

Evidence mode: Static + Ran

## Current State

W-A characterized the watershed CLI and scoped the original routed-output work.
W-B cleared the no-impoundment parser seam. W-C cleared the WS10 channel guard
seam and published WAT-backed watershed outputs. W-D ran the totalwatsed3 audit
and fixed confirmed publication defects, but the W-D closure gate failed with
`closure_reconstructed_with_storage_total_mm=2950.498418`.

T-A applied the operator-directed architecture pivot: totalwatsed3 is
hillslope-only and must move to a dedicated openWEPP-native
`openwepp-cli-totalwatsed3`. It is not channel-routed watershed output and has
no channel loss/storage terms.

T-B implemented that dedicated CLI and produced an arboreal-dendrite
`totalwatsed3.parquet` that the wepppy audit can read without schema repair.
The live T-C blocker is now the remaining independent closure residual:
`57.409871 mm` (`0.345805%` of precipitation).

## T-A Scope Result

`totalwatsed3-cli-scope.md` is the controlling design artifact for T-B/T-C.
It records:

- authoritative semantics read from
  `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py` and
  `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`;
- the required hillslope inputs: PASS event rows, WAT rows, optional soil rows,
  optional element rows, and area/selector rules;
- PASS `runvol` as the independent `Runoff` operand;
- WAT flux/storage aggregation, including outlet-only MOFE `latqcc`;
- openWEPP-native output schema requirements with W-D unit/depth repairs;
- T-B red tests and T-C closure gates.

T-A sampled the arboreal-dendrite interchange schemas under
`/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/` and confirmed the
reference shape uses combined `H.pass.parquet`, `H.wat.parquet`,
`H.soil.parquet`, and `H.element.parquet` with `wepp_id`/`ofe_id` selectors.

## Remaining Implementation Gap

T-B created the openWEPP-native PASS/WAT aggregation surface needed before
closure can be claimed. T-C must now explain and close the remaining
`57.409871 mm` audit residual without substituting self-consistency checks for
the independent conservation identity.

## T-B Result

- Added `openwepp-cli-totalwatsed3`.
- Added native totalwatsed3 aggregation from PASS + WAT + optional
  soil/element parquets.
- Bound `Runoff` to PASS `runvol`; WAT `Q` remains diagnostic.
- Preserved MOFE outlet-only `latqcc`.
- Removed totalwatsed3 aggregation ownership from `openwepp-cli-watershed`.
- Added focused red/green tests and a unit-registry lineage regression.
- Ran the real arboreal-dendrite producer: `2192` rows emitted.
- Ran the wepppy audit read: zero profile violations; closure residual remains
  `57.409871 mm`, owned by T-C.

## Next Dispatch

```text
Execute increment T-C of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

## T-C Requirements

- Run `openwepp-cli-totalwatsed3` on arboreal-dendrite.
- Run the wepppy `totalwatsed3_daily_closure_audit.py` on the emitted parquet.
- Accept only independent nonzero-at-noise closure. Exact-zero closure on the
  real cohort is a tautology hold.
- On pass, update the package disposition, remove the ROADMAP deferral, and
  name `WATERSHED-CHANWB-ROUTED-OUTPUT` as the decoupled channel-output
  follow-on.

## Watchpoints

- `openwepp-cli-watershed.rs` and
  `crates/openwepp-watershed-output/src/writers.rs` are above the 2000-line
  warning threshold but below the 3000-line split threshold. T-C should avoid
  growing either unless a focused split is included.
- No production wepppy edits are in scope. wepppy is semantic/audit evidence
  only unless a future package explicitly scopes cross-repo changes.
