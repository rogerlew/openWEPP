# Worker Handoff

Status: WSHED01 COMPLETE 2026-06-14 — T-C closed (totalwatsed3 native closure
resolved the WBVAL06/6a deferral). Next package: MOFE-FARPOINT01.

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
reviewed defective. T-B2-REDO corrected to `Q * outlet Area`, which review
found under-scaled runoff. T-B2-REDO2 corrected native PASS `runvol` to
`QOFE * outlet Area`. totalwatsed3 can consume the corrected per-hillslope
PASS/WAT files directly.

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

## Remaining Implementation Gap — CLOSED

T-B2-REDO2 created the corrected openWEPP-native PASS/WAT aggregation surface;
T-C (documentation/governance) performed final closure disposition on the REDO2
output. The audit residual is the expected storage-init shape: `30.544142 mm`
whole run, day 1 `+30.9533178099056 mm`, ex-day-1 basic-storage residual
`-0.409175395336963 mm` over `2191` days. No implementation gap remains in
WSHED01 scope; see **T-C Result** and **Next Package(s)** below.

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

## T-B2-REDO2 Result

- Corrected MOFE PASS `runvol` to
  `outlet.row.wb13_row.qofe * outlet.row.wb13_row.area / 1000`, deleting the
  crossed `Q * outlet Area` pairing.
- Focused regression:
  `mofe01_tb2_redo2_pass_runvol_uses_qofe_outlet_area_not_q_outlet_area`.
- Corrected arboreal-dendrite evidence root:
  `/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z/`.
- Real rerun outputs: `36` HBP, `36` WAT, `36` PASS parquet, `36` manifests.
- HBP/WAT anchor comparison vs `/tmp/openwepp_mofe01_mi_final/output`:
  `anchor_mismatches=0`.
- PASS QOFE-area audit: `78912` rows,
  `max_abs_pass_minus_qofe_area_m3=0.0`,
  `sum_runvol=27691217.37511973 m3`.
- Native totalwatsed3 output:
  `/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z/totalwatsed3.parquet`,
  `2192` rows.
- wepppy audit read:
  `closure_reconstructed_with_storage_total_mm=30.544142`;
  day 1 `+30.9533178099056 mm`; ex-day-1 basic-storage residual
  `-0.409175395336963 mm`, `0` days above `1 mm`.

## T-C Result (documentation/governance closure, Claude 2026-06-14)

T-C's substantive closure gate was met by the T-B2-REDO2 run; T-C recorded the
resolution (no further production code). Independent Claude verification (Ran)
on the REDO2 root confirmed genuine closure:

- `Σ runvol = 27.691 Mm³` (coeff 0.554); runoff < precip every year (two-sided
  bound holds), and **independent** of the WAT-`Q` column (`Σ Q = 18.895 Mm³`).
- Identity `P − (Runoff + Lateral + ET + Perc + Interception) − ΔStorage` closes
  ex-day-1 at `−0.41 mm` over 2191 days; daily residuals `[−0.248, +0.005] mm`
  (nonzero-at-noise, not 0==0). Day-1 `+30.95 mm` is the storage-prepend init
  (producer-agnostic, benign).
- Anchors byte-identical (`anchor_mismatches=0`); MOFE physics untouched.

Closure deliverables done: WBVAL06/6a deferral resolved; `docs/ROADMAP.md`
item 1 removed (queue renumbered; next = `MOFE-FARPOINT01`);
`docs/work-packages/README.md` item 9 marked complete with the W-arc→T-arc
pivot, ADR-0019/0020, and the runvol arc; this handoff names the follow-ons.

## Next Package(s)

- **`MOFE-FARPOINT01`** (ROADMAP queue item 1, next): take MOFE routing to a
  >10-OFE substrate where legacy's WB defect appears and show the three-identity
  closure holds past the legacy ceiling.
- **`WATERSHED-CHANWB-ROUTED-OUTPUT`** (decoupled follow-on): the channel
  water-balance routed watershed output (`chanwb`/`chnwb`), distinct from the
  hillslope-only totalwatsed3 per ADR-0020. The W-B/W-C watershed-CLI seams
  (no-impoundment parse, WS10 zero-sediment/`nchnum=0` guards, WAT-backed
  publication) landed under WSHED01 but channel routed water-balance output is
  its own rung.
- **`MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF`**: sediment-coupled routing
  (deferred from MOFE01 M-G).

## Watchpoints

- `openwepp-cli-watershed.rs` and
  `crates/openwepp-watershed-output/src/writers.rs` are above the 2000-line
  warning threshold but below the 3000-line split threshold. T-C should avoid
  growing either unless a focused split is included.
- No production wepppy edits are in scope. wepppy is semantic/audit evidence
  only unless a future package explicitly scopes cross-repo changes.
