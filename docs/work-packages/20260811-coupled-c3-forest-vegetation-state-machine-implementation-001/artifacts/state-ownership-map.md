# State Ownership Map

Status: `V6 occupancy candidates active / receiving water-owner candidate uncommitted`

The authority audit confirmed that the V2 per-layer root-potential lanes have
no canonical accepted-update mapping from E14's common root-node solution.
Potential candidates therefore remain unavailable and no lane is broadcast.

Vegetation solely owns one canopy-liquid and coupled warm-start lane per valid
`(stratum_id,tile_id)` occupancy, plus shared stratum phenology, geometry, and
six-tissue C/N pools. The diagnostic water state owns per-layer soil water; BGC
owns independent layer/species mineral N and typed litter/CWD receipts; the
energy owner independently reconstructs occupancy and weighted-stand operands;
the orchestrator owns transaction identity and the one atomic commit. Water
identity includes transaction, owner, occupancy, layer, resource, and amount
basis through request, authorization, finalized use, debit, and receipt.

The thirteen historical failure injections remain regression evidence. V2 must
extend byte-identical rollback to every occupancy lane, shared C/N state, all
owner candidates, pending transfers, and transaction identity.

Ran: `CoupledOwnedState` contains only typed shared-stratum and occupancy maps.
Every mutable canopy-liquid and numerical warm-start field exists solely in an
`OccupancyState` lane. Exact configuration-derived occupancy equality,
root-layer ordering, model/configuration/state digests, and initial/prior
transaction lineage pass focused tests. No V2 execution path can access the
historical shared liquid or hydraulic warm starts; those fields remain only in
the explicitly named V1 migration DTO.

Increment 2A borrows all beginning owner state immutably and returns owned
column candidates. It never edits an occupancy map, shared stratum, pending
transfer, state digest, or transaction identity. Injected failure at a lower
occupancy after upper candidate work preserves the serialized beginning state
byte-for-byte. Whole-owner atomicity is still a Milestone 5 obligation.

## V4 Shared-State Ownership

Ran: executable state now binds `OPENWEPP_C3_WOODY_V4` and the exact v8 field
set. `StratumSharedState` contains the six tissue display/storage/transfer C/N
pools, retranslocation and reserves, standing dead, phenology/timers,
`previous_gsi`, typed pending transfers, `t10_k`, three derived area caches,
and accepted transaction lineage. The unconsumed
`previous_leaf_offset_flux` and `previous_root_offset_flux` fields exist only
in the historical V3 migration DTO and are removed during the explicit V3-to-
V4 migration.

Displayed leaf carbon is the sole owner of accepted LAI and the three exact
area caches. Displayed leaf nitrogen is the sole positive-LAI leaf-N input to
FvCB capacity and Atkin/Rd ownership. Storage and transfer leaf subpools remain
mass state but cannot manufacture area, capacity, or a second leaf-maintenance
debit. Non-leaf maintenance continues to consume the admitted complete tissue
N identities.

The whole-state digest uses the structural
`OPENWEPP_V4_STATE_CANONICAL_V1` encoder, including exact typed paths,
transaction lineage, pending-transfer identities, shared state, occupancy
identity, and all occupancy lanes. Recursive exact-shape validation rejects
missing/unknown fields before typed decode; duplicate structural identities
are rejected. Production serialization matches the independent released
shared-state vectors and all 155 whole-state scalar mutation digests.

The explicit V3-to-V4 migration validates source definition/configuration/state
digests, complete membership, tissue/domain/lineage/transfer identities, and
bit-exact V3 displayed-area caches. It compares the unchanged constitutive
configuration payload, removes only the two obsolete fields, copies every
other shared and occupancy value, rebinds V4 identities, recomputes the V4
digest, and revalidates the candidate. Invalid simultaneous owners produce one
deterministic exhaustive report and no candidate. V1-to-V4 and V2-to-V4 direct
normalization remain prohibited.

The public transaction remains fail-closed before the authorization-capped
second pass. Therefore this map claims V4 state ownership and migration only;
it does not claim accepted E20--E22 state updates, owner candidates, or atomic
commit.

## V5 Identity And Capped Ownership

V5 imports the V4 state payload byte-for-byte but is not an executable alias.
The V4-to-V5 transition must validate the complete V4 source, bind distinct
caller-supplied V5 model and configuration identities, recompute the V5 state
digest, and reject stale V4 identity without producing a partial result. No
state field may be synthesized or normalized.

During the capped pass, hydrology continues to own fixed maximum
authorizations and beginning inventory. Vegetation may finalize only exact
`F_W = f_t * q_i * dt`, and hydrology debits only that finalized use, never the
authorization. Implementation and owner-validation tests are active; this map
does not yet claim V5 accepted state, debit, receipt, or atomic commit.

## V6 Public Water-Phase Ownership

The public water stage borrows the complete beginning vegetation state and
water owner immutably. Potential occupancy lanes and final capped occupancy
lanes are unaccepted numerical candidates: their
`last_accepted_transaction_id` remains the beginning identity. They cannot be
converted to a whole `CoupledOwnedState` until shared C/N and every receiving
owner candidate are complete.

Hydrology owns a typed immutable `WaterOwnerSnapshot`, returns it with maximum
authorizations and one required reason per request, and constructs
`WaterOwnerCandidate` from the exact typed D/A/F protocol against that same
snapshot. Vegetation independently validates transaction, owner, occupancy,
layer, basis, reason, per-layer aggregate authorization, finalized debit, and
ending store. The candidate is exposed
only through `UncommittedWaterPhase`; no vegetation-only or water-only commit
API exists. Failure injection through owner validation returns no phase and
preserves beginning vegetation bytes.
