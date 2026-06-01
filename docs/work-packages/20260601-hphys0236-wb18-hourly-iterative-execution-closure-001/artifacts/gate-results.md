# HPHYS0236 Gate Results

Status: completed  
Evidence mode: Ran

## Commands

1. `cargo build -p openwepp-runner --bin openwepp-cli-hill`
2. `cargo fmt --check`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo test --workspace`
5. `cargo deny check`
6. 39-hillslope rerun via `target/debug/openwepp-cli-hill` over  
   `/tmp/hphys0236_20260601T230600Z/parity/runs/p{1..39}_openwepp.run`
7. Semantic comparator loop (`H1..H39`) via  
   `/tmp/hphys0233_20260601T211306Z/.venv/bin/python` and
   `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py` with
   `--tolerance-config tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
   and `--candidate-year-offset 2012`.
8. Semantic summary aggregation script over  
   `/tmp/hphys0236_20260601T230600Z/parity/reports/semantic/H*.semantic.json`.

## Outcomes

1. pass  
2. pass  
3. pass  
4. pass  
5. pass (`duplicate` and `license-not-encountered` warnings only; exit success)  
6. pass (`39/39`, all `rc=0`)  
7. pass (`39/39`, all `rc=0`)  
8. pass (summary artifacts generated)

## Gate Decision

All required execution gates passed. Stream disposition remains `HOLD` for
physics-adjudication reasons documented in `hphys0236_disposition.md`, not
because of gate failure.
