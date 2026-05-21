# Peridot 2023.3 Slope Format Assessment

Evidence mode: `Static`
Date: 2026-05-21

## 1. Assessment Goal

Assess compatibility delta between current openWEPP slope parser/contract and
Peridot-generated `2023.3` hillslope `.slp` files, then define implementation
scope for INIMPL08.

## 2. Direct Evidence

1. Peridot hillslope writer emits `datver=2023.3`, fixed `nofes=1`, a
   three-value metadata line, and comma-delimited slope-point pairs:
   - `[DIRECT]` `/workdir/peridot/src/watershed_abstraction/flowpath.rs:217-246`
2. wepppy slope helper explicitly recognizes `2023*` first-line variant,
   expects one OFE, parses line 3 as `azm fwidth z0`, and removes commas before
   parsing point pairs:
   - `[DIRECT]` `/workdir/wepppy/wepppy/topo/watershed_abstraction/slope_file.py:79-99`
3. Current openWEPP slope parser supports only the `97.5`/legacy grammar branch,
   parses OFE line 3 as exactly `azm fwidth`, line 4 as `nslpts slplen`, and
   does not normalize comma tokens in tokenizer:
   - `[DIRECT]` `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/slope.rs:254-347`
   - `[DIRECT]` `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/slope.rs:658-683`
4. Current openWEPP slope spec/contract still carries undispositioned HOLD gap
   for `2023*` extension acceptance:
   - `[DIRECT]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md`
   - `[DIRECT]` `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
5. Peridot channel bundle format is separately versioned (`2025.8`) and should
   remain out of slope-hillslope parser scope for this package:
   - `[DIRECT]` `/workdir/peridot/src/watershed_abstraction/flowpath_collection.rs:652-695`

## 3. Compatibility Delta (Current -> Required)

### 3.1 Format-level delta

Current accepted canonical shape (openWEPP draft):
- `97.5`
- `nelem`
- per-OFE `azm fwidth`
- per-OFE `nslpts slplen`
- `(xinput slpinp){nslpts}`

Peridot `2023.3` shape:
- `2023.3`
- `nofes` (currently `1`)
- `azm fwidth elevation`
- `npts length`
- one line of comma-delimited normalized pairs, e.g. `0.0000, 0.0123 ...`

### 3.2 Parser-failure points in current implementation

1. Strict mode rejects non-`97.5` datver; `2023.3` is rejected.
2. OFE header arity mismatch: current parser reads line 3 as two tokens only,
   so `elevation` shifts token positions and corrupts `nslpts` parsing.
3. Comma tokens (`0.0000,`) are not sanitized; numeric parse fails.

## 4. Recommended Normative Policy for INIMPL08

1. Treat Peridot `2023.3` hillslope `.slp` as an accepted first-party branch of
   `infile-slope-slp`.
2. Keep legacy/canonical symbol continuity; add explicit alias mapping for
   Peridot-only metadata (`elevation`) rather than replacing canonical symbols.
3. Keep channel `2025.8` and `.slps` bundle parsing out of this package scope.
4. Preserve typed error behavior and invariant guards; no parser-side silent
   correction.

## 5. Proposed Guard/Invariant Additions for 2023.3 Branch

1. `datver == 2023.3` branch selector.
2. `nofes >= 1` and explicit policy on whether `nofes != 1` is rejected or
   future-compatible.
3. metadata-line arity guard (`azm fwidth elevation`).
4. pair-line cardinality guard (`2 * npts` values after comma normalization).
5. normalized-distance guard (`xinput` values monotone, first `0`, terminal `1`
   within tolerance) for Peridot branch unless spec chooses dual-mode behavior.

## 6. File-Change Surface for INIMPL08

1. Spec:
   - `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md`
2. Contract:
   - `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
3. Parser:
   - `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/slope.rs`
4. Tests/fixtures:
   - `/home/workdir/openWEPP/tests/integration/infile_slope_parser_contract.rs`
   - `/home/workdir/openWEPP/tests/fixtures/infile/slope/**`

## 7. Key Decisions to Lock During Execution

1. Strict-mode acceptance policy for `2023.3` (recommended: accept in strict).
2. Whether Peridot `elevation` is required metadata in simulation model or
   preserved as optional provenance field.
3. Whether `2023.x` (wildcard) is accepted, or exact `2023.3` only.
4. Whether `nofes > 1` is rejected for now or accepted with repeated
   three-field metadata blocks.

## 8. Readiness Verdict

[INFERENCE] INIMPL08 is ready to execute. Evidence is sufficient to scope a
single package that updates spec + contract + parser + tests without expanding
into channel/watershed parser surfaces.
