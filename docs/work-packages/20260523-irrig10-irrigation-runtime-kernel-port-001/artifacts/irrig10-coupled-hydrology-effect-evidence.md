# IRRIG10 Coupled Hydrology Effect Evidence

Status: `completed`
Evidence mode: `Ran`

## Fixed-Date Coupling Effect

Source test:

- `irrig10_fixeddate_contract_vector_couples_irrigation_depth_into_runoff_and_storage`

Observed coupled effects:

- Irrigated run produces greater runoff than non-irrigated baseline (`Q_irrigated > Q_baseline`).
- Irrigated run produces greater storage reconciliation than baseline
  (`wb12_storage_reconciled_irrigated > wb12_storage_reconciled_baseline`).
- Irrigation depth is explicitly emitted as `Irr = 0.4`.

## Depletion Coupling Effect

Source test:

- `irrig10_depletion_contract_vector_activates_period_trigger`

Observed coupled effects:

- Depletion-triggered event emits positive irrigation depth `Irr = 0.15`.
- Runtime schedule-source trace confirms depletion branch (`source = 1`).
