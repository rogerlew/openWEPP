# Real-Hydrology Surface-Liquid Hold Audit

Status: `executed-hold / bounded soil-layer checkpoint retained`

Evidence class: `Static + Ran`

## Exact Blocked Interface

`OPENWEPP_SNOW_FREE_LSE_V1` requires the hydrology owner to provide and mutate
the immutable beginning mass for `surface_liquid` and `litter_liquid` sources,
and to accept signed condensation credits. The Child-3 forest-litter endpoint
therefore requires one real-owner path preserving source, OFE, tile,
transaction, request, authorization, finalized use, ending storage and restart
lineage.

The production `DirectRunFrame` and the released Child-2
`RealHydrologyShadowAdapter` expose production soil-layer liquid/frozen state
only. `RealHydrologySourceKey` is `(OFE lane, soil layer)`;
`authorize_direct_layer_withdrawals` and
`apply_direct_finalized_layer_liquid_debit` accept subsurface-layer withdrawals
only. No accepted production operation credits surface or litter condensation.

## Evidence That Nearby Values Are Not The Missing Store

- `DirectEvapotranspirationInputs::residue_interception_m` is a daily ET input,
  not persistent hydrology-owned state. Production WB17 consumes it as a
  residue-evaporation operand and returns unused material to the top soil layer;
  it has no accepted transaction, tile store, restart or ending-store lineage.
- `DirectEvapotranspirationSurfaceState::residue_interception_after_m` is a
  same-pass result record, not a beginning owner store; current production sets
  it to exact zero.
- infiltration/depression `depression_storage_delta_m` and WAT5 hourly
  retention are interval diagnostics/flux partitions. They do not provide a
  persistent beginning surface-water store and cannot be relabeled as one.
- snow Stage-3 `liquid_water_m` belongs to snow layers and is outside the
  admitted snow-free domain.

Using any of these as the LSE beginning litter store would invent custody,
omit restart lineage or borrow a different owner/process identity.

## Safe Routes Attempted

1. Traced `DirectRunFrame`, seeded `DirectDayFrame`, subsurface layers, WB17 ET,
   residue inputs, infiltration/depression state, WAT5 retention and snow
   Stage-3 liquid surfaces.
2. Reused the actual production authorization and finalized-debit kernels for
   mixed vegetation-root and bare-ground soil-layer requests.
3. Added typed rejection for `surface_liquid`, `litter_liquid` and
   condensation at the real-owner boundary rather than synthesizing an
   inventory or mutating a diagnostic record.
4. Preserved the low-level LSE constitutive forest-litter and condensation
   implementations and fixtures; only the real-owner connection is withheld.

## Why The Campaign Cannot Close Around The Block

A soil-layer-only runtime would be a prohibited bare-soil-only forest
integration. A sidecar `BTreeMap`, test inventory, residue-flux alias or
depression-delta alias would repeat the diagnostic-owner defect that Child 2
was created to remove. Ground evaporation without a real water owner and
condensation without a real credit candidate also violate exact-one water and
latent-energy custody.

## First Concrete Lift Action

Admit and implement a persistent hydrology-owned, restart-serialized
per-`(OFE,tile,surface class)` liquid store with:

- an exact residue/canopy-ingress initialization and carry rule;
- immutable beginning-snapshot extraction;
- maximum authorization and finalized debit;
- signed condensation credit;
- capacity, infiltration, runoff and downstream-routing joins;
- candidate validation and atomic rollback; and
- an explicit scheduler state point shared with root withdrawals.

That state must be part of the actual hydrology owner or an authority-approved
dependency-neutral extraction of it. Until it exists, Child 3 cannot truthfully
close as a forest-floor runtime connected to real hydrology, and Child 4 cannot
begin its required real forest consumer.
