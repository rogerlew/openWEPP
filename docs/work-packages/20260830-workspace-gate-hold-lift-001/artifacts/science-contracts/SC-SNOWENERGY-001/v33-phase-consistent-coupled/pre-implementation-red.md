# SC-SNOWENERGY-001 v33 pre-implementation expected red

Date: 2026-08-30

Disposition: expected red is isolated to missing version-33 production
symbols/behavior. Contract, independent phase-projection oracle, retained v32
contract/production obligations, formatting, binding-exposure lint, unit lint,
and exact diff checks pass against the frozen production sources.

## Commands and results

| Command | Result |
|---|---|
| `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | PASS: 20 binding-exposure rows fully consolidated. |
| `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | PASS: no findings. |
| `nix develop -c rustfmt --edition 2021 --check tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | PASS. |
| `git diff --check -- docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md docs/specifications/science-contracts/index.md tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | PASS. |
| `nix develop -c rustc --edition 2021 --test tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs -o /tmp/wghl_v33_contract_test` | PASS. |
| `/tmp/wghl_v33_contract_test --test-threads=1` | EXPECTED RED: 8 passed, 1 failed. Sole failure: `v33_production_symbols_and_behavior_are_required`, first missing symbol `PhaseConsistentCoupledSolveV1`. |

The isolated harness therefore releases implementation only for the missing
private v33 production surface and behavior:

- `PhaseConsistentCoupledSolveV1`
- `phase_consistent_coupled_solve_v1`
- `phase_consistent_coupled_authentic_final_evaluation_v1`
- `phase_consistent_coupled_authentic_final_replay_reseal_v1`
- exact 60/120-second authentic-cycle dispatch;
- cold, mixed-phase, and fusion-boundary roots;
- root distinction from all v31/v32 affine/synthetic states;
- coupled-authentic final replay/reseal;
- full poison rollback and one cumulative 96-evaluation budget.

## Canonical contract hashes

| File | SHA-256 at expected red |
|---|---|
| `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `359160d9aa8cf6e054f69108dd6fe5ab962f13fdfb1624de32c40918522e0684` |
| `docs/specifications/science-contracts/index.md` | `5780b717ce68127f6716dbb2d3abbcfecf2f95eec59bcf59c6d800def00f5e68` |
| `tests/integration/snow_terminal_enthalpy_event_numerics_contract.rs` | `7bb17deff5eade2e834019b9523942013ae896dedcf7e6dc2641d2c0b469b3cc` |

The implementation gate must invalidate and rerun this evidence after any
contract, index, test, or frozen production-source change. Canonical one-day
qualification must report accepted/rejected counts, accepted-width
distribution, solver calls/residual evaluations, wall time, limiting rejection
reasons, maximum ledger residuals, and absence of repeated 96-evaluation
ceilings; microstepping diagnostics must not persist in production.
