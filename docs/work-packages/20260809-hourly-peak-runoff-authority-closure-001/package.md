# Hourly Peak-Runoff Authority Closure

Status: `closure candidate — terminal verification pending`

Date: `2026-08-09`

Package ID: `20260809-hourly-peak-runoff-authority-closure-001`

Plan class: `Defect-Closure ExecPlan (DC-ExecPlan)`

Defect IDs: `PEAK-HOURLY-001`, `PEAK-RETURN-002`, `PEAK-UNITS-003`

This living ExecPlan is governed by `docs/codex_exec_plans.md` and
`docs/defect_closure_execplans.md`.

## Purpose

Replace openWEPP's WB16 rainfall-envelope peak surrogate with a peak derived
from the modeled hourly surface-runoff series. Preserve the hour in which the
soil-water calculation returns excess water to the surface; do not collapse it
to a daily depth or redistribute it over rainfall-excess intervals. Publish an
hourly mean runoff depth rate internally and convert it to hillslope volumetric
flow exactly once using the same area basis as event runoff volume. Demonstrate
continuity and physical ordering by executing the frozen Topanga small-mutation
hillslope design with openWEPP.

## Implementation Intent

- Intent: `science implementation + defect closure + independent cohort validation`.
- Risk: `Critical`; peak runoff drives erosion, routing, hazard, and public output.
- Calibration: `NOT_APPLICABLE`; no coefficient is fitted.
- Pre-implementation base: `a65cc3973ddd04b07cad108fcb33d83a8c161abb`.
- Canonical target: native openWEPP behavior, not legacy peak parity.

## Rationale And Updated Evidence

The 2026-08-09 Topanga report records 1,088 legacy mutation trials and 225,654
event rows. Event runoff is stable while peak response has a structured extreme
tail. Source forensics establish that legacy WEPP discards already calculated
hourly surface return, sums it to daily `surdra`, and assigns new timing over a
selected duration; a solver switch adds another discontinuity. This makes
additional legacy replication unnecessary for this package.

openWEPP already retains the ingredients for a closing 24-bin post-partition
runoff ledger: WB14 infiltration excess, hourly surface-saturation return, and
producer-timed routed melt/runon supplies. The initial implementation review
exposed an important ownership correction: melt and runon must enter WB14
infiltration/depression partition once and cannot also be appended as raw
runoff limbs. The defect is that WB16 ignores the resulting modeled timing and
reconstructs a peak from daily runoff, rainfall elapsed duration, and maximum
rainfall intensity. The same value is then mislabeled internally as `m3/s`
even though its computation is a depth rate; publication does not apply area.

## Correction Authority Envelope

### Observed violations

1. `PEAK-HOURLY-001`: distinct modeled hourly runoff shapes can produce the
   same WB16 peak because WB16 consumes a rainfall-envelope surrogate.
2. `PEAK-RETURN-002`: hourly soil-water surface return is present in runtime
   state but is not authoritative for peak timing.
3. `PEAK-UNITS-003`: an internal `m/s` depth rate is named and published as
   `m3/s` without applying hillslope area.

### In-scope write set

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/index.md`
- peak/hourly-runoff code and types under
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/`
- direct-publication and run-input wiring under `crates/openwepp-runner/src/hillslope/`
- affected unit, crate, integration, schema, and source-guard tests
- package-local `tools/`, artifacts, and compact frozen mutation-study metadata
- `Cargo.toml` only if an owned integration target is required
- this package tree, backlog note/tracker, and `docs/work-packages/README.md`

Adjacent same-process files may be added before first edit with a Decision Log
entry. Changes to watershed routing equations, Green-Ampt process equations,
ET, canopy, management, Ksat, erosion coefficients, or calibration values are
prohibited. Hourly melt/runon supply admission and post-partition ledger
closure at the existing WB14 boundary are explicitly in scope.

### Allowed production edits

- Make the closing post-partition 24-bin runoff depths the sole WB16 timing
  input and take the maximum hourly mean depth rate (`bin depth / 3600 s`).
- Admit producer-timed routed melt and resolved surface/lateral runon through
  WB14 infiltration/depression partition exactly once; do not append either
  daily source as a post-partition runoff limb.
- Preserve hourly saturation-return water in its modeled hour; no rain-window,
  positive-excess-window, or storm-duration reassignment.
- Fail closed for positive runoff without a reconstructible hourly series;
  uniform synthetic timing is not acceptable for a production peak claim.
- Rename internal depth-rate fields to `*_m_s` and retain explicit method and
  peak-bin provenance.
- Convert internal depth rate to `m3/s` exactly once at publication using the
  event-runoff volume area basis.
- Retain `runoff depth / hourly peak depth rate` only as an explicitly named
  rectangular-equivalent duration.

No subhourly interpolation or instantaneous-peak claim is authorized. The
published quantity is the maximum hourly mean hillslope flow.

### Acceptance criteria

1. Contract-first authority defines the 24-bin operands, units, spatial basis,
   peak definition, return-flow timing, dry behavior, guards, and provenance.
2. Anti-tautology tests distinguish concentrated versus spread hourly shapes
   with equal daily runoff and cover saturation-only, melt-supply-only,
   runon-supply-only, infiltrating-melt, and positive-runoff/missing-shape
   cases.
3. The real erosion and public-output consumers read the hourly-derived peak;
   the rainfall-envelope WB16 branch no longer carries the production claim.
4. Hourly depths independently sum to event runoff within the existing closure
   tolerance; public `m3/s` independently reconstructs as
   `max(hourly depth / 3600 s) * area_m2`.
5. Public flow scales linearly with area while internal depth rate is invariant.
6. Rectangular-equivalent duration reconstructs from named operands and is not
   described as rainfall duration, hydrograph duration, or time to peak.
7. The complete frozen Topanga 1,088-trial Ksat/cover design reaches terminal
   openWEPP results; event pairing and prevalence are reported without legacy
   parity, calibration, routed-watershed, or observed-flow claims.
8. Small mutations produce no unexplained branch discontinuity; any material
   tail is source-traced from hourly depths and separately dispositioned.
9. Critical focused, quick, full-workspace, formatting, Clippy, doctest,
   documentation, line-count, dual-review, and dual-verification gates pass.

### Protected boundaries

- Do not reproduce, instrument, repair, or target legacy APPMTH/HDRIVE.
- Do not claim an instantaneous subhourly peak from hourly bins.
- Do not calibrate or modify Ksat, cover, canopy, ET, or soil-water parameters.
- Do not use watershed routing to validate the hillslope source peak.
- Do not treat the legacy mutation census as correctness authority.

## Conversion Rule And Seven-Gate Bar

Reproduction is the static/runtime WB16 bypass and unit mismatch; mechanism is
rainfall-envelope reconstruction plus missing area conversion; ownership is the
declared WB16/hourly/publication write set; authority is the modeled hourly
mass lineage and contract-authorized dimensional/conservation invariant; safety
requires no clamp/default/proxy; testability uses unequal hourly shapes and
areas; validation uses independent reconstruction and the Topanga cohort.
Because all seven are measurable in-envelope, the package must implement and
may not stop at diagnosis.

## HOLD Legitimacy

HOLD is permitted only for missing/contradictory canonical authority, a proven
different process family, invalid upstream input with correct typed failure, or
evidence unavailable in this environment. Before HOLD, write
`artifacts/hold-legitimacy-audit.md` with boundary proof and the considered
in-envelope route. Effort, cohort size, or residual investigation are not
legitimate boundaries.

## Operand Lineage And Publication Acceptance

Before production edits, `artifacts/operand-lineage.md` records every hourly
source, units, temporal basis, spatial basis, normalization, authority, and
rejected aliases. Acceptance must reconstruct from produced hourly operands,
not restate the producer formula using its final peak. Equal-volume fixtures
must distinguish adjacent diagnostics and the old rainfall-envelope formula.

## Phase Plan

1. Freeze updated evidence, call chain, required reading, intent, and operands.
2. Amend `SC-WATBAL-001`; add contract-derived red tests; record the failing gate.
3. Implement hourly peak authority, typed provenance, unit rename, and public
   area conversion through real consumers.
4. Run focused/quick validation and the complete openWEPP Topanga mutation study.
5. Run Critical closure gates, independent review, finding disposition, two
   terminal verifications, exact-diff reconciliation, and close package/backlog.

## Validation

- Focused owning crate and integration tests.
- `cargo fmt --all --check`.
- warnings-denied Clippy for affected crates/tests.
- `cargo nextest run --workspace --profile quick`.
- `cargo nextest run --workspace --profile full` (Critical exact-head closure).
- workspace doctests.
- mutation-study validation and independent recomputation.
- Markdown lint/path checks and `git diff --check`.
- `.rs` line-count governance: 2000+ `WARN`; 3000+ nonexempt blocks closure.

`TMPDIR` for heavy gates must be an absolute directory outside this checkout,
for example `/home/workdir/openwepp-task-tmp`.

## Subagent Authorization And Requirements

Subagent authorization: this package explicitly authorizes spawning/delegating
to bounded read-only source investigators, two independent hydrology/science
reviewers, the primary `rust_code_reviewer`, the secondary `rust_qa_reviewer`,
two independent terminal verifiers, and `comparator_suite_runner`. Investigators
return compact findings; reviewers/verifiers may write only their named package
artifacts; the suite runner may write package logs and generated external/target
evidence but no production, contract, or test source.

Subagent requirement: REQUIRED. Spawn `comparator_suite_runner` for the full
Topanga mutation cohort and all heavy batch/closure/full-workspace/doctest runs.
The parent must not run them unless that role is unavailable and the failure is
recorded. Use `rust_code_reviewer` and `rust_qa_reviewer` as Rust gates and two
independent science reviewers for the hydrologic authority and claim boundary.

## Progress

- [x] (2026-08-09) User promoted the backlog and retired legacy replication.
- [x] (2026-08-09) Read the newer Topanga census and confirmed the general
  legacy daily-return retiming defect plus structured mutation tail.
- [x] (2026-08-09) Scoped hourly authority, return timing, units, publication,
  and the full openWEPP mutation cohort into one closure envelope.
- [x] (2026-08-09) Completed contract-first authority and captured the expected
  pre-implementation red gate.
- [x] (2026-08-09) Implemented the shared-hour peak, source-timing custody,
  internal depth-rate naming, and exactly-once public area conversion.
- [x] (2026-08-09) Corrected the initial raw-source assembly after independent
  review: routed melt and runon now enter WB14 supply once, and peak/transfer/
  erosion/HBP consume the closing post-partition hourly ledger.
- [x] (2026-08-09) Built exact anchor `949349e70` and passed a fresh
  provenance-bound one-baseline/one-mutation Topanga probe.
- [x] (2026-08-09) Passed focused implementation and real-consumer validation;
  executed all 1,088 frozen Topanga trials and found no unexplained
  volume-stable peak discontinuity.
- [x] (2026-08-09) Completed dual science review at implementation/test commit
  `df41f3526`; Rust review then exposed stale `SC-SED-001` peak authority and
  warmed-fixture H2637 evidence-counter drift. Both findings were reconciled
  and independently re-reviewed at `33831787b`.
- [x] (2026-08-09) Passed the exact-head Critical full-workspace regression
  (2,346/2,346), reconciled the quick-profile inventory against that complete
  receipt, and passed workspace doctests.
- [ ] Complete dual terminal verification and final disposition.

## Surprises And Discoveries

- openWEPP already has one shared 24-bin runoff shape used by inter-OFE
  transfer, hourly erosion, and HBP serialization; WB16 alone ignores it.
- The current internal field name says `m3/s`, but the computation and
  `Q / peak` duration prove the stored value is `m/s`.
- A native Topanga probe exposed positive subtraction roundoff with no hourly
  source. Source-informed canonicalization is required at the runoff partition,
  not a peak floor: only `<=1e-12 m` source-free residuals become exact zero.
- Initial review exposed that appending raw routed melt after partition would
  double count melt that can infiltrate. The corrected design admits melt and
  runon as hourly WB14 supply and derives weights only after hourly/daily
  closure.
- WB14 independently accumulates up to 24 interval results; their arithmetic
  ledger can differ from the daily scalar by at most the contract-declared
  `TOL-WATBAL-009`. Material mismatch remains a typed hard failure.
- Independent science review rejected distributing a daily-only frost debit
  across positive hourly runoff as proxy timing. Complete retention may clear
  the local series, but partial retention leaving runoff now requires an hourly
  producer and otherwise fails closed.
- Independent science review exposed that even a local-only daily same-pass
  infiltration correction invents timing when it debits earliest hourly bins.
  The correction is retired: WB14 now solely owns cumulative infiltration and
  hourly residuals for rainfall, routed melt, and runon.
- HBP EVENT output had retained a fixed fixture calendar year, making a
  multi-year Parquet join ambiguous. It now publishes the selected producer
  row's calendar identity, and p61/p102 join on year plus Julian day.
- The first full cohort attempt completed case execution but its summary write
  exposed heterogeneous scalar/dictionary mutation values in one Arrow column.
  The harness now stores those plan operands as stable JSON strings and has a
  focused Parquet-construction regression.
- A generated-watershed test carried a dry second day after its runoff day;
  HBP correctly publishes the latest state when no sediment event qualifies,
  so the old nonzero assertion depended on accidental peak-driven erosion.
  The fixture now uses one explicit runoff day and continues to prove generated
  HBP inventory and downstream routing without that accidental threshold.

## Decision Log

- Decision: do not vendor or replay the legacy Hill 106 reproducer.
  Rationale: the 1,088-trial audit and source instrumentation establish the
  general legacy timing defect; legacy parity is not openWEPP correctness.
- Decision: define peak as maximum hourly mean, not instantaneous peak.
  Rationale: this is exactly supported by retained model resolution and avoids
  invented subhourly physics.
- Decision: surface return remains in the WB19-produced hour.
  Rationale: it preserves modeled process timing and mass without synthetic
  reassignment to rainfall intervals.
- Decision: routed melt and runon are hourly WB14 supply, not runoff limbs.
  Rationale: each source must receive infiltration and depression-storage
  opportunity exactly once before its residual can contribute to a peak.
- Decision: never allocate a daily frost-retention scalar across hourly runoff.
  Rationale: mass closure cannot manufacture subdaily process timing. The
  current producer can prove complete retention, while partial positive
  retention remains a typed missing-upstream failure until hourly custody is
  implemented by an authorized follow-on.
- Decision: do not apply any later daily-only infiltration reconstruction to
  hourly WB14 runoff, including a local-only snowmelt case.
  Rationale: WB14 already consumes producer-timed rainfall, melt, and runon and
  owns cumulative infiltration. A later daily debit has no lawful hour and
  would manufacture peak timing.
- Decision: use `/home/workdir/openwepp-task-tmp` for heavy local gates.
  Rationale: it is an absolute external scratch directory consistent with the
  canonical temporary-directory guidance and avoids repository-root
  confinement failures. Reuse this pattern in later packages rather than an
  in-checkout `target/` descendant.
- Decision: normalize mutation-plan source/expected operands to JSON strings in
  the event-pair Parquet.
  Rationale: Ksat trials use scalars while paired-cover trials use objects; a
  stable JSON column preserves both without Arrow type inference failure.
- Decision: add `SC-INFILE-HBP-001.md` to the adjacent consumer-contract write
  set before editing it, and add
  `crates/openwepp-hillslope-output/src/hillslope_pass.rs` for the matching
  public Parquet field metadata.
  Rationale: the HBP contract still mislabeled rectangular-equivalent duration
  as storm duration and did not state maximum-hour peak reconstruction from its
  own minor-1 hourly volumes; leaving that stale would contradict the corrected
  producer and public schema; the Parquet metadata must name the same quantity.
- Decision: add `SC-SED-001.md` to the adjacent consumer-contract write set
  after the full consumer gate exposed stale analytical and volumetric peak
  authority in the erosion contract.
  Rationale: erosion already consumes the corrected internal maximum-hour
  depth rate. Closure requires its active authority to name the same `m/s`
  operand, rectangular-equivalent duration, public-only area conversion, and
  no-fallback posture before an exact-head gate can be admitted.
- Decision: give erosion's independent rectangular-duration reconstruction a
  seconds-dimensional absolute custody tolerance (`TOL-SED-009`) matching the
  active erosion guard (`1.001e-9 s`).
  Rationale: the first SC-SED reconciliation incorrectly reused
  `TOL-SED-001`, whose units are sediment mass flux. Duration is derived once
  by WB16 and passed unchanged; any independent check must compare seconds to
  seconds, cannot borrow an unrelated conservation tolerance, and cannot use a
  seconds-squared scale expression.

## Outcomes And Retrospective

WB16 now publishes the maximum hourly mean flow derived from the closing
post-partition 24-bin runoff ledger. WB14 owns infiltration and hourly residual
timing for rainfall, routed melt, and runon; WB19 surface return remains in its
modeled hour. The internal peak is a depth rate in `m/s`, and public hillslope
flow applies area exactly once to obtain `m3/s`. Missing positive hourly
custody fails closed; APPMTH/rainfall-envelope timing and synthetic uniform
shape fallbacks are not production authority.

The exact release binary at implementation/contract/test commit `33831787b`
completed 280 baselines and all 1,088 frozen Topanga mutations. Across
1,913,199 paired event rows, there were zero invalid maximum-hour fractions,
zero zero-runoff topology mismatches, and zero cases with runoff volume within
5% but peak at least 2x. The largest ratio-decomposition residual was
`4.440892098500626e-16`; the extreme raw peak ratio came only from a near-zero
denominator and did not represent a volume-stable discontinuity.

Critical closure passed 2,346/2,346 exact-head workspace tests, focused
consumer and authority gates, warnings-denied Clippy, formatting, doctests,
anti-evasion checks, documentation checks, two independent science reviews,
Rust correctness and QA reviews. Dual terminal verification is the remaining
closure step.
The supported result is a maximum hourly mean hillslope runoff flow. It is not
an instantaneous/subhourly peak, legacy-parity result, calibrated or observed
flow validation, or routed watershed/channel-flow claim.
