# Gate Results

Ran: required ARCH18 gate commands executed in `/home/workdir/openWEPP`.
Status: hold (parallel-package blocker).

1. `cargo fmt --check`
- Result: fail
- Blocker: formatting drift in concurrent ARCH17 runtime-input files:
  - `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

2. `cargo clippy --workspace --all-targets -- -D warnings`
- Result: fail
- Blocker: concurrent ARCH17 runtime-input compile/clippy errors, including:
  - missing `BoundarySymbol` imports in `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
  - key-lookup type mismatches in `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - numeric-cast lint failures in `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

3. `cargo test --workspace`
- Result: fail
- Blocker: same concurrent ARCH17 runtime-input compile errors above.

4. `cargo deny check`
- Result: pass
- Notes: non-failing `license-not-encountered` warnings only;
  terminal status `advisories ok, bans ok, licenses ok, sources ok`.

## In-Scope Supplemental Validation (ARCH18)

These commands passed and validate ARCH18-owned HBP scope changes directly:

1. `cargo test --test infile_hbp_parser_contract`
- Result: pass (`14 passed; 0 failed`)

2. `cargo test --manifest-path crates/openwepp-legacy-bridge/Cargo.toml`
- Result: pass (`13 passed; 0 failed`)

3. `cargo test --manifest-path crates/openwepp-input-contract/Cargo.toml`
- Result: pass

4. `cargo fmt --manifest-path crates/openwepp-input-contract/Cargo.toml --check`
- Result: pass

5. `cargo fmt --manifest-path crates/openwepp-legacy-bridge/Cargo.toml --check`
- Result: pass

6. `cargo clippy --manifest-path crates/openwepp-input-contract/Cargo.toml --all-targets -- -D warnings`
- Result: pass

7. `cargo clippy --manifest-path crates/openwepp-legacy-bridge/Cargo.toml --all-targets -- -D warnings`
- Result: pass
