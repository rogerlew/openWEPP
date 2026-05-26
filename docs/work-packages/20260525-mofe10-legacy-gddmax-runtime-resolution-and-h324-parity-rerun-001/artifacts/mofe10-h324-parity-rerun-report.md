# MOFE10 H324 Parity Rerun Report

Status: complete (lane rerun executed)
Evidence mode: Ran

Execution intent:
- Re-run carved-letter `H324` candidate lane after closing `gddmax=0` runtime
  sentinel handling and monthly climate projection requirements.

Command:
- `cargo run -p openwepp-runner --bin openwepp-cli-hill -- --run-dir /tmp/openwepp_mofe324_semantic_parity/runs --run-file p324.run --output-dir /tmp/openwepp_mofe324_semantic_parity/output_mofe10 --policy compat`

Result:
- Prior blocker (`gddmax` zero-domain rejection) is no longer the first failure.
- New downstream blocker observed:
  - `CLIHILL-E-011 runtime surface failure for management: HS-RUNTIME-E-050: PL projection field oratea at slot 1 crop-slot 1 is out of domain (0, allowed >0.0)`

Candidate output status:
- No candidate WAT/parquet outputs emitted under
  `/tmp/openwepp_mofe324_semantic_parity/output_mofe10`.

Semantic comparator status:
- Not executable in this rerun because no candidate WAT surface was produced.

Interpretation:
- MOFE10 objective to replicate legacy `gddmax` sentinel behavior is satisfied.
- Parity lane remains blocked at a new management runtime-seam guard
  (`oratea` domain handling).
