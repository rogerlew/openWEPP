# PL07 Fixture Runtime Projection Coverage Matrix

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL07 requires fixture-backed integration evidence that `.man` parser outputs fully project into PL runtime schedule, growth, and decomposition/resup symbol families.

Ran:
- Integration tests executed from `tests/integration/parser_runtime_seam_integration.rs` and passed.

| coverage_id | fixture class | runtime family coverage | test anchor | result |
|---|---|---|---|---|
| `PL07-FIX-001` | `canonical_cropland_nonzero_98_4.man` | full schedule/growth/decomp family projection and merged seed aliases | `management_fixture_projects_full_pl_runtime_surface_families` | `pass` |
| `PL07-FIX-002` | `canonical_rotation_nonzero_98_4.man` | full multi-slot rotation projection across schedule/growth/decomp families with slot-count closure assertions | `management_rotation_fixture_projects_schedule_growth_and_decomp_runtime_surface_families` | `pass` |
| `PL07-FIX-003` | canonical 98.4 mutation set | typed reject-path coverage for required PL seam inputs (`HS-RUNTIME-E-036..045`) | `management_runtime_surface_rejects_*` tests | `pass` |

## Coverage Notes

- Family-completeness checks are centralized in:
  - `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:821`
  - `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:848`
  - `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:899`
- Fixture path binding for real `.man` files:
  - `/home/workdir/openWEPP/tests/integration/parser_runtime_seam_integration.rs:809`
