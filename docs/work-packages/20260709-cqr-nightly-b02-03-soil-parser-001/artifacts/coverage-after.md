# Coverage After

Ran: delegated isolated final measurement against target source SHA-256
`c9c70bbfd9cfea3807d1e570f1018398d77eeb91e32a6fe44ffdaa4fe6065d32`.

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t03-target-final4-target \
  cargo llvm-cov clean -p openwepp-input-contract
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t03-target-final4-target \
  cargo llvm-cov -p openwepp-input-contract --lib --lcov \
  --output-path /tmp/openwepp-cqr-b02-t03-target-final4.lcov
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t03-target-final4-target \
  cargo llvm-cov report --json \
  --output-path /tmp/openwepp-cqr-b02-t03-target-final4-coverage.json
```

Clean and LCOV commands exited `0`; the instrumented library run passed `17/17`
tests. `cargo llvm-cov report` rejects `--lib` for its report subcommand, so the
documented equivalent report command above ran without `--lib` and exited `0`.

Excluding the `#[cfg(test)]` block beginning at line `1477`, production-only
coverage is `1085/1108` lines (97.924%) and `1434/1571` regions (91.279%).

Outputs: LCOV SHA-256
`e65a8c3a8320c47b8a4c27b1fab502a502ff74a9172f8d521abe77d6f91a37e9`; LLVM
JSON SHA-256
`1cf1ef4a518d0ee591ded9dce6cfb1b13540b9f42a6b3f8a18046c60fac5aa38`.
