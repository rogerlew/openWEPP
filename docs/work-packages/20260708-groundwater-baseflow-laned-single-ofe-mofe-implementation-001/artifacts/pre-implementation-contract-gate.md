# Pre-Implementation Contract Gate

Status: `PASS`

Evidence: `Static`

Contract gate before production code:

- Read `SC-GWBASEFLOW-001` and `SC-INFILE-GWCOEFF-001`.
- Read prerequisite handoffs from M-T2A, M-T2S, and M-T2R.
- No contract amendment is needed before the first implementation pass.
- `TV-GWBASEFLOW-001` through `TV-GWBASEFLOW-008` are mapped in
  `artifacts/test-plan.md`.
- Production math will implement only the contract recurrence:
  `S_i = S_{i-1} + D_i - Qb_{i-1} - Qs_{i-1}`,
  `Qb_i = bfcoeff * S_i`, `Qs_i = dscoeff * S_i`.
- No surrogate, nonlinear, inferred-default, `latqcc`, or `cbase` substitute
  physics is authorized.
