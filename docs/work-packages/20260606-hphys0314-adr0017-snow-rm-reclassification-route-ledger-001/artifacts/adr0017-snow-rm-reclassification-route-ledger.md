# ADR0017 Snow/RM Reclassification Route Ledger

Status: complete

Evidence mode: Static

Static:

- Source ledger:
  `docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/snowpack-settling-carry-recursion-ledger.json`.
- Source contract authority:
  `SC-SNOWFREEZE-001#INV-SNOWFREEZE-040` and
  `SC-WATBAL-001#INV-WATBAL-088`.
- ADR0017 verdict taxonomy retained here:
  `HARNESS-SURFACE-MISMATCH`, `LEGACY-DEFECTIVE`, `OPENWEPP-DEFECTIVE`,
  `UNRESOLVED`.

## Reclassification Rules

1. Stale HPHYS0298 `OPENWEPP-DEFECTIVE` labels are superseded; they are not
   production authority after ADR0017.
2. `OPENWEPP-DEFECTIVE` requires same-unit/same-lineage proof and independent
   correctness authority. HPHYS0314 adds no such proof.
3. `HARNESS-SURFACE-MISMATCH` is reserved for proven surface or lineage-stage
   mismatches; HPHYS0314 does not relabel the HPHYS0313 rows as harness defects.
4. HPHYS0313 rows remain `UNRESOLVED` and owned `HOLD` with explicit follow-on
   gates.
5. `production_edit_authorized=false` for every row.

## Route Ledger

| Hillslope | Window | Target year | Source HPHYS0312 route | HPHYS0313 route | Carried rows | Key evidence | ADR0017 verdict | Owner/follow-on gate | Production edit |
|---:|---|---:|---|---|---:|---|---|---|---|
| 1 | spring-2014 | 2014 | `settling-depth-update-hold` | `hourly-snowfall-input-lineage-hold` | 8 | baseline `hrsnow=0.0007454545120708644 m`; openWEPP homologous snowfall depth `0.0 m`; delta `-0.0007454545120708644 m` at 2013 day 11 hour 11 | `UNRESOLVED` | HPHYS0315 hourly snowfall input lineage | `production_edit_authorized=false` |
| 1 | spring-2016 | 2016 | `year-start-inherited-state-hold` | `recursive-year-start-inherited-state-hold` | 15 | first material paired divergence already present at 2014 day 1 hour 1; depth delta `0.013144251023522513 m` | `UNRESOLVED` | HPHYS0316 2013 terminal carry recursion | `production_edit_authorized=false` |
| 7 | spring-2014 | 2014 | `settling-depth-update-hold` | `hourly-snowfall-input-lineage-hold` | 7 | baseline `hrsnow=0.0007454545120708644 m`; openWEPP homologous snowfall depth `0.0 m`; delta `-0.0007454545120708644 m` at 2013 day 11 hour 11 | `UNRESOLVED` | HPHYS0315 hourly snowfall input lineage | `production_edit_authorized=false` |
| 7 | spring-2016 | 2016 | `year-start-inherited-state-hold` | `recursive-year-start-inherited-state-hold` | 9 | first material paired divergence already present at 2014 day 1 hour 1; depth delta `0.015279465660242741 m` | `UNRESOLVED` | HPHYS0316 2013 terminal carry recursion | `production_edit_authorized=false` |
| 39 | spring-2014 | 2014 | `settling-depth-update-hold` | `hourly-snowfall-input-lineage-hold` | 9 | baseline `hrsnow=0.0007454545120708644 m`; openWEPP homologous snowfall depth `0.0 m`; delta `-0.0007454545120708644 m` at 2013 day 11 hour 11 | `UNRESOLVED` | HPHYS0315 hourly snowfall input lineage | `production_edit_authorized=false` |
| 39 | spring-2016 | 2016 | `year-start-inherited-state-hold` | `recursive-year-start-inherited-state-hold` | 9 | first material paired divergence already present at 2014 day 1 hour 1; depth delta `0.0147979087518893 m` | `UNRESOLVED` | HPHYS0316 2013 terminal carry recursion | `production_edit_authorized=false` |

## Supersession

- Superseded label: stale HPHYS0298 `OPENWEPP-DEFECTIVE`.
- Replacement disposition: `UNRESOLVED` owned `HOLD` under HPHYS0315 or
  HPHYS0316 until same-unit/same-lineage proof and independent correctness
  authority exist.
- Authorized production edits from this package: none.
