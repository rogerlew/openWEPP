# Public API Parity

Evidence class: Static + Ran.

## Public Surface

Static:

```text
rg -n "pub use hillslope::\{execute_hillslope_run, execute_hillslope_run_with_runtime_selection\}" \
  crates/openwepp-runner/src/lib.rs
```

Result:

```text
22:pub use hillslope::{execute_hillslope_run, execute_hillslope_run_with_runtime_selection};
```

The crate-level runner re-export remains unchanged.

## Entry Points

Static:

```text
rg -n "pub fn execute_hillslope_run|pub fn execute_hillslope_run_with_runtime_selection" \
  crates/openwepp-runner/src/hillslope
```

Result:

```text
crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:1108:pub fn execute_hillslope_run(
crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs:1119:pub fn execute_hillslope_run_with_runtime_selection(
```

Both public entrypoints remain in the `hillslope` module through `include!`;
only their source file changed.

## Module Wiring

Static:

```text
sed -n '1,12p' crates/openwepp-runner/src/hillslope/mod.rs
```

Result:

```rust
mod indexed_shadow_surface;
mod intake_lane_setup;
mod symbol_registry_audit;

include!("00_runner_intake_and_lane_setup.rs");
include!("01_scheduler_and_trace.rs");
include!("02_output_and_climate_helpers.rs");
include!("04_direct_publication.rs");
include!("05_runner_execution_and_outputs.rs");
include!("03_tests.rs");
```

Because the repo uses textual `include!` files for this module, private item
visibility and public exports remain module-equivalent after the split.

## Compile Evidence

Ran:

```text
cargo check -p openwepp-runner
```

Result: passed.

Ran:

```text
cargo test -p openwepp-runner --lib hillslope -- --nocapture
```

Result: passed, `107 passed; 0 failed`.
