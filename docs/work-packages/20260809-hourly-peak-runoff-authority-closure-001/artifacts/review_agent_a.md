# Independent Hydrology / Science Re-Review A

Status: `executed`

Reviewed identity: `33831787b7029b28b0716c8458f08a11899db446`

Reviewed range: `a65cc3973..33831787b7029b28b0716c8458f08a11899db446`

Evidence class:

- `Static`: terminal base-to-implementation diff; exact source, contracts,
  tests, package artifacts, publication, manifest, and downstream consumers.
- `Ran`: `git diff --check
  a65cc3973..33831787b7029b28b0716c8458f08a11899db446`; PASS.
- `Ran`: focused WB14/WB16 source-custody, frost, melt/runon, and typed-guard
  expression; 8 passed, 463 skipped, nextest run
  `a41e5d84-25fe-4d1e-a660-90361fdb375e`.
- `Ran`: focused contract-derived peak/ealpha tests; 2 passed, 2 skipped,
  nextest run `11086081-0e1a-469b-9014-de93be7ba6bc`.
- `Ran`: real single-OFE p61 and routed multi-OFE p102 consumers; 2 passed,
  0 skipped, nextest run `02870d32-9f2a-4a37-86fc-44a0908f1e12`.
- `Ran`: exact R4I -> R4J -> R4K pure-melt limited-capacity regression; 1
  passed, 471 skipped, nextest run
  `d299c257-ec94-4944-bedf-ecc861bff658`.
- `Ran`: exact-commit `peak_hourly_authority_contract`; 4 passed, 0 skipped,
  nextest run `a6cf9ca5-4401-4ee6-b2dc-6275f3bebbea`.
- `Ran`: exact-commit `erod16_wave1_continuity_fixture_conservation` and
  `laned_shadow_h2637`; 9 passed, 2 skipped, nextest run
  `066d6421-fedc-4a1a-a40b-6630da598953`.
- `Ran`: exact-commit `peak_hourly_authority_contract`, p61, and p102; 6
  passed, 0 skipped, nextest run
  `e700bffe-e6a2-4063-a614-bd4c5cc91f4d`.
- `Ran`: exact-commit `peak_hourly_authority_contract` after SC-SED rev62; 4
  passed, 0 skipped, nextest run
  `87582508-5d33-46ea-b72f-ec201856bb83`.
- `Ran`: exact-commit SC-SED rev63 source/contract guard; 4 passed, 0 skipped,
  nextest run `f13f214b-e48a-477f-a720-a8a6766a892f`.
- `Ran`: exact-commit absolute-seconds duration behavior at 0.25 s, 10 s,
  and 80,000 s scales; 1 passed, 472 skipped, nextest run
  `453394d5-e94d-46f2-a1d1-1d0817efe2f8`.
- `Static/package-recorded`: remaining package gates and census evidence were
  inspected but not independently rerun by this reviewer.

Verdict: `PASS`

## Findings

### `REVIEW-A-MAJOR-005` — resolved

SC-SED-001 rev63 correctly defines the internal erosion peak as the maximum
hourly mean depth rate in `m/s`, defines the public `m3/s` value as publication
only with one area multiplication, names `watdur = Q / peakro_depth` as a
rectangular-equivalent duration, and prohibits uniform, rainfall-window, or
analytical fallback. Rev63 corrects rev62's malformed seconds-squared relative
expression: `TOL-SED-009` is now the absolute
`abs(watdur - Q / peakro_depth) <= 1.001e-9 s` threshold. The named live guard
constant has the same value and the active erosion check consumes it directly.
Its authority explicitly says it cannot absorb missing or mismatched hydrology
operands, is not scale-relative, and is not a sediment-continuity tolerance.
Both active WB16 and EROD13 validation steps cite `TOL-SED-009`;
`TOL-SED-001` remains scoped to sediment continuity. Contract/source guards
reject both the former sediment-tolerance reuse and rev62 relative expression.
Behavioral vectors prove below-threshold acceptance and above-threshold failure
at small, ordinary, and near-day duration scales. All exact-commit vectors
passed.

No other Critical, Major, or Minor hydrology/science finding remains at the
exact reviewed implementation/contract/test commit.

### `REVIEW-A-MAJOR-004` — resolved

`SC-WATBAL-001` v170 correctly says that `ealpha` and the APPMTH operands are
retired from production WB16 and may remain only in explicitly historical
diagnostic schemas (`WB16 Deterministic Peak-Flow Rules`, rule 8, around line
1272). The exact runtime agrees: no orchestrator peak producer or consumer of
`ealpha` remains; the runner fixes compatibility use to `false` and publishes
`wb16_ealpha_seed_policy = "retired_not_applicable"`
(`00_runner_intake_and_lane_setup.rs:411` and
`05_runner_execution_and_outputs.rs:545,632-634`). Focused contract tests pass
for that retirement posture.

`SC-WATBAL-001` v170 now marks `GAP-WATBAL-005` as `closed — superseded`, records
the former producer migration as historical, and states that no active peak
producer or compatibility branch consumes `ealpha`. It binds retained manifest
fields to `wb16_ealpha_compatibility_seed_used=false` and
`wb16_ealpha_seed_policy=retired_not_applicable` solely for historical schema
lineage (`SC-WATBAL-001.md:2441`). Revision v170 explicitly records the
supersession. The exact-commit four-test contract suite enforces the gap marker
and retired manifest policy and passed. Current runtime, manifest, contract,
and contract test provenance therefore agree.

## Implementation Assessment

### WB14 sole infiltration and hourly-residual authority

The prior post-WB14 path is retired. `run_r4k_infiltration_depression_span`
now accepts `infiltration_depression.cumulative_infiltration_m` directly and no
longer calls `resolve_r4m_same_pass_infiltration_m`, reconstructs infiltration
from daily snow/liquid totals, or removes that reconstructed debit from hourly
runoff (`direct_runtime/runoff.rs:220-253`). The resolver and its test shim are
deleted, and the contract-derived test rejects the former
`snow_reconstructed_same_pass_infiltration_m` symbol.

The remaining `remove_depth_from_hour_bins_earliest` call is not a later daily
infiltration alias: it is inside `compute_wb14_infiltration_depression_with_profile`
and applies WB14's own depression-storage debit to WB14's own interval excess
before returning the post-depression hourly ledger
(`direct_runtime/runoff.rs:1840-1910`). Thus WB14 alone owns cumulative
infiltration and hourly residual timing for producer-timed rainfall, routed
melt, and area-scaled surface/lateral runon.

The terminal test-only delta from `f9082926` exercises the real R4I -> R4J ->
R4K pure-melt path with finite WB14 infiltration capacity. It proves
`0 < infiltration < melt supply`, publishes that exact WB14 infiltration to
same-pass percolation, closes the residual to `melt - infiltration`, and keeps
all residual runoff in melt hour 5. The test passed at the reviewed identity;
the delta makes no production or contract change and does not affect the
finding above.

### Exact-positive frost residual

Complete daily frost retention clears the hourly series only when
`partition_runoff_m == 0.0`. Every exactly positive partition residual,
including `5e-13 m`, takes the partial-retention missing-producer failure when
frost retention is material; it is neither cleared nor redistributed
(`direct_runtime/runoff.rs:1389-1438`). The focused vector proves that the
original positive hourly bins remain unchanged on failure. Positive daily
runoff with an empty hourly ledger also remains fail-closed.

### Actual-value WB16 diagnostics

WB16 now constructs `Wb11HydrologyKernelGuardError` at each validation site.
Non-finite errors retain the observed non-finite value; negative/domain errors
retain the actual operand and bounds; hourly-source closure reports the actual
source total with bounds derived from daily `Q` and tolerance
(`direct_runtime/runoff.rs:1483-1614`). The former adapter that fabricated
`NaN`, `-1.0`, or `1.0` is gone. Focused tests inspect the typed variants and
values, not only their error codes.

### Prior hourly, area, and consumer claims

The earlier source-custody conclusions remain valid. Melt and separated,
area-scaled surface/lateral runon enter WB14 once on their producer shapes;
positive runon without a WB14 producer or hourly transfer shape fails. WB19
saturation return remains in its produced hour. Closing WB14/WB19 depths must
sum to daily `Q`, and the peak is `max(q_hourly)/3600` rather than a daily
scalar reconstructed through normalized weights.

Publication validates positive area and multiplies the basis-adjusted internal
depth rate by the event-runoff area exactly once. `SC-INFILE-HBP-001` binds the
public value to `max(V_h)/3600` and `sum(V_h) = runvol`. The exact-commit p61
and p102 tests independently reconstructed the single-OFE and routed outlet
HBP/pass-Parquet peak and passed.

### Terminal erosion-unit and Lane-D fixture delta

The `d934ab9b..df41f352` delta changes tests only. The EROD16 continuity fixture
now reads public PASS `peakro` as `m3/s` and divides by
`area_m2 = fwidth_m * efflen_m` exactly once before applying the legacy
depth-rate passby threshold or constructing Wave-1 erosion operands. This is
the inverse of the reviewed publication conversion and restores the internal
`m/s` basis without altering runoff depth or adding another area factor
(`tests/integration/erod16_wave1_continuity_fixture_conservation.rs:623-646`).
Static inspection confirms that the original concave deposition geometry,
independent cell-ledger reconstruction, and 25% bar remain unchanged. The
fixture passed and continued to require nonzero conserving deposition.

The H2637 fixture copies the canonical management, slope, soil, and climate,
then changes only daily maximum temperature, minimum temperature, and dewpoint
to a frost-free envelope. It does not change precipitation or any management,
soil, slope, or routing operand. This prevents the intentionally fail-closed
partial-frost missing-clock guard from preempting a test whose authority target
is Lane-D routing coefficients; it does not bypass, relax, or mutate that
production guard. The Lane-D suite passed its fail-closed and active-owner
vectors on the isolated forcing.

## Claim And Evidence Boundary

The implementation review supports only a hillslope-scale, non-calibrated
maximum-hourly-mean runoff peak. It does not support an instantaneous peak,
subhourly timing, watershed/channel routing, observed-flow validation, or
legacy numerical parity.

Separately, package closure evidence remains unreconciled to this exact commit:
`implementation-test-evidence.md` says terminal gates are pending;
`mutation-study.md` and `summary.md` identify anchor `949349e7055c5d19277eeb708401c4614a52cd77`;
`gate-results.md` is queued; and `disposition.md` remains executing. An untracked
`topanga-openwepp-census-full-v4.log` was present and was not treated as
reviewable committed evidence. These evidence gaps do not create the
implementation finding above, but they independently preclude a package-close
claim.

Implementation and canonical authority are acceptable on all reviewed science
axes, including rev63's peak units, timing, duration custody, area conversion,
and fallback posture. Terminal package disposition remains outside this
review's PASS until its separate exact-anchor evidence is reconciled.
