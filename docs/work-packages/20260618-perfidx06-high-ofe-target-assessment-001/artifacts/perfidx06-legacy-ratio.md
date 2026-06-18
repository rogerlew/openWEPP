# PERFIDX06 legacy ratio

Evidence: Ran.

## Method

Legacy comparator:

```text
(cd /tmp/perfidx06/legacy/<variant>/runs && \
  /usr/bin/time -f "<variant>\t<rep>\t%e\t%M" \
  /home/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill \
  < p2637.run)
```

The FARPOINT01 run directories were copied into `/tmp/perfidx06/legacy` before timing, so
the original fixture outputs were not overwritten.

Legacy binary:

| Item | Value |
| --- | --- |
| Binary | `/home/workdir/wepp-forest_260430_baseline/release/wepp_260430_hill` |
| SHA256 | `3b2fdd2b7a9e264b84f1e7b161dfb0730d49d3cb652218139efeb3ba17d7a160` |
| No-UI runfile | `/tmp/openwepp_farpoint01_h2637/without_ui/runs/p2637.run` |
| With-UI runfile | `/tmp/openwepp_farpoint01_h2637/with_ui/runs/p2637.run` |

## Legacy Timings

Raw file: `/tmp/perfidx06/artifacts/legacy-wallclock-times.tsv`.

| Variant | Rep | Seconds | Max RSS KB | RC |
| --- | ---: | ---: | ---: | ---: |
| `without_ui` | 1 | 9.12 | 4608 | 0 |
| `without_ui` | 2 | 9.12 | 4608 | 0 |
| `without_ui` | 3 | 9.02 | 4608 | 0 |
| `with_ui` | 1 | 11.63 | 4992 | 0 |
| `with_ui` | 2 | 11.41 | 4992 | 0 |
| `with_ui` | 3 | 11.54 | 4992 | 0 |

Summary:

| Variant | Mean s | Median s | Min s | Max s | Spread s |
| --- | ---: | ---: | ---: | ---: | ---: |
| `without_ui` | 9.0867 | 9.12 | 9.02 | 9.12 | 0.10 |
| `with_ui` | 11.5267 | 11.54 | 11.41 | 11.63 | 0.22 |

## Ratio

Primary ratio uses the no-UI H2637 fixture because that is the headline PERFHO01/PERFIDX04
measurement path.

| OpenWEPP no-UI s | Legacy no-UI median s | Ratio | Absolute gap s |
| ---: | ---: | ---: | ---: |
| 666.82 | 9.12 | 73.12x | 657.70 |

The with-UI reference is:

| OpenWEPP with-UI s | Legacy with-UI median s | Ratio | Absolute gap s |
| ---: | ---: | ---: | ---: |
| 667.44 | 11.54 | 57.84x | 655.90 |

Conclusion: the same-machine, same-fixture ratio confirms the pre-PERFIDX06 expectation.
The current endpoint is still roughly `58x-73x` slower than legacy, depending on variant,
with the no-UI primary ratio at `73.12x`.
