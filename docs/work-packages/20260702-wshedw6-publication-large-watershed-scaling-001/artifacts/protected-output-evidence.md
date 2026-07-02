# Protected Output Evidence

Status: `passed`

Evidence mode: `Ran:` full fixture row/content identity checks.

## Identity Method

Each required watershed parquet output was read with `pyarrow.parquet`.
Acceptance required both:

- identical Arrow schema;
- identical table row content in existing row order.

No contract-governed deltas were accepted or needed.

## Onshore-Xenophobia

Fixture: `tests/fixtures/watershed/onshore-xenophobia/`.

Comparison: `--jobs 1` vs `--jobs 48`.

Result: `PASS`, all `14` required watershed parquet outputs matched.

Evidence:

- `/tmp/wshedw6_onshore_scaling_rerun/jobs1-full/out/interchange`
- `/tmp/wshedw6_onshore_scaling_rerun/jobs48-full/out/interchange`
- `artifacts/scaling/onshore-xenophobia-scaling-summary.json`

## Carnivorous-Adobo

Fixture: `tests/fixtures/watershed/carnivorous-adobo/`.

Comparison: `--jobs 1` vs `--jobs 32`.

Result: `PASS`, all `14` required watershed parquet outputs matched.

Evidence:

- `/tmp/wshedw6_carnivorous_scaling_rerun/jobs1-full/out/interchange`
- `/tmp/wshedw6_carnivorous_scaling_rerun/jobs32-full/out/interchange`
- `artifacts/scaling/carnivorous-adobo-scaling-summary.json`
