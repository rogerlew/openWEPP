# PERFARRAY02 Verification B

Evidence: Static + Ran.

## Recomputed Floor

Inputs:

- H2637 phases: `235961`
- array-native ns: `192971255563`
- legacy per OFE-day: `38.65 us`
- <=10x budget: `386 us/OFE-day`

Reconstruction:

```text
192971255563 ns / 235961 = 817809.958 ns/OFE-day
817809.958 ns = 817.810 us/OFE-day
817.810 / 38.65 = 21.16x legacy
```

Boundary:

```text
397599710887 ns / 235961 = 1685022.995 ns/OFE-day
1685022.995 ns = 1685.023 us/OFE-day
```

Result: independent reconstruction matches `perfarray02-floor-measurement.md`.

## Gate Non-Deferral

No current-scope gate is deferred:

- bit identity was run for OFE5 and H2637;
- floor was run on H2637;
- structural proof has current static and perf evidence;
- full Rust closure gates were run.

The package can close NO-GO.
