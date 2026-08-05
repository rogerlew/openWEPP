# Snowbird Development Fixture Provenance

Status: materialized and verified

Evidence mode: Ran

- Canonical: `tests/fixtures/snotel_observed/snotel_snowbird_ut/p8.cli`
- Canonical SHA-256 before/after:
  `10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7`
- Derived:
  `tests/fixtures/snotel_observed/snotel_snowbird_ut/development/precip_x1p2155576/p8.cli`
- Derived SHA-256:
  `c673145ee7fd41e71e3f2e21c529fba2d12691abd5f0f055444e621fb0b80afb`
- Transform: daily precipitation only, exact decimal factor `1.2155576`,
  `0.1 mm` decimal `ROUND_HALF_UP`; all other parsed fields unchanged.
- Rows: `14,245` daily rows; `4,472` positive/changed precipitation rows.
- Totals: canonical `46,491.8 mm`; derived `56,519.1 mm`.
- Classification: `DEVELOPMENT_ONLY`.

Ran from `/home/workdir/openWEPP`:

```text
.venv/bin/python docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/materialize_snowbird_development_cli.py
.venv/bin/python docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/materialize_snowbird_development_cli.py --check
```

Both exited `0`. Future consumers copy the complete canonical fixture to a
fresh run directory and replace only the staged `p8.cli`; canonical source is
never overwritten. This lane is not precipitation truth, an observation,
calibration/default authority, independent physics validation, or transferable
evidence.
