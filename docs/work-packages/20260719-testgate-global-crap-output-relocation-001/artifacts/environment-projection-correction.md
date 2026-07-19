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
  'test(environment_projection) |
  test(declared_non_utf8_environment_value_fails_closed)'`: 3/3 PASS, 65
  filtered; 0.141 seconds after a 2.42-second incremental build.
- `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`: PASS in
  2.58 seconds on the final focused diff.
- Unit regression: changes to `_` and an undeclared secret leave the projection
  identical; a changed declared `PATH` changes it.
- Policy regression: the derived union equals the four current registry keys and
  excludes `_`.
- Declared non-UTF-8 value regression: typed `GATE-ENVIRONMENT-NONUTF8` PASS.
- `git diff --check`: PASS.

Line-count governance: `execution_context.rs` is 263 lines and the previously
changed integration test is 558 lines; both are below the 2,000-line warning
threshold. No kernel/process Rust changed.
