# B01 WB14 evidence interface design

Static: **INCOMPLETE / HOLD — stopped at two unsuccessful correction cycles.**
No implementation-ready design, runtime qualification or source adoption.

## Authorization, intent and bounds

The owner adopted `/tmp/openwepp_b01_wb14_evidence_interface_design_authorization.md`
by requesting its execution. One deliverable: an implementation-ready,
independently reviewed runner-to-orchestrator evidence interface specification,
exact future fixture/test scope, and one implementation-envelope adoption decision.
This record is the only maintained narrative. No Rust, tests/assertions, contracts,
fixtures, manifests/lockfiles, features/defaults/cfg, toolchain or runtime state
may change here. Supporting static evidence and locator/catalog links are allowed.
Future source edits described below are proposals requiring separate owner adoption.

Governing checkout: `a0af6afac5caa6e84e2a782948c658f9a7b8dff3`, initially clean.
Inspection source: `reconciled-available145-r1`, retained without cadence patch at
`/workdir/openwepp-experiments/b01-wb14-source-reconciliation/reconstructed-available145`.
Reuse the [accepted predecessor assessment](../20260911-b01-wb14-source-reconciliation-001/package.md)
and [source identities/substitutions](../20260911-b01-wb14-source-reconciliation-001/artifacts/revised-baseline-proposal.json).
The 927-entry actual-map digest is
`c7aea0707d0c6a0876ec4bee5a7fcf15e751a092e086f517ced7edba8282b71e`;
regular-file inventory digest is
`8b29e888b96c223f46d8c2432c49560bbf68c15871491cfdaa68b1221cd373c6`;
symlink-target inventory digest is
`6c50ef172d22672ba98f0b66f3a45da1849b0c47409f771affd079430196b58f`.
These distinct inventories are not interchangeable. Historical working145 map
`b6fb949e967af911bd3d508c02e023bf0a349b30e3c0697631276b351b50e88f`
remains unrecovered. The available substitutions at
`tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs`
and `raw_hash_tests.rs` retain UNKNOWN historical semantic deltas; all 18 available
precedence and four raw-hash assertions remain protected.

Selected validation: deterministic selected-file identity comparison against
retained evidence, exact source/API/reference and document-link checks, evidence
syntax/behavior checks, JSON parsing, terminal diff and whitespace reconciliation,
and complementary independent correctness and QA/evidence reviews. Run allowance:
**zero Rust builds, compiled listings, tests, simulations, performance/full-tail
runs or saved-checkpoint resumes**. Every future runtime command is NOT RUN.
No archive/Git recovery campaign is repeated. Necessary inaccessible source,
material unexplained drift, scientific/authority contradiction or downstream
out-of-scope failure stops dependent work.

Owner hard limit: 60 active minutes or two unsuccessful design-correction/
verification cycles, whichever comes first. Conservative ledger start:
2026-09-12 06:46:20 UTC; includes reading, reasoning, edits, orchestration and
review, concurrent wall time once; only explicitly recorded pure waits excluded.
No predecessor unused allowance carries forward. Terminal cycles: **2 of 2**,
the binding stop reason; the 60-minute limit was not reached.

Astra owns orchestration/integration. `/root/interface_design` (implementer,
Terra/medium) owns bounded detailed source/signature design and selected-source
identity evidence; no nested delegation. Existing predecessor reviewers are not
live in this thread; distinct replacements reviewed only this new design.
Source adoption, cadence repair, wider modeling and release remain HOLD.

## Governing science and claim boundaries

Authority is the current pinned governing checkout, not the older reconstructed
runtime tree. SC-SURFACELIQUID-001 INV-012/013/014 require exact parent/child
support and receipts, one final persistent advance, and complete atomic owners;
INV-026/C-016 protect inactive native litter during represented snow;
INV-035/C-025 authenticate the complete accepted prefix without WB14 calls,
cumulative changes, receipts or physical-child ordinal advance. SC-COUPLEDTIME-001
INV-005/006/024/031 and OBL-014 retain accepted owner/support joins, parent-only
transaction increment and reconstructable chronology. SC-SNOWENERGY-001
INV-083/087 and C-055 retain the native covered map and exact snow-free successor.
The canonical ordered guards and existing typed failures remain binding.

B01's separately authorized positive-origin native drainage and custody-only
finalization are distinct from WB14 prefix work. Zero prefix WB14 deltas do not
mean zero absolute stores/cumulatives or absence of that authorized transfer.
No physical equations, tolerances, custody rules or authority are amended here.
Conservation operands are design obligations; no conservation, calibration,
identifiability or empirical validation result is claimed.



## Technical design

This unapproved draft replaces the [retired conflicting draft](artifacts/retired-design-draft.md.gz),
retained losslessly as raw review evidence. It is not an adopted implementation envelope.
All cited runtime paths are in `reconciled-available145-r1`; selected identities
are in `artifacts/selected-source-identity.json` and must be expanded before
adoption to every path listed below, using the predecessor
`fresh-recipe-whole-source-comparison.json` map.

### Facade and session

Add feature-only `openwepp_hillslope_orchestrator::restart_authority_evidence`:

```rust
pub struct Wb14EvidenceSession { _not_send_sync: PhantomData<Rc<()>>, /* private */ }
pub struct Wb14EvidenceRequest {
  pub run_id: u64, pub hillslope_id: u32, pub parent_transaction_id: u64,
  pub parent_start_ns: u128, pub parent_end_ns: u128,
  pub child_start_ns: u128, pub child_end_ns: u128,
  pub ofe_id: String, pub lane_id: u32, pub wb14_child_ordinal: u32,
  pub coupled_accepted_ordinal: usize, pub phase: Wb14EvidencePhase,
}
pub enum Wb14EvidencePhase { ObserveOnly, OutcomeLedgerBuilt, SubslabAccepted, FinalOwnerJoinCompleted }
pub enum Wb14EvidenceError { ActiveSession, InvalidRequest, ForeignTarget, DuplicateFire, Unreached, Unfired, MissingTarget, SessionThread }
pub struct Wb14EvidenceSnapshot { /* immutable records below */ }
pub fn begin_wb14_evidence(request: Wb14EvidenceRequest) -> Result<Wb14EvidenceSession,Wb14EvidenceError>;
impl Wb14EvidenceSession { pub fn finish(self)->Result<Wb14EvidenceSnapshot,Wb14EvidenceError>; }
```

`Wb14EvidenceSession` is explicitly `!Send + !Sync` through private
`PhantomData<Rc<()>>`; `SessionId(NonZeroU64)` is a private monotonic
thread-local capability. The reached runner chain contains no spawn/rayon in
runner05, accuracy helper, attachment runtime, adaptive execution, stack
helpers, or real-parent execution. Begin/finish happen inside the fresh worker
process's libtest thread. Private tests that explicitly spawn a thread begin
inside that closure. TLS state is `None | Active { id, request, mode, records }`;
begin refuses non-None, foreign target records an error, armed phase fires once,
observe-only never requires a fire, finish consumes and clears, and Drop clears
on panic. Required tests cover stale, foreign, duplicate, parallel and no-active
session leakage. The feature is absent without `restart-authority-evidence`;
no cfg(test), public owner constructor, mutation API, CLI, or manifest change.

The public DTO deliberately uses primitives/String; conversion to private
`TransactionId`/`OfeId` occurs only in the crate bridge. `lib.rs` gets the
feature-gated module and re-export. The bridge in
`snow_stage3_v11_attachment_runtime.rs:502-518` maps only exact matching
requests to existing `pub(crate) Stage3V11FailureInjection`; it does not expose
that enum or `DirectSnowStage3V11ShadowAttachment` to runner.

### Observation, injection, and precedence

There are three counters per run/parent/OFE/lane: (1) broad ingress attempts at
`surface_liquid_ingress.rs:1203` before lookup; (2) physical-call attempts
immediately before **every** `advance_wb14_continuation_interval` call at 2292,
after `DirectWb14ContinuationIntervalInputs` is assembled at 2283-91; and (3)
completed transitions immediately after `Ok`, before state/receipt mutation at
2299. A failed solver call contributes attempt plus rejected record, never
completed. Each immutable row has run/hillslope/parent identity, OFE/lane,
WB14 ordinal, input window start/end, input/ending cumulative bits, transition
input bits, outcome bits, receipt bytes/digest/disposition and routing target.

Only existing phases inject: `OutcomeLedgerBuilt(n)` at adaptive execution
2510-17 precedes receipt predecessor/ending-clock joins 2542-58;
`SubslabAccepted(n)` is after state assignment and `owner_joins.extend` at
2563-84; `FinalOwnerJoinCompleted` at stack helpers 1943-50 precedes parent
finalization. Existing typed errors and unwind are retained. Compound poisons
must assert the earlier existing public guard E001--E011, then, when guards
pass, the selected Stage3 typed injection; no new E012/error is invented.

The authentic adaptive coupled ordinal and exact child support are unavailable
without the forbidden authentic run. Therefore the negative case is a required
two-stage future protocol: an observe-only fresh worker captures the exact
matched tuple and exits success; a second fresh worker arms the identical
captured tuple and must reach/fire it. It cannot use the fixed-60 vector's
ordinal as an authentic value. This is an explicit prerequisite, not a numeric
assumption. Separately, the private fixed-60 vector asserts prefix
`[385200,385920)`, 18 accepted suffix children/OFE ordinal 0--17, and one final
advance to day4/interval23.

### Fixture, worker and restart

Extract only lawful setup from `tests03/snow_accuracy_experiment.rs:28-105`: read
frozen `gradual-warm-tail7-cases.json`, select `gradual_warm_tail7`, write the
seven ordinary climate rows to `p102.cli`, call
`prepare_native_stage3_lane_d_scale_fixture`, and authenticate with
`author_stage3_v11_owner_seed_fixture`. Do not call `stage3_snow_accuracy_case`:
it writes diagnostics and requires success. New ignored runner test
`hillslope::tests::wb14_day4_interval22_transaction255_fresh_process_prefix_cadence_replay`
uses a custom worker. Parent `current_exe()` spawns only when worker env != `1`;
child args are `--exact`, that full test name, `--ignored`, `--nocapture`; child
sets worker=`1`, an isolated fixture/output dir and a distinct create-new harness
snapshot path. It authenticically executes day indices 0,1,2,3 and day4
intervals 0..21 before the target. After exact Err/reached/fired assertion and
after output transaction Drop, it create_new-writes only the harness snapshot.
Allowed output is that snapshot; final/staged runner outputs, manifest, and
stage3 evidence must be absent. Runtime cost is unknown and must be measured
prospectively; the protected cold checkpoint is not used.

In-progress restart is distinct from committed-day Workflow: use
`stage_prepared_day_until_posture_v2` (`snow_stage3_v11_restart.rs:910-40`) at
an exact `DirectSnowStage3V11InterruptionPostureV2`, then
`finish_in_progress_prepared_day_v2` (944-75) with the same prepared day.
`DirectSnowStage3V11InProgressExecutionV2` owns day/support candidates, indexes,
prepared support restart rows, terminal/owner/subslab/adaptive/successor receipt
histories, expected child digest, pending request and trial quanta (98-122).
Compare uninterrupted/resumed values and bytes at each of those fields. The
complete native runner envelope additionally uses Workflow project/restore
(`native_custody_restart_experiment.rs:80-157`) at the runner's four wire
positions: resume 379-395, pre-day rollback capture 475, Err record 504, and
checkpoint project/write 600-631. A new runner-owned capture schema at runner05
must record actual `DirectRunFrame` fields (identity, lanes, phase_plan,
publication, all lane transfer fields, groundwater, surface_liquid_shadow,
snow attachment, laned state/summary), exact serialized in-progress bytes,
output-transaction final/staged/private/manifest/evidence paths and existence.
Capture before target; collect after Err and transaction Drop. A boolean audit is
not an oracle.

### Independent operands and outputs

For every source tile, retain immutable `DirectSurfaceLiquidStoreKey` (run/OFE/
tile/surface/source identities), `tile_fraction`, `ofe_area_m2`, support window,
`DirectIngressAmount` mass/temperature/specific enthalpy, parcel source and
destination, WB14 input/output bits, and authenticated receipt/routing fields.
With `m_t` in kg m^-2 tile-ground, calculate physical kg `M=m_t*f_tile*A_ofe`;
energy `E=M*h` J, and depth `m_t/1000` m using
`WATER_DENSITY_KG_M3=1000`. Aggregate independently by support/OFE and compare
receipt partitions after converting their kg/J m^-2 OFE-ground basis by
`A_ofe`; retain routing destination and windows so a runon cannot be counted
twice. Compare water supply/infiltration WB14 transition bits and energy from
sealed snow-owner/transfer bytes. This reconstruction may not call the
production residual or derive inputs from receipts. Controls include unequal
tile fractions/areas, multi-hop routing, zero/positive source, nonzero physical
control, hook-disabled == observe-only state/receipts/output, and prefix zero
only for WB14 (Policy13 custody changes remain permitted).

### Exact future writes, checks, and unresolved acceptance

Future writes: orchestrator `src/lib.rs`, new
`src/restart_authority_evidence.rs`, `snow_stage3_v11_attachment_runtime.rs`,
`snow_stage3_v11_adaptive_execution.rs`,
`snow_stage3_v11_adaptive_execution_stack_helpers.rs`,
`direct_runtime/surface_liquid_ingress.rs`, new feature vector module and its
existing direct-runtime registration; runner `tests03/snow_accuracy_experiment.rs`,
`03_tests.rs`, `05_runner_execution_and_outputs.rs`,
`output_transaction.rs`, and a new runner capture/helper module. No other path
is authorized. Add all these files, `snow_stage3_v11_attachment.rs`,
`snow_stage3_v11_restart.rs`, `native_custody_restart_experiment.rs`,
`surface_liquid_owner.rs`, frozen JSON and fixture template to selected identity
artifact before implementation.

All commands are prospective, NOT RUN:

```bash
cd /workdir/openwepp-experiments/b01-wb14-cadence/baseline-red
env CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-interface/red RUST_MIN_STACK=67108864 \
cargo test -p openwepp-runner --lib --locked --release --features test-fixture-authority,openwepp-hillslope-orchestrator/restart-authority-evidence,openwepp-hillslope-orchestrator/persisted-restart-v1 hillslope::tests::wb14_day4_interval22_transaction255_fresh_process_prefix_cadence_replay -- --ignored --exact --nocapture
cd /workdir/openwepp-experiments/b01-wb14-cadence/candidate-green
env CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-interface/green RUST_MIN_STACK=67108864 \
cargo test -p openwepp-hillslope-orchestrator --lib --locked --features restart-authority-evidence direct_runtime::wb14_evidence_fixed_60s_prefix_zero_suffix_18_and_final_advance -- --exact
```

Private module registration/name is not source-visible today, so the latter
filter is a proposed exact registration and must be corrected to the actual
fully-qualified libtest name before execution; zero selected is failure. Run
the existing integration target with a nonexact name filter and parse counts
requiring exactly 18 precedence and four raw-hash tests; do not use `--exact`
with module prefixes. The unavailable authentic target ordinal/support and
unknown authentic replay cost leave this specification **incomplete pending the
observe-only capture protocol**; no source adoption, cadence repair or runtime
approval follows from this design.

## Execution and review ledger

The following 06:54 entry is historical and superseded by the hard-stop
disposition below; its then-current next action is no longer authorized.

At 2026-09-12 06:54 UTC, initial integration verification of Terra's revised
technical draft FAILED: corrected appended prose coexists with contradictory
facade phase/injection/worker definitions, and exact private-test commands and
complete future envelope remain insufficient. **Unsuccessful correction cycles:
1 of 2.** The current technical draft is not implementation-ready or approved.
Initial drafting feedback itself was not counted as a correction cycle; the
subsequent corrected draft's failed verification is counted. No allowance reset.
One recorded pure wait lasted 30.3909 seconds; conservative final consumption
may include it rather than exclude it.

The predecessor reconciliation reviewers are unavailable in the live agent
inventory (only root and current implementer existed). Distinct replacements
`/root/interface_correctness` (Sol/high) and `/root/interface_qa` (Terra/medium),
neither an author/adviser of this design, are reviewing only this new scope.
They return findings without shared-record edits and will verify their own
accepted fixes. No accepted reconciliation review is reopened. Next action:
collect both complete initial finding sets, make one bounded correction and
verify it; stop if that correction fails required verification or the time cap.

### Hard-stop disposition

At 2026-09-12 07:04:45 UTC, Astra verified the second corrected draft and found
required acceptance still unmet: exact interruption posture and complete native
wire positions, executable/nonzero complete test selection, complete named write
paths, authentic bounded fixture/target derivation, and precise immutable
observation/operand schema remain unresolved or deferred. The correction itself
labels the specification incomplete. **Cycle 2 of 2 FAILED; design work stopped.**
The incomplete specification is not a completed negative design study: the
requested implementation-ready deliverable was not achieved. Absence of Rust
execution is required here and is not the reason to waive missing design detail.

Both independent reviewers received the same corrected specification for final
fix verification. Subsequent work is limited to truthful state preservation,
review collection, document/evidence checks and carried scoped evidence custody;
no third design correction, implementation, authentic capture or cadence work.
The retired first draft was archived losslessly for review provenance, not kept
as hidden live instructions. All predecessor ledgers/source/checkpoints remain
unchanged. No necessary runtime source was found inaccessible; the fixture's
initially mistaken source-root lookup was resolved against retained governing
evidence with the exact required hash.

**Owner decision: do not adopt the proposed implementation envelope against
`reconciled-available145-r1` in its current incomplete form.** That source remains
a prospective inspection source only. A future continuation needs explicit
owner authorization because this checkpoint's two-cycle allowance is exhausted;
this record grants none. Source adoption, cadence repair, wider modeling and
release remain HOLD. The historical two-test semantic gaps remain UNKNOWN.

### Independent final findings and evidence checks

Static: same-reviewer verification returned **FAIL / HOLD for design adoption**
from both [correctness](artifacts/correctness-review.txt)
(`/root/interface_correctness`, Sol/high) and [QA/evidence](artifacts/qa-review.txt)
(`/root/interface_qa`, Terra/medium). Each inspected retained primary source and
verified accepted fixes in its own scope. These are independent design reviews,
not runtime approval. The preserved initial responses and final findings retain
line references to their reviewed cuts; archival moved the current line numbers.

Verified improvements include retiring the contradictory first draft, separating
broad ingress attempts from each actual physical call and its successful return,
using a custom rejection worker with lawful construction and authentic day
chronology, and adding tile/area/enthalpy operand reconstruction. Those local
improvements do not meet the complete deliverable.

The blocking technical findings remain deliberately unfixed at this hard stop:

- The DTO wrongly uses `u64` for the kernel's `u128` transaction identity and
  conflates that sequence with the coupled parent's digest; the snapshot and
  normal-runtime injection bridge are still underspecified.
- Observe-only discovery requires the unknown target in its own request and
  proposes success from a baseline known to refuse before that phase. Capturing
  the tuple after cadence repair would not satisfy the frozen preceding-evidence
  obligation. This is an unresolved protocol flaw, not permission to repair cadence.
- No exact interruption variant, complete role-by-role native wire mapping, or
  typed runner-owned capture/collection interface is specified.
- Matched runner/private commands, exact registrations, phase/restart coverage,
  executable 18/4 protection and nonzero-selection guards remain incomplete.
- The future write and fixture dependency envelope is still not complete.

Preservation-only follow-up reconciles the historical ledger and locator/catalog
with the two-cycle stop and records [23 inspected source identities](artifacts/final-inspected-source-identities.json).
All 23 match the predecessor regular-file map; additional hashes in the original
selected-source artifact are also checked, with the tail7 input rooted in the
governing evidence tree. This binds the files actually inspected/cited here;
it does not complete the future fixture's transitive dependency/constructor
scope. The previous review finding remains visible with this limited follow-up.

Ran from `/workdir/openWEPP`:
`.venv/bin/python docs/work-packages/20260911-b01-wb14-evidence-interface-design-001/artifacts/validate_design.py`
— PASS; [exact argv/results](artifacts/executor-checks.json). Checks cover pinned
authority/input and selected source bytes, JSON/Python syntax, proposed bash
syntax only, owned document links, retired-draft lossless decompression and
whitespace. Command syntax PASS does not make the incomplete future selections
valid. Zero Rust builds, compiled listings, Rust tests, simulations, performance
runs, full-tail runs or saved-checkpoint resumes were executed.

Diff classification: documentation and supporting static evidence only; this
package and two locator/catalog additions. No Rust, test assertion, scientific
contract, fixture, manifest, lockfile, feature/default/cfg, toolchain, runtime
state, historical package ledger or source snapshot was changed. The external
suite posture/required-case bindings were not edited, so executable anti-evasion
and Rust checks are not triggered by this diff and remain NOT RUN under the
explicit zero-run allowance. No calibration or scientific closure claim is made.

Custody publication is pending the final scoped commit/push and remote changed-
file byte verification below. The carried permission is explicitly retained by
the adopted authorization and predecessor; no branch change is requested or made.
Full historical base/LFS retrieval remains NOT RUN and is not implied by this
package's evidence publication.

Same reviewers subsequently verified preservation and disposition consistency:
correctness **PASS for publication as INCOMPLETE/HOLD evidence**; QA **PASS for
custody/disposition**, with its requested final response now preserved in
`artifacts/qa-review.txt`. Neither changes the design FAIL/HOLD. They checked
ledger supersession, catalog consistency, retained blockers and the explicit
limits of inspected-source custody. No further design correction occurred.

Allowance at hard stop: **18 minutes 25 seconds elapsed** (06:46:20–07:04:45 UTC),
conservatively all charged as active time. Final preservation/publication time
is additional consumption recorded below. No pure waits are deducted; concurrent
agent wall time is counted once. Delegated work comprised Terra's initial draft
and two correction submissions plus distinct correctness/QA initial reviews,
same-reviewer fix checks and preservation checks, all within this same ledger.
The stop was the exhausted two unsuccessful correction cycles, not elapsed time.

### Publication custody

Scoped evidence commit `817f683e8b72ebf8a238aedc547309226a6cac50` was pushed to
existing `origin/main` under the carried permission. Ran the retained predecessor
[publication verifier](../20260911-b01-wb14-source-reconciliation-001/artifacts/verify_publication.py)
with that exact commit: **11/11 changed files retrieved from GitHub and matched
byte-for-byte against `git show COMMIT:path`**, with no LFS pointers. The
[receipt](artifacts/remote-evidence-verification.json) retains URLs, byte counts,
hashes, exact argv and result. This verifies this new evidence only; full old
runtime-source base and historical LFS archive retrieval remain NOT RUN.

At the verified-custody cut, 2026-09-12 07:10:32 UTC, conservative cumulative
consumption is **24 minutes 12 seconds**, including all waits and concurrent
reviews counted once; unsuccessful correction cycles remain **2/2**. The final
receipt/closure annotation commit adds only custody preservation. No unused
minutes override the exhausted cycle stop, and no successor action is authorized.
