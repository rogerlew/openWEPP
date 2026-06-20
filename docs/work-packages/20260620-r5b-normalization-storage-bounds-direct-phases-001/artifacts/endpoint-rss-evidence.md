# Endpoint / RSS Evidence

Static/Ran:

R5B records default-disabled H2637 timing and protected output comparison. No
public direct-only endpoint is introduced.

- Release build:
  - `release_build 58.74 1085960`
  - runner SHA:
    `063dae91e5a0acb5ce40cb22e6fd2bc7ad8f18e6ee26c78acddfb8a4e5cf74d9`
  - sidecar SHA:
    `001d45c475819762d5775570b7d7797434749c09b4e8e237274af530551aaf75`
- H2637 default-disabled:
  - rep1 `643.38 s`, `229332 KB`
  - rep2 `640.54 s`, `229036 KB`
  - rep3 `644.59 s`, `229020 KB`
  - median `643.38 s`
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
  - No public direct-only CLI endpoint exists in R5B; opt-in evidence remains
    runner API selection through `HillslopeRuntimeSelection::DirectSkeletonNoop`.

Verdict: PASS for R5B. No public output authority changed.
