# Artifacts

Status: `queued`.

This directory owns the result-blind protocol, authority and operand freezes,
contract/test/implementation evidence, compact four-site results, independent
reviews and dispositions, gate records, exact-diff reconciliation, terminal
verifications, and worker handoff. The rejected first execution remains under
ignored `target/snow_stage3_operator_reconciliation/` as read-only custody.
Any corrected prospective cohort writes only to the separately authorized
ignored namespace. Corrected-but-rejected v2 is retained read-only at
`target/snow_stage3_operator_reconciliation_v2/`; the next prospectively
admitted cohort writes only to
`target/snow_stage3_operator_reconciliation_v3/`.
