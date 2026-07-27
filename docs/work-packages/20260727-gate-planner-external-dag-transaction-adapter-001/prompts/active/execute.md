# Execute Gate Planner External DAG Transaction Adapter

Execute
`docs/work-packages/20260727-gate-planner-external-dag-transaction-adapter-001/`
end-to-end.

Read root, crate, standards, and work-package instructions before governed
edits. Freeze failing fixtures first. Reuse the canonical Rust pre-heavy audit,
receipt, verifier, package-admission, and durable-ledger implementations; do not
create a package-local policy duplicate or standalone audit transport.

Implement confined external-DAG transactions and a second authenticated
post-custody transition. Rework CAL-04B only at the coordinator/output boundary;
do not change science, calibration design, source observations, or Harvard
custody. Do not run CAL-04B population or holdout in this prerequisite.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to bounded implementation workers, two independent
read-only reviewers, two independent read-only terminal verifiers, and the
`comparator_suite_runner` for selected heavy gates. Keep worker ownership
disjoint. Heavy execution is comparator-owned and must consume the canonical
READY transition.

Continue through truthful closeout unless a hard boundary outside the declared
write set is proven.

