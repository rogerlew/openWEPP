# Snowbird precipitation-scaled development lane

`p8.cli` is a deterministic, development-only derivative of the canonical
Snowbird climate file two directories above. It changes daily precipitation
only by the exact decimal factor `1.2155576`, rounded to the CLIGEN field's
`0.1 mm` resolution with decimal `ROUND_HALF_UP`.

This lane is not precipitation truth, an observation, a calibration, a model
default, independent validation of snow physics, or evidence transferable to
another site. It exists so future snow work packages can distinguish the known
Snowbird input-mass limitation from model-process behavior.

To consume it, copy the complete canonical Snowbird fixture into a fresh run
directory and then replace only the staged `p8.cli` with this file. Never
overwrite the canonical fixture. Regenerate or verify it with:

```bash
.venv/bin/python \
  docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/materialize_snowbird_development_cli.py
.venv/bin/python \
  docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/materialize_snowbird_development_cli.py --check
```

`manifest.json` freezes the source and derivative hashes, row counts, totals,
transform, consumer protocol, and claim limits.
