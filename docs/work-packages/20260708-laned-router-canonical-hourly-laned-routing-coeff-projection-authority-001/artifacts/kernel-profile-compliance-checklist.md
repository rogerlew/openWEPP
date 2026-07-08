# Kernel Profile Compliance Checklist

Status: complete.
Evidence class: Static.

Touched contract:
`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.

| Profile surface | Disposition |
|---|---|
| Frontmatter/version | Updated `contract_version` from `47` to `48`. |
| Purpose/scope | Scope now distinguishes explicit coefficient authority from rejected legacy-field projection. |
| Authority anchors | Added management-lanuse and baseline `frcfac.for` anchors. |
| Variables/units | No new physics variable introduced; unit map now rejects implicit conversion from legacy cropland fields. |
| Algorithm | No solver or algorithm math changed. |
| Branch/guard table | Friction operand sourcing, active opt-in, and conditional default rows amended fail-closed. |
| Invariants | `INV-OFEROUTE-010` updated with rev-48 source-authority wording. |
| Producer/consumer obligations | `OBL-OFEROUTE-P-007` updated for no implicit projection. |
| Symbol alias map | Static coefficient aliases remain explicit `ofeN_route_*`; rejected legacy aliases named. |
| Test-vector obligations | Added unsupported projection as a negative authority vector. |
| Binding Exposure Index | Added `OFEROUTE-ROUTE-COEFF-PROJECTION-AUTHORITY`. |
| Gaps | Added `GAP-OFEROUTE-008` as held/rejected projection authority. |
| Revision history | Added rev 48 row. |

No production kernel math, runtime selector, closure tolerance, mesh policy, or
Rust implementation was changed.
