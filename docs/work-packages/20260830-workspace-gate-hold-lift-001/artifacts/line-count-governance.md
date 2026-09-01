# Line-count governance

Status: `EXECUTING`

Evidence mode: `Ran`

Vegetation terminal counts after the mechanical Clippy correction:

- `crates/openwepp-vegetation/src/v11.rs`: 2,920 lines (`WARN`, below 3,000),
  +161/-121 from the package baseline;
- `crates/openwepp-vegetation/src/v11/tests/v11_bgc_tests.rs`: 281 lines,
  +3/-2.

The increase isolates restart replay state/control flow and preserves the
existing public function shape; no 3,000+ exception is requested. Publication,
fixed-point, LSE, runner, and terminal package-artifact counts remain pending
their assigned corrections.

## WGHL-FULL-001D

Ran: terminal v32 owned-source counts are `fixed_point.rs` 1,779 lines,
`open_snow.rs` 2,895 lines, `open_snow_convergence_tests.rs` 986 lines,
`snow_stage3_v11_adaptive_execution_tests.rs` 623 lines, and the direct DFF
integration test 220 lines. All remain below 3,000 lines; no exception is
requested. The v31/v32 helper and support-image types are private and add no
public API or serialized surface.

## WGHL-FULL-001F

Ran: `crates/openwepp-land-surface-energy/src/solver_covered_solve.rs` is 1,207
lines after the no-update witness correction (`PASS`, below 2,000). The new
predicate and its refusal vectors remain inline with the covered solve they
govern; no line-count exception is requested.

## WGHL-FULL-001D open-snow structural split

Ran: pre-split `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs`
is 4,435 lines (`BLOCKING`, at or above 3,000). The authorized exact moved
blocks are 407 lines of convergence metrics and 1,578 lines of physical-support
and reuse helpers. Expected terminal counts are approximately 2,452, 407, and
1,578 lines respectively. No 3,000-line exception is requested.

Ran: terminal counts are `open_snow.rs` 2,452 lines (`WARN`, below 3,000),
`open_snow_convergence_metrics.rs` 407 lines (`PASS`), and
`open_snow_physical_support.rs` 1,578 lines (`PASS`). The former blocking
4,435-line monolith is eliminated and no exception is required.

## WGHL-FULL-001D V36 geometry-complete follow-on

Ran: after the V36 density-coordinate and exact constitutive-branch guards,
`open_snow.rs` is 2,509 lines (`WARN`, below 3,000). The active coupled-solve
region remains inline through V36 qualification to avoid a simultaneous
behavioral and structural change. A follow-on exact-move split of that region
is required before it reaches 3,000 lines; no line-count exception is requested.

## WGHL-FULL-001D V37 derived-thickness follow-on

Ran: after the V37 derived `R_z` carrier and same-map integration,
`open_snow.rs` is 2,515 lines (`WARN`, below 3,000),
`phase_consistent_coupled_solve.rs` is 1,094 lines (`PASS`),
`stable_monotone.rs` is 546 lines (`PASS`), and
`open_snow_convergence_tests.rs` is 1,963 lines (`PASS`). The active solver
region remains below the hard boundary; the existing exact-move split intent
remains required before `open_snow.rs` reaches 3,000 lines.

## WGHL-FULL-001D V38 finalization-equivalent-map follow-on

Ran: after the V38 endpoint-map reconstruction, exact replay-input carrier, and
focused projection/poison vectors, `open_snow.rs` is 2,637 lines (`WARN`, below
3,000), `phase_consistent_coupled_solve.rs` is 1,183 lines (`PASS`),
`fixed_point.rs` is 1,596 lines (`PASS`), and
`open_snow_convergence_tests.rs` is 2,221 lines (`WARN`, below 3,000). The
active finalization-equivalent region remains inline for canonical
qualification. The existing exact-move split intent remains binding before
either warning file reaches 3,000 lines; no exception is requested.

## WGHL-FULL-001D V39 soil-energy transaction-custody follow-on

Ran: after the V39 source/soil transaction authority was wired through the
final-equivalent map and all existing physical operand call sites,
`open_snow.rs` is 2,660 lines (`WARN`, below 3,000). V39 and the V33 rolling
reset correction add no new solver algorithm or diagnostic state. The existing
exact-move split intent remains binding before this file reaches 3,000 lines;
no exception is requested.

## WGHL-FULL-001D V40 parity-monotone active-set follow-on

Ran: after the V40 four-window eligibility seam, retained V31 diagnostic
oracle, and focused poison vectors, `open_snow.rs` is 2,708 lines (`WARN`,
below 3,000), `phase_consistent_coupled_solve.rs` is 1,379 lines (`PASS`),
`fixed_point.rs` is 1,720 lines (`PASS`), and
`open_snow_convergence_tests.rs` is 2,458 lines (`WARN`, below 3,000). The
existing exact-move split intent remains binding before either warning file
reaches 3,000 lines; no exception is requested.

## WGHL-FULL-001D V41 one-way phase-boundary follow-on

Ran: after the V41 exact one-way canonical enthalpy-boundary eligibility seam
and focused poison vectors, `open_snow.rs` is 2,719 lines (`WARN`, below
3,000), `phase_consistent_coupled_solve.rs` is 1,580 lines (`PASS`),
`fixed_point.rs` is 1,720 lines (`PASS`), and
`open_snow_convergence_tests.rs` is 2,602 lines (`WARN`, below 3,000). The
existing exact-move split intent remains binding before either warning file
reaches 3,000 lines; no exception is requested.

## WGHL-FULL-001D V42 cold-content-export coordinate follow-on

Ran: after the V42 exact authentic cold-content-export support coordinate and
five captured/compatibility/poison vectors, `open_snow.rs` is 2,719 lines
(`WARN`, below 3,000), `phase_consistent_coupled_solve.rs` is 1,559 lines
(`PASS`), `fixed_point.rs` is 1,717 lines (`PASS`), and
`open_snow_convergence_tests.rs` is 2,766 lines (`WARN`, below 3,000). The
existing exact-move split intent remains binding before either warning file
reaches 3,000 lines; no exception is requested.

## WGHL-FULL-001D V43 projected-base custody follow-on

Ran: after the V43 typed numerical-coordinate fixed-point posture and five
positive/byte-lock/poison/no-publication vectors,
`v9_real_consumer_shadow/v10_soil_thermal_v2.rs` is 2,221 lines (`WARN`, below
3,000) and `v10_soil_thermal_v2_tests.rs` is 2,187 lines (`WARN`, below
3,000). The correction is deliberately localized beside the existing base and
same-support custody validators so ordinary and projected postures can be
reviewed together. Before either file reaches 3,000 lines, perform an exact
move of the unpublished fixed-point/continuation validators and their tests
into a sibling module; no exception is requested.

## WGHL-FULL-001D V44 uncommitted LSE closure-posture follow-on

Ran: after the terminal V44 corrected-exchange/V8-selector remediation,
`open_snow.rs` is 2,722 lines (`WARN`, below 3,000),
`phase_consistent_coupled_solve.rs` is 1,688 lines (`PASS`), and
`open_snow_convergence_tests.rs` is 2,955 lines (`WARN`, below 3,000). The V44
exact-move sibling `open_snow_convergence_v44_tests.rs` is 79 lines (`PASS`),
the source-bound integration contract is 1,213 lines (`PASS`), the typed V8
selector file is 1,350 lines (`PASS`), and its existing V2 test module is
2,246 lines (`WARN`, below 3,000). The exact move restores the nonexempt source
limit without splitting the active coupled-map region. Existing split intents
remain binding before either warning file reaches 3,000 lines; no exception is
requested.

## WGHL-FULL-001D V45 authentic receipt root-polishing follow-on

Ran: after the V45 shared safeguarded-step, exact branch/ordinal bundle, and
expanded runtime poison vectors, `open_snow.rs` is 2,750 lines (`WARN`, below
3,000), `phase_consistent_coupled_solve.rs` is 2,168 lines (`WARN`, below
3,000), and `open_snow_convergence_tests.rs` is 2,976 lines (`WARN`, below
3,000). The split V45 runtime test module is 483 lines (`PASS`). The existing
exact-move split intent is immediately binding before any warning file reaches
3,000 lines; no exception is requested. V45 keeps the one active safeguarded
numerical core intact rather than splitting it mid-correction.

## WGHL-FULL-001D V46 complete-step budget-preflight follow-on

Ran: after the V46 budget-atomic preflight and split behavior vectors,
`open_snow.rs` is 2,750 lines (`WARN`, below 3,000),
`phase_consistent_coupled_solve.rs` is 2,216 lines (`WARN`, below 3,000), and
`open_snow_convergence_tests.rs` is 2,977 lines (`WARN`, below 3,000). The new
exact-move `open_snow_convergence_v46_tests.rs` is 340 lines (`PASS`). The
existing split-before-3,000 intent remains immediately binding. V46 adds no
second numerical core and does not split the active safeguarded-step region
mid-correction; no exception is requested.

## WGHL-FULL-001D V48 fixed-point final-install authority follow-on

Ran: after the V48 authenticated prepared-beginning final-install seam and
expanded exact R122/poison/no-op vectors,
`v9_real_consumer_shadow/v10_soil_thermal_v2.rs` is 2,468 lines (`WARN`), its
included `v10_soil_thermal_v2_tests.rs` is 2,956 lines (`WARN`), and
`v11_covered/owner_finalization.rs` is 2,933 lines (`WARN`). The V48 correction
is deliberately kept adjacent to the
existing accepted-owner and finalizer install branches for review. Before the
V10 source or test reaches 3,000 lines, perform the already-planned exact move
of unpublished/continuation installation and its tests into sibling modules.
Before `owner_finalization.rs` reaches 3,000 lines, exact-move its leading
`#[cfg(test)] terminal_custody_lane_set_tests` module into a sibling included
test file; do not split the active finalization control-flow region. No
line-count exception is requested.

## WGHL-FULL-001D V50 envelope-source transition authority follow-on

Ran: after the V50 opaque envelope-source transition authority and its exact
mixed-beginning/poison vectors,
`v9_real_consumer_shadow/v10_soil_thermal_v2.rs` is 2,882 lines (`WARN`),
`v11_covered/owner_finalization.rs` is 2,951 lines (`WARN`), its exact-move
V50 transition include is 118 lines (`PASS`), the retained V10 test file is
2,962 lines (`WARN`), the split `v10_soil_thermal_v2_v49_tests.rs` is 813
lines (`PASS`), and `covered_v8_owner.rs` is 1,271 lines (`PASS`). Existing exact-move
split intents remain binding before either warning file reaches 3,000 lines;
no exception is requested.

## WGHL-FULL-001D V49 multi-child prepared-install authority follow-on

Ran: the retained V48 V10 test module is already in immediate `WARN`
posture near 3,000 lines. V49 therefore adds no new behavior body to that
file. Its exact R123/repeated-child/poison vectors are exact-moved into sibling
`v10_soil_thermal_v2_v49_tests.rs`, included only by a distinct `#[cfg(test)]`
module. The production resident/atomic/install control flow remains contiguous
for review; no active production region is split mid-correction. Terminal
counts are: `v10_soil_thermal_v2.rs` 2,645 (`WARN`), retained
`v10_soil_thermal_v2_tests.rs` 2,962 (`WARN`), V49 sibling 552 (`PASS`), and
`owner_finalization.rs` 2,936 (`WARN`). Existing exact-move plans remain
binding before any warning file reaches 3,000 lines; no exception is requested.

## Terminal orchestrator structural splits

Ran: pre-split `snow_stage3_v11_attachment.rs` was 3,191 lines and
`v11_covered/execution.rs` was 3,105 lines (`BLOCKING`, at or above 3,000).
Terminal counts are `snow_stage3_v11_attachment.rs` 2,880 lines (`WARN`, below
3,000), `snow_stage3_v11_prepared_support_identity.rs` 312 lines (`PASS`),
`v11_covered/execution.rs` 2,919 lines (`WARN`, below 3,000), and
`v11_covered/execution_carrier_humidity.rs` 187 lines (`PASS`). Expanded-source
hash parity passed for both seams. The two blocking files are eliminated and
no 3,000-line exception is required.

## WGHL-FULL-001D V51 post-crossing contraction follow-on

Ran: after centralizing the V41/V51 validated trace and exact-moving the V51
behavior body, `phase_consistent_coupled_solve.rs` is 2,359 lines (`WARN`),
`open_snow.rs` is 2,770 lines (`WARN`),
`open_snow_convergence_tests.rs` is 2,999 lines (`WARN`, below 3,000), and the
new `open_snow_convergence_v51_tests.rs` is 226 lines (`PASS`). The source-bound
integration contract is 1,778 lines (`PASS`). The exact move keeps the active
coupled numerical core contiguous and eliminates a blocking test-file growth.
Existing split-before-3,000 intent remains binding; no exception is requested.

## WGHL-FULL-001D V52 CN heat coordinate follow-on

Ran: after the V52 typed CN-consumption selector, enlarged physical residual,
and exact-move behavior vectors, `open_snow.rs` is 2,886 lines (`WARN`),
`phase_consistent_coupled_solve.rs` is 2,523 lines (`WARN`), and
`open_snow_convergence_tests.rs` is 2,978 lines (`WARN`, below 3,000). The
dedicated `open_snow_convergence_v52_tests.rs` split is 418 lines (`PASS`), and
the source-bound integration contract is 1,842 lines (`PASS`). The active
charged-map and safeguarded-solver regions remain contiguous for correctness
review. Existing exact-move split intent remains binding before any warning
file reaches 3,000 lines; no exception is requested.

## WGHL-FULL-001D V53 same-map CN heat seed follow-on

Ran: after the shared fresh/legacy same-map Q seed assembler and its dedicated
behavior split, `open_snow.rs` is 2,876 lines (`WARN`),
`phase_consistent_coupled_solve.rs` is 2,576 lines (`WARN`),
`open_snow_convergence_tests.rs` is 2,978 lines (`WARN`, below 3,000),
`open_snow_convergence_v52_tests.rs` is 420 lines (`PASS`), and
`open_snow_convergence_v53_tests.rs` is 210 lines (`PASS`). Every active file
remains below 3,000 lines. V53 behavior is isolated in the new split; no active
solver region is mechanically divided. Existing split-before-3,000 intent
remains binding and no exception is requested.

## WGHL-FULL-001D V54 representable receipt-cycle witness follow-on

Ran: after the bounded exact-cycle detector, own-artifact endpoint projection,
charged authentic witness search, and dedicated behavior split,
`open_snow.rs` is 2,919 lines (`WARN`),
`phase_consistent_coupled_solve.rs` is 2,862 lines (`WARN`),
`open_snow_convergence_tests.rs` is 2,978 lines (`WARN`, below 3,000),
`open_snow_convergence_v54_tests.rs` is 655 lines (`PASS`), and the
source-bound integration contract is 1,972 lines (`PASS`). Every active file
remains below 3,000 lines. The numerical core remains contiguous and the V54
behavior is isolated in the new split. Existing split-before-3,000 intent
remains binding and no exception is requested.
## V55 private Q-lattice witness disposition

Static/Ran: V55 keeps
`v11_covered/phase_consistent_coupled_solve.rs` at 2883 lines and extracts the
bounded lattice helper to `phase_consistent_private_q_lattice.rs` at 124 lines.
The main `open_snow_convergence_tests.rs` is 2981 lines; the retained V54 split
is 657 lines and the V55 focused split is 521 lines. No active production file
exceeds the 3000-line hard split threshold. The source-bound integration
catalogue is 2044 lines and receives a WARN/split intent: a later mechanical
package should split versioned obligations without changing assertions.
