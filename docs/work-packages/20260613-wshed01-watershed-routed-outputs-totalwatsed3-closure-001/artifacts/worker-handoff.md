# Worker Handoff

Status: T-B2-REDO executed; T-C ready on corrected native outputs

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

T-B2 then replaced the remaining legacy-input dependency for runoff delivery,
but its first MOFE `runvol` formula used `QOFE * publication area` and was
reviewed defective. T-B2-REDO corrected native PASS `runvol` to the published
`Q * Area` dual. totalwatsed3 can consume the corrected per-hillslope PASS/WAT
files directly.

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

T-B2-REDO created the corrected openWEPP-native PASS/WAT aggregation surface
needed before closure can be claimed. T-C must now explain and close the
remaining `6948.564523 mm` audit residual on corrected native output without
substituting self-consistency checks for the independent conservation identity.

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

## T-B2 Result

- Added optional `outputs.pass_parquet` to hillslope runfiles.
- Published `HillslopePassRow` parquet from openWEPP-controlled runoff
  delivery data.
- First MOFE `runvol` formula used terminal outlet
  `current_transfer_output.qofe * publication_area_m2`; review later found
  that formula over-scaled runoff and seeded T-B2-REDO.
- Totalwatsed3 now discovers and consumes native per-hillslope
  `H*.pass.parquet`/`H*.wat.parquet` files.
- Real arboreal-dendrite evidence root:
  `/tmp/openwepp_wshed01_tb2/`.
- Real rerun outputs: `36` HBP, `36` WAT, `36` PASS parquet.
- HBP/WAT anchor comparison vs `/tmp/openwepp_mofe01_mi_final/output`:
  `anchor_mismatches=0`.
- Native totalwatsed3 output:
  `/tmp/openwepp_wshed01_tb2/totalwatsed3.parquet`, `2192` rows.
- PASS identity audit: `78912` rows,
  `max_abs_runvol_diff_m3=1.4551915228366852e-11`.

## T-B2-REDO Result

- Corrected MOFE PASS `runvol` to
  `outlet.row.wb13_row.q * outlet.row.wb13_row.area / 1000`, deleting the old
  `QOFE * publication area` self-consistency surface.
- Focused regression:
  `mofe01_tb2_redo_pass_runvol_uses_published_q_area_not_qofe_area`.
- Corrected arboreal-dendrite evidence root:
  `/tmp/openwepp_wshed01_tb2_redo_qarea/`.
- Real rerun outputs: `36` HBP, `36` WAT, `36` PASS parquet, `36` manifests.
- HBP/WAT anchor comparison vs `/tmp/openwepp_mofe01_mi_final/output`:
  `anchor_mismatches=0`.
- PASS dual audit: `78912` rows,
  `max_abs_pass_minus_q_area_m3=0.0`; old `QOFE * Area` formula differs by up
  to `21766.4323911278 m3`.
- Water-year annual bound: `252` hillslope-water-years,
  `violation_count=0`, `max_runvol_precip_ratio=0.9857497687436844`.
- Native totalwatsed3 output:
  `/tmp/openwepp_wshed01_tb2_redo_qarea/totalwatsed3.parquet`, `2192` rows.
- wepppy audit read:
  `closure_reconstructed_with_storage_total_mm=6948.564523`; T-C owns this
  residual.

## Next Dispatch

```text
Execute increment T-C of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```

## T-C Requirements

- Run `openwepp-cli-totalwatsed3` on corrected arboreal-dendrite native
  PASS/WAT output, not `/tmp/openwepp_wshed01_tb2`.
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
