# Kernel-profile and readiness matrix

Evidence mode: `Static + Ran + Expected-red`

Contract set: `SC-LANDSURFACEENERGY-001` revision 31,
`INV-LANDSURFACEENERGY-164`, `OBL-LANDSURFACEENERGY-C-020`.

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
implementation_readiness = HOLD
```

| Profile obligation | Status | Evidence and rationale |
|---|---|---|
| Canonical authority and registry | `PASS` | Contract and lifecycle index agree on revision 31, active status, date, and the bounded component-temperature dependency-replay amendment. |
| State and algorithm surface | `PASS` | One immutable generation/all-input/caps/frozen/graph/trial/map/solve/iteration/sweep-bound base serves the sweep; fresh coordinate/sign/perturbation/exact-probe/actual-stencil-bound capabilities are consumed per signed probe and dropped with zero mutation. Typed identities/borrows or compact seals provide exact custody; costly `Debug`, length-only, clone or repeated full-scan proxies do not. |
| Topology generality | `PASS` | Formulae are parameterized by occupancy count `N` and soil count `S`; graph creation cannot inspect branch outcomes or hardcode the authentic `N=2`, `S=6` fixture. |
| Dependency completeness | `PASS` | Stable node IDs and complete direct-edge families bind both wet regions/finalizations, all lower routing, reciprocal longwave, route mismatch, terminal release/stemflow, leaf/CI/hydraulics, lower-to-shared reductions, shared-to-tolerance, normalization and output. CTDR-RRB-003 makes the ten previously implicit source-real direct edges explicit, and the authority test asserts them. An independent exact edge/hash oracle, not reachability-only sampling, governs the versioned inclusive closure and conservative unknown edges. |
| Shared canonical evaluator | `PASS` | Complete and replay paths must consume one shared canonical node/tail implementation. Mirrored or duplicated physical, tolerance, residual, branch or output arithmetic is forbidden. |
| Numerical invariance | `PASS` | Probe coordinates, perturbations, bounds, minus-before-plus order, finite-difference arithmetic, residual order, dense matrix, LU/pivots, backtracking, convergence, errors, trajectory, diagnostics, and output remain unchanged. |
| Prohibited alternatives | `PASS` | Analytic/AD derivatives, coloring, simultaneous perturbations, sparse LU, cross-boundary caches, approximation, fallback, and hardcoded fixture topology are expressly forbidden. |
| Probe accounting | `PASS` | Generic fully centered accounting is `2*(10*N+3+S)` logical, `2*(1+S)` anchor, `8*N` replay, and `12*N+4` complete; `58=14+16+28` is one named authentic sweep, not a release total. Per-sweep and release aggregation cover potential/final, centered/inward and every source-real completion class. A short-circuit class is present only if the canonical solver has that real path. |
| Branch, bound, and error posture | `PASS` | Ordinary ineligibility selects complete evaluation only before replay; private integrity mismatch fails typed; every post-start node error returns directly. The normative matrix classifies current leaf as canonically crossable, maximum leaf and other named fallible nodes as noncrossable absent an authentic counterexample, and remaining nodes as infallible. Crossable errors require source-real first-error/rollback pairs; noncrossable rows require implication plus authentic success fields; infallible rows never use synthetic errors. The differential corpus catches every naturally occurring error, and mutation/fault hooks are forbidden. |
| Audit identity and lifecycle | `PASS` | Map, solve, iteration and sweep identities come from distinct authenticated lifecycle positions; copied ordinals/proxies are forbidden. Completed means every required probe finished, failed means the source-real first error with actual counts, rejected-before-probe remains a stencil outcome, and absent solver states are not fabricated. |
| Conservation and custody | `PASS` | No equation, owner, receipt, storage, publication, or output change is admitted; full evaluation/solve/output bit equality and rollback are mandatory. |
| Units/constants/tolerances | `NOT_APPLICABLE` | No new dimensional symbol, unit, alias, conversion, constant, parameter, tolerance, or normalization. Unit-compliance lint passes. |
| Calibration/identifiability | `NOT_APPLICABLE` | No observation, objective, empirical calibration, validation, or identifiability claim changes. |
| External constitutive suite | `NOT_APPLICABLE` | No constitutive equation or required external-authority cohort changes. |
| Contract-derived assertion | `PASS` | Focused revision-31 authority assertion passes 1/1. |
| Structural production seam | `EXPECTED_RED` | Seven graph/sweep-base/probe/audit production items are absent. The test masks comments plus cooked/raw strings and character/byte-character literals without masking lifetimes, parses item bodies, and rejects every arbitrarily stacked/multiline `cfg`/`cfg_attr`-gated top-level item through visibility/modifier prefixes. It classifies unconditional top-level item presence only; it makes no connectivity, invocation, consumption, counter, or behavior claim. |
| Full contract target isolation | `PASS_WITH_EXPECTED_RED` | 24 tests pass, exactly the named revision-31 structural seam fails, zero ignored, and no unrelated contract assertion fails. |
| Binding Exposure Index | `PASS` | Strict lint reports 15 fully consolidated rows, including `LSE-V31-COMPONENT-TEMPERATURE-DEPENDENCY-REPLAY`. |
| Performance baseline | `PASS` | Three exact CPU-0 runs preserve source manifest `78d756...bbbe` and binary `9a91c8...73f`; raw total/potential values yield medians `4903570/353431 us` and conjunctive candidate ceilings `4803570/253431 us`. Evidence is `component_dependency_replay_baseline_3run.log`. |
| Rejected production increment | `PASS` | Static inspection after the ordered full revert finds all seven replay graph/evidence/audit/function declarations absent; no partial dependency-replay production mechanism is retained. |
| Production implementation and behavior | `FAIL_REVERTED` | A fresh graph-driven implementation passed `14/14` focused, `154/154` full LSE, complete differential/full-solve/runner/file parity, and dual implementation review at manifest `edc3f0...be71`, but failed the binding release audit and was fully reverted. Production again contains none of the seven declarations. |
| Implementation review/disposition | `PASS_FOR_REJECTED_CANDIDATE` | Seven immutable review cuts closed every accepted correctness, graph, custody, error, lifecycle, full-owner/backtracking, evidence, and feature-lint finding. Both final reviewers approved the exact candidate; approval does not override the failed release conjunct or retain reverted code. |
| Corrected manifest custody | `PASS` | The ordered eight-file manifest includes the contract, registry, authority test, both parent-owned stale-pin integration corrections, and all three component-replay artifacts. |
| Independent corrected contract re-review | `PASS` | Both corrected-authority reviewers and both preimplementation verifiers accepted ordered authority manifest `767bc190...b1583`. |
| Independent dual verification | `PASS_FOR_TERMINAL_HOLD` | Both final post-revert verifiers independently confirmed the run-1 failure, correct omission of runs 2/3, complete revert, seven-symbol expected red, `140/140` LSE, exact `2813f6...ee0d` current manifest, forensic `78d756...bbbe` reconstruction, four sequencing defects, and legitimacy of terminal HOLD. |
| Release keep/revert | `FAIL_REVERTED` | Candidate run 1 used unchanged source `039a3125...1abc` and binary `f9386eec...eeaf` but the real release aggregate had no authentic completed `N=2,S=6` `58/14/16/28` sweep. Exit was `101`; runs 2/3 were not run after the failed conjunct, and v31 was fully reverted. |

## Current disposition

Corrected same-v31 authority, frozen baseline, rejected-candidate correctness,
the failed release-retention result, full revert, and dual terminal verification
are established. The structural seam is again exactly expected red.
Implementation/release disposition is `FAIL_REVERTED`; package disposition is
terminal `HOLD`. No production-performance or release-qualification claim is
made.
