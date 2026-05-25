# MOFE08 H324 Parity Rerun Report

Status: complete (lane rerun executed)
Evidence mode: Ran

Execution intent:
- Re-run carved-letter `H324` candidate lane after CLIGEN `5.323` parser
  compatibility closure.

Command:
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe08 --policy compat`

Result:
- Climate parse blocker is resolved (no `unsupported datver 5.323` error).
- New downstream blocker observed:
  - `CLIHILL-E-011 runtime surface failure for soil: HS-RUNTIME-E-003: primary soil layer missing required theta_r_rosetta (thetdr)`

Candidate output status:
- No candidate WAT/parquet outputs emitted under `/tmp/openwepp_mofe324_semantic_parity/output_mofe08`.

Semantic comparator status:
- Not executable in this rerun because no candidate WAT surface was produced.

Interpretation:
- MOFE08 objective (CLIGEN compatibility + parity lane rerun) executed.
- Parity closure remains blocked by a post-climate runtime-surface contract gap
  outside scoped CLIGEN parser compatibility.
