# Calibration Readiness Matrix

Status: `PASS`

Evidence class: `Ran`

| Obligation | Status | Evidence path | Rationale |
|---|---|---|---|
| typed/enumerable parameter surface | `PASS` | `candidate-configurations.csv; later-stage-design.csv` | All axes and complete deterministic levels are retained. |
| observation operator with units and scale | `PASS` | `executor-schema.md; later-stage-results.csv` | Interval timing, biomass, LAI, activity, and cover operators retain units and evidence roles. |
| deterministic candidate execution | `PASS` | `/home/workdir/cal04b-attempt-20260728-pathfix/objects/hubbard-gsi-identity.csv` | Native raw trace has exact candidate/calendar/source/binary identity. |
| objective reconstruction | `PASS` | `/home/workdir/cal04b-attempt-20260728-pathfix/objects/verification/verification-receipt.csv` | Independent reconstruction is byte-identical across all derived ledgers. |
| sensitivity analysis | `PASS` | `later-stage-results.csv` | Native finite differences include central and boundary schemes. |
| identifiability/confounding analysis | `PASS` | `identifiability-and-equifinality.md` | GSI ensemble and all downstream equifinal sets remain explicit. |
| boundary, saturation, and failure reporting | `PASS` | `saturation-evidence.csv; failure-ledger.csv; later-stage-results.csv` | All candidate families, typed failures, and enumeration boundaries are retained. |
| equifinality/uncertainty retention | `PASS` | `accepted-calibration-ensemble.csv; later-stage-membership.csv` | No convenience point selection replaces accepted membership. |
| synthetic recovery | `PASS` | `synthetic-recovery-results.csv` | Five native recovery cases pass with limits stated. |
| additional-data inventory | `PASS` | `additional-data-inventory.csv` | Measurements needed for stronger separation are stage-specific. |
