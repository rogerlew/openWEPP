# ARCH18 Disposition

Static: ARCH18 authority/convergence/provenance artifacts complete.
Ran: HBP convergence tests pass; required full gate commands executed.
Status: `HOLD`.

## Disposition Summary

- `CRF-006` closure evidence produced for:
  - explicit HBP parser-vs-bridge authority split,
  - cross-surface compatibility convergence constraints,
  - ADR-0012-compliant provenance SHA pinning.
- ARCH18-owned code/test changes compile and pass targeted validation.
- Full workspace gate triad (`fmt`, `clippy`, `test`) is currently blocked by
  concurrent ARCH17 runtime-input files outside ARCH18 HBP scope.

## Hold Reason

Correctness-over-completion policy applies:

- required full gate criteria are not all green yet,
- blocker is external to ARCH18-owned files but still affects package exit criteria.

## Hold Lift Condition

After ARCH17 stabilizes runtime-input surfaces, rerun:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo deny check`

If all pass, ARCH18 can be moved from `HOLD` to `GO-WITH-AMENDMENTS` (or
`GO`) without additional HBP-scope code changes.
