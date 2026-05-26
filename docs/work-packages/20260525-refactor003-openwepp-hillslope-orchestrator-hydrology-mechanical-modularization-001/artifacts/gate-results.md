# Gate Results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Required REFACTOR003 exit gates were executed.
- All required gates passed.

## Ran
1. `cargo fmt --check`
   - result: pass
2. `cargo clippy --workspace --all-targets -- -D warnings`
   - result: pass
3. `cargo test -p openwepp-hillslope-orchestrator`
   - result: pass
4. `cargo test --workspace`
   - result: pass
5. `cargo deny check`
   - result: pass (warnings only; advisories/bans/licenses/sources all ok)

Warning capture (`cargo deny check`):
- duplicate lock entries:
  - `getrandom`
  - `hashbrown`
  - `twox-hash`
- unmatched license allowlist entries:
  - `ISC`
  - `Unicode-DFS-2016`
