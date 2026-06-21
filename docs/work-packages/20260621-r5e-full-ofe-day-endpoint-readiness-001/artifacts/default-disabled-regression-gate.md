# Default-Disabled Regression Gate

Status: complete.
Evidence mode: Ran.

Gate:

- run at least three clean H2637 default-disabled reps;
- record min, median, max, and RSS;
- PASS requires median `<= 676.67 s`;
- protected output identity/equivalence must pass.

Release build:

```text
/usr/bin/time -f 'release_build\t%e\t%M' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: PASS.

```text
release_build  59.22  1104420
79ad882de39a03bbc91217968b5ace1330665badcb6f8abdcebffd5e083f9390  target/release/openwepp-cli-hill
5bd9bdeb167d03e4b6532530d09fc5f9f5e7eb75d054bfd91e466a5a18ec5f94  target/release/openwepp-cli-hill.json
```

Default-disabled H2637 reps with direct-runtime and diagnostic env vars unset:

```text
r5e_h2637_default_rep1  641.37  228724
r5e_h2637_default_rep2  642.02  227956
r5e_h2637_default_rep3  635.47  228940
```

Result:

- min: `635.47 s`
- median: `641.37 s`
- max: `642.02 s`
- max RSS observed: `228940 KiB`
- timing threshold: `<= 676.67 s`
- verdict: PASS

Each run emitted the known `MOFE01-MG-W-001` warning. Classification and text
were not changed by R5E.

Protected output comparison passed. HBP, WAT, loss, and plot hashes matched
between the opt-in direct-skeleton and default-disabled comparison outputs.
PASS parquet bytes differed, so DuckDB row equivalence was run and passed with
`12419` rows and zero bidirectional differences.
