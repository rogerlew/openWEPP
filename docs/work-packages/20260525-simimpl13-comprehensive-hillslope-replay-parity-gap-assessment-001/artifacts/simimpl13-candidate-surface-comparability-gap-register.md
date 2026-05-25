# Simimpl13 candidate surface comparability gap register

Status: phase-c-complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- This register records candidate-surface comparability blockers after Phase C
  assessment. Items remain open until follow-on implementation packages close
  them with execution evidence.

## Ran
- Source evidence consumed:
  - SIMIMPL11 replay bundle strict/semantic comparator JSON artifacts.
  - Candidate run manifest and candidate `H5.wat` outputs.
  - Runner publication path in `crates/openwepp-runner/src/lib.rs`.

## Gap register
| gap_id | blocker statement | comparator impact | contract linkage | owner | status |
|---|---|---|---|---|---|
| `SIMIMPL13-COMP-001` | Candidate WB13/H.wat span is one row while baseline replay span is 1095 keyed rows (1123 strict lines including non-numeric/header rows). | No meaningful row-wise parity comparison can occur. | `SC-WATBAL-001` `INV-WATBAL-017`/`INV-WATBAL-020`; `SC-SYSTEM-001` `INV-SYSTEM-017`/`INV-SYSTEM-020` | runner WB13 publication lifecycle | open |
| `SIMIMPL13-COMP-002` | Row-key domain mismatch: candidate key example `(1,1,2000)` vs baseline key domain `(1, J, Y=1..3)` produces `common_row_count=0`. | Semantic comparator cannot compute shared-row column deltas. | `SC-WATBAL-001` `INV-WATBAL-017`; `SC-SYSTEM-001` `INV-SYSTEM-017` | runner key semantics + comparison suite key policy | open |
| `SIMIMPL13-COMP-003` | Strict comparator lane only runs when candidate surface is `.dat`; parquet lane skips strict comparator by design. | Strict lane coverage diverges by surface format, complicating promotable parity assertions. | `SC-SYSTEM-001` `INV-SYSTEM-012`/`INV-SYSTEM-017` | legacy comparison suite orchestration | open |
| `SIMIMPL13-COMP-004` | Dat strict lane currently depends on parquet-to-dat conversion output (`rows=1`) rather than native replay-length dat emission from candidate runtime. | Strict comparator reflects conversion-limited surface span, not independently emitted runtime-equivalent trajectory. | `SC-WATBAL-001` `INV-WATBAL-020`; `SC-SYSTEM-001` `INV-SYSTEM-020` | runner output strategy + comparison harness staging | open |
| `SIMIMPL13-COMP-005` | Parquet semantic lane reports `baseline_only_columns=["Total-Soil"]` and `investigation_columns_missing=["Total-Soil"]` despite candidate parquet carrying a `Total-Soil` column. | Investigation-column completeness and column-alignment confidence are degraded. | `SC-WATBAL-001` `INV-WATBAL-017`; `SC-SYSTEM-001` `INV-SYSTEM-017` | semantic comparator parquet alias mapping | open |

## Phase C prioritization order
1. `SIMIMPL13-COMP-001`
2. `SIMIMPL13-COMP-002`
3. `SIMIMPL13-COMP-005`
4. `SIMIMPL13-COMP-003`
5. `SIMIMPL13-COMP-004`

## Phase C conclusion
- Candidate-surface comparability is still non-promotable.
- The primary closure dependency remains timeseries span + key-domain overlap;
  tooling alignment remains a necessary secondary closure lane.
