# Acceptance Matrix

Status: `IMPLEMENTATION REVIEW HOLD ACCEPTED`

Evidence class: `Static + Ran`

| ID | Obligation | Evidence |
|---|---|---|
| AUTH11-01 | Blocked plan retained | `blocked-plan.md` and external attempt root |
| AUTH11-02 | Exact node selected once | `auth11_required_suite_node_is_selected_once_for_authority_surfaces` |
| AUTH11-03 | Inventory independently bound | Exact sorted three-name node inventory; global unique 2,376→2,378; per-node sum 3,090→3,095; workspace 2,350→2,352 |
| AUTH11-04 | Existing authority gates preserved | Exact prerequisite node-ID edges and 12→13-node reconstruction |
| AUTH11-04A | Selection is bounded | Plan-level `auth11_required_suite_node_is_not_selected_for_unrelated_critical_diff` |
| AUTH11-04B | Real integration surface selects | Exact matcher and positive case use `tests/integration/auth11_required_suite_obligation_guards_contract.rs` |
| AUTH11-05 | No manual injection | Canonical plan/audit/receipt chain |
| AUTH11-06 | Exact-head correctness passes | Comparator-owned admitted heavy receipt |
| AUTH11-07 | Independent acceptance | Dual review and dual verification |
