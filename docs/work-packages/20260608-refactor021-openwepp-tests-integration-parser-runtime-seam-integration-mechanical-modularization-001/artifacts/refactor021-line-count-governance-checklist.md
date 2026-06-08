# REFACTOR021 Line-Count Governance Checklist

Status: complete
Evidence mode: Static/Ran

Static:
- Pre-split source: `tests/integration/parser_runtime_seam_integration.rs` at 2,631 lines.
- `.rs` warning profile requires decomposition above 2,000 lines for maintainability.
- Post-split counts:
  - `tests/integration/parser_runtime_seam_integration.rs`: 6 lines (facade)
  - `tests/integration/parser_runtime_seam_integration/common.rs`: 752 lines
  - `tests/integration/parser_runtime_seam_integration/runtime_projection_and_management.rs`: 860 lines
  - `tests/integration/parser_runtime_seam_integration/plant_contracts.rs`: 1018 lines
- Total split-source lines: 2,636.

Ran:
- 2026-06-08T23:39:12Z: `wc -l` captured for all touched `.rs` files.

Governance outcome:
- Main module reduced below line-count threshold and split into cohesive modules.
- All resulting `.rs` files are below 2000 lines and no module approaches 3000-line threshold.
