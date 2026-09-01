# V36 geometry-complete solver pre-implementation red

Evidence state: `EXPECTED RED — RETAINED`

## Retained r88 field audit

Log: `/tmp/wghl_001d_v35_64m_r88_field_audit.log`

SHA-256: `55a904fbbb35126a00f50af60ba3c7d296e3298c575a784cbd3eedaa7f24ec65`

Result: `FAIL` after `312.42 s` test execution (`5:24.80` command wall time)
on exact `1800..1860 s` support. Lane-1 `layer.thickness_m` changed from bits
`4569208177783694401` (`3.06254815664577839e-3 m`) to
`4569208162027237604` (`3.06254132337190239e-3 m`), independently reported
delta `6.833273876e-9 m`. Finalization LSE/boundary were closed while Stage 3
remained rejected and the shared physical-evaluation budget exhausted. This is
failure evidence, not canonical qualification.

## Contract authority gate

Ran:

```text
nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v36_contract_binds_geometry_complete_physical_solver --no-capture
```

Result: `PASS`, run `10d008e8-c020-4356-a5fd-62887c99c9f5`, one passed,
fifteen filtered. Assertions bind the one density coordinate, canonical
`z=I/rho`, physical `R_rho`, generalized `R_W/R_H/R_rho/R_E/R_T`, exact
branch authority with evolving rho/z, the unchanged shared budget, v35 receipt
stabilization/authentic finalization, and all prohibited shortcuts.

## Source-bound implementation gate

Ran:

```text
nix develop -c cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v36_geometry_complete_production_seams_are_required --no-capture
```

Result: `EXPECTED FAIL`, exit `100`, run
`724f8809-ac4b-435f-8956-5c151c723577`, zero passed, one failed, fifteen
filtered. Production lacks three required seams:

- `CoveredTerminalDensityGeometryCoordinateV1`;
- `covered_terminal_density_geometry_residual_evaluate_v1`;
- `r_rho_kg_m3`.

It also lacks five required behavior vectors covering `I/rho`, physical
`R_rho` rather than map difference, branch-stable evolving rho/z, shared-budget
plus v35 stabilization, and branch/interpolation/repair/bypass refusal.

Pre-red log: `/tmp/wghl_v36_geometry_complete_pre_red.log`

SHA-256: `27e6c6780aeebd126344958b50c970cd51f7ef64fc1d66d5b3f8847488e60fe0`

Snapshot SHA-256 values:

- contract: `d9c99dc59cb63e374924e89b17c2213f804e092196c0d7aeafbb7d734ee4a55a`
- package: `6e6120df50d7e8ffc21d3ec4e8d16c415306c4c3b48582d4aefde554c862e0de`
- contract test: `608b9fb0fab09d9371de72ca69a10b8a88a31f496f0c13c26db8676d7b78ebe4`
