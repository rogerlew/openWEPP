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
