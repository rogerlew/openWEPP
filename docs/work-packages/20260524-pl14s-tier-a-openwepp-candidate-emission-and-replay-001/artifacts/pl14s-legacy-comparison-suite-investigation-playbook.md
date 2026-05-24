# PL14S Legacy Comparison Suite Investigation Playbook

Status: `completed`
Evidence mode: `Static + Ran`

## Static
- Investigation sequence for future PL14S/PL15S reruns:
  1. Emit openWEPP candidate `H*.wat.parquet` with `open_wepp_runner run-hillslope`.
  2. Run `run_pl14s_legacy_suite.py` with baseline fixture + binary + candidate wat path.
  3. Review semantic report row-key coverage first:
  - `common_row_count`
  - `only_baseline_count`
  - `only_candidate_count`
  4. If row sets overlap, inspect per-column tolerance stats and `top_divergent_rows`.
  5. If strict lane is skipped (parquet candidate), confirm skip reason is explicit in provenance.

## Ran
- Applied this playbook in PL14S execution:
  - identified initial blocker (`pyarrow` missing) and remediated via locked dependency sync:
```bash
uv pip sync --python /tmp/pl14s_venv/bin/python tools/legacy_comparison_suite/requirements.lock.txt
```
  - identified stale/non-rebuilt runner binary behavior (non-parquet output payload), rebuilt binaries, and reran candidate emission:
```bash
cargo build -p openwepp-runner --bin open_wepp_runner --bin openwepp-cli-hill
```
  - reran suite successfully and captured persisted artifacts.
- Investigation conclusion from this run:
  - lane is reproducible and provenance-valid,
  - semantic parity remains failed due row-set non-overlap, not due comparator execution failure.
