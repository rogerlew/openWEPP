# Implementation Test Evidence

Status: HOLD.
Evidence mode: Static/Ran.

## Candidate Implementation

Temporary candidate:

- cached disabled PERFDEEP02 roundtrip path lookup;
- short-circuited inactive indexed-shadow observe/validate hooks;
- initially tried a scheduler flag-hoist micro-change, then reverted it before
  timing because `scheduler.rs` is over 3000 lines.

The timed runner hook-cache candidate was rejected and reverted. No production
Rust edit remains.

## Commands Run

- `cargo fmt --check`: pass.
- `cargo test -p openwepp-runner`: pass.
- `cargo test -p openwepp-hillslope-orchestrator writeback`: pass.
- `cargo test -p openwepp-kernel-contract indexed_request_without_dense_slots_keeps_dense_surface_absent`: pass.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: pass.
- H2637 default-disabled candidate run: pass execution, fail timing
  (`691.93 s`, RSS `229444 KB`).
- `git diff --check`: pass before artifact edits.

## Not Run

Not run because the package is `HOLD`, not `READY-FOR-R2`:

- three-run H2637 median gate;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo test --workspace`;
- `cargo deny check`.
