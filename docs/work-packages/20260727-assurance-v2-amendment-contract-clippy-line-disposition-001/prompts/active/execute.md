# Execute ASSURANCE-V2-CLIPPY-LINE-01

Execute the narrow function-scoped Clippy line disposition through exact
validation, dual review, dual terminal verification, comparator-owned
canonical execution, dual receipt verification, and closeout. Do not change
test behavior, assurance authority, CAL data, or Harvard state.

Subagent authorization: this package explicitly authorizes one bounded
implementation worker, two independent read-only reviewers, two independent
terminal/receipt verifiers, and the `comparator_suite_runner`; writes are
limited to the declared write set.

Subagent requirement: REQUIRED: use `comparator_suite_runner` for canonical
heavy execution. Do not run heavy work on the parent model. If the role is
unavailable, disposition `HOLD`.

Canonical transaction requirement: after dual terminal PASS, derive the exact
clean subject head and use the committed scaffold as the authority base. Run
`tools/local_ci/testgate.py --execute` with intent package
`docs/work-packages/20260727-assurance-v2-amendment-contract-clippy-line-disposition-001/package.md`,
boundary `INCREMENT`, campaign `ASSURANCE-V2-CLIPPY-LINE-01`, and exact claims
recorded in the retained command. Use one fresh external artifact root and one
fresh durable ledger. Do not reuse either prior ledger-bootstrap canonical
root, do not inject a placeholder, do not retry, and do not fall back to the
parent model.

Require and retain, in order: package chain `READY`; intent and terminal plans
with exact predecessor; LIGHT PASS; ten-check pre-HEAVY audit `READY`; HEAVY
receipt PASS; balanced durable ledger; exact node, planned/executed/unavailable
inventory reconciliation; and attempt-index hashes. Two independent receipt
verifiers must validate the receipt, audit, ledger chain, counts, execution
claims, exact head/base/package/campaign identity, and unchanged prior canonical
roots before closeout.
