# PERFIDX05 Gate Results

Ran:
- `cargo fmt --check` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test --workspace` -> pass
- `cargo deny check` -> pass
- `git diff --check` -> pass
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` -> pass

Gate logs:
- `/tmp/perfidx05/artifacts/gates/cargo-fmt-check.log`
- `/tmp/perfidx05/artifacts/gates/cargo-clippy-workspace-all-targets.log`
- `/tmp/perfidx05/artifacts/gates/cargo-test-workspace.log`
- `/tmp/perfidx05/artifacts/gates/cargo-deny-check.log`
- `/tmp/perfidx05/artifacts/gates/git-diff-check.log`
- `/tmp/perfidx05/artifacts/gates/cargo-build-release-final.log`

Binary validation:
- Post-clippy incremental release rebuild produced the same binary SHA as the timed binary:
  `4eebabb5f4679b000516177271c996483ef639ca76697093797370685ec1c087`.
