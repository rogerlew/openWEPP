# Gate results

Status: `EXECUTING / PRE-REVIEW`.

- Ran: source split `cargo check --tests`: PASS with retained warnings.
- Ran: Candidate matrix nextest `4329acf0-6d70-4eb8-996a-65c393e97008`:
  7/7 PASS.
- Ran: real complete-owner fixture nextest
  `043c2c94-78a8-452c-904c-98062b92931b`: 1/1 PASS.
- Ran: `cargo fmt --all -- --check`: PASS before the last evidence-only edits.
- Ran: `git diff --check`: PASS before the last evidence-only edits.
- Static: production mode remains `BelowCarrierDomain`; the alternative enum
  variants, allocator, evidence DTO, and execution path are all cfg(test).

Affected regression, Clippy, final formatting/diff, reviews, verification, and
terminal exact-diff reconciliation remain pending.
