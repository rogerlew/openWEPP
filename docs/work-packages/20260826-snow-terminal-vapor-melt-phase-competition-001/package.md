# Snow terminal vapor/melt phase competition

Status: `EXECUTING / RESEARCH / PRODUCTION UNCHANGED`

Date: `2026-08-26`

Package ID: `20260826-snow-terminal-vapor-melt-phase-competition-001`

Plan class: defect-closure science implementation and qualification.

## Objective

Close `CHILD1-TERMINAL-PHASE-COMPETITION-001`: select and qualify the physical
disposition of simultaneous pack melt, deposition, sublimation, refreeze,
liquid, cold-content change, and excess energy at terminal supports. Compare
result-blind Candidate A (simultaneous phase complementarity), Candidate B
(owned residual surface frost), and Candidate C (typed unsupported).

The intake observation is the real complete-owner endpoint at
`615737728343 ns`: beginning-pack melt and ending liquid are each
`0.6 kg m^-2`; deposition and ending ice are each
`0.002815601898351902 kg m^-2`; no event occurs; terminal-unallocated energy is
`1.014879671856761e-6 J m^-2`. The preceding nanosecond is `PreTerminal`, two
brackets select the same boundary, and replay is byte-identical.

## Starting identity and dependencies

- Exact intake: `1eb009cabdb1e7b96e2404af90b954aa4f3b1acf`.
- Physical discrete-root evidence: `97accd99a62d9e4418d2eb7533c4474fe405427d`.
- Last fully qualified physical implementation:
  `43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.
- Closed holds: `CHILD1-REAL-DAE-001` and
  `CHILD1-DISCRETE-SUPPORT-ROOT-001`.
- Current authority: `SC-SNOWENERGY-001@18` and the current released
  `SC-SNOWFREEZE-001`.
- Separate workspace debt package
  `20260826-workspace-baseline-assurance-v2-reconciliation-001` is immutable to
  this work and must pass before contract promotion or production cutover.

## Correction Authority Envelope

Included: package-local analytical/numerical tools; cfg(test) alternative phase
allocators; private/test-only snow/orchestrator accessors; typed candidate
states, ledgers, receipts, and poisons; exact real-fixture and perturbation
execution; source-governance split and private helper centralization; affected
canonical contract successors only after a candidate passes and dual GO;
default-off production implementation only after that same dual GO.

Protected boundaries: production Stage-3 equations, tolerances, 600-ms floor,
terminal behavior, public APIs/outputs, owner publication, restart, receiver,
runner, selectors/defaults, activation, CoE retirement, Child 3/4, and cutover.
Production remains `BelowCarrierDomain` during research. A new surface-frost
owner or output requires explicit owner authorization before implementation.

No continuous DAE, LTE, Hermite, discrete-root, or search-method continuation
is in scope. No external dependency may enter production solely for this work.

## Intended write set

- this package tree and `docs/work-packages/README.md`;
- private/cfg(test) snow terminal evidence and tests in
  `crates/openwepp-hillslope-orchestrator/src/`;
- package-local tools;
- only after dual GO: the actually affected SnowEnergy/SnowFreeze contracts,
  contract-derived tests, and default-off terminal-phase implementation.

Before reusing endpoint evidence, split cfg(test) endpoint evidence out of
`snow_stage3_v11_terminal_execution.rs` and centralize endpoint result/closure
assembly plus duration-to-WB14-ceiling mapping. The split is behavior
preserving and must keep production bytes/behavior unchanged.

## Candidate obligations

Candidate A derives signs, bounds, vapor latent-energy custody, refreeze rules,
and whether deposited/refrozen solid can melt on the same support before
freezing a complementarity condition. It must preserve exact solid, liquid,
water, and energy identities and prohibit order-dependent double counting.

Candidate B must define a distinct frost owner's mass, enthalpy, chronology,
surface exchange, subsequent melt/deposition, disappearance/reappearance,
terminal-liquid custody, and restart posture. Frost cannot alias liquid or be
deleted by tolerance.

Candidate C retains typed unsupported when neither physical model is supported
and records that atomic Stage-3 cutover remains blocked.

## Result-blind matrix

Execute zero vapor; sublimation; deposition below/at/above melt-energy balance;
refreeze; deposition plus refreeze; rain-on-snow; positive/zero cold content;
start/interior/end events; persistent deposition after exhaustion; support
partition sensitivity; exact vapor latent-energy custody; mass/energy/water
closure; the current real fixture; and nearby forcing perturbations. Record
every attempted model and failure. No accepted state may contain material
positive ice and material positive unallocated energy unless a contract
explicitly defines it.

## Conversion rule and sequence

If a reproducible in-envelope model has canonical/literature/physical authority,
is safe, contract-testable, measurable, closes independently, and passes the
matrix, freeze it; obtain snow thermodynamics/numerics and
ownership/receiver/chronology reviews; resolve findings; then author only the
affected contract successors and implement default-off under this package.
SnowEnergy and SnowFreeze are expected; LSE/CoupledTime versioning requires an
actual interface or custody change. Re-run complete real-fixture terminal
chronology after implementation.

Contract-first ordering after dual GO is mandatory: contract successors,
contract-derived tests, pre-implementation gate, then production code.
No surrogate/provisional/proxy/heuristic production physics is permitted.

## Validation intent

Research changes are test-only/private and conservation-sensitive. Run focused
matrix tests, real-fixture replay/closure, formatting, check, affected Clippy,
and affected crate regression. If production or canonical contracts change,
escalate to Critical full correctness and exact-head workspace qualification.
Record pre-implementation intent and reconcile the exact terminal diff. Heavy
batch/comparator runs must be delegated to `comparator_suite_runner` when
available.

## Reviews and delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to independent snow-thermodynamics/numerics, ownership/receiver/chronology,
Rust correctness/QA, verification, and `comparator_suite_runner` subagents.
Expected outputs are compact findings, exact commands/results, and artifact
text or bounded edits inside this package tree; source review is read-only.

Dual reviews and dual verification must assess science, exact-once custody,
anti-tautological closure, gate legitimacy, protected boundaries, exact diff,
and line-count governance. Every finding is accepted, rejected, deferred with
a legitimate future boundary, or fixed before disposition.

## HOLD legitimacy

A defect-shaped HOLD is legitimate only if every scientifically defensible
allocation fails; closure needs unrelated physics; vapor latent energy cannot
be assigned exactly once; residual frost is required but owner authorization is
unavailable; the selected result is materially partition-dependent; or
production changes outside the terminal phase boundary are required. The hold
audit must cite evidence, name considered correction routes, and explain why
none closes in-envelope.

## Exit criteria

- source split and helper centralization complete before endpoint reuse;
- every candidate and matrix row has direct evidence;
- selected result has independent mass/energy/water reconstruction and
  anti-alias poisons;
- production remains unchanged unless matrix plus dual GO authorizes the
  contract-first default-off increment;
- reviews, verifications, finding disposition, validation, exact-diff, and
  line-count artifacts are complete;
- final disposition is truthful `PASS` or defect-shaped `HOLD`.

Security impact: none intended. No secrets, external services, dependency
admission, or authority-suite weakening is allowed.
