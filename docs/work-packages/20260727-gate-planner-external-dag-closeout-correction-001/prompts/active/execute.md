# Execute GATE-EXTERNAL-DAG-DC-01

Execute
`docs/work-packages/20260727-gate-planner-external-dag-closeout-correction-001/`
autonomously through correction, focused gates, canonical admission, delegated
heavy gates, dual review, dual verification, and closeout. Do not execute
CAL-04B population or inspect Harvard content.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to bounded implementation workers, two independent
read-only scaffold/implementation reviewers, two independent read-only
terminal verifiers, and the `comparator_suite_runner` for selected heavy gate
execution. Expected outputs are bounded code/evidence, retained receipts, and
compact verdicts; write access is limited to the package's declared write set.
