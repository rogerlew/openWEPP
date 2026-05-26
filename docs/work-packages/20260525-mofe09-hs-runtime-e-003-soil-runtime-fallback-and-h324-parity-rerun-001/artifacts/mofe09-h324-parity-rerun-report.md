# MOFE09 H324 Parity Rerun Report

Status: complete (lane rerun executed)
Evidence mode: Ran

Execution intent:
- Re-run carved-letter `H324` candidate lane after closing runtime soil theta
  projection blocker (`HS-RUNTIME-E-003`).

Command:
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe09 --policy compat`

Result:
- Soil runtime seam blocker `HS-RUNTIME-E-003` is no longer observed.
- New downstream blocker observed:
  - `CLIHILL-E-011 runtime surface failure for management: HS-RUNTIME-E-050: PL projection field gddmax at slot 1 crop-slot 1 is out of domain (0, allowed >0.0)`

Candidate output status:
- No candidate WAT/parquet outputs emitted under
  `/tmp/openwepp_mofe324_semantic_parity/output_mofe09`.

Semantic comparator status:
- Not executable in this rerun because no candidate WAT surface was produced.

Interpretation:
- MOFE09 objective to resolve `HS-RUNTIME-E-003` is satisfied.
- Parity lane remains blocked at a new management runtime-seam guard.
