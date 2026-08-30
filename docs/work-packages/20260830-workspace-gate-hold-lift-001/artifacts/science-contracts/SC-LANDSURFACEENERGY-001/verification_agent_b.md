# Independent verification B — WGHL-FULL-001F

Status: `HOLD — 001F IMPLEMENTATION PASS; EXTERNAL CLOSURE EVIDENCE OPEN`

Recommendation: `HOLD`

Evidence mode: `Static + Ran`

## Scope and identity

Static: independently verified the preserved
`SC-LANDSURFACEENERGY-001@13#INV-LANDSURFACEENERGY-139` amendment and its
covered-solver implementation with an adversarial numerical/transaction lens.
The exact preserved v13 contract-first snapshot is SHA-256
`922917e963788ae10faae699ab8c6eb95180748d53a94b15aa484a34eeadfede`.
The current canonical successor is version 14 at SHA-256
`857b49f06fdb675cd91fe2776727388aea72d19fdb999e2e4cd6e248f0e836d1`;
it retains `INV-LANDSURFACEENERGY-139` and clarifies that the examined halving
exponent is added to the existing cumulative backtracking diagnostic. The
separately owned frozen-litter v14 science is outside this verification.

Static: verified
`crates/openwepp-land-surface-energy/src/solver_covered_solve.rs` at SHA-256
`f9f047632a936a9def0804c7a82890397988bb134a7e277431f6a9d7913d0331`
and 1,172 lines (`PASS`, below the 2,000-line warning threshold). Applicable
root, crate, work-package, and science-contract governance; package authority;
both independent reviews; the v13 contract reference; current v14
clarification; solver diff; controller tests; frozen oracle; natural failure;
and transaction rollback vectors were inspected.

## Direct execution

Ran:

```text
nix develop -c cargo nextest run -p openwepp-land-surface-energy \
  -E 'test(covered_halved_no_update_witness_tests) | \
      test(covered_multirank_potential_fixed_cap_and_alternate_start_match_frozen_oracle) | \
      test(covered_natural_failures_match_frozen_diagnostics_and_publish_no_candidate) | \
      test(numerical_failure_errors_preserve_kind_diagnostics_and_rollback_lineage)'
```

Result: `8/8 PASS`, run `23f86a44-ab40-4a68-8f8c-a70326762a74`.

Ran: `nix develop -c cargo nextest run -p openwepp-land-surface-energy
--no-fail-fast`: `97/97 PASS`, run
`31de0cbe-4b3a-4174-91e4-d133763c3d0c`. An immediately preceding run
`72fb1ec1-9478-4cd7-a111-60f80b58fe2d` observed one concurrently edited v14
frozen-litter assertion before that owner completed its change; the exact
current rerun closes that transient result.

Ran: `nix develop -c rustfmt --edition 2021 --check
crates/openwepp-land-surface-energy/src/solver_covered_solve.rs`: `PASS`.

Ran: `git diff --check` over the 001F solver, canonical contract, and
contract-cycle artifact paths: `PASS`. Static scans found no added public item
and no `eprintln!`, `println!`, `dbg!`, or `WGHL-F` production diagnostic in
the covered solver.

Ran: warnings-denied all-target/all-feature LSE Clippy reached the crate and
returned exit `101` only on separately owned v14 frozen-litter files
(`litter_phase*`, `transaction_v3`, `v3_state`, and the v14 error variants).
It emitted no diagnostic for `solver_covered_solve.rs`. This is not an 001F
implementation finding; terminal package Clippy remains a parent/v14-owner
gate.

## Verification findings

### `001F-VER-B-001` — PASS — complete trigger and refusal predicates

Static: the current residual predicate refuses an empty vector and checks
every member independently for finiteness and inclusive `abs <= 1` passage.
This avoids the pre-existing infinity-norm fold's possible nonfinite masking
for this witness. The full-trial trigger is typed and limited to
`DomainInvalid` or `GovernedStepThresholdExceeded`; a domain-valid full trial
whose four governed steps already pass retains priority and never invokes the
halved controller.

Static/Ran: exact-threshold positive vectors cover both typed trigger classes.
Refusal vectors cover out-of-tolerance, infinity and NaN residual members;
absence of a qualifying full refusal; later-not-first identity; each governed
hydraulic, beta, temperature and humidity excess; and a nonfinite governed
step. Binary comparisons make a nonfinite value in any governed coordinate
refuse. `ci_pa` remains diagnostic and intentionally ungoverned, as required
by the contract. The focused selection passes.

### `001F-VER-B-002` — PASS — enclosing order and no-skip behavior

Static/Ran: the production-used private controller iterates exactly
`b=1..=20`. It skips only `DomainInvalid` probes. It returns immediately with
no witness when the first domain-valid probe is evaluation-incomplete or its
complete governed steps fail; it cannot search a later, smaller witness.
Direct vectors prove invalid `b=1/b=2` followed by first complete `b=3`,
immediate refusal at the first incomplete/step-failing domain-valid `b=1`, and
no probe at all without a typed full-trial refusal. The ordinary actual-update
search remains subsequent and independently starts at `b=0`.

### `001F-VER-B-003` — PASS — current-state acceptance and transaction safety

Static: the controller's success type contains only `(exponent,
CoveredStepNorms)`; it cannot transport a prospective solution or evaluation.
The acceptance call passes the exact current `x` and current `detail` and only
records prospective norms plus `backtracking_count + exponent`. Thus no part
of a prospective trial becomes solution, evaluation, active branch, water
request/use, ledger operand, owner candidate, or publication state.

Static/Ran: every installed actual update still passes
`is_strict_residual_decrease`; the frozen covered oracle and alternate-start
vector, natural backtracking/iteration failure with no candidate publication,
and typed numerical-failure rollback-lineage vector all pass. The complete
crate also passes, including transaction/closure tests. No public or persisted
field was added for the witness diagnostic.

### `001F-VER-B-004` — PASS — protected numerical and temporal boundaries

Static: the terminal solver diff adds the private witness/controller and tests
only. It does not edit `numerics.rs`, support admission, or closure code.
`MAX_NEWTON_ITERATIONS=50`, `MAX_BACKTRACKING_HALVINGS=20`, and the exact
step thresholds (`1e-7` hydraulic, `1e-10` beta, `1e-8 K` temperature,
`1e-12` humidity) remain unchanged. The exact 60-second support floor remains
owned by `support.rs`; no phase bound, constitutive equation, pivot/stencil,
residual tolerance, event, custody, topology, receipt, ledger, rollback, or
fail-closed rule changed.

### `001F-VER-B-005` — HOLD — mandatory real-consumer terminal proof

Ran evidence review: retained consumer run
`ec067bbd-443d-45ce-ba76-5c4fdd2e252b` proves both unchanged interior-terminal
paths advance beyond the former iteration-4 `LSEB-E-034`. Both then stop at
the separately inventoried `WGHL-FULL-001H` guard `qualification terminal
snow-free successor chronology`. Therefore terminal completion, unchanged
owner/ledger closure, and no trial installation are not yet observed in the
real consumer. Per assignment, this verifier did not repeat the long consumers
before 001H correction. Owner: 001H chronology worker/parent. Trigger: land
001H, then rerun both unchanged named consumers. This is an external evidence
HOLD, not an 001F production defect.

### `001F-VER-B-006` — HOLD — canonical source binding and A0 admission

Static: the parent-owned authority test now expects contract version 14, but
still contains no exact `INV-LANDSURFACEENERGY-139`/first-domain-valid binding.
The impact map contains no exact-path entry for
`solver_covered_solve.rs` paired with that authority test. Successor-safe source
binding, exact impact-map admission, focused authority execution, A0, and
anti-evasion evidence therefore remain open. Owner: parent authority-policy
reconciliation. This verifier made no edits outside the assigned artifact and
did not claim those gates passed.

## Final disposition

`HOLD` for external closure evidence only. The 001F numerical/controller and
transaction implementation is independently `PASS`: refusal poisons,
first-domain-valid ordering, no-skip behavior, current-state-only acceptance,
strict-decrease updates, oracle/failure/rollback behavior, unchanged limits and
60-second floor, private API posture, and no-print/no-persistence posture all
survive direct inspection and execution. Closure still requires (1) passing
post-001H real consumers with owner/ledger/no-trial proof and (2) the
parent-owned `INV-139` source binding plus exact impact-map/A0/anti-evasion
evidence.
