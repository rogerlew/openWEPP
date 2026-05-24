# PL14S Gate Results

Status: `completed-with-hold`
Evidence mode: `Static + Ran`

## Static
- Required closeout gates for code-touching PL14S execution:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`
- Semantic replay acceptance posture:
  - comparator execution success is required,
  - semantic parity pass is evaluated from report evidence and may retain hold.

## Ran
- `cargo fmt --check`
  - pass
- `cargo clippy --workspace --all-targets -- -D warnings`
  - pass
- `cargo test --workspace`
  - pass
- `cargo deny check`
  - pass (`advisories ok, bans ok, licenses ok, sources ok`)
  - non-blocking warnings observed:
    - duplicate crate versions (`getrandom`, `hashbrown`, `twox-hash`)
    - unmatched allowlist license entries (`ISC`, `Unicode-DFS-2016`)
- PL14S semantic comparator gate
  - execution pass (`returncode=0`)
  - semantic parity verdict: **failed** (`semantic_pass=false`, row-set non-overlap)
  - disposition note: HOLD retained because candidate emission currently reflects
    first-day synthesized WB13-style output and not full daily watbal scheduler
    execution in runner/CLI.
