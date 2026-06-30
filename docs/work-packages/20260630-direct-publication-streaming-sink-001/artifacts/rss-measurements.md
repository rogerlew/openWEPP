# RSS Measurements

Evidence class: Ran

Measurements used `target/release/openwepp-cli-hill` with `/usr/bin/time -v`.

| Case | Days | OFEs | Publication rows | Outputs | Max RSS | Elapsed | Status |
| --- | ---: | ---: | ---: | --- | ---: | ---: | --- |
| Prior held H2637 full | `12419` | `19` | `235961` | HBP/loss/plot/WAT/PASS | `316212 KiB` | not rerun here | prior package |
| Prior held H2637 required-only | `12419` | `19` | `235961` | HBP/loss | `184644 KiB` | not rerun here | prior package |
| Streaming H2637 full | `12419` | `19` | `235961` | HBP/loss/plot/WAT/PASS | `112652 KiB` | `1:08.58` | pass |
| Streaming H2637 required-only | `12419` | `19` | `235961` | HBP/loss | `52228 KiB` | `1:07.97` | pass |
| Streaming W9 longer-day fixture | `16437` | `1` | `16437` | HBP/loss/plot/WAT/PASS | `47856 KiB` | `0:03.32` | pass |
| Streaming cli01 short fixture | `2` | `1` | `2` | HBP/loss/plot/WAT | `20736 KiB` | `0:00.09` | pass |

Interpretation:

- The retained-row slope is flattened for the required-output endpoint:
  H2637's `235961` rows now peak at `52228 KiB`, close to the W9
  `16437`-row fixture's `47856 KiB`.
- Full-output H2637 remains higher than required-only because WAT/PASS parquet
  emission and OS/file buffers are real output work, but it drops another
  `203560 KiB` from the prior package and no longer retains whole-run WAT/PASS
  row vectors.
- A synthetic 2x H2637 climate was attempted but rejected as measurement
  evidence: it fails at simulated year `69` because H2637 management/growth
  authority spans `34` years. That failure is an input-domain guard, not a
  publication failure.
