# Gate Results

Evidence class: Ran

Focused gate:

- `cargo test --test clim06_frost_frozen_soil_kernel_contract`
- Exit code: 0
- Result: 46 passed; 0 failed; 0 ignored.

Required closure loop:

- `cargo fmt --check`
  - Exit code: 0
  - Result: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Exit code: 0
  - Result: passed.
- `cargo test --workspace`
  - Exit code: 0
  - Result: passed.
- `cargo deny check`
  - Exit code: 0
  - Result: `advisories ok, bans ok, licenses ok, sources ok`.

Note:

- `cargo fmt` was run before `cargo fmt --check` to format the moved Rust
  modules.
