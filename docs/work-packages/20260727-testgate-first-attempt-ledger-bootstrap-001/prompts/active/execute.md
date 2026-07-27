# Execute TESTGATE-LEDGER-BOOTSTRAP-01

Execute the secure first-attempt ledger bootstrap through focused/full
validation, dual review, dual terminal verification, canonical admission,
comparator-owned execution, dual receipt verification, and closeout. Preserve
both failed roots, do not inject gates or placeholders, do not populate CAL,
and do not access Harvard.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to one bounded implementation worker, two independent
read-only reviewers, two independent terminal/receipt verifiers, and the
`comparator_suite_runner`; writes are limited to the declared write set.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for all heavy
batch/closure/comparator runs; do NOT run them on the parent model unless the
subagent is unavailable, in which case record command-level evidence. Outputs
are compact metrics plus retained audit, receipt, ledger, and log paths;
repository write access is read-only.
