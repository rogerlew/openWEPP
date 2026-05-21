# Symbol Alias Registry

Status: Draft (ARCH03)
Evidence: Static
Ran evidence: none

## Purpose

Define the canonical WEPP/wepp-forest symbol authority surface and explicit
openWEPP boundary-name aliases.

Implementation path:
`crates/openwepp-sim-contract/src/symbols.rs`.

## Authority Rule

- Canonical WEPP/wepp-forest symbols remain authoritative contract keys.
- openWEPP field names are aliases only.
- Reverse alias lookup must resolve to exactly one canonical symbol.

## Registry Validation Rules

`SymbolAliasRegistry::new(...)` enforces:

1. non-empty canonical symbols,
2. non-empty boundary aliases,
3. no duplicate `(canonical, alias)` rows,
4. no ambiguous alias reuse across different canonical symbols,
5. non-empty registry.

Violations return typed `SymbolAliasRegistryError` values.

## ARCH03 Baseline Canonical Map

| canonical symbol | openWEPP boundary alias |
| --- | --- |
| `runoff` | `runoff_depth_m` |
| `runvol` | `runoff_volume_m3` |
| `sbrunf` | `subsurface_runoff_depth_m` |
| `drainq` | `tile_drain_flow_m` |
| `sep` | `deep_seepage_depth_m` |
| `st` | `layer_storage_m` |
| `frzw` | `layer_frozen_water_m` |
| `frozen` | `layer_frozen_fraction` |
| `thetdr` | `layer_theta_residual` |
| `thetfc` | `layer_theta_field_capacity` |
| `dg` | `layer_thickness_m` |
| `solthk` | `soil_profile_depth_m` |
| `peakro` | `peak_runoff_rate_m3s` |
| `watdur` | `runoff_duration_s` |

## Lookup Surfaces

- canonical -> aliases: `aliases_for_canonical(...)`
- boundary alias -> canonical: `canonical_for_boundary_alias(...)`

Missing symbols are explicit typed errors:
- `CanonicalSymbolNotFound`
- `BoundaryAliasNotFound`

## ARCH03 Test Linkage

Covered by:
- `tests/integration/sim_contract_symbol_alias_registry.rs`
