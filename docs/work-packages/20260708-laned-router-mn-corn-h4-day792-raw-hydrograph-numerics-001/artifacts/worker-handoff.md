# Worker Handoff

Status: `READY-FOR-FOLLOW-ON`.

## Next Package

Scaffold `20260708-laned-router-active-router-timestep-policy-adjudication-001`.

## Objective

Resolve whether active-router target-`dx` adequacy must be adjudicated as a
coupled space-time convergence policy before renewing `dx5` production mesh
promotion.

## Starting Evidence

Read:

- This package's `artifacts/mechanism-attribution.md`
- This package's `artifacts/hold-legitimacy-audit.md`
- `SC-OFEROUTE-001` rev-41 active router sections
- `20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001`
- `20260708-laned-router-mn-corn-h4-routed-shape-attribution-001`

Binding observation:

- `dx1p25` vs `dx0p625` fails the raw-hydrograph/routed-shape adequacy check.
- Source, upstream, clamp, limiter, and boundary-sign discriminants are clean.
- The fine rung crosses from 300-second cap-limited stepping into CFL-limited
  stepping.

## First Actions

- Add or use a controlled diagnostic timestep mechanism that does not change
  default/off behavior.
- Rerun `mn_corn_h4` day 792 lane 1 at fixed `dx1p25` with `max_dt` halved
  (`300`, `150`, `75` seconds at minimum).
- Compare with `dx0p625` under comparable timestep controls.
- If timestep refinement explains the miss, draft a contract-first timestep
  adequacy rule before any renewed mesh-policy promotion.
- If timestep refinement does not explain the miss, hold for a deeper
  numerical analysis package with the step trace evidence attached.

## Non-Actions

- Do not widen routed-shape thresholds.
- Do not promote `dx5`.
- Do not change production `LANED_ACTIVE_MAX_DT_S` without contract authority
  and selected-cohort evidence.
- Do not revive hybrid stepping.
