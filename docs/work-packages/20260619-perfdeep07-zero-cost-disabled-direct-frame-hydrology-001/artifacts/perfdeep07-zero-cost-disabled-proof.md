# PERFDEEP07 Zero-Cost Disabled Proof

Status: queued.
Evidence mode: not-run.

## Proof Obligation

Prove statically that when all PERFDEEP opt-ins are disabled, execution does not
construct or resolve:

- compact dense request/view state;
- indexed shadow surfaces;
- direct-frame seed/shadow state;
- hot symbol tables for dense/direct-frame paths;
- direct-frame publication projection shadow machinery;
- dense/logical refresh or flush adapters.

The proof must name source files, functions, guard conditions, and any remaining
allowed diagnostic/test-only use.
