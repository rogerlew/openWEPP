# Hourly Peak-Runoff Authority Closure

Status: `executing`

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

openWEPP already retains the source-complete 24-bin runoff shape:
WB14 infiltration excess + hourly surface-saturation carry + routed melt/runon.
The defect is that WB16 ignores that modeled timing and reconstructs a peak from
daily runoff, rainfall elapsed duration, and maximum rainfall intensity. The
same value is then mislabeled internally as `m3/s` even though its computation
is a depth rate; publication does not apply area.

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
- `docs/specifications/science-contracts/index.md`
- peak/hourly-runoff code and types under
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/`
- direct-publication and run-input wiring under `crates/openwepp-runner/src/hillslope/`
- affected unit, crate, integration, schema, and source-guard tests
- package-local `tools/`, artifacts, and compact frozen mutation-study metadata
- `Cargo.toml` only if an owned integration target is required
- this package tree, backlog note/tracker, and `docs/work-packages/README.md`

Adjacent same-process files may be added before first edit with a Decision Log
entry. Changes to watershed routing equations, infiltration, ET, canopy,
management, Ksat, erosion coefficients, or calibration values are prohibited.

### Allowed production edits

- Make the existing source-complete 24-bin runoff depths the sole WB16 timing
  input and take the maximum hourly mean depth rate (`bin depth / 3600 s`).
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
   with equal daily runoff and cover saturation-only, melt-only, runon-only,
   and positive-runoff/missing-shape cases.
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
- [ ] Complete contract-first authority and red tests.
- [ ] Implement the real consumer path and public conversion.
- [ ] Execute focused and Topanga cohort validation.
- [ ] Complete Critical closure, review, verification, and disposition.

## Surprises And Discoveries

- openWEPP already has one shared 24-bin runoff shape used by inter-OFE
  transfer, hourly erosion, and HBP serialization; WB16 alone ignores it.
- The current internal field name says `m3/s`, but the computation and
  `Q / peak` duration prove the stored value is `m/s`.

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

## Outcomes And Retrospective

Pending execution.
