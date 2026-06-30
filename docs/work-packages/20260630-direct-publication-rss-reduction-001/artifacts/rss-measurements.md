# RSS Measurements

Evidence class: Ran

## H2637 and Short-Run Measurements

| Run | Outputs | State | Wall time | Max RSS KiB | Notes |
| --- | --- | --- | ---: | ---: | --- |
| H2637 | HBP/WAT/PASS/loss/plot | Prior package baseline | `1:09.18` | `1159672` | Full output, direct production |
| H2637 | HBP/loss only | Prior package baseline | `1:13.77` | `1159296` | Minimized output, direct production |
| `cli01` | fixture default | Prior package baseline | `0:00.09` | `19584` | Short run |
| H2637 | HBP/WAT/PASS/loss/plot | Stage A jemalloc rerun | `1:14.32` | `1153536` | Same scale as baseline |
| H2637 | HBP/WAT/PASS/loss/plot | False-start clear-after-alloc | `1:09.23` | `1110524` | Allocation peak already paid |
| H2637 | HBP/loss only | False-start clear-after-alloc | `1:08.07` | `1110912` | Allocation peak already paid |
| H2637 | HBP/WAT/PASS/loss/plot | Final post-fix | `1:07.31` | `316212` | Byte-identical outputs |
| H2637 | HBP/loss only | Final post-fix | `1:07.03` | `184644` | Byte-identical HBP/loss |
| `cli01` | fixture default | Final post-fix | `0:00.09` | `19584` | Still direct production |

## Reduction

| Comparison | RSS KiB before | RSS KiB after | Delta KiB | Reduction |
| --- | ---: | ---: | ---: | ---: |
| H2637 full output | `1159672` | `316212` | `843460` | `72.7%` |
| H2637 HBP/loss-only | `1159296` | `184644` | `974652` | `84.1%` |
| `cli01` | `19584` | `19584` | `0` | `0.0%` |

## Slope Status

The RSS-vs-run-length slope is materially reduced but not flat:

- H2637 HBP/loss-only remains `184644 KiB`.
- `cli01` remains `19584 KiB`.
- The remaining gap is consistent with whole-run retained
  `DirectPublicationDayRow` values plus allocator overhead.

Full-output H2637 remains higher than minimized H2637 because WAT/PASS
projection vectors and parquet/Arrow writer buffers are still constructed for
the whole requested run.
