# Stage 3 native-vegetation laned watershed throughput recovery

Status: `QUEUED — SCAFFOLDED — PRE-IMPLEMENTATION`

Execution mode: `package-end-to-end after explicit kickoff`

Implementation intent: `kernel/runtime architecture correction + performance qualification`

This is a living ExecPlan maintained under `docs/codex_exec_plans.md`. The
package is authorized by owner direction on 2026-09-01 as the replacement for
the suspended workspace-gate numerical work. Scaffolding does not authorize
production edits or resumption of the predecessor package.

## Progress

- [x] 2026-09-01: scaffold replacement package and roadmap entry.
- [ ] Freeze the inherited source identity, complete dirty-diff inventory, and
  reconcile every v33--v57 production/runtime seam.
- [ ] Establish numeric runtime budgets and a representative workload ladder.
- [ ] Establish physically justified solver and conservation tolerance
  authority before production edits.
- [ ] Amend architecture/science authority and record contract-derived expected
  reds for the selected replacement.
- [ ] Replace the accretive runtime chain and remove superseded production
  branches.
- [ ] Qualify focused regimes, full-day/full-season workloads, Lane D MOFE, and
  the representative 100-year workload.
- [ ] Complete exact-diff reconciliation, dual review, dual verification,
  disposition, and delivery.

## Objective

Recover production throughput for the coupled non-CoE Stage 3 snow,
native-vegetation/ET, soil, and Lane D per-OFE runtime so the engine has a
credible path to typical watershed workloads of approximately 5,000
hillslopes, at least 10 OFEs per hillslope, and 100 years of climate.

The package must replace the v33--v57 accretive solver architecture with one
canonical solver per explicitly defined physical regime. It must separate
continuous numerical convergence from exact discrete identity and custody,
derive tolerances from physical/numerical error scales, bound physical-map
cost, and prove the real runner consumes the replacement path.

## Predecessor disposition and intake truth

The predecessor
`20260830-workspace-gate-hold-lift-001` is suspended, not complete. Canonical
r151 failed at `1800..1860 s` inside the frozen temperature-primary
safeguarded solve after `5:09.55` wall with peak RSS `442368 KiB`. It did not
complete one simulated day, so accepted/rejected counts, completed step-width
distribution, qualified runtime, and final water/energy closure are
unavailable. V56 focused 10/10 and V57 focused 6/6 plus contract 2/2 evidence
do not establish canonical acceptance.

The inherited worktree was reported at suspension as 90 modified and 98
untracked entries. Phase 0 must freeze the actual kickoff identity and diff;
the reported counts are historical intake evidence, not a terminal manifest.
No V58, numerical successor, replacement implementation, commit, or push was
performed by the predecessor.

## Binding architecture rule: no accretive solvers

This package applies accepted ADR-0044 and
`docs/standards/numerical-solver-architecture.md`.

Production kernels must not use accretive solver dispatch. A successor solver
may not retain earlier solver versions as sequential, eligibility-based,
convergence-based, or failure-recovery fallbacks.

- One canonical solver is allowed for each explicitly contracted physical
  regime.
- Regime selection occurs before iteration from physical state and authoritative
  domain predicates, never from solver history or failure chronology.
- A successor replaces the superseded implementation in its regime. Superseded
  production branches and selectors are deleted, not disabled or left behind.
- Nonconvergence returns a typed error or invokes the one canonical adaptive
  time-support response. It may not invoke an older solver.
- Temporary comparison implementations may exist only in test/diagnostic code,
  must be unreachable from production, and require a named deletion gate.
- Runtime version labels may describe persisted schema migrations; they may not
  encode a production cascade of historical numerical algorithms.
- Static anti-accretion tests must fail if a superseded solver becomes reachable
  from the production runner.

This rule is an owner requirement. It cannot be relaxed merely to preserve the
current worktree, pass a fixture, or avoid deleting historical implementation.

## Protected scientific and runtime requirements

The following are hard requirements throughout characterization,
implementation, and qualification:

1. **Lane D MOFE:** retain native upstream-to-downstream per-OFE Lane D state,
   transfer custody, and routing. Hillslope aggregation, daily aggregate
   substitution, or single-OFE collapse cannot carry a multi-OFE claim.
2. **Native vegetation and ET:** retain the native process implementation
   governed by `SC-VEGETATION-001`, including its admitted ecosystem-physics
   derivation and ET coupling. Legacy vegetation, PMET-only substitution,
   parameter-default shortcuts, or a surrogate canopy process cannot carry the
   production claim.
3. **Non-CoE Stage 3 snow:** Stage 3 remains the snow energy/phase process under
   qualification. CoE may be a diagnostic comparator only and cannot generate
   production melt, act as a fallback, or form a dual-owner path.
4. **Frozen litter:** retain the admitted frozen forest-litter process and
   source authority, including
   `references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf`. Performance
   work may change representation or scheduling, not substitute litter physics.
5. **Conservation and custody:** retain typed conservation, phase, topology,
   receipt, restart, rollback, event, and owner-custody invariants. Exact
   discrete identities remain exact; continuous-state convergence does not
   become bit-exact merely because receipts are authenticated.

## Numerical tolerance policy

No production solver edit may begin until `artifacts/tolerance-authority.md`
is accepted in package review and the applicable canonical contracts are
amended first.

The tolerance design must:

- distinguish nonlinear iteration stopping error, temporal truncation error,
  physical ledger closure, constitutive-domain guards, and discrete/custody
  equality;
- use dimensional absolute/relative scales tied to forcing precision,
  represented storage/flux scales, support duration, constitutive sensitivity,
  and independently reconstructed conservation limits;
- document why each tolerance is materially below the smallest process signal
  the model is expected to resolve without demanding meaningless binary64
  equality;
- prohibit exact-bit fixed points for continuous temperatures, fluxes,
  enthalpies, vapor, or CN heat unless a separately approved authority proves
  exactness is physically and numerically necessary;
- retain exact equality for identifiers, ordering, branch/event identity,
  exact-one ownership, duplicate-transfer detection, and other genuinely
  discrete invariants;
- define a small, measured physical-evaluation budget and a typed outcome when
  it is exhausted; and
- demonstrate sensitivity of accepted outputs and closure to the selected
  tolerances across frozen, mixed-phase, snow-free, wet-canopy, and Lane D
  multi-OFE regimes.

Tolerance loosening without this evidence is prohibited. Conversely, retaining
an inherited tolerance solely because it is stricter is not acceptable
authority.

## Throughput contract

Before production edits, `artifacts/performance-budget.md` must define numeric
budgets, hardware/source identity, measurement method, and pass/fail thresholds
for:

- physical-map evaluations and solver iterations per accepted OFE support;
- CPU and wall time per OFE-day for snow-free, frozen, mixed-phase, and
  thaw/refreeze workloads;
- native-vegetation/ET and Stage 3 cost shares;
- Lane D 1-, 10-, and greater-than-10-OFE scaling;
- memory per active hillslope/OFE and bounded temporary allocation;
- one complete day, one complete snow season, one complete representative
  10+-OFE year, and one complete representative 10+-OFE 100-year hillslope;
- scaling projection to at least `5,000 * 10 * 365.25 * 100 = 1,826,250,000`
  OFE-days, with explicit concurrency and I/O assumptions.

The owner-reported current posture is approximately 30,000 times too slow.
That statement is a severity bound, not a benchmark result. Phase 1 must
reproduce and normalize the gap on stable release binaries before selecting
the terminal budget. A sub-day extrapolation alone cannot qualify throughput.
The 100-year single-hillslope workload must execute; the 5,000-hillslope result
may be a measured linear scaling projection because watershed scheduling is a
wepppy concern, but its assumptions and confidence bounds must be explicit.

## Defect and correction authority envelope

| ID | Observed condition | Required disposition |
| --- | --- | --- |
| `THROUGHPUT-001` | R151 consumed 309.55 seconds and failed after only 1,860 simulated seconds; the target workload is billions of OFE-days. | Establish normalized release-mode costs, select hard budgets, and make every qualified workload meet them. |
| `SOLVER-ACCRETION-001` | V33--V57 accumulated guarded numerical successors and retained prior paths. | Replace the chain with one canonical solver per physical regime and prove superseded production paths are unreachable or deleted. |
| `NUMERICS-TOLERANCE-001` | Continuous solves pursue exact receipt/state witnesses beyond useful physical resolution. | Contract physically justified convergence/closure scales and remove bit-exact continuous-state admission. |
| `R151-FROZEN-SOLVE-001` | V57 reaches the frozen temperature-primary solve, which fails before canonical completion. | Close the valid frozen regime through the replacement architecture, not V58 or another specialization. |

Allowed correction classes are profiling/benchmark instrumentation,
architecture and science-contract amendments, contract-derived tests,
replacement solver/runtime implementation, deletion of superseded solver
branches, performance-preserving data-layout/batching work, typed error and
adaptive-controller correction, restart migration if an actually published
schema requires it, and package evidence.

No implementation may use proxy physics, reduced-process stand-ins, hidden
fallbacks, threshold-only test weakening, or a compatibility wrapper as the
production closure path.

## Included scope

- exact source/diff and runtime-path inventory of the preserved worktree;
- release-mode profiling with result-blind counters;
- workload fixtures spanning snow/vegetation/Lane D regimes;
- solver architecture, numerical tolerance, evaluation-budget, and adaptive
  support policy for the coupled Stage 3/native-vegetation/LSE/soil path;
- removal of v33--v57 production dispatch and obsolete solver-only restart or
  carry surfaces after publication/migration audit;
- data-layout, allocation, batching, and repeated-evaluation elimination inside
  openWEPP;
- real runner and downstream Lane D/ledger consumer proof;
- focused, frost-profile, representative long-run, and critical terminal gates;
- dual independent review, dual verification, and exact-diff disposition.

## Excluded scope

- wepppy GIS, watershed delineation, climate acquisition, run-state, and job
  scheduling;
- replacing Lane D with hillslope aggregation or legacy MOFE routing;
- replacing native vegetation/ET with legacy vegetation, PMET-only ET, or a
  surrogate process;
- reverting snow melt/phase ownership to CoE or enabling dual CoE/Stage 3
  generation;
- changing frozen-litter constitutive physics without separate source/contract
  authority;
- empirical vegetation calibration or claims of site-specific fitness;
- weakening conservation, custody, event, topology, restart, rollback, or
  fail-closed guards to obtain speed;
- adding V58 or any new historical-version solver fallback;
- claiming the suspended workspace-gate package complete.

## Deliverables

- `artifacts/required-reading-map.md`;
- `artifacts/inherited-worktree-and-runtime-inventory.md`;
- `artifacts/solver-chain-deletion-map.md`;
- `artifacts/performance-budget.md`;
- `artifacts/tolerance-authority.md`;
- `artifacts/workload-and-benchmark-matrix.md`;
- `artifacts/operand-lineage.md`;
- `artifacts/pre-implementation-gates.md`;
- `artifacts/implementation-and-focused-validation.md`;
- `artifacts/long-run-throughput-and-closure.md`;
- `artifacts/real-consumer-proof.md`;
- `artifacts/line-count-governance.md`;
- `artifacts/gate-results.md`;
- independent `review-a.md`, `review-b.md`, `verification-a.md`, and
  `verification-b.md`;
- `artifacts/finding-disposition.md` and `artifacts/final-disposition.md`.

## Dependencies and authority

Core predecessor and governance:

- `AGENTS.md`;
- `docs/codex_exec_plans.md`;
- `docs/defect_closure_execplans.md`;
- `docs/work-packages/AGENTS.md`;
- `docs/standards/testing-and-gate-strategy.md`;
- `docs/standards/kernel-work-package-preparation.md`;
- `docs/standards/numerical-solver-architecture.md`;
- `docs/decisions/0044-prohibit-accretive-production-solver-dispatch.md`;
- `docs/work-packages/20260830-workspace-gate-hold-lift-001/package.md`;
- predecessor r151 disposition and gate results.

Process/runtime authority:

- `SC-SNOWENERGY-001`, `SC-VEGETATION-001`,
  `SC-LANDSURFACEENERGY-001`, `SC-COUPLEDTIME-001`,
  `SC-OFEROUTE-001`, `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and
  `SC-SYSTEM-001` as selected by the exact implementation diff;
- `docs/decisions/0033-ofe-by-ofe-overland-flow-routing.md`;
- `docs/decisions/0037-abandon-hybrid-implicit-stepping.md`;
- `docs/decisions/0025-array-native-hillslope-day-frame.md`;
- `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` where baseline process provenance
  applies;
- `references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf` and the frozen
  forest-litter authority package.

## Prospective write set

Scaffold/intake writes:

- `docs/ROADMAP.md`;
- `docs/work-packages/README.md`;
- this package tree.

Phase 0 may read all preserved changes but may write only this package tree.
Before any benchmark/test scaffold, architecture ADR, canonical contract, or
production edit, the package must amend this section with exact files selected
by the deletion map and run `tools/agents/find-agents --for` on every selected
path.

Expected candidate surfaces, not authorization to edit before that amendment:

- canonical contracts named above and their registry/contract tests;
- `crates/openwepp-hillslope-orchestrator/src/v11_covered/` solver and
  convergence modules;
- Stage 3 attachment/execution and exact owner/restart seams actually retained
  by the replacement;
- `crates/openwepp-vegetation/`, land-surface-energy, coupled-time, and Lane D
  modules only where profiling proves in-scope cost or coupling ownership;
- runner benchmark/fixture paths and affected integration tests;
- release authority impact-map entries selected by the exact diff.

Broad directory ownership is forbidden. Each production phase must name exact
files before edits; overlapping user/other-agent changes must be preserved.

## Phase plan and gates

### Phase 0 — frozen intake and architecture inventory

Freeze exact source identity and dirty diff, map every reachable solver branch,
classify v33--v57 symbols as `delete`, `migrate`, `test-only`, or
`schema-compatibility`, trace the real runner through Stage 3, native vegetation,
soil/LSE, and Lane D, and record what still reads every old path. No production
code changes.

Gate: the deletion map has no unclassified production branch, and the package
write set is amended to exact files.

### Phase 1 — measured budgets and tolerance authority

Build exact release binaries, implement only result-blind benchmark/profiler
instrumentation, run the workload ladder, quantify cost centers and scaling,
derive numeric performance budgets, and author dimensional tolerance evidence.

Gate: `performance-budget.md` and `tolerance-authority.md` contain numeric
pass/fail values, reproducible commands, hardware/source identity, sensitivity
evidence, and review acceptance. No production solver change precedes this
gate.

### Phase 2 — contract-first replacement design

Apply ADR-0044, amend all affected canonical science contracts, add
contract-derived/static anti-accretion tests, and record
expected pre-implementation failures. The design must name physical regimes,
one solver per regime, convergence/adaptive behavior, evaluation budget,
conservation/custody boundaries, and restart migration.

Gate: applicable kernel profiles pass, contract tests fail only on absent
replacement production seams, and independent design reviews accept the
replacement without an old-solver fallback.

### Phase 3 — replacement and deletion

Implement the smallest canonical solver/runtime architecture, delete
superseded production paths in the same increment, preserve required process
models, and run focused regime, closure, restart, and real-consumer tests.

Gate: anti-accretion scans prove no production reachability of superseded
solvers; each physical regime has exactly one solver; focused correctness and
performance budgets pass.

### Phase 4 — representative qualification

Run complete day, snow-season, 10+-OFE year, and 10+-OFE 100-year workloads in
release mode. Reconstruct conservation independently, record native vegetation
ET and non-CoE Stage 3 activation, prove Lane D per-OFE execution, measure
scaling, and project the 5,000-hillslope workload with confidence bounds.

Gate: every declared runtime/memory/evaluation budget passes; no run is aborted,
silently narrowed, or qualified solely from an incomplete fixture.

### Phase 5 — terminal gates and disposition

Reconcile exact diff intent, execute the critical terminal validation selected
under the testing strategy, complete dual Rust/science reviews and independent
verification, disposition every finding, and report truthful completion or
HOLD.

## Exit criteria

- the inherited v33--v57 production cascade is removed; no V58 or replacement
  cascade exists;
- one canonical solver per contracted physical regime is statically and
  dynamically proven from the real runner;
- nonconvergence invokes only the typed failure/canonical adaptive response;
- Lane D operates per OFE on representative 10+-OFE workloads;
- native `SC-VEGETATION-001` vegetation and ET are active and no legacy/PMET-only
  substitute carries the claim;
- Stage 3 is the active non-CoE snow process and no CoE generation/fallback is
  reachable for the claim;
- frozen-litter authority and physics remain intact;
- physically justified tolerances and evaluation limits pass all sensitivity,
  domain, and anti-evasion vectors;
- complete day, season, year, and 100-year hillslope workloads finish within
  the Phase-1 numeric budgets;
- the 5,000-hillslope/10+-OFE/100-year projection is reported with measured
  scaling and explicit concurrency/I/O assumptions;
- mass, energy, phase, liquid, receipt, restart, owner, topology, event, and
  rollback closure pass independent reconstruction;
- warnings-denied affected Clippy and every exact-diff-selected correctness gate
  pass; critical/full-workspace gates run on the exact terminal source;
- all 2,000+/3,000+ line-count findings are dispositioned under repository
  policy;
- both independent reviews and both verifications accept the terminal evidence;
- the package status, roadmap, catalog, prompt, and final disposition agree.

## Review, verification, and subagent requirement

This package explicitly authorizes subagent spawning/delegation to:

- `comparator_suite_runner` for heavy full-workspace, long-run workload,
  comparison, and terminal gate execution, returning compact metrics and log
  paths with read-only source access;
- `rust_code_reviewer` for independent correctness/architecture review;
- `rust_qa_reviewer` for independent test, maintainability, and performance-gate
  review; and
- bounded worker agents only after exact file ownership is recorded, with no
  permission to revert or overwrite other worktree changes.

Heavy batch/closure runs must use `comparator_suite_runner` when available; the
parent must record unavailability before running an equivalent heavy gate
locally. Reviews are independent and findings require explicit disposition and
verification.

## Security and external-state gate

Expected security impact is `NONE`. If implementation changes subprocess,
untrusted input, protected assurance, or external data surfaces, amend the
package before edits and execute the applicable security/anti-evasion gates.

No network action, branch creation, commit, push, wepppy edit, or deployment is
authorized by this scaffold.

## HOLD boundaries

Effort, slow tests, implementation size, dirty-worktree complexity, or a
partially improved benchmark are not legitimate HOLD boundaries. A HOLD is
permitted only for missing/contradictory science authority, a required owner
choice that materially changes the runtime budget, unavailable representative
input evidence, an out-of-envelope dependency, or an external-state/security
boundary. Any HOLD requires `artifacts/hold-legitimacy-audit.md`.
