# IRRIG10 Runtime Authority and Guard Map

Status: `completed`
Evidence mode: `Static`

## Runtime Authority to Code Map

- Schedule-source resolution:
  - `resolve_active_irrigation_event`
  - precedence: fixed-date first, depletion second
- Fixed-date event resolution:
  - `resolve_fixeddate_irrigation_event`
  - keyed by `(day, year, ofe_id=1)`
- Depletion event resolution:
  - `resolve_depletion_irrigation_event`
  - keyed by period window + depletion trigger threshold
- Runtime payload normalization:
  - `normalize_irrigation_event`
  - enforces finite/non-negative depth/rate/duration and bounded duration
- Coupled runoff execution:
  - `run_runoff_reconciliation`
  - rainfall closure includes irrigation depth
  - runoff/infiltration liquid forcing includes irrigation
  - publishes runtime traces + `Irr`
- Coupled storage execution:
  - `run_storage_reconciliation`
  - consumes `Irr` from flux surface in WB12 closure

## Guard Mapping

- Missing required irrigation/schedule symbols:
  - `HKERNEL-WB14-RUNOFF-E-001`
- Non-finite irrigation/schedule symbols:
  - `HKERNEL-WB14-RUNOFF-E-002`
- Irrigation domain/closure violations:
  - `HKERNEL-WB14-RUNOFF-E-003`
- Storage-side missing/non-finite/domain posture for coupled closure:
  - `HKERNEL-WB12-STORAGE-E-001..003`

## Runtime Symbols Published

Runoff phase writeback publishes:

- State: `irrigation.runtime_schedule_source`, `irrigation.runtime_depth_m`,
  `irrigation.runtime_duration_s`, `irrigation.runtime_rate_m_per_s`,
  `irrigation.runtime_event_index`, `irrigation.runtime_system_type`
- Flux: `Irr`

Storage phase consumes:

- Flux: `Irr`
