# PL15R Risk-Acceptance Approval Reference

Status: `complete`
Evidence mode: `Static + Ran`

## Applicability

PL15R requires explicit risk-acceptance reference when unresolved Tier-A
blockers remain after active evidence classification.

## Review Outcome

| field | value |
|---|---|
| unresolved Tier-A blocker count | `3` |
| approved risk-acceptance reference present | `no` |
| approval owner | `N/A (not approved)` |
| rationale | `N/A (not approved)` |
| scope | `N/A (not approved)` |
| policy effect | `retain hold` |

## Active Blocker Register

| blocker_id | class | description | source |
|---|---|---|---|
| `PL15R-BLK-001` | provenance | Tier-A candidate lane strict-pass classification is not from direct openWEPP runtime emission. | `pl14r-comparator-run-provenance-manifest.md`, `pl14r-schema-aligned-day-by-day-retest.md` |
| `PL15R-BLK-002` | runtime-surface | No executable openWEPP hillslope driver path is available to generate authoritative comparator candidates. | workspace cargo metadata (`[[bin]]` absent) |
| `PL15R-BLK-003` | physics-authority | WB11 ET/percolation/lateral/drainage and WB12/WB14 reconciliation posture remain insufficient for full physics-parity hold-lift claims. | `wb11-kernel-algorithm-guard-map.md`, `SC-WATBAL-001` |

## Decision

No formal risk-acceptance approval artifact was provided for active PL15R
blockers. Under policy, PL08 hold-lift cannot be issued.
