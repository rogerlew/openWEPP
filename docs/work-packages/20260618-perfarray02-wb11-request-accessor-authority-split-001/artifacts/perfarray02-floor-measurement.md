# PERFARRAY02 Floor Measurement

Evidence: Ran.

## Basis

H2637 same-machine reference from PERFIDX06:

| Item | Value |
| --- | ---: |
| Legacy no-UI median | `9.12s` |
| Legacy per OFE-day | `38.65 us` |
| <=10x budget | `386 us/OFE-day` |
| 5x stretch | `193 us/OFE-day` |

PERFARRAY02 H2637 pilot:

```text
PERFARRAY02_TIMING phases=235961
seed_ns=174021089610
kernel_ns=113456941573
payload_ns=76660631969
evaluate_ns=1827594139
apply_ns=1026087882
export_ns=223565457787
reindex_ns=13163490
array_native_ns=192971255563
boundary_ns=397599710887
total_recorded_ns=590570966450
array_native_ns_per_phase=817809.958
boundary_ns_per_phase=1685022.995
```

`phase_count` is the H2637 runoff phase count and equals the H2637 OFE-day count
used for the floor.

## Result

| Segment | ns/OFE-day | us/OFE-day | Ratio vs legacy | Verdict |
| --- | ---: | ---: | ---: | --- |
| Array-native pilot segment | `817809.958` | `817.810` | `21.16x` | FAIL <=10x |
| Boundary seed/materialize | `1685022.995` | `1685.023` | `43.60x` | Reported separately |
| Recorded pilot segment total | `2502832.953` | `2502.833` | `64.76x` | Not the floor |

Wall-clock runs:

| Variant | Seconds | Max RSS KB |
| --- | ---: | ---: |
| default | 671.88 | 229108 |
| pilot | 1096.11 | 229920 |

## Interpretation

The scoped pilot proved the request/accessor seam and identity, but the measured
array-native segment is above the package stop condition: `817.810 us/OFE-day` is
more than `2.1x` the `386 us/OFE-day` <=10x budget and more than `4.2x` the `193
us/OFE-day` 5x stretch. Boundary seed/materialize is also dominant and would need
Stage-C-style authority work to remove.

Conclusion: NO-GO for ADR-0023 ratification from PERFARRAY02.
