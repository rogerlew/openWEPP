# Aggregate Verification A

Static: PASS for retained technical evidence at exact HEAD
`eadc01459df18e83d94362dc225219232f0a4c65`.

Ran: canonical receipt-envelope verification, ledger and attempt-snapshot
validation, checkpoint/inventory comparison, source-manifest comparison,
binary hash, plan, audit, Nextest logs, and CRAP report checks passed. Receipt
`c22fe3f...f06ca` has 15/15 PASS, zero retries, unchanged source, ordinary and
instrumented Nextest 2,293/2,293 PASS, and closure-eligible global CRAP with
zero actionable rows. The 151-record ledger closes PASS at `2096272c...b067b`;
no effective tooling defect remains open.

Static: this verifier records technical eligibility only. Repository-reviewed
attestation remains a separate trust-boundary requirement.
