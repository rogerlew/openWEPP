# Current API and owner map

Status: `Static / pre-implementation audit`

## Closure path

- Repository climate owner: `HillslopeClimateRuntimeRequest`.
- Draft GSI receipt: `DirectGsiDailyReceiptV1`; it embeds beginning/ending
  state, parameters, forcing, and result but lacks explicit owner/run/day/source
  joins required by the resumed campaign.
- Static forcing configuration: `SnowFreeHalfHourStaticConfiguration`.
- Provider cursor: `SnowFreeHalfHourProviderCursor`; daily GSI is correctly
  outside its static identity, but restoration and public routes are not yet
  sealed to the required static configuration contract.
- Prepared provider envelope: `PreparedSnowFreeGsiDayV1` with staged GSI state
  and `ValidatedSnowFreeHalfHourForcingReceipts`.
- Consumer owner: `DirectV10RealConsumerShadow`; legacy constructors and raw
  execution/projection routes remain visible and must be narrowed.
- Canonical scientific owners: CP-GSI01, provider cursor, V10 vegetation,
  LSE-V2, direct hydrology (including the single surface-liquid state), soil
  thermal, BGC, and scheduler position.
- Reconstructed transient owners: V9 vegetation and LSE-V1; restoration must
  exact-check non-identity payload equivalence.

## Current defects to close

- `prepare_snow_free_gsi_day` accepts caller-completed `GsiDailyForcing`
  instead of deriving it from the selected repository day.
- `DirectGsiDailyReceiptV1` lacks `owner_id`, `run_id`, `day_index`,
  `source_climate_sha256`, and explicit beginning/ending owner digests.
- closure constructors still fabricate generalized GSI parameters, empty GSI
  state, and a default provider cursor.
- the legacy completed-GSI forcing configuration and raw receipt route remain
  externally callable.
- the restart contract still describes duplicated top-level owners plus an
  in-progress envelope rather than the exact two-variant phase union.
