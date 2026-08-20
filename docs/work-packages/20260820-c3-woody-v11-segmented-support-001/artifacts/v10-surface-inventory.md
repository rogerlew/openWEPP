# V10 Surface Inventory

Status: PASS / frozen at launch HEAD `5708dac06`

Evidence mode: Ran inventory + Static source inspection

## Duration custody

`VegetationConfiguration.dt_s` is finite-positive canonical configuration
identity and is bit-joined to forcing/receipts. It flows into column interval,
interception/store caps, radiation/gas/energy evaluation, hydraulics amount/rate
conversion, T10, maintenance respiration, carbon offer/allocation, turnover,
mortality, and phenology timers. Principal files are `config.rs`, `column.rs`,
`water_phase.rs`, `persistent_phase.rs`, `carbon_nitrogen.rs`, `hydraulics.rs`,
`energy.rs`, and `occupancy_solver/{constitutive,evaluator,capped_pass}.rs`.
V11 must inject coupled-time `duration_s_bits` once; no local reconversion or
nominal-cadence read may enter a segment solve.

## Transaction and candidate custody

`water_phase.rs` derives `TransactionId(beginning.last_transaction_id + 1)`
before potential request, one water authorization, capped solve, final use, and
water-owner candidate. `persistent_phase.rs` requires the same ID, executes
phenology/turnover and one mineral-N arbitration, and produces staged C/N state.
`vegetation_candidate.rs` constructs ending state, sets shared/occupancy
transaction lineage, binds material proposals and water/energy/C/N/dry-material
ledgers, but exposes no commit. The complete orchestrator owner envelope owns
commit. Reusing this entry per segment would incorrectly increment the parent.

## Sequential physical state

Canopy liquid and occupancy warm starts, tile canopy air, T10, displayed/
storage/transfer tissue pools, NSC/XS/retranslocation, phenology phase/timers/GSI,
derived areas, and transaction lineage are stateful. Segment `k+1` must begin
from accepted staged ending `k`, not the parent beginning.

## Resource owners

- Water: potential requests -> same-snapshot authorization -> capped finalized
  use -> hydrology candidate; only final use is debit.
- Nitrogen: per-layer/species NH4/NO3 potential request -> authorization ->
  final use -> BGC candidate; internal retranslocation precedes external use.
- Energy: immutable occupancy/tile operands and reciprocal owner candidates;
  latent mass/energy and canopy release enthalpy join exactly.
- Material: phenology/turnover/mortality amounts are constructed during each
  persistent solve, then bound into deterministic transaction-scoped proposals.
  V11 authority must accumulate accepted segment proposals into one parent
  proposal/ledger; it may not recompute nonlinear turnover only from final state.

## Scheduled-once candidates

Inventory before release: forcing/GSI receipt consumption, phenology edge
selection, management/calendar/day transitions, daily initialization, parent
material proposal finalization, receipt publication, and transaction increment.
The first four require explicit boundary authority; finalization/publication/
increment occur once per parent, never per slab.

## Restart

Existing DirectV10 V1 snapshots interval-granular complete committed owners and
in-progress day state. No V11 mid-parent staged checkpoint exists. V11 restart
must be additive and compose coupled-time V2 with staged vegetation and owner
candidates; it cannot add fields to existing V1 DTOs or change released bytes.

## Frozen source hashes

- `config.rs`: `b49b8904...176d9bf`
- `v10_state.rs`: `fed6f222...74f595`
- `water_phase.rs`: `486f262c...4706de`
- `persistent_phase.rs`: `e1891335...f47b8def`
- DirectV10 restart schema: `71c6905d...11a05d`
- coupled-time restart V2 schema: `96003072...f8e8b29`
