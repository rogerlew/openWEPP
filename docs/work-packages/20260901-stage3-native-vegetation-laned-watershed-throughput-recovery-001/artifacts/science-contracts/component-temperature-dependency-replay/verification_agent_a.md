# Independent component-temperature dependency-replay verification A — final post-revert

Evidence mode: `Static + Ran + Expected-red + Forensic reconstruction`

Verifier: `rust_code_reviewer`

Final candidate manifest:
`edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`

Current post-revert Rust manifest:
`2813f6e8faabb9408bac5e59b9271626ff5bcdc7fe49ab6dda810d3a1c3eee0d`

Verdict: `APPROVE TERMINAL HOLD / FAIL_REVERTED`

## Findings

No blocker remains to rejecting the candidate, accepting the complete revert,
and placing revision 31 and the package on terminal `HOLD`. Production
retention and release approval remain blocked.

### CRITICAL — the binding release audit failed

Candidate run 1 reached the authentic runner assertion but its real aggregate
had no completed `N=2,S=6` sweep with exact
`logical/anchor/replay/complete = 58/14/16/28`. The command exited `101`
before emitting `STAGE3_LANED_RELEASE_PROBE` JSON. The frozen retention rule
requires every run to contain that authentic record, so this is a failed
release conjunct, not an inconclusive timing sample. Runs 2/3 were correctly
not run; there are no candidate timing or RSS medians to adjudicate.

The contract requires complete v31 reversion on any failed conjunct. Static and
bounded executable checks confirm that disposition. No partial replay, graph,
cache, audit, runner hook, or alternate evaluator remains.

### MEDIUM — recorded sequencing defects remain governance history

The package truthfully retains four defects: eight paths were edited before
retrospective write-set listing; `v11_vegetation_consumer.rs` was edited before
its exact entry; the component replay extracted LSE/runner paths were edited
before exact listing; and the earlier vapor/emax correction preceded capture of
its intended expected red. None is backdated or relabeled. The candidate-only
extracted component files are now absent. These defects do not authorize
retention and do not block the truthful `FAIL_REVERTED / HOLD` result.

## Candidate approval and premeasurement gates

The captured ordered candidate manifest has exactly 16 entries. Rehashing that
record independently gives
`edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`.
Both final implementation reviewers independently bind this exact digest and
return `APPROVE` for measurement, superseding their earlier cuts.

Their immutable evidence records:

- focused component replay: `PASS 14/14`;
- full candidate LSE: `PASS 154/154`;
- feature-enabled compiler-negative ownership/lifetime diagnostics:
  `PASS 2/2`;
- default rustdoc surface: `PASS 0/0`;
- authentic release-profile replay-versus-forced runner parity:
  `PASS 1/1`, including internal owner/backtracking records and exact
  HBP/WAT/PASS bytes;
- LSE feature-enabled warnings-denied Clippy, affected crate checks,
  formatting, diff hygiene, graph/custody/audit reconciliation, and file-size
  gates: `PASS`.

Those passes establish the rejected candidate's reviewed correctness. They do
not override the subsequent release failure or approve current production.

## Release-run evidence

Direct inspection of
`artifacts/terminal-heavy-gates/component_dependency_replay_candidate_3run.log`
finds:

- exactly one `RUN 1 BEGIN` and one `RUN 1 EXIT 101`;
- no run-2 or run-3 begin;
- one explicit
  `RUNS_2_AND_3 NOT_RUN_AFTER_FAILED_CONJUNCT` disposition;
- no benchmark JSON record;
- the exact failed assertion requiring occupancy 2, soil 6, counts
  `58/14/16/28`, and completed status;
- unchanged candidate Rust manifest
  `039a312502a5e6ef442b1e81ac78b988141199f6283fedcc86518ba78ba61abc`
  before and after run 1; and
- binary
  `openwepp_runner-ce7ba1c0f7527921`, independently rehashed to
  `f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`,
  exactly matching the raw log.

The heavy release command was not rerun during this verification.

## Full-revert and residue verification

All seven candidate-only extracted/test-support paths from the 16-path manifest
are absent:

- the LSE replay test-support module;
- the three extracted LSE replay test modules; and
- the three extracted runner audit/internal/file-parity modules.

The restored solver evaluation, solver, transaction, bridge, solver-test, runner
manifest, and runner qualification hosts differ from candidate hashes as
expected. Static searches of the restored production/test hosts find none of
the seven required v31 declarations:

- `CoveredComponentTemperatureDependencyGraph`;
- `ValidatedCoveredComponentReplaySweepBase`;
- `ValidatedCoveredComponentProbeReplay`;
- `CoveredComponentDependencyReplayAudit`;
- `covered_component_temperature_probe_residuals`;
- `begin_covered_component_dependency_replay_audit`; or
- `take_covered_component_dependency_replay_audit`.

The LSE `test-support` feature, orchestrator dev-dependency, and private
transaction projection accessors remain intentionally. They are not v31
residue: the package prospectively authorized them before this candidate for
private parity projection, and the retained orchestrator shadow consumer still
uses the accessors. The runner's v31-only feature enablement and all v31 modules
are gone.

The expected-red test fails on exactly the seven absent declarations. This is
the required post-revert state, not an implementation failure accidentally
being promoted.

## Current-manifest forensic reconciliation

The documented current-manifest recipe over sorted changed/untracked Rust files
produces 179 entries and exact SHA-256
`2813f6e8faabb9408bac5e59b9271626ff5bcdc7fe49ab6dda810d3a1c3eee0d`.
The current authority-test entry is
`9250d0dd7e5335cb866bc7d4057fcc291e2746436d715bdaddb2c205a04a2451`.

Forensic replacement of only that entry by the independently reconstructed
pre-v31 authority-test hash
`912bb3deae3708f681a82417a631ebf6dcb7079e84ab64542ebbba00e8772096`
produces exact aggregate
`78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`,
matching both frozen baseline before/after markers. This independently confirms
that the apparent aggregate delta is the authorized post-baseline authority-test
patch sequence, not retained component-replay production semantics.

## Bounded checks run

- Current Rust manifest reconstruction: `PASS`,
  `2813f6e8...ee0d`.
- Frozen-manifest one-entry forensic reconstruction: `PASS`,
  `78d756be...bbbe`.
- Captured 16-path candidate-manifest record: `PASS`,
  `edc3f0b9...be71`.
- LSE nextest: `PASS 140/140`.
- Exact v31 structural seam: `EXPECTED_RED`, exit `101`, exactly seven
  absent declarations.
- LSE, hillslope-orchestrator, and runner all-target `cargo check`: `PASS`.
- Workspace formatting: `PASS`.
- Whole-worktree `git diff --check`: `PASS`.
- Candidate path, symbol, feature-consumer, raw release-log, and cached-binary
  audits: `PASS`.

The only observed build warning is the known unrelated dead-code warning for
`CoveredTerminalTrialRequestV1::{coupling_iteration, ending_snow_hint}`.

## Final disposition and residual risk

Revision 31 remains valid authority but is unimplemented. Reintroducing the
reviewed candidate would require a new exact candidate, independent review, all
behavioral evidence, and the unchanged release gate; weakening or synthesizing
the missing authentic sweep is forbidden.

The package's final documents consistently report bounded retained correctness,
failed release/performance qualification, and terminal `HOLD`. The component
readiness matrix's pending-final-verification row is a truthful sequencing
marker until both terminal verifier artifacts are incorporated; it does not
claim production readiness.

No safe, credible local replay/cache/validation-elision route remains capable of
closing the orders-of-magnitude package gap under current workload, budgets,
solver architecture, and science authority. Safe continuation now requires an
owner decision to revise the runtime premise/qualification protocol or
authorize a materially different canonical solver/evaluation architecture and
new contract/work-package scope. Evidence-only lifecycle reconciliation may
proceed, but it cannot change the `HOLD`.

Final disposition: `APPROVE TERMINAL HOLD`. This does not approve release,
performance qualification, exact-workspace correctness, or package completion.
