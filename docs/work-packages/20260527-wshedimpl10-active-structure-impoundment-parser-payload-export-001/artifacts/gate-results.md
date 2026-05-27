# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Required repository gates from `/workdir/openWEPP/AGENTS.md` were executed.
- WSHED10 scoped checks for active parser payload export and runtime fail-closed
  seam posture were executed.

## Ran
1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test --workspace`
   - result: pass
4. `cargo deny check`
   - result: pass; existing warnings observed for duplicate lockfile entries
     (`getrandom`, `hashbrown`, `twox-hash`) and unmatched allowed licenses
     (`ISC`, `Unicode-DFS-2016`).
5. `cargo test -p openwepp --test infile_watershed_impoundment_parser_contract strict_mode_parses_active_structure_payload_exports`
   - result: pass
6. `cargo test -p openwepp-watershed-orchestrator watershed_impoundment_runtime_seed_rejects_active_structure_projection_gap`
   - result: pass
