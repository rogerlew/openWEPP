# Characterization

Ran: focused characterization passed before production decomposition.

- Native attestation: one isolated parent test passed five child-process
  scenarios: missing `gh`, malformed JSON, nonzero status, empty verification,
  and successful nonempty verification. The success case asserted the complete
  ordered `gh attestation verify` argument vector. The ignored child remains an
  intentional harness entry and executes only under the parent.
- Checkpoint admission: 10 checkpoint-focused tests passed, including the two
  new table-driven tests for valid admission; identity, root, execution,
  claims, and attempt precedence; artifact shape/read/digest order; and receipt
  membership.
- Candidate discovery: the new focused test passed ledger-read-before-plan,
  plan-shape, reverse-ledger, and newest explicit-root fail-closed cases.
- Constructed READY audit: the existing sealed audit fixture passed after being
  extended with valid empty-ledger resume discovery plus non-READY, plan-ID,
  and plan-digest binding rejections.

Ran: focused command summaries were 10/10 checkpoint tests, 1/1 attestation
parent, 1/1 candidate precedence, and 1/1 constructed-audit fixture. Formatting
passed with `cargo fmt --all -- --check`. These focused commands are not the
post-change matching-module metric run.

Static: test-only characterization lives in `resume_coverage_tests.rs` to keep
the production module below the line-count warning. The constructed-audit call
is placed in the existing pre-HEAVY coverage fixture because only that child
module can construct the private proof type without widening production API.
