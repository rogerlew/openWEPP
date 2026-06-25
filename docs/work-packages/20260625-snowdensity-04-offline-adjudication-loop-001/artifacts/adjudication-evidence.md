# Adjudication Evidence

Ran:

```bash
.venv/bin/python tools/snowfreeze_observed/physics_bulk_adjudication.py \
  --observations-dir tests/fixtures/snotel_observed/observations \
  --output-dir target/snowdensity04_adjudication \
  --snowbench-binary target/debug/openwepp-snowbench \
  --h-comparator-json target/snowfrost_fidelity_h/three_way_comparison.json
```

Committed compact artifacts:

- `artifacts/physics-bulk-adjudication.json`
- `artifacts/physics-bulk-adjudication.md`

Full run outputs remain under:

- `target/snowdensity04_adjudication/variants/candidate_v1/`
- `target/snowdensity04_adjudication/variants/slow_melt_v1/`
- `target/snowdensity04_adjudication/variants/dense_slow_melt_v1/`
- `target/snowdensity04_adjudication/variants/cold_dense_slow_melt_v1/`

## Result

| Profile | Robust fail | Robust ordinal score | Robust counts |
|---|---:|---:|---|
| `openwepp_as_built` | `9` | `84` | `fail=9`, `marginal=8`, `pass=8`, `strong=20`, `unavailable=15` |
| `legacy_as_built` | `9` | `84` | `fail=9`, `marginal=8`, `pass=8`, `strong=20`, `unavailable=15` |
| `candidate_v1` | `24` | `34` | `fail=24`, `marginal=13`, `pass=3`, `strong=5`, `unavailable=15` |
| `slow_melt_v1` | `6` | `95` | `fail=6`, `marginal=8`, `pass=3`, `strong=27`, `unavailable=16` |
| `dense_slow_melt_v1` | `6` | `102` | `fail=6`, `marginal=3`, `pass=6`, `strong=29`, `unavailable=16` |
| `cold_dense_slow_melt_v1` | `15` | `65` | `fail=15`, `marginal=4`, `pass=5`, `strong=17`, `unavailable=19` |

`dense_slow_melt_v1` is the best profile by robust failure count and robust
ordinal score. It beats both openWEPP and legacy as-built under the package
rule. Cell comparisons show `17` better / `24` equal / `3` worse robust cells
against openWEPP as-built and `18` better / `22` equal / `4` worse robust cells
against legacy as-built.

## Disposition

`COMPLETE-PROMOTION-CANDIDATE`.

This means SNOWDENSITY-05 is authorized to scaffold runtime opt-in coupling for
`snow_model = physics_bulk` using `dense_slow_melt_v1` as the candidate default
inside the opt-in lane. It does not authorize default activation, deletion of
legacy WEPP snow, or frost attribution.
