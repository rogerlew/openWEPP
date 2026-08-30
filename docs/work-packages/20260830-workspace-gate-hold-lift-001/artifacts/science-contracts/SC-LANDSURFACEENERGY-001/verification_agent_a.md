# Independent verification A — WGHL-FULL-001F

Status: `HOLD — LOCAL VERIFICATION PASS; EXTERNAL CLOSURE PENDING`

Recommendation: `HOLD`

Evidence mode: `Static + Ran`

## Scope and exact identity

Static: independently verified only `WGHL-FULL-001F`, the preserved
`SC-LANDSURFACEENERGY-001@13#INV-LANDSURFACEENERGY-139` amendment, its
covered-solver implementation, and its focused evidence. The exact v13
contract-first snapshot remains SHA-256
`922917e963788ae10faae699ab8c6eb95180748d53a94b15aa484a34eeadfede`.
The canonical file has since advanced under the separately owned v14
frozen-litter successor. That successor is not dispositioned here; only its
clarification that the examined exponent is added to the existing cumulative
backtracking diagnostic was checked for 001F compatibility.

Verification source identity:

- Git HEAD: `6fa804082273c1c4340614ffc208a74a8b48e408` with a concurrent dirty
  package worktree;
- covered solver SHA-256:
  `f9f047632a936a9def0804c7a82890397988bb134a7e277431f6a9d7913d0331`;
- current cumulative v14 contract SHA-256:
  `857b49f06fdb675cd91fe2776727388aea72d19fdb999e2e4cd6e248f0e836d1`;
- current extracted Version-13 section SHA-256:
  `6af896968839fd6b56a2cc1e86bcba937ab7d06220ed2f31c249f6d19419ddf4`;
- `solver_covered_solve.rs`: 1,172 lines, `PASS` below the 2,000-line warning
  threshold.

## Static implementation verification

`PASS` for the 001F implementation itself.

- Complete residual gate: `covered_complete_residuals_pass` requires a
  nonempty vector and checks every member for finiteness and `abs <= 1`; a
  scalar infinity norm cannot hide a nonfinite member.
- Typed trigger classes: the only halved-witness triggers are
  `DomainInvalid` and `GovernedStepThresholdExceeded`. A domain-valid full
  trial whose existing no-update witness passes returns through that existing
  path before halved probing. A full trial with no authorized refusal cannot
  enter the controller.
- First-valid order: the private controller owns ordered `b=1..20` probing,
  skips only domain-invalid candidates, and returns immediately when the first
  domain-valid trial is evaluation-incomplete or has a failing governed step.
  It cannot skip to a later smaller witness. This controller completes before
  the ordinary `b=0..20` actual-update loop begins.
- No trial transport or installation: the controller returns only
  `(exponent, CoveredStepNorms)`, never a trial vector or evaluation. Witness
  acceptance passes the unchanged current `x` and `detail` to
  `accept_covered_candidate`; prospective values cannot become solution,
  branch, state, ledger, or owner-candidate values.
- Governed steps: hydraulic, beta, temperature, and humidity use the exact
  inclusive existing thresholds. A nonfinite governed step refuses because
  the inclusive comparisons fail. Derived `ci` remains diagnostic and has no
  independent threshold.
- Actual updates: every installed update still passes
  `is_strict_residual_decrease`. Evaluation/step refusal of the first
  domain-valid witness falls through to that unchanged strict-decrease search;
  exhaustion retains typed `BacktrackingLimit` failure.
- Diagnostic semantics: successful halved witnessing supplies
  `backtracking_count + exponent` to the existing diagnostic and supplies its
  prospective step norms. No separate exponent, serialized field, public
  field, or persistent microstepping surface was added.
- Protected numerics: comparison with the pre-change source confirms the
  unchanged `1e-7 mm`, `1e-10`, `1e-8 K`, and `1e-12 kg kg^-1` step
  thresholds, `MAX_NEWTON_ITERATIONS`, `MAX_BACKTRACKING_HALVINGS`, and strict-
  decrease call. No bound, residual threshold, pivot rule, constitutive law,
  60-second floor, ledger, receipt, custody, phase, topology, event, or
  rollback rule changed in the 001F source diff.
- Public API parity: a zero-context public-item diff scan found no added,
  removed, or changed public item. All controller/refusal/probe additions are
  private; the existing public `CoveredStepNorms` shape is unchanged.
- Production diagnostic scan found no `eprintln!`, `println!`, or `dbg!` in
  `solver_covered_solve.rs`.

## Direct execution

Ran from `/workdir/openWEPP`:

```text
nix develop -c cargo nextest run -p openwepp-land-surface-energy \
  -E 'test(covered_halved_no_update_witness_tests)'
```

Result: `PASS`, run `a1f7fec7-d6d5-4b4f-aa44-3426498e8a45`, 5/5 passed,
92 skipped. This directly covers both typed full-trial refusals, first-valid
ordering, incomplete/step-refusing stop behavior, no-trigger refusal, complete
residual member poisons, and every governed step-coordinate poison.

```text
nix develop -c cargo nextest run -p openwepp-land-surface-energy \
  -E 'test(covered_oracle_conformance_tests::covered_multirank_potential_fixed_cap_and_alternate_start_match_frozen_oracle) | test(covered_oracle_conformance_tests::covered_natural_failures_match_frozen_diagnostics_and_publish_no_candidate) | test(transaction::tests::numerical_failure_errors_preserve_kind_diagnostics_and_rollback_lineage)'
```

Result: `PASS`, run `a34651f3-fcac-4aa6-862c-2d5058db83f6`, 3/3 passed,
94 skipped. The frozen oracle, natural numerical-failure/no-publication, and
typed failure-diagnostic/rollback-lineage obligations retain their prior
disposition.

```text
nix develop -c cargo nextest run -p openwepp-land-surface-energy
```

Result: `PASS`, run `cf69b0fc-c62a-4ac8-b56f-4d0152a93e17`, 97/97 passed,
0 skipped.

```text
nix develop -c cargo fmt --all -- --check
```

Result: `PASS`, exit 0.

```text
git diff --check -- \
  crates/openwepp-land-surface-energy/src/solver_covered_solve.rs \
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md \
  docs/work-packages/20260830-workspace-gate-hold-lift-001/artifacts/science-contracts/SC-LANDSURFACEENERGY-001
```

Result: `PASS`, no output, including the completed verification artifact.

## Findings and disposition

### `001F-VA-001` — PASS — implementation and focused verification

No contract/solver defect was found. The current controller, production call
site, diagnostics, tests, and protected numerical/failure behavior conform to
the preserved v13 amendment plus the v14 cumulative-exponent clarification.

### `001F-VA-002` — EXTERNAL HOLD — real-consumer completion

The two contract-required interior terminal-event consumers were deliberately
not rerun in this verification. Their prior post-001F run cleared the original
iteration-4 `LSEB-E-034` but reached the separately owned
`WGHL-FULL-001H` successor-chronology guard. Per verifier assignment, the long
consumers remain parent-deferred until 001H is stable. Closure still requires
both unchanged consumers to pass and directly prove current-state acceptance,
unchanged owner/ledger closure, and no trial installation. This is an external
HOLD, not a local 001F solver failure and not a waiver.

### `001F-VA-003` — EXTERNAL HOLD — canonical source-bound admission

Review B finding `LSE-001F-B-003` remains present at this verification
identity. Static scans find no `INV-LANDSURFACEENERGY-139` or first-domain-
valid assertion in
`tests/integration/land_surface_energy_balance_authority_contract.rs`, and no
exact `solver_covered_solve.rs` or authority-test path entry in
`tools/release/authority-policy/impact-map.json`. The lifecycle-row assertion
has advanced to v14, but that does not protect the retained v13 invariant.
Parent-owned binding/A0/anti-evasion closure remains required; it is not a
production-code defect and was outside this verifier's write set.

## Final recommendation

`HOLD` on external evidence only. The exact 001F solver source passes static
verification, five direct controller/predicate vectors, three protected
oracle/failure/rollback vectors, the complete 97-test LSE crate, workspace
rustfmt, diff hygiene, public-API parity, line-count governance, and no-print
inspection. Do not close the contract cycle until the parent lands and proves
the source-bound INV-139 admission and the two unchanged real consumers pass
after 001H stabilization.
