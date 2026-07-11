# Pre-Implementation Contract Gate

Status: `PASS-AUTHORIZED-TO-IMPLEMENT`

Evidence mode: `Static + Ran`

Recorded before the production edit.

| Seven-gate criterion | Result | Evidence |
| --- | --- | --- |
| Reproduction | `PASS` | Fresh canonical 34-year run fails deterministically at lane 8/day 2621 with `NegativeOutletBin`. |
| Mechanism | `PASS` | Raw predictor outlet face `2 q[n-1] - q[n-2]` becomes negative for eight terminal source-quiet steps; rev-41 lower-bound enforcement is absent. |
| Ownership | `PASS` | Mechanism is in `ofe_routing::kinematic_wave::step`, inside the declared write set. Zero local source and finite nonnegative upstream inflow exclude snow physics and seam booking. |
| Authority | `PASS` | `SC-OFEROUTE-001` rev 51 makes exact-zero downstream face admissibility implementation-authoritative under `INV-OFEROUTE-005/006`. |
| Safety | `PASS` | Correction acts inside conservative face construction, before depth update and ledger booking. No guard/tolerance relaxation, storage clamp, damping, or mass injection/removal is authorized. |
| Testability | `PASS` | Contract-derived dry-front vector fails pre-fix with `NegativeOutletBin`; recorder defense is separately pinned. |
| Validation | `PASS` | Focused closure, D10B/Case-4, 19-OFE cascade, selected active suite, default/off identity, and canonical 34-year false/true endpoint gates are measurable. |

## Authorized Production Edit

Apply the exact-zero lower bound to the downstream predictor stage face before
the existing rev-41 available-water upper cap, so the bounded face is used by
both the depth update and booked outflow. Keep `NegativeOutletBin` and recorder
deficit handling unchanged as defensive invariant guards.

No edit to snow/winter physics, supply booking, daily/off routing, mesh policy,
celerity/local numerics, TVD dissipation, closure tolerance, or hybrid code is
authorized.
