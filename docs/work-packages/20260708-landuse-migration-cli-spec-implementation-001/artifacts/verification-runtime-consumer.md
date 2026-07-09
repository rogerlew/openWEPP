# Runtime Consumer Verification

Status: PASS.
Evidence mode: Ran.

## Consumer Path

- Producer source: frozen legacy flat management fixture
  `canonical_cropland_nonzero_98_4.man`.
- Migrator: `openwepp-landuse-migrate` via `migrate_path`.
- Output document: canonical `openwepp-management-yaml`, schema version 1,
  `datver: ow-lanuse-1`, default output path `field.man.yaml`.
- Parser/schema type:
  `openwepp-management-schema::ManagementYamlDocument`.
- Runtime intake call site:
  `parse_management_document_from_path(..., ParseMode::Strict)`.
- Downstream consumer:
  `build_hillslope_pl_runtime_surfaces_from_management`.
- Output surface:
  PL schedule route symbols for `k_o`, form `C_d`, `D_r_m`, `lambda`, and
  vegetation `C_d`.

## Negative Proof

The integration test reads the migrated YAML path, not the original `.man`, an
optional report, or a sidecar. The route coefficients asserted in PL surfaces
come from the YAML document's embedded `routing_coefficients`.

## Command

```bash
cargo test --test landuse_migration_cli_contract
```

Result: 1 test passed.
