# Unit Governance Gap Analysis

Status: completed
Evidence mode: static

Static: HPHYS0277 closes one identified unit-governance gap: finite but
physically impossible hourly winter radiation was previously not rejected.

Ran: not-run; validation commands are recorded in `gate-results.md`.

## Closed Gap

- Gap: after HPHYS0272 corrected `radly` to `radmj`, runtime still accepted
  finite high hourly radiation values if a future unit or branch error produced
  an impossible `MJ m^-2 h^-1` flux.
- Closure: production now derives a physical upper bound from baseline
  `radcur.for` potential-radiation lineage and rejects violations before
  boundary publication.
- Governance posture: no fixed heuristic cutoff, clipping, capping, or
  downstream compensation was introduced.

## Remaining Gaps Outside HPHYS0277

- HPHYS0276 raw dimensional conversion candidate inventory remains the broader
  unit-governance remediation queue.
- HPHYS0278 output unit metadata registry alignment remains queued.
- HPHYS0279 machine-checkable `SC-*` unit-compliance lint remains queued.
- Snowpack/ET/storage semantic residuals remain outside this package and retain
  the current HOLD posture.
