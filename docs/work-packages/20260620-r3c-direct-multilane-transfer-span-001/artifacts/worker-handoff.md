# Worker Handoff

Status: complete.
Evidence mode: Static.

R3C is closed. The direct runtime now has three proved spans:

- R3A: lane/day transfer-input accounting;
- R3B: lane/day water-ledger diagnostic chain;
- R3C: run-level multi-lane transfer/topology propagation.

The next package should choose between:

- **R4A first direct hydrology-process span** if canonical `SC-*` authority and
  identity evidence can be scoped tightly enough for one process branch; or
- **R3D bounded pre-process span** only if another non-process direct propagation
  surface is required before touching process math.

Recommended route: R4A, narrowly scoped to one hydrology-process span with no
publication cutover. R3A-R3C now cover the core pre-process runtime mechanics:
inputs, direct compute, direct state mutation, downstream operands, shadow
projection, per-lane and run-level aggregation, phase-span identity, and
no-compatibility proof. Additional R3 work should have a specific blocker it
removes.

Carry forward gates:

- no default activation;
- no publication/schema cutover unless explicitly authorized;
- direct-runtime forbidden-token source scan;
- scheduler no-diff check;
- explicit opt-in direct counters positive;
- default-disabled direct counters zero;
- default-disabled H2637 median `<= 676.67 s`;
- full Rust gates: fmt, clippy, test workspace, deny.

Carry forward caution:

R4A must not use diagnostic R3 ledgers as science authority. Process migration
needs canonical science-contract provenance and independent closure evidence for
the selected hydrology process.
