# CRAP Checkpoint After First Extraction

Ran: delegated affected-module measurement at source SHA-256
`e946b005cdbb92ee71fb46e3bbe0b05e449f500db36b575bc08ece88135dc037`
(`5d2c0d0f`) used one LCOV traversal and its matching CRAP report:

```text
cargo llvm-cov -p openwepp-gate-planner --lib --ignore-run-fail --lcov \
  --output-path /tmp/cqr-pre-heavy-pbCbqS/openwepp-gate-planner.lcov
cargo crap --lcov /tmp/cqr-pre-heavy-pbCbqS/openwepp-gate-planner.lcov \
  --min 0 --format json \
  --output /tmp/cqr-pre-heavy-pbCbqS/openwepp-gate-planner-crap.json
```

LLVM coverage exited 0 with 104/104 library tests passing in 496.36 seconds;
CRAP exited 0. The target file measured 1,124/1,686 lines (66.67%). The current
eight over-threshold rows are:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `validate_audit` | 325 | 25 | 0% | 650 |
| `build_audit` | 92 | 22 | 0% | 506 |
| `validate_stage_receipt` | 1279 | 15 | 0% | 240 |
| `cheap_prerequisites` | 760 | 13 | 0% | 182 |
| `light_attempt_isolated` | 1020 | 13 | 0% | 182 |
| `inventory_and_arguments_are_exact` | 824 | 8 | 0% | 72 |
| `no_open_tooling_defect` | 1202 | 9 | 19.0476% | 51.9708 |
| `validate_audit_for_execution` | 396 | 6 | 0% | 42 |

The first extraction removed five prior actionable rows. Retained checksums:
LCOV `c82c50ca2b6d4f5c76c9c7626abe49448a686f3bece96fcc75b509b188d18f0b`;
CRAP JSON `f8850edf3828d313fd6e36e305888c87faa118e7797bf3b71f64aa87d0a778f8`.
The target-only LCOV warns about source files outside the measured package; that
is expected and does not affect the included `pre_heavy.rs` rows.
