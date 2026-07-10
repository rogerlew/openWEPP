# CRAP After

Ran:

```text
cargo llvm-cov -p openwepp-kernel-contract --lcov \
  --output-path /tmp/openwepp-cqr-b02-t01-final.lcov
cargo crap --workspace --lcov /tmp/openwepp-cqr-b02-t01-final.lcov --min 0 \
  --format json --output /tmp/openwepp-cqr-b02-t01-final-crap.json
```

All target rows are `<= 30`. The former rows are:

| Function | Before | After |
|---|---:|---:|
| `BoundaryValue::unit_label_for_variant` | 183.888 | 18.0 |
| `BoundaryValue::scalar_value` | 142.634 | 18.0 |
| `HillslopeKernelPhaseClass::phase_label` | 96.688 | 14.0 |
| `HillslopeConsumerAdapter::adapter_label` | 56.0 | 7.0 |

Target maximum after: `18.0`; target rows above `30`: `0`. The original public
methods are now one-call seams, so the required behavior-preserving decomposition
is production code rather than test-only metric reduction.

The focused LCOV naturally lacks other workspace files; `cargo crap` reports
unmatched-path warnings for those files. They do not affect the target rows.
