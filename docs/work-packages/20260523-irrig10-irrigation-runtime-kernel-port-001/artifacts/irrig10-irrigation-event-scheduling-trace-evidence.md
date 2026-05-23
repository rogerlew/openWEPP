# IRRIG10 Irrigation Event Scheduling Trace Evidence

Status: `completed`
Evidence mode: `Ran`

## Trace Vector A: Fixed-Date Scheduler Activation

Source test:

- `irrig10_fixeddate_contract_vector_couples_irrigation_depth_into_runoff_and_storage`

Observed trace outcomes:

- Fixed-date event keyed by `(day=120, year=1, ofe=1)` activated.
- Runtime trace surfaces emitted with fixed-date source marker:
  - `irrigation.runtime_schedule_source = 2`
- Coupled irrigation depth published:
  - `Irr = 0.4`

## Trace Vector B: Depletion Scheduler Activation

Source test:

- `irrig10_depletion_contract_vector_activates_period_trigger`

Observed trace outcomes:

- Depletion period window/threshold path activated.
- Runtime trace surfaces emitted with depletion source marker:
  - `irrigation.runtime_schedule_source = 1`
- Coupled irrigation depth published:
  - `Irr = 0.15`

## Trace Vector C: Missing Climate Schedule Key

Source test:

- `irrig10_contract_vector_missing_schedule_day_symbol_is_typed`

Observed trace outcomes:

- Scheduler halted at `RunoffReconciliation`.
- Typed failure code emitted:
  - `HKERNEL-WB14-RUNOFF-E-001`
- Boundary class:
  - `MissingRequiredInput`
