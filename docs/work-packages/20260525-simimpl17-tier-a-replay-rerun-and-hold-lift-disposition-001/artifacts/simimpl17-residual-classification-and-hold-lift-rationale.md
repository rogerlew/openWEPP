# simimpl17-residual-classification-and-hold-lift-rationale

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL13 hold policy remains authoritative:
- hard criteria `CRIT-001..007` must pass for `GO`.
- `CRIT-008` may remain partial only with explicit risk acceptance.

## Ran
- Residual classes after SIMIMPL17 rerun:
- `open-hard`: `SIMIMPL13-CRIT-001`, `SIMIMPL13-CRIT-002`,
  `SIMIMPL13-CRIT-003`, `SIMIMPL13-CRIT-004`.
- `closed`: `SIMIMPL13-CRIT-005`, `SIMIMPL13-CRIT-006`,
  `SIMIMPL13-CRIT-007`.
- `governance-partial`: `SIMIMPL13-CRIT-008`.

## Hold-lift rationale
- Hold-lift cannot be granted because required hard closure criteria fail.
- Primary unresolved blocker class is replay span/key parity closure against
  baseline (`1093` unmatched baseline rows).
- Dat strict lane is non-promotable and guard-terminated under
  conversion-derived row-consistency policy.
- No risk-acceptance reference is provided for `CRIT-008` partial status.

## Ownership posture
- Residual closure ownership remains with the replay/parity implementation
  stream tracked from SIMIMPL13 queue artifacts.
- Follow-on package planning is required before hold-lift can move to `GO`.
