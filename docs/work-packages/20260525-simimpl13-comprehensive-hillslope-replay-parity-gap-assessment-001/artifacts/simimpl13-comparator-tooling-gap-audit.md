# Simimpl13 comparator tooling gap audit

Status: phase-c-complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- Phase C scope is comparator tooling-gap identification and queue-driving
  closure framing.

## Ran
- Reviewed comparator orchestration script:
  - `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
- Reviewed semantic comparator implementation:
  - `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- Cross-checked outputs in SIMIMPL11 replay bundle:
  - `suite_dat/investigation/h5_wat_strict_comparator.json`
  - `suite_dat/investigation/h5_wat_semantic_comparator.json`
  - `suite_parquet/investigation/h5_wat_semantic_comparator.json`
  - provenance manifests for dat/parquet lanes.
- Verified candidate parquet schema via `duckdb` (`DESCRIBE SELECT ...`).

## Tooling gap findings
| gap_id | finding | evidence | impact | status |
|---|---|---|---|---|
| `SIMIMPL13-TOOL-001` | Strict comparator is skipped for parquet candidates by design (`reason: candidate is not .dat`). | `run_pl14s_legacy_suite.py:160`..`165`; parquet provenance `executions.strict_compare.skipped=true` | Strict parity signal is format-dependent and incomplete for parquet-first lanes. | open |
| `SIMIMPL13-TOOL-002` | Semantic parquet column alias map expects `"Total-Soil Water"` rather than actual candidate parquet field `"Total-Soil"`. | `semantic_hillslope_wat_compare.py:55`..`81`; parquet semantic report missing `Total-Soil`; `duckdb DESCRIBE` shows `Total-Soil` present | False structural drift signal in parquet semantic lane (`baseline_only_columns`, investigation-column missing). | open |
| `SIMIMPL13-TOOL-003` | Semantic report encodes parquet numeric width as `[0]`, unlike dat widths `[20|25]`, reducing format diagnostics comparability. | `semantic_hillslope_wat_compare.py:195`..`203`; report `candidate_numeric_widths=[0]` | Limits direct width-class diagnostics and can mask schema-shape expectations for parquet lanes. | open |
| `SIMIMPL13-TOOL-004` | Dat strict lane in SIMIMPL11 consumed a conversion-produced one-row dat candidate, inheriting runtime span collapse. | `candidate/parquet_to_dat.log` (`rows=1`), strict report line-count mismatch | Tooling lane can report structural mismatch but cannot distinguish conversion-lane vs runtime-lane responsibility without extra metadata. | open |

## Evidence-backed non-findings
- Candidate parquet schema already contains canonical investigation fields,
  including `Total-Soil` and `SoilWaterTotal`; the missing `Total-Soil` signal
  is mapping logic drift, not absence in candidate parquet payload.

## Phase C closure hooks
1. Amend parquet alias mapping to canonical `Total-Soil` key handling and add
   regression tests in comparison suite.
2. Decide strict-parquet strategy (native strict support vs explicit governance
   framing that semantic lane is authoritative for parquet).
3. Add explicit provenance tagging to distinguish native candidate dat emission
   from conversion-derived dat surrogates in strict-lane artifacts.

## Phase C conclusion
- Comparator tooling gaps are real but secondary to the dominant span/key-domain
  closure blockers.
- Tooling maintenance is required before final promotable parity claims.
