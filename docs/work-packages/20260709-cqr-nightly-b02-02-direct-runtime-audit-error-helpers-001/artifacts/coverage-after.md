# Coverage After

Ran: delegated isolated final measurement against target SHA-256
`e84d57713eb5e374c1c86abffa2695fa37a81cf89158e8fbb9797453aeda1625`.

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t02-final-target \
  cargo llvm-cov clean -p openwepp-hillslope-orchestrator
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t02-final-target \
  cargo llvm-cov -p openwepp-hillslope-orchestrator --lib --lcov \
  --output-path /tmp/openwepp-cqr-b02-t02-final.lcov
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t02-final-target \
  cargo llvm-cov report --json \
  --output-path /tmp/openwepp-cqr-b02-t02-final-coverage.json
```

Every command exited `0`; the instrumented library run passed `339/339` tests
in `329.80s`. Raw target coverage is `547/556` lines (98.3813%) and `625/639`
regions (97.8091%). Excluding the `#[cfg(test)]` block starting at line `750`,
production-only coverage is `417/426` lines (97.8873%) and `528/542` regions
(97.4170%).

Outputs: LCOV `987,454` bytes, SHA-256
`5903a5d439e143c9a18618ce8f1f705ab41c3925f936057f8b1e1eda1611f814`; LLVM
JSON `2,790,641` bytes, SHA-256
`5b980dd64f8c24f045cbc943b615615d69227eeb3658d94a02f993ed91ede5a6`.
