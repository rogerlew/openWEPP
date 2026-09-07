Static: independent prospective R runtime test/API review A, before behavior
changes. No build or test execution by this reviewer. The actual isolated root
is `/tmp/openwepp-controlled-mechanisms-Hb6uS2/R`, not its `src/` subdirectory.

## Findings on initial cut

| ID | Severity | Path/line relative to isolated root | Finding | Disposition |
| --- | --- | --- | --- | --- |
| RA1 | MEDIUM | `crates/openwepp-land-surface-energy/src/solver_component_dependency_replay_tests.rs:17` | Derived f64 PartialEq is weaker than rev31 bit identity: a signed-zero change can pass. Compare per-probe residual Result values through to_bits while retaining exact typed errors. The full-solve PartialEq assertion at line 27 may support the test but cannot close the required full-field bitwise oracle. | Requested correction before behavior admission. |
| RA2 | MEDIUM | `docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/R-implementation-plan.md:5` and `:29` | The generic graph/oracle/custody description does not yet enumerate the concrete direct-edge/hash, authentic lifecycle joins, potential/fixed-final field/Jacobian/trajectory and crossability requirements retained by INV-164/C-020. Bind these explicitly as current-arm premeasurement gates and make clear that a shared lifetime or zero full-dispatch counter is not the proof. | Requested bounded prospective plan clarification; no passing runtime required before implementation. |

Initial cut identities: new test
`f43f6a9d27f1c04fee1378b32a3d17e0615eebd6b64b1416ae0538ac5825615c`;
`solver_tests.rs`
`f7c31caa00a21f0f80c7de1825ca9b253e8afdddc5cb918491317b33d0a20a83`;
R implementation plan
`b884f165699a45d56708f2fb29e7fe41f83020e7a8403ff886ac903b081fea67`.
The isolated source base is `2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3`.

## Design assessment and required evidence boundary

The seed helper is wired into the existing authentic represented-snow fixture
in `covered_v8_block_matches_frozen_joint_solution`. Temperature perturbation
units match the canonical constructor, with lawful signed probes evaluated
minus before plus. Baseline A dispatches those component probes completely,
so the zero-complete-call assertion is a legitimate static expected-red seed.
It has not run; it does not yet prove real dispatcher replay or shared capture.
Historical assertions remain present.

One canonical evaluator with optional successful-base capture is compatible
with the authorized mechanism. Reusing graph-unreachable liquid/leaf nodes
while recomputing the unchanged tail is conservative; report the actual work
saved. Additional tail execution must not be mislabeled as eliminated work.
Capture must become usable only after complete successful base evaluation;
failure must retain source order and return directly without a complete retry.
No duplicated equations, new derivative, fallback or solver redesign is
authorized by this prospective review.

Independent graph assurance must compare every expanded normative node and
direct edge, schema/hash and edge-removal sensitivity for N=1/S=1 and N=2/S=6.
Constructor-owned probe custody must bind exact base/input/caps/frozen/graph
and authentic map/solve/iteration/sweep identity, coordinate/sign/perturbation/
probe/stencil. Compile-time lifetimes and non-Clone by-value consumption are
useful mechanisms, but equal lifetimes alone do not distinguish two bases.
Real constructor/error/lifecycle vectors must prove the actual joins and
success/error single-use semantics.

Full capture/evaluation fields, raw/tolerance/normalized residuals, complete
Jacobian, pivots/norms and solve trajectory need bitwise forced-complete
comparison in potential and fixed-final modes. Source-real leaf current error
vectors, every normative noncrossable guard implication plus authentic boundary
successes, and naturally occurring differential errors remain required. The
parent's independent real-runner stencil/lifecycle oracle, nonzero completed
replay, output/closure parity and full critical-cut validation are additional
premeasurement evidence; this seed cannot replace them.

Initial decision: HOLD prospective admission only for RA1/RA2's bounded test/
plan corrections. Implementation need not pass its future runtime gates before
being authored; those gates must remain explicit and complete before measurement.

## Corrected prospective cut: GO for behavior authoring

Static: read the complete corrected 329-line runtime test, runtime admission
matrix and updated implementation/custody plan. No compilation or execution.
Independently checked SHA-256 identities:

- Runtime test: `5659be6e221e900072f78dbaffe6fe141042628ad9b079020fed20e5f1ef389e`.
- `R-runtime-test-plan.md`: `3f7b7e504780824ba6fe059d8e74283a679ce66cbcb71a271d5b566e4bbb3a8c`.
- `R-implementation-plan.md`: `cf3acebb2fbae4dc3171c55aa9f64231da098bf6fbdb0dc5e707b4add24ce92e`.

RA1 RESOLVED prospectively: successful residual Results use explicit float-bit
comparison and typed errors retain exact equality. Concrete helpers compare
evaluation/capture fields, complete Jacobian entries and an actual solve trace
whose API is required to retain matrix/RHS/pivot/norm/trajectory bits. The
all-coordinate independent perturbation-unit classification now matches
hydraulic 1000, shared humidity 0.001 and remaining coordinates 1. Full-solve
PartialEq is supporting evidence only, not the bitwise closure oracle.

RA2 RESOLVED prospectively: the binding matrix explicitly retains full graph,
real-consumer N2/S6 potential/fixed-final, custody/compile-negative, canonical
first-error/noncrossable guard, exact field/trajectory and full current-arm
gates. The intrinsic-scope plan distinguishes exact live bases by pointer
identity and consumed constructor-owned capabilities; observation IDs do not
authorize reuse. Graph/custody obligations stated above remain binding even
where separate workers own their concrete implementation.

Residual risk and missing execution: API wiring, complete graph oracle and
edge-deletion sensitivity, all private binding/second-use checks, authentic
N2/S6 corpus hookup, every named guard implication/boundary success, byte-exact
rollback, real-runner lifecycle/stencil/output/closure parity, and critical-cut
validation are still premeasurement requirements. The proposed correctness-only
callback must observe actual successful potential and fixed-final N2/S6 bases
and invoke the full corpus; graph topology alone cannot discharge that row.
Tests may need lawful fixture corrections during execution, without changing
canonical h, guard order, arithmetic or authority to manufacture a pass.

Decision: GO for isolated R behavior authoring on this prospective test/API
design. No remaining prospective blocker. This is not implementation approval,
runtime PASS, measurement admission or permission to defer required gates.
