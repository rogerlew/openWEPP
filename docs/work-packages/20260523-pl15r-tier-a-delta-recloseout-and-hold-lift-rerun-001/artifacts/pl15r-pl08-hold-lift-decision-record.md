# PL15R PL08 Hold-Lift Decision Record

Status: `complete`
Evidence mode: `Static + Ran`
Decision: `RETAIN PL08 HOLD`
Decision date: `2026-05-23`

Supersedes:
- `docs/work-packages/20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/artifacts/pl15-pl08-hold-lift-decision-record.md`
- prior `LIFT` wording previously recorded in this PL15R package

Static:
- Decision authority follows ADR-0011 confidence-tier posture with explicit
  provenance-validity constraints for Tier-A parity claims.

Ran:
- Evaluated PL14R strict replay artifacts, provenance manifest, schema-aligned
  retest method, and PL15R criteria matrix outputs.

## Decision Criteria Evaluation

| criterion | target | result |
|---|---|---|
| Direct strict Tier-A replay evidence exists | reproducible artifacts sourced from openWEPP executable candidate lane | `fail` |
| Required Tier-A include-surface completeness (`H5.wat.dat`, `H5.plot.dat`) | strict pass on provenance-valid openWEPP candidate lane | `fail` |
| Tier-A blocker set empty or approved risk acceptance | blocker set empty OR approved explicit reference | `fail` |
| No implicit down-classification | provenance and historical-failure handling are explicit | `pass` |
| Refreshed semantic parity direction supports hold lift | acceptance-positive on provenance-valid Tier-A evidence | `fail` |

## Rationale

1. PL14R schema-aligned strict-pass evidence is not authoritative for PL08
   hold-lift because candidate provenance uses legacy lane substitution.
2. No openWEPP executable candidate lane is available to produce authoritative
   Tier-A replay evidence in current package posture.
3. Active physics-authority blockers remain for full parity claims.
4. No approved risk-acceptance artifact exists for these blockers.

## Final Verdict

`RETAIN PL08 HOLD`

## Forward Path

Parity recovery packages are defined in:
- `docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`
  (`CLI10`, `WB17..WB20`, `PL14S`, `PL15S`)

## References

- `docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/pl14r-comparator-run-provenance-manifest.md`
- `docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/pl14r-schema-aligned-day-by-day-retest.md`
- `docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-closeout-decision-criteria-matrix.md`
- `docs/work-packages/20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/artifacts/pl15r-risk-acceptance-approval-reference.md`
