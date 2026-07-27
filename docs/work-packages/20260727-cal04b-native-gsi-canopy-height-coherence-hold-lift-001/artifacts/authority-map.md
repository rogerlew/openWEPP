# Authority Map

Status: `A0 ADMITTED`

Evidence class: `Static`

| Authority / surface | Role | Required action |
|---|---|---|
| `SC-PLANT-001` | Canonical current canopy-state and consumer-order authority | CP-GSI02 currently omits a native `Hc/canhgt` law and operand basis; A0 amendment/decision is required before code |
| `SC-PLANT-001` Binding Exposure Index | Contract-to-code/test visibility | Map every changed invariant, guard, projection, and consumer |
| Pinned legacy baseline | Equation/order provenance | PL16 Eq. 8.2.6 uses total above-ground `vdmt`; rangeland/forest uses geometry. Neither directly authorizes foliar-only `Bf` substitution |
| Production day builder | Same-day GSI state projection | Correct under contract authority |
| Orchestrator growth state | Typed biomass/LAI/cover/height state | Preserve one coherent state and typed guards |
| Lane D active router | Direct production consumer | Prove it reads corrected post-growth height and LAI |
| Snow/ET/WB15/erosion/residue/frost | Downstream consumers | Prove real same-day reads; reject stale/static/shadow paths |
| CAL-04B attempt 004 | Exact reproducer | Replay read-only `GSI-5557` transition and entire frozen native-proof plan |

Package-local notes do not replace canonical authority.

## Confirmed Gap

`CP-GSI02` defines `Bf`, `LAI`, and `Cc`, but not `Hc`. Its typed operands omit
`bbb` and `hmax`, and its algorithm's publication step names only
`Bf/LAI/Cc`. The general symbol map and `INV-PLANT-036` prove that height is a
required real-consumer surface; they do not define native height physics.

The pinned PL16 equation cannot be applied by merely renaming native `Bf` to
legacy `vdmt`: their declared mass bases differ. Persistent native structural
biomass `Bs` is also excluded from seasonal foliar transfer. A `Bf+Bs`
construction or a cover-to-height inference would be new physics without a
separate authority argument.

Revision 24 closes the gap with checked internal `Bt=Bs+Bf`, Chapter 8
Equation 8.2.8, the identical pinned `grow.for` expression, and same-day
`Hc` publication. `Bf` remains the foliar/interception biomass handoff.
Independent contract reviewers A and B passed the final amendment.
