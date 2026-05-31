# HPHYS0217 Verification Agent A

Status: completed
Evidence mode: Static + Ran

## Verification checks
1. Confirmed rerun status files contain `39` success rows each:
   - `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_batch_status.tsv`
   - `/tmp/hphys0217_20260531T071120Z/parity/reports/semantic_status.tsv`
2. Confirmed monitored-family summary exists and values are internally
   consistent in:
   - `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.json`
   - `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.tsv`
3. Confirmed comparison claim vs HPHYS0216 reference summary.

## Result
- pass
