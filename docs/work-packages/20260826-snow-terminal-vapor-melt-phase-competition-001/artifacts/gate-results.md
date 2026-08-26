# Gate results

Status: `TERMINAL HOLD / LOCAL GATES COMPLETE`.

- Ran: source split `cargo check --tests`: PASS with retained warnings.
- Ran: Candidate matrix nextest `4329acf0-6d70-4eb8-996a-65c393e97008`:
  7/7 PASS.
- Ran: real complete-owner fixture nextest
  `043c2c94-78a8-452c-904c-98062b92931b`: 1/1 PASS.
- Ran: `cargo fmt --all -- --check`: PASS before the last evidence-only edits.
- Ran: `git diff --check`: PASS before the last evidence-only edits.
- Static: production mode remains `BelowCarrierDomain`; the alternative enum
  variants, allocator, evidence DTO, and execution path are all cfg(test).

- Static review: snow thermodynamics/numerics `NO-GO` at `2064b72a1`.
- Static review: ownership/receiver/chronology `NO-GO` at `2064b72a1`.
- Static review: Rust/QA found the same joint-coupling and custody defects plus
  fail-open non-finite validation and tolerance deletion. The latter two local
  defects were corrected before terminal disposition.

- Ran: final `cargo fmt --all -- --check`: PASS.
- Ran: final `cargo check -p openwepp-hillslope-orchestrator --tests`: PASS
  with retained warnings.
- Ran: final Candidate A/B research matrix nextest
  `f91b64d6-3698-45f6-ad14-5981d33756a3`: 8/8 PASS, including the explicit
  Candidate A partition counterexample and overflow guard.
- Ran: final real fixture nextest
  `d210e917-3c13-4f18-aca1-8beada10e550`: 1/1 PASS; production still returns
  the expected `BelowCarrierDomain` evidence path.
- Ran: `cargo clippy -p openwepp-hillslope-orchestrator --tests`: exit 0 with
  retained workspace warnings.
- Ran: final `git diff --check`: PASS.

Candidate promotion, affected comparator escalation, `cargo deny`, contract
gates, and production qualification are not applicable after the scientific
stop condition. The exact terminal diff is reconciled in
`terminal-diff-reconciliation.md`.
