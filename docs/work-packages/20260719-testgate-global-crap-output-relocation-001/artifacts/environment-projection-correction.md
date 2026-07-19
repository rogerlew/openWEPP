# Environment Projection Correction

Evidence class: `Static` and `Ran`

`environment_record` now loads the validated gate registry, unions every
definition's `environment_allowlist`, and projects process variables against
that set before hashing. The four current keys are `PATH`, `CARGO_HOME`,
`RUSTUP_HOME`, and `RUSTUP_TOOLCHAIN`; the implementation derives rather than
hard-codes them.

Compiler, target, platform, feature, runner-image, Cargo-configuration, and
Git-local-configuration identities are unchanged. Undeclared variables are not
recorded or hashed. A declared variable with non-UTF-8 content continues to fail
closed.

Focused evidence:

- `cargo fmt --check`: PASS.
- `cargo nextest run -p openwepp-gate-planner -E
  'test(environment_projection)'`: 2/2 PASS, 65 filtered; 0.142 seconds after a
  7.58-second build.
- `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`: PASS in
  3.90 seconds.
- Unit regression: changes to `_` and an undeclared secret leave the projection
  identical; a changed declared `PATH` changes it.
- Policy regression: the derived union equals the four current registry keys and
  excludes `_`.
- `git diff --check`: PASS.

Line-count governance: `execution_context.rs` is 249 lines and the previously
changed integration test is 558 lines; both are below the 2,000-line warning
threshold. No kernel/process Rust changed.
