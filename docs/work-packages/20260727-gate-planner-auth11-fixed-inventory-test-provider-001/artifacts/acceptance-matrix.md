# Acceptance Matrix

Status: `RECEIPT VERIFICATION REQUIRED`

Evidence class: `Static + Ran`

| ID | Obligation | Evidence |
|---|---|---|
| TP-01 | Exact failure retained | predecessor implementation-gates artifact and Nextest run `b47dca9b-8772-46fe-a596-c5efc464851f` |
| TP-02 | Test-only correction | exact terminal diff contains only declared paths |
| TP-03 | Exact AUTH11 inventory | Provider and assertion bind `auth11_all_active_required_suite_targets_exist_and_are_registered`, `auth11_obligations_schema_and_anchor_bindings_are_enforced`, and `auth11_registry_posture_and_protocol_guard_paths_exist` |
| TP-04 | Production enforcement unchanged | no production path diff; focused negative/positive tests green |
| TP-05 | Full suite restored | gate-planner Nextest 227/227 |
| TP-06 | Independent acceptance | PASS through dual terminal; receipt verification queued |
| TP-07 | Security impact | exact diff contains only the provider branch, deterministic-test assertions, and authorized evidence/docs; no production behavior changes |
