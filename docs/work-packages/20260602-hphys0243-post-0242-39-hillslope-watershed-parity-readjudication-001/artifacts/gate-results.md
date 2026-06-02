# HPHYS0243 Gate Results

Status: complete
Evidence mode: Ran

## Commands and Outcomes

- `cargo build -p openwepp-runner --bin openwepp-cli-hill --bin openwepp-cli-watershed`
  - Result: passed.
- `target/debug/openwepp-cli-hill` over
  `/tmp/hphys0243_20260602T042747Z/parity/runs/p{1..39}_openwepp.run`
  - Result: passed; `39/39`, all `rc=0`.
- `target/debug/openwepp-cli-watershed` over
  `/tmp/hphys0243_20260602T042747Z/parity/runs/pw0_openwepp.run`
  - Result: passed; `pw0 rc=0`.
- Semantic comparator loop over `H1..H39`
  - Result: passed execution; `39/39`, all `rc=0`, all `common_row_count=1461`.
- Hillslope summary aggregation
  - Result: passed.
- Watershed interchange comparison
  - Result: passed as investigation report; row-shape mismatch prevents
    promotable semantic parity claims for watershed outputs.
- `wctl doc-lint --path docs/work-packages/README.md`
  - Result: passed; 1 file validated, 0 errors, 0 warnings.
- `wctl doc-lint --path docs/work-packages/20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001`
  - Result: passed; 0 configured files validated.
- `git diff --check`
  - Result: passed.
