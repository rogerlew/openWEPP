# Default-Disabled Regression Gate

Status: passed.

Threshold: H2637 default-disabled three-run median `<= 676.67 s`.

Ran:

1. Build `target/release/openwepp-cli-hill`.
2. Record binary hashes.
3. Run three default-disabled H2637 reps with direct-runtime and diagnostic
   environment variables unset.
4. Record unchanged known `MOFE01-MG-W-001` warning once per rep.
5. Compare protected PASS identity against the retained PERFDEEP07 baseline
   with DuckDB row equivalence.

Evidence:

```text
release_build  58.64  1127024
15d1fb25167a9efe850ae23be7cfcbc4101b743d03128fbaea3b453a1c6a0b5f  target/release/openwepp-cli-hill
28cdca1f1e11dd9eb70546b48998c1d4e49ca38b484ac9e787530367aed4bf19  target/release/openwepp-cli-hill.json
```

```text
r4pqz_h2637_default_rep1  645.54  227408
r4pqz_h2637_default_rep2  644.74  228796
r4pqz_h2637_default_rep3  640.28  229216
median                   644.74
threshold                <= 676.67
```

Each H2637 repetition emitted exactly one known warning:

```text
sidecar-warning: MOFE01-MG-W-001 EROD14 Wave-2 qin is seeded from water-transfer provenance only; true sediment-coupled qin/qout and particle-fraction handoff remains MOFE01 M-G follow-on scope.
```

PASS row equivalence:

```text
baseline_rows  candidate_rows  left_minus_right  right_minus_left
12419          12419           0                 0
```

Candidate PASS column count:

```text
17
```

Candidate protected output and manifest checksums:

```text
43ece8ca3c539a4dfa21b9f569786d5b05b4281e9e2ba45fb6e9fb087e06b9c2  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.pass.parquet
33ce1a661259be1565f27504f88384a925cf230e3355cd3cceaef4467e9b711a  /tmp/r4pqz-h2637/default/rep1/h2637_same/openwepp_hillslope_run_manifest.json
```

Note: the run manifest again recorded public output checksums under the
runfile-selected anchor output root
`/tmp/perfmig01-final/current/anchor/h2637_same/`; the package-local output
directory contained the run manifest.
