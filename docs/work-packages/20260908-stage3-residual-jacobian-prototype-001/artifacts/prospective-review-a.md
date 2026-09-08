# Prospective correctness review A

Static: reviewed the prospective v33 authority/protocol, canonical covered
residual and reciprocal-longwave source, package acceptance/write-set records,
and applicable science/work-package review guidance on 2026-09-08 PDT. No
experimental implementation or candidate evidence exists. Ran: read-only
`git status`/`git diff`/`git rev-parse`, source searches, and numbered source
inspection; no test, build, oracle, or runner command was executed.

Role/session: independent prospective correctness reviewer A; requested effort
high; effective runtime setting UNOBSERVED. Reviewed cut: working-tree authority
diff above actual scaffold commit
`827a7470e058a5e09f9feced9375b776e5959789`; the protocol records a different,
nonexistent object. Assigned output is this artifact only.

## Findings

### PR-A-001 — HIGH — The declared dry-stem affected-row map is mathematically false

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:101-118`;
mirrored in `artifacts/authority-and-protocol.md:14-20` and asserted by
`tests/integration/land_surface_energy_balance_authority_contract.rs:19-21`.

Evidence: the canonical longwave evaluator forms each layer emission from that
layer's component temperatures, then propagates it downward only into later
boundary values and upward only into earlier boundary values
(`crates/openwepp-land-surface-energy/src/physics.rs:370-396`). A component net
at layer `i` uses `downward[i] + upward[i+1]` plus only that component's own
emission term (`physics.rs:397-409`). Therefore a stem temperature in layer `o`
affects (a) its own stem row through its direct fourth-power emission and
sensible heat, (b) positive-area component rows in every *other* occupancy
through one propagated incident boundary, (c) shared heat through stem sensible
heat, and (d) the ordinary ground row through propagated downward longwave. It
does **not** affect the sun, shade, or wet component rows in the same occupancy:
neither incident boundary for layer `o` contains layer `o` emission, and those
components' direct emission terms do not use `T_stem,o`. The canonical caller
constructs exactly this layer-local input and one whole-column recurrence at
`solver_covered_evaluation.rs:2021-2052`.

The proposed statement that every positive-area component row in every
occupancy is affected conflicts with its own claim that every other row is an
independently proved exact zero. It would bind an incorrect sparsity mask,
oracle enumeration, omission tests, and cost denominator. The earlier v31 graph
was deliberately conservative and is not proof that every graph-reachable
component value has a nonzero derivative.

Required disposition: amend the canonical row map and protocol with the actual
recurrence-derived cases (`i < o`, `i = o`, `i > o`), including exact-zero
same-layer sibling rows and zero-area weights; update the textual binding test;
and add an oracle case that independently distinguishes the same-layer zeros
from both upward and downward cross-occupancy edges. Re-review is required.

### PR-A-002 — HIGH — The derivative oracle is not deterministic or reproducible

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:168-204`;
`artifacts/authority-and-protocol.md:42-52`.

Evidence: nine step sizes are fixed, but `J_oracle` is never defined from their
estimates. “Exhibit a resolvable truncation/roundoff basin” has no numerical
selection predicate, the uncertainty formula does not say which admitted `h`
is used in term (b) or how rejected scales split consecutive basins, and the
minimum adjacent-estimate difference can be selected outside the alleged
basin. The directional check likewise omits the norm/component reduction for
the vector remainder, initial `t`, halving sequence, direction normalization,
and conversion of the `K^-1` entry floor into a residual remainder floor.
Consequently two conforming oracles can select different references,
uncertainties, support classifications, and pass/fail results. This leaves room
for candidate-aware basin selection despite the prospective-freeze rule.

Required disposition: specify, before any J observation, the exact centered or
inward estimate at each scale, deterministic branch-consistency rejection,
consecutive-basin predicate and tie-breaking, selected `J_oracle`, the `h` used
in every uncertainty term, and treatment of one-sided truncation order. Define
direction vectors/units, `t0`, vector or per-row norm, halving limit, and the
dimensionally correct floor for Taylor remainders. Freeze these in executable
oracle fixtures derived only from A. Re-review is required.

### PR-A-003 — HIGH — There is no reviewable stable cut or concrete initial implementation write set

Location: `artifacts/authority-and-protocol.md:3-5,104-112`;
`package.md:50-65`; `artifacts/worker-handoff.md:26-35`.

Evidence: the protocol claims base
`827a7470e6357c431b00757674cc7990d9feeb1c`, but the actual scaffold commit is
`827a7470e058a5e09f9feced9375b776e5959789`; the recorded object does not
resolve. The protocol names only “A-source-05” pending reconciliation and no
source/build manifest exists in the package. No detached path, base/patch
identity, exact Rust/test/collector paths, or owned functions are frozen. This
is especially material because a nonduplicated analytic longwave tangent likely
requires an explicitly reviewed change to the shared `physics.rs` recurrence,
not only the two solver anchor files. The handoff still says the authority,
source, corpus, and oracle map are future work even though the protocol declares
them frozen.

The kickoff requires prospective reviewers to approve the concrete initial
write set. A directory-level promise to enumerate it later cannot satisfy that
gate, and the bad base object prevents exact diff reconstruction.

Required disposition: correct the immutable review-base identity; reconcile
and name the exact A source-kit identity and compile-time authority inputs; add
the source/build manifest and exact initial functions/files for shared primal
factoring, tangents, assembly, tests, corpus capture, counters, and collectors;
and update the handoff/reading state to the actual review cut. Any later
expansion must be separately enumerated before its edits. Re-review the stable
cut.

### PR-A-004 — MEDIUM — Smooth/nonsmooth admission and one-sided scope are ambiguous and can change coverage

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:120-166`;
`artifacts/authority-and-protocol.md:22-27`.

Evidence: the rule requires “each participating operand” to be nonzero without
defining whether that means every canonical tolerance summand or only an
operand whose value varies with the selected stem coordinate. The canonical
stem tolerance contains a structural zero latent operand
(`solver_covered_evaluation.rs:1460-1476`), and represented-snow shared-heat
scales can contain a structural zero ground-sensible operand
(`solver_covered_evaluation.rs:2186-2219`). Absolute value of a structurally
constant zero is constant and differentiable with respect to `T_stem`; a
literal all-summand interpretation would nevertheless make the intended block
universally or broadly unsupported. Conversely, derivative-active longwave or
aggregate-sensible operands at exact zero are genuine kinks. The “certified
local interval” has no radius/construction, so near-kink support can vary by
implementation. The method also excludes closed endpoints from v33 admission
but later describes inward estimates “for each supported column,” leaving it
unclear whether inward validation applies to the analytic block or only to the
canonical FD arm selected before entry.

Required disposition: define derivative-active operands, the exact
`energy_tolerance` scale/max derivative (including the constant-1 branch), the
interval radius and exact/tie comparisons, and pre-entry ordering. State
unambiguously that an exact closed stem endpoint is canonical-FD-only, if that
is intended, and preserve invalid-current/nonfinite error precedence rather
than treating invalid input as ordinary fallback eligibility.

### PR-A-005 — MEDIUM — New failure surfaces are not bound to a typed taxonomy or precedence

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:135-166,273-285`;
`artifacts/kernel-profile-compliance-checklist.md:10-13`.

Evidence: v33 says only “typed integrity failure” and “typed failure” after
entry. It does not bind stale/foreign/mutated/second-use, derivative nonfinite,
normalization arithmetic, or internal consistency failures to exact existing or
new error variants/codes, nor place them in the canonical first-error order.
The invariant's failure cell repeats the generic label. This is insufficient to
verify that structural FD selection is not a recovery cascade, that a
post-entry failure cannot be swallowed, or that matched invalid-input precedence
remains unchanged. The profile checklist therefore overstates guard/error
mapping as PASS.

Required disposition: add a v33 branch/guard table mapping every pre-entry
inapplicability and post-entry failure family to an exact typed error and source
order, including whether existing source-real arithmetic errors propagate
unchanged. Bind no-FD-retry and no-partial-assembly observables for every
post-entry family, then update C-021/tests and the profile checklist.

### PR-A-006 — MEDIUM — Accepted-result tolerances are not yet an executable, justified comparison table

Location: `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md:206-217`;
`package.md:93-99`; `artifacts/authority-and-protocol.md:68-77`.

Evidence: `2e-8 K + 2e-10*|T|` does not identify which arm supplies `T` and is
not written symmetrically, while “each published field” does not enumerate the
temperature, flux, water, energy, or closure fields and units to which the
generic `2e-7 + 2e-9*max(...)` floor applies. No field-specific existing closure
threshold/publication resolution or rule for taking the tighter bound is
provided, and no baseline-only scale/conditioning evidence is cited for these
constants. Receipt language depends on identifying exactly which continuous
payload field legitimately changed, so this ambiguity also weakens the digest
exception boundary.

Required disposition: freeze a field-by-field symmetric comparison table with
units, formula operands, existing closure/publication bound, effective tighter
predicate, baseline-only rationale, and the exact payload-derived receipt fields
allowed to differ. Fields without an applicable resolution must not inherit an
unjustified universal floor.

### PR-A-007 — MEDIUM — The package's preimplementation sequence contradicts its frozen acceptance

Location: `package.md:68-75,88-91`;
`artifacts/preimplementation-contract-gate.md:13-20`.

Evidence: package authority requires derived expected-red tests and captured
expected-red evidence before dual prospective review/disposition, but the gate
record says those tests follow accepted prospective review. The current review
is therefore occurring while a package-declared predecessor gate is NOT RUN.
This is a self-inconsistent staged acceptance boundary.

Required disposition: choose and record one order consistent with the owner
kickoff. Either capture the expected-red evidence before the approval cut, or
prospectively amend package sequencing to place the reviews before expected-red
capture while still requiring expected-red evidence before candidate code. Do
not mark the Phase-A/preimplementation gate complete until the selected order
and every current-scope item have direct evidence.

## Residual risk and missing tests

- No implementation, authentic corpus, expected-red result, derivative oracle,
  full solve, consumer, closure/custody, cost, timing, memory, or multi-OFE
  evidence exists; none was assessed as passed by this review.
- The contract-derived integration test is text-presence enforcement only and
  is recorded NOT RUN. After the mathematical corrections it must be updated
  and executed with directory/schema/binding checks; it cannot establish
  derivative correctness or connectivity.
- The initial stem block does not cross an inner solve. The leaf-temperature
  expansion remains correctly conditional on a new explicit implicit-derivative
  derivation, finite inner-residual error budget, complete maximum/current-leaf
  dependency map, and separate prospective review. This review does not approve
  that expansion.
- Even after the authority is corrected, exact same-state primal arithmetic is
  at risk if the longwave recurrence is duplicated or reordered. The declared
  A-to-Ax bit-identical mechanical control and exact first-error evidence remain
  mandatory before derivative results can be interpreted.
- Production remains HOLD, and the v33 authority must not be treated as a
  production solver exception or a basis for weakening canonical FD behavior
  outside the detached experiment.

## Verdict

**HOLD.** The prospective authority is not approved for implementation. Resolve
PR-A-001 through PR-A-007, capture a corrected stable cut and exact write set,
then obtain focused independent re-review before any candidate implementation.
