# Prospective re-review A

Static + Ran: 2026-09-08 PDT. I reviewed the corrected authority cut in the
primary checkout at `827a7470e058a5e09f9feced9375b776e5959789`, the detached
A-source-05 observation tree at `e89befa4678eadec039b3e7f7fe0a176af8e9dc5`,
the original PR-A findings and their disposition, and the frozen corpus. No
derivative implementation exists. Read-only checks verified both declared
SHA-256 values, decoded the stem-area words in all eight corpus records,
inspected the capture/evaluator seam, and ran the focused v33 authority-binding
test under offline Nix (1 passed, 26 skipped). I did not run a corpus replay,
oracle, derivative, hybrid-solve, or runner comparison because none exists.

## Findings

### HIGH — PR-A-R-001: the frozen authentic corpus contains no admissible stem derivative column

`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authentic-corpus.md:25`
calls the eight records authentic smooth workload states, and
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/preimplementation-contract-gate.md:17`
marks corpus capture PASS. However, decoding
`/tmp/openwepp-rj-corpus-v1-combined.json:1` shows that
`inputs.occupancies[*].stem_area_m2_m2_tile` and
`base_evaluation.occupancies[*].component_areas_m2_m2_tile[3]` are exact
`0000000000000000` for both occupancies in every one of the four Potential and
four FixedFinal records. The v33 target admits only finite positive executed
`dry_stem` (`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:105`
and `:166`), while a zero-area stem remains canonical identity/FD handling.
Consequently the corpus contains zero supported columns and cannot establish
the derivative, cross-occupancy affected rows, local cost, or the required
`>=75%` broad-timing coverage. It also does not meet the protocol's positive-area
coverage requirement at
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authority-and-protocol.md:56`.

Required before implementation: capture and freeze authentic early/late
Potential and FixedFinal bases with at least one finite positive executed stem
per intended case and the required longwave/ordinary-ground edges. If the real
runner cannot produce such bases, record that as an applicability blocker; do
not manufacture a synthetic state and call it authentic.

### HIGH — PR-A-R-002: FixedFinal snapshots do not contain the complete input needed to replay the actual residual

The corpus claim says every record contains complete `u` and a reconstructible
base
(`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authentic-corpus.md:11`;
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authority-and-protocol.md:61`).
The actual `Snapshot` contains only `CoveredColumnInputs`, caps, `x`, frozen
branches, and a predecessor `CoveredColumnEvaluation`
(`/tmp/openwepp-residual-jacobian-FC2T1v/J/crates/openwepp-land-surface-energy/src/solver_residual_corpus_capture.rs:23`).
The V3 evaluator additionally requires `V3LitterResidualContext`, which carries
the litter configuration, complete beginning state including ice, and optional
finalized vapor
(`/tmp/openwepp-residual-jacobian-FC2T1v/J/crates/openwepp-land-surface-energy/src/solver_litter_phase.rs:21` and
`:71`). Those operands affect validation, vapor, heat capacity, storage, and the
residual
(`/tmp/openwepp-residual-jacobian-FC2T1v/J/crates/openwepp-land-surface-energy/src/solver_covered_evaluation.rs:2485`).
The FixedFinal hook records only `detail.predecessor`, not that context or the
complete `V3PhaseFreeCoveredEvaluation`
(`/tmp/openwepp-residual-jacobian-FC2T1v/J/crates/openwepp-land-surface-energy/src/solver_litter_phase.rs:243`).
A stored base
evaluation cannot substitute for the missing inputs when finite differences
must reevaluate at perturbed `x`.

Required before implementation: version the corpus schema with an evaluator
kind and every evaluator-specific input needed to reconstruct the exact primal
call, including the V3 context/finalization state; capture again; and prove exact
base residual/error replay before running the oracle.

### HIGH — PR-A-R-003 / PR-A-002: the deterministic oracle is specified but has not frozen baseline support before J

The amended algorithm is now deterministic on paper, including step scales,
basin predicate/ties, Richardson estimate, uncertainty, direction sequence, and
self-tests
(`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:204`).
It also explicitly requires “No basin in baseline A” to freeze an entry
unsupported before J exists (`:218`). Yet the gate records only PASS-STATIC for
the oracle
(`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/preimplementation-contract-gate.md:11`),
the build manifest lists no executable
(`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/source-and-build-manifest.json:66`),
and there is no per-entry or per-direction A-only support mask, `J_oracle`, `U`,
branch/sign/floor trace, or oracle self-test result. The zero-stem and incomplete
snapshot findings make the current corpus incapable of supplying that result.

Required before implementation: implement the independent primal-only oracle
and its four required self-tests as Phase-A evidence, replay the repaired
corpus, and freeze the exact A-only support decisions and numerical references.
The oracle/support freeze must precede—not land alongside—the analytic
derivative whose comparison it governs.

### HIGH — PR-A-R-004 / PR-A-003: the detached observation cut and claimed exact write set are not reproducibly frozen

The source manifest now gives the correct package scaffold and A-source-05
identities
(`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/source-and-build-manifest.json:4`
and `:6`), but it does
not bind the current observation changes layered onto the detached tree by a
patch/source hash. The tree is dirty; the corpus capture module is untracked and
is absent from the purported exact write set, even though `lib.rs` wires it and
the runner uses it. The entries at
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/source-and-build-manifest.json:32`
are file-level descriptions rather than exact function/symbol ownership, and
`executables` is empty at `:66`. The external `/tmp` corpus still has pending
custody at `:67`. A later candidate therefore cannot prove its A/J source
difference or reproduce the evidence-producing observation binary from this
manifest.

Required before implementation: freeze the full detached observation patch and
new-source archive (including `solver_residual_corpus_capture.rs`) with hashes;
list exact owned symbols/functions and forbidden edits; bind the exact built
binary and capture/replay commands; and preserve the corpus plus source inputs
in the package custody bundle before replacement or measurement.

### MEDIUM — PR-A-R-005 / PR-A-004: the shared-heat normalizer floor rule is internally inconsistent

The shared-heat row defines
`z=max(abs(H_canopy)+abs(H_ground)+abs(H_reference),1)`
(`numerical-methods.md:145`), but the next rule says `ds=0` for `z<1`, `ds=b*dz`
for `z>1`, and exact `z=1` is unsupported (`:148`). With the table's definition,
`z` can never be below one, so every smooth floor-active state with a pre-floor
sum below one is indistinguishable from the nonsmooth equality at one and is
incorrectly unsupported. This leaves normalization/freezing semantics
ambiguous despite the disposition claiming closure.

Required before implementation: name the pre-floor operand, for example
`p=abs(H_canopy)+abs(H_ground)+abs(H_reference)`, define
`s=a+b*max(p,1)`, and apply `ds=0`, unsupported tie, or `b*dp` according to
`p<1`, `p=1`, or `p>1`. Use the same pre-floor predicate in applicability and
oracle sample admission.

### MEDIUM — PR-A-R-006 / PR-A-004: nonfinite dry-stem disposition contradicts source-order validation

`numerical-methods.md:121` says a nonfinite executed `dry_stem` retains
identity/FD handling and is not derivative-treated, while `:168` says invalid or
nonfinite current trials reject before applicability and are never fallback
eligible. The ordered guard table preserves source-real validation (`:171`), so
a nonfinite computed area/input must not be silently converted into an identity
or FD case.

Required before implementation: restrict the identity/FD statement to valid
finite nonpositive/zero-area structural cases, and state that nonfinite
`dry_stem` follows the existing validation/domain error in source order.

### MEDIUM — PR-A-R-007 / PR-A-007: expected-red evidence covers only a name, not assembly or no-recovery behavior

The protocol requires expected-red tests to demonstrate missing derivative
capability, assembly, and counters, including no-recovery error propagation
(`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authority-and-protocol.md:64`).
The only captured compile failure is an unresolved capability name
(`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/expected-red.md:6`).
The current test
was then replaced by an unconditional panic
(`/tmp/openwepp-residual-jacobian-FC2T1v/J/crates/openwepp-land-surface-energy/src/solver_stem_jacobian_tests.rs:3`),
which neither invokes a seam nor fixes assembly/counter/no-recovery obligations;
making it green necessarily entails rewriting its behavior. Thus corrected
sequencing closes the ordering contradiction, but not the expected-red contract
surface.

Required before implementation: retain separate executable expected-red
obligations for capability construction, actual hybrid-column participation,
named counters, and post-entry typed-error propagation with no FD retry/partial
LU. Candidate work should turn those same assertions green rather than replace
or weaken them.

### MEDIUM — PR-A-R-008: FixedFinal capture fabricates lifecycle scopes instead of joining the real nonlinear solve

The artifact claims every record has a lifecycle join
(`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authentic-corpus.md:11`).
In the V3 adjustment callback, however, the
observation hook creates a new `Solve` scope, hard-codes iteration zero, and
creates a new `Sweep` scope solely for capture
(`/tmp/openwepp-residual-jacobian-FC2T1v/J/crates/openwepp-land-surface-energy/src/solver_litter_phase.rs:243`).
Those IDs prove only that the capture-local scopes are nonzero; they do not join
the surrounding generic nonlinear solve/Jacobian lifecycle. This cannot support
the required within-run producer/consumer and exact-one-use claims.

Required before implementation: pass through and record the actual solve,
iteration, sweep, and Jacobian-base identity from the nonlinear controller, or
label these IDs capture-local and do not use them as authentic lifecycle
evidence.

### MEDIUM — PR-A-R-009: broad-timing coverage has no executable statistic

The declared exclusive spans are `base_primal`, `jacobian_identity`,
`jacobian_v33_stem`, `jacobian_fd_other`, `linear_solve`, `line_search`, and
`remaining_solver`
(`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:292`).
The immediately following modifiable-fraction formula references an undeclared
`A.jacobian_fd_stem` span and divides it by `A.complete_runner` (`:295`). That is
neither computable from the declared spans nor the stated fraction of baseline
Jacobian-construction time. The broad timing gate then requires `>=75%` of A
Jacobian-construction wall time (`:297`; also
`docs/work-packages/20260908-stage3-residual-jacobian-prototype-001/artifacts/authority-and-protocol.md:87`)
without defining the exact numerator, denominator, classification of identity
work, or aggregation over multiplicities. Broad-timing eligibility is therefore
not deterministic.

Required before implementation: declare the baseline stem-FD span in the
mutually exclusive taxonomy and define a separate exact coverage statistic over
baseline Jacobian-construction spans, including corpus multiplicities and
zero-denominator handling. Keep any whole-run Amdahl fraction distinct from the
`>=75%` Jacobian-coverage gate.

## Original PR-A finding closure

| Original finding | Re-review state | Basis |
| --- | --- | --- |
| PR-A-001 affected-row map | CLOSED-STATIC | The amended `i<o` upward / `i>o` downward recurrence and same-layer exact zeros are now complete. Executable evidence remains future, and the present corpus exercises none of it. |
| PR-A-002 deterministic oracle | PARTIAL / OPEN | The algorithm is deterministic, but the mandatory A-only support/reference freeze has not run. |
| PR-A-003 identity/write set | PARTIAL / OPEN | Scaffold and A-source-05 identities are corrected; the observation cut, capture module, exact symbols, executable, and custody are not frozen. |
| PR-A-004 smooth/one-sided/nonsmooth semantics | PARTIAL / OPEN | Active-zero and boundary rules improved; the shared-heat floor variable and nonfinite disposition remain contradictory. |
| PR-A-005 typed error/no recovery | CLOSED-STATIC | The ordered guard table, dedicated integrity variant, discard-partial rule, and no-retry/no-LU semantics are explicit. |
| PR-A-006 result tolerances | CLOSED-STATIC | Symmetric units/bases, exact treatment of unlisted fields, one-tenth cap, and A0 closure are explicit. |
| PR-A-007 staged sequence | PARTIAL / OPEN | Expected-red now precedes candidate code and re-review, but it does not exercise the required assembly/counter/no-recovery seams. |

## Residual risk and missing tests

- No admissible authentic derivative state, exact corpus replay, independent
  oracle support mask, or numerical reference has been demonstrated.
- No derivative affected-row/zero-mask, normalization tangent, elementary
  sign/unit, Taylor contraction, hybrid no-recovery, single-use capability,
  full solve, consumer participation, closure, custody, or cost test exists yet.
- The focused authority text-binding test passed, but it validates document
  binding rather than numerical feasibility or corpus completeness.
- The optional leaf expansion remains correctly unapproved and outside the
  initial write set.

## Verdict

**HOLD.** The corrected authority is materially stronger and closes three of
the seven original findings statically, but derivative implementation is not
authorized. Repair and freeze an authentic positive-stem, fully reconstructible
corpus; execute and freeze the baseline-only oracle/support decisions; freeze
the exact observation source/executable/write set; and reconcile the remaining
normalization, nonfinite, expected-red, lifecycle, and cost-coverage issues
before requesting another independent prospective review.
