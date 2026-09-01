# Numerical solver architecture

Status: Active

Authority: ADR-0044 and owner direction dated 2026-09-01

Scope: every production numerical solver, nonlinear iteration, fixed-point
iteration, root finder, implicit stepper, and coupled process solve in openWEPP.

## Rule

Production kernels must not use accretive solver dispatch. A new solver may not
retain earlier solver versions as sequential, eligibility-based,
convergence-based, or failure-recovery fallbacks.

Each physically defined regime has one canonical solver. A successor replaces
the superseded production implementation in the same regime. If that solver
does not converge, control may only:

1. use the bounded nonconvergence response defined by that same canonical
   algorithm;
2. ask the canonical adaptive time-support controller to retry the same
   physical regime and solver on a different support; or
3. return a typed failure to the owning orchestrator.

It may not invoke an older solver, a prior contract version, a compatibility
implementation, or a newly added special-case solver.

## Definitions

`Physical regime`
: A domain selected before numerical iteration from authoritative physical
  state and process predicates, such as snow-free, strictly frozen, or
  mixed-phase. Solver failure, iteration count, residual history, exact-bit
  mismatch, or prior algorithm outcome is not a physical regime.

`Canonical solver`
: The single production algorithm specified by the owning contract for one
  physical regime, including its bounded internal steps and failure behavior.

`Accretive solver dispatch`
: Runtime control flow that tries, retains, or re-enters multiple historical or
  successor solvers for the same physical regime based on eligibility misses,
  nonconvergence, exact-witness failure, iteration history, or error recovery.

`Successor`
: A correction or replacement of an existing solver for the same physical
  regime. A successor is incomplete while the superseded production path
  remains reachable.

## Allowed composite algorithms

A canonical solver may contain normal bounded numerical components such as a
line search, safeguarded Newton step, trust region, preconditioner, bracketing
step, or continuation stage when all of the following hold:

- the owning contract specifies them prospectively as one algorithm;
- they solve the same authoritative equations and residual system;
- their selection/order is algorithmic, not a chain of historical versions;
- one shared evaluation and runtime budget covers the complete algorithm;
- one convergence and admission policy governs the final result; and
- failure cannot transfer control to a superseded implementation.

Renaming historical solvers as stages, strategies, specializations, or rescue
steps does not satisfy this exception.

## Legitimate physical-regime dispatch

Different solvers are permitted only for materially different physical regimes
when canonical contracts define:

- mutually exclusive or explicitly ordered physical predicates evaluated
  before iteration;
- the equations/state owned by each regime;
- boundary behavior and event transitions between regimes;
- a unique solver for each selected regime; and
- typed handling for an unsupported or ambiguous state.

A regime selector may not test whether another solver converged. A coordinate
or phase transition discovered during a solve follows the current algorithm's
contracted event/adaptive response; it does not dispatch a historical solver.

## Exactness and convergence

Exact identity remains appropriate for discrete authority: owner IDs,
transaction IDs, ordering, topology, event identity, exact-one custody,
duplicate-transfer detection, and other genuinely discrete state.

Continuous temperatures, fluxes, enthalpies, heat terms, water masses, vapor,
and nonlinear residuals use physically and numerically justified dimensional
tolerances. Bit-exact continuous-state or receipt fixed points require separate
explicit science/numerical authority; exactness must not be introduced merely
to make authenticated receipts equal.

## Replacement and rollback

- Cutover replaces and deletes the superseded production solver in one stable
  increment.
- Source and release history are the rollback mechanism. A live old solver is
  not rollback.
- Persisted schema readers may decode historical formats and migrate them into
  canonical current state. A schema version may not select a historical
  numerical solver.
- A temporary comparison/shadow implementation may exist only outside
  production admission/publication, must be visibly diagnostic, and needs a
  named owner and deletion gate.
- Feature flags may stage measurement or an atomic cutover; they may not create
  an indefinite production fallback chain or allow a failed new solver to run
  the old one.

## Work-package requirements

Before editing a production solver, the owning package must record:

1. a solver/regime table naming every production-reachable solver;
2. a call-path inventory from the real runner;
3. a deletion/migration map for every superseded path;
4. equations, residuals, tolerance authority, evaluation budget, and typed
   nonconvergence behavior;
5. contract-first expected-red evidence for changed kernel behavior;
6. representative correctness, conservation, and performance budgets; and
7. the exact files authorized for implementation and deletion.

Closure requires direct evidence that:

- every regime has exactly one production solver;
- failure injection cannot reach an older/alternate solver;
- source/call-path scans find no superseded production selector;
- the real downstream consumer uses the canonical result;
- comparison/shadow code cannot admit or publish state;
- applicable workload and physical-evaluation budgets pass; and
- deleted solver tests are replaced by equation/regime/guard tests rather than
  retained as obligations for dead implementations.

## Current noncompliance quarantine

The preserved Stage 3 v33--v57 chain is known noncompliant evidence, not an
exception or accepted precedent. It is quarantined under
`20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001`.

Until that package removes it:

- no V58 or other successor/fallback may be added;
- no other process may copy its dispatch structure;
- work touching the chain may diagnose, benchmark, contract, replace, or delete
  it, but may not expand its production reachability; and
- no package may claim solver-architecture completion from focused tests while
  the chain remains reachable.

The quarantine ends only when terminal evidence proves the superseded
production paths are deleted or unreachable and the real runner uses one
canonical solver per contracted physical regime.
