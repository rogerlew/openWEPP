# PERFDEEP09 - Disabled-Path Iterative Defect Closure

Status: executed - `READY-FOR-R2`.

Package type: Defect-Closure ExecPlan / iterative performance blocker closure.

Defect ID: `PERFDEEP09-DISABLED-PATH-R2-BLOCKER`.

## Objective

Close the R2+ blocker left by PERFDEEP07 and PERFDEEP08: the default-disabled
H2637 hillslope endpoint remains above the P0 gate `<= 676.67 s` even when all
PERFDEEP opt-ins are off. This package must benchmark, attribute, remediate,
and repeat inside a single defect-closure envelope until it can truthfully
report `READY-FOR-R2`, `NO-GO`, or a legitimate DC `HOLD` at a named boundary.

The package must not implement R2+ direct-frame hydrology, a direct executor,
runtime schema cutover, publication cutover, or default activation. Its purpose
is to remove the default-disabled regression that blocks those later packages.

## Rationale

PERFDEEP07 retained a cleanup that improved the default-disabled H2637 endpoint
from PERFDEEP05's `701.95 s` to `685.85 s`, but it failed the required
`<= 676.67 s` threshold. PERFDEEP08 tested disabled diagnostic-hook caching and
measured `691.93 s`, slower than the retained PERFDEEP07 point; that candidate
was reverted. The current blocker is therefore not a single known hook. It is a
diagnostic-first performance defect that needs a wider iterative closure loop:
measure the retained path, attribute cost to named mechanisms, land
same-envelope remediations, re-benchmark, and continue until the R2+ blocker is
cleared or a legitimate boundary is proven.

## Correction Authority Envelope

Observed violation:

- `PERFDEEP09-DISABLED-PATH-R2-BLOCKER`: with all PERFDEEP opt-ins disabled,
  current `main` has not produced a valid H2637 default-disabled three-run
  median `<= 676.67 s`. The best retained PERFDEEP07 single run is `685.85 s`,
  and the PERFDEEP08 candidate single run is `691.93 s`. This blocks R2+
  direct-frame runtime implementation under the R0/R1 planning package and
  revised array-native runtime architecture.

In-scope write set:

- `docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only if execution
  discovers that unblock criteria or architecture wording must be clarified
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs` only for benchmark-safe
  CLI plumbing, not user-facing semantics
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` only if profiling
  proves the blocker is there; touching this file requires line-count closure
  before final disposition because it is over 3000 lines
- `crates/openwepp-hillslope-orchestrator/src/scheduler/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**` only for
  disabled-path data-shape guards or tests; no physics authority changes
- `tests/integration/**` only for runner/CLI regression tests tied to this
  defect
- `tools/owcmp/**` or `tools/release/**` only for benchmark/comparator harness
  reuse; do not change authority-suite posture without running anti-evasion
  guards

Allowed production edit classes:

- remove, bypass, lazily initialize, or make strictly opt-in any dense,
  indexed, direct-frame, hot-symbol, writeback, trace, audit, scheduler, or
  publication compatibility machinery that is constructed or resolved on the
  default-disabled path only because of failed PERFDEEP opt-in work;
- replace per-day/OFE hot-path logical name lookups with precomputed ids only
  when output identity and fail-closed opt-in behavior are preserved;
- add instrumentation that is compile-time or environment-gated and absent from
  the measured default-disabled path;
- split files or extract helper modules when required by line-count governance.

Protected boundaries:

- no process-physics formula changes;
- no `SC-*` invariant changes unless the package is amended before production
  edits;
- no output schema, unit, metadata, publication meaning, or conservation-math
  changes;
- no default activation of PERFDEEP02/03/05/07 opt-ins;
- no direct-frame hydrology implementation, direct executor, R2+ runtime schema,
  or publication-ledger promotion;
- no silent fallback wrappers for missing dependencies or invalid inputs;
- no accepting an opt-in speedup while the default-disabled gate remains failed.

Authority:

- `docs/architecture/array-native-runtime-specification.md` owns the
  architecture direction and R2+ block.
- `docs/decisions/0025-array-native-hillslope-day-frame.md` owns the
  HillslopeDayFrame decision context.
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/`
  records the planning-only R0/R1 envelope and hold-lift conditions.
- PERFDEEP06/07/08 artifacts define the default-disabled gate, observed timing
  history, and rejected/slower candidates.

Conversion rule:

If this package establishes a reproducible root cause inside the declared
envelope, and the expected behavior is supported by the architecture authority,
existing contracts, and protected-output identity requirements, it must proceed
through focused test or benchmark evidence, production remediation, validation,
review, and disposition in this package. It may not close as `HOLD` because
more profiling is possible or because only one candidate has been tested.

Seven-gate bar:

1. Reproduction: the default-disabled H2637 blocker is reproduced on current
   `main` or replaced by a same-machine control baseline with binary SHA,
   command, seconds, RSS, and output identity evidence.
2. Mechanism: profiler, micro-benchmark, trace counter, or static proof reduces
   the blocker to a named mechanism or ranked set of mechanisms.
3. Ownership: the named mechanism lies inside the declared write set, or a
   protected boundary is proven with citation.
4. Authority: the expected remediation preserves architecture authority,
   contract authority, typed guards, and protected output identity.
5. Safety: the fix does not loosen guards, silently default values, change
   physics, change output meaning, or activate opt-ins by default.
6. Testability: a focused regression, benchmark, counter, or static proof can
   fail or identify cost before the fix and pass or remove cost after the fix.
7. Validation: after remediation, the package measures H2637 default-disabled
   endpoint timing and protected output identity; `READY-FOR-R2` requires a
   three-run median `<= 676.67 s`.

## Iterative Defect-Closure Loop

This package is intentionally iterative. After the initial baseline and
profile/micro-benchmark attribution, each candidate iteration must record:

1. hypothesis and attributed mechanism;
2. touched files and line-count implications;
3. pre-change timing/profile/counter evidence;
4. production or harness change;
5. focused tests and identity evidence;
6. screening H2637 timing result;
7. retain/revert decision with rationale.

If a retained candidate improves the endpoint but remains above `676.67 s`, the
package must continue the loop unless a legitimate `HOLD` boundary is reached.
If a candidate is slower, it must be reverted or justified as a prerequisite for
a subsequent same-package remediation before another endpoint gate is claimed.

## Scope

In scope:

- reproduce current default-disabled timing and protected output identity;
- build or reuse profile/micro-benchmark evidence for the retained default path;
- remediate in-envelope default-disabled overhead;
- keep all prior failed PERFDEEP islands explicit and fail-closed;
- record every rejected/slower candidate so the same patch is not repeated;
- run enough candidate iterations to clear the blocker or prove a legitimate
  DC boundary;
- update package artifacts, roadmap/catalog state, review, verification,
  disposition, and worker handoff.

Out of scope:

- R2+ direct-frame runtime implementation;
- opt-in endpoint activation work;
- process-physics changes;
- output publication schema or unit changes;
- broad style refactors unrelated to the attributed mechanism;
- closing by "profile next function" handoff.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/defect_closure_execplans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/package.md`
- `docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/package.md`
- `docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/artifacts/disposition.md`
- `docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/artifacts/perfdeep08-r2-blocker-disposition.md`
- `docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/artifacts/perfdeep08-rejected-candidates-ledger.md`
- `docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/package.md`
- `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/perfdeep07-disabled-path-baseline.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md`
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/perfdeep07-hold-lift-disposition.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`

Required before Rust edits:

- `crates/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `tests/AGENTS.md` before editing root tests
- `tools/owcmp/AGENTS.md` before editing or relying on owcmp tooling

On-demand source inventory:

- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-hillslope-orchestrator/src/consumer_boundary.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/**`

## Dependencies

- PERFDEEP07 and PERFDEEP08 are `HOLD`; both failed the same R2+ unblock gate.
- R0/R1 planning is complete but explicitly blocks R2+ runtime work until this
  default-disabled hold is closed or superseded.
- The revised runtime specification says compatibility-edge shaving is
  insufficient as a final architecture, but the disabled path must still be
  zero-cost before the next runtime package.
- Protected H2637 output identity surfaces from prior PERFDEEP packages remain
  required.

## Phase Plan

1. Populate required-reading and owned-file artifacts. Record byte budget and
   execution authority.
2. Establish a no-edit same-machine control baseline for current `main`:
   release binary SHA, manifest SHA, command, opt-in environment, seconds, RSS,
   and protected output checksums.
3. Profile and/or micro-benchmark the retained default-disabled path. Attribute
   at least the top endpoint costs to named mechanisms before production edits.
4. Enter the remediation loop. For each candidate, record hypothesis, evidence,
   patch, focused validation, identity, timing, and retain/revert decision.
5. Continue remediation until a candidate's screening run reaches or clearly
   crosses the P0 band, then run the three-run default-disabled H2637 endpoint
   gate with min/median/max seconds and RSS. Required median: `<= 676.67 s`.
6. Prove zero-cost-disabled statically and with counters/profile evidence where
   feasible. The proof must show opt-in-only structures are not constructed,
   resolved, refreshed, flushed, or published on the disabled path.
7. Run full closure gates when the P0 gate passes:
   `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo deny check`, scoped docs lint, and
   `git diff --check`.
8. Complete line-count governance, dual review, finding disposition, dual
   verification, roadmap/catalog updates, and worker handoff.
9. Close as `READY-FOR-R2`, `NO-GO`, or legitimate DC `HOLD`. `READY-FOR-R2`
   requires the P0 gate and all closure gates to pass. `HOLD` must name a DC
   boundary, not a next diagnostic step.

## Legitimate HOLD Boundaries

This package may close `HOLD` only when one of these boundaries is proven:

- the dominant mechanism is outside the declared write set and cannot be
  amended without crossing into R2+ direct-frame implementation;
- the correct remediation requires changing output meaning, physics authority,
  or canonical `SC-*` text that is not safely amendable inside this package;
- required benchmark evidence cannot be generated in the available environment;
- line-count governance requires a prerequisite split that is too large to
  combine with the performance correction without losing reviewability;
- a same-machine control proves the threshold failure is an external
  environment artifact rather than a code-path defect.

Forbidden `HOLD` states:

- "profile the next function";
- "try another candidate";
- "root cause is in the declared files but implementation is deferred";
- "candidate improved but remains above threshold and no boundary was proven".

## Acceptance Criteria

- Same-machine baseline/control evidence is recorded before production edits.
- Profile or micro-benchmark evidence names the attributed mechanism before
  each retained remediation.
- Candidate ledger records every retained, reverted, or rejected candidate.
- Default-disabled H2637 protected output identity passes for HBP, WAT, PASS,
  plot/loss, and manifest/provenance surfaces.
- At least three clean H2637 default-disabled no-UI endpoint runs are recorded
  after the final retained remediation with min/median/max seconds and RSS.
- The final three-run median is `<= 676.67 s`.
- Static and runtime evidence prove opt-in-only dense/direct-frame machinery is
  zero-cost when disabled.
- No direct-frame hydrology, direct executor, R2+ runtime schema, output
  publication cutover, or default opt-in activation is implemented.
- Full Rust closure gates pass before claiming `READY-FOR-R2`.
- Markdown lint and `git diff --check` pass for touched docs.
- Line-count governance is recorded and any touched 3000+ `.rs` file is split
  or dispositioned before closure.
- Dual reviews and dual verifications explicitly check Gate Evidence
  Non-Deferral, DC `HOLD` legitimacy, envelope adequacy, and protected-boundary
  integrity.

## Conservation / Output Acceptance

This package is performance-only but protects output surfaces. It may not
change publication operands, metadata meaning, normalization, units,
conservation math, or process physics.

If execution discovers a necessary output-meaning change, stop and amend the
package under the contract-first and conservation/publication rules before any
production edit.

## Contract-First Rule

No `SC-*` contract change is intended. If execution discovers that correcting
the performance defect requires changing invariant authority, guard semantics,
diagnostic attribution policy, output meaning, units, or process physics, stop
and amend the package before implementation.

## Security Impact Gate

No secrets, credentials, external network dependencies, or user data are in
scope. Do not weaken fail-closed behavior, typed error handling, validation
gates, output schema contracts, or serialization safeguards.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only profiler/benchmark runner, comparator runner, reviewer, and
verifier subagents for default-disabled H2637 endpoint runs, protected output
identity checks, heavy Rust closure gates, package artifact review,
line-count-governance review, and gate-legitimacy verification. Expected
outputs are compact metrics, log paths, and findings recorded in package
artifacts. Write access is limited to package artifacts unless this package is
explicitly amended.

When available, heavy batch/closure/comparator work should be delegated rather
than run on the parent agent. If delegation is unavailable, record that fact in
`artifacts/implementation-test-evidence.md` before running locally.

## Deliverables

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/perfdeep09-defect-authority-envelope.md`
- `artifacts/perfdeep09-benchmark-protocol.md`
- `artifacts/perfdeep09-profile-and-microbench-evidence.md`
- `artifacts/perfdeep09-attribution-loop.md`
- `artifacts/perfdeep09-remediation-iteration-log.md`
- `artifacts/perfdeep09-candidate-ledger.md`
- `artifacts/perfdeep09-h2637-identity-timing-evidence.md`
- `artifacts/perfdeep09-zero-cost-disabled-proof.md`
- `artifacts/perfdeep09-r2-blocker-disposition.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`

## Required Reading Budget

Local required-reading bytes total: `265417`.

Disposition: `OK` (`<=400000` bytes).

See `artifacts/required-reading-map.md`.

## Autonomy

Execute end-to-end when triggered. Do not stop after a single diagnostic run or
a single failed candidate. Continue the attribution/remediation loop until the
R2+ blocker is cleared or a legitimate DC boundary is proven. Do not ask the
user for next steps unless a hard blocker prevents truthful disposition.
