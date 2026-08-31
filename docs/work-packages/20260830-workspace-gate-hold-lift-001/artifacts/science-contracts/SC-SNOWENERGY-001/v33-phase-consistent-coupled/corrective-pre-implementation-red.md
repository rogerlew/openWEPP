# SC-SNOWENERGY-001 v33 corrective pre-implementation red

Date: 2026-08-30

Disposition: the initial v33 pre-red is invalidated for the trigger/equation
seam. Design review proved that bitwise raw-authentic `A==A` is not the retained
reset and that the current `F(x)-x` draft does not expose physical residuals.
This corrective red preserves contract version 33 and releases implementation
only for the exact transition-reset detector, equation-level physical evaluator,
one shared evaluation budget, and Picard-only `CoupledAuthentic` admission.

## Corrected authority

- Trigger exactness applies to unchanged support/source/event/topology/custody/
  receipt joins and the ordered transition `root/interface -> branch-entry ->
  opposite pure-vapor raw-authentic -> same root/interface/reset coordinates and
  branch predicates`. Asymptotically changing raw-authentic continuous owner
  fields need not be bitwise equal.
- `CoveredPhaseConsistentResidualEvaluationV1` must carry concrete `R_W`, `R_H`,
  `R_E`, and `R_T` reconstructed through the unchanged water, snow-energy,
  soil-CN, soil enthalpy--temperature, LSE, and receipt equations/constraints.
  Coordinate-map `F(x)-x` is forbidden.
- One `CoveredPhysicalEvaluationBudgetV1`, bounded by the unchanged 96, spans
  trigger confirmation, residual/Jacobian, trust and rejected trials, fresh
  evaluation, and final replay without reset.
- `CoveredConvergenceAdmissionV1::CoupledAuthentic` may bypass only ordinary
  Picard current/candidate equality and convergence. All residual tolerances,
  side constraints, finalization, reseal, event, identity, custody, rollback,
  and publication guards remain active.

## Commands and results

| Command | Result |
|---|---|
| `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | PASS: 20 binding-exposure rows consolidated. |
| `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | PASS: no findings. |
| `nix develop -c rustfmt --edition 2021 --check tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | PASS. |
| `git diff --check -- <authorized corrective paths>` | PASS. |
| `nix develop -c rustc --edition 2021 --test tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs -o /tmp/wghl_v33_corrective_contract_test` | PASS. |
| `/tmp/wghl_v33_corrective_contract_test --test-threads=1` | EXPECTED RED: 9 passed, 1 failed. All retained contract/oracle tests pass; sole failure is `v33_corrective_production_seams_are_required`. |

The single aggregated expected-red failure reports:

- forbidden coordinate-map `F(x)-x` residual draft;
- forbidden bitwise raw-authentic `A==A` trigger;
- missing `phase_consistent_coupled_active_set_transition_reset_v1`;
- missing `CoveredPhaseConsistentResidualInputsV1`,
  `CoveredPhaseConsistentResidualEvaluationV1`, and
  `covered_phase_consistent_residual_evaluate_v1`;
- missing equation-level `r_w_kg_m2`, `r_h_j_m2`, `r_e_j_m2`, and `r_t_k`;
- missing `CoveredPhysicalEvaluationBudgetV1` and
  `covered_physical_evaluation_budget_charge_v1`;
- missing `CoveredConvergenceAdmissionV1::CoupledAuthentic` and its enum/final
  dispatch seam; and
- missing corrective transition, residual, shared-budget, and Picard-only
  admission behavior vectors.

## Frozen current-draft sources

| Source | SHA-256 | Git object |
|---|---|---|
| `v11_covered/phase_consistent_coupled_solve.rs` | `85f837a509c3dbceb58bf9ec2cfe9026b3aa2ea55f3f62062eb4e35a33bd41f9` | `b4e3ae995500901e4d81cd826105f4cfbb76b9e8` |
| `v11_covered/fixed_point.rs` | `754d1a8e6b2166a8ab4902e89413ad694e9ab7269ea6d22a1973261029f16090` | `611390e476238f542a657805bfd1ee59599d3fbb` |
| `v11_covered/open_snow.rs` | `2cd7f47ce537cf3760313ff52779b5ce8cf27bd7c69a21f2d9251826558a11e8` | `205f0baec8135341d1741c6477361df622aed093` |
| `v11_covered/open_snow_convergence_tests.rs` | `75b18e35269bb8d10fca6b32416ad780bd723ed24e54a7c71304773521f9c3fa` | `e5707749bae21e687720e8dab1ab47ac81efc4ab` |
| `v11_covered/owner_finalization.rs` | `f9a29cdf90be236a4bca3a7a03dbe5b708e254e8a908d7695d44cd711b0d5417` | `67c8be5d917a3c15a68a5e088b0d0a0735512247` |

Shared-worktree HEAD at corrective freeze:
`58217f532d126b967bfe93b6e52417c61a974bcd`. No production file was edited by
this contract-first corrective slice.

## Corrective contract hashes

| File | SHA-256 |
|---|---|
| `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `a3a4de78b02af73de9a336ece5e6569d2871ea58c6f55512d75100f68a1e016c` |
| `docs/specifications/science-contracts/index.md` | `cc881b9a7d7dd32405244a580ec8554dd27371955e462d05919f0fe3abae71e9` |
| `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | `904bc6a3a232ada49e066db432c9002ea203efebf7b029ac8834347d39af5385` |
| `docs/work-packages/20260830-workspace-gate-hold-lift-001/package.md` | `4faaa3fea66da16cad0960494388092890b518e9ce5864a0fdd2cb38bdb5edee` |

Any source or contract change invalidates these hashes and requires the
corrective expected-red to be rerun. The exact 60-second floor, tolerances,
physics, events, topology, custody, receipts, rollback, schema, no-persistent-
diagnostics rule, and publication safeguards remain unchanged.
