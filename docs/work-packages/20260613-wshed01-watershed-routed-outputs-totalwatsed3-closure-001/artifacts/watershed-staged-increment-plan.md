# Watershed Staged Increment Plan — Dispatch Artifact

Status: active - T-B2-REDO2 ACCEPTED (runvol defect closed, Claude-verified
2026-06-14); T-C closure proven on the REDO2 output → T-C is now recording/
governance (retire WBVAL06, ROADMAP item 1, README log, chanwb handoff)
Author: Claude Code, 2026-06-13
Template: FDHP01 `d3-staged-increment-plan.md` / MOFE01
`mofe-staged-increment-plan.md` (proven; agent memory
`staged-increment-port-template`).

## Universal rules (every increment)

- **Conservation is acceptance**: the totalwatsed3 identity must close at the
  established floor on the active architecture's output surface. For the T-arc
  that surface is hillslope-only dedicated CLI output, not channel-routed
  watershed output. Identity checks use **independent operands** — no 0==0 /
  self-built / alias closure
  (the M-E4-REDO/M-I lesson, hard-won this series). A residual that is exactly
  0.0 is the tautology smell; genuine closure is nonzero-at-noise.
- **Hillslope inputs are settled**: MOFE01 HBP shards are read-only inputs;
  no per-OFE physics re-opened. Single-OFE/MOFE anchors stay unchanged by any
  watershed-side increment.
- **Comparator posture**: legacy is a flag (ADR-0017). Acceptance is
  conservation, not legacy-match.
- Contract-first per increment; red tests before production edits; commit each
  increment `executed-hold` until acceptance; truthful evidence labels;
  `Static:`/`Ran:`.
- Subagent requirement per package.md §4a (comparator_suite_runner REQUIRED
  for heavy runs).
- Cross-repo boundary: no wepppy production edits without explicit scope; name
  cross-repo needs as follow-ons.
- Read the lines, don't infer from symbol tables (the Dh lesson).

## Increment W-A — characterization + watershed routing scope (no production edits)

Ground everything in measured reality before any code (the M-A lesson):

1. **Watershed CLI current behavior**: run `openwepp-cli-watershed` on
   arboreal-dendrite with the closed MOFE01 hillslope HBP shards. Record where
   it fails/succeeds, what it routes (channel network from `chan.inp`), what
   watershed output it produces (or doesn't), and the full failure chain.
2. **The `jpond=0` impoundment finding**: read
   `watershed_impoundment.rs` — does the parser reject `jpond=0` (no
   impoundments) as `IMP-E-004`, and is that a **parser defect** (zero ponds
   is valid; should yield an empty impoundment set) or genuinely invalid
   input? Confirm against legacy impoundment-file handling. Cite lines.
   Record in `impoundment-no-pond-finding.md`.
3. **Routing + output seams**: map the channel-routing path
   (`openwepp-watershed-orchestrator`), the watershed-output schema
   (`openwepp-watershed-output`, the interchange contract), and what
   **totalwatsed3**
   (`/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py`) expects as
   input — the exact columns/schema the audit consumes. Read the lines.
4. **Scope artifact** (`watershed-routing-scope.md`): legacy watershed-routing
   authority map (channel routing, pass-file consumption, watershed WB
   aggregation), openWEPP seam mapping, the totalwatsed3 input contract, the
   conservation identity (watershed-total = Σ hillslope contributions routed,
   closing against external inputs), red-test definitions, and the
   implementation increment breakdown + sizing.

- Gates: no production edits; evidence + scope artifact with current-tree
  file:line citations; the `jpond=0` finding classified (defect vs invalid)
  with evidence; the totalwatsed3 input contract documented.

W-A execution result (2026-06-13):

- Ran current watershed CLI on arboreal-dendrite with closed M-I HBP shards.
  It failed before `chan.inp`, HBP parsing, dispatch, or output writing:
  `CLIWAT-E-010` wrapping `IMP-E-004` on `pw0.imp` line 2, `jpond=0`;
  output file count was `0`.
- Classified `jpond=0` as a parser defect on valid no-impoundment input.
- Mapped the routing, output writer, and totalwatsed3 audit seams.
- W-B is the next implementation increment.

W-B execution result (2026-06-14):

- Amended the watershed runfile contract to pin explicit no-impoundment
  semantics: `inputs.pw0_imp` remains required in schema v1, but supported
  `.imp` files with `jpond=0` are valid typed empty impoundment sets when
  `pw0_str` declares zero impoundments.
- Added red/green parser and watershed CLI tests for explicit `jpond=0`.
- Implemented count reconciliation before the bare `jpond >= 1` guard, so
  `expected_structural_count=Some(0)` admits an empty item set, positive
  structural counts fail as `IMP-E-007`, and bare parses still fail as
  `IMP-E-004`.
- Re-ran the arboreal-dendrite watershed CLI. It now proceeds past
  `CLIWAT-E-010` and reaches the next hard stop:
  `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`, with zero output files.
- W-C is the next implementation increment.

W-C execution result (2026-06-14):

- Classified `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003` as valid routing
  input rejected by over-strict guards: a complete zero-sediment hillslope HBP
  payload with zero particle fractions, plus a hidden `nchnum=0`
  output-disabled channel state.
- Amended `SC-ROUTE-001` to pin zero-sediment contributor semantics and
  `nchnum=0` output-disabled semantics.
- Implemented the WS10 guard corrections and WAT-backed multi-row watershed
  publication.
- Re-ran arboreal-dendrite configured and legacy-discovery CLI paths; both
  exited `0`, emitted all `14` watershed parquet outputs, and produced
  `2192` `totalwatsed3.parquet` rows with non-placeholder WAT fields.
- W-D is the next implementation increment for totalwatsed3 closure.

W-D execution result (2026-06-14):

- Ran the wepppy totalwatsed3 audit against fresh configured and
  legacy-discovery arboreal-dendrite watershed outputs.
- Corrected keepable openWEPP publication defects: exact totalwatsed3
  hydrology columns now emit `m^3` volumes while depth aliases remain mm;
  MOFE `latqcc` aggregates only outlet OFEs; optional WAT profile and
  interception fields now publish into `totalwatsed3`.
- The producer no longer trips profile-cap false violations
  (`profile_violations_days=...:0`) and publishes total interception
  `551.502748 mm`.
- W-D remains held because the independent closure gate still fails:
  `closure_reconstructed_with_storage_total_mm=2950.498418`,
  `17.772166%` of precipitation. The remaining localized blocker is missing
  independent daily PASS `runvol` lineage; the current producer still fills
  `runvol` from WAT `Q`, which makes runoff consistency a self-consistency
  check rather than conservation proof.
- At W-D closeout, W-D-REDO was the next implementation increment. The
  operator-directed T-arc now supersedes that watershed-CLI route.

## Subsequent increments (refined by W-A; provisional)

- **W-B — impoundment no-pond handling**: executed-hold. Gate met:
  arboreal-dendrite watershed CLI proceeds past the impoundment parse, and
  existing impoundment fixtures are non-regressed.
**W-B execution result (2026-06-14, `ea95d372`) — accepted (Claude):** `jpond=0` accepted only for `expected_structural_count==Some(0)`; the structure-vs-file mismatch is preserved as a distinct typed error `IMP-E-007` (CountMismatch), `IMP-E-004` retained without structure context — guard preserved, not loosened. The absent-`.imp` case got an explicit decision (pw0_imp still required in schema v1, pinned in the runfile contract) — stricter than legacy (which never reads it) but deliberate and pinned. Red-first. The arboreal-dendrite CLI now clears `CLIWAT-E-010`.

**W-C now starts at the next hard stop:** `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003` (a channel-node *DomainViolation* in the WS10 watershed kernel; `kernel/types.rs:154`), 0 output files. W-C's first task is to characterize this exactly as W-A/jpond did: is it (a) valid channel input rejected (guard too strict — confirm legacy runs the same channel state), (b) a genuine domain violation (real bad routed channel state), or (c) missing routing data the channel kernel needs from the hillslope shards? Read the lines + compare to legacy `chnrt`/channel routing; do not assume defect.

- **W-C — watershed routing + output**: executed-hold. Gate met for routed
  publication: the CLI routes the hillslope shards over the channel network,
  emits all `14` interchange parquet outputs, publishes multi-row WAT-backed
  `totalwatsed3.parquet`, and rejects placeholder/default-zero publication.
- **W-D — totalwatsed3 end-to-end audit + closure**: executed-hold. Gate not
  met: totalwatsed3 profile/interception/unit defects were corrected, but the
  independent closure audit still reports `2950.498418 mm` whole-run residual.
- **W-D-REDO — PASS runvol lineage + closure**: superseded by the T-arc. It
  would have exposed or reconstructed
  canonical daily PASS runoff volume from HBP/PASS publication authority,
  bind it into `totalwatsed3.runvol`/`Runoff`, and rerun the configured and
  legacy-discovery audits. Acceptance remains nonzero-at-noise independent
  closure; exact WAT `runvol == Q * Area / 1000` self-consistency is not
  sufficient.

(W-A sizing may split W-C/W-D further; each increment behind a conservation
hard stop.)

## ARCHITECTURE PIVOT (operator-directed 2026-06-14) — totalwatsed3 = own openWEPP-native CLI

totalwatsed3 is hillslope-only (see package.md architecture decision). It moves
OUT of `openwepp-cli-watershed` into a dedicated **`openwepp-cli-totalwatsed3`**,
openWEPP-native (NOT sharing wepppyo3 `wepp_interchange`, which stays
wepp-legacy-only). The W-A/B/C watershed-CLI channel fixes stay (valid landed
work for the decoupled `chanwb` follow-on); W-C's in-watershed-CLI totalwatsed3
build and W-D's via-watershed-CLI closure are **superseded** by the T-arc.
The watershed-routed-output (`chanwb`/`chnwb`) is a decoupled follow-on
(`WATERSHED-CHANWB-ROUTED-OUTPUT`), not on the totalwatsed3 path.

### Increment T-A — totalwatsed3 CLI design + scope (no production code)

Design `openwepp-cli-totalwatsed3` before code (the M-A/W-A lesson). Read the
authoritative-semantics reference
`/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py` +
`tools/totalwatsed3_daily_closure_audit.py` (the CLOSURE SEMANTICS to match —
NOT a code dependency) and produce `totalwatsed3-cli-scope.md`:
- Inputs: per-hillslope `H.pass`/`H.wat`/`H.soil`/`H.element` parquets +
  area lookup; how openWEPP's MOFE01 hillslope outputs map to them.
- Aggregation semantics (hillslope-only, area-weighted): **Runoff from PASS
  `runvol`** (the independent operand); MOFE per-OFE collapse — **latqcc
  outlet-OFE-only**, QOFE summed, storage area-weighted; ET/Dp/Interception
  terms.
- openWEPP-native output schema (openWEPP-controlled, not legacy-bound; carry
  the W-D keepable unit/field fixes: m³ exact fields, depth aliases mm).
- The closure identity with **independent operands** (no 0==0 — PASS runoff is
  independent of WAT storage/flux) and the noise-floor tolerance.
- Red-test definitions + T-B/T-C breakdown + sizing.
- Remove/relocate `build_watershed_daily_rows_from_wat` from the watershed CLI.

T-A execution result (2026-06-14):

- Produced `totalwatsed3-cli-scope.md` as the controlling T-arc scope
  artifact.
- Confirmed the authoritative semantics from
  `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py` and
  `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py`: the
  producer is hillslope-only, `Runoff` comes from PASS `runvol`, WAT supplies
  storage/flux operands, MOFE `latqcc` is outlet-OFE-only, and channel terms
  are not part of totalwatsed3.
- Sampled the arboreal-dendrite interchange schemas under
  `/wc1/runs/ar/arboreal-dendrite/wepp/output/interchange/`: combined
  `H.pass.parquet`, `H.wat.parquet`, `H.soil.parquet`, and
  `H.element.parquet` use `wepp_id`/`ofe_id` selectors.
- Confirmed the openWEPP implementation gap: current hillslope output requires
  `.hbp` pass files, the HBP writer emits zero event volume slots, and the HBP
  parser does not expose PASS `runvol`. T-B must create the native PASS
  parquet/adapter surface before totalwatsed3 closure can be claimed.
- No production code was edited. T-B is the next implementation increment.

### Increment T-B — totalwatsed3 CLI implementation

Build the `openwepp-cli-totalwatsed3` producer to the T-A scope: contract-first,
red tests first, hillslope-only area-weighted aggregation, PASS-runvol Runoff,
MOFE per-OFE collapse, openWEPP-native schema. Gate: produces totalwatsed3
parquet on the MOFE01 arboreal-dendrite hillslope outputs; per-OFE collapse
correct (no cross-OFE double-count); single-OFE/MOFE hillslope outputs
unchanged (read-only consumer).

T-B execution result (2026-06-14):

- Added the dedicated `openwepp-cli-totalwatsed3` binary and native
  aggregation module. The CLI consumes `H.pass.parquet` and `H.wat.parquet`
  plus optional `H.soil.parquet`/`H.element.parquet`, and writes
  `totalwatsed3.parquet` without routing through `openwepp-cli-watershed`.
- Added red/green coverage for the missing binary, typed missing-input
  failures, PASS `runvol` vs WAT `Q` independence, and MOFE outlet-only
  `latqcc` collapse.
- Updated the output writer and unit registry so exact hydrology fields remain
  volume fields where declared, `Runoff` is a publication-only PASS-volume
  depth, and WAT `Q` remains diagnostic.
- Removed totalwatsed3 aggregation ownership from the watershed CLI path; the
  dedicated CLI now owns WAT/PASS aggregation for this output.
- Ran the producer on arboreal-dendrite interchange inputs. It wrote `2192`
  rows to `/tmp/openwepp_wshed01_tb/totalwatsed3.parquet`; the wepppy audit
  reads the output without schema repair and reports zero profile violations.
- T-B is executed, but package closure is still queued for T-C: the current
  independent audit residual is
  `closure_reconstructed_with_storage_total_mm=57.409871`
  (`0.345805%` of precipitation).

### Increment T-B2 — openWEPP-native runoff-delivery (runvol) output

T-B's closure ran on LEGACY interchange parquets (Jun-7 wepppy-produced);
openWEPP emits no `runvol`/PASS surface (Claude T-B review). For genuine
ADR-0019 closure, openWEPP must produce its OWN independent runoff-delivery
`runvol`. Operator-directed (2026-06-14): source it from the **MOFE outlet-OFE
routed runoff**, then run totalwatsed3 on openWEPP-native outputs.

- **Source (precise, read-the-lines):** the outlet-OFE
  `current_transfer_output` surface runoff —
  `per_ofe_internal_wb13.rs:161 physical_surface_outflow_mm`
  (`= current_transfer_output.qofe * 1000`), i.e. the **same outlet runoff the
  M-I hillslope-total identity already uses** (`:548` `if index ==
  outlet_index: external_out += physical_q_mm`, which closed at `3.31e-13`).
  This is the routed-transfer-delivery path, **genuinely independent of the
  WAT `Q` balance publication** — not a WAT-Q restatement (which would be the
  self-consistency the T-arc exists to avoid).
- **runvol = outlet exported runoff volume** (m³), per day. ⚠ CORRECTED
  (2026-06-14, [review-tb2-runvol-area-defect.md](review-tb2-runvol-area-defect.md)):
  `qofe`/`physical_surface_outflow_mm` is **slplen-normalized** (referenced to
  the outlet OFE), so the exported volume is `qofe · A_outlet_OFE`, **NOT**
  `qofe · A_hillslope`. The original wording here said *"× hillslope area"* —
  that is the dimensional mistake T-B2 implemented (runoff came out 2–3× precip).
  Use the in-tree dual: `qofe · outlet.area_m2` (matches the M-I outlet
  weighting and the adjacent `sbrunv` line) **or** published `q · hillslope area`
  (matches the non-MOFE path). It is the **outlet net delivery**, NOT the
  area-weighted per-OFE sum (the W-D double-count that gave 2950 mm). `sbrunv`
  from the outlet lateral delivery; sediment companions zero per the
  deferred-sediment posture (MOFE-EROSION follow-on).
- **Emit openWEPP's own runoff-delivery parquet** (ADR-0019: openWEPP-controlled
  schema) carrying the columns totalwatsed3 needs (`runvol`/`sbrunv`/date keys);
  the T-B CLI reads openWEPP's surface, not the legacy `output/interchange/`.
- Output-surface addition only: **no hillslope/MOFE physics change**; single-OFE
  + MOFE WAT/HBP anchors stay byte-identical.
- Red tests: runvol equals the outlet `physical_surface_outflow` volume (NOT
  the per-OFE sum, NOT WAT `Q`); a multi-OFE fixture proves runvol = outlet,
  not Σ-per-OFE; missing/zero-runoff days produce zero runvol.
- Gate: openWEPP emits its own runoff-delivery parquet from the
  arboreal-dendrite MOFE01 run; anchors unchanged; full Rust loop.

T-B2 execution result (2026-06-14):

- Added optional hillslope `outputs.pass_parquet` publication. The new
  openWEPP-native PASS parquet carries date keys, `wepp_id`, `runvol`,
  `sbrunv`, peak-runoff and zeroed deferred-sediment companion columns.
- Built MOFE `runvol` from the terminal outlet record:
  `physical_surface_outflow_mm * publication_area_m2 / 1000`, so it is outlet
  delivery over the hillslope publication area, not a per-OFE sum and not WAT
  `Q`.
- Extended `openwepp-cli-totalwatsed3` to consume sorted per-hillslope
  `H*.pass.parquet` and `H*.wat.parquet` files when combined `H.pass.parquet`
  / `H.wat.parquet` files are absent. Per-file WAT ingestion overrides local
  row `wepp_id` from the `H<number>.wat.parquet` file name.
- Real arboreal-dendrite MOFE01 rerun emitted `36` HBP, `36` WAT, and `36`
  native PASS parquet files under `/tmp/openwepp_wshed01_tb2/output`.
- HBP/WAT anchor hash comparison against `/tmp/openwepp_mofe01_mi_final/output`
  reported `anchor_mismatches=0`.
- Direct PASS identity audit over all `78912` hillslope-day rows reported
  `max_abs_runvol_diff_m3=1.4551915228366852e-11` for PASS `runvol` vs outlet
  WAT `QOFE * hillslope area / 1000`.
- `openwepp-cli-totalwatsed3` consumed the native per-hillslope files and
  wrote `2192` rows to `/tmp/openwepp_wshed01_tb2/totalwatsed3.parquet`.
- Full Rust loop passed: fmt, clippy, workspace tests, and deny. T-C remains
  the next closure increment.

> ⚠ **DEFECT (Claude review 2026-06-14,
> [review-tb2-runvol-area-defect.md](review-tb2-runvol-area-defect.md)):** the
> `runvol = physical_surface_outflow_mm * publication_area_m2` formula uses the
> **whole-hillslope** area against a **slplen-normalized** outlet depth, so
> `runvol` came out **2.0–3.1× precip every year** (physically impossible;
> closure −32,855 mm cumulative). The `1.46e-11` "PASS identity" above is a
> **self-consistency** check — it compares `runvol` to the same wrong formula,
> so it could not catch this. **Hold T-C** until the reference area is
> corrected (`qofe · outlet.area_m2` or published `q · hillslope area`) and an
> **independent** bound (`Σ runvol ≤ Σ precip`) replaces the self-consistency
> check.

### Increment T-B2-REDO — correct the runvol reference area

Disposition of [review-tb2-runvol-area-defect.md](review-tb2-runvol-area-defect.md).

- Correct `build_hillslope_pass_row_from_outlet_delivery`
  (`02_output_and_climate_helpers.rs:728`): the exported runoff volume is
  `qofe · A_outlet_OFE` (≡ published `q · A_hillslope`), **not**
  `qofe · A_hillslope`. Pick the in-tree dual that reads cleanest next to the
  adjacent `sbrunv` line (`:729`, already `outlet.area_m2`) and the non-MOFE
  path (`:713`, `q · area_m2`) — disposition is the implementer's.
- Replace the self-consistency PASS check with an **independent** bound: red
  test `Σ runvol ≤ Σ precip` (annual, per hillslope) + a multi-OFE fixture
  asserting `runvol = qofe·A_outlet ≠ qofe·A_hillslope`. The old
  `runvol == qofe·A_hillslope` audit must be deleted, not kept (it encodes the
  bug).
- Single-OFE anchors stay byte-identical (their outlet OFE *is* the hillslope,
  area unchanged); MOFE PASS `runvol` changes by design — re-baseline the MOFE
  PASS expectation, not the HBP/WAT anchors.
- Gate: arboreal-dendrite rerun shows annual `Σ runvol ≤ Σ precip` for every
  hillslope; full Rust loop; anchors unchanged.

T-B2-REDO execution result (2026-06-14):

- Replaced MOFE PASS `runvol` publication with the published volume dual:
  `outlet.row.wb13_row.q * outlet.row.wb13_row.area / 1000`. This deletes the
  defective `QOFE * publication area` identity surface instead of keeping it as
  validation.
- Added a focused regression fixture where `Q=2.5 mm`, `QOFE=5.0 mm`, and
  `Area=200 m2`; correct `runvol` is `0.5 m3`, while the two wrong formulas
  produce `1.0 m3` and `0.25 m3`.
- Fresh arboreal-dendrite rerun under
  `/tmp/openwepp_wshed01_tb2_redo_qarea` emitted `36` HBP, `36` WAT, `36` PASS
  parquet files, and `36` manifests.
- HBP/WAT anchor comparison against `/tmp/openwepp_mofe01_mi_final/output`
  reported `anchor_mismatches=0`.
- Corrected PASS audit over `78912` rows reported
  `max_abs_pass_minus_q_area_m3=0.0`; the old buggy formula now differs by up
  to `21766.4323911278 m3`, and total PASS `runvol` dropped from
  `126757678.32012111 m3` to `6851275.733726179 m3`.
- Water-year annual bound passed for every hillslope-water-year:
  `252` annual hillslope-water-years, `violation_count=0`,
  `max_runvol_precip_ratio=0.9857497687436844`.
- Native totalwatsed3 production from corrected PASS/WAT wrote `2192` rows to
  `/tmp/openwepp_wshed01_tb2_redo_qarea/totalwatsed3.parquet`; totalwatsed3
  `runvol` differs from summed PASS by `9.313225746154785e-10 m3`.
- wepppy audit read succeeded but reports
  `closure_reconstructed_with_storage_total_mm=6948.564523`. No T-C closure is
  claimed from T-B2-REDO.
- Full Rust loop passed: fmt, clippy, workspace tests, and deny.

> ⚠ **STILL DEFECTIVE (Claude review 2026-06-14, review-tb2-runvol-area-defect.md
> §Follow-up):** the closure got *worse* (6948 vs W-D's 2950) because
> `q · outlet.area` is the **crossed pairing** — totlen-normalized `Q` against
> the **outlet OFE** area — under-scaling runoff by `totlen/slplen` (~4× cohort;
> exactly `5×` on the 5-OFE H1, where `QOFE/Q=5.0`). The Rust loop went green
> because (a) `Σ runvol ≤ Σ precip` is one-sided (under-scaled 0.137 passes) and
> (b) the new fixture **encodes the wrong formula** (asserts `Q·A_outlet`
> correct). Reconstructing the export from WAT `QOFE_outlet · A_outlet`
> (independent operands) closes the totalwatsed3 at **+30.5 mm = day-1 init,
> ex-day-1 −0.41 mm/2191 days**. Fix = `q → qofe` (`:737`). See **T-B2-REDO2**.

### Increment T-B2-REDO2 — fix the crossed pairing (Q→QOFE at the outlet area)

Disposition of the follow-up review (review-tb2-runvol-area-defect.md, REDO
section). T-B2-REDO did NOT clear the defect — it swapped over-scaling for
**under-scaling** (`runvol` ~4× too small). `:737` pairs `wb13_row.q` (totlen-
normalized) with `wb13_row.area` (the **outlet OFE** area) — a crossed pairing.
`Q`(totlen) belongs with the **hillslope** area; `QOFE`(slplen) with the
**outlet** area.

- Fix (empirically verified to close): `:737` use the slplen-normalized depth
  with the outlet area — `outlet.row.wb13_row.qofe * outlet.row.wb13_row.area`
  (≡ the M-I export `physical_surface_outflow_mm · outlet.area_m2`, which closed
  the per-hillslope identity at 3.31e-13). Disposition of which QOFE surface is
  the implementer's.
- **Delete/invert the REDO fixture** (`Q=2.5,QOFE=5.0,Area=200 → "correct"
  0.5 m³`): it encodes the bug. With `Area` = outlet OFE area, the correct
  `runvol` is `QOFE·Area = 1.0 m³`; `Q·Area = 0.5 m³` is the under-scaled wrong
  value. The fixture must assert `QOFE·A_outlet`.
- **Acceptance is the CLOSURE, not a precip ratio.** `Σ runvol ≤ Σ precip` is
  one-sided — REDO's under-scaled runvol passed it (0.137 < 1). The gate is the
  totalwatsed3 identity dropping to floor (verified: `+30.5 mm`, entirely day-1
  storage-init; **ex-day-1 −0.41 mm over 2191 days**) on the arboreal-dendrite
  cohort with the corrected runvol. A per-hillslope `≤ precip` check, if kept,
  must use the **totalwatsed3-aggregated** precip basis, not a per-OFE WAT-`P`
  sum (which under-counts ~6×).
- Anchors unchanged; full Rust loop.

T-B2-REDO2 execution result (2026-06-14):

- Replaced the crossed REDO formula with the slplen-normalized outlet pairing:
  `outlet.row.wb13_row.qofe * outlet.row.wb13_row.area / 1000`. This makes
  native PASS `runvol` equal `QOFE_outlet · A_outlet`, not the under-scaled
  `Q_outlet · A_outlet`.
- Deleted/inverted the REDO fixture. The focused regression now asserts the
  two-OFE fixture's correct `runvol` is `1.0 m3` and rejects the REDO
  `Q · A_outlet = 0.5 m3` pairing.
- Fresh arboreal-dendrite rerun under
  `/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z` emitted `36`
  HBP, `36` WAT, `36` PASS parquet files, and `36` manifests.
- HBP/WAT anchor comparison against `/tmp/openwepp_mofe01_mi_final/output`
  reported `anchor_mismatches=0`.
- Corrected PASS audit over `78912` rows reported
  `max_abs_pass_minus_qofe_area_m3=0.0`,
  `sum_runvol=27691217.37511973 m3`; the rejected REDO `Q · A_outlet`
  under-scaled sum is `6851275.733726182 m3`.
- Native totalwatsed3 production wrote `2192` rows to
  `/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z/totalwatsed3.parquet`;
  totalwatsed3/PASS `runvol` sum diff was `-4.0978193283081055e-08 m3`.
- wepppy audit read succeeded and reported
  `closure_reconstructed_with_storage_total_mm=30.544142` and enriched
  storage `30.543864`. The day-1 residual is `+30.9533178099056 mm`; excluding
  day 1 the basic-storage residual is `-0.409175395336963 mm` over `2191`
  days with `0` days above `1 mm`.
- Full Rust loop passed: fmt, clippy, workspace tests, and deny. T-C is now
  queued on the REDO2-corrected output; the REDO
  `/tmp/openwepp_wshed01_tb2_redo_qarea` output is superseded.

> ✅ **ACCEPTED (Claude independent verification 2026-06-14, Ran on the REDO2
> root).** `Σ runvol = 27.691 Mm³` (coeff `0.5537`); runoff < precip every year
> (0.48–0.68) — the **two-sided** bound holds. **Genuinely independent:**
> `Σ runvol (27.691) ≠ Σ Q-column (18.895)` — the QOFE×outlet-area surface, not
> a WAT-`Q` restatement. **Closure is real, not 0==0:** ex-day-1 daily residuals
> span `[−0.248, +0.005] mm` (nonzero-at-noise); cumulative ex-day-1 `−0.41 mm`
> over 2191 days; the `+30.95 mm` day-1 term is the audit's storage-prepend init
> (present for any correct producer, legacy included), not an openWEPP defect.
> My WAT-`QOFE_outlet·A_outlet` reconstruction predicted `30.544142 / −0.409175`
> before the run — the producer reproduces it to all digits. The runvol defect
> (over-scale → under-scale → correct, across T-B2/REDO/REDO2) is **closed**.
> The slight negative skew (`−0.41 mm/2191 d`) is the known small over-drainage
> residual at the cohort noise floor — not actionable at this rung.

### Increment T-C — totalwatsed3 closure on openWEPP-NATIVE outputs (the WBVAL06/6a deferral resolved)

**READY after T-B2-REDO2.** The previous
`/tmp/openwepp_wshed01_tb2_redo_qarea` output is superseded because it was
under-scaled (`Σ runvol = 0.137·Σ precip`; closure `+6948 mm`). T-C runs on
REDO2-corrected output:
`/tmp/openwepp_wshed01_tb2_redo2_qofearea_20260614T213618Z/totalwatsed3.parquet`.

The closure audit on the openWEPP-native totalwatsed3 output — produced from openWEPP's OWN H.pass(runvol) + H.wat, NOT the legacy interchange dir. Gate: the
identity `P − (Runoff + Lateral + ET + Perc + Interception) − ΔStorage` closes
at the established floor with **independent operands** (PASS runoff, not WAT Q;
nonzero-at-noise, not 0==0) on the arboreal-dendrite cohort. Expected on the
REDO2 fix: cumulative `~+30 mm` carried entirely by day-1 storage-init,
**ex-day-1 sub-mm over 2191 days** (Claude reconstruction from WAT
`QOFE_outlet · A_outlet`). On pass: the WBVAL06/6a end-to-end totalwatsed3
deferral is resolved; ROADMAP item 1 removed; README execution log; handoff
naming the decoupled `chanwb` follow-on. The earlier `+2,950 mm` "residual
caveat" was an **artifact of the wrong (WAT-`Q`) runvol**, now retired — the
corrected runvol closes; do not chase a phantom second term.

## Dispatch instructions

Each Codex dispatch: *"Execute increment <W-A|...|T-A|T-B|T-B2|T-B2-REDO|T-B2-REDO2|T-C> of
`docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md`
end-to-end."* Required reading order: this plan; `package.md`;
`watershed-routing-scope.md`; `totalwatsed3-cli-scope.md` after T-A; the
MOFE01 + FDHP01 staged plans (the failure modes these rules encode —
tautology, clone, hollow closure). An increment that cannot meet its gates
backs out with evidence, localized to its seam.
