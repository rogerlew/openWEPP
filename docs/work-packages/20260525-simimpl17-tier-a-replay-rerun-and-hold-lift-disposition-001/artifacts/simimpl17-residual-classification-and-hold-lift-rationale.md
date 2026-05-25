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
  baseline (`730` unmatched candidate rows after shared-key join).
- Dat strict lane remains non-promotable (`structure_diff`, `393` baseline
  lines vs `1095` candidate lines).
- Legacy baseline evidence in both lanes records a one-year clamp
  (`Number of years to simulate can't be larger than 1`), which keeps replay
  span closure unresolved under current lane inputs.
- No risk-acceptance reference is provided for `CRIT-008` partial status.

## Diagnostic clues carried forward
- First shared row mismatch is immediate (`OFE=1`, `J=1`, `Y=1`) with opposing
  winter partition signatures (`baseline RM=0.00` vs `candidate RM=4.40`,
  `baseline Snow-Water=4.40` vs `candidate Snow-Water=250.00`).
- Candidate storage outputs are invariant across all emitted days in this rerun
  (`Total-Soil=76.00`, `frozwt=0.00`, `Snow-Water=250.00`,
  `SoilWaterTotal=76.00`), while baseline storage is dynamic in year 1.
- Semantic report confirms forcing alignment in overlap (`P` column passes), so
  next package should target winter-state/update coupling and hydout emission
  mapping rather than precipitation ingest.

## Ownership posture
- Residual closure ownership remains with the replay/parity implementation
  stream tracked from SIMIMPL13 queue artifacts.
- Follow-on package planning is required before hold-lift can move to `GO`.
