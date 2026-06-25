# Work Packages

> **Canonical roadmap: [../ROADMAP.md](../ROADMAP.md)** — a **forward-only planning
> queue** (what is next and deferred). The section below is this roadmap's
> **execution log**: the home for **completed** work — package status, detail, and
> commits. When a queue item closes it is removed from `ROADMAP.md` and recorded
> here. If the two disagree on what is next, `ROADMAP.md` wins.
> Scaffolded but unexecuted packages stay discoverable from `ROADMAP.md`, their
> package-local `package.md`, and the active/held package pointer below; they
> enter the execution log after closure.

## Current Active/Held Packages

State as of `2026-06-25`:

- `20260625-snowfrost-fidelity-g0-pysnobal-input-bridge-001/` is
  executed-held at `HOLD-PYSNOBAL-SANITY-FAILURE`. It added the diagnostic
  Rust `openwepp-snowbench export-pysnobal` exporter, reusing openWEPP WEPP
  parsing and SIMIMPL28 daily-to-hourly forcing surfaces, plus the thin
  `tools/snowfreeze_observed/pysnobal_compare.py` runner. Exporter schema,
  anti-alias, build, focused test, clippy, workspace test, and deny gates are
  green, and 14 of 15 all-site PySnobal lanes pass sanity. The Morris
  `Tg=-0.5 degC` lane aborts inside PySnobal C code despite finite exported
  forcing, so PySnobal comparator use is blocked until that lane has a
  minimal reproducer or explicit lane-policy disposition. PySnobal output
  remains diagnostic evidence only, not correctness authority or production
  physics.
- `20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/` is
  complete as the legacy snow-depth comparator/output-capture assessment. It
  added `tools/snowfreeze_observed/legacy_snow_compare.py`, proved pinned
  legacy WAT `Snow-Water` is SWE only, captured dated legacy physical snow
  depth from daily-winter hour-24 rows, and retained large graphics
  `treal(73)=snodpy*1000` / `treal(75)=densg` only as sparse operand
  provenance. Fresh all-site evidence found legacy closer by mean absolute
  observed-depth residual on Sleepers South and Morris, current openWEPP closer
  on Sleepers W9, and both models failing snow-depth control on all three
  paired-snow sites. Current openWEPP SWE is close to legacy SWE on common
  dates, so the next route remains snow-depth producer/carry/input/settlement
  adjudication, not legacy bit-parity or frost heat-flow/frozen-K tuning.
- `20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/` is
  complete as the SNOWFROST-FIDELITY-E correspondence and direction audit. It
  added `SC-SNOWFREEZE-001#INV-SNOWFREEZE-048`, extended the observed harness
  with signed snow-depth residuals, depth-vs-SWE anti-alias evidence, and
  adjacent-day timing/stage checks, and added
  `tools/snowfreeze_observed/snow_depth_audit.py`. Fresh all-site evidence
  routes Sites 1, 2, and 4 to `SNOW-DEPTH-FIDELITY-ISSUE` with dominant
  modeled-over-observed snow depth; Sites 3 and 5 remain insufficient for
  snow-control because they lack paired observed snow-depth rows. No frost
  heat-flow, frozen-K/SFCC, impedance, or `Qwet` work is authorized from these
  residuals; the next route is snow-depth producer/carry/input/settlement
  adjudication.
- `20260625-snowfrost-fidelity-c-sfcc-frozen-k-diagnostics-001/` is
  complete as diagnostic-only SFCC/frozen-conductivity comparison tooling. It
  added `tools/snowfreeze_observed/frozen_k_diagnostics.py`, a deterministic
  JSON/Markdown diagnostic surface for Clapeyron/SFCC liquid water,
  SFCC-Mualem frozen conductivity, Watanabe/Flury-style capillary-bundle
  screening, Cheng-style impedance scaling, and Amankwah-style salinity
  sensitivity. The focused contract proves bounded/monotonic curves,
  non-production labels, salinity sensitivity, and no production `crates/`
  coupling. It does not select texture defaults, authorize `Qwet`, tune field
  residuals, change production runtime physics, or promote direct activation.
- `20260625-snowfrost-fidelity-b-no-qwet-heatflow-benchmarks-001/` is
  complete as benchmark-only no-migration heat-flow closure. It added CLIM06
  `snowfrost_b_*` gates for a Kurylyk/Stefan-style one-dimensional freezing
  bound, independent surface resistance reconstruction, snow/residue
  insulation, lower-front dry heat, and latent-energy-bounded fine-layer
  mutation. Production `crates/` still contain no `qwet`, `Qwet`, or `frzftp`
  implementation. The package does not validate field frost-depth fidelity or
  authorize physics tuning; modeled snow depth exposure and diagnostic
  SFCC/frozen-K candidate work remain follow-on scope.
- `20260625-snowfrost-fidelity-a-observation-residual-classification-001/` is
  complete as the first Snow and Frost Fidelity Adjudication package. It ran
  all five observed frost-depth pilot sites through the direct observed
  harness, added `tools/snowfreeze_observed/classify_residuals.py`, and
  classified zero sites as eligible for frost-model defect attribution. Sites
  1/2/4 are `SNOW-CONTROL-BLOCKED`; sites 3/5 are `INCONCLUSIVE`. The next
  field-validation step must expose modeled snow depth and rerun classification
  before tuning heat flow, frozen conductivity, SFCC/impedance, or migration
  heat.
- `20260625-snowfreeze-frost-depth-literature-annotation-001/` is complete.
  It annotated the newly available frost-depth physics literature, classified
  the two CC-BY vendorable PDFs (`Amico2011.pdf`, `Devoie2022.pdf`), updated
  the reference bibliography, and recorded the physics ladder for
  `GAP-SNOWFREEZE-002`: observation harness first, snow-insulation attribution,
  no-`Qwet` heat-flow baseline, then bounded SFCC/frozen-K/impedance candidate
  evaluation before any migration-heat promotion.
- `20260624-snowfreeze-direct-storage-reconciliation-unblock-001/` is complete
  as the DC successor to the observed frost-depth harness. It owns
  `SNOWFREEZE-DRSTOR-001` and `SNOWFREEZE-DRSTOR-002`: site3/site4 direct
  observed comparisons failed before exit-0 metric-bearing comparison at the
  R4B explicit frost storage projection nonnegative guard. The package closed
  the in-envelope projection defect without frost physics tuning,
  observation-threshold changes, default activation, or compatibility-runtime
  deletion; site3 and site4 now emit metric-bearing `UNRESOLVED` observation
  reports.
- `20260624-snowfreeze-observed-frost-depth-harness-001/` is complete. It
  acquired/normalized the pilot historic frost-depth observation corpus, added
  the local comparison harness under `tools/snowfreeze_observed/`, and installed
  local contract coverage for the corpus and harness. Direct remains opt-in;
  compatibility frost bit-parity is not the target. The site3/site4
  metric-bearing comparison blocker was lifted by the direct storage
  reconciliation package above.
- `20260624-r7h-iterative-completion-001/` is closed `OPT-IN` by operator
  decision on 2026-06-24. It retained the R7H direct performance fix and
  zero-compatibility evidence, left direct mode opt-in, and reclassified the
  remaining typed-frost vs compatibility divergence under reopened
  `GAP-SNOWFREEZE-002`. Do not resume R7H frost-vs-compatibility bit-parity
  from this package; the successor work is a frost-depth fidelity DC validated
  against historic observations.
- `20260624-r7h-closure-activation-gates-001/` is executed-held at
  `HOLD-R7H-H2637-DIRECT-PERFORMANCE-AND-PROTECTED-PARITY`. It executed the
  ADR-0026 "Closure and activation gates" step after consumer cutover/deletion,
  fixed two in-envelope no-material frost closure failures exposed by full
  H2637 direct default-candidate runs, and advanced current direct H2637 to
  endpoint with `compatibility_edge_invocations = 0`. Activation remains
  blocked: the current direct endpoint is `113.53 s / 1083636 KiB` against the
  `91.2 s` `<=10x` budget, and protected-output parity is still red/not
  current-matrix green. Default activation remains disabled.
- `20260624-r7g-consumer-cutover-deletion-001/` is complete. It executed the
  ADR-0026 "Consumer cutover and deletion" step after typed frost solver
  extraction by moving R4A/direct-publication consumers from the temporary
  `DirectFrostRunoffSurface` / `DirectFrostLiquidPartition` bridge to typed
  winter-column state and outcomes, deleting the production bridge fields/API,
  and preserving the R7B day-frame size bound by passing typed frost compute
  inputs as execution context instead of resident day-frame state. Required
  closure gates passed. It does not claim R7G terminal output parity, default
  activation, or performance closure.
- `20260624-r7g-typed-frost-solver-extraction-001/` is complete. It executed
  the ADR-0026 "Typed frost solver extraction" step by moving production direct
  active-frost partition computation from the `DirectFrostRunoffSurface` /
  `HillslopeKernelRequest` bridge to typed winter-column inputs and typed
  `Wb11HydrologyKernel` partition compute. Production direct day input no
  longer assigns a frost runoff surface, focused source scans prove the hot
  path avoids compatibility request/surface calls, and typed-vs-adapter parity
  fixtures pass for active and inactive/no-material cases. The residual
  compatibility surface remains allowed only as a named comparator seam until
  the later consumer cutover/deletion package.
- `20260624-r7g-frost-state-skeleton-comparator-seam-001/` is complete.
  It targets the ADR-0026 "Frost state skeleton and comparator seam" step by
  making `DirectWinterColumnState.frost` the canonical direct lane/day
  skeleton, leaving `DirectFrostRuntimeCarry` as a temporary mirror, and
  isolating the remaining `DirectFrostRunoffSurface` bridge behind named
  comparator-seam helpers. Focused tests, source scans, workspace Rust gates,
  dependency policy, and scoped Markdown lint passed. It does not claim typed
  frost solver extraction, bridge deletion, output parity, default activation,
  or R7G closure.
- `20260623-r7g-snow-lane-migration-001/` is complete. It executed the
  ADR-0026 "Snow lane migration" step by making
  `DirectWinterColumnState.snow` the production direct lane/day authority for
  seeded snow state, R4G same-day snow mutation, and direct publication
  snow/frost prior-snow reads. `DirectSnowRuntimeCarry` remains only as a
  temporary direct-runtime compatibility mirror for residual frame surfaces.
  The package added focused lifecycle/source-scan tests and passed the required
  Rust closure gates. It does not claim frost subsolver migration, output
  parity closure, performance closure, default activation, or deletion of
  residual carry surfaces.
- `20260623-r7g-winter-column-mechanical-containment-001/` is complete. It
  installed the ADR-0026 winter-column containment boundary outside
  `direct_runtime`, added inert boxed `DirectWinterColumnState` ownership hooks
  to direct lane/day frames, reused existing runtime-input hourly winter forcing
  authority, proved the new module has no compatibility request/symbol
  authority, split the oversized direct-publication day-input helper into
  sub-3000-line chunks, and fixed the active-frost no-freeze hourly diagnostic
  gate blocker exposed by workspace validation. It does not claim solver
  migration, publication parity, performance closure, default activation, or
  R7G closure.
- `20260623-r7g-iterative-completion-001/` is executing against
  `R7G-006-FROST-SNOW-PROJECTION-PARITY-RESIDUALS`. It lifted the inherited
  surface-free active-snow hold by making sidecar-only snow inactive, adding
  typed active-snow partition authority, persistent snow carry, snow liquid
  event routing, and the same-day EROD14 qout handoff fix. It also installed
  production active-frost execution enough for full H2637 direct default to
  reach endpoints with `compatibility_edge_invocations = 0`; active-frost
  performance was reduced from `163.88 s` to retained timing `89.88 s`, under
  the `91.2 s` `<=10x` legacy budget. R7G remains incomplete because protected
  HBP/WAT/PASS parity is red: WAT `frozwt`/`frdp` still differ on `34363` rows,
  and `Snow-Water`/`RM` residuals are material. The current continuation must
  reduce frost/snow projection parity while preserving zero compatibility
  counters and the green performance gate, or prove a narrower legitimate
  out-of-envelope boundary after attempted in-envelope correction.
- `20260623-r7g-performance-closure-fixture-hardening-001/` is executed-held
  at
  `HOLD-R7G-SURFACE-FREE-ACTIVE-SNOW-PARTITION-AUTHORITY-ABSENT`. It ran the
  same-binary H2637 matrix after R7F: default-disabled compatibility passed at
  `645.51 s / 229560 KiB`, rollback compatibility passed at
  `637.10 s / 229016 KiB`, and the two compatibility modes had identical
  protected output checksum maps. Direct default candidate and explicit direct
  both failed closed before endpoint timing because production direct lacks
  typed, surface-free active snow partition authority for lane 1. The follow-up
  must add typed active snow state/partition compute, downstream operands,
  shadow projection, publication operands, fixtures, no-compatibility scans,
  and helper line-count remediation before R7G can rerun.
- `20260623-r7f-direct-day-input-hot-loop-isolation-001/` is complete. It
  closed
  `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE` by
  replacing the production direct interleaved day-input builder hot-loop
  dependency with typed direct day-input/state projection. Production direct
  manifests now report `compatibility_edge_invocations = 0` because the
  hot-loop edge is removed, not because accounting missed it. Focused R7/R6
  suites, workspace clippy/tests, `cargo deny check`, `cargo fmt --check`,
  `git diff --check`, and scoped Markdown lint passed.
- `20260623-r7e-r7h-direct-runtime-completion-001/` is executed-held at
  `HOLD-R7F-DIRECT-DAY-INPUT-BUILDER-COMPATIBILITY-SURFACE-HOT-EDGE`. It
  closed R7E default-candidate/rollback selection mechanics and made runtime
  selection manifest-visible, then corrected the direct runtime audit so the
  production direct interleaved day-input builder is counted as a compatibility
  edge. R7F-R7H remain blocked until that builder is replaced by typed direct
  day-input/state projection and no-compatibility counters return to zero for
  real.
- `20260623-r7d8-direct-hbp-erod15-export-alias-parity-001/` is complete. It
  lifted `HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP`
  by carrying producer-authoritative direct EROD15 sediment export aliases
  into HBP publication without compatibility runtime wrapping. Fresh H2637
  5-day evidence under `/tmp/r7d8ad-h2637-5day` has default/direct exits `0`,
  HBP/loss/PASS/PLOT/WAT byte identity, parsed HBP latest-event parity, and
  direct `compatibility_edge_invocations = 0`.
- `20260623-r7d7-direct-wb16-peak-publication-parity-001/` is executed-held
  with final disposition
  `HOLD-R7D7-HBP-EROD15-SEDIMENT-EXPORT-ALIASES-DIRECT-PRODUCER-GAP`. It
  closed the R7D6 PASS `peakro` residual by making compatibility PASS consume
  runtime `peakro` and direct PASS consume direct runoff peak authority before
  the erosion copy. Fresh H2637 5-day WAT and PASS outputs are byte-identical
  with direct `compatibility_edge_invocations = 0`; HBP now differs only on
  EROD15 sediment export aliases (`total_detachment_kg = 0.6` and
  `sediment_concentration_kg_m3 = 6.816136920064195` in compatibility versus
  direct zeros). R7D8 lifted this hold for the current H2637 5-day gate.
- `20260623-r7d6-direct-erod13-erod14-typed-producer-001/` is executed-held
  with final disposition
  `HOLD-R7D6-PASS-HBP-PEAKRO-COMPATIBILITY-ZERO-RESIDUAL`. It lifted
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT` by adding
  typed direct EROD13/EROD14/EROD15 producer authority and the direct WB16
  peak-duration producer required by active erosion publication. H2637 direct
  production exits `0` with `compatibility_edge_invocations = 0`; WAT is
  byte-identical and PASS sediment fields are parity-clean. Its PASS/HBP
  `peakro` hold was narrowed by R7D7, and the remaining HBP sediment export
  alias hold was lifted by R7D8.
- `20260623-r7d5-direct-erod14-sediment-publication-001/` is executed-held
  with final disposition
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT`. It proved
  direct production had no direct sediment producer and replaced the silent
  zero active-sediment publication path with a fail-closed guard when
  `erod14_wave2_enabled` is true. Focused H2637 direct production now exits
  `1` at `R7D5 direct EROD14/EROD15 sediment producer must execute before this
  span`; R7D HBP/PASS sediment parity remains blocked on the queued R7D6 typed
  producer implementation.
- `20260622-r7d4-direct-mofe-dynamic-carry-transfer-001/` is executed-held
  with final disposition
  `HOLD-R7D4-HBP-EROD14-SEDIMENT-PRODUCER-ABSENT`. It lifted
  `HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT` by copying current-lane
  R4O/R4L `ui_LfCrf`/`ui_SCrunf` arrays forward into downstream typed transfer
  buffers and making R3A/R4J consume them with area-ratio provenance. Focused
  H2637 evidence now has byte-identical WAT and PASS, and loss/plot differ
  only by `run_name`. HBP remains held because default contains nonzero
  sediment concentration/detachment/deposition payload bytes where direct still
  publishes zero erosion authority.
- `20260622-r7d3-direct-wb14-r4k-infiltration-producer-001/` is executed-held
  with final disposition
  `HOLD-R7D3-DIRECT-MOFE-DYNAMIC-CARRY-TRANSFER-ABSENT`. It lifted
  `HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT` by
  implementing typed direct WB14/R4K infiltration/depression producer
  authority, wiring same-pass infiltration into R4A/WB18/ET/publication, and
  adding R4L direct hourly saturation addback from R4O carry arrays. H2637
  direct production exits 0 with `compatibility_edge_invocations=0`; the
  remaining dynamic transfer blocker was subsequently lifted by R7D4.
- `20260622-r7d2-multiofe-lane-seed-authority-001/` is executed-held with final
  disposition
  `HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT`. It
  lifted `HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT` by replacing
  production direct topology/area-only lane seeds and single aggregate
  runtime-surface day-input profile authority with lane-indexed constructor and
  day-input seed/profile authority. Focused one-OFE HBP/loss/PASS/WAT parity
  remains green, and H2637 direct production improved to
  `182.83 s / 627436 KiB`, but HBP/PASS/WAT parity remains blocked because
  direct R4K has no baseline-authoritative WB14 infiltration/depression
  producer, so R4A still computes runoff as liquid input.
- `20260622-r7d-direct-publication-producer-authority-001/` is executed-held
  with final disposition
  `HOLD-R7D-MULTIOFE-DIRECT-LANE-SEED-AUTHORITY-ABSENT`. It proved the
  production direct consumer path writes from `DirectRunPublicationFrame` and
  not `execution.wb13_rows`, and the focused one-OFE fixture is parity-clean.
  H2637 remains non-parity for HBP, WAT, and PASS because production direct
  still constructs topology/area-only lane frames and seeds day inputs from a
  single aggregate runtime surface instead of lane-indexed typed direct
  constructor authority.
- `20260622-r7c-production-direct-executor-path-001/` is complete with final
  disposition `COMPLETE-R7C-PRODUCTION-DIRECT-EXECUTOR-PATH`. It added the
  opt-in production direct executor route from parsed typed constructor state
  into `DirectFrameExecutor`, with manifest-visible direct counters,
  no-compatibility-edge proof, default-mode preservation, and R6J cutover
  preservation. Same-binary H2637 evidence recorded default compatibility
  `642.77 s / 228804 KB` and direct production `753.76 s / 625132 KB`;
  direct production is not performance-ready, and HBP/PASS/WAT output parity
  remains R7D scope.
- `20260622-r7b-parsed-input-typed-frame-constructors-001/` is complete with
  final disposition `COMPLETE-R7B-PARSED-INPUT-TYPED-FRAME-CONSTRUCTORS`. It
  added the R7B parsed-input typed constructor boundary for `DirectRunFrame`,
  `DirectLaneFrame`, and `DirectDayFrame`, with constructor fixtures, static
  no-compatibility scans, type-size/layout evidence, package-local
  review/verification artifacts, and full Rust closure gates. It did not
  activate production direct mode or change output schemas.
- `20260622-r7a-architecture-state-reconciliation-001/` is complete with final
  disposition `COMPLETE-R7A-ARCHITECTURE-STATE-RECONCILIATION`. It reconciled
  the array-native runtime architecture authority with the post-R6J state:
  PERFDEEP09 is recorded as the PERFDEEP07 hold-lift authority, R2-R5 are
  direct-runtime scaffold and phase-coverage evidence, R6J is explicitly
  opt-in direct publication cutover rather than default activation or full
  runtime completion, and the spec now includes a mode-state matrix separating
  compatibility, shadow, direct publication cutover, and future production
  direct mode. ADR-0025 now points to Revision 3 and the R7A-R7H burndown
  sequence. Scoped Markdown lint and `git diff --check` passed.
- `20260622-direct-runtime-section-split-001/` is complete with final
  disposition `COMPLETE-DIRECT-RUNTIME-SECTION-SPLIT`. It mechanically split
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` into ordered
  included section files under `src/direct_runtime/`, reducing the root module
  from `2922` lines to `210` lines. The new sections are `1001`, `454`, `433`,
  `391`, and `434` lines; all production direct-runtime `.rs` files are below
  `2000` lines. The crate-level direct-runtime export remains unchanged, the
  direct-runtime source-scan test now reads the included files, and Rust
  closure gates, `cargo deny check`, `git diff --check`, and scoped Markdown
  lint passed.
- `20260622-runner-intake-lane-setup-mechanical-split-001/` is complete with
  final disposition
  `COMPLETE-RUNNER-INTAKE-LANE-SETUP-MECHANICAL-SPLIT`. It mechanically split
  the execution/output/manifest and public runner-entrypoint tail out of
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  into `05_runner_execution_and_outputs.rs`, leaving the original setup file at
  `1741` lines and the new execution/output section at `1255` lines. Public
  runner exports remain unchanged; static MOFE source-scan tests now inspect
  both included runner files; Rust closure gates, `cargo deny check`,
  `git diff --check`, and scoped Markdown lint passed.
- `20260621-r6j-direct-publication-cutover-blocker-closure-001/` is complete
  with final disposition `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER`. It closed
  the inherited manifest writer blocker, run-local direct counter provenance,
  direct/shadow manifest selection, current-fixture HBP/WAT/PASS/loss parity,
  direct-only production cutover writing, checksum/readback evidence, H2637
  public-output parity, and default-disabled isolation. Fresh same-binary H2637
  evidence: default `640.41 s / 227396 KiB`; direct cutover
  `637.53 s / 349400 KiB`; HBP/WAT/PASS/loss/plot byte identity; WAT
  `235961` rows and PASS `12419` rows with zero bidirectional DuckDB
  differences; manifest source `direct-publication-frame`; all direct runtime
  counters, including `compatibility_edge_invocations`, are `0`.
- `20260621-r6h-direct-pmet-day-state-carry-builder-001/` is executed-held at
  `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`. It cleared
  `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` by replacing the
  precomputed multi-day PMET publication input vector with an interleaved
  direct day/lane builder. Current-fixture HBP byte identity remains green,
  WAT storage totals are bit-identical, and the remaining current-fixture WAT
  residual is exactly day-2 `Es` at ulp-scale PMET layer carry.
- `20260621-r6g-direct-wat-producer-authority-001/` is executed-held at
  `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`. It reduced the R6F WAT
  producer-authority gap for the inherited current fixture: `wepp_id`,
  simulation `year`, profile fields, first-day `Es`, and first-day storage no
  longer reduce after parsed typed direct producers and residual-inclusive
  direct storage projection were bound. Current-fixture HBP identity remains
  green and the first WAT row is bit-identical. Canonical multi-OFE WAT id
  authority and lane-dimensional day inputs remain follow-up. The remaining
  WAT mismatch is exactly day-2 `Es`, `Total-Soil`, and
  `SoilWaterTotal`, blocked on an interleaved PMET day-input builder that uses
  direct-carried layer state after the previous direct day commit.
- `20260621-r6f-direct-publication-cutover-blocker-closure-001/` is
  executed-held at
  `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`. It closed the inherited
  HBP byte mismatch on the current near-zero runoff fixture by publishing
  direct `peakro`/`watdur` operands, fixed the direct climate precipitation
  unit projection, proved current-fixture HBP byte identity, reduced WAT to
  exact field deltas, added typed direct process input slots, profile
  projection fields, and lane-carried layer state, and scaffolded R6G for the
  remaining production parsed-input producer authority gap. Full R6 HBP closure
  still requires nonzero peak-runoff/event-duration fixture coverage.
- `20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/` is
  executed-held at
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`. It reproduced the R6D
  fail-closed cutover state, split the direct-publication helper block out of
  the over-3000-line runner file, resolved production direct-runtime
  input-binding for parsed climate by building retained cutover publication
  through direct capture, and moved the first blocker to HBP direct process
  parity. Complete R6 closure still requires real
  `DirectPublicationFrameCutover`: HBP byte identity, WAT/PASS Arrow
  row/schema/metadata parity, loss JSON identity, manifest provenance/checksum
  parity, anti-alias fixtures, independent reconstruction, no compatibility
  authority, successful direct output writes, and default-disabled isolation.
- `20260621-r6d-production-direct-publication-producer-retention-001/` is
  executed-held at
  `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`. It lifted the R6C
  retained-producer absence for cutover by adding a cutover-only retained
  `DirectRunPublicationFrame` producer surface to the production climate
  lifecycle. The retained frame uses parsed climate/calendar and slope geometry
  and is consumed by `DirectPublicationFrameCutover` without skeleton direct
  frame construction or post-hoc publication capture. Public direct-output
  writes remain fail-closed until parity-grade hydrology/storage/subsurface/
  evaporation/PASS/loss/manifest/erosion producers, anti-alias fixtures,
  independent reconstruction, manifest provenance, and line-count governance
  are closed.
- `20260621-r6c-direct-publication-typed-operand-bridge-001/` is
  executed-held at
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`. It corrected the R6B
  candidate so `DirectPublicationFrameCutover` fails before skeleton
  direct-frame construction/publication capture, proving the remaining blocker
  is not at the output helper but at the production climate lifecycle: retained
  direct run/lane/day publication producers do not yet exist.
- `20260621-r6b-direct-publication-parity-manifest-cutover-001/` is
  executed-held at
  `HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`. It confirmed the
  R6 cutover candidate still builds from a skeleton direct frame with
  zero/absent publication operands, retained a fail-closed diagnostic marker,
  and left anti-alias, reconstruction, manifest, output-family, default-disabled,
  and endpoint/RSS gates blocked behind the missing typed operand bridge.
- `20260621-r6-direct-publication-cutover-001/` resumed after R5E completed at
  pushed commit `d8f6bbea`. It promoted the PERFDEEP06 publication operand
  ledger into canonical architecture authority, held until R6A supplied the
  run-bound direct publication frame and direct projection consumers, then
  resumed with a guarded `DirectPublicationFrameCutover` candidate. The
  candidate fails closed at HBP byte identity and still needs manifest
  provenance cutover; wrapping compatibility WB13 rows or runtime surfaces in
  direct-named structures is not a valid cutover.

## Current roadmap execution log

State as of `2026-06-21`:

- R6G (`20260621-r6g-direct-wat-producer-authority-001/`) is executed-held
  with final disposition
  `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`. The package bound direct
  WAT identity/year/profile producers, added parsed static/climate direct
  day-input translation for WB11/WB17/WB18/WB19 operands, preserved lane-carried
  direct layer state for later days, corrected direct projection storage to
  include residual liquid water, and proved current-fixture HBP byte identity
  plus first WAT row parity. The identity/year/profile work is current-fixture
  parity evidence; canonical multi-OFE WAT id authority, lane-dimensional day
  inputs, and allowlisted symbol lineage remain follow-up. It stopped because
  day-2 PMET `Es` and dependent storage still use precomputed component inputs
  built before the prior direct day commits carried layer state. The next R6
  package must implement dynamic interleaved PMET day-input construction rather
  than reading compatibility WB13 rows or runtime surfaces.
- R6H (`20260621-r6h-direct-pmet-day-state-carry-builder-001/`) is
  executed-held with final disposition
  `HOLD-R6H-WAT-PMET-LAYER-CARRY-ULP-PARITY`. The package replaced the
  precomputed PMET publication input vector with an interleaved direct day/lane
  builder, added fail-closed later-day layer requirements, preserved
  current-fixture HBP byte identity, and reduced WAT from day-2 `Es`,
  `Total-Soil`, and `SoilWaterTotal` to `Es` only. R6I subsequently closed
  this `Es` parity blocker; broader multi-OFE output authority remains later
  R6 scope.
- R6I (`20260621-r6i-direct-pmet-layer-ulp-parity-001/`) is complete with
  verdict `COMPLETE-R6I-DIRECT-PMET-LAYER-ULP-PARITY`. The package localized
  the day-2 PMET `Es` residual to direct lane commit carrying the WB17
  post-root-uptake layer vector instead of the post-runoff-reconciliation
  active-frost fine-layer topology projection. It added typed
  `DirectFrostLayerCarryProjection`, builds it from direct seed-surface frost
  options and layer geometry, applies it during lane commit, and proved
  current-fixture HBP byte identity plus WAT identity with no R6G/R6H marker.
  `DirectPublicationFrameCutover` still fails closed before public writes, now
  at manifest writer wiring: `manifest direct projection is not wired to the
  production manifest writer`.
- R6F (`20260621-r6f-direct-publication-cutover-blocker-closure-001/`) is
  executed-held with final disposition
  `HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`. The package closed the
  R6E HBP byte mismatch on the current near-zero runoff fixture, proved
  current-fixture HBP identity on the CLI fixture, corrected the direct climate
  precipitation unit projection, and reduced the next blocker to WAT `wepp_id`,
  simulation `year`, `Es`, storage, and profile fields. It added typed direct
  process input slots, lane-carried subsurface layer state, and profile
  depth/porosity projection fields, then stopped at the remaining production
  parsed-input producer authority boundary rather than wrapping compatibility
  WB13 rows or runtime surfaces as direct authority. R6G is scaffolded to close
  that boundary; nonzero HBP fixture coverage remains a later R6 gate.
- R6E (`20260621-r6e-direct-publication-cutover-iterative-defect-closure-001/`)
  is executed-held with final disposition
  `HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`. The package reproduced the
  cutover failure, preserved fail-closed no-output behavior, moved
  direct-publication helpers into `04_direct_publication.rs`, reduced
  `00_runner_intake_and_lane_setup.rs` to `2787` lines, added typed
  `DirectPublicationDayInput` binding for parsed climate, and changed retained
  cutover publication to a full direct executor capture. It did not complete R6
  direct publication cutover; the next package must close contract-backed HBP
  direct process parity before WAT/PASS/loss/manifest parity work can honestly
  resume.
- R6D (`20260621-r6d-production-direct-publication-producer-retention-001/`) is
  executed-held with final disposition
  `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`. The package added
  cutover-only retained direct publication rows to production climate execution,
  sourced from parsed climate/calendar, parsed slope geometry, and run/lane/day
  identity. The cutover branch now consumes that retained frame and no longer
  runs skeleton direct-frame publication capture. Focused runner and CLI tests
  pass and prove fail-closed no-output behavior. The next package must add
  parity-grade retained publication producers for hydrology/storage/subsurface/
  evaporation/PASS/loss/manifest/erosion families and split the monolithic
  runner helper surface before full R6 closure.
- R6C (`20260621-r6c-direct-publication-typed-operand-bridge-001/`) is
  executed-held with final disposition
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`. The package did not
  close R6 parity/manifest cutover. It changed the opt-in
  `DirectPublicationFrameCutover` path so it refuses to construct a skeleton
  direct publication frame when production direct publication producers are not
  retained. Focused runner and CLI tests pass and confirm no direct frame,
  executor, skeleton, publication capture, compatibility-edge invocation, or
  public output write occurs on the failure path. The next package must add a
  retained production direct publication producer surface to the climate
  lifecycle before parity, anti-alias, reconstruction, manifest, and benchmark
  gates can close.
- R6A (`20260621-r6a-run-bound-direct-publication-frame-001/`) is complete with
  verdict `COMPLETE-R6A-RUN-BOUND-DIRECT-PUBLICATION-FRAME`. It added
  `DirectRunPublicationFrame`, direct run/lane/day capture, the
  `DirectPublicationFrameShadow` opt-in, and direct HBP/WAT/PASS/loss/manifest
  projection consumers. Full production writer cutover remains R6 scope.
- R5E is complete with verdict
  `COMPLETE-R5E-FULL-OFE-DAY-ENDPOINT-READINESS`. The package closed R5 by
  proving the direct executor records exactly one canonical 14-phase entry per
  OFE-day in `DirectPhaseKind::ORDERED`, with R4/R5 direct spans folded under
  canonical phase entries as sub-operation counters rather than duplicate phase
  executions. R5E added explicit canonical phase-entry reporting, a focused
  endpoint-readiness test, and expanded no-compatibility source-scan coverage
  to include direct growth runtime code. Public outputs remain
  compatibility-authoritative: no HBP/WAT/PASS/loss/manifest cutover, schema
  change, default activation, or direct-only public-output endpoint occurred.
  Full Rust gates passed. Final default-disabled H2637 reps were `641.37 s`,
  `642.02 s`, and `635.47 s` (median `641.37 s`, threshold `<= 676.67 s`);
  the opt-in direct-skeleton endpoint ran at `638.33 s`. Protected output
  comparison passed through HBP/WAT/loss/plot byte identity and PASS DuckDB row
  equivalence (`12419` rows, zero bidirectional differences). Package:
  `20260621-r5e-full-ofe-day-endpoint-readiness-001/`. Pushed commit:
  `d8f6bbea`.
- R5D is complete with verdict
  `COMPLETE-R5D-GROWTH-TRANSITION-DIRECT-PHASES`. The package promoted direct
  `AnnualGrowthTransition` and `PerennialGrowthTransition` executor calls,
  adding typed inputs, direct compute, direct state mutation, downstream
  operands, and shadow projection for both phases. The direct growth state
  covers cumulative GDD, biomass, canopy cover, LAI, root mass, root depth,
  harvest index, climate/stress inputs, and legacy `gddmax` sentinel
  resolution. R5D also added an R4N direct-runtime guard so ET/root uptake can
  fail closed when required growth context is absent. Public outputs remain
  compatibility-authoritative: no WB13 ET, WAT plant metadata, PASS, loss,
  manifest, scheduler, runner API, default activation, or endpoint cutover
  occurred. Full Rust gates passed. Final default-disabled H2637 reps were
  `647.54 s`, `647.93 s`, and `644.88 s` (median `647.54 s`, threshold
  `<= 676.67 s`) with protected output comparison passing through HBP/WAT byte
  identity, PASS DuckDB row equivalence, and run-name-only normalized loss/plot
  differences. Package:
  `20260620-r5d-growth-transition-direct-phases-001/`. Pushed commit:
  `2fbd3802`.
- R5C is complete with verdict
  `COMPLETE-R5C-DECOMPOSITION-RESIDUE-DIRECT-TRANSITIONS`. The package
  promoted direct `DecompositionTransition` and `ResiduePartitionTransition`
  executor calls, adding typed inputs, direct compute, direct state mutation,
  downstream operands, and shadow projection for both phases. R5C implements
  the `SC-RESIDUE-001` PL17 tracked seed-pool update over typed active
  decomposition context, validates missing/ambiguous context and invalid
  residue/fraction/rate domains as typed hard failures, and projects typed
  residue partition operands without public-output cutover. Full Rust gates
  passed. Final default-disabled H2637 reps were `639.05 s`, `646.33 s`, and
  `643.96 s` (median `643.96 s`, threshold `<= 676.67 s`) with protected
  output comparison passing through HBP/WAT byte identity, PASS DuckDB row
  equivalence, and run-name-only normalized loss/plot differences. Package:
  `20260620-r5c-decomposition-residue-direct-transitions-001/`. Pushed commit:
  `efdf6710`.
- R5B is complete with verdict
  `COMPLETE-R5B-NORMALIZATION-STORAGE-BOUNDS-DIRECT-PHASES`. The package
  promoted direct `Normalization` and `StorageBounds` executor calls, adding
  typed inputs, direct compute, direct state mutation, downstream operands, and
  shadow projection for both phases. `StorageBounds` now reports as `Executed`
  in lifecycle status counts; decomposition, residue, annual growth, and
  perennial growth remain explicit `Hold` phases for R5C-D. R5B validates the
  scalar storage/domain state available in the direct frame and does not claim
  layer-capacity physics, public output cutover, scheduler changes, or default
  activation. Full Rust gates passed. Final default-disabled H2637 reps were
  `643.38 s`, `640.54 s`, and `644.59 s` (median `643.38 s`, threshold
  `<= 676.67 s`) with protected output comparison passing through HBP/WAT byte
  identity, PASS DuckDB row equivalence, and run-name-only normalized
  loss/plot differences. Package:
  `20260620-r5b-normalization-storage-bounds-direct-phases-001/`. Pushed
  commit: `27de814c`.
- R5A is complete with verdict
  `COMPLETE-R5A-FULL-DAY-DIRECT-EXECUTOR-LIFECYCLE`. The package widened the
  direct executor from day-0 lane skeleton execution to all
  `day_count * lane_count` direct day frames, added persistent lane-state
  handoff into each day frame, added end-of-day lane commits, recorded
  day-frame commit counters, and exposed canonical phase status counts. R5A
  keeps the five non-hydrology phases reserved for R5B-D as explicit `Hold`
  statuses and leaves public output authority compatibility-owned. Full Rust
  gates passed. Final default-disabled H2637 reps were `643.98 s`,
  `647.95 s`, and `643.45 s` (median `643.98 s`, threshold `<= 676.67 s`) with
  protected output comparison passing through HBP/WAT byte identity, PASS
  DuckDB row equivalence, and run-name-only normalized loss/plot differences.
  Package: `20260620-r5a-full-day-direct-executor-lifecycle-001/`. Pushed
  commit: `3edfca66`.
- R4P/Q/Z is complete with verdict
  `COMPLETE-R4PQZ-HYDROLOGY-PROJECTION-R4-CLOSURE`. The package closed R4 by
  adding a shadow-only direct hydrology projection span after the direct
  hydrology compute chain. R4P/Q/Z requires direct upstream shadows from R4A,
  R4B, R4G, R4J, R4M, R4O, and R4N; recomputes aggregate storage from the
  final R4N layer vector; separates frozen-layer and explicit frozen storage;
  and assembles typed direct projection operands for runoff, ET, percolation,
  lateral/drainage, snow/frost, carry, profile-capacity placeholders, and
  publication comparison fields. Public output authority remains
  compatibility-owned: no WB13/WAT/PASS/loss/schema cutover, default
  activation, or scheduler change occurred. Full Rust gates passed. Final
  default-disabled H2637 reps were `645.54 s`, `644.74 s`, and `640.28 s`
  (median `644.74 s`, threshold `<= 676.67 s`) with protected output identity
  and PASS DuckDB row equivalence. Package:
  `20260620-r4pqz-hydrology-projection-r4-closure-001/`.
- R4N is complete with verdict
  `COMPLETE-R4N-DIRECT-WB17-ET-ROOT-UPTAKE-COMPUTE-PROMOTION`. The package
  promoted the R4E-H aggregate evapotranspiration handoff into request-free
  direct WB17 evapotranspiration and post-WB19 root-uptake compute. R4N now
  computes and shadow-projects surface/residue ET, soil-evaporation layer
  mutation, SWU/root-uptake vectors, water stress, and final aggregate ET.
  R4O consumes the R4N ET-mutated layer vector when present, and R4B requires
  final R4N ET before storage reconciliation. R4N remains no-publication,
  no-default-activation, and no-scheduler: public output paths and
  compatibility runtime remain authoritative. Full Rust gates passed. Final
  default-disabled H2637 reps were `643.84 s`, `650.42 s`, and `649.22 s`
  (median `649.22 s`, threshold `<= 676.67 s`) with protected output identity
  and PASS DuckDB row equivalence. Package:
  `20260620-r4n-direct-wb17-et-root-uptake-compute-001/`.
- R4M/O is complete with verdict
  `COMPLETE-R4MO-DIRECT-SUBSURFACE-COMPUTE-PROMOTION`. The package promoted
  the R4D/R4E-H subsurface handoff surface into request-free direct WB18/WB19
  compute from typed layer vectors. R4M computes and shadow-projects direct
  `D`, `Pe`, and per-layer percolation fluxes, mutating direct layer storage
  and feeding R4B `deep_seepage_m`. R4O computes and shadow-projects direct
  lateral `q`, tile drainage `Qdd`, final `Qd`, carry arrays, capacity/target
  diagnostics, and layer withdrawals, feeding R4B `subsurface_loss_m`. R4B now
  requires R4M and R4O shadows before storage reconciliation. R4M/O remains
  no-publication, no-default-activation, and no-scheduler: public output paths
  and compatibility runtime remain authoritative. Full Rust gates passed. Final
  default-disabled H2637 reps were `643.70 s`, `646.33 s`, and `639.62 s`
  (median `643.70 s`, threshold `<= 676.67 s`) with protected output identity
  and PASS DuckDB row equivalence. Package:
  `20260620-r4mo-direct-subsurface-compute-promotion-001/`.
- R4I-L is complete with verdict
  `COMPLETE-R4IL-DIRECT-RUNOFF-PATH-INPUT-COMPLETION`. The package implemented
  direct handoff producers for R4A `liquid_input_m`, `runon_input_m`,
  `cumulative_infiltration_m`, `depression_storage_delta_m`, and
  `surface_saturation_runoff_m`. Each producer has typed inputs, direct
  handoff compute, state mutation, downstream operands, and shadow projection.
  R4A now requires R4I liquid input, R4J runon/carry, R4K
  infiltration/depression, and R4L saturation-addback producers before runoff
  partition. R4I-L remains handoff-only, no-publication, no-default-activation,
  and no-scheduler: public output paths and compatibility runtime remain
  authoritative. The package split runoff-specific direct-runtime code into
  `direct_runtime/runoff.rs` and added a focused R4I-L test module. Full Rust
  gates passed. Final default-disabled H2637 reps were `646.47 s`,
  `642.52 s`, and `640.20 s` (median `642.52 s`, threshold
  `<= 676.67 s`) with protected output identity and PASS DuckDB row
  equivalence. Package:
  `20260620-r4il-direct-runoff-path-input-completion-001/`.
- R4E-H is complete with verdict
  `COMPLETE-R4EH-DIRECT-STORAGE-BUDGET-HANDOFF-COMPLETION`. The package
  implemented direct handoff producers for R4B `subsurface_loss_m` / `Qd`,
  aggregate `evapotranspiration_m`, and signed `snow_coupling_m`. Each
  producer has typed inputs, direct handoff compute, state mutation, downstream
  operands, and shadow projection. R4B now requires R4C storage input, R4D deep
  seepage, R4E-H subsurface loss, R4E-H evapotranspiration, R4E-H signed
  snow-coupling, and R4A runoff before storage reconciliation. R4E-H remains
  no-publication, no-default-activation, and no-scheduler: public output paths
  and compatibility runtime remain authoritative. Full Rust gates passed. Final
  default-disabled H2637 reps were `648.48 s`, `652.43 s`, and `642.26 s`
  (median `648.48 s`, threshold `<= 676.67 s`) with protected output identity
  and PASS DuckDB row equivalence. Package:
  `20260620-r4eh-direct-storage-budget-handoff-completion-001/`.
- R4D is complete with verdict
  `COMPLETE-R4D-DIRECT-DEEP-SEEPAGE-PRODUCER`. The package implemented a direct
  WB18/WB12 deep-seepage handoff producer feeding R4B `deep_seepage_m`. R4D
  consumes a dedicated direct deep-seepage handoff input, validates finite
  nonnegative `D`, mutates direct deep-seepage state and the R4B
  `deep_seepage_m` input, produces downstream operands, and shadow-projects the
  result. R4B now requires R4C storage input, R4D deep seepage, and R4A runoff
  before storage reconciliation. R4D remains no-publication, no-default-
  activation, and no-scheduler: public output paths and compatibility runtime
  remain authoritative. Full Rust gates passed. Final default-disabled H2637
  reps were `635.94 s`, `650.91 s`, and `645.47 s` (median `645.47 s`,
  threshold `<= 676.67 s`) with protected output identity and PASS DuckDB row
  equivalence. Package:
  `20260620-r4d-direct-deep-seepage-producer-001/`.
- R4C is complete with verdict
  `COMPLETE-R4C-DIRECT-STORAGE-INPUT-PRODUCER`. The package implemented a
  direct WB12 storage-input producer and split storage-related direct-runtime
  code into `direct_runtime/storage.rs`, reducing `direct_runtime.rs` below the
  2000-line WARN band. R4C consumes R3A direct precipitation and current direct
  storage, mutates the R4B `storage_initial_m` and `precip_input_m` inputs,
  produces downstream storage-input operands, and shadow-projects the result.
  R4B now requires R4C storage input and R4A runoff before storage
  reconciliation. R4C remains no-publication, no-default-activation, and
  no-scheduler: public output paths and compatibility runtime remain
  authoritative. Full Rust gates passed. Final default-disabled H2637 reps were
  `637.63 s`, `640.25 s`, and `639.19 s` (median `639.19 s`, threshold
  `<= 676.67 s`) with protected output identity and PASS DuckDB row
  equivalence. Package:
  `20260620-r4c-direct-storage-input-producer-001/`.
- R4B is complete with verdict
  `COMPLETE-R4B-DIRECT-STORAGE-RECONCILIATION-CONSUMER-SPAN`. The package
  implemented the downstream direct WB12 storage-reconciliation consumer of the
  R4A runoff result. It consumes R4A direct `q_runoff_m`, reconciles storage
  from explicit direct operands, mutates only direct storage state, produces
  downstream storage operands, and shadow-projects storage plus closure
  residual. R4B remains no-publication, no-default-activation, and
  no-scheduler: public output paths and compatibility runtime remain
  authoritative. Full Rust gates passed. Final default-disabled H2637 reps were
  `637.34 s`, `641.14 s`, and `646.88 s` (median `641.14 s`, threshold
  `<= 676.67 s`) with protected output identity and PASS DuckDB row
  equivalence. `direct_runtime.rs` is now in the 2000+ line WARN band at 2101
  lines, below the 3000-line blocker. Package:
  `20260620-r4b-direct-storage-reconciliation-consumer-001/`.
- R4A is complete with verdict
  `COMPLETE-R4A-DIRECT-RUNOFF-PARTITION-SPAN`. The package implemented the
  first direct hydrology-process span: a narrow SC-RUNOFFPART-authoritative
  runoff-partition closure slice. It consumes direct liquid, runon,
  cumulative-infiltration, depression-storage, and saturation-addback operands;
  computes direct runoff partition state; mutates only direct runtime water
  state; produces direct downstream runoff operands; and shadow-projects the
  direct runoff result. R4A remains no-publication and no-default-activation:
  it does not migrate full WB12/WB14, Green-Ampt infiltration, scheduler paths,
  compatibility APIs, output schemas, or production publication. Full Rust gates
  passed. Final default-disabled H2637 reps were `644.01 s`, `646.84 s`, and
  `643.66 s` (median `644.01 s`, threshold `<= 676.67 s`) with protected output
  identity and PASS DuckDB row equivalence. Package:
  `20260620-r4a-direct-runoff-partition-span-001/`.
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
