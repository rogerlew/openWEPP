# Execute Assurance Single-Approver Semantics And Draft Return

Tighten governance language to match the current one-person-per-approval-role
event model: applicable producers and maintainers are ineligible approvers;
producer co-approval is not representable without a future schema extension.

Implement a typed, immutable-history-preserving `IN_REVIEW` to `DRAFT` return
event and apply it to the snow/frost flagship. Keep public report count zero,
invent no approval, run the package gates and reviews, and close truthfully.
