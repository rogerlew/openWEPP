# HPHYS0204 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates (MEASURE-HP204-001)
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass.
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
3. `cargo test --workspace` -> pass.
4. `cargo deny check` -> pass (duplicate crate and
   `license-not-encountered` warnings only; exit code `0`).

## Cohort diagnostics execution (MEASURE-HP204-002)
Integrated diagnostics were recomputed from the latest 39-hillslope semantic
lane:
`/tmp/hphys0207_20260530T042607Z/parity/`.

1. Hillslope batch status:
   `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_batch_status.tsv`
   -> `39/39` rows with `rc=0`.
2. Semantic comparator status:
   `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic_status.tsv`
   -> `39/39` rows with `rc=0`.
3. Semantic summary:
   `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_semantic_summary.json`
   -> `total_hillslopes=39`, `semantic_pass_count=0`,
   `semantic_fail_count=39`, `total_common_rows=56979`.
4. Targeted residual tallies were recomputed directly from
   `semantic/H*.semantic.json` reports and recorded in package artifacts.
