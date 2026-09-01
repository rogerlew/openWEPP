# R101 native-V2 child transaction separation

Status: `IMPLEMENTED_FOCUSED_GREEN_CANONICAL_PENDING`

Static: retained canonical r101 (`/tmp/wghl_001d_v38_64m_r101.log`, SHA-256
`ebed9ac875f59637cf024d6419d015e63a2ffbfb2ee0cb7d937deb00a466a5c9`)
proved that the composed parent retained outer LSE/forcing transaction 42
while the authenticated second soil child owned transaction 43, predecessor
42, state-last 42, and support `1860..1980 s`. The former runtime identity used
the outer transaction for both owners and therefore refused the exact native
V2 soil support join.

Implemented: `RuntimeTileIdentity` now carries the distinct
`soil_thermal_transaction_id`. The existing `transaction_id` remains the
outer LSE/forcing/canopy/hydrology authority. V8 populates the soil field from
the validated prepared/unpublished V2 beginning and uses the outer transaction
for V1. Native-V2 support and finalization compare only against the exact soil
field; all water, request, forcing, canopy, vegetation, biogeochemistry, and
publication transaction uses remain on the outer field. Plain non-composed V2
support still requires outer and soil transactions to be identical.

Ran:

- `cargo nextest run -p openwepp-land-surface-energy native_v2_finalization`:
  4 passed, run `8f23e008-3b1a-4601-ac62-766eb8afb69a`.
- `cargo nextest run -p openwepp-hillslope-orchestrator native_v2_suffix_support_joins_outer_authority_to_exact_child_and_refuses_poisons`:
  1 passed, run `7eb31da2-2665-4304-914c-eda747931c6f`.
- `cargo nextest run -p openwepp-hillslope-orchestrator v38`: 6 passed, run
  `df0cfdcb-7326-4fec-b670-31e1fb1fa000`.
- `cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v38`:
  2 passed, run `82bb9630-23fb-4bb8-bcc0-47ad17272d06`.
- all-target checks for `openwepp-land-surface-energy` and
  `openwepp-hillslope-orchestrator`: passed.

Behavior vectors cover parent transaction 41, first child outer/soil 42, and
second child outer 42 plus soil 43. Zero/missing, stale 42, foreign/out-of-order
44, support overlap/gap/end, and unpublished-binding substitutions fail closed.
No production diagnostic probe remains. Canonical r102 is required for the
real composed consumer and performance disposition.
