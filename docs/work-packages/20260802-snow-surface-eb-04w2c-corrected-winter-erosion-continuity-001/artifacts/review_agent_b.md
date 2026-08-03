# Review B

Status: **FINDINGS — QA HOLD**

Evidence mode: **Static + Ran**

## Findings

### HIGH — B-01: the contract-derived test does not distinguish Simpson integration from the removed trapezoid implementation and omits required boundary cases

Evidence:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/tests/erosion_hb04.rs:61-70`
  tests only linearly spaced rates. Composite trapezoid produces the same stated
  `0.04`, `0.075`, and `0.175` results for all three vectors, so a regression
  back to trapezoids would pass.
- The one-interval fallback at
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs:420-422`
  has no test even though `SC-SED-001.md:146` makes it the only permitted
  trapezoid case.
- The test calls only `wave1_integrate_rate_block`; it does not exercise the
  nonoverlapping block partition at `erosion_continuity.rs:443-466` or the
  region/clamp segmentation at `erosion_continuity.rs:468-499`. Consequently,
  overlap, omission, incorrect odd-tail placement, and region/clamp crossing
  defects can pass the named contract test.
- `artifacts/contract-test-evidence.md:7-17` nevertheless represents the test
  as coverage of the matched-order contract.

Closure requirement: use curved polynomial vectors for which Simpson and
trapezoid differ; directly cover one, two, three, four, five, and longer odd
and even interval runs; and add synthetic-grid tests proving nonoverlap and
correct splitting at region and clamp boundaries. The pre-implementation
compile failure proves only that a helper name was absent, not that the
required behavior was red.

### HIGH — B-02: the claimed independent conservation reconstruction is the same-operand producer identity and does not satisfy anti-tautology acceptance

Evidence:

- Production derives boundary loads and positive/negative totals from the same
  `grid.load` sequence and checks
  `(exported - inflow) - (detachment - deposition)` at
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs:2219-2247`.
- The EROD16 test repeats that exact formula over the returned state fields at
  `tests/integration/erod16_wave1_continuity_fixture_conservation.rs:461-475`.
  Its deposition-debit bound at lines 477-485 is an algebraic restatement of
  the same identity for this zero-inflow fixture, not an independent operand
  lineage.
- `artifacts/operand-lineage.md:16-20` requires independent reconstruction of
  boundary loads and signed per-cell changes, but the test exposes or rebuilds
  no per-cell changes. `artifacts/independent-conservation-reconstruction.md:7-22`
  overclaims this same-operand check as independent acceptance.

Closure requirement: retain an independently reconstructable per-cell or
produced-output ledger for the fixture, rebuild signed changes and boundary
loads from a lineage distinct from `wave1_totals`, and include rejected-formula
checks whose values differ for plausible aliases. The existing identity remains
useful sanity evidence but cannot carry the package's anti-tautology gate.

### HIGH — B-03: canonical `SC-SED-001` still states the superseded algorithm and tolerance mapping

Evidence:

- The canonical invariant row at
  `docs/specifications/science-contracts/contracts/SC-SED-001.md:135` still
  calls the diagnostic "trapezoid-vs-RK4" and incorrectly calls
  `TOL-SED-005` the telescoping mass gate.
- A later addendum at `SC-SED-001.md:138-150` says that text is superseded,
  leaving two contradictory descriptions in the same canonical authority
  instead of correcting the invariant itself.
- The kernel profile requires changed behavior to be incorporated into the
  algorithm specification, branch/guard mapping, and test-vector obligations
  (`docs/specifications/science-contracts/kernel-process-contract-profile.md:60-103`).
  Revision 56 adds a sidecar paragraph and tolerance rows, while the Wave-1
  algorithm/test-vector section at `SC-SED-001.md:325-378` contains no W2C
  block/degenerate-case obligations.
- `artifacts/kernel-profile-compliance-checklist.md:7-16` marks a generic pass
  but does not disposition the profile's required algorithm, guard, and
  test-vector items.

Closure requirement: replace the stale `INV-SED-016(f)` text in place, bind the
new algorithm and typed refusal in the owning algorithm/guard sections, add
explicit even/odd/single/region/clamp test-vector obligations, and reconcile
the compliance checklist against every mandatory profile item.

### MEDIUM — B-04: retained validation evidence lacks the minimum exact-command and source identity needed for a Critical package

Evidence:

- `artifacts/gate-results.md:7-26` records labels, counts, and log names but not
  exact argv, working directory, source commit plus dirty-tree identity, or
  fixture/config identities. The retained logs show tool output but generally
  do not contain the invoking command; `logs/16-fmt-check.log` is empty.
- `artifacts/owned-file-manifest.md:7-18` hashes selected changed files, but it
  does not identify the repository commit/complete dirty state against which
  the broad workspace runs executed.
- The canonical minimum is explicit at
  `docs/standards/testing-and-gate-strategy.md:294-301`, and increment closure
  requires exact commands/results at lines 139-155.

Closure requirement: record exact argv, `/home/workdir/openWEPP` as working
directory, base commit and dirty-tree identity, relevant fixture/config hashes,
duration/exit status, and log mapping for each accepted run. Terminal
verification may supply fresh exact-head evidence, but the present summaries
alone are insufficient provenance for the technical-pass claim.

### MEDIUM — B-05: current authorization and package disposition artifacts contradict one another

Evidence:

- `package.md:124-134`, the active prompt, and `package.md:175-176` explicitly
  authorize the four delegated review/verification roles.
- The same package's decision log at `package.md:200-203`,
  `artifacts/disposition.md:19-22`, and
  `artifacts/kernel-profile-compliance-checklist.md:19-21` state that the
  package/prompt do not contain that authorization.
- `artifacts/worker-handoff.md:5-8` still directs the next increment to obtain
  authorization that already exists.

Closure requirement: preserve the earlier lack of authorization as dated
historical context if useful, but reconcile all current-status and handoff text
to the authorization now present before terminal verification.

## Non-blocking debt and follow-ups

- Static inspection found the implementation's current even-pair, final
  three-interval odd-tail, single-interval fallback, and contiguous
  region/clamp selection coherent. This does not waive B-01's regression-test
  gap.
- Independent read-only parsing of `storm-partition.csv` reproduced all eight
  transition counts and the `37/227` prior versus `61/231` corrected refusal
  populations. `logs/08-erod16-terminal-focused.log` contains exactly four
  explicit current refusals and a passing `227/231` accepted population.
- The exact working tree stays within the declared write set. Current file
  hashes match `owned-file-manifest.md`; `git diff --check` passes.
- Line counts independently match the checklist: `erosion_continuity.rs` is
  2639 lines, below the 3000-line hard threshold, and its warning rationale and
  follow-on split intent are adequate.
- Roadmap, snow-roadmap, and package-catalog entries truthfully say technical
  pass with independent review pending; none claim package completion. They
  must be revisited if accepted findings change that disposition.
- No manifest, lockfile, dependency, or feature-resolution file changed, so
  the recorded `cargo deny check` non-applicability is reasonable. Assurance
  files are unchanged, and the retained assurance plan/validation logs pass;
  no assurance publication or approval is claimed.

## Checks run by Review B

- `git diff --check` — PASS.
- `wc -l` on the four touched Rust/test files — matched the retained
  line-count artifact.
- Focused Nextest for
  `eb04w2c_matched_order_flux_quadrature_covers_even_and_odd_blocks` — PASS
  (`1/1`), while static mutation-equivalence analysis produced B-01.
- Read-only CSV reconstruction and retained-log cross-check — population and
  refusal counts matched the package artifacts.
- SHA-256 comparison of the retained owned-file set — matched the manifest.

## QA verdict

Review B does not pass this increment. B-01 through B-03 are closure-blocking
test/authority/anti-tautology deficiencies; B-04 and B-05 must also be
dispositioned before terminal verification and formal package closure.

## Fresh Re-review — 2026-08-02

Status: **FINDINGS — QA HOLD**

Evidence mode: **Static + Ran**

### Findings

#### HIGH — FR-B-01: revision 57 still permits diagnostic quadrature across an unrecorded non-grid sub-march boundary

The point-owned zone representation does not prove that the whole interval
ending at that point belongs to the recorded zone:

- `erosion_continuity.rs:393-397` describes `diagnostic_zone[i]` as the
  provenance of the interval ending at point `i`, and
  `erosion_continuity.rs:482-520` treats any nonzero ending-point zone as an
  eligible interval when region and clamp checks pass.
- A new detachment march starting off-grid advances its first grid point from
  the exact `xb`/`ldlast` state (`erosion_continuity.rs:1200-1204`) and then
  assigns the new zone to that point (`erosion_continuity.rs:1231-1234`). The
  analytic-deposition path likewise starts a new zone and assigns it to its
  first committed grid point (`erosion_continuity.rs:1053-1112`).
- For the test's segment boundary at `x = 0.015`, the interval ending at grid
  point `x = 0.02` therefore represents the load change from `x = 0.01` to
  `x = 0.02`, crossing the old/new sub-marches at `x = 0.015`, but is marked
  wholly as the new nonzero zone. Because both endpoints remain in the same
  detachment region and are unclamped, `wave1_flux_closure` includes that
  straddling interval.
- `erosion_hb04.rs:150-240` asserts only that the endpoint zones differ. It
  never invokes `wave1_flux_closure` or proves the straddling interval is
  excluded. Its passing assertions therefore mask rather than prevent this
  failure.

This contradicts `SC-SED-001.md:140-158`, its algorithm at lines 367-378, and
test-vector 9 at lines 404-406. It leaves `A-001` and the boundary portion of
`B-01` unresolved. It also means the numerator and denominator use the same
*selected* interval population, but that population is not the contract's
eligible population: the non-grid seam is included in both.

Closure requirement: represent interval eligibility/provenance explicitly and
exclude a first interval that straddles an off-grid sub-march start, or retain
the split contributions needed to integrate it without crossing the seam. Add
detachment and deposition tests that call the closure builder at non-grid
segment and critical-shear boundaries and assert both residual and scale omit
or correctly split the crossing interval.

#### HIGH — FR-B-02: the corrected per-cell test still does not independently reconstruct or verify the published boundary operands

The EROD16 correction reconstructs positive and negative cell deltas from
`state.load` and compares them with the producer's detachment/deposition totals
(`erod16_wave1_continuity_fixture_conservation.rs:461-500`). That is useful new
coverage, and lines 502-517 reject a detachment-only alias on depositing days.
However:

- `reconstructed_boundary_kg_m` is only the endpoint difference from the same
  load vector. The residual at lines 475-488 is the exact telescoping identity
  of the preceding window sum and is therefore zero for any finite trajectory.
- The test never reconstructs `load.last() * denorm` and `load[0] * denorm`
  separately and compares them with `state.exported_sediment_kg_m` and
  `state.inflow_sediment_kg_m`.
- `state.exported_sediment_kg_m` appears only in the tolerance scale and a
  one-sided bound. A plausible publication defect that reports too little
  export, including zero, can satisfy both while the load trajectory and the
  two aggregate comparisons remain unchanged.

Production publishes those boundary fields separately at
`erosion_continuity.rs:2344-2350`, after deriving all four closure operands
from the grid at lines 2260-2271. Thus the corrected test does not yet provide
the claimed independent exact sediment-closure reconstruction. `A-005` and
`B-02` are only partially corrected.

Closure requirement: reconstruct the top and toe boundary loads separately
from the normalized trajectory plus input denormalization, compare both with
the published inflow/export fields, and form the independently reconstructed
closure from those endpoints and signed cell changes. Retain the depositing
alias rejection and add a rejected wrong-boundary-output form.

#### MEDIUM — FR-B-03: focused tests do not exercise production long-run partitioning or the returned eligible scale

The curved helper vectors at `erosion_hb04.rs:62-85` now distinguish Simpson
from trapezoid for one through seven intervals. The closure-builder test at
lines 87-122, however, exercises only one two-interval run and one
three-interval run. Production partitions a longer run into successive
two-interval blocks and possibly a final three-interval block at
`erosion_continuity.rs:457-479`; no test drives the closure builder through a
four-, five-, longer-even, or longer-odd run, nor through its actual
one-interval fallback. A partition regression could therefore pass every
helper-vector assertion. The same test discards `flux_scale` in both calls, so
the claimed identical eligible numerator/denominator population has no direct
behavioral assertion even for its region/clamp cases.

Closure requirement: exercise `wave1_flux_closure` itself over actual
one-through-five, longer-even, and longer-odd eligible runs with curved rates,
and assert exact residual and scale changes when seams, regions, and clamps are
introduced.

### Finding-by-finding correction status

| Finding | Fresh status | Basis |
|---|---|---|
| `A-001` | **FAIL / HOLD** | Point-owned zones still admit the off-grid straddling interval described in `FR-B-01`. |
| `A-002`, `B-01` | **PARTIAL / HOLD** | Curved helper, bad-load, and consumer vectors were added, but non-grid exclusion is false and production long/single partition plus scale behavior remains uncovered. |
| `A-003` | **PASS** | The hourly production fold's solver seam proves only `erosion.wave1.flux_closure` becomes zero sediment plus one refusal; publication closure remains a typed hard error (`erosion_hb04.rs:242-273`). |
| `A-004`, `B-03` | **DOCUMENT CORRECTION PASS; CONFORMANCE HOLD** | Revision 57 corrects the invariant, tolerance, guard/consumer, algorithm, and vector authority surfaces. The implementation does not yet satisfy its non-grid-boundary rule (`FR-B-01`). |
| `A-005`, `B-02` | **PARTIAL / HOLD** | Per-cell sign reconstruction and detachment-only alias rejection are present, but the independent boundary-output reconstruction remains absent (`FR-B-02`). |
| `B-04` | **PASS** | Review-correction evidence records exact argv, working directory, base/diff/config/fixture identities, result, duration, and log mapping. Broad pre-correction runs are truthfully labeled historical and pending renewal. |
| `B-05` | **PASS** | Package, disposition, checklist, and handoff now consistently recognize the explicit delegated-review authorization; stale contrary text is historical only. |
| Review A eligible-scale residual | **PARTIAL / HOLD** | Numerator and denominator iterate the same selected slices, but the off-grid crossing interval is wrongly selected and tests do not assert the returned scale. |

### Checks run for the fresh re-review

- `cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(eb04w2c)'`
  — PASS, `5/5`.
- `cargo nextest run --test erod16_wave1_continuity_fixture_conservation --no-capture`
  — PASS, `1/1`; 227 clean/depositing solves and four named refusals.
- An initial exploratory invocation incorrectly supplied a nonexistent package
  feature (`legacy-comparator`) and exited `101` before compilation. It is not
  accepted as gate evidence; the exact recorded integration command above was
  then run successfully.
- `git diff --check` — PASS.
- Retained logs 21-24 agree with the exact commands and summaries in
  `gate-results.md`. Current production/test/contract hashes agree with
  `owned-file-manifest.md`, and the recorded result-affecting diff identity was
  independently reproduced.
- HEAD remains `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`. The nine tracked
  modifications plus the untracked package tree remain within the declared
  write set; no manifest, lockfile, dependency, feature, fixture, forcing,
  observation, or assurance source changed.

### Non-blocking debt and follow-ups

- Revision 57 is structurally complete as contract text, and the named error
  and hourly-consumer tests are adequate for their claimed behavior.
- The line-count artifact remains exact (`2675`, `1284`, `908`, and `648`);
  the large-module warning rationale is reasonable for this bounded repair.
- Roadmap, snow-roadmap, and catalog wording currently says fresh review is
  pending and does not claim package closure. Following this fresh-review
  HOLD, those current-status surfaces and the correction disposition must be
  reconciled before another review. The catalog's “revisions 56–57 separates”
  should also be corrected to “separate” when that authorized update occurs.
- `cargo deny check` remains not applicable because dependency and feature
  resolution surfaces are unchanged. The broader gates correctly remain
  historical and should not be renewed until the findings above are corrected
  and freshly reviewed.

### Fresh QA verdict

**HOLD.** B-04 and B-05 pass, revision 57's authority structure is corrected,
and the focused tests run cleanly. The remaining off-grid interval-provenance
defect and incomplete independent boundary reconstruction are closure-blocking;
terminal verification and W2B resumption must not proceed on this tree.

## Second Fresh Re-review — 2026-08-02

Status: **FINDINGS — QA HOLD**

Evidence mode: **Static + Ran**

### Findings

#### MEDIUM — SFR-B-01: excluded-seam scale behavior remains asserted only in prose

The implementation correction is coherent: an off-grid sub-march's first
ending interval receives zone zero at `erosion_continuity.rs:1115-1121` and
`erosion_continuity.rs:1243-1249`; `wave1_flux_closure` skips that interval at
lines 485-503 and accumulates both residual and scale from the same accepted
slice at lines 505-523. The actual segment, critical-shear, and deposition
tests now prove the crossing point is zero-zone and the next complete interval
is eligible (`erosion_hb04.rs:188-280`).

The remaining test claim is not executable, however. The synthetic seam test
says its deliberately large delta is excluded from both residual and scale at
`erosion_hb04.rs:102-110`, but both closure calls discard the scale at lines
117 and 122. The new single-/seven-interval test at lines 126-160 asserts scale
only when every interval is eligible. Consequently, a regression that adds
zone-zero, clamped, or region-crossing deltas to the denominator while keeping
them out of the numerator would still pass all six focused tests. That is the
exact dilution mode covered by the earlier eligible-scale finding and the
explicit closure requirement in `FR-B-03`.

Closure requirement: assert the exact scale from the synthetic seam test
before and after its clamp/region mutations, proving the two large excluded
deltas do not change the denominator. Retain the current one- and seven-
interval production-partition assertions.

#### MEDIUM — SFR-B-02: post-correction source identity and exact gate provenance are stale

The retained logs support the reported outcomes, and this reviewer reproduced
them, but the evidence package does not bind those outcomes to the current
second-correction tree:

- `artifacts/gate-results.md:41-59` still records result-affecting diff identity
  `2713141c...` and an exact-command table ending at logs 21-24. The current
  result-affecting diff identity is
  `f9aaa46fcb97ac27dd142db5d62099c74a079e38e78da7305dac9d0c326d0262`;
  exact argv, durations, and source identity for final logs 28-30 are absent.
- `artifacts/owned-file-manifest.md:11-15` retains pre-second-correction hashes.
  For example, current `erosion_continuity.rs` is `78db79e8...`,
  `erosion_hb04.rs` is `001d9848...`, the EROD16 test is `c194c3d3...`, and
  `SC-SED-001.md` is `3ad30c32...`; none matches its recorded value.
- `artifacts/line-count-governance-checklist.md:9-12` reports
  `2675/1284/908/648`; the current values are `2690/1284/949/660`.
- `artifacts/review-disposition.md:25-28` maps the second corrections to logs
  25-27, although log 25 ran only five focused tests. The final six-test,
  boundary-ledger, and clippy evidence is in logs 28-30.

This reopens `B-04`: exact validation provenance was correct for the first
review-correction tree but was not reconciled after the result-affecting seam,
test, integration, and contract changes.

Closure requirement: refresh the result-affecting diff identity, owned-file
hashes, exact argv/duration/log table for 28-30, line counts, and correction-log
mapping on one stable source tree. Reconcile the independent-conservation
artifact, handoff, snow roadmap, disposition, and catalog text that still
describe five tests or omit the now-verified boundary projections.

### Finding-by-finding status

| Finding | Second fresh status | Basis |
|---|---|---|
| `A-001`, `FR-B-01` | **PASS** | Off-grid first intervals now receive zone zero; actual segment, critical-shear, and deposition cases prove the next full interval resumes with a nonzero new zone. The `32 * f64::EPSILON` dimensionless alignment rule is explicit in code and revision 57. |
| `A-002`, `B-01` | **PARTIAL / HOLD** | Curved helper vectors, actual single-/seven-interval closure partitioning, large-seam residual exclusion, bad-load rejection, and consumer behavior pass. Excluded-seam scale anti-evasion remains absent (`SFR-B-01`). |
| `A-003` | **PASS** | The production hourly fold still converts only the named flux diagnostic to zero sediment plus one refusal; publication closure remains hard-fail. |
| `A-004`, `B-03` | **PASS** | Revision 57 consistently binds the corrected invariant, numerical authority, `32 * f64::EPSILON` alignment rule, algorithm, guard/consumer map, tolerances, and test vectors. Runtime behavior now conforms to the non-grid rule. |
| `A-005`, `B-02`, `FR-B-02` | **PASS** | EROD16 separately projects inflow and toe export from endpoint loads, reconstructs signed cell totals, compares all four published operands, and rejects the detachment-only alias (`erod16_wave1_continuity_fixture_conservation.rs:461-529`). |
| `B-04` | **FAIL / HOLD** | Final logs pass, but their exact argv and current source identity are not recorded; manifest, line-count, and correction-log mappings are stale (`SFR-B-02`). |
| `B-05` | **PASS** | Current authorization state remains consistently reconciled; the prior absence is historical only. |
| Review A eligible-scale residual, `FR-B-03` | **STATIC PASS; TEST-EVIDENCE HOLD** | Numerator and denominator now use identical correct slices, and actual one-/seven-interval partitioning is tested. The excluded seam's returned scale is still discarded. |

No new production correctness defect was found in the second correction. The
remaining findings concern required behavioral anti-evasion and exact evidence
reconciliation for this Critical package.

### Checks run for the second fresh re-review

- `cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(eb04w2c)'`
  — PASS, `6/6`.
- `cargo nextest run --test erod16_wave1_continuity_fixture_conservation --no-capture`
  — PASS, `1/1`; 227 clean/depositing solves and four explicit refusals.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
  — PASS.
- `cargo fmt --all -- --check` — PASS.
- `git diff --check` — PASS.
- Retained logs 28-30 agree with the six-test, EROD16, and clippy outcomes.
  The current result-affecting diff hash, individual file hashes, and line
  counts were independently recomputed and produced `SFR-B-02`.
- HEAD remains `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`. The nine tracked
  modifications and untracked package tree remain inside the declared write
  set. No dependency, feature-resolution, fixture-forcing, observation, or
  assurance source changed; `cargo deny check` remains not applicable.
- Broad quick, frost, erosion, full-workspace, assurance, Markdown, and doctest
  runs were not renewed; the package correctly labels them historical pending
  fresh acceptance.

### Non-blocking debt and follow-ups

- The alignment threshold has useful code and contract documentation, but a
  focused vector immediately inside and outside `32 * f64::EPSILON` would make
  its classification boundary more regression-resistant.
- `Wave1RouteGrid::clamped` still describes “trapezoid flux-residual
  accounting” at `erosion_continuity.rs:392-394`; update it to matched-order
  diagnostic terminology during the authorized evidence correction.
- The roadmap and catalog continue to hold W2B and avoid a completion claim,
  but their five-test wording and “revisions 56–57 separates” grammar should be
  corrected when `SFR-B-02` is dispositioned.

### Second fresh QA verdict

**HOLD.** The two previous high findings are technically corrected, revision
57 and the consumer behavior now conform, and all rerun focused gates pass.
Fresh acceptance still requires a direct excluded-seam scale assertion and
provenance/artifact reconciliation against the exact post-correction tree.

## Third Fresh Re-review — 2026-08-02

Status: **FINDINGS — QA HOLD**

Evidence mode: **Static + Ran**

### Finding

#### MEDIUM — TFR-B-01: current status and conservation narratives are not fully reconciled to the seven-test tree

The material portions of `SFR-B-02` are corrected. The current five-file
result-affecting diff independently hashes to
`a41615fc0a673ca23b70de45e09b6b8a8b2cdfa32e2ce1ba0ac5059c5d9fb176`;
the individual hashes in `owned-file-manifest.md` match; line counts are
exactly `2698/1284/969/660`; and `gate-results.md:42-65` records the base,
config/fixture identities, exact commands, durations, and logs 31-33.

Several current narrative surfaces named in the second-review closure
requirement remain stale:

- `artifacts/disposition.md:14-18` still says five focused W2C tests pass,
  although the accepted current suite contains seven.
- `docs/planning/snow-surface-energy-balance-roadmap.md:158` likewise reports
  five focused tests in the current W2C row.
- `artifacts/independent-conservation-reconstruction.md:12-23` documents the
  endpoint delta and the two producer aggregates but omits the separately
  reconstructed inflow and export comparisons now performed at
  `erod16_wave1_continuity_fixture_conservation.rs:475-512`.
- `artifacts/review-disposition.md:10` describes six focused tests in its
  finding row. Its later chronology correctly identifies the final seven-test
  suite, but the row is not labeled as the intermediate six-test state.

These are understatements rather than a false completion or physics claim:
the main roadmap and handoff still hold W2B, and the contract-test evidence
correctly describes seven tests. Nevertheless, a Critical work package's
current disposition and campaign row must match its accepted exact evidence;
this leaves the narrative-reconciliation portion of `SFR-B-02` incomplete.

Closure requirement: update the current disposition and snow-roadmap to seven
tests, record the separate inflow/export boundary projections in the
independent-conservation evidence, and make the six-test review-disposition row
explicitly historical or current. Correct the catalog's “revisions 56–57
separates” grammar in the same authorized documentation reconciliation.

### Finding-by-finding status

| Finding | Third fresh status | Basis |
|---|---|---|
| `A-001`, `FR-B-01` | **PASS** | Off-grid detachment and analytic-deposition first intervals receive zone zero; actual segment and critical-shear cases prove the next complete interval resumes eligibility. |
| `A-002`, `B-01`, `SFR-B-01` | **PASS** | Curved one-through-seven helper vectors, actual one-/seven-interval closure partitioning, exact seam/clamp/region scale exclusion, alignment edges, injected bad load, and hourly consumer behavior are executable and pass. |
| `A-003` | **PASS** | Only `erosion.wave1.flux_closure` is converted to zero sediment plus a refusal count; publication closure remains hard-fail. |
| `A-004`, `B-03` | **PASS** | Revision 57 and runtime consistently bind the matched-order diagnostic, numerical authority, alignment threshold, boundary provenance, guards, tolerances, and consumer semantics. |
| `A-005`, `B-02`, `FR-B-02` | **PASS** | EROD16 reconstructs both endpoint loads and signed per-cell changes, compares inflow/export/detachment/deposition, and rejects the detachment-only alias. |
| `B-04`, `SFR-B-02` | **PARTIAL / HOLD** | Source identity, file hashes, line counts, exact argv/durations, and logs are now exact. Current status/conservation narratives remain unreconciled (`TFR-B-01`). |
| `B-05` | **PASS** | Authorization state remains consistent and current. |
| Review A eligible-scale residual, `FR-B-03` | **PASS** | Numerator and denominator use identical eligible slices; the injected `10.0` seam and clamp/region delta are now explicitly absent from the asserted scale. |

No new Rust correctness, numerical-contract, error-disposition, consumer, or
test-robustness finding was identified on the third tree.

### Checks run for the third fresh re-review

- `cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(eb04w2c)'`
  — PASS, `7/7`.
- `cargo nextest run --test erod16_wave1_continuity_fixture_conservation --no-capture`
  — PASS, `1/1`; 227 clean/depositing solves and four explicit refusals.
- `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`
  — PASS.
- `cargo fmt --all -- --check` — PASS.
- `git diff --check` — PASS.
- Retained logs 31-33 agree with the rerun outcomes. Current source/contract
  hashes, result-affecting diff identity, Nextest/fixture identities, and line
  counts independently match the updated evidence artifacts.
- HEAD remains `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`. The nine tracked
  modifications and untracked W2C package remain within the declared write
  set. No dependency, feature, fixture forcing, observation, or assurance
  source changed; `cargo deny check` remains inapplicable.
- Broad quick, frost, erosion, full-workspace, assurance, Markdown, and doctest
  runs remain correctly labeled historical pending fresh acceptance and were
  not rerun in this review.

### Non-blocking debt and follow-ups

- The `Wave1RouteGrid::clamped` comment still says “trapezoid flux-residual
  accounting” at `erosion_continuity.rs:392-394`; matched-order diagnostic
  terminology would prevent future reader confusion.
- The current roadmap/catalog correctly avoid completion and keep W2B held;
  no queue-order correction is needed beyond `TFR-B-01`'s evidence wording.

### Third fresh QA verdict

**HOLD.** The implementation, revision 57 contract, tests, exact identities,
and focused gates now pass every technical review requirement. Fresh acceptance
is withheld only until the remaining current disposition, roadmap, and
conservation-evidence narratives are reconciled to the exact seven-test tree.

## Final Review Closure — 2026-08-02

Status: **QA PASS**

Evidence mode: **Static + retained/live Ran evidence**

### Findings

No blocking or new findings.

### Closure reconciliation

`TFR-B-01` is resolved:

- `artifacts/disposition.md:15-19` and the current snow-roadmap W2C row report
  seven focused tests and retain the fresh-review/terminal hold;
- `artifacts/review-disposition.md:9-30` uses seven in the current finding row
  and labels the five-test and later correction sequence as historical;
- `artifacts/independent-conservation-reconstruction.md:12-25` now records the
  separate first/last-load projections that verify published inflow and export,
  in addition to the signed per-cell ledger and producer aggregates;
- `artifacts/gate-results.md:33-35` includes logs 31-33 in its top result table,
  while lines 45-68 retain their exact identity and command provenance; and
- `artifacts/implementation-evidence.md:30-31` names logs 31-33 as the final
  review-correction evidence.

The result-affecting tree is unchanged from the accepted third-review checks:
the five-file diff is
`a41615fc0a673ca23b70de45e09b6b8a8b2cdfa32e2ce1ba0ac5059c5d9fb176`;
all owned hashes match; and line counts remain `2698/1284/969/660`.

### Final finding status

| Finding family | Final status |
|---|---|
| `A-001`, `FR-B-01` — non-grid sub-march provenance | **PASS** |
| `A-002`, `B-01`, `FR-B-03`, `SFR-B-01` — nonlinear, partition, boundary, scale, error, and consumer tests | **PASS** |
| `A-003` — real hourly refusal consumer | **PASS** |
| `A-004`, `B-03` — revision 57 authority and runtime conformance | **PASS** |
| `A-005`, `B-02`, `FR-B-02` — independent boundary/per-cell reconstruction and alias rejection | **PASS** |
| `B-04`, `SFR-B-02`, `TFR-B-01` — validation provenance and evidence reconciliation | **PASS** |
| `B-05` — authorization-state reconciliation | **PASS** |
| Review A eligible-scale residual | **PASS** |

### Final checks

- Inspected the exact current narrative corrections, logs 31-33 mappings,
  owned hashes, result-diff identity, and line counts — PASS.
- `git diff --check` — PASS.
- No executable Rust gate was rerun for this narrative-only closure. The
  result-affecting hashes are unchanged from this reviewer's preceding live
  `7/7` focused, `1/1` EROD16, warnings-denied clippy, and format passes, and
  they match retained logs 31-33.
- The nine tracked modifications and untracked W2C package remain within the
  declared write set. Dependency, feature, forcing, fixture, observation, and
  assurance-source exclusions remain intact; `cargo deny check` remains
  inapplicable.

### Non-blocking debt

- `Wave1RouteGrid::clamped` still uses the historical phrase “trapezoid
  flux-residual accounting”; matched-order diagnostic wording would be clearer.
- The work-package catalog's “revisions 56–57 separates” should be corrected to
  “separate” during terminal documentation cleanup. Neither item changes
  behavior, evidence identity, status, or review acceptance.

### Final QA verdict

**PASS.** Review B accepts the exact current correction tree. This is fresh
review acceptance, not formal package completion: renewed broad gates and both
authorized terminal verifications remain required before W2C closes or W2B
resumes.

## Revision 58 Correction Review — 2026-08-02

Status: **QA HOLD — terminal provenance and mandatory profile detail remain
incomplete**

Evidence mode: **Static + Ran**

### Findings

#### HIGH — R58-B-01: the accepted terminal diff/status/lint corrections are not yet recorded for the current tree

The tracked portion of `artifacts/terminal-diff-reconciliation.md:13-32` is
current and reproducible: nine tracked paths, 686 insertions, 57 deletions,
complete tracked-diff identity `fffa6be6...1ea`, four-file runtime/test identity
`ada609e0...ee`, and five-file runtime/test/contract identity
`1e7eb2ba...00` all match this reviewer's live reconstruction.

The untracked and Markdown portions remain prospective, however. The same
artifact claims a 76-file package tree at lines 34-38 and defers the sorted-path
identity until after review. The live package contains 79 files because logs
45-47 are now present. `artifacts/terminal-markdown-scope.md:3,15-20` still says
the post-revision-58 run is required and promises a future document-root
identity. `artifacts/gate-results.md:35-51,69-89` stops at log 44 and therefore
does not record logs 45-47, their exact commands, or a current documentation-
root identity. Log 47 reports 35 clean files, but it was produced before this
fresh-review append and contains only the summary outcome.

Thus `VA-001`, `VB-02`, and `VB-04` have been dispositioned, but their required
final evidence is not yet complete. The imperative/future-tense actions in
`artifacts/verification-disposition.md:14,17,19` accurately describe work still
to do; they do not prove it was done.

Required correction: after both revision-58 reviewers finish, record the exact
dirty status and complete untracked package inventory (including count and
reproducible sorted-path identity), renew the complete tracked diff identity and
line counts, rerun the documented Markdown scope plus `git diff --check`, and
record exact cwd, argv/path scope, exit, selected-file count, log mapping, and a
reproducible documentation-root identity. Update the disposition verbs and
gate summary to completed actions before terminal re-verification.

#### MEDIUM — R58-B-02: the profile checklist overstates pre/postcondition and unit-conversion coverage

Revision 58 materially improves `SC-SED-001`: the state surfaces, ordered
diagnostic steps, branch/guard map, constants, tolerances, calibration posture,
and readiness matrix are coherent. The explicit
`CALIBRATION_NOT_APPLICABLE` rationale is appropriately narrow and does not
claim empirical validation.

Two mandatory profile details remain asserted rather than documented. The
checklist says preconditions/postconditions are explicit at
`artifacts/kernel-profile-compliance-checklist.md:14-16`, but the cited
contract section (`SC-SED-001.md:122-155`) lists aggregate inputs/outputs and a
five-step sequence without explicit step-local preconditions or postconditions.
It covers the one-interval fallback but does not disposition the zero-eligible-
interval degenerate case, for which the implementation returns `(0, 0)`.

Likewise, the unit-governance row at `SC-SED-001.md:166-172` refers only to the
“existing Wave-1 normalization/denormalization” as the conversion owner. It
does not name a directional helper or code anchor, nor record an explicit gap
or inlined-conversion/raw-literal exception. That is weaker than the conversion-
helper-or-gap obligation in `kernel-process-contract-profile.md:69-73,77-85`,
despite the checklist's completed claim at lines 23-25.

Required correction: add explicit W2C diagnostic preconditions and
postconditions, state the zero-eligible-population behavior, and bind the
normalization/denormalization conversion to a named code path (or document the
permitted scalar/raw-literal exception or an explicit gap). Then make the
checklist wording match the actual authority. The profile's noncompliance rule
keeps this documentation-only omission blocking until corrected or formally
risk-accepted.

### Accepted revision-58 surfaces

- `review-disposition.md` has one separate row for all 20 retained/recovered
  review findings, with source, severity, decision, action, artifact, and
  rationale. The conservative severities for unavailable Review A history are
  explicit rather than invented as original evidence.
- `review_agent_a.md` plainly discloses that the original and first-fresh Review
  A prose was overwritten and that the reconstructed table is not verbatim.
- `verification-disposition.md` has one row for each of `VA-001`, `VA-002`, and
  `VB-01` through `VB-04`, preserving the verifier-assigned severities and
  required action families.
- The four runtime/test file hashes and line counts remain unchanged from the
  accepted revision-57 tree. Reuse of Rust gates 34-43 is therefore technically
  justified for this documentation-only revision; logs 45 and 46 additionally
  reproduce `7/7` focused and `1/1` EROD16 with four refusals from 231 storms.
- The roadmap, catalog, package, and handoff consistently keep W2C in evidence
  HOLD, W2B at its cross-domain gate, and EB-04X held. No premature campaign
  release was found.

### Checks run or reconstructed

- `cargo fmt --all -- --check` — **PASS**.
- `git diff --check` — **PASS**.
- Canonical Markdown lint on this owned review artifact — **PASS**, one file
  with zero errors or warnings; `uk2us` normalization preview is clean.
- Reconstructed HEAD, tracked status/stat, all four runtime/test hashes,
  revision-58 contract hash, four-/five-/complete-diff identities, and line
  counts — all match the recorded tracked evidence.
- Counted the current untracked package and inspected logs 45-47 — 79 files;
  focused `7/7`, EROD16 `1/1` with `4/231` refusals, and Markdown `35` files
  with zero findings. These logs do not cure `R58-B-01`'s missing final-tree
  provenance.
- Inspected the retained terminal broad-gate results against the unchanged
  runtime/test identities. Broad Rust gates were not rerun in this review.
- `cargo deny check` remains not applicable because no manifest, lockfile,
  dependency, or feature-resolution input changed.

### Non-blocking debt and follow-ups

- Clarify `terminal-diff-reconciliation.md:49-53` to say “no runtime/output
  schema change”; revision 58 intentionally adds contract-profile schema, so
  the unqualified “no schema change” phrasing is easy to misread.
- The historical `Wave1RouteGrid::clamped` comment still says “trapezoid flux-
  residual accounting.” Matched-order diagnostic terminology would better
  reflect the accepted implementation.

### Revision-58 QA verdict

**HOLD.** Finding-history recovery, one-row disposition completeness,
calibration-not-applicable posture, unchanged runtime/test identity, broad-gate
reuse, and campaign holds are acceptable. Fresh revision-58 acceptance is
withheld until `R58-B-01` and `R58-B-02` are corrected. Do not complete W2C,
resume W2B or W2A, or advance EB-04X before both fresh reviewers and both
terminal verifiers accept the final reconciled tree.

## Revision 59 Re-review — 2026-08-02

Status: **QA PASS — no blocking findings**

Evidence mode: **Static + Ran**

### Findings

None.

### Revision-58 finding closure

| Finding | Revision-59 status | Evidence |
|---|---|---|
| `R58-A-001` | **PASS** | `SC-SED-001` binds the exact ADR-0042 field names and allowed values. The readiness matrix has ten separate obligation rows using only `PASS` or `NOT_APPLICABLE`, each with an evidence path and structure-backed rationale. The unit map uses all required columns. |
| `R58-A-002` | **PASS** | The Binding Exposure Index has one row for each of the six active/historical addenda, uses canonical status/classification vocabulary, and maps active residue to existing `INV-SED-*` IDs. Both normal and strict checker modes pass with six rows fully consolidated. |
| `R58-A-003` | **PASS** | The package contains 80 files. The locale-stable sorted relative-path list reproduces SHA-256 `6bd84fb21c7fb6c5a4ca774124834ce286f7c6f4b388df3c614135a81c850b29`; review appends and the planned log-47 overwrite cannot change that path identity. |
| `R58-A-004` | **PASS for review admission; final content identity pending as planned** | `terminal-markdown-scope.md` records cwd and the exact six-path lint argv, including the full package tree. Gate results bind log 47 to that scope and explicitly defer the final post-review rerun, exit/file count, and self-excluding content-root identity. |
| `R58-B-01` | **PASS** | Exact tracked identities, 80-path inventory identity, and final-lint sequencing are now explicit. The only intentionally deferred value is the Markdown content identity that must follow both review appends. |
| `R58-B-02` | **PASS** | The revision-59 step table names each step's preconditions, postconditions, degenerate behavior, and code owner; no eligible intervals explicitly yields accepted `(0, 0)`. The unit map names `wave1_totals`, `wave1_flux_closure`, `wave1_integrate_rate_block`, and EROD16 conversion/reconstruction ownership. |

`review-disposition.md` retains one independent canonical row for each of these
six findings, preserving source, severity, decision, action, artifact, and
rationale. No revision-58 finding is silently grouped, rejected, waived, or
left without an action path.

### Identity and gate verification

- HEAD remains `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36` with nine tracked
  modifications and the single untracked W2C package tree, all inside the
  declared write set.
- Live tracked reconciliation reproduces 706 insertions and 57 deletions,
  complete diff `96ee4311...f09`, unchanged four-file runtime/test diff
  `ada609e0...4ee`, and revision-59 five-file runtime/test/contract diff
  `c9dc4e98...64f`.
- Runtime/test hashes remain `b95bb390...b5037`, `869f9f33...527a`,
  `ae76c95e...a52f2`, and `c194c3d3...17c3`; line counts remain
  `2698 / 1284 / 969 / 660`. `SC-SED-001` is revision 59 at
  `299470cc...bec4`. Broad Rust-gate reuse is therefore valid.
- Log 45: focused W2C **PASS**, `7/7`, 428 skipped.
- Log 46: EROD16 **PASS**, `1/1`, with the same explicit four refusals from 231
  storms and 227 depositing solves.
- Log 47: pre-final scoped Markdown lint **PASS**, 35 files and zero findings.
  The required final overwrite/content-root renewal remains correctly scheduled
  after both revision-59 review appends.
- Log 48: Binding Exposure Index **PASS**, six rows fully consolidated. A live
  rerun of both the normal and `--strict` commands also passed.
- `git diff --check` and the current exact scoped Markdown command pass.
- `cargo deny check` remains not applicable because no manifest, lockfile,
  dependency, or feature-resolution input changed.

### Lifecycle and claim posture

The roadmap, catalog, package, disposition, and handoff continue to report W2C
as a technical pass with verification evidence held. W2B remains at
`HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`; W2A's earlier rerun remains ineligible;
and EB-04X remains held after W2B. Revision 59 does not release any campaign
gate or expand EROD16 from produced-operand accounting into empirical process
validation.

### Non-blocking debt and follow-ups

- During the already-required final Markdown/root renewal, normalize current
  lifecycle prose that still says “revision 58” in
  `terminal-markdown-scope.md`, `owned-file-manifest.md`,
  `terminal-diff-reconciliation.md`, `worker-handoff.md`, and the profile
  checklist. The recorded revision-59 hashes, status, path identity, and gate
  sequence are unambiguous, so this wording cleanup does not reopen a finding.
- Clarify “no schema change” in terminal reconciliation as “no runtime/output
  schema change”; revisions 58–59 intentionally added contract-profile schema.

### Revision-59 QA verdict

**PASS.** Review B accepts revision 59 and closes `R58-A-001` through
`R58-A-004` plus `R58-B-01/02`. This is fresh-review acceptance, not formal W2C
completion. Renew log 47 and the self-excluding Markdown content-root identity
after both review appends, then obtain both required terminal-verifier passes
before completing W2C or resuming W2B.

## Revision 60 Final Review — 2026-08-02

Status: **QA PASS — no blocking findings**

Evidence mode: **Static + Ran**

### Findings

None.

### Revision-59 finding closure

| Finding | Revision-60 status | Evidence |
|---|---|---|
| `R59-A-001` | **PASS** | The active EROD13 Binding Exposure row now includes `INV-SED-016` while retaining `INV-SED-001..007` and `INV-SED-013/014`. Its note explicitly identifies the `TOL-SED-007/008` sub-march/refusal residue. This matches the active EROD13 algorithm and canonical `INV-SED-016(f)` ownership. |
| `R59-A-002` | **PASS** | Terminal reconciliation, Markdown scope, owned manifest, profile checklist, science-contract index, catalog, package, and roadmaps identify revision 60 where current authority/status is material. Reconciliation now distinguishes intentional contract/profile schema edits from unchanged runtime/output schema. Final content-root evidence remains correctly post-review. |

`review-disposition.md` retains separate canonical rows for both revision-59
findings and for every earlier stable finding ID. Each row preserves source,
severity, accepted decision, action, artifact, and rationale; no finding is
silently grouped, rejected, deferred, or waived.

### Binding Exposure verification

- The index still contains exactly six active/historical addendum rows.
- EROD13's active diagnostic/refusal residue now maps to `INV-SED-016` as well
  as its existing Wave-1 invariants.
- Live normal checker: **PASS**, six rows fully consolidated.
- Live `--strict` checker: **PASS**, six rows fully consolidated.
- Revision history identifies version 60 as the semantic EROD13/`INV-SED-016`
  correction and explicitly records no runtime/output schema or behavior
  change.

### Current identity and gate evidence

- HEAD remains `a74af48b8e98f91b5d5acdebc0e2da0bf988ba36`; status contains the
  same nine tracked modifications and one untracked W2C package tree within the
  declared write set.
- Tracked reconciliation reproduces 707 insertions and 57 deletions, complete
  diff SHA-256 `95fa23bb...8fe`, unchanged four-file runtime/test diff
  `ada609e0...4ee`, and revision-60 five-file runtime/test/contract diff
  `2089324b...f80`.
- Runtime/test hashes remain exactly `b95bb390...b5037`,
  `869f9f33...527a`, `ae76c95e...a52f2`, and `c194c3d3...17c3`; line counts
  remain `2698 / 1284 / 969 / 660`. The revision-60 contract hash is
  `c0d73c88...c2c1`. Broad Rust-gate reuse remains justified.
- The package still contains 80 paths. Its locale-stable sorted relative-path
  SHA-256 remains
  `6bd84fb21c7fb6c5a4ca774124834ce286f7c6f4b388df3c614135a81c850b29`;
  appending to existing review files and overwriting log 47 cannot change it.
- Logs 45 and 46 retain focused `7/7` and EROD16 `1/1` with the same four
  explicit refusals from 231 storms. Log 48 records the six-row BEI pass; live
  strict validation agrees.
- `git diff --check` and the current exact six-path Markdown scope pass. The
  retained log-47 overwrite, final selected-file count, and self-excluding
  Markdown content-root identity correctly follow both revision-60 appends.
- `cargo deny check` remains not applicable because no manifest, lockfile,
  dependency, or feature-resolution input changed.

### Lifecycle and claim posture

W2C remains a technical pass with verification evidence held. W2B remains at
`HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`, W2A's earlier rerun remains ineligible,
and EB-04X remains held after W2B. Revision 60 changes neither those gates nor
the narrow produced-operand-accounting interpretation of EROD16.

### Non-blocking post-review renewal

The final evidence writer should advance the top status in `gate-results.md`
from revision 59 to revision 60 and update `worker-handoff.md`'s reviewer step
from revision 58 to revision 60 while overwriting log 47 and recording the
final Markdown content-root identity. These are expected post-review lifecycle
updates; the authoritative contract, terminal reconciliation, manifest,
roadmaps, catalog, and package already carry the current revision-60 boundary.

### Revision-60 QA verdict

**PASS.** Review B accepts the narrow revision-60 BEI correction and the exact
current technical/evidence identities. After both final review appends, renew
the scoped Markdown record and content-root identity, then obtain both terminal-
verifier passes before completing W2C or resuming W2B.
