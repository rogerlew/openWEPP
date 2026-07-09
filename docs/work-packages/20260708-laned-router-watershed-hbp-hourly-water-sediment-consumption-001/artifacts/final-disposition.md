# Final Disposition

Status: `EXECUTED-COMPLETE-WATERSHED-HBP-HOURLY-CONSUMPTION`
Evidence mode: `Static` plus `Ran`

Outcome:

- M-T3 is complete.
- The production watershed/channel consumer now distinguishes all-hourly
  inlets from all-no-hourly inlets and fails closed for partial, malformed, or
  mixed hourly authority.
- `SC-ROUTE-001` rev 49 authorizes the tightened inlet rule; rev 50 closes the
  Binding Exposure Index profile gate.
- A production CLI HBP schema-1.1 consumer test proves that equal daily
  runoff/sediment totals with different hourly distributions change watershed
  consumer outputs.

Final gates:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo nextest run --workspace --profile full`: PASS, 1479 passed, 3 skipped.
- `cargo deny check`: PASS.
- Scoped Markdown lint, diff check, contract BEI/unit checks, and unit registry:
  PASS.

Residual limitation:

- Downstream dependency nodes still fail closed when side hillslope contributors
  carry hourly surfaces and upstream channel nodes do not yet carry channel-hourly
  surfaces. That is intentional M-T3 behavior, not a closure blocker.
