# MOFE08 CLIGEN Compatibility Implementation Report

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Scope implemented in climate input parser/contract/test surfaces only.
- No kernel process-physics equations or routing logic were modified.

Implemented changes:
1. Climate parser contract authority
- Updated `SC-INFILE-CLIMATE-001` (`contract_version: 0.1.6`) to accept
  `5.3 <= datver < 5.4` and canonicalize parser output to `5.3`.
- Added openWEPP cross-reference evidence anchor to
  `/workdir/jimf-cligen532/README.md` lineage guidance.

2. Climate spec cross-reference
- Updated `climate-file.spec.md` to document `5.3` family interpretation and
  cross-reference CLIGEN lineage guidance path.

3. Parser implementation
- `crates/openwepp-input-contract/src/parsers/climate.rs` now applies datver
  canonicalization policy:
  - exact: `0.0`, `4.0`, `4.3`
  - family: any `5.3 <= datver < 5.4` normalized to canonical `5.3`
  - all others: typed `UnsupportedDatver`.

4. Contract-derived tests/fixtures
- Added fixture `tests/fixtures/infile/climate/datver_5_323.cli`.
- Added climate integration tests for:
  - strict acceptance/canonicalization of `5.323`.
  - strict rejection of `5.4` boundary.

5. Cross-repo guidance
- Updated `/workdir/jimf-cligen532/README.md` with explicit policy that
  `.cli` format-breaking changes must bump minor version.
