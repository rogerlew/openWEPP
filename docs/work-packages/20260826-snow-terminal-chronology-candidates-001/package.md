# Snow terminal chronology candidates

Status: `EXECUTING / TEST-ONLY RESEARCH / PRODUCTION UNCHANGED`

Date: `2026-08-26`

Package ID: `20260826-snow-terminal-chronology-candidates-001`

Plan class: defect-closure science characterization and qualification.

## Objective

Evaluate four successors left unqualified by
`20260826-snow-terminal-vapor-melt-phase-competition-001`:

1. chronology-resolved released phase ordering;
2. an event-driven pack/surface-frost hybrid;
3. time-resolved complementarity over an authoritative forcing trajectory;
4. a tagged surface-frost subtype inside the existing snow owner.

Select a candidate only if its physical chronology, exact mass/energy/water
custody, support-refinement behavior, event semantics, and ownership/restart
posture are independently supportable. Otherwise close typed unsupported with
a defect-shaped HOLD.

## Intake and authority

- Exact starting HEAD: `56b1bfc7ea62d491dcb54745d4911276e3dcf089`.
- Prior phase-competition HOLD: same commit.
- Physical discrete-root evidence: `97accd99a62d9e4418d2eb7533c4474fe405427d`.
- Last fully qualified physical implementation:
  `43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.
- Released authority: `SC-SNOWENERGY-001@18`, especially
  `INV-SNOWENERGY-017/029/030/032/034`, and current `SC-SNOWFREEZE-001`.

This package performs science characterization, not calibration. Fixture data
are `DIAGNOSTIC_ONLY`; calibration and empirical validation are not applicable.

## Correction Authority Envelope

Included: package-local analytical tools; cfg(test) trajectory/phase allocators;
typed trajectory, candidate, event, receipt, ledger, restart, and poison DTOs;
private/test-only snow/orchestrator accessors; synthetic and current real-fixture
execution; ordinary compiler/test correction; and source decomposition required
by line-count governance.

Protected: all production Stage-3 equations, constants, tolerances, the 600-ms
carrier floor, public APIs/outputs, ordinary persistent-support behavior,
owner publication, restart/receiver/runner, selectors/defaults, Stage-3
activation, CoE retirement, Batch V2, Child 3/4, cutover, canonical contracts,
and the separate Assurance V2 workspace-debt package.

No production or contract edit is authorized by this package. If research
identifies a candidate requiring a new owner, public output, contract
successor, or production chronology, record the exact authorization boundary
and stop before that change.

## Candidate definitions

- `ReleasedOrderedTrajectory`: on each sealed trajectory segment, satisfy cold
  content/refreeze and melt using beginning solid less bounded sublimation,
  then apply deposition. Multiple disappearance/reappearance transitions are
  retained rather than collapsed into one endpoint label.
- `EventDrivenFrostHybrid`: localize exhaustion inside a segment under its
  declared constant rates; deposition after exhaustion enters a distinct frost
  state with explicit mass and enthalpy.
- `TimeResolvedComplementarity`: apply the simultaneous complementarity map to
  each sealed trajectory segment; test convergence under refinement of one
  fixed path, never invariance under arbitrary forcing reorder.
- `ExistingSnowFrostSubtype`: use the hybrid chronology but retain pack and
  frost as distinct tagged substates within one research snow-owner envelope;
  prove canonical restart and no alias/deletion. This is not owner authority.

## Matrix and acceptance

Evaluate zero vapor, sublimation, deposition below/at/above melt balance,
refreeze, deposition+refreeze, rain-on-snow, positive/zero cold content,
start/interior/end meltout, persistent post-exhaustion deposition, disappearance
and reappearance, sign-changing energy, forcing-order permutations, fixed-path
support refinement, exact vapor mass/latent reconstruction, mass/energy/water
closure, replay, rollback, substitution poisons, canonical restart, the real
fixture, and nearby forcing perturbations.

A candidate must distinguish physical-path dependence from numerical partition
dependence. Fixed-path refinement must converge; arbitrary forcing reorder need
not agree but must preserve and report distinct chronology. No accepted state
may contain positive material pack ice and positive unallocated melt energy.
Frost mass may coexist with energy only under an explicitly derived frost
enthalpy/state rule.

## Phase plan

1. Scaffold and record authority/readings/intent.
2. Implement pure cfg(test) trajectory models and result-blind matrix.
3. Adapt the real fixture without changing the complete-owner production path.
4. Freeze or reject each candidate with quantitative evidence.
5. Obtain independent snow thermodynamics/numerics and
   ownership/receiver/restart reviews plus Rust/QA verification.
6. Resolve in-envelope findings; run final gates; close PASS or HOLD.

## Validation intent

Run rustfmt, affected warnings-denied Clippy where baseline permits, test build,
focused candidate matrix, real fixture, snow/frost profile when applicable,
diff hygiene, source scans, line counts, exact terminal-diff reconciliation,
dual review, and dual verification. Full workspace/comparator execution is not
required for a rejected cfg(test)-only characterization unless review identifies
production impact.

## Reviews and delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to independent snow-thermodynamics/numerics, ownership/receiver/restart, Rust
correctness, QA verification, and comparator-suite-runner subagents for
read-only review or bounded package-artifact work. Expected outputs are compact
findings, exact commands/results, and explicit GO/NO-GO dispositions; source
review is read-only.

Every finding is accepted, rejected with evidence, corrected, or assigned to a
named authorization boundary. Dual review and dual verification must assess
gate legitimacy, anti-tautology, protected boundaries, and line counts.

## HOLD legitimacy

HOLD is legitimate only if every evaluated chronology fails fixed-path
convergence/closure/determinism; authoritative forcing chronology is absent;
exact vapor energy cannot be assigned once; a frost state is required but owner
authorization is absent; or any viable result requires protected production or
contract changes. Effort and ordinary implementation defects are not blockers.

## Exit criteria

- Four candidates receive direct matrix and real-fixture disposition.
- Physical-path dependence is separated from numerical partition dependence.
- Exact operand lineage and anti-tautological closure limits are recorded.
- Restart/owner implications are explicit for both frost candidates.
- Production and canonical contracts remain unchanged.
- Required reviews, verifications, gates, exact diff, and line counts are
  complete.
- Final disposition is truthful PASS or defect-shaped HOLD.

Security impact: none. No dependencies, credentials, external services, or
authority-suite weakening are authorized.
