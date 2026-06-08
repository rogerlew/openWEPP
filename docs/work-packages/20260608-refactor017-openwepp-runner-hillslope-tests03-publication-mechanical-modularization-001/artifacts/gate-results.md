# Gate Results

## Evidence mode
- Static: completed
- Ran: completed

## Required gates

- `cargo fmt --check`
  - Ran at 2026-06-08T21:19:14Z
  - Exit: 0
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Ran at 2026-06-08T21:19:14Z
  - Exit: 0
- `cargo test -p openwepp-runner --tests`
  - Ran at 2026-06-08T21:19:14Z
  - Exit: 0
  - Outcome: `73 passed` in publication-focused suite and all scoped package tests passed
- `cargo test --workspace`
  - Ran at 2026-06-08T21:19:14Z
  - Exit: 0
- `cargo deny check`
  - Ran at 2026-06-08T21:19:14Z
  - Exit: 0
  - Outcome: pass with pre-existing warnings (duplicate lock entries for `getrandom`, `hashbrown`, `twox-hash`; unmatched `ISC` and `Unicode-DFS-2016` allowances)

## Additional verification

- `wc -l` on modularized publication files confirms structural target and line-count bounds.
- `rg "^\s*#\[test\]" ...` counted 49 tests after split.
