# Endpoint / RSS Evidence

Static plan:

R5A must record endpoint/RSS evidence because it changes direct executor
lifecycle. Public outputs remain compatibility-authoritative.

Required:

- release build timing/RSS and binary SHA;
- three default-disabled H2637 reps;
- protected output identity or row-equivalence evidence if outputs vary;
- opt-in direct skeleton evidence from runner tests until a public direct-only
  CLI endpoint exists.

Ran:

- Release build:
  - `release_build 57.93 1111320`
  - runner SHA:
    `c1c180b2fb3049288d60b28fc6f0bee7fbf44f3da453ec97414dc1717d0705a0`
  - sidecar SHA:
    `28cdca1f1e11dd9eb70546b48998c1d4e49ca38b484ac9e787530367aed4bf19`

- H2637 default-disabled:
  - rep1 `643.98 s`, `228784 KB`
  - rep2 `647.95 s`, `228604 KB`
  - rep3 `643.45 s`, `229304 KB`
  - median `643.98 s`

- Protected output comparison against retained PERFDEEP07 baseline:
  - `H2637.hbp`: byte-identical.
  - `H2637.wat.parquet`: byte-identical.
  - `H2637.pass.parquet`: parquet bytes differ, DuckDB row equivalence PASS:
    `baseline_rows=12419`, `candidate_rows=12419`, `left_minus_right=0`,
    `right_minus_left=0`; candidate column count `17`.
  - `H2637.loss.json`: differs only by `run_name`; normalized `jq -S
    'del(.run_name)'` diff is empty.
  - `H2637.plot.parquet`: this path contains the existing text placeholder,
    not a parquet payload. It differs only by `run_name`; normalized
    `sed '/^run_name=/d'` diff is empty.

- Explicit opt-in endpoint evidence:
  - `cargo test -p openwepp-runner r2a_ -- --nocapture` PASS.
  - No public direct-only CLI endpoint exists in R5A; opt-in evidence remains
    runner API selection through `HillslopeRuntimeSelection::DirectSkeletonNoop`.

Verdict: PASS for R5A. No public output authority changed.
