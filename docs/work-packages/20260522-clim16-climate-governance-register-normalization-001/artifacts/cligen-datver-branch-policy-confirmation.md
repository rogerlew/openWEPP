# CLIGEN `datver` Branch Policy Confirmation (CLIM16)

Evidence mode: `Static`
Status: `confirmed`

## Confirmation Statement

Static:
- Legacy/openWEPP policy confirmation is explicit and consistent:
  - accepted override branch: `datver=0.0` -> `iclig=0`
  - accepted CLIGEN branch: `datver>=4.0` -> `iclig=1`
  - rejected branch: `0.0 < datver < 4.0`
- `iclig=1` branch applies `ip *= 0.70` uniformly for accepted nonzero
  `datver` values (`4.0`, `4.3`, `5.3`) unless contrary baseline authority is
  identified.

## Source Evidence (Static)
1. Parser allowlist explicitly includes `0.0`, `4.0`, `4.3`, `5.3`:
- `crates/openwepp-input-contract/src/parsers/climate.rs` (`ALLOWED_DATVERS`).

2. Runtime branch gate is threshold-based (`datver>=4.0`) and does not split
   among `4.0`, `4.3`, `5.3`:
- `crates/openwepp-climate-runtime-adapter/src/lib.rs` (`resolve_iclig`).

3. Runtime `iclig=1` correction factor is explicit and provenance-tagged:
- `crates/openwepp-climate-runtime-adapter/src/lib.rs`
  (`CLIGEN_V4_IP_CORRECTION_FACTOR = 0.70` plus baseline-source comment).

4. CLIM01 detailed specification ratifies the same policy shape and confirms
   this is not a `0.8` rule:
- `docs/work-packages/20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/artifacts/openwepp-climate-model-detailed-specification.md`
  (`datver=0.0`, `datver>=4.0`, `ip*=0.70`, pre-4 nonzero rejected).
