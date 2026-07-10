# Coverage After

Ran: delegated isolated final measurement against source SHA-256
`45da7d807835f33d54b51bc57cb7dd89d7eeb9c51962110e42fbed9f601e04d2`.
The instrumented public contract suite passed `6/6` tests.

```text
CARGO_TARGET_DIR=/tmp/openwepp-cqr-b02-t05-final-target \
  cargo llvm-cov -p openwepp-runner --bin openwepp-cli-totalwatsed3 \
  --test totalwatsed3_cli_contract --lcov \
  --output-path /tmp/openwepp-cqr-b02-t05-final.lcov
```

Production coverage is `186/197` direct DA lines (94.416%). The LLVM report
contains duplicate compilations of the same source; deduplicating identical
source-region coordinates by maximum counter yields the valid target result
`285/305` regions (93.443%). All primary functions clear 75% regions; the
lowest is `validate_required_options` at `20/25` (80%).

LCOV SHA-256: `30dedd372f0f7842d846a700af259aa80fd52b30f578cc80e5f84cd0b5bdc53a`.
LLVM JSON SHA-256:
`734593c6f0131a983497ee496597888f7f31873172abebae56c73455b7eeff46`.
