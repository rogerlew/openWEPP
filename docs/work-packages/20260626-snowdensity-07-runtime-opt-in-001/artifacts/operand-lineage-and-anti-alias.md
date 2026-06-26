# Operand Lineage And Anti-Alias Evidence

Evidence class: Static + Ran.

## Lineage

The typed snow partition computes CoE snow coupling first. That result remains
the authority for:

- runtime SWE
- signed snow coupling `S`
- raw melt
- redistributed melt
- routed melt
- snowpack SWE loss
- post-winter rain
- albedo state

The SNOWDENSITY-07 opt-in density update receives the CoE SWE result as
`boundary_swe_after_m` and force-normalizes the density state to that SWE. The
opt-in may change only:

- runtime physical snow depth
- runtime bulk snow density

The CoE boundary carry remains separate:

- `coe_boundary_depth_after_m`
- `coe_boundary_density_after_kg_m3`
- `coe_boundary_settle_day_count_after`

## Anti-Alias Proof

`snowdensity07_opt_in_changes_only_runtime_density_depth_surface` compares
`legacy_wepp` and `physics_bulk_density_compaction_v1` using the same cold-pack
fixture. It asserts equality for SWE, signed snow coupling, raw/redistributed/
routed melt, SWE loss, post-winter rain, and albedo state, while requiring
distinct opt-in runtime depth and density.

The same test asserts the opt-in CoE boundary depth/density/settle are equal to
the legacy CoE runtime result, proving the density publication surface is not
fed back into the next CoE melt boundary.

`snowdensity07_r4g_projects_runtime_and_boundary_carry_without_compat_edge`
asserts R4G projects both the opt-in runtime state and separate CoE boundary
carry through state mutation, downstream operands, shadow projection, and
runtime carry with `compatibility_edge_invocation_count = 0`.

`snowdensity07_surface_driven_publication_path_remains_default_disabled`
asserts the direct publication builder uses `LegacyWepp` and does not contain
`PhysicsBulkDensityCompactionV1`.

