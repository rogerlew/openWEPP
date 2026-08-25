# Terminal same-support dependency/SCC and forcing inventory V2

Status: `Static / corrected successor authority input`

Base: `83fb00514e8932561bee5aff26ccdf7c130d470f`

This artifact closes intake finding `TBTV20-NUM-006`: prescribed upstream
amounts, arm-generated amounts and state-dependent absorption were not fully
separated in v20. It does not authorize implementation.

## Dependency rule

Draw edge `A -> B` when a typed ending, algebraic variable, rate or generated
amount from `A` changes `B` on the same candidate support. Strongly connected
components are solved together. A purported follower is exact only when it has
no path back into the coupled component on that support. If runtime/source
inspection finds such a path, the owner moves into the implicit core; no lag,
frozen beginning value or post-hoc merge is authorized.

## Same-support graph

```text
prescribed atmosphere/provider amounts and boundary series
                         |
                         v
  +-----------------------------------------------------------+
  | implicit physical SCC                                     |
  | snow state <-> snow boundary/LSE <-> shared canopy air    |
  |      ^                 <-> vegetation thermal/water stores |
  |      |                              |                      |
  | soil thermal <-> q_ss               v                      |
  |      ^                    generated ground-liquid amounts  |
  |      |                              |                      |
  | hydrology/soil water <-> surface liquid/WB14 <-------------+
  |      |                    |                                |
  |      +---- root supply ---+                                |
  +-----------------------------------------------------------+
               | accepted typed candidate only
               v
  exact followers: BGC debit/pool transition when beginning-pool-only;
  canonical owner serialization; hashes/receipts; diagnostics
```

The BGC transition is a follower only in the admitted exact-one-bearing-OFE
domain where current-support mineral use is fixed by the converged vegetation
candidate and ending BGC cannot alter that candidate. Any mineral-limitation
feedback makes BGC part of the SCC. Receipt construction and serialization are
always discrete followers and never numerical unknowns.

## Core unknown order and units

For each finite active set, the scaled unknown vector is ordered:

1. lanes by increasing `lane_id`: ice `kg m^-2`, retained liquid `kg m^-2`,
   cold content/material enthalpy `J m^-2`, ordered represented-layer storage;
2. OFE then soil layer: participating soil enthalpy `J m^-2`;
3. OFE/tile/vertical occupancy: vegetation liquid stores `kg m^-2` and
   component temperatures `K`;
4. OFE/tile: LSE surface temperatures `K`;
5. carrier nodes by canonical topology: temperature `K`, specific humidity
   `kg kg^-1`;
6. OFE hydrology/surface states in owning typed depth/mass/energy units.

Residual order matches this list: storage balances, soil equal/opposite energy,
vegetation water/energy, LSE component energy, carrier sensible/vapor closure,
surface-liquid/WB14 closure and hydrology transition constraints. Discrete
layer counts, route tags, active sets, hashes and owner bytes are excluded from
the vector and validated exactly.

## Owner construction

Both low and high arms begin from one immutable seven-owner set. They solve the
complete SCC, then execute exact followers from that arm's converged typed
state. Each follower emits a receipt proving its inputs and absence of feedback.
The complete ending owner set is serialized once. No derivative, interpolation
or norm of owner bytes exists.

## Forcing and output classification

| Class | Members | Arm rule |
|---|---|---|
| prescribed upstream amount | atmospheric solid/liquid parcel mass and advected heat; provider-integrated incident radiation/longwave energy when supplied as amounts; externally timed liquid supplies | intersect/project from sealed provider support once; identical receipt/value in every arm; never divided into a rate or re-quadratured |
| prescribed boundary series/rate | incident shortwave irradiance, atmospheric longwave, air state, wind/exposure when provider exposes tick-evaluable series | evaluate only at contract-authorized endpoint/collocation ticks; provider identity identical across arms |
| arm-generated amount | vegetation throughfall, initial/second drainage, stemflow, condensation/evaporation totals; bounded deposition/sublimation; melt/refreeze/retention/routing; surface ingress/runoff/runon; BGC debit | generated inside each complete arm from that arm's state; cannot be copied between arms or treated as prescribed |
| state-dependent endpoint/collocation rate | absorbed shortwave after state-dependent canopy/snow optics; emitted/reciprocal longwave; sensible/vapor exchange; latent rate; snow--soil/interlayer conduction | participates in owning BE/CN/defect quadrature; each evaluation binds typed state and tick |
| algebraic variable | canopy/LSE temperatures, shared-air temperature/humidity, transfer flux closure variables | solved inside SCC at each required tick/state; no independent time derivative unless owning storage exists |
| discrete event output | terminal posture, layer lifecycle, route choice, group topology, pending terminal parcel, accepted event receipt | emitted only after accepted high state; exact predicate and receipt; never used in floating estimator |

Incident and absorbed shortwave are distinct. Atmospheric precipitation parcels
and arm-produced vegetation releases are distinct. Terminal liquid remains a
snow-owned discrete output after accepted prefix and is absent from pre-event
hydrology/WB14/surface-liquid ingress.

## Feedback audit gate

Before implementation, source-level guards and typed tests must prove follower
inputs are functions only of prescribed beginnings plus the converged SCC
candidate. Wrongly classified follower feedback is a contract failure and
typed unsupported disposition, not permission to freeze or lag the edge.
