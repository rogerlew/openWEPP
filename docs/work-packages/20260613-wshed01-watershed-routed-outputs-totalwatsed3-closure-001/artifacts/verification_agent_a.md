# Verification Agent A

Status: T-B2-REDO2 local verification complete

Evidence mode: Ran + Static

## Verification Record

Verified:

- Current CLI failure command was run and recorded.
- Output directory contained zero files after failure.
- `jpond=0` finding is classified and cites openWEPP + legacy lines.
- `watershed-routing-scope.md` documents routing, output, totalwatsed3 schema,
  conservation identity, and W-B/W-C/W-D red tests.
- No production source file was changed.

Residual risk:

- W-A could not observe post-impoundment routing because the current parser
  blocker prevents reaching `chan.inp`, HBP parsing, dispatch, or output
  writing.

## W-D Verification Record

Evidence mode: Ran + Static

Verified:

- Fresh configured and legacy-discovery watershed CLI runs exited `0`.
- Both runs emitted `2192` `totalwatsed3.parquet` rows.
- wepppy `totalwatsed3_daily_closure_audit.py` ran against both outputs.
- Publication repairs are present: exact hydrology columns publish as `m^3`,
  depth aliases remain mm, `latqcc` uses the outlet-facing OFE, and
  profile/interception fields publish.
- Profile audit violations are zero for the W-D runs.
- Final gates passed: fmt, clippy, focused tests, workspace tests, deny, diff
  check, and scoped markdown lint.

Residual blocker:

- The conservation audit still reports
  `closure_reconstructed_with_storage_total_mm=2950.498418` for both W-D
  outputs. The remaining missing input is independent daily PASS `runvol`
  lineage, so W-D is verified as `executed-hold`, not complete.

## T-A Verification Record

Evidence mode: Static + Ran

Verified:

- `totalwatsed3-cli-scope.md` exists and covers all T-A required sections:
  inputs, aggregation semantics, output schema, independent closure identity,
  red tests, and T-B/T-C breakdown.
- The artifact cites the correct wepppy producer path:
  `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py`.
- The artifact records the wepppy audit closure semantics from
  `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`.
- The pyarrow schema sample was run against the arboreal-dendrite
  `H.pass.parquet`, `H.wat.parquet`, `H.soil.parquet`, and
  `H.element.parquet` files.
- No production source files were edited.

Residual blocker:

- T-B must implement the dedicated CLI and PASS `runvol` lineage before T-C can
  claim totalwatsed3 closure.

## T-B Verification Record

Evidence mode: Ran + Static

Verified:

- `openwepp-cli-totalwatsed3` exists as a dedicated binary.
- Focused CLI tests passed (`2` tests).
- Unit-registry lineage tests passed (`15` tests).
- The real arboreal-dendrite producer emitted `2192` rows.
- The wepppy audit script read the T-B output without schema repair.
- Profile audit violations are zero.
- Full Rust gates passed: fmt, clippy, workspace tests, and deny.

Residual blocker:

- The conservation audit still reports
  `closure_reconstructed_with_storage_total_mm=57.409871`. T-B is verified as
  executed for producer/audit-read gates; T-C owns closure.

## T-B2 Verification Record

Evidence mode: Ran + Static

Verified:

- `outputs.pass_parquet` is present as an optional hillslope output path.
- `hillslope_pass` parquet writer/schema exists and is unit-registry covered.
- Focused T-B2 tests passed.
- Real arboreal-dendrite rerun emitted `36` native PASS parquet files.
- HBP/WAT anchor comparison reported `anchor_mismatches=0`.
- PASS `runvol` matches outlet WAT `QOFE * area / 1000` at
  `1.4551915228366852e-11 m^3` max absolute difference over `78912` rows.
- Native totalwatsed3 production emitted `2192` rows.
- Full Rust gates passed: fmt, clippy, workspace tests, and deny.

Residual blocker:

- T-C still must run and close the totalwatsed3 conservation audit on the
  native output.

Supersession:

- This T-B2 verification record is superseded by
  [review-tb2-runvol-area-defect.md](review-tb2-runvol-area-defect.md) and
  T-B2-REDO. The `QOFE * area` identity was a defective self-consistency
  surface for MOFE runoff-volume acceptance.

## T-B2-REDO Verification Record

Evidence mode: Ran + Static

Verified:

- The focused regression test
  `mofe01_tb2_redo_pass_runvol_uses_published_q_area_not_qofe_area` passes and
  rejects the old `QOFE * publication area` result.
- `openwepp-cli-hill` reran arboreal-dendrite p1-p36 under
  `/tmp/openwepp_wshed01_tb2_redo_qarea` and emitted `36` HBP, `36` WAT,
  `36` PASS parquet files, and `36` manifests.
- HBP/WAT anchor comparison against `/tmp/openwepp_mofe01_mi_final/output`
  reports `anchor_mismatches=0`.
- The corrected PASS dual audit reports
  `max_abs_pass_minus_q_area_m3=0.0` over `78912` rows, while the old
  `QOFE * Area` surface differs by up to `21766.4323911278 m3`.
- The water-year annual precipitation bound reports `violation_count=0` over
  `252` hillslope-water-years.
- Native totalwatsed3 production emitted `2192` rows from corrected PASS/WAT
  files, and summed `runvol` differs from PASS by
  `9.313225746154785e-10 m3`.
- Full Rust gates passed: fmt, clippy, workspace tests, and deny.

Residual blocker:

- The corrected-output wepppy audit still reports
  `closure_reconstructed_with_storage_total_mm=6948.564523`. T-C owns closure.

Supersession:

- This T-B2-REDO verification record is superseded by T-B2-REDO2. REDO used
  `Q * outlet Area`, which under-scaled native PASS `runvol`.

## T-B2-REDO2 Verification Record

Evidence mode: Ran + Static

Verified:

- The focused REDO2 regression passes and rejects `Q * outlet Area`,
  `QOFE * internal OFE area`, `QOFE * publication Area`, and per-OFE summed
  volume aliases.
- `hillslope_pass.runvol` unit metadata now names outlet `QOFE` and outlet WAT
  row area.
- `openwepp-cli-hill` reran arboreal-dendrite p1-p36 under
  `/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z` and emitted
  `36` HBP, `36` WAT, `36` PASS parquet files, and `36` manifests.
- HBP/WAT anchor comparison reports `anchor_mismatches=0`.
- The PASS audit reports `max_abs_pass_minus_qofe_area_m3=0.0` over
  `78912` rows.
- Native totalwatsed3 production emitted `2192` rows and the wepppy audit
  reports `closure_reconstructed_with_storage_total_mm=30.544142` with
  ex-day-1 basic-storage residual `-0.409175395336963 mm`.
- Full Rust gates passed: fmt, clippy, workspace tests, and deny.
