# Validation Status

Ran: `cargo nextest run --test assurance_v2_source_contract`

- PASS: 12/12.

Ran: `cargo nextest run --test assurance_v2_amendment_contract report_lead_can_return_pending_review_to_draft_without_erasing_history`

- PASS: 1/1.

Ran: `cargo nextest run --workspace`

- First run: FAIL after 100 passes because two source-contract assertions still
  expected the snow report to be `IN_REVIEW`.
- Corrected rerun: interrupted after 1,158.839 seconds with 208 passes, 5
  skipped, 2 interrupted tests, and 2,063 tests not run.
- Disposition: `NOT RUN` to completion. The interruption is not a test failure,
  but it is not PASS and blocks package closure.

Static: generation
`f9884c0556bea183c9df5d084298d28a4b9243c75208c59591ab6c0f338de0ea`
projects all three assurance reports as `DRAFT`; the public report count is
zero.
