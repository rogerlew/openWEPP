# Rust Line-Count Governance

Static: current touched Rust line counts are:

- `executor.rs`: 2,294 lines — WARN; below the 3,000-line closure block.
- `verifier.rs`: 2,305 lines — WARN; below the 3,000-line closure block.
- `planner.rs`: 2,698 lines — WARN; below the 3,000-line closure block.
- `main.rs`: 576 lines — below WARN threshold.
- `repository.rs`: 1,268 lines — below WARN threshold.

Disposition: all three WARNs are accepted for this package because no file
reaches the blocking threshold and the added lines are predominantly closed
wire-contract handling plus adversarial fixtures. Split intent: the next
package that changes node construction or receipt execution must first extract
planner inventory/quality projection and executor/verifier test fixtures into
dedicated modules before adding another responsibility.
