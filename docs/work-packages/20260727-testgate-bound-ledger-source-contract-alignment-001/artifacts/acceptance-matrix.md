# Acceptance Matrix

Status: `SCAFFOLD BASE BINDING REQUIRED`

Evidence class: `Static + Ran`

| ID | Obligation | Evidence |
|---|---|---|
| BL-01 | Exact full-profile failure retained | predecessor `implementation-gates.md` |
| BL-02 | Stale assertion replaced exactly | implementation diff |
| BL-03 | Bound-text and retained-read calls required | focused source-contract test |
| BL-04 | Production unchanged | exact Rust path reconciliation |
| BL-05 | Full regression restored | 2,361-test full profile |
| BL-06 | Independent closure | A/B review, terminal, receipt artifacts |
| BL-07 | Exact canonical acceptance | fresh PASS receipt and ledger |
