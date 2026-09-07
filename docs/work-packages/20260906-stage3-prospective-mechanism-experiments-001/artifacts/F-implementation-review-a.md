Static: independent implementation review A of all 14 changed/new Rust paths
in `/tmp/openwepp-controlled-mechanisms-Hb6uS2/F`, against immutable A commit
`2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`. No compilation, Rust test or
simulation run by this reviewer. Ran: read-only diff/source/hash inspection
and `git diff --check` (PASS). Review B findings were not consulted.

## Findings

No blocking correctness finding on this cut.

LOW, `crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs:2837`:
the inherited textual immutable-receiver assertion still names
`execute_covered_carrier_phase_v1`, which now identifies a cfg(test) adapter.
When maintaining this classifier, also bind the actual renamed
`execute_covered_carrier_feed_forward_phase_v1` receiver. This is not a runtime
parity claim or blocker: the concrete production receiver remains `&self`, the
physical invocation is directly visible, and the real runtime oracle remains
mandatory. No historical assertion needs removal.

## Correctness assessment

The real attachment selects the feed-forward provider before evaluation and
passes `CoveredTerminalFeedForwardRequestV1` through the physical carrier,
shared batch/leader requests and snow-boundary projection. The type contains
all physical/custody fields and structurally excludes hint/iteration. The old
physical interface is test-only. Static inspection of A's physical source finds
hint/ordinal only in three leader-request constant initializers, not physical
reads; the reference adapter's removal of those fields is therefore justified
by the original source dependency, not only by the new type's shape.

`terminal_carrier_evaluation.rs:9` centralizes the original receipt join chain,
surface-energy call, required diagnostic and nine flux operands once. The
support/beginning/child-role/attempt/digest precedence, multiplication/division
order and trial-support duration are retained. No equation, clamp, unit,
threshold, default, serialization or typed error has changed in that extraction.
Feed-forward invokes its selected provider once, validates its receipt, runs
the canonical flux/terminal-preview checks and returns. It does not call a
second provider on failure or fabricate convergence. Feedback-capable callers
retain the original 32-call bound, same hint progression,
four-component break, post-loop check and mandatory live-convergence guard.
The resolved-domain call remains a distinct actual single invocation.

The production call owns its local request and result; immutable beginning
candidate borrows and existing retention/unwind boundaries are preserved.
There is no new memoization, cached cross-invocation result or discovered/exact
reuse. The existing provider retention map is not repurposed to skip physics.
The large physical carrier body changes request types/constant-field removal,
not scientific arithmetic. No substantial duplicated physics was introduced;
the legacy feedback loop and feed-forward path consume the same flux helper.

Observation projections reconstruct a first-call audit request only inside
detailed/evidence-enabled hooks. They never route the production physical
call back through the old interface. The actual reference callback is cfg(test)
and explicitly enabled on the dedicated fixture thread; ordinary tests and
measured production-crate execution do not run its extra physical work.

The prospective lower tests remain unchanged: exactly one invocation for equal
independent and exact-path requests, outer and receipt-join first errors,
all five roles, and one-call versus original two-call feedback flux/receipt
comparison. The actual complete-owner oracle invokes fresh physical references
twice for each actual F invocation, compares every phase-result field plus
private candidate/owner payloads, checks canonical preview equality and beginning
owner immutability, and requires all five roles plus discovery/exact modes.
It does not substitute fingerprint equality for complete output comparison.
Its shared physical implementation is intentional: it tests repeat-call
invariance and custody; independent A/F scientific-output/closure parity remains
the separate guard against extraction drift.

## Residual risk and pending evidence

Static review is not execution. The exact test/oracle paths must compile and
pass, including complete real-carrier references, typed error/rollback and
existing authority guards. Actual workload provider/evaluator cardinality,
full scientific outputs/owner/closure parity, full critical-cut correctness,
lint and package measurement admission remain required before comparison.
No prospective assertion is waived or weakened if execution exposes a defect.
No new gate beyond the approved F scope is requested.

The largest touched sources remain under 3,000 lines: stage3_solver.rs 2,954,
terminal execution 2,942 and carrier phase 2,943; their existing size warnings
remain, with the loop/flux extraction reducing evaluation.rs to 2,298.

## Reviewed cut identities

Paths below are relative to `crates/openwepp-hillslope-orchestrator/src/`.

| Path | SHA-256 |
| --- | --- |
| `hydrology/03_kernel_support_00_support_helpers.rs` | `4d3f75ca712722b5717036aa83712da84dd2e6753d47ce372dc568928e87aef9` |
| `hydrology/support_helpers_mod/mod.rs` | `606f4d2c157d133b5f9f2d03041a8f01ac89e3d7d853e580213483941494f9e8` |
| `hydrology/support_helpers_mod/runoff_reconciliation.rs` | `f03d6f7e5426661970b0396d8b9f69e80be60878e9d8d4d154b0d9405db72687` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs` | `b01db67b2226edf2be07a8db615f3ff80139acffc6d284d68cba6e8b863f37ff` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs` | `be4a88ec959216a72ded7d86f27532887c9719fe3c16f5c3494666b166ef2944` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/support.rs` | `27208c28f121273cf301fea7d54774b6769c1eb79f8c2d32ade4688a2771e3f8` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_carrier_evaluation.rs` | `c7037c8b72d76db48ee92eebb142cb5512efbd81a3e3ae24ddf93823d7593efd` |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_feed_forward_tests.rs` | `1f78f061ef5093f99edc05d96d1f5241445f0991f0bccb300d43e06a9cf6d3c6` |
| `hydrology/support_helpers_mod/runoff_reconciliation/terminal_carrier_provider.rs` | `dcacb51d7ffc5e3b304152935b11f2ace70beb589cc813ca86e614cbcf99e0c8` |
| `snow_stage3_v11_adaptive_production_tests.rs` | `6a1cc7a02caf0d6eb1a889695cac93522f8de5bbaa1d7200e9bac098a0446954` |
| `snow_stage3_v11_terminal_execution.rs` | `ed090ee54d61a1a74e7ca392459466843a9828d9dabd64f182082e02ac972834` |
| `snow_stage3_v11_terminal_feed_forward_tests.rs` | `4fbd55a350afab3ebb994fc6872cde0f68dc415236352031de52c9053cc7cef3` |
| `v11_covered/carrier_phase.rs` | `d45dca53b9188e266446218a7f6dcfab40e8dc34f074522857f86823ca0abfc5` |
| `v11_covered/carrier_phase/snow_boundary.rs` | `a2a06ec9fe039850958ecfc7f6fd0f7b832362c2151c40c898012eae78c6c09c` |

Decision: GO / no static implementation blocker on this isolated cut. Runtime
gates are pending; this does not admit measurement, production promotion or
package completion.

## Test-only maintenance addendum

Static: reviewed the two additional assertions. The carrier receiver classifier
now also names the actual typed feed-forward entry, and the accepted executor's
existing no-physics-rerun list now includes that entry. The LOW finding is
resolved without removing historical checks or changing behavior. Current
`v11_covered/carrier_phase.rs` hash:
`0c0747455e2ea4f26eb2a38da573b8d443643b211accb883f63b27941c6225e1`.
The additional test-only path `v11_covered/open_snow_tail_tests.rs` hash:
`9d3758bfc6fc65911904e0e4e0c498dcaae3e5fd7de7d67fe8a513a285928206`.
All other reviewed identities are unchanged. GO remains static only; execution
is pending and no runtime claim is added.

## Resolved-provider source-guard repair: prospective GO

Static: current `stage3_evaluation_validation_tests/persistent_tests.rs` hash
`9a9133b1f2fe07454cbf6e218f5983beb324a390bb799ea01043d550358ac183`
still asserts the old direct provider-call spelling. Reviewed the actual single
`provider.evaluate_resolved(CoveredTerminalTrialRequestV1 { ... })` seam,
unchanged boundary/joint/child checks and subsequent boundary handoff in
`stage3_solver/evaluation.rs`, plus the direct dispatch in provider source hash
`e241828f0225db2c56cd2ff53a4a5b29311645e7d67278405ef7a474a7908971`.

GO to replace only that stale spelling with an exact single-dispatch assertion,
retain the boundary-routing check, add the transition-boundary and existing
join-error checks, and inspect the actual included dispatcher for one direct
callback per FeedForward and cfg(test) Feedback arm with no adaptive/coupling
helper re-entry. Preserve the neighboring executable provider-before-raw-carrier
typed-error test. This corrects source maintenance, not physics or required
independent resolved work. Final edited test identity/execution remain pending.

Correction confirmed: final `persistent_tests.rs` hash
`2d8ce81ff9350eb808feb205da86dd6cf660c0610f22f301183d0f2c0bc85426`
implements the reviewed exact resolved dispatch, boundary/join and direct-arm
callback counts, rejecting both adaptive and feedback invocation helper names.
GO for the edited source; corrected executable test evidence remains pending.

Separately, prospective GO for one field-specific
`cfg_attr(not(test), expect(dead_code, reason = ...))` on legacy request
`coupling_iteration`. Current semantic reads are test-only; production retains
the field to preserve the shared derived-Debug/audit schema. This narrow warning
disposition changes no fields, request values or behavior. It must not suppress
the surrounding type/module, the inherited `ending_snow_hint` warning or any
other warning. Actual attribute source identity remains to be recorded.
## Prospective derived day-frame telemetry qualification: GO

Prospective scale addendum: GO for the exact extended attribution
`7536ede62b8b16ca7a9723a0c4deddef73a4465ac7a88595af936ee0c29cdb33`.
Independently read the real full-frame clone into the authenticated owner,
restore equality against the fresh live frame, complete-owner lane/layer-map
joins, and controlled fixture cardinality assertions. For N=1/10/19 those joins
establish that each successful native provider's two adapters seed all N lanes,
not merely the currently selected covered lane.

Before timing, the common collector may qualify exactly this one manifest leaf
as `{qualification: F-two-full-lane-seeds, noncarrier_constructions: D-2*N*P}`.
Retain raw D/N/P and the exact bound rule in admission evidence and raw records.
Require nonnegative checked integer arithmetic, authenticated matching N,
balanced successful Provider=Carrier counts, no errors or dropped events,
unchanged applicable batch/branch and invocation populations, and exact other
science/control/output fields. N=1 retains all raw constants and residual405
below. N=10/19 require fresh independently matched admissions with equal actual
residuals; do not import405 or predict their provider/construction counts.
The extension is prospective collector-design GO, not scale execution evidence
or permission to erase arbitrary counter differences.

Static: reviewed `F-day-frame-attribution.md` hash
`854d553edb829a97c49308d3ee1f3b25f7811c8e777c88f23f16287b276fb21f`
before collector qualification or measurements. Independently inspected the
unchanged native carrier chain: the V4 physical evaluator first prepares V3
fixed-final inputs, constructing one hydrology adapter, then constructs its own
second adapter. Each adapter seeds exactly one day frame per real lane. The sole
construction increment is in `DirectDayFrame::seed`. Runner frame construction
and the authenticated fixture bind the primary one-OFE workload to one lane.
The adapter/native/counter/runner source paths have no diff against A.

Ran: read-only parsing of A-admission-02 and F-admission-01 confirms balanced
error-free Provider=Carrier counts 400 and 200, unchanged Outer=72/Evaluator=200,
BatchProvider=0, and construction counts 1205 and 805. Every other direct-runtime
counter is identical, including commits=0 and state mutations=10.

Approve only the exact manifest leaf
`/direct_runtime_counters/day_frame_constructions` as mechanism-dependent
telemetry, not volatile provenance. For this pinned one-lane primary case require
all of: A=1205, F=805; successful/error-free Provider=Carrier A=400/F=200;
`D_A-D_F = 2*(P_A-P_F) = 400`; and
`D_A-2*P_A = D_F-2*P_F = 405`. Preserve raw counts and relation evidence.
The residual is not an accepted-day count or a guessed cost attribution.
All other manifest fields retain exact comparison under the already reviewed
explicit provenance rules. Never remove the counter object or admit arbitrary
differences, failed/unbalanced events, other workloads or lane counts under
these constants. This is an intended downstream construction reduction from
the removed duplicate carrier work, not changed scientific ownership.

Actual invocation/role/custody multiset and result fingerprints, complete
physical-reference oracle, output/closure and full validation remain independent
gates. Aggregate algebra alone cannot substitute for any of them. No timing,
collector implementation execution or broader F admission is claimed here.
