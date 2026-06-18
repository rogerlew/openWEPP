# PERFARCH03 Floor Measurement

Evidence class: Ran.

Status: complete 2026-06-18.

Raw timing output: `perfarch03-floor-prototype.tsv`.

## Command

```bash
/usr/bin/time -f 'elapsed=%e max_rss_kb=%M' \
  -o /tmp/perfarch03-measure.time \
  docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/artifacts/perfarch03-floor-prototype/target/release/perfarch03-floor-prototype \
  > /tmp/perfarch03-measure.tsv
```

External time output:

```text
elapsed=70.38 max_rss_kb=3072
```

Budget constants from PERFIDX06:

| Budget | us/OFE-day |
|---|---:|
| Legacy H2637 no-UI | 38.650 |
| <=10x budget | 386.000 |
| <=5x budget | 193.000 |

Legacy H2637 no-UI endpoint: `9.120 s` over `235,961` OFE-days.

## Median results

Five release-binary repeats were recorded. Medians:

| Metric | Median us/OFE-day | Ratio vs legacy us/OFE-day | Projected H2637 seconds |
|---|---:|---:|---:|
| Current logical production kernel, same branch | 140.826054 | 3.643624x | 33.229457 |
| Array physics only | 0.074554 | 0.001929x | 0.017592 |
| Array dense output write only | 1.063708 | 0.027522x | 0.250994 |
| Array combined hot loop | 0.959423 | 0.024823x | 0.226386 |
| Boundary materialize once | 108.068963 | 2.796092x | 25.500061 |

The measured combined hot loop is the floor number for this branch. It is the
timed combination of branch physics plus dense output writes, not a sum of the
separately measured rows.

## Interpretation

The branch floor clears both target budgets by a wide margin:

- `0.959423 us/OFE-day` is below the `193 us/OFE-day` <=5x budget.
- `0.959423 us/OFE-day` is below the `386 us/OFE-day` <=10x budget.
- It is about `146.8x` faster than the current logical production kernel on the
  same synthetic branch (`140.826054 / 0.959423`).

The result also separates the transitional boundary cost from the hot loop:

- one-shot logical materialization costs `108.068963 us/OFE-day`;
- that cost is not part of the array-native floor and must not be repeated
  inside a production migrated hot path.

This measurement does not prove full H2637 runtime after migration, because it
does not port every phase and branch. It does prove the measured WB11 runoff
branch is not physics-floor-bound above the <=5x or <=10x viability budgets.
