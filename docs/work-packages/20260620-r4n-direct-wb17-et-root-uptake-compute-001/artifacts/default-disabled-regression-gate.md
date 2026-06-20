# Default-Disabled Regression Gate

Status: passed.

Threshold: H2637 default-disabled three-run median `<= 676.67 s`.

Ran:

1. Built `target/release/openwepp-cli-hill`.
2. Recorded binary hashes.
3. Ran three default-disabled H2637 reps with direct-runtime and diagnostic
   environment variables unset.
4. Recorded the unchanged known `MOFE01-MG-W-001` warning once per rep.
5. Compared protected PASS identity against the retained PERFDEEP07 baseline
   with DuckDB row equivalence.

Evidence:

```text
release_build  59.24  1121580
20baa2545fa584ffa83e06d2c1d6db55101c52c4eb3b50fa9474af9f55006389  target/release/openwepp-cli-hill
ff97ea6219cd2b5e2a600edb45e87e0e1026d8b60ea2583fd3e75b7a66ee9c92  target/release/openwepp-cli-hill.json
```

```text
r4n_h2637_default_rep1  643.84  228408
r4n_h2637_default_rep2  650.42  229204
r4n_h2637_default_rep3  649.22  228940
median                 649.22
threshold              <= 676.67
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

Note: the run manifest again recorded public output checksums under the
runfile-selected anchor output root
`/tmp/perfmig01-final/current/anchor/h2637_same/`; the package-local output
directory contained the run manifest.
