# Characterization

Status: PASS
Evidence mode: Ran

The production source remained byte-identical to the fresh baseline while the
cover-first safety net was expanded. Its SHA-256 was
`1b9d8d124bf34a3d5f9189eb901a2ac87ff89d51076a58632c596ec878e47ac9`.
The final characterization test SHA-256 was
`9e8bfee0bea7753cf606659ffbb27ff566f59ab6a4a8790648c82cdd96a9f33f`.

The focused suite ran 15 tests and passed 15. It covers:

- all TW3 typed error codes, displays, and source behavior;
- missing PASS/WAT/optional paths, empty WAT, invalid Parquet, and writer
  failure mapping;
- required-column missing/type/null priority and all supported aliases;
- exhaustive PASS and WAT non-finite guards and nonnegative-domain guards;
- optional absent, all-null, mixed-null, non-finite, unmatched-key, and
  partial-presence behavior;
- zero runoff, aggregated-area overflow, and sediment zero-denominator logic;
- independent PASS runoff, WAT area/depth/volume, outlet lateral-flow, all-OFE
  QOFE, optional TSMF/QRain/QSnow, and deliberately unequal alias candidates.

Commands run before decomposition:

```text
cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract
cargo llvm-cov clean --workspace
cargo llvm-cov -p openwepp-runner --test totalwatsed3_cli_contract --lcov \
  --output-path /tmp/fq04-totalwatsed3-safety.lcov
cargo llvm-cov clean --workspace
cargo llvm-cov -p openwepp-runner --test totalwatsed3_cli_contract --json \
  --output-path /tmp/fq04-totalwatsed3-safety.json
```

The focused nextest run passed 15/15 in 1.534 seconds. LCOV capture passed
15/15 and took 24.40 seconds. JSON capture passed 15/15 and took 25.29
seconds. Raw evidence is retained as `lcov-safety-net.info` and
`coverage-safety-net.json`.
