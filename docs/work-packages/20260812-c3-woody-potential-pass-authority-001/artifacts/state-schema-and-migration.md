# V3 State Schema and Migration

Status: `approved authority complete`

Evidence mode: `Static`

V3 occupancy state has exactly 15 recursively lexicographic fields:

```text
beta_hyd
canopy_air_specific_humidity_kg_kg
canopy_air_temperature_k
canopy_liquid_kg_h2o_m2_tile_ground
dry_stem_temperature_k
last_accepted_transaction_id
root_node_potential_mm
shade_ci_pa
shade_leaf_potential_mm
shade_leaf_temperature_k
stem_potential_mm
sun_ci_pa
sun_leaf_potential_mm
sun_leaf_temperature_k
wet_surface_temperature_k
```

Every field enters canonical state serialization and the state digest. Layer
identity remains in configuration, soil forcing, `q3_i`, water requests,
authorizations, finalized use, owner debit, and receipts. It is not persistent
root-node numerical state.

The persisted `beta_hyd` is not a common constitutive stress factor. Stage A
and the capped pass solve distinct internal `beta_sun` and `beta_shade`. After
acceptance, `beta_hyd` is their exact Emax-weighted aggregate, or exact one for
the zero-maximum-demand branch, and is used only as the next solve's common
initial guess and as a diagnostic.

V2 migration is deterministic only when the ordered V2 layer vector is nonempty
and every potential has identical IEEE-754 bits. That common value becomes the
V3 root node. Otherwise the report names every affected occupancy and field
`root_node_potential_mm` with reason
`ambiguous_v2_layer_root_warm_starts`. Averaging, taking the first entry,
root-fraction weighting, resetting, or broadcasting is prohibited.
