# ADR-0021 Coverage Closure

Tier: science, because the target owns typed kernel request/writeback boundaries.

Ran focused coverage uses `cargo llvm-cov -p openwepp-kernel-contract` and
excludes all appended `#[cfg(test)]` source lines from the production denominator:

| Metric | Result | Science-tier threshold |
|---|---:|---:|
| Production line coverage | `603 / 603` (`100%`) | `>= 90%` |
| Production region coverage | `628 / 628` (`100%`) | `>= 90%` |

The extracted mapping helpers are fully characterized (`100%` function coverage):
`BoundaryValue::scalar_value`, `BoundaryValue::unit_label_for_variant`,
`HillslopeKernelPhaseClass::phase_label`, and
`HillslopeConsumerAdapter::adapter_label`.
JSON region inspection confirms every named target function is at or above the
`75%` floor. The dense-view-to-legacy-slot `or_else` closures are directly
covered, so no closure-body exclusion or instrumentation-only disposition is
needed. Test obligations bind `INV-KWRITEBACK-005` through request/payload
identity and `INV-USB-001`, `INV-USB-002`, and `INV-USB-005` through valid/
invalid typed constructors and canonical labels.
