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

## Final CRAP Closure

Ran: exact-head measurement at `f1774586800525dd2339ac836349f252166d74ca`
and source SHA
`b7548382fc19ddaca3e2275cd4abeca4ed82abdc100ac38471ef1ee4727bf842`
passed 105/105 instrumented library tests in 455.79 seconds (469.84 seconds
wall). Matching CRAP exited 0 with zero target rows above 30 and a maximum of
exactly 30. Evidence root:
`/tmp/cqr-pre-heavy-final-yzFYI4`.

LCOV SHA-256:
`72a1f014d7ba0d7d3835f4adca9dc2585b03237ad5af1cb2a19f05c88f09ba38`.
CRAP JSON SHA-256:
`d30084ed51f2d6b29cdf5a92668acc2a48e679921c9e509f725f65d8783b6bb2`.

Ran: after direct characterization, exact head `3d6e8817` retained zero rows
above 30 and a maximum of exactly 30. Matching CRAP JSON SHA-256:
`673e514f8190382be7e00926e153ed0d733595ebc9e4aa380198e2744d3e6968`.

Ran: final source-bound measurement at `b1096a78` contains 111 target
production entries, zero entries above 30, and a maximum CRAP score of 17.0.
The matching CRAP JSON SHA-256 is
`df76b158a44df61feb4d752f4b9b3fd7909ee675f99f9d37a345779546e31bca`.

Ran: corrected final measurement at `68e9b747` retains zero production rows
above 30 and lowers the maximum to 17.0 (`validate_started_successor`). Matching
CRAP JSON SHA-256:
`100f9ff7d79e0c2137e3e591077bf2e76055224782a25b3c4bc4fcf41ac8adde`.
