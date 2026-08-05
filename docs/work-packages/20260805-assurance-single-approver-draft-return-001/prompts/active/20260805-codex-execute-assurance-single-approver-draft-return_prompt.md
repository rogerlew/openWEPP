# Execute Assurance Single-Approver Semantics And Draft Return

Tighten governance language to match the current publication contract:
applicable producers and maintainers are ineligible approvers; producer
co-approval is not representable without a future schema extension.

Implement a typed, immutable-history-preserving `IN_REVIEW` to `DRAFT` return
event and apply it to the snow/frost flagship. Keep public report count zero,
invent no approval, run the package gates and reviews, and close truthfully.

This prompt explicitly authorizes subagent spawning/delegation to the
read-only reviewers, terminal verifiers, and comparator-suite runner declared
by the package.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for the
immediate full-workspace correctness regression after the implementation
increment is committed and the typed DRAFT return is applied.
