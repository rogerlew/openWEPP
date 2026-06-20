# Work Packages

> **Canonical roadmap: [../ROADMAP.md](../ROADMAP.md)** — a **forward-only planning
> queue** (what is next and deferred). The section below is this roadmap's
> **execution log**: the home for **completed** work — package status, detail, and
> commits. When a queue item closes it is removed from `ROADMAP.md` and recorded
> here. If the two disagree on what is next, `ROADMAP.md` wins.

## Current roadmap execution log

State as of `2026-06-20`:

- R3C is complete with verdict
  `COMPLETE-R3C-DIRECT-MULTILANE-TRANSFER-SPAN`. The package implemented a
  run-level direct-runtime span,
  `LateralTransfer -> RunoffReconciliation -> ClosureDiagnostics`, that consumes
  direct lane topology, upstream-area ratios, lane areas, and direct transfer
  buffers; computes a diagnostic per-lane transfer ledger; mutates direct
  run-level state; produces downstream operands; and shadow-projects run-level
  transfer totals. R3C added reciprocal topology validation after review and
  remains diagnostic-only: it does not migrate hydrology-process equations, cut
  over publication, activate direct mode by default, or claim endpoint
  improvement. Full Rust gates passed. Final default-disabled H2637 reps were
  `640.85 s`, `643.41 s`, and `644.07 s` (median `643.41 s`, threshold
  `<= 676.67 s`) with protected output identity. Package:
  `20260620-r3c-direct-multilane-transfer-span-001/`.
- R3B is complete with verdict `COMPLETE-R3B-DIRECT-WATER-LEDGER-SPAN`. The
  package implemented a second direct-runtime span,
  `RunoffReconciliation -> StorageReconciliation -> ClosureDiagnostics`, that
  consumes R3A input-accounting state plus direct water and publication fields,
  computes a signed diagnostic ledger residual, mutates direct ledger state,
  produces downstream ledger operands, and shadow-projects the result. The
  residual is diagnostic-only; R3B does not migrate hydrology-process equations,
  cut over publication, claim endpoint improvement, or activate direct mode by
  default. Full Rust gates passed. Final default-disabled H2637 reps were
  `640.67 s`, `643.05 s`, and `639.21 s` (median `640.67 s`, threshold
  `<= 676.67 s`) with protected output identity. Package:
  `20260620-r3b-direct-water-ledger-span-001/`.
- R3A is complete with verdict `COMPLETE-R3A-PHASE-SPAN`. The package
  implemented direct transfer-input accounting as the first complete direct
  phase span on top of the R2A skeleton:
  `DirectPhaseKind::Normalization -> DirectPhaseKind::LateralTransfer`.
  The span includes typed inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection. Phase-span identity passed with
  exact binary-fraction fixture evidence; no-compatibility proof passed by
  forbidden-token source scan, scheduler no-diff, and runtime counters; the
  explicit opt-in path records one production compatibility-edge handoff while
  direct span execution records zero edge invocations. Full Rust gates passed.
  Final default-disabled H2637 reps were `630.31 s`, `640.85 s`, and
  `632.08 s` (median `632.08 s`, threshold `<= 676.67 s`) with protected
  output identity.
  R3A did not cut over publication, activate direct mode by default, or claim
  R4/R6/endpoint readiness. Package:
  `20260620-r3a-first-direct-phase-span-001/`.
- R2A is complete with verdict `COMPLETE-R2A-SKELETON`. The package introduced
  a distinct direct-runtime namespace, typed direct-frame shells, a no-op/shadow
  direct executor skeleton, explicit one-time runner setup selection, default
  inactivity proof, and executable no-compatibility proof hooks. Review removed
  misleading reserved forbidden-call counters; forbidden-call absence is proven
  by direct-runtime source/call-graph evidence, while runtime counters prove
  default-disabled direct-skeleton inactivity and explicit opt-in skeleton
  execution. Final default-disabled H2637 reps were `634.06 s`, `636.01 s`,
  and `640.93 s` (median `636.01 s`, threshold `<= 676.67 s`), with protected
  output identity. No phase math, publication cutover, endpoint-improvement
  claim, or default activation occurred. Follow-on: R3A first complete direct
  phase span. Package: `20260619-r2a-direct-runtime-skeleton-001/`.
- PERFDEEP09 executed with verdict `READY-FOR-R2`. Same-machine no-edit control
  reproduced the default-disabled blocker at `682.65 s`, RSS `228924 KB`.
  The retained remediation collapses repeated per-root perennial decomposition
  indexed-overflow scans into one slot/crop pass while preserving typed guard
  behavior. Final H2637 default-disabled reps were `634.61 s`, `635.65 s`,
  and `636.58 s` (median `635.65 s`, RSS `228856/228280/228168 KB`), clearing
  the `<= 676.67 s` P0 gate. HBP, loss, WAT, and plot checksums were stable;
  PASS parquet passed the established Arrow/DuckDB row-equivalence identity
  lane. R2+ direct-frame runtime implementation is unblocked for the next
  package, but remains unimplemented in PERFDEEP09. Package:
  `20260619-perfdeep09-disabled-path-iterative-defect-closure-001/`.
- PERFDEEP08 executed with verdict `HOLD`. The package tested one scoped
  disabled-path hard-isolation candidate: caching the PERFDEEP02 roundtrip env
  lookup and short-circuiting inactive indexed-shadow hooks. The candidate
  preserved protected output checksums but measured `691.93 s`, RSS
  `229444 KB`, slower than PERFDEEP07's retained `685.85 s` and above the P0
  `<= 676.67 s` gate. The candidate was reverted; no production Rust edit was
  retained. R2+ direct-frame runtime implementation remains blocked. Package:
  `20260619-perfdeep08-disabled-path-hard-isolation-001/`.
- R0/R1 array-native schema and frame planning is complete with verdict
  `COMPLETE-PLANNING-ONLY`. The package recorded the direct runtime schema
  envelope, direct-frame type-boundary decision, R1 constructor/projection
  plan, publication-ledger promotion plan, no-compatibility proof plan, and
  PERFDEEP07 hold-lift conditions. It made no Rust, test, output schema, or
  contract edits and does not authorize R2+ runtime implementation. Package:
  `20260619-r0-r1-array-native-schema-frame-planning-001/`.
- PERFDEEP07 executed with verdict `HOLD`. The package partially reduced the
  default-disabled tax (`701.95 s` -> `685.85 s`) but did not pass the P0
  three-run median threshold `<= 676.67 s`, so direct-frame hydrology
  implementation was not started. PERFDEEP02/03/05 opt-ins remain fail-closed,
  and R2+ array-native runtime work remains blocked until the hold is closed or
  explicitly superseded. Package:
  `20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/`.
- PERFDEEP06 is executed with verdict `READY-FOR-PERFDEEP07`. The package
  produced the array-native fast-path frame inventory, publication operand
  ledger, direct-frame API plan, layout/allocation ledger, no-hot-loop-map
  proof, and follow-on package sequence. It also recorded the default-disabled
  regression as a P0 follow-on gate: PERFDEEP05 default-disabled H2637 measured
  `701.95 s` versus the `669.97 s` reference, and PERFDEEP03 default-disabled
  measured in the `697-708 s` band. PERFDEEP07 must make the opt-in plumbing
  zero-cost when disabled before adding more direct-frame machinery. No
  production activation or Rust implementation occurred in PERFDEEP06. Package:
  `20260619-perfdeep06-array-native-fast-path-inventory-001/`.
- PERFDEEP05 is complete with verdict
  `NO-GO - sync hotspot removed, endpoint still fails activation gate`. The
  package removed `sync_from_writeback_surface` from the PERFDEEP03 opt-in daily
  H2637 hot loop, applies MOFE transfer input directly to lane-owned dense state
  through cached transfer symbol ids, and added cached-slot daily refresh for
  prepared hot/static symbols. Final-code H2637 identity passed: HBP/WAT
  byte-identical, PASS Arrow-equivalent, and plot/loss differences limited to
  `run_name`. Final-code default-disabled H2637 measured `701.95 s`,
  `227712 KB`; final-code opt-in measured `911.11 s`, `229820 KB`, versus the
  PERFDEEP01 `669.97 s` activation reference. The PERFDEEP04 full-sync hotspot
  is gone from the profile, but remaining dense-edge costs dominate:
  `refresh_cached_slots_from_writeback_surface` (`16.20%` children,
  `9.07%` self), `apply_kernel_writeback_payload` (`10.47%` children),
  `SymbolRegistry::id_of` (`7.72%` children), and
  `flush_dirty_to_writeback_surface` (`6.72%` children). No default activation.
  Follow-on: PERFDEEP06 fast-path inventory/API planning, not another
  compatibility-edge optimization. Package:
  `20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/`.
- PERFDEEP04 is complete with verdict
  `PROFILED - cut PERFDEEP05 at lane-dense sync removal`. The package scaffolded
  and executed matched `perf record` profiles for the PERFDEEP03 opt-in H2637
  lane-dense no-go and the default-disabled H2637 path. Opt-in measured
  `1164.31 s`, `519160 KB`, `61248` samples under profiler; default measured
  `704.82 s`, `320640 KB`, `37051` samples. The top PERFDEEP03-specific hotspot
  is `HillslopeLaneDenseState::sync_from_writeback_surface` at `33.49%`
  inclusive / `14.19%` self, absent from default. Dense reads helped
  (`state_value_for_symbol` fell from `14.83%` inclusive default to `3.80%`
  opt-in), but daily logical/indexed-to-dense resync, hot-symbol vector rebuilds,
  symbol-id lookup, and boundary BTreeMap flush dominate. Follow-on:
  `PERFDEEP05 - Lane-Dense Transfer Authority and Sync Removal`. Package:
  `20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/`.
- PERFDEEP03 is complete with verdict
  `NO-GO - section 7 falsification / re-profile before expanding`. The package
  implemented the PERFDEEP02 ownership correction: lane-owned persistent compact
  dense state carried through `OfeLanePersistentState`, compact dense slot views
  on `HillslopeKernelRequest`, direct dense writeback application, dirty-slot
  boundary flush, and default-disabled runner activation behind
  `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1`. Correctness gates passed:
  HBP/WAT byte identity, PASS Arrow equivalence, 235961 diagnostic roundtrip
  rows with zero mismatches, full Rust gates, and `cargo deny`. The load-bearing
  opt-in H2637 endpoint failed: `1147.96 s`, `229580 KB` versus the PERFDEEP01
  `669.97 s` reference. Default-disabled identity passed, but default endpoint
  flatness was not proven (`697.36 s` / `707.80 s`), so there is no default
  activation. Follow-on work must re-profile the current no-go implementation
  before expanding the island or deleting more logical surfaces. Package:
  `20260619-perfdeep03-persistent-lane-owned-dense-state-001/`.
- PERFDEEP02 is complete with verdict `NO-GO - performance blocked`. The
  package implemented the Stage-1 dense-slot `HillslopeDayFrame` hydrology
  island mechanics, dense-first request reads, dirty-id frame writeback flush,
  and focused full-family frame roundtrip tests. Full Rust gates passed.
  Production opt-in H2637 endpoint attempts failed by more than 2x versus the
  PERFDEEP01 `669.97 s` reference, so the island is fail-closed behind
  `OPENWEPP_PERFDEEP02_FRAME_ISLAND=1`. Follow-on work must remove per-day/OFE
  frame lifecycle cost before default activation. Package:
  `20260619-perfdeep02-hydrology-island-core-001/`.
- PERFMIG02 is executed-redirect. The rung preserved identity while migrating hot
  scalar helpers to dense-first reads and retiring logical materialization for
  six internal WB11/WB12/WB14 symbols, but the final-code H2637 no-UI endpoint
  was flat/negative versus PERFMIG01 (`669.97s` -> `672.14s` / `675.00s`, RSS
  `228144 KB` -> `227636 KB` / `228152 KB`). The strict package attribution
  subgate also failed: artifact-local `apply_indexed` materialize-all measured
  `104.752336 us/payload`, while the conservative skip-six policy measured
  `105.460510 us/payload` because fail-closed stale-logical removal costs more
  than six avoided inserts. Verdict: REDIRECT; next perf work should pivot to a
  deep single-phase array-native read+compute+write migration rather than another
  writeback-only or tiny materialization-retirement rung.
  Package:
  `20260618-perfmig02-wb11-consumer-cluster-boundary-retirement-001/`.
- PERFMIG01 is complete with verdict `CONTINUE`. ADR-0023 was ratified and the
  production WB11 warm-rain runoff writeback branch now emits a dense
  `SymbolId`-backed payload: 543 state updates plus 8 flux updates, with the
  logical payload empty on the migrated success path. Focused tests proved exact
  materialized map equality and exact `f64::to_bits()` equality. The H2637
  no-UI endpoint rerun was semantically identical to PERFIDX06 outputs but
  measured `669.97s`, `228144 KB` versus PERFIDX06 `666.82s`, `228508 KB`
  (`+0.47%`). The transition apply boundary measured `107.531649 us/payload`
  (`25.373275s` projected over H2637 OFE-days), so the first-rung regression is
  a named retireable compatibility-boundary result. Next perf rung should
  migrate a contiguous WB11-consumer cluster. Package:
  `20260618-perfmig01-wb11-runoff-array-authoritative-production-migration-001/`.
- PERFARCH03 is complete with verdict `GO - branch floor clears <=5x and
  <=10x`. The artifact-local full array-native WB11 runoff branch prototype
  validated 543 state outputs plus 8 flux outputs against the current production
  kernel by exact numeric `to_bits()` equality. Median array combined hot-loop
  cost was `0.959423 us/OFE-day` (`0.024823x` legacy us/OFE-day; projected
  `0.226386 s` over H2637 OFE-days), while one-shot boundary materialization was
  measured separately at `108.068963 us/OFE-day`. Dense slot working set was
  `18,208 bytes` and release-binary RSS was `3,072 KiB`. Verdict authorizes a
  follow-on array-authoritative production migration package / ADR-0023 revival,
  starting with WB11 runoff; it does not claim full H2637 endpoint closure yet.
  Package: `20260618-perfarch03-full-array-native-floor-prototype-001/`.
- POST-BASECOND01-H2637-MAGNITUDE-DISPOSITION is complete. The package
  synthesized FARPOINT01, MAGPARITY01, STAGE2-LATQCC, REFINTENT001,
  STAGE2-BASE-CONDUCTIVITY, and BASECOND01 evidence, then resolved the H2637
  `71.0036550031206%` magnitude flag as `CORRECT-BY-CONSTRUCTION` / `NO DEFECT`
  for the internal openWEPP lateral lineage. The remaining absolute physical
  magnitude question is an external-authority `CONTRACT-GAP`, recorded as
  `docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md`;
  it is not a queue blocker and does not authorize a production edit. Package:
  `20260618-post-basecond01-h2637-magnitude-disposition-001/`.
- BASECOND01 is complete-with-correction. `SC-INFILE-SOIL-001` v0.1.11 now
  explicitly separates vertical `ssc` from hourly lateral `ui_ssh`: the top
  normalized 200 mm interval uses the baseline top source-layer `ksat` rule,
  lower split-source vertical `ssc` is inverse-conductivity/harmonic, and
  `wb19_lateral_ssh` remains arithmetic from `ksat*anisotropy`. Regression tests
  prove the surfaces are non-aliased. The H2637 no-UI rerun was aggregate-inert
  (`runvol_pct_precip` remained `71.0036550031206`), so BASECOND01 closes the
  vertical `ssc` defect but did not by itself close the remaining FARPOINT01
  magnitude flag; POST-BASECOND01 closed that flag by disposition after the full
  evidence chain was synthesized. Package:
  `20260618-basecond01-ssc-harmonic-normalization-defect-closure-001/`.
- STAGE2-BASE-CONDUCTIVITY-H2637-MAGNITUDE is complete with verdict
  `OPENWEPP-DEFECTIVE`. The package proved base `ksat` is byte-live on H2637
  (`ksat_x0.9` changed WAT/PASS checksums, aggregate `latqcc`, PASS `runvol`,
  and peak WAT `latqcc`). Source intent splits the surfaces: vertical
  `wb18_perc_ssc` is inverse-conductivity normalized, while modern hourly
  `wb19_lateral_ssh` is arithmetic `ssc2*ui_anisrt`. At the time of that
  package, openWEPP made vertical `ssc` arithmetic too, inflating H2637
  split-layer `ssc` from `117.955408163210` to `270.8259 mm/h`. FARPOINT01
  remained open and routed to BASECOND01 for vertical `ssc` 200 mm
  normalization while preserving hourly `ui_ssh`. Package:
  `20260618-stage2-base-conductivity-h2637-magnitude-001/`.
- REFINTENT001-KSATADJ-SATFRAC is complete. WB14 `ksatadj` now forms
  `sat_frac` from the ratified source-intent operands
  `avsat/(avpor*avcpm)` with the two `avsat` caps and top-two tillage weighting;
  the old `sum(theta)/sum(ul)` surrogate is removed. Focused WB14 tests, full
  workspace gates, H2637 both UI variants, and the OFE1-OFE5 ladder passed.
  H2637 `runvol` remained `71.003655003121%` of precipitation because
  `ksatadj = 0` on H2637, so REFINTENT001 did not close FARPOINT01. Package:
  `20260618-refintent001-ksatadj-satfrac-defect-closure-001/`.
- STAGE2-LATQCC-H2637-MAGNITUDE is complete with verdict `CONTRACT-GAP`.
  H2637 `latqcc` was traced through WB19 per-substep operands for selected
  high-magnitude days across all 19 OFEs; emitted WAT `latqcc` equals WB19 `q`,
  and recomputed Eq [6.2.4]/Dun-style potential matches to floating-point
  precision. No openWEPP equation, withdrawal, conductivity-override,
  active-depth, or `drfc` formula defect was found. The remaining FARPOINT01
  Stage-2 flag is an absolute lateral-flow magnitude authority gap, not a
  defect-closure handoff. Package:
  `20260618-stage2-latqcc-h2637-magnitude-001/`.
- REFACTOR022 is complete for behavior-preserving monolith line-count cleanup.
  The four target-tier WARN-band files closest to the 3000-line required-refactor
  threshold were split by domain responsibility:
  `routing.rs`, `scheduler_seed_and_runtime.rs`, `core_types.rs`, and
  `hydrology_phase_lateral_drainage.rs`. Every resulting parent/section file is
  below 2000 lines, the true pre-refactor HEAD anchor closed with
  `anchor_mismatches = 0`, and required Rust gates passed. The six 2000-2500
  line files remain deferred advisory WARN work. Package:
  `20260618-refactor022-monolith-line-count-split-001/`.
- PERFARRAY02 is executed-NO-GO (WB11 request/accessor authority split +
  integrated floor). The flag-gated array request/accessor seam and real WB11
  runoff pilot landed, and default-vs-pilot identity passed on OFE5 and H2637
  (HBP/loss/plot/wat checksums equal; pass parquet rows equal). The H2637
  array-native pilot measured `817.810 us/OFE-day`, above the `386 us/OFE-day`
  <=10x budget and `193 us/OFE-day` 5x stretch. Boundary seed/materialize was
  `1685.023 us/OFE-day` and reported separately. Verdict: do not ratify
  ADR-0023 from this pilot; do not proceed to broad Stage C-F migration without
  a new kernel output/writeback-shape decision. Package:
  `20260618-perfarray02-wb11-request-accessor-authority-split-001/`.
- PERFARRAY01 is executed-NO-GO as scoped (WB11 integrated
  array-authoritative pilot, Stage A + B). Stage A landed a default-unwired
  array contract shell in `openwepp-kernel-contract` and focused crate gates
  passed. Stage B did not run: static inspection showed the current
  `HillslopeKernelRequest` and scheduler still require logical `BTreeMap`
  state/flux maps for kernel reads, consumer-boundary validation, logical
  writeback apply, and indexed mirror synchronization. Any pilot from that seam
  would violate the package's no per-day export or no dual-write proofs. No
  H2637 floor measurement; ADR-0023 remains unratified. Package:
  `20260618-perfarray01-wb11-integrated-array-authoritative-pilot-001/`.
- PERFARCH02 is complete (architecture scoping + floor prototype). Verdict:
  CONDITIONAL GO to an integrated WB11 array-authoritative pilot. The
  artifact-local prototype preserved exact exported-map identity for the
  prototyped writeback/guard flow, preserved fail-closed rejection/message-id
  class with lazy failure subjects, and measured the array-authoritative
  writeback/guard path at roughly 49.9x faster than the current logical
  writeback/guard path. Interpretation: <=10x remains credible only through an
  integrated WB11 pilot; 5x remains unproven. Package:
  `20260618-perfarch02-array-authoritative-hot-path-state-redesign-001/`.
- PERFIDX06 is complete (Stage 6: high-OFE target assessment). Same-machine H2637
  measurements pinned the PERFIDX04 endpoint at `666.82s` no-UI and `667.44s`
  with UI; pinned legacy medians were `9.12s` no-UI and `11.54s` with UI. The
  resulting ratios are `73.12x` no-UI and `57.84x` with UI. Verdict: `<=10x`
  is not closed, `<=5x` is not plausible under the current read-mirror design,
  and the next perf move is redesign scoping, not more narrow write-side
  id-table work. Package:
  `20260618-perfidx06-high-ofe-target-assessment-001/`.
- PERFIDX05 is HELD (Stage 5: writeback/guards by SymbolId). Bit-identical but
  performance-NEGATIVE (H2637 −5.3–5.8%) — the write/guard-side dual-write cost
  (logical + mirror) exceeds the id saving; a structural ceiling of the read-mirror
  design, not incompleteness. Code discarded, record kept. Package:
  `20260617-perfidx05-writeback-guards-by-id-001/`.
- PERFIDX04 is complete (Stage 4: resolve-once hot-symbol-id tables + indexed
  read-mirror). H2637 −24.3%/−25.2%, bit-identical, irrigation excluded. Package:
  `20260617-perfidx04-hot-symbol-id-tables-001/`. Endpoint stands as the perf state.
- PERFIDX03B is complete as the blocker-closure follow-on to held PERFIDX03.
  Scope: indexed kernel seam/export-cache work needed before Stage 4. Package:
  `20260617-perfidx03b-indexed-kernel-seam-or-export-cache-001/`.
- CQR36 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-input-contract/src/parsers/watershed_impoundment.rs`.
  Final target `parse_impoundment` CRAP is `15.0`, with zero unique
  target-file rows above `30`. WARNs remain for `cargo crap` LCOV source-map
  warnings.
  Package:
  `20260616-cqr36-watershed-impoundment-parser-complexity-001/`.
- CQR35 is complete-with-warnings for live-metric
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`.
  Fresh before and after metrics prove the highest target-file row is
  `Wb11HydrologyKernel::wb19_lateral_transfer_inputs` at CRAP
  `26.541362973760947`, with zero target-file rows above `30`. WARNs remain
  for `cargo crap` LCOV source-map warnings and the target file line count
  above the older caution threshold.
  Package:
  `20260616-cqr35-lateral-drainage-complexity-001/`.
- CQR34 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-summary-accumulator/src/lib.rs`. The scoped target reduced
  `SummaryAccumulatorError::fmt` CRAP from `240.0` to `1.0`; the extracted
  private helper `SummaryAccumulatorError::write_display` is CRAP `15.0`.
  WARNs remain for `cargo crap` LCOV source-map warnings and the same-file
  out-of-scope `Wb13DailyWaterBalanceRow::from_surface` row above CRAP `30`.
  Package:
  `20260615-cqr34-summary-accumulator-complexity-001/`.
- CQR33 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-input-contract/src/parsers/watershed_structure.rs`. The
  scoped target reduced `WatershedStructureParseError::fmt` CRAP from `240.0`
  to `1.0`; the extracted private helper
  `WatershedStructureParseError::write_display` is CRAP `15.0`. WARNs remain
  for `cargo crap` LCOV source-map warnings and the same-file out-of-scope
  parser row above CRAP `30`. Package:
  `20260615-cqr33-watershed-structure-parser-complexity-001/`.
- CQR32 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity closure of
  `crates/openwepp-input-contract/src/parsers/climate.rs`. The scoped target
  reduced `ClimateParseError::fmt` CRAP from `240.0` to `1.0`; the extracted
  private helper `ClimateParseError::write_display` is CRAP `15.0`. WARNs
  remain for `cargo crap` LCOV source-map warnings, same-file out-of-scope
  parser rows above CRAP `30`, and target-file line coverage below the ADR-0021
  glue-tier threshold. Package:
  `20260615-cqr32-climate-parser-complexity-001/`.
- CQR31 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`.
  The scoped target reduced `build_simulation_owned_wb13_row_for_ofe` CRAP from
  `251.62932776803854` to `16.0`, with every newly extracted helper CRAP
  `<= 12.584884659264825`. WARNs remain for `cargo crap` LCOV source-map
  warnings and the pre-existing same-file out-of-scope
  `derive_profile_fc_store_from_authoritative_layers` row above CRAP `30`.
  Package: `20260615-cqr31-runner-output-climate-complexity-001/`.
- CQR30 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod13.rs`.
  The scoped target reduced `Wb11HydrologyKernel::run_erod13_wave1_core`
  CRAP from `265.2636791582994` to `8.0`, with every newly extracted helper
  CRAP `<= 29.0`. WARNs remain for `cargo crap` LCOV source-map warnings.
  Package: `20260615-cqr30-erod13-wave1-complexity-001/`.
- CQR29 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`.
  The scoped target reduced `Wb11HydrologyKernelGuardError::fmt` CRAP from
  `272.0` to `1.0`, with every newly extracted helper CRAP
  `<= 8.000751314800901`. WARNs remain for `cargo crap` LCOV source-map
  warnings.
  Package: `20260615-cqr29-guard-errors-complexity-001/`.
- CQR28 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`.
  The scoped target reduced `run_percolation` CRAP from
  `281.82979375564685` to `17.19373252009578`, with every newly extracted
  helper CRAP `<= 22.896222121074196`. WARNs remain for `cargo crap` LCOV
  source-map warnings and pre-existing same-file out-of-scope rows above CRAP
  `30`. Package: `20260615-cqr28-plant-percolation-complexity-001/`.
- CQR27 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-input-contract/src/parsers/management.rs`. Package:
  `20260615-cqr27-management-parser-complexity-001/`. Final target:
  `parse_yearly_annual_fallow`, CRAP `4.0`.
- CQR26 is complete-with-warnings for live-metric closure of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`.
  Package:
  `20260615-cqr26-lateral-drainage-complexity-001/`.
- CQR25 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.
  Final target CRAP: `12.4198250729`. Package:
  `20260615-cqr25-runner-intake-lane-setup-complexity-001/`.
- CQR24 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`.
  The scoped target reduced `produce_wb16_ealpha_from_runtime_surface` CRAP
  from `317.2103869084884` to `6.010666666666666`, with every newly extracted
  WB16 helper at CRAP `<= 15.401920438957477`, without changing public API,
  runtime symbols, publication formulas, typed guard behavior, parser
  compatibility, or science-contract behavior. WARNs remain for target-file
  coverage below the ADR-0021 line threshold and pre-existing same-file
  out-of-scope rows above CRAP `30`. Package:
  `20260615-cqr24-scheduler-seed-runtime-complexity-001/`.
- CQR23 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`.
  The scoped target reduced `run_erod19_route_segment_migration` CRAP from
  `351.9234211799049` to `9.00460855712335`, with every newly extracted helper
  below `15`, without changing public API, runtime symbols, writeback order,
  typed guard behavior, parser compatibility, or science-contract behavior.
  WARNs remain for target-file coverage below the ADR-0021 line threshold and
  the pre-existing out-of-scope `erod19_depend` row at CRAP
  `87.98408081839372`. Package:
  `20260615-cqr23-erod19-route-segment-complexity-001/`.
- CQR22 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of `crates/openwepp-input-contract/src/parsers/soil.rs`.
  Package: `20260615-cqr22-soil-parser-complexity-001/`. Final target CRAP:
  `5.0`.
- CQR21 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of `crates/openwepp-climate-runtime-adapter/src/lib.rs`.
  Package: `20260615-cqr21-climate-runtime-adapter-complexity-001/`. Final
  target CRAP: `2.0`.
- CQR20 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`.
  Package: `20260615-cqr20-projection-helpers-complexity-001/`. Final target
  CRAP: `9.0`.
- CQR19 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-watershed-orchestrator/src/runtime_inputs_mod/types.rs`.
  Package: `20260615-cqr19-watershed-runtime-types-complexity-001/`. Final
  target CRAP: `6.0`.
- CQR18 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-input-contract/src/parsers/hbp/payload_validator.rs`.
  Package: `20260615-cqr18-hbp-payload-validator-complexity-001/`. Final
  target CRAP: `9.0`.
- CQR17 completed behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod19.rs`.
  Package: `20260615-cqr17-hydrology-erod19-complexity-001/`. Final target
  CRAP: `2.0`.
- CQR16 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-sim-contract/src/units_mod/registries.rs`.
  The scoped target reduced `BoundaryUnitRegistryError::fmt` CRAP from
  `506.0` to `6.0`, with every newly extracted helper at CRAP
  `11.00102848303003` or lower, without changing public API, registry rows,
  aliases, units, publication units, scalar exceptions, parser compatibility,
  or science-contract behavior. Required Rust closure gates passed. WARNs
  remain for target-file coverage below the full ADR-0021 module threshold and
  the pre-existing out-of-scope `validate_entry` row at CRAP
  `62.4742520806637`. Package:
  `20260615-cqr16-unit-registries-complexity-001/`.
- CQR15 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`.
  The scoped target reduced `seed_wb11_runtime_surface_inputs` CRAP from
  `580.6018405181356` to `15.0`, with every newly extracted helper at CRAP
  `23.01930315500686` or lower, without changing public API, runtime symbols,
  lane policy, typed guard behavior, formulas, parser compatibility, or
  science-contract behavior. Required Rust closure gates passed. WARNs remain
  for target-file coverage below the full ADR-0021 module threshold, target
  file line count above `2000`, and unrelated out-of-scope target-file rows
  above CRAP `30`. Package:
  `20260615-cqr15-scheduler-seed-runtime-complexity-001/`.
- CQR14 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-runner/src/release.rs`. The scoped target reduced
  `lint_release_directory` CRAP from `650.0` to `4.0`, with every newly
  extracted release-lint helper below `9`, without changing public API,
  release sidecar schema, binary role classification, stable error variants,
  candidate filtering, HBP pair parity, hash, timestamp, or JSON field
  behavior. Required Rust closure gates passed. WARN remains for the
  pre-existing out-of-scope `validate_release_sidecar_unlocked` row at CRAP
  `31.459079074798446`. Package:
  `20260615-cqr14-runner-release-complexity-001/`.
- CQR13 is complete for live-metric closure of the rank-7
  CRAP/cyclomatic-complexity row in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`.
  Fresh before metrics proved the snapshot row had already been closed by
  prior runtime core type decomposition: the highest current target-file CRAP
  row is `HillslopeRuntimeInputError::soil_core_code` at
  `14.0478515625`, with every row below `30` and target-file line coverage
  `497/515`. No production refactor was needed. Required Rust closure gates
  passed. Package:
  `20260615-cqr13-runtime-core-types-complexity-001/`.
- CQR12 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.
  The scoped quality target reduced
  `seed_hillslope_runtime_surface_from_irrigation_depletion` CRAP from
  `1122.0` to `2.0`, with every newly extracted depletion helper below
  `10`, without changing public API, typed guard classes, stable error fields
  and allowed strings, depletion irrigation symbols, units, parser
  compatibility, period iteration, sprinkler/furrow field meanings, or
  kernel-facing projection behavior. Required Rust closure gates passed. WARN
  holds remain for target-file coverage below the science-tier threshold and
  the pre-existing out-of-scope frost `too_many_lines` suppression. Package:
  `20260615-cqr12-irrigation-depletion-runtime-001/`.
- CQR11 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-input-contract/src/parsers/management.rs`. The scoped
  quality target reduced `parse_yearly_perennial` CRAP from `1406.0` to `4.0`,
  with every newly extracted perennial parser helper below `10`, without
  changing public parser API, typed error variants, stable error IDs, field
  names, count/cardinality guards, branch compatibility, parser output shape, or
  runtime/kernel-facing management semantics. Required Rust closure gates
  passed. WARN holds remain for target-file coverage below the science-tier
  threshold and pre-existing out-of-scope CRAP rows above `30`. Package:
  `20260615-cqr11-management-parser-complexity-001/`.
- CQR10 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/04_snow_frost_irrigation.rs`.
  The scoped quality target reduced
  `seed_hillslope_runtime_surface_from_irrigation_fixeddate` CRAP from
  `1482.0` to `4.0`, with every newly extracted fixed-date helper below
  `15`, without changing public API, typed guard classes, stable error fields
  and allowed strings, fixed-date irrigation symbols, units, parser
  compatibility, event order, furrow formulas, or kernel-facing projection
  behavior. Required closure gates passed. WARN holds remain for target-file
  coverage below the science-tier threshold and the pre-existing out-of-scope
  depletion CRAP row above `30`. Package:
  `20260615-cqr10-irrigation-fixeddate-runtime-001/`.
- CQR09 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs`.
  The scoped quality target reduced `build_annual_decomposition_control` CRAP
  from `1497.0871919084125` to `9.179748500041095`, with every newly extracted
  annual helper below `14`, without changing public API, typed guard classes,
  stable error reasons, decomposition symbols, units, parser compatibility,
  scheduler payload fields, or output formulas. Required closure gates passed.
  WARN holds remain for target-file coverage below the science-tier threshold
  and pre-existing out-of-scope CRAP rows above `30`. Package:
  `20260615-cqr09-decomposition-equations-complexity-001/`.
- CQR08 is complete for behavior-preserving function-length/lint-debt
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/00_core_types.rs`.
  The scoped quality target removed the `HillslopeRuntimeInputError`
  `fmt::Display` `#[allow(clippy::too_many_lines)]` suppression and reduced the
  target error-code/display CRAP rows from `964.0467577461321` and `4290.0` to
  helper rows all below `15`, without changing stable error codes, display text,
  typed variant semantics, runtime projection guards, or public API behavior.
  Required closure gates passed. Package:
  `20260615-cqr08-runtime-core-types-display-001/`.
- CQR07 is complete-with-warnings for behavior-preserving
  function-length/lint-debt decomposition of
  `crates/openwepp-runner/src/watershed_wat.rs`. The scoped quality target
  removed the `read_batch_into` `#[allow(clippy::too_many_lines)]` suppression,
  reducing `read_batch_into` CRAP from `4830.0` to `4.0`, without changing WAT
  reader, aggregation, optional-column, fail-closed, or public publication
  behavior. Required closure gates passed. WARN holds remain for target coverage
  below the science-tier threshold and pre-existing out-of-scope CRAP rows above
  `30`. Package:
  `20260615-cqr07-watershed-wat-complexity-001/`.
- CQR06 is complete-with-warnings for behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_lateral_drainage.rs`.
  The scoped quality target decomposed WB19 lateral-transfer, drainage, and
  top-layer conductivity adjustment helpers so every eligible target-module
  function has CRAP `<= 26.541362973760947`, without changing WB19 formulas,
  typed guard IDs, symbol names, arithmetic grouping, thresholds, unit
  conversions, writeback order, or public crate APIs. Required closure gates
  passed. WARN holds remain for target-file line count over 2000 and target
  coverage below the science-tier threshold after private helper extraction.
  Package:
  `20260615-cqr06-lateral-drainage-complexity-001/`.
- CQR05 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_storage_erosion/hydrology_phase_erod14.rs`.
  The scoped quality target decomposed `run_erod14_wave2` so every eligible
  target-module function has CRAP `<= 23.0`, without changing EROD14 Wave-2
  formulas, typed guard IDs, symbol names, arithmetic grouping, thresholds,
  writeback order, or public crate APIs. Required closure gates passed. WARN
  hold remains for target coverage below the science-tier threshold after
  private helper extraction. Package:
  `20260615-cqr05-erod14-wave2-complexity-001/`.
- CQR04 is complete-with-warnings for behavior-preserving
  CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs`.
  The scoped quality target decomposed high-risk watershed channel routing
  helpers, especially `ws20_route_case12_segment_family`, so every eligible
  target-module function has CRAP `<= 30`, without changing WS10/WS11/WS20-WS24
  routing behavior, typed guard IDs, symbol names, arithmetic grouping,
  thresholds, or public crate APIs. Required closure gates passed. WARN holds
  remain for target-file line count over 2000 and target coverage below the
  science-tier threshold after private helper extraction. Package:
  `20260615-cqr04-watershed-routing-complexity-001/`.
- CQR03 is complete for behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`.
  The management runtime projection dispatcher and primary live-canopy
  assimilation helper are decomposed into private stage helpers, the obsolete
  target-file `too_many_lines` suppressions are removed, and every eligible
  target-module function has CRAP `<= 17.16724537037037` after the refactor.
  Required closure gates passed. Package:
  `20260615-cqr03-management-runtime-inputs-complexity-001/`.
- CQR02 is complete for behavior-preserving CRAP/cyclomatic-complexity
  decomposition of
  `crates/openwepp-input-contract/src/parsers/hbp/layout_parser.rs`.
  `parse_layout` is now a staged dispatcher over private parser helpers, public
  HBP parser APIs are unchanged, and every eligible target-module function has
  CRAP `<= 20.0` after the refactor. Required closure gates passed. Package:
  `20260615-cqr02-hbp-layout-parser-complexity-001/`.
- CQR01 is complete for behavior-preserving code-quality decomposition of
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`.
  `compute_active_frost_coupling` no longer carries the
  `#[allow(clippy::too_many_lines)]` suppression, remains public-surface
  compatible, and its target CRAP row improved from `238.28646229402713` to
  `8.003859752282304`. Required closure gates passed. Package:
  `20260615-cqr01-frost-entry-complexity-001/`.
- REFACTOR024 is complete for a behavior-preserving line-count split of
  `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`. The root
  integration test is now an 11-line module harness with support and concern
  modules under `tests/integration/clim06_frost_frozen_soil_kernel_contract/`;
  all split files are below 1000 lines. The original 46 test functions remain
  present and the required closure gates passed. Package:
  `20260614-refactor024-clim06-frost-test-line-count-split-001/`.
- REFACTOR023 is complete for the 3000+ line
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`
  mechanical split. Scope is behavior-preserving module extraction only:
  `coupling.rs` remains the snow/interval wiring surface while frost helpers
  moved under `support_helpers_mod/coupling/`. Final line counts:
  `coupling.rs=230`, `coupling/frost.rs=1838`,
  `coupling/frost_entry.rs=1000`. Required closure gates passed. Package:
  `20260614-refactor023-hillslope-coupling-line-count-split-001/`.
- HPHYS0320 **closed the SIMIMPL28 storm-start timing seam** (`wnttim < 1.0 -> 1.0`,
  `INV-CLIMATE-018`). This was the first real forcing correction of the entire
  HPHYS0298->0320 snow-comparator arc — and it was a **climate-forcing timing
  defect, not snow physics**. The snow surface was only where the symptom showed.
- The HPHYS0298->0320 snow/`RM` comparator route (the combined `57` carried rows)
  remains retired per ADR-0017 (comparator is a flag, not a target). **Do not open
  HPHYS0321 to continue that route.**
- The snow science review (`docs/backlog/20260605-snow-code-deferred-science-review.md`)
  is now **promoted and split into two stages** (static analysis of the J-95
  negative-SWE site, 2026-06-06): **Stage 1 = snow mass conservation /
  single-sourcing** — an architecture/conservation hard gate that sits on rung-1's
  closure gate, so it is **active now** (see the SNOWSCI Stage-1 package below);
  **Stage 2 = snow physics-magnitude** — the `snowd.for` equation adjudication,
  which **stays deferred behind the protected boundary.** Snow *conservation* is no
  longer suspended; snow *magnitude* still is.
- **WSHED01 closed the openWEPP-native totalwatsed3 CLI + closure** (2026-06-14,
  item 9) — the WBVAL06/6a end-to-end totalwatsed3 deferral is **resolved** on
  openWEPP-native output (`openwepp-cli-totalwatsed3`, ADR-0019/0020), closing
  ex-day-1 at `−0.41 mm/2191 d` with independent operands. Channel
  water-balance routed output (`chanwb`) is a **separate** follow-on
  (`WATERSHED-CHANWB-ROUTED-OUTPUT`), decoupled from the hillslope-only
  totalwatsed3 per ADR-0020.
- **FARPOINT01 closed the MOFE >10-OFE far-point demonstration** (2026-06-16,
  item 11) — openWEPP's three identities close at 19 OFEs on H2637, past the
  legacy ≤10-OFE ceiling. F-B closed (contract-first) a frost `watbtm`
  double-count the substrate surfaced; F-C contrasted closure (legacy with_ui
  runoff = 127.7 % of precip — q-cap violation — vs openWEPP 71 %, bounded); the
  `watpdg` branch-out resolved as a validated non-defect. **MAGPARITY01 closed
  2026-06-18** with no transfer/area/export defect. **STAGE2-LATQCC closed
  2026-06-18** with no WB19 equation or operand-bound defect; the remaining
  bounded runoff delta is an absolute lateral-flow magnitude authority gap. The
  ~80–110× high-OFE
  wall-clock gap is scaffolded as `PERFHO01`.

Active work sequence (each rung adds one mechanism on an already-closed
foundation; boundaries are closure gates, not calendar phases).

[kernel refactor follow-on package-complete-with-hold] complete `lib_mod/kernel.rs` decomposition
from `kernel_core.rs` into bounded modules before any bounded surface migration.
 WBVAL02 and
WBVAL03 are Defect-Closure ExecPlan unblockers created from WBVAL01 evidence;
they are bounded defect closures, not a return to diagnostic relay packages.
WBVAL04 is the right-sized post-climate-fix redo of WBVAL01, gated first by a
publication-safe Daymet CLI audit:

1. **WBVAL01** *(executed-hold)* — validation/characterization of single-OFE
   water-balance **conservation closure** on a real CLIGEN daily (non-breakpoint)
   Rocky Mountain run (`/wc1/runs/in/indispensable-presenter`, DRIGGS ID).
   Execution discovered `22` single-OFE hillslopes plus `pw0` as a 9-OFE
   observed-only surface. `12/22` single-OFE hillslopes emitted complete WAT
   ledgers and all `12` are `conservation-break` for years `2..6`; `10/22`
   failed closed before WAT publication (`CLIM-RUNTIME-E-017` or
   `HKERNEL-WB11-PERC-E-003`). This grounds frost targets for emitted ledgers
   while preserving a required follow-on unblocker for the domain-guarded
   hillslopes and the missing year-1 initial-storage surface.
2. **WBVAL02-SIMIMPL28-RADBOUND** *(complete: validated invalid upstream input)* — closed defect
   `WBVAL02-CLIM-RUNTIME-E-017-RADBOUND` for the six WBVAL01 radiation-bound
   fail-closed single-OFE hillslopes (`p2`, `p4`, `p6`, `p9`, `p14`, `p17`).
   The shared DRIGGS daily climate record is invalid at the active SIMIMPL28
   source seam: on `1990-02-18`, `radly=486 Ly d^-1` exceeds baseline `sunmap`
   horizontal potential `r3=453.068716 Ly d^-1`. WBVAL02 amended
   `SC-CLIMATE-001`, added contract tests, and moved the fail-closed evidence
   to typed source symbol `radly`; no radiation guard was loosened and no
   snow/percolation compensation was authorized.
3. **WBVAL03-SNOWMELT-WB-CLOSURE** *(executed-hold)* — close the four
   WBVAL01 J-95 `HKERNEL-WB11-PERC-E-003` fail-closed hillslopes (`p7`, `p11`,
   `p18`, `p20`) and attribute the emitted-ledger conservation residual using a
   complete water-balance identity. Authority/write-set is
   snowmelt/storage/percolation/WAT publication. The closure leak is
   diagnostic-first only inside the package; it is not a diagnostic-only
   package. Current execution is legitimately held behind the upstream DRIGGS
   `radly` source-bound defect (`WBVAL04`): after WBVAL02, all four J-95
   targets and all 12 prior WAT-emitting hillslopes fail earlier at
   `CLIM-RUNTIME-E-017`, `radly=486`.
4. **WBVAL04-WBVAL01-REDO** *(executed-hold)* — reran the whole WBVAL01 Rocky
   Mountain single-OFE validation population after the observed-Daymet producer
   emitted CLI-safe radiation. The climate precondition now passes with zero
   `rad > baseline sunmap.r3` rows. The release validation batch ran all `22`
   single-OFE hillslopes: `18` emitted WAT and all `18` are
   conservation-break for years `2..6`; `p7`, `p11`, `p18`, and `p20` still
   fail closed at J-95 with `HKERNEL-WB11-PERC-E-003`. WBVAL04 routes two
   defect-shaped follow-ons: `WBVAL05-J95-HKERNEL-WB11-PERC-E-003` and
   `WBVAL06-SINGLE-OFE-WAT-CONSERVATION-RESIDUAL`.
5. **WBVAL05-J95-PERCOLATION** *(executed, hold-boundary)* — landed a
   contract-first WB18 fix (`SC-PERC-001` v29: WB18 consumes a published
   `wb12_infiltration` instead of recomputing the WB14/WB12 snow-liquid partition
   and re-validating snow state it does not own); no guard loosening. This cleared
   `HKERNEL-WB11-PERC-E-003` but relocated the fail-closed to
   `HKERNEL-WB14-RUNOFF-E-003`, exposing the true root cause: **negative
   `snow.runtime_swe = -0.006171`**. Legitimately held at the snow boundary; its
   negative-SWE follow-on is folded into SNOWSCI Stage 1.
6. **SNOWSCI Stage 1 — snow mass conservation / single-sourcing**
   *(closed-with-follow-up-postreview)* — closed
   `SNOWSCI-S1-SNOW-MASS-NONCONSERVATION` for the observed J-95 negative-SWE
   fail-closed mechanism by single-sourcing routed snowpack melt to the
   authoritative post-hourly depth/density store. The fix removed the WBVAL05
   publication blocker for `p7`, `p11`, `p18`, and `p20` without a snow
   physics-magnitude change or silent clamp. Post-review gates ran
   `cargo test --workspace`, workspace clippy, `cargo deny check`, fresh
   H1..H39 release/semantic validation, and WBVAL06 before/after residual
   measurement. WBVAL06 annual residual attribution was closed by
   `20260606-wbval06-single-ofe-wat-conservation-residual-defect-closure-001/`.
   Package:
   `20260606-snowsci-stage1-snow-mass-conservation-closure-001/`.
6a. **totalwatsed3 interception-flux audit companion** *(DONE — wepppy
   `aeef2cc6c`)* — WBVAL06 published the daily interception flux as
   `H.wat.Interception`, but openWEPP closure was shown only under its own
   identity audit. The acceptance surface is the **totalwatsed3** WB audit, which
   closes `P - (Runoff + Lateral + ET + Percolation) - ΔStorage`. This WP added
   `Interception` as an optional first-class outflow in
   `wepppy/wepp/interchange/totalwatsed3.py` and
   `tools/totalwatsed3_daily_closure_audit.py` (default 0 when absent, so legacy
   runs close unchanged; **`ET` untouched**). On openWEPP post-WBVAL06 output the
   totalwatsed3 closure identity now closes to ~`2e-7 mm/yr` for years `2..6`
   (vs ~15-19 mm without interception). WP:
   `wepppy/docs/work-packages/20260607_totalwatsed3_interception_flux_closure/`.
   Note: acceptance used a WAT-aggregated totalwatsed3-like surface; a full
   end-to-end totalwatsed3 run awaits openWEPP watershed outputs (MOFE rung).

   **RUNG-1 (single-OFE water-balance closure) is COMPLETE:** SNOWSCI-S1 (snow
   conservation) + WBVAL06 (interception publication) + 6a (totalwatsed3 audit
   consumes interception) → single-OFE WB closes and is auditable on the real
   surface. Next rung: **frost** (item 7).
7. **frost** *(rung-2 — FROSTVAL01 complete after follow-ons)* —
   infiltration/percolation gate (`ksflag`/`ksatadj`) on the closed single-OFE
   vertical balance, with no routing to alias it. **FROSTVAL01** originally ran
   the standard-WEPP `ksflag = 1` frost validation on
   `/wc1/runs/al/algebraic-radium` (43 single-OFE; all lanuse=1→ksflag=1; gridmet
   daily; comparator `wepp_260606`) and held. Findings from that first run (per
   Claude review): 37/43 blocked by `HS-RUNTIME-E-062` (soil-coverage); the
   frost-closure ledger was broken (its ~10 mm inputs were a tool-aggregation bug
   — openWEPP WAT `P` was verified correct/complete at 911 mm/yr — so the
   `frost-break` verdict was withdrawn); and openWEPP's own output showed real
   zero-term anomalies on the runnable cohort (`Q`/`Ep`/`Er`/`Interception` = 0)
   plus likely frost non-activation (`frozwt`=0 at a freezing site with real
   water). The ordered follow-ons closed those blockers: **FQ-1** soil-coverage
   unblock; **FQ-2** ledger fix folded into FQ-4; **FQ-3** ET/runoff zero-term
   characterization/closures; **FQ-4** frost-activation closure. A 2026-06-11
   FROSTVAL01 rerun over all 43 single-OFE prefixes now satisfies activation and
   closure-under-frost: `43/43` `frsoil.active=true`, `43/43` nonzero `frozwt`,
   paired frost-off runs change hydrology on all prefixes, and annual closure
   over 258 rows has max abs residual `3.22e-11 mm`. This exercises the standard
   `ksflag` gate, not the forest `ksatadj` model (separate concern). Package:
   `20260608-frostval01-ksflag-frost-single-ofe-closure-validation-001/`.
7a. **FQ-1 soil corrected-layer coverage** *(executed-hold-boundary)* — closed
   the population-scale `HS-RUNTIME-E-062` soil coverage blocker from
   FROSTVAL01. `SC-SOIL-001` v23 now requires valid parser-layer corrected
   diagnostics to extend the deepest normalized corrected interval to parser
   profile bottom while preserving normalized WB11/WB18/WB19 seed-grid authority.
   Post-fix algebraic-radium validation has zero `HS-RUNTIME-E-062` failures:
   `42/43` prefixes emit `H.wat.parquet` + `H.hbp`; `p11` now fails later at the
   protected percolation boundary with `HKERNEL-WB11-PERC-E-003` on `1990-162`.
   Handoff: `FQ1-P11-HKERNEL-WB11-PERC-E-003-J162`. Package:
   `20260608-fq1-soil-corrected-layer-coverage-closure-001/`.
7b. **FQ-3 runoff `Q/QOFE` underproduction** *(complete)* — closed
   `FQ3-DC-RUNOFFPART-QQOFE-001` for the post-FQ1 algebraic-radium single-OFE
   population. `SC-RUNOFFPART-001` v39 now requires WB12/WB14 to apply the
   top-two-layer storage limit before same-pass infiltration publication and to
   consume the WB18/percolation-produced infiltration value when it already
   owns the same-pass storage update. Post-fix validation produced nonzero
   `Q/QOFE` on all `42/42` runnable prefixes while preserving annual WAT closure
   at numerical noise (`max_abs=2.81e-11 mm`). Package:
   `20260608-fq3dc-runoffpart-q-qofe-closure-001/`.
7c. **FQ-3 Corn annual ET/canopy engagement** *(complete)* — closed
   `FQ3-DC-ET-CORN-ENGAGEMENT-001` for the post-FQ1 algebraic-radium Corn
   population. The annual PL activation sentinel was being deleted on pre-plant
   days and the scheduler calendar `day` symbol was day-of-month instead of
   Julian day, so annual Corn never reached its `jdplt` activation path.
   `SC-PLANT-001` v18 and `SC-EVAP-001` v26 now require annual pre-plant skips
   to be day-local and preserve PL schedule sentinels. Validation over all
   `36/36` Corn prefixes produced nonzero `Ep` and `Interception` with annual
   closure at numerical noise (`max_abs=3.16e-11 mm`). Upstream FQ-3 evidence
   classified `Er=0` as expected-config-zero (`legacy=0`), so this package
   closes the Corn engagement defect for `Ep`/canopy interception and records
   the original `Er` wording as an overclaim, not an unresolved defect. Package:
   `20260608-fq3dc-et-corn-engagement-closure-001/`.
7d. **FQ-4 ksflag frost activation + closure** *(complete)* — closed
   `FQ4-FROST-KSFLAG-ACTIVATION-001`. The root cause was an overbroad activation
   gate: openWEPP treated `frost.options.frost_file_present=0` as disabling
   frozen-soil coupling even when parsed missing-file defaults supplied valid
   standard frost controls with `wintRed=1`. `SC-SNOWFREEZE-001` v53 now makes
   frost file presence provenance-only for activation; `wintRed=1` plus active
   thermal/runtime triggers activates `frsoil`. Post-fix validation ran all `43`
   single-OFE prefixes: all emitted WAT, all had `frsoil.active=true`, all had
   nonzero `frozwt`, and annual closure with `SoilWaterTotal` held at numerical
   noise (`max_abs=3.22e-11 mm`). The old FROSTVAL01 `frost-break` verdict is
   withdrawn as a defective ledger artifact. Package:
   `20260608-fq4-ksflag-frost-activation-closure-001/`.
7e. **FDMC01 frost-depth comparator characterization** *(complete)* — sized the
   frost depth-model gap left open by FQ-4: openWEPP's freeze-index proxy
   (`frdp = 0.20·clamp(−mean_temp/6)`, capped 0.20 m) vs legacy heat-flow.
   Verdict **materially off** — depth capped 200 mm vs legacy 240–503 mm
   (43/43 exceed the cap), depth-series median correlation 0.13, frozen
   duration +258 days (ratchet over-persistence). This verdict + the
   settle-vertical-before-routing principle promoted frost-depth heat-flow
   parity to ROADMAP queue item 1 ahead of MOFE (2026-06-07). Package:
   `20260608-fdmc01-frost-depth-comparator-characterization-001/`.
7f. **FDHP01 frost-depth heat-flow parity** *(complete)* — replaced the
   freeze-index proxy with the single-OFE fine-sublayer heat-flow frost state
   machine (`INV-SNOWFREEZE-006`/`-012`, legacy `frostn` lineage, CRM Ch. 3.8,
   Dun et al. 2010), added WAT `frdp` publication, restored WAT
   `SoilWaterTotal` as the unfrozen `Total-Soil` alias, and bound WAT `frozwt`
   to the layered `Σ soilf(i)` store. The D3 staged arc landed daily
   `frwatc` handoffs, fine-layer freeze/thaw arms, capacity/overflow
   ownership, in-hour resistance feedback, seasonal lower-front heat,
   residue/shallow-front resistance, fixed frozen-path conductivity authority,
   and legacy `hr_tmp`/`tmpadj` surface-temperature synthesis. Dk certified the
   package at the declared ADR-0017 boundary: the Dj/Dk forced-snow cohort is
   `43/43` clean, years 2-6 independent `Total-Soil + frozwt` closure is
   `5.09e-7 mm`, profile-bound pinning is gone (`0/43`), mean/median max depth
   are `501.36/492.36 mm`, median depth correlation rose from the FDMC01
   `0.13` baseline to `0.764`, and frozen-duration residual collapsed from
   `+258` to `+61` days. Residual items are handoffs, not blockers: F4 snow
   density/depth-split magnitude, `p2` individual attribution, dynamic
   residue/decomposition `resdep` lifecycle exposure, and characterized
   upper-envelope subgroup deltas. `SC-SNOWFREEZE-001` v69 closes/re-states
   `GAP-SNOWFREEZE-002`; MOFE is now the next ROADMAP item. Package:
   `20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/`.
8. **MOFE** *(rung 3 — MOFE01 hillslope water-routing closure complete)* —
   closed inter-OFE run-on/run-off routing on the frost-settled per-element
   balance using the `/wc1/runs/ar/arboreal-dendrite/wepp` graded 1–5-OFE
   ladder. M-H ran all 36 hillslopes with fresh openWEPP outputs: 36/36 exited
   zero, row cardinality matched exactly (`271808/271808` rows), transfer
   residual max was `0.0 mm`, per-element residual max was
   `5.968558980384842e-13 mm`, aggregate cancellation residual max was
   `0.0 mm`, downstream `QOFE == Q` alias rows were zero, hydrology clone
   active days were zero, and the 7 single-OFE anchors were 28/28
   byte-identical to the M-F-REDO2 anchor. M-I added the independent in-runner
   hillslope-total identity and closed it at `3.306423012547295e-13 mm`
   against `1e-9 mm`, with all multi-OFE cases nonzero-at-noise; it also
   source-guards the mutually exclusive multi-OFE persistent and single-OFE
   aggregate scheduler lifecycles. Local `owcmp` was run directly without the
   comparator subagent: row keys align for all 36 hillslopes, while semantic
   value-family comparison remains an ADR-0017 investigation signal, not an
   acceptance target. M-G deliberately left sediment-coupled erosion `qin/qout`
   plus particle-fraction handoff as a named follow-on. Package:
   `20260612-mofe01-inter-ofe-routing-closure-001/`. **Closure (2026-06-14):** MOFE01 water-routing closure is done-done on the 36-run 1–5-OFE ladder. Named follow-ons: `MOFE-FARPOINT01` (>10-OFE exceed-the-ceiling demonstration), `MOFE-MAGPARITY01` (completed 2026-06-18; no transfer/area/export defect, Stage-2 lateral/subsurface magnitude flag), `REFACTOR022` (line-count split), plus watershed/totalwatsed3 (queue item 1) and `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` (sediment coupling).

   **Next rung — WSHED01 (openWEPP-native totalwatsed3 CLI + closure)** *(complete 2026-06-14)*: closed the end-to-end totalwatsed3 water-balance audit on openWEPP-native output (the WBVAL06/6a deferral). See item 9 below for the W-arc→T-arc pivot (ADR-0019/0020), the three-iteration runvol fix, and the closure evidence. Package: `20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`.
9. **openWEPP-native totalwatsed3 CLI + closure** *(WSHED01 complete
   2026-06-14 — the WBVAL06/6a deferral resolved)* — consume the closed MOFE
   hillslope outputs and close the end-to-end `totalwatsed3` water-balance audit
   on **openWEPP-native** output. The package began as a watershed-CLI route
   (W-A→W-D): W-A/W-B/W-C cleared the `jpond=0` no-impoundment parser defect
   (`IMP-E-004`/`CLIWAT-E-010`, the `IMP-E-007` count-mismatch split) and the
   over-strict WS10 zero-sediment/`nchnum=0` channel guards, reaching watershed
   output; W-D's audit then exposed that `totalwatsed3` is **hillslope-only**
   (no channel terms) and that the producer was filling `runvol` from WAT `Q`
   (a self-consistency check, not conservation). **Pivot (operator-directed):**
   two ADRs — [ADR-0019](../decisions/0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md)
   (openWEPP owns its output surface; `wepppyo3 wepp_interchange` frozen
   legacy-only) and [ADR-0020](../decisions/0020-totalwatsed3-dedicated-output-aggregation-cli.md)
   (totalwatsed3 is a dedicated `openwepp-cli-totalwatsed3`, an
   output-aggregation tier separate from the simulation binaries) — redirected
   the close to a native T-arc (T-A scope → T-B CLI → T-B2 native PASS `runvol`
   → T-C closure). The native `runvol` is sourced from the MOFE outlet-OFE
   routed runoff (the same surface the M-I hillslope-total identity closes on),
   genuinely independent of WAT `Q`. **Runvol took three iterations** —
   `QOFE·A_hillslope` over-scaled 2.5× (runoff > precip; caught by the closure),
   `Q·A_outlet` under-scaled ~4× (a crossed pairing that passed the one-sided
   `≤precip` bound and a self-restating test; caught by reconstructing the
   export from independent operands), and finally **`QOFE_outlet·A_outlet`**
   (≡ `Q·A_hillslope`). **Closure (`openwepp-cli-totalwatsed3` on the native
   arboreal-dendrite PASS/WAT):** `Σ runvol = 27.691 Mm³` (coeff 0.554), runoff
   < precip every year, independent of the WAT-`Q` column (18.895 Mm³); the
   `P − (Runoff + Lateral + ET + Perc + Interception) − ΔStorage` identity
   closes ex-day-1 at `−0.41 mm` over 2191 days with nonzero-at-noise daily
   residuals `[−0.248, +0.005] mm` (day-1 `+30.95 mm` is the storage-prepend
   init, present for any correct producer). Anchors byte-identical
   (`anchor_mismatches=0`); MOFE physics untouched (output-surface-only). The
   forensic record of the runvol arc is `artifacts/review-tb2-runvol-area-defect.md`;
   the durable geometry fact is agent memory `reference-qofe-q-area-duality`.
   Package: `20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/`.
   **Named follow-ons:** `WATERSHED-CHANWB-ROUTED-OUTPUT` (the decoupled
   channel-routing / `chanwb`/`chnwb` watershed output, distinct from the
   hillslope-only totalwatsed3 per ADR-0020 — the W-B/W-C watershed-CLI work
   landed there but channel water-balance routed output remains its own rung)
   and `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF` (sediment-coupled routing).
10. **snow physics-magnitude (Stage 2, deferred)** — the `snowd.for`
   melt/settling/density/partition equation adjudication against external authority
   (CRM Ch. 3.7, WEPP User Doc), behind the protected boundary. Distinct from snow
   *conservation* (Stage 1, item 6, done now); judged last against a fully closed,
   routed balance.

11. **MOFE >10-OFE far-point demonstration** *(FARPOINT01 complete 2026-06-16)* —
   `20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001/`.
   Demonstrated openWEPP's three-identity conservation closure on the **H2637
   19-OFE** substrate (in-repo wepp-forest provenance; legacy comparator
   `wepp_260606`), past the legacy ≤10-OFE ceiling.
   - **F-A** staged the fixture + clean legacy baseline (both `wepp_ui` variants);
     the openWEPP run **surfaced** a per-element WB13 fail-closed at OFE5 on a
     frost day (residual ≡ `watbtm`).
   - **F-B** *(Defect-Closure ExecPlan)* closed it **contract-first**: the frost
     bottom-overflow `watbtm` was double-counted (inflow frost adjustment **and**
     `Dp` outflow). SC-WATBAL-001 v161→v162 + `per_ofe_internal_wb13.rs:432` fix +
     regression; all four AGENTS gates green; H2637 both variants then run to
     completion (235,961 wat rows × 19 OFEs × 34 yr, exit 0). Commits
     `41469058`, `a724e2ae`.
   - **F-C** contrasted closure: legacy `wepp_ui` outlet runoff = **127.7 % of
     precip** (runoff > precip — the WB-05A q-cap, quantified) vs openWEPP **71 %**
     (bounded, `wepp_ui`-invariant, conservation-closed). Comparator a flag
     (ADR-0017); the 71 % vs 55.5 % magnitude gap → `MOFE-MAGPARITY01`.
   - **`watpdg`** branch-out **resolved**: instrumented detection found `watpdg>0`
     on 4 OFE-days with the gates still closing → it cancels on both sides →
     validated non-defect (no change). Commit `877ff25f`.
   Follow-ons: Stage-2 lateral/subsurface magnitude (`MAGPARITY01` completed the
   no-transfer-defect adjudication), `PERFHO01` (the ~80–110× high-OFE wall-clock
   gap — characterized, item 12), `WATERSHED-CHANWB-ROUTED-OUTPUT`,
   `MOFE-EROSION-QIN-QOUT-PARTICLE-HANDOFF`.

12. **High-OFE hillslope performance characterization** *(PERFHO01 complete
   2026-06-16, Codex-executed)* —
   `20260616-perf-high-ofe-hillslope-characterization-001/`. Attributed openWEPP's
   ~80–110× single-hillslope wall-clock gap vs legacy on H2637 (`978.55 s` vs
   ~10 s). CPU-bound (`977.99/978.55` user s) — **not** I/O/parquet; OFE-count
   scaling roughly linear-to-modestly-superlinear (`b≈1.12`), i.e. a large
   constant per-OFE-day cost. GDB-sampled dominant cost (perf blocked by
   `perf_event_paranoid`): per-OFE-day symbol-keyed `BTreeMap` runtime-surface
   churn + success-path writeback validation (`11/15` samples); the scaffold's
   WB13-string lead was tested and found **not** dominant. Verdict: not acceptable
   as-is → follow-on `PERFOPT01` (bit-identical, determinism-preserving;
   ~1.5–2.5× expected, 3.75× Amdahl cap — first step, not full closure). No
   production/contract edit. Claude review: sound and honest (15-sample limit +
   residual-gap caveats disclosed).

13. **Runtime-surface map-churn optimization** *(PERFOPT01 complete 2026-06-16,
   Codex-executed + Claude-reviewed)* —
   `20260616-perfopt01-runtime-surface-map-churn-001/`. Behavior-preserving
   optimization of the PERFHO01-named hot path: removed the per-OFE-day
   report-to-persistent-state + climate-overlay runtime-surface clones (move/extend
   not clone) and made kernel-writeback validation detail lazy. **~1.15×** on H2637
   (`978.55→849.86 s`; 10–18 % on the 1–5-OFE ladder), **bit-identical**
   (`anchor_mismatches = 0` across 7 fixtures — HBP byte + parquet table equality),
   determinism-preserving, all four gates green. Independent Claude review proved
   the lazy fast-path **exactly equivalent** to the original validation (inclusive
   bounds match `check_min`/`check_max`/`check_range`; inverted-bounds delegated,
   not suppressed) and re-confirmed bit-identity against a **separate** pre-opt
   baseline — resolving the no-independent-dual-review caveat Codex flagged.
   Residual → `PERFHO02` (now characterized). No contract/physics/output change.

14. **Post-PERFOPT high-OFE performance characterization** *(PERFHO02 complete
   2026-06-16, Codex-executed)* —
   `20260616-perfho02-post-perfopt-characterization-001/`. Characterized the
   post-PERFOPT01 H2637 residual with a 20-sample GDB window, then supplemented
   it with `perf record` after `kernel.perf_event_paranoid=0` became visible in
   the session. Dominant sampled cost:
   hydrology typed-symbol lookup, dynamic symbol formatting, frost/decomposition
   and PL guard work (`13/20`, 65 %). Secondary residual:
   `apply_kernel_writeback` sorting/allocation/insertion (`4/20`, 20 %).
   Scheduler/daily-loop insertion/allocation plus consumer-boundary validation
   accounted for the rest; `perf record` confirmed `execute_persistent_scheduler_kernel_lifecycle`
   at `96.24 %` children and `apply_kernel_writeback` at `12.46 %`; output
   writers were again absent. Follow-on:
   `PERFOPT02-symbol-access-and-writeback-application`. No production/contract
   edit.

15. **Indexed runtime-surface architecture design** *(PERFARCH01 complete
   2026-06-16, Codex-executed)* —
   `20260616-perfarch01-indexed-runtime-surface-design-001/`. Designed the
   architectural replacement for the string-keyed runtime surface: a frozen
   run-scoped `SymbolRegistry`, sorted-order `SymbolId`, and dense indexed
   state/flux storage while preserving the logical `BoundarySymbol` seam. A
   standalone prototype over 6,396 symbols measured 109.85× faster dense clone,
   219.16× faster pre-resolved lookup, and 115.77× faster update batches versus
   the modeled `BTreeMap<String, f64>` pattern; sorted id order matched string
   sort. Feasibility verdict: <=10× is plausible if implementation migrates
   roughly 89-90 % of elapsed time out of string-keyed surface mechanics; <=5×
   needs roughly 95-96 % and is not a storage-only promise. Proposed ADR:
   `docs/decisions/0022-indexed-runtime-surface-representation.md`. Follow-on:
   `PERFIDX01-run-scoped-symbol-registry-001`. No production/contract edit.

16. **Run-scoped symbol registry** *(PERFIDX01 complete 2026-06-16,
   Codex-executed)* —
   `20260616-perfidx01-run-scoped-symbol-registry-001/`. Implemented ADR-0022
   Stage 1: `SymbolId`, frozen sorted `SymbolRegistry`, BTreeMap export adapter,
   and an env-gated no-lazy-interning audit path. Completeness passed on H2637
   both UI variants plus OFE1-5 (`unknown_symbol_count = 0`); bit identity and
   determinism passed (`ANCHOR_MISMATCHES=0`, `DETERMINISM_MISMATCHES=0`).
   Runtime storage authority remains the existing BTreeMap surface. Follow-on:
   `PERFIDX02-indexed-shadow-runtime-surface-001`.

17. **Indexed shadow runtime surface** *(PERFIDX02 complete 2026-06-16,
   Codex-executed)* —
   `20260616-perfidx02-indexed-shadow-runtime-surface-001/`. Implemented
   ADR-0022 Stage 2: a sparse sorted `Vec<(SymbolId, BoundaryValue)>` shadow
   surface and an env-gated shadow report hook, while keeping BTreeMap storage
   authoritative. The tightened H2637 registry is 44,746 symbols, with 0
   unknown symbols on H2637 both UI variants plus OFE1-OFE5. H2637 sparse clone
   speedup measured 69.882x without UI and 54.096x with UI; shadow equality,
   bit identity, determinism, and full cargo gates passed. Follow-on:
   `PERFIDX03-indexed-surface-authority-001`.

Acceptance target at each rung is **closure** (does it conserve), not **magnitude**
(is the forcing physically right) and not comparator-match. See memory
`project-work-sequencing-wb-frost-mofe-snow` for the rationale and the two
ladder invariants (single-before-MOFE hard dependency; frost is per-column so
single-OFE fully settles it).

## Series index

Per-package execution logs are split by work-package series (newest first within
each). The narrative above is the live cross-cutting state; the docs below are the
archival per-package detail.

| Series | Head package | State | Log |
|---|---|---|---|
| HPHYS | `hphys0320` (2026-06-06) | snow/`RM` comparator arc **retired** per ADR-0017 — do not continue | [series/hphys.md](series/hphys.md) |
| WBVAL | `wbval06` (2026-06-06) | rung-1 single-OFE WB closure **complete** | [series/wbval.md](series/wbval.md) |
| SNOWSCI | `snowsci-stage1` (2026-06-06) | Stage 1 (conservation) **closed**; Stage 2 (magnitude) deferred | [series/snowsci.md](series/snowsci.md) |
| Governance / ADR | `adr0017` (2026-06-05) | comparator-distrust ratified | [series/governance.md](series/governance.md) |

**Frost (FROSTVAL / FQ / FDMC / FDHP):** the recent rung-2 frost packages are logged
inline in the active-work-sequence narrative above (items 7, 7a–7f), not in a
separate series doc.

**Other / historical series** (`auth`, `soilauth`, `infile`, `inspec`, `sci`,
`simimpl`, `wshedimpl`, `inimpl`, `arch`, `pl`, `clim`, `erod`, `wb`, `mofe`,
`refactor`, …): these predate this curated log or were never carried in it. Their
detail lives in each package's dated directory (`package.md` + `artifacts/`). They
are not summarized here; the canonical forward queue is
[../ROADMAP.md](../ROADMAP.md).

Initiative tracking convention inherited from wepp-palimpsest. Each work package lives in a dated directory under this tree.

## Directory naming
`YYYYMMDD-<short-slug>/`

## Required files
- `package.md` — scope, deliverables, dependencies, exit criteria
- `prompts/` — agent prompts (active and archived)
- `artifacts/` — produced docs, contracts, evidence

## Autonomous execution intent (required)
- A work package is an execution-ready plan, not a lightweight task note.
- Planning must be front-loaded into the package so execution can proceed
  autonomously from kickoff through disposition without user intervention.
- `package.md` and kickoff prompts must define concrete sequencing, explicit
  file targets, gate commands, and expected evidence updates.
- Kickoff prompts must include an explicit `Autonomy:` line requiring
  end-to-end execution for the declared scope without additional user
  intervention unless hard-blocked.
- Kickoff prompts default to `Execution mode: package-end-to-end` and should
  direct execution across all package phases through disposition.
- Single-phase kickoff prompts are exception-only and must declare
  `Execution mode: phase-only (exception)` plus explicit rationale and
  follow-on trigger.
- Kickoff prompts must include a `Required reading` list with explicit path
  references to orientation and authority documents so agents do not need to
  independently search onboarding context.
- Kickoff prompts must tier required-reading as `Core`, `Conditional`, and
  `On-demand` to preserve authority while minimizing unnecessary pre-read load.
- `Core` should remain small and stable (global governance + package-local
  authority). Put large mechanism-specific authorities in `On-demand` unless
  package scope requires them before edits.
- Each package should include `artifacts/required-reading-map.md` documenting:
  path, tier, rationale, applicability trigger, and when it was read.
- Kickoff prompts should record required-reading budget metrics for local-repo
  files, using canonical thresholds defined in
  `docs/standards/kernel-work-package-preparation.md`.
- When `REQUIRES-JUSTIFICATION` is reached, author must explain why each heavy
  pre-read is mandatory and cannot be deferred to `On-demand`.
- Work-package authoring must reference and follow:
  `docs/codex_exec_plans.md`.
- Mechanical refactor packages should additionally follow:
  `docs/standards/mechanical-refactor-authoring-guide.md`.

## Dual review and disposition (required)

- Every work package must include two independent review artifacts:
  `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`.
- Every review finding must be dispositioned as `accepted`, `rejected`,
  `deferred`, or `follow-up` before package closure.
- Accepted findings must be fixed and verified; rejected findings must include
  rationale; deferred/follow-up findings must be linked from
  `artifacts/disposition.md` and `artifacts/worker-handoff.md`.
- Dual verification artifacts must verify both technical gates and that no
  review findings remain undispositioned.

## Phase shape (inherited from wepp-palimpsest)
- **Phase 0**: docs-only audit / inventory
- **Phase 1**: architecture decision with operator-signed acceptance
- **Phase 2**: single-mechanism implementation, replay-and-checkpoint between mechanisms
- **Phase 3**: closeout disposition

## Conventions
- Dates are UTC.
- Evidence classification per claim: `[DIRECT]` (read source / contract / output) vs `[INFERENCE]` (reasoned from evidence).
- Evidence mode per assessment: **Static** (read and reasoned) vs **Ran** (commands actually invoked).
- Single-mechanism rule: one landed change per replay checkpoint.
- Correctness over completion: unresolved contract/invariant correctness gaps keep package disposition in `HOLD` until explicitly resolved or risk-accepted.
- Kernel-affecting packages (including runtime projection controlling kernel branches) must list:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  as dependencies, and must include a kernel-profile compliance checklist artifact.
- Code-authoring work packages should use contract-first sequencing when applicable:
  1. implement/ratify canonical contract amendments,
  2. implement contract-derived tests,
  3. record a pre-implementation contract gate, then
  4. modify production code.
- `package.md` dependencies for authored packages should include:
  - `/workdir/openWEPP/docs/codex_exec_plans.md`
- Missing kernel-profile/procedure compliance keeps disposition in `HOLD`.
