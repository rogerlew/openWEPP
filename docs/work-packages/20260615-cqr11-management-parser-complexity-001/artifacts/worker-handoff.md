# Worker Handoff

Status: complete-with-warnings.

Static: CQR11 is ready for package commit/push and tracker update.

Package path:

- `docs/work-packages/20260615-cqr11-management-parser-complexity-001/`

Changed files for package commit:

- `crates/openwepp-input-contract/src/parsers/management.rs`
- `tests/integration/infile_management_parser_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260615-cqr11-management-parser-complexity-001/**`

Do not stage:

- pre-existing unrelated `AGENTS.md` modification.

Final scoped metric:

- `parse_yearly_perennial`: before CRAP `1406.0`, after CRAP `4.0`.

Follow-up candidates:

- continue CQR burn-down with the next ranked management parser targets;
- target-file coverage remains below the science-tier threshold;
- pre-existing out-of-scope rows above `30` include
  `parse_yearly_annual_fallow`, `parse_operation_section`,
  `ManagementParseError::fmt`, `parse_contour_section`,
  `parse_management_from_str`, and `parse_initial_section`.

First actionable next step after package push:

- update `docs/work-packages/cqr-burndown-execplan.md` for CQR11 with pushed
  commit SHA, branch `main`, date `2026-06-15`, package path, and final CRAP
  `4.0`, then commit and push the tracker update.
