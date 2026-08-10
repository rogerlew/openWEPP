# Rust QA Review

Status: `complete`

Review target: exact terminal commit
`669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`, against the declared
pre-implementation base `a65cc3973ddd04b07cad108fcb33d83a8c161abb`.

Evidence class: `Static + Ran`. The narrow ADR/test reconciliation was
inspected from the exact Git objects and checked in the exact checkout. Prior
exact-head implementation evidence remains reusable because the reopened delta
does not change runtime code. Concurrent package-evidence edits and untracked
suite logs were excluded from source conclusions and inspected only for gate
disposition.

## Findings

No blocking QA findings remain at the reviewed commit.

### LOW — ADR-0036 D3 contains a stranded sentence fragment

Path:
`docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md:194`

The amendment leaves `The` at the end of the fallback sentence and begins the
next line with `For`, rendering as “The For a current hourly-surface payload”.
This does not alter the decision or evade the guard, but it interrupts the
canonical ADR's consumer-boundary explanation. Remove the stranded `The` in a
documentation-only cleanup.

### PASS — ADR-0036 and its source guard bind one native peak authority

Paths:

- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md:13`
- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md:198`
- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md:273`
- `tests/integration/peak_hourly_authority_contract.rs:127`

The amendment reconciles D2 through D5 and Alternative 4 with the canonical
SC-WATBAL/SC-SED authority. For current hourly-surface payloads, internal peak
is the maximum modeled hourly mean depth rate and public/HBP peak is the same
maximum after exactly one area conversion. A separate analytical estimator is
explicitly retired from native authority; scalar peak fallback is restricted
to legacy shards that lack the paired hourly water surface and cannot support a
current hourly-peak acceptance claim.

The source guard requires the maximum-hour equation, the rejection of
independent peak authority, and the legacy-only compatibility boundary. It also
rejects the three exact former contradictory statements from D4 and
Alternative 4. The focused binary passed all four tests in reviewer-run nextest
run `413d3124-bf58-4973-9abf-4e4a725d736e`.

### RESOLVED — TOL-SED-009 is dimensionally valid and matches runtime

Paths:

- `docs/specifications/science-contracts/contracts/SC-SED-001.md:374`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs:1110`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/tests/erosion_hb01.rs:192`
- `tests/integration/peak_hourly_authority_contract.rs:84`

SC-SED rev63 replaces the seconds-squared expression with the absolute,
dimensionally valid bound
`abs(watdur - Q / peakro_depth) <= 1.001e-9 s`. Production names that same
threshold as `DIRECT_EROD13_DURATION_CUSTODY_TOLERANCE_S` and the live duration
guard consumes the named constant directly. The contract, implementation, and
source-level guard therefore describe the same custody rule.

The behavioral test checks sub-threshold acceptance and supra-threshold refusal
at 0.25, 10, and 80,000 seconds, proving the tolerance is absolute rather than
scale-relative. That focused test passed in nextest run `3856c183` (`1/1`), and
the expanded contract/source binding passed in run `381239de` (`4/4`).

### RESOLVED — H2637 source-complete routing oracle matches the warmed fixture

Path: `tests/integration/laned_shadow_h2637.rs:39`

The initial ignored-test diagnostic correctly exposed that the shared warming
mutation changed the D12 routing population: 731 days routed rather than the
historical 622. The terminal reconciliation renames that test to describe its
source-complete purpose and updates the coupled manifest counters together:
731 days seen, 731 days routed, zero uniform-shape days, and zero uniform days
in both melt subclasses. The assertions now describe the fixture actually
being exercised rather than preserving a pre-mutation classification.

All ten H2637 tests, including both ignored campaign tests, passed at exact
`0d5fa08b2` in nextest run `ccdec8be` (388.545 seconds). The prior failing
diagnostic run remains useful evidence that the new assertions were prompted by
an observed fixture effect, not weakened speculatively.

## Prior Finding Rechecks

### PASS — positive frost residuals are no longer tolerance-cleared

Paths:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1385`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1423`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:277`

Complete daily-only frost retention clears the hourly series only when the
reconciled `partition_runoff_m` is exactly zero. Every positive reconciled
residual with material daily frost retention, including the `5e-13 m` vector,
takes the typed missing-hourly-producer path without mutating the source bins.
The aggregate reconciliation tolerance adjudicates the independently summed
ledgers; it no longer authorizes erasing positive timed runoff.

### PASS — the post-WB14 daily same-pass retiming alias is retired

Paths:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:207`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r7g_frost.rs:724`
- `tests/integration/peak_hourly_authority_contract.rs:43`

R4K now publishes WB14's own cumulative infiltration and hourly excess without
a later snow-derived daily reconstruction or an earliest-bin debit. The added
real-span R4I -> R4J -> R4K pure-melt vector uses limited infiltration capacity,
proves partial infiltration, independently closes the residual depth, and
proves the positive residual remains solely in the producer's hour. Separate
source-complete peak tests prove that same melt residual selects its produced
peak hour. The remaining earliest-bin helper is internal to WB14's chronological
depression-storage solve and is not a post-partition timing reconstruction.

### PASS — census receipts bind the complete reusable input and calendar

Paths:

- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/topanga_openwepp_census.py:26`
- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/test_topanga_openwepp_census.py:1`

Record schema v3 binds case, plan, binary, all four primary input hashes, and
all 12 discoverable runner sidecars: frost, snow, WEPP UI, PMETPARA, both
irrigation forms, groundwater coefficients, phosphorus, `tc`, `tcr`, `lcwb`,
and channel input. It also binds expected row count, calendar digest, and exact
simulation-year/Julian arrays. Reuse rejects corrupt, empty, truncated,
wrong-calendar, changed-binary, changed-primary, changed-sidecar, non-finite,
and negative records. Output ingestion independently checks the Parquet
calendar before publishing the receipt.

### PASS — HBP and Parquet consumers use the event's complete day identity

Paths:

- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs:436`
- `tests/integration/erosion_single_ofe_p61_sediment.rs:154`
- `tests/integration/erosion_multi_ofe_p102_chain.rs:84`

The HBP EVENT carries the selected producer row's calendar year and Julian day.
Both real consumers derive the simulation year from that calendar year, join
Parquet by simulation year plus Julian day, and independently reconstruct
`sum(V_h) = runvol` and `max(V_h) / 3600 = peakro`; p61 also reconstructs the
rectangular-equivalent duration. The p61 test explicitly identifies its copied
climate's controlled 2x dominant-storm mutation, treats it as a real-consumer
exercise rather than a legacy oracle, and applies a broad physical plausibility
band. The p102 soil mutation is likewise performed only in a copied fixture and
documents the intended per-OFE provenance probe.

### PASS — typed WB16 guards retain their actual operands

Paths:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1477`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:188`

WB16 now constructs typed hydrology guards at the validation sites. Non-finite
errors retain the observed non-finite value; closure errors retain the observed
hourly total and meaningful lower/upper bounds. Focused tests destructure the
typed variants and assert those values instead of checking only message codes.

### PASS — retired `ealpha` provenance and authority agree

Paths:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs:411`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:545`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:2441`
- `tests/integration/cli03_runner_contract_derived_tests.rs:449`

No active peak path consumes `ealpha`. Retained manifest lineage publishes
`wb16_ealpha_compatibility_seed_used=false` and
`wb16_ealpha_seed_policy=retired_not_applicable`. Contract v170 marks the old
`GAP-WATBAL-005` producer-chain claim `closed — superseded`, and contract plus
fixture tests bind the new disposition.

### PASS — line-count governance is complete and actionable

Path:
`docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/line-count-governance.md:1`

Independent exact-commit counts match all seven changed Rust files at or above
2,000 lines: 2,996, 2,981, 2,892, 2,760, 2,742, 2,707, and 2,028 lines. No file
reaches 3,000 lines. The artifact provides a concrete extraction seam for each
production and test file rather than a generic split promise.

### PASS — terminal EROD16 correction preserves the acceptance bar

Paths:

- `tests/integration/erod16_wave1_continuity_fixture_conservation.rs:623`

The terminal delta is test-only. EROD16 now reads public PASS `peakro` as
`m3/s`, divides by `area_m2 = fwidth_m * efflen_m` exactly once, and supplies
the resulting `m/s` depth rate to both the legacy passby gate and Wave-1
operands. This is the inverse of production's exactly-once public area
conversion. The concave profile, independent cell-ledger reconstruction,
per-day `1e-9` conservation bounds, nonzero deposition requirement, deposition
engagement bar (`depositing_days * 4 >= clean_days`), and bounded refusal bar
(`refusals <= 20%`) are unchanged by the delta.

The H2637 helper still copies the canonical fixture before mutation.
Independent token inspection found exactly 731 daily rows. The helper changes
only zero-based
fields 7, 8, and 12 (`tmax`, `tmin`, and dewpoint) to `20`, `10`, and `8 C`;
date, precipitation, duration, peak timing/intensity, radiation, wind, row
count, and ordering remain unchanged. It does not disable or weaken the
production partial-frost guard. Its derived routing effects are now represented
by the source-complete D12 manifest assertions described above.

### PASS — full-gate failures have bounded, evidence-preserving dispositions

Paths:

- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/gate-results.md:1`
- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/artifacts/topanga-openwepp-census-full-v4-nextest-full-retry-threads2.log:1`

The default-concurrency quick run's two 600-second assurance timeouts and the
operator-interrupted scheduling probes are correctly non-admitted; no timeout,
selection, or assertion was relaxed. The completed two-thread full run reached
all 2,345 selected tests and reported 2,339 passes plus six failures:

- Four peak-authority source-reading failures came from a cached test binary
  whose compile-time `CARGO_MANIFEST_DIR` named a deleted exact-review archive.
  They are shared-target contamination, not source behavior; a clean target is
  the correct disposition.
- EROD16 exposed the test's stale use of public volumetric peak as an internal
  depth rate. The df41 correction restores units without changing the
  conservation/deposition acceptance bar.
- H2637 exposed an unrelated, intentionally fail-closed partial-frost guard
  before the Lane-D boundary under test. The copied-fixture thermal envelope
  isolates the intended test authority without changing production. The first
  ignored-test run then exposed its derived routing-population effect; the
  `0d5fa08b2` assertions reconcile all affected counters together.

The exact terminal non-ignored fresh-target focused receipt is green: four peak-authority
tests plus the EROD16 test and eight non-ignored H2637 tests passed (`13/13`;
the two explicitly ignored campaign tests remained skipped). The subsequent
diagnostic run-ignored receipt failed `1/2` and identified the stale D12 oracle.
After reconciliation, a terminal all-H2637 receipt passed all ten tests,
including both ignored campaign tests. Terminal full and quick workspace
receipts remain separately required for package closure.

## Exact Evidence Assessed By This Reviewer

- `cargo fmt --all -- --check` — PASS at exact `669269ee4`.
- Warnings-denied Clippy for `peak_hourly_authority_contract` — PASS at exact
  `669269ee4`.
- Warnings-denied Clippy for all `openwepp-hillslope-orchestrator` test targets
  remains PASS from `33831787b`; runtime and hillslope test source are unchanged
  by the intervening closure/reopen commits.
- The broader warnings-denied Clippy receipt for unchanged production and
  affected crates remains PASS from exact predecessor `d934ab9b`.
- Five focused pure-melt, frost-boundary, typed-guard, and peak-hour tests —
  PASS (`5/5`) on unchanged exact source.
- `peak_hourly_authority_contract`, p61, p102, and retired-`ealpha` fixture
  coverage — PASS (`7/7`) on unchanged exact source/tests.
- Exact terminal EROD16, non-ignored H2637, and peak-authority fresh-target
  receipt — PASS (`13/13`, two explicit campaign tests skipped).
- Initial H2637 run-ignored diagnostic — FAIL (`1/2`): D16 active-owner passed;
  D12 observed 731 routed days against the stale 622 expectation.
- Reconciled exact-terminal H2637 receipt — PASS (`10/10`, including both
  ignored campaign tests), nextest run `ccdec8be` in 388.545 seconds.
- Exact-terminal absolute-duration behavioral coverage — PASS (`1/1`), nextest
  run `3856c183`.
- Exact-terminal `peak_hourly_authority_contract` — PASS (`4/4`), nextest run
  `413d3124-bf58-4973-9abf-4e4a725d736e`; the guard binds rev63, its live named
  runtime constant, and amended ADR-0036 authority.
- Census receipt/provenance tests — PASS (`6/6`) on unchanged exact tooling.
- `git diff --check 33831787b..669269ee4` and
  `git diff --check a65cc3973..669269ee4` — PASS. The reopened delta touches 53
  files, and the complete exact terminal diff touches 113 files.
- An initial archive-root attempt was excluded because a shared cached test
  binary retained a deleted archive's compile-time manifest path. The rerun
  resolved that path to the exact archive and passed; this was an isolation
  setup issue, not a repository test failure.
- Not rerun by this reviewer for the narrow documentation/source-guard delta:
  full-workspace quick/full, doctests, `cargo deny`, or the complete 1,088-trial
  cohort. The previously admitted closure evidence remains applicable to
  unchanged runtime behavior; its reuse and final package disposition are
  separately owned lifecycle evidence.

## Non-Blocking Debt And Follow-Ups

- `runoff.rs` retains parallel generic-error and typed-WB16 validation/closure
  adapters around a shared hourly-depth assembler. Consolidate the semantic
  checks behind one typed operator so transfer-shape and peak consumers cannot
  drift while preserving caller-appropriate error conversion.
- The new real-span pure-melt test ends at R4K and composes with separate R4A /
  WB16 peak tests. A single R4I-through-R7D6 test would make that cross-span
  proof easier to diagnose if future orchestration changes break the chain.
- `test_valid_record_reuses` exercises `record_matches`, not the actual
  `run_case(..., resume=True, ...)` bypass. Add a subprocess-spy test proving a
  valid receipt skips execution and every mismatch executes anew.
- `DISCOVERED_SIDECARS` duplicates the runner contract in Python. Add a parity
  test or canonical resolved-input manifest so a new runner sidecar cannot
  silently escape future receipts.
- The p61/p102 joins use the complete year/Julian key but select `max_by` rather
  than asserting exactly one matching public row. Assert uniqueness so a
  duplicate-day publication cannot be masked.
- The contract/ADR authority test is intentionally lightweight source
  inspection. Required ADR markers can occur outside D4 or Alternative 4, and
  retired markers are sensitive to harmless wrapping changes. A small Markdown
  section extractor would bind each assertion to the authoritative section and
  make future failures easier to diagnose. The exact former contradictions are
  nevertheless guarded today.
- Retained `ealpha` manifest fields are historical schema debt. A future schema
  version should remove the dormant compatibility boolean and the helper's
  boolean parameter rather than carry an impossible state indefinitely.
- On census batch failure, canceling futures does not terminate already running
  subprocesses or emit a terminal partial-run receipt.
- Before the reopened package returns to terminal disposition, its lifecycle
  artifacts must bind `669269ee4`, record the ADR guard receipt and proportional
  re-reviews, and explicitly disposition reuse of the previously admitted
  runtime/campaign evidence.

## QA Verdict

`PASS — ACCEPTABLE FOR ADR-GUARD QA.` ADR-0036, canonical SC-* authority, the
source guard, and focused evidence agree at
`669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`. The guard's section-insensitive
implementation is maintainability debt, not a present coverage blocker: it
requires the corrected native/legacy authority markers and rejects every exact
former contradiction. Final lifecycle evidence and package disposition remain
separately owned closure obligations.
