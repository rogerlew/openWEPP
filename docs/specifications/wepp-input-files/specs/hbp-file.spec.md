# Hillslope Binary Pass Input File Specification (`H<hillslope_id>.hbp`)

## 1. Header metadata
- `spec_id`: `SPEC-INFILE-HBP-001`
- `surface_id`: `infile-hillslope-binary-pass-hbp`
- `title`: `WEPP Hillslope Binary Pass Input Surface (HBP)`
- `status`: `draft`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-22T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence anchors
- [DIRECT] Canonical HBP file-family contract (schema `1.x` and `2.x`, naming, invariants, layout, payload model).
  Evidence: `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`.
- [DIRECT] Watershed reader contract (run-level rules, shard-set constraints, no text fallback).
  Evidence: `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`.
- [DIRECT] Existing Rust reference reader behavior (`wepppyo3`) for schema parsing and invariant checks.
  Evidence: `/workdir/wepppyo3/wepp_interchange/src/hill_hbp.rs`.
- [DIRECT] openWEPP parser implementation target surface.
  Evidence: `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp.rs`.

## 2. Surface scope and applicability
- File surface: `H<hillslope_id>.hbp`.
- Domain: process-mode hillslope binary pass shard consumed by watershed/replay boundaries.
- Applicability: parser surface is active whenever openWEPP consumes HBP shards.

[DIRECT] Process-mode contract requires direct HBP shard consumption and rejects fallback to legacy text pass files.
Evidence: `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` (`Explicit Non-Goals`, `Pass Family Naming`, `HBP2-R08`).

## 3. Version / schema applicability matrix

| Case | File schema | Source behavior | openWEPP parser stance |
| --- | --- | --- | --- |
| A | `schema_major=1`, `schema_minor<=0` | [DIRECT] daily payload region (`schema 1.x`) supported. | accept under strict structural/invariant validation. |
| B | `schema_major=2`, `schema_minor<=0` | [DIRECT] compressed yearly payload blocks (`schema 2.x`) supported. | accept under strict structural/invariant validation. |
| C | unsupported major | [DIRECT] rejected. | typed `UnsupportedSchemaMajor` failure. |
| D | supported major, higher unsupported minor | [DIRECT] rejected. | typed `UnsupportedSchemaMinor` failure. |

[DIRECT] Supported schema rules are explicitly defined in file-family contract and reader implementation.
Evidence: `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`, `/workdir/wepppyo3/wepp_interchange/src/hill_hbp.rs`.

## 4. Naming and pass-family policy

Canonical process-mode HBP naming:

```text
H<hillslope_id>.hbp
```

Invalid naming patterns:

```text
H<hillslope_id>.pass.hbp
H<hillslope_id>.pass.dat.hbp
```

Compatibility path-derivation policy:
- A compatibility reader may derive `H<hillslope_id>.hbp` from legacy deck path `H<hillslope_id>.pass.dat`.
- Missing/invalid derived `.hbp` remains fatal (no text fallback).

[DIRECT] Naming and no-fallback policy are explicit.
Evidence: `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (`File Naming`), `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` (`Pass Family Naming`, `HBP2-R08`).

## 5. Binary layout grammar

### 5.1 Shared prefix

```text
file_header
dimension_unit_block
hillslope_metadata_block
year_table
state_registry_block
day_directory
```

### 5.2 Schema `1.x`

```text
schema1_payload_region
footer
```

### 5.3 Schema `2.x`

```text
payload_block_table
payload_block_region
footer
```

[DIRECT] Layout structure is defined by contract.
Evidence: `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (`File Layout`).

## 6. Field dictionary (parser-governed boundary fields)

| Canonical symbol | Location | Units | Type | Required | Notes / constraints | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- |
| `magic` | file header | bytes | byte[8] | yes | must equal `WFPHBP01` | `hbp.magic` |
| `schema_major` | file header | none | u16 | yes | supported majors: `1`, `2` | `hbp.schema.major` |
| `schema_minor` | file header | none | u16 | yes | must be <= supported minor for major | `hbp.schema.minor` |
| `header_bytes` | file header | bytes | u32 | yes | exact serialized header length | `hbp.header_bytes` |
| `header_crc32c` | file header | checksum | u32 | yes | CRC over header with CRC field zeroed | `hbp.header_crc32c` |
| `hillslope_id` | dimension/unit block | none | u32 | yes | explicit one-based hillslope id | `hbp.hillslope_id` |
| `nyear` | dimension/unit block | count | u32 | yes | year-table cardinality basis | `hbp.nyear` |
| `npart` | dimension/unit block | count | u16 | yes | particle count; must close with particle vector | `hbp.npart` |
| `nofe` | dimension/unit block | count | u16 | yes | OFE axis cardinality for state arrays | `hbp.nofe` |
| `max_layers` | dimension/unit block | count | u16 | yes | soil-layer axis cardinality for layered states | `hbp.max_layers` |
| `simulation_mode` | dimension/unit block | enum | u8 | yes | schema `2.0` requires continuous mode | `hbp.simulation_mode` |
| `year_table[]` | year table | mixed | struct[] | yes | ordered, one-based `sim_year_index` | `hbp.year_table` |
| `state_registry[]` | state registry block | mixed | struct[] | yes | required state ids must be present exactly once | `hbp.state_registry` |
| `day_directory[]` | day directory | mixed | struct[] | yes | strict deterministic ordering | `hbp.day_directory` |
| `payload_block_table[]` | schema `2.x` | mixed | struct[] | conditional | required for `schema_major=2` | `hbp.payload_blocks` |
| `footer` | file footer | mixed | struct | yes | CRC + record-count + magic closure | `hbp.footer` |

## 7. Conditional branches and mode-specific requirements
1. Schema branch:
- `schema_major=1`: payload references direct daily payload offsets.
- `schema_major=2`: payload references raw day slices inside compressed yearly blocks.

2. Schema `2.0` year-table rules:
- `days_in_year = 366`
- `first_julian_day = 1`
- `last_julian_day = 366`
- `single_storm_flag = 0`

3. Schema `2.0` block/day rules:
- one block per represented year,
- block/day slot mapping must be contiguous and gap-free,
- payload codec must be zlib.

[DIRECT] These branches and invariants are contract-governed.
Evidence: `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (`Schema Profiles`, `Year Table`, `Payload Block Table`, `Reader Invariant Catalog`).

## 8. Cross-file consistency and coupling dependencies
1. HBP shard-set coupling:
- parser-local file checks feed run-level shard-set closure checks.

2. Required registry coupling:
- state registry must include all required canonical state IDs expected by downstream routing/replay paths.

3. Year-table/day-directory coupling:
- directory keys must stay within year-table ranges and remain deterministic.

4. Footer and CRC coupling:
- directory/table/file CRC fields are closure-authoritative for load acceptance.

[DIRECT] Coupling rules are explicit in the contracts and reference reader.
Evidence: `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`, `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`, `/workdir/wepppyo3/wepp_interchange/src/hill_hbp.rs`.

## 9. Defaulting and missing-file behavior

| Condition | strict policy | compatibility policy |
| --- | --- | --- |
| invalid HBP name family | typed failure | typed failure |
| direct `.hbp` missing | typed open failure | typed open failure |
| legacy `.pass.dat` path provided | typed naming failure | derive `.hbp` path + warning; missing derived `.hbp` still typed open failure |
| malformed/truncated file | typed format failure | typed format failure |

[DIRECT] No text fallback is permitted on missing/invalid HBP.
Evidence: `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` (`Explicit Non-Goals`, `HBP2-R08`).

## 10. Example references
- Canonical path examples:
  - `H1.hbp`
  - `H24.hbp`
- Invalid family examples:
  - `H1.pass.hbp`
  - `H1.pass.dat.hbp`

## 11. Gap/conflict register

| Gap ID | Statement | Evidence | Status |
| --- | --- | --- | --- |
| `HBP-GAP-001` | Schema `2.x` single-storm encoding is not defined in current contract; single-storm remains schema `1.x`. | [DIRECT] `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (`Schema Profiles`) | `NOTE` |

## 12. Parser-contract handoff map (`SC-INFILE-HBP-001`)

| Contract area | Source requirement | Parser-contract expectation |
| --- | --- | --- |
| Naming policy | Sections 2, 4 | enforce canonical naming, reject forbidden suffixes, compatibility derivation with explicit warning. |
| Schema selection | Sections 3, 5, 7 | strict major/minor gating, deterministic schema branch selection. |
| Structural invariants | Sections 5, 6, 7 | header/year-table/registry/directory/payload/footer closure via typed failures. |
| Cross-file closure handoff | Section 8 | parser emits typed metadata for downstream shard-set orchestration checks. |
| Defaulting behavior | Section 9 | no silent defaulting and no text fallback. |

- `parser_contract_id`: `SC-INFILE-HBP-001`
- `canonical_contract_path`: `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `handoff_status`: `ready-for-contract-authoring`
