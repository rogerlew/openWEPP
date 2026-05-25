# simimpl13-contract-test-blind-spot-assessment

Status: phase-c-complete
Evidence mode: Static + Ran
Date: 2026-05-25

## Static
- This artifact identifies contract-derived test coverage gaps that currently
  permit replay/parity blockers to persist undetected by automated gates.

## Ran
- Inspected runner and integration tests plus comparator tooling scripts:
  - `crates/openwepp-runner/tests/simimpl04_*.rs`
  - `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`
  - `tests/integration/pl14_tier_a_candidate_replay_contract.rs`
  - `tests/integration/pl14r_tier_a_replay_rerun_contract.rs`
  - `tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs`
  - `tools/legacy_comparison_suite/run_pl14s_legacy_suite.py`
  - `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`

## Blind-spot register
| blind_spot_id | statement | evidence | closure requirement | status |
|---|---|---|---|---|
| `SIMIMPL13-TEST-001` | No contract-derived test asserts replay candidate trajectory span against baseline (row-count parity or minimum overlap). | Runner tests validate manifest fields; no row-span assertions over emitted `H*.wat` surfaces. | Add end-to-end replay-span tests asserting non-zero overlap and expected keyed extent for canonical fixture lane. | open |
| `SIMIMPL13-TEST-002` | No test enforces row-key semantic alignment (`Y` domain) between candidate and baseline comparators. | Comparator reports `common_row_count=0` with candidate key `(1,1,2000)`; no failing preflight contract test currently catches this. | Add key-domain preflight gate test: candidate key policy must align baseline replay key semantics before comparator execution. | open |
| `SIMIMPL13-TEST-003` | Parquet semantic alias drift for `Total-Soil` is not regression-tested. | Semantic comparator map expects `"Total-Soil Water"`; candidate parquet provides `"Total-Soil"`; parquet lane reports investigation-column missing. | Add comparator unit/integration tests covering parquet field-name alias continuity for required investigation columns. | open |
| `SIMIMPL13-TEST-004` | Strict comparator skip for parquet lanes has no compensating test gate that requires equivalent strict evidence for promoted replay closure claims. | `run_pl14s_legacy_suite.py` marks strict skipped for parquet without enforceable closure compensation logic. | Add governance test asserting promotable claims require strict-equivalent evidence route when parquet strict lane is skipped. | open |
| `SIMIMPL13-TEST-005` | Dat lane can consume conversion-derived one-row candidate without explicit test differentiating conversion-surface artifacts from native runtime dat emission. | `parquet_to_dat.log` reports `rows=1`; strict comparator then runs against converted single-row dat. | Add provenance test requiring explicit source classification and row-count consistency checks between dat/parquet candidate surfaces. | open |

## Phase C conclusion
- Existing tests prove manifest/schema/tooling markers but do not gate the
  span/key comparability invariants required for promotable replay closure.
