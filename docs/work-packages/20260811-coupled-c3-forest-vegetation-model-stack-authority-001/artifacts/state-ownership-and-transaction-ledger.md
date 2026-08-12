# State Ownership And Transaction Ledger

Status: `selected`

Evidence mode: `Static`

| Owner | Sole mutable state | May only request/read |
|---|---|---|
| vegetation | stratum C/N pools including `NSC_C` and signed maintenance reserve, liquid canopy store, equilibrium-hydraulic warm starts, geometry, phenology and pending donor transfers | atmospheric forcing, radiation/energy receipts, soil-layer observations, mineral-N and water authorizations |
| land-surface energy | canopy/ground absorbed-energy and sensible/latent/conductive ledgers | vegetation temperatures/conductances and actual water phase changes |
| hydrology | soil-layer liquid/frozen water and competing withdrawal allocation | immutable vegetation layer requests |
| biogeochemistry | layer mineral NH4/NO3 and litter/CWD C/N/DM receiving pools | immutable vegetation N and material requests |
| orchestrator | transaction identity and atomic commit set | all candidate states/receipts |

One interval uses one immutable beginning snapshot and this order:

1. Validate schema, topology, initial/current-state identities and supported
   branches. Radiation traverses each topology column top-to-bottom.
2. Vegetation constructs a candidate interception store and jointly solves
   sun/shade FvCB--Medlyn--energy--hydraulics. It emits potential layer water
   requests, potential carbon gain, N demand, and transfer proposals; nothing is
   yet actual or published.
3. Hydrology arbitrates all water requests and competing withdrawals per layer
   from the same snapshot and returns maximum authorizations `A_W<=D_W`; it
   does not yet debit soil. Vegetation re-solves energy, gas exchange and
   hydraulic complementarity under those caps and returns finalized
   `F_W<=A_W`. Hydrology validates `F_W`, then forms its candidate debit.
4. Stage-A potential GPP plus carried `NSC_C` defines potential full-growth N
   demand. Vegetation first forms turnover/retranslocation candidates, uses the
   frozen internal-N offer, and distributes only the remaining external demand
   by caller mineral-N root fractions and NH4 preference. Final water-limited
   GPP can only lower that external need.
5. Biogeochemistry arbitrates all same-snapshot mineral-N requests and returns
   maximum `A_N<=D_N` without mutation. Vegetation recomputes final demand,
   distributes `min(final_need,sum A_N)` proportional to the authorizations,
   emits `F_N<=A_N`, assigns tissue N by exact C:N, and carries N-limited carbon
   in `NSC_C`. Unused authorization and retranslocation remain owned and are
   never debited. Biogeochemistry validates finalized use and constructs its
   candidate mineral-N debit plus material-receiver credits.
6. Energy independently reconstructs canopy and ground closure. Hydrology
   proves `T_s=sum_l F_W,s,l`. Biogeochemistry and vegetation independently prove
   finalized N use and each litter/CWD C/N/DM transfer. Canopy-water closure is
   `start+rain+condensation=end+wet_evap+throughfall+stemflow+drainage`.
7. Only the orchestrator atomically commits the complete owner set. Any invalid
   receipt, mismatch, nonconvergence, or closure failure discards every
   candidate and preserves byte-identical beginning state.

For both resources the binding order is `request -> authorization -> finalized
use -> owner validation -> atomic commit`, with
`0<=finalized<=authorization<=request`. Default mineral-N arbitration is not a hidden ecological preference: v1
canonically selects proportional allocation among positive same-layer requests,
`receipt_i=available*request_i/sum(requests)` when oversubscribed, otherwise
full supply. Every competing mineral-N owner uses the same three-stage protocol,
so no full competing debit can bypass the proportional authorization. A future
caller policy needs a new model version. Water arbitration uses the same
version-5 equal-status proportional same-layer policy.

Forest-floor evaporation is computed only by its owning surface from its own
energy, resistance, humidity and water state. No canopy shortfall or reduced
LAI term appears in that equation, making agricultural PMET donation
structurally impossible.
