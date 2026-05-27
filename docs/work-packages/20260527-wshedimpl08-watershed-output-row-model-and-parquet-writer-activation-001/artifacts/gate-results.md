# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-27

## Static
- Required repository gates and scoped watershed activation tests for
  WSHEDIMPL08.

## Ran
1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test --workspace`
   - result: pass
4. `cargo deny check`
   - result: pass with existing duplicate-crate and unmatched-license warnings
5. Scoped activation checks:
   - `cargo test -p openwepp-watershed-output` (pass)
   - `cargo test -p openwepp-runner --test watershed_cli_behavior_contract`
     (pass; WSHED03 vector active)
