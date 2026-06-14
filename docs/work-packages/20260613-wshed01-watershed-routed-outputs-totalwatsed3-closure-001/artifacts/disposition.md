# Disposition

Status: COMPLETE 2026-06-14 — T-C closed

Evidence mode: Ran + Static

W-A, W-B, W-C, the keepable W-D publication repairs, T-A design scope,
T-B dedicated CLI, T-B2/REDO/REDO2 native runvol (over→under→correct), and the
T-C documentation closure are complete. **totalwatsed3 closes on openWEPP-native
output** at the established floor with independent operands (ex-day-1
−0.41 mm/2191 d; `Σ runvol` 27.691 ≠ `Σ Q-col` 18.895 Mm³; nonzero-at-noise),
resolving the WBVAL06/6a deferral. The package is closed; follow-ons
(MOFE-FARPOINT01, WATERSHED-CHANWB-ROUTED-OUTPUT,
MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF) are named in `worker-handoff.md`.

Disposition:

- W-A baseline: fail-closed before watershed output writing at
  `CLIWAT-E-010`/`IMP-E-004`.
- `jpond=0`: fixed as a parser defect on valid no-impoundment input.
- Runfile contract: schema v1 still requires `inputs.pw0_imp`; an explicit
  supported `.imp` file with `jpond=0` is accepted only when `pw0_str` declares
  zero impoundments.
- W-B arboreal-dendrite behavior: proceeds past `CLIWAT-E-010`; next observed
  hard stop is `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`; output file
  count remains `0`.
- W-C hard-stop classification: valid zero-sediment HBP payload and `nchnum=0`
  output-disabled channel state were rejected by over-strict WS10 guards.
- Routing/output result: arboreal-dendrite configured and legacy-discovery
  runs exit `0`, emit all `14` watershed parquet outputs, and produce `2192`
  `totalwatsed3.parquet` rows with non-placeholder WAT fields.
- totalwatsed3 contract: documented from openWEPP and wepppy sources.
- W-D publication repairs: exact volume fields now emit `m^3`, depth aliases
  remain mm, MOFE `latqcc` is outlet-only, optional profile/interception WAT
  fields are published, and profile audit violations are zero.
- W-D closure gate: FAIL/HOLD. Configured and legacy-discovery audits both
  reported `closure_reconstructed_with_storage_total_mm=2950.498418`; at W-D
  closeout, package closure was blocked on independent daily PASS `runvol`
  lineage.
- T-A architecture pivot: executed. `totalwatsed3-cli-scope.md` establishes
  `openwepp-cli-totalwatsed3` as a hillslope-only openWEPP-native CLI, with
  PASS `runvol` as `Runoff`, WAT storage/flux operands, outlet-only MOFE
  `latqcc`, no channel terms, and no wepppyo3 dependency.
- T-B dedicated CLI implementation: executed. `openwepp-cli-totalwatsed3`
  reads hillslope interchange PASS/WAT inputs, optional soil/element inputs,
  publishes the native `totalwatsed3.parquet`, uses PASS `runvol` as the
  `Runoff` operand, leaves WAT `Q` diagnostic, and removes totalwatsed3
  aggregation ownership from the watershed CLI path.
- T-B real-run behavior: arboreal-dendrite emits `2192` rows and the wepppy
  audit reads the file without schema repair; profile violations are zero.
- T-B held-for-next gate: the current independent audit residual is
  `closure_reconstructed_with_storage_total_mm=57.409871`
  (`0.345805%` of precipitation), so closure is not claimed until T-C.
- T-B2 native runoff-delivery publication: executed, then reviewed defective.
  The first MOFE PASS `runvol` formula used `QOFE * publication area`, which
  over-scaled runoff and made the old PASS identity self-consistent but wrong.
- T-B2-REDO runoff-delivery correction: executed. MOFE PASS `runvol` now uses
  the published `Q * Area` volume dual. Corrected arboreal-dendrite output
  under `/tmp/openwepp_wshed01_tb2_redo_qarea` produced `36` native PASS files,
  `anchor_mismatches=0` for existing HBP/WAT outputs, and a corrected
  totalwatsed3 file with `2192` rows.
- T-B2-REDO conservation pre-gate: water-year annual `sum(runvol) <= sum(P *
  Area / 1000)` passes for all `252` hillslope-water-years
  (`violation_count=0`, `max_runvol_precip_ratio=0.9857497687436844`).
- T-B2-REDO held-for-next gate: superseded by T-B2-REDO2. The wepppy audit on
  `/tmp/openwepp_wshed01_tb2_redo_qarea` reported
  `closure_reconstructed_with_storage_total_mm=6948.564523` because REDO
  under-scaled runoff with `Q * outlet Area`.
- T-B2-REDO2 crossed-pairing correction: executed. MOFE PASS `runvol` now
  uses `QOFE * outlet Area / 1000`. Fresh arboreal-dendrite output under
  `/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z` produced `36`
  native PASS files, `anchor_mismatches=0` for existing HBP/WAT outputs, and a
  corrected totalwatsed3 file with `2192` rows.
- T-B2-REDO2 conservation pre-gate: PASS `runvol` matches WAT outlet
  `QOFE * Area / 1000` over `78912` rows with
  `max_abs_pass_minus_qofe_area_m3=0.0`; summed PASS `runvol` is
  `27691217.37511973 m3`.
- T-B2-REDO2 audit result: no package closure is claimed in this increment,
  but the blocker is cleared. The wepppy audit reports
  `closure_reconstructed_with_storage_total_mm=30.544142`; day 1 is
  `+30.9533178099056 mm`, and excluding day 1 the basic-storage residual is
  `-0.409175395336963 mm` over `2191` days with `0` days above `1 mm`.

Next required increment:

```text
Execute increment T-C of docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md end-to-end.
```
