# External DAG Path Equivalence

Status: `HISTORICAL / SUPERSEDED`

Evidence class: `Static`

This artifact previously supported the external-DAG transaction adapter. That
adapter is no longer a prospective CAL-04B execution path. Its retained plans,
attempts, and receipts remain historical evidence and are neither rewritten
nor executed by Order 2.

The prospective path is package-local direct execution governed by
`direct-execution-plan.json` and `execution-control-contract.md`. Output-path
injection remains explicit, but it no longer requires equivalence to a planner
transaction path that has no prospective consumer.
