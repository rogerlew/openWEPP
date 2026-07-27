# Acceptance Matrix

Status: `SCAFFOLD REVIEW REQUIRED`

Evidence class: `Static + Ran`

| ID | Obligation | Evidence |
|---|---|---|
| AUTH11-01 | Blocked plan retained | `blocked-plan.md` and external attempt root |
| AUTH11-02 | Exact node selected once | `auth11_required_suite_node_is_selected_once_for_authority_surfaces` |
| AUTH11-03 | Inventory independently bound | Exact three-test inventory and 2,376→2,379 delta assertion |
| AUTH11-04 | Existing authority gates preserved | Exact 12-node prefix/set preservation and 13-node green assertion |
| AUTH11-04A | Selection is bounded | `auth11_required_suite_node_is_not_selected_for_unrelated_critical_diff` |
| AUTH11-05 | No manual injection | Canonical plan/audit/receipt chain |
| AUTH11-06 | Exact-head correctness passes | Comparator-owned admitted heavy receipt |
| AUTH11-07 | Independent acceptance | Dual review and dual verification |
