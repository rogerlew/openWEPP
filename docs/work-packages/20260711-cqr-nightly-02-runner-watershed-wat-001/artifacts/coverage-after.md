# Coverage After

Ran: the focused ten-test LLVM run produced production-only coverage of
`607/629` lines (`96.502%`) and `904/995` deduplicated regions (`90.854%`).

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-20260711-t02-target cargo llvm-cov \
  -p openwepp-runner --lib --lcov \
  --output-path /tmp/openwepp-cqr-20260711-t02-characterized.lcov \
  -- watershed_wat::tests
CARGO_TARGET_DIR=/tmp/openwepp-cqr-20260711-t02-target cargo llvm-cov \
  -p openwepp-runner --lib --json \
  --output-path /tmp/openwepp-cqr-20260711-t02-characterized.json \
  -- watershed_wat::tests
```

Both commands exited `0`, each with `10/10` tests passing.

- LCOV SHA-256:
  `03814b5ac87dbf144dd8488fdf8b94c1d9ab5e6316ef31725cfd9ff32ab906c0`.
- LLVM JSON SHA-256:
  `f26ef16a73fbf0f51f8ccc2c960a527f9ee10df42c6658f3f15f0c83ab4632f3`.

Raw files include the test module; the production-only boundary and method are
recorded in `coverage-closure.md`.
