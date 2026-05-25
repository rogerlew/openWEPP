# MOFE07 Parser Compatibility Implementation Report

Status: complete
Evidence mode: mixed (Static + Ran)

Static:
- Scope implemented in `openwepp-input-contract` parser surfaces only.
- No kernel process-physics or routing formulas were modified.

Implemented changes:
1. Slope parser compatibility (`crates/openwepp-input-contract/src/parsers/slope.rs`)
- Added compatibility fallback for legacy shared-geometry MOFE slope form
  (`azm,fwidth` declared once and reused across OFEs).
- Preserved strict-mode rejection posture for shared-geometry form.
- Refactored OFE shape parsing into reusable helper paths to avoid divergent
  validation logic.

2. Soil parser compatibility (`crates/openwepp-input-contract/src/parsers/soil.rs`)
- Added compatibility tokenization for quoted `7778` headers with embedded
  whitespace in `slid`/`texid`.
- Added compatibility normalization for legacy quoted `7778` headers that omit
  trailing `avke` (`avke := 0.0`).
- Added compatibility acceptance for per-OFE restrictive rows (legacy placement)
  when all per-OFE restrictive rows are identical; normalized to one
  profile-level restrictive row.
- Preserved strict-mode rejection for quoted/per-OFE restrictive compatibility
  forms.

3. Contract authority updates
- `SC-INFILE-SLOPE-001` updated for shared-geometry compatibility form.
- `SC-INFILE-SOIL-001` updated for quoted-header/omitted-`avke` compatibility
  and per-OFE restrictive-row compatibility normalization.

Ran:
- Runtime replay with generated `p324.run` in `/tmp/openwepp_mofe324_semantic_parity/runs`
  moved failure past slope/soil parsing to climate parsing
  (`unsupported datver '5.323'`), confirming scoped slope/soil blockers were
  removed.
