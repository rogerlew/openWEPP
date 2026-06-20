# R3B Artifacts

Status: complete.
Evidence mode: Static + Ran.

R3B implemented the direct water-ledger span:
`RunoffReconciliation -> StorageReconciliation -> ClosureDiagnostics`.

Key evidence:

- direct span consumes R3A input-accounting state plus direct water and
  publication fields;
- direct compute mutates `DirectWaterLedgerState`, produces ledger downstream
  operands, and shadow-projects a signed diagnostic residual;
- no WB11/WB12/WB14/WB17/WB18/WB19 process equation migration;
- no output writer/schema/scheduler/publication cutover;
- focused R3B/R2A/runner tests pass;
- full Rust closure gates pass;
- default-disabled H2637 reps: `640.67/643.05/639.21 s`, median `640.67 s`
  against threshold `<= 676.67 s`;
- protected identity passes.
