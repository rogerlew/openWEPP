# R4B Disposition

Status: complete.
Evidence mode: Ran.

Verdict:

`COMPLETE-R4B-DIRECT-STORAGE-RECONCILIATION-CONSUMER-SPAN`

Summary:

R4B implemented the direct WB12 storage-reconciliation consumer span. It
consumes R4A direct `q_runoff_m`, reconciles storage from explicit direct
operands, mutates direct state, produces downstream operands, and
shadow-projects storage and closure residual. It does not publish direct
storage, alter scheduler behavior, edit compatibility runtime APIs, change
output schemas, or activate direct runtime by default.

Closure basis:

- `SC-WATBAL-001` authority and operand lineage passed before Rust edits.
- Focused R4B tests passed.
- Runner default-disabled and opt-in direct-counter tests passed.
- Full Rust gates passed.
- No-compatibility proof passed.
- Default-disabled H2637 gate passed: median `641.14 s <= 676.67 s`.
- Protected output identity passed.
- PASS parquet row equivalence passed.
- Dual local review and verification artifacts found no blocking issues.

Accepted non-blocking finding:

- `direct_runtime.rs` is now 2101 lines. This is a WARN, not a closure blocker.
  A direct-runtime split or extraction should be preferred before another large
  direct span.
