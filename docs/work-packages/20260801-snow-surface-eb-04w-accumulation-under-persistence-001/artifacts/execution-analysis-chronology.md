# Intermediate Execution And Analysis Chronology

Evidence mode: **Ran**.

The command

`/usr/bin/time -f elapsed... .venv/bin/python
docs/work-packages/20260801-snow-surface-eb-04w-accumulation-under-persistence-001/tools/run_accumulation_diagnostics.py
--execute --workers 4`

completed all 16 model cells and wrote `execution-receipt.json` before entering
analysis. The receipt binds the in-memory execution tool SHA-256
`38764e678a3ea53fe3e423676680470c26103f2f7c5186d0ecc118f32da95535`
and release binary SHA-256
`b28e241bed0fa3d21eaf94cab4ab7bbb4e642734027eed90b269901e66fd3ded`.

During analysis, the synthesis prose was corrected to distinguish seasonal
peak-magnitude ratios from storage retained on observed peak dates. Because
that changed the on-disk analysis tool while the process retained the old
in-memory module, the analysis phase was intentionally interrupted with
SIGINT. The raw time record therefore truthfully contains both “signal 2” and
the misleading `/usr/bin/time %x` value `0`; it is not a successful combined
workflow receipt.

The execution boundary was valid at that source identity: the receipt is written only after all 16
cells return zero and binds the exact execution tool, binary, production source
hashes, per-cell provenance hashes, command, and working directory. A fresh
timed `--analysis-only` invocation under the stable current analysis tool is the
authoritative analysis boundary. No model cell is reused across a production
source change. It was later superseded by the snowbench phase-operand correction
documented in `invalidated-pre-snowbench-phase-fix.md`; neither this receipt nor
its outputs are terminal evidence.
