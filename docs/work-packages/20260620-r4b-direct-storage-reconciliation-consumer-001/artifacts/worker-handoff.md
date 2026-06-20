# R4B Worker Handoff

Status: complete.
Evidence mode: Static.

R4B is complete with verdict
`COMPLETE-R4B-DIRECT-STORAGE-RECONCILIATION-CONSUMER-SPAN`.

Carry forward boundaries:

- no default activation;
- no publication/schema cutover;
- no scheduler edits;
- no compatibility storage/request/writeback/symbol lookup inside direct
  runtime;
- no use of R3B diagnostic ledger as storage authority.

R4B closure facts:

- default-disabled H2637 median `641.14 s`, threshold `<= 676.67 s`;
- protected identity and PASS row equivalence passed;
- `direct_runtime.rs` is 2101 lines and carries a WARN-band split obligation.

Recommended next route:

Scaffold R4C as a narrow direct-runtime continuation that first handles the
WARN-band structure risk if needed, then migrates one canonical upstream
producer for a currently explicit R4B storage operand. Good candidates are a
single ET, deep-seepage `D`, subsurface-loss `Qd`, precipitation-input, or snow
coupling producer, chosen by strongest contract authority and lowest publication
risk. Do not jump to publication until anti-tautological output identity can be
proven over the direct producer/consumer chain.
