---
contract_id: SC-INFILE-HBP-001
title: Hillslope Binary Pass Input Parser Contract (H<hillslope_id>.hbp)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static
last_updated_utc: 2026-05-22T00:00:00Z
---

# SC-INFILE-HBP-001 Hillslope Binary Pass Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Evidence Anchors

- `[DIRECT][E-SPEC-HBP-01]` `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/hbp-file.spec.md` (canonical openWEPP HBP parser-surface specification).
- `[DIRECT][E-WF-HBP-01]` `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (normative file-family format and invariants).
- `[DIRECT][E-WF-HBP-02]` `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` (run-level reader policy and no-fallback rules).
- `[DIRECT][E-WP3-HBP-01]` `/workdir/wepppyo3/wepp_interchange/src/hill_hbp.rs` (reference Rust implementation for schema checks and invariant enforcement).
- `[DIRECT][E-OW-HBP-01]` `/home/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp.rs` (openWEPP parser implementation surface).

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for HBP shard surface `infile-hillslope-binary-pass-hbp`
(`H<hillslope_id>.hbp`) and parse-to-runtime handoff of typed HBP metadata,
directory mappings, and payload-block metadata.

### 1.2 Version/Schema Applicability Matrix

| Case | Input schema | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | `schema_major=1`, `schema_minor<=0` | Accept. | Parse and validate schema `1.x` daily payload layout. | `[DIRECT][E-SPEC-HBP-01]`, `[DIRECT][E-WF-HBP-01]` |
| B | `schema_major=2`, `schema_minor<=0` | Accept. | Parse and validate schema `2.x` block-directory layout. | `[DIRECT][E-SPEC-HBP-01]`, `[DIRECT][E-WF-HBP-01]` |
| C | unsupported major | Reject. | Emit typed unsupported-schema failure. | `[DIRECT][E-WF-HBP-01]` |
| D | supported major but higher unsupported minor | Reject. | Emit typed unsupported-minor failure. | `[DIRECT][E-WF-HBP-01]` |

## 2. Source Grammar and Source-vs-Simulation Model

### 2.1 Source Grammar (Normative Draft)

```ebnf
hbp_file = shared_prefix schema_tail ;

shared_prefix = file_header dimension_unit_block hillslope_metadata_block
                year_table state_registry_block day_directory ;

schema_tail = schema1_tail | schema2_tail ;

schema1_tail = daily_payload_region footer_v1 ;
schema2_tail = payload_block_table payload_block_region footer_v2 ;
```

### 2.2 Two-Layer Model Contract

- Source model preserves file-faithful binary parse surfaces:
  - header fields,
  - year-table entries,
  - state-registry entries,
  - day-directory entries,
  - schema-branch payload locators,
  - footer closures.
- Simulation model normalizes into typed parser outputs:
  - `schema_profile`, `schema_major`, `schema_minor`,
  - path-resolution mode,
  - dimensional metadata (`hillslope_id`, `nyear`, `npart`, `nofe`, `max_layers`),
  - directory and payload-block metadata arrays,
  - typed strict/compat warning surfaces.
- Parser contract is file-local. Run-level shard-set checks remain downstream
  orchestration responsibility.

## 3. Field Specification Table

| Canonical symbol | Source-model field | Simulation-model field | Units | Type | Cardinality | Required | Datver applicability | Default/derivation | openWEPP alias |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `magic` | `header.magic` | `hbp.header.magic` | bytes | byte[8] | 1 | yes | all | must equal `WFPHBP01` | `magic` |
| `schema_major` | `header.schema_major` | `hbp.schema_major` | none | u16 | 1 | yes | all | branch selector | `schema.major` |
| `schema_minor` | `header.schema_minor` | `hbp.schema_minor` | none | u16 | 1 | yes | all | bounded by supported minor per major | `schema.minor` |
| `hillslope_id` | `dimension.hillslope_id` | `hbp.hillslope_id` | none | u32 | 1 | yes | all | must close against optional expected hillslope id | `hillslope_id` |
| `nyear` | `dimension.nyear` | `hbp.nyear` | count | u32 | 1 | yes | all | year-table count basis | `nyear` |
| `npart` | `dimension.npart` | `hbp.npart` | count | u16 | 1 | yes | all | must close with particle metadata count | `npart` |
| `nofe` | `dimension.nofe` | `hbp.nofe` | count | u16 | 1 | yes | all | OFE axis cardinality for state payload checks | `nofe` |
| `max_layers` | `dimension.max_layers` | `hbp.max_layers` | count | u16 | 1 | yes | all | layer axis cardinality for state payload checks | `max_layers` |
| `simulation_mode` | `dimension.simulation_mode` | `hbp.simulation_mode` | enum | u8 | 1 | yes | all | schema `2.0` requires continuous mode | `simulation_mode` |
| `year_table[]` | `year_table.entries[]` | `hbp.year_entries[]` | mixed | struct[] | `nyear` | yes | all | deterministic ordered and internally consistent | `year_entries` |
| `state_registry[]` | `state_registry.entries[]` | `hbp.state_registry` | mixed | struct[] | >=1 | yes | all | required state IDs must be present exactly once | `state_registry` |
| `day_directory[]` | `directory.entries[]` | `hbp.directory_entries[]` | mixed | struct[] | >=1 | yes | all | strict key ordering and year-table closure | `directory_entries` |
| `payload_block_table[]` | `schema2.block_entries[]` | `hbp.payload_blocks[]` | mixed | struct[] | `nyear` when schema2 | conditional | schema2 | omitted for schema1 | `payload_blocks` |
| derived `path_resolution` | naming/policy branch | `hbp.path_resolution` | enum | string | 1 | yes | all | direct or derived from legacy `.pass.dat` | `path_resolution` |
| derived `warnings[]` | strict/compat policy branch | `hbp.warnings[]` | list | warning[] | 0..n | yes | all | `HBP-W-001` on compat path derivation | `warnings` |

## 4. Propagation Map Table

| Source symbol | Parser model field | Runtime state field | Owning module | Phase | Mutability | Downstream consumers | Guard IDs |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `magic` | `header.magic` | `hbp.header.magic` | `input::hbp` | init,watershed,replay | immutable | schema branch gate | `G-HBP-001` |
| `schema_major/minor` | `header.schema_major/minor` | `hbp.schema` | `input::hbp` | init,watershed,replay | immutable | branch-specific decode and invariant checks | `G-HBP-002` |
| `header_crc32c` | `header.header_crc32c` | `hbp.header_crc32c` | `input::hbp` | init | immutable | structural integrity gate | `G-HBP-003` |
| `hillslope_id` | `dimension.hillslope_id` | `hbp.hillslope_id` | `input::hbp` | init,watershed,replay | immutable | per-hillslope routing/replay mapping | `G-HBP-004` |
| `nyear/year_table` | `year_table.*` | `hbp.year_entries` | `input::hbp` | init,watershed,replay | immutable | directory-key closure and day iteration | `G-HBP-005` |
| `state_registry` | `registry.entries` | `hbp.state_registry` | `input::hbp` | init,watershed,replay | immutable | required-state payload validation | `G-HBP-006` |
| `day_directory` | `directory.entries` | `hbp.directory_entries` | `input::hbp` | init,watershed,replay | immutable | payload locator dispatch and ordering closure | `G-HBP-007` |
| `schema2.payload_block_table` | `schema2.payload_blocks` | `hbp.payload_blocks` | `input::hbp` | init,watershed,replay | immutable | decompression and raw-slice validation | `G-HBP-008` |
| `footer fields` | `footer.*` | `hbp.footer_closure` | `input::hbp` | init | immutable | record-count and CRC closure | `G-HBP-009` |
| derived `path_resolution` | `derived.path_resolution` | `hbp.path_resolution` | `input::hbp` | init | immutable | compatibility policy observability | `G-HBP-010` |
| derived `warnings` | `derived.warnings` | `hbp.warnings` | `input::hbp` | init | immutable | strict/compat branch observability | `G-HBP-010` |

## 5. State Ownership and Mutability

- `input::hbp` owns parsed binary surfaces and parser-derived metadata.
- HBP parser outputs are immutable after parse finalization.
- Downstream orchestration modules may read parser outputs and copy values into
  run-owned mutable buffers; they must not mutate parser-owned surfaces.
- Forbidden mutation path: downstream modules mutating `hbp.directory_entries`
  or `hbp.payload_blocks` in place after parser closure.

## 6. Derived Rules and Closure Hooks

| Derivation ID | Rule | Timing | Closure hook |
| --- | --- | --- | --- |
| `D-HBP-001` | Derive strict/compat path-resolution branch from input path family. | parse preamble | `C-HBP-001` |
| `D-HBP-002` | Derive schema profile (`schema1x`/`schema2x`) from validated major/minor. | parse header | `C-HBP-002` |
| `D-HBP-003` | Derive record/block counts from validated directory/block tables. | parse finalize | `C-HBP-003` |

Closure hooks:
- `C-HBP-001`: naming policy is deterministic and no-fallback-safe.
- `C-HBP-002`: schema branch selection must be unambiguous and typed.
- `C-HBP-003`: directory/block cardinality and ordering closure must be explicit.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `HBP-E-000` | io | direct `.hbp` open/read failure (including derived-compat path missing/open failure) |
| `HBP-E-001` | naming-policy | invalid process HBP name family or strict rejection of `.pass.dat` input path |
| `HBP-E-002` | syntax | bad file magic |
| `HBP-E-003` | semantic | unsupported schema major or invalid schema-branch key ordering invariants |
| `HBP-E-004` | semantic | unsupported schema minor for supported major |
| `HBP-E-005` | semantic | unsupported endianness marker |
| `HBP-E-006` | semantic | header/dimension metadata policy mismatch |
| `HBP-E-007` | checksum | header CRC mismatch |
| `HBP-E-008` | semantic | year-table invariants violated |
| `HBP-E-009` | semantic | required state-registry invariants violated |
| `HBP-E-010` | semantic | day-directory invariants violated |
| `HBP-E-011` | semantic | schema2 payload-block invariants violated |
| `HBP-E-012` | checksum | footer/directory/table/file CRC closure mismatch |
| `HBP-E-013` | syntax | truncated payload or malformed payload/state encoding |
| `HBP-E-014` | cross-file | expected hillslope id mismatch |
| `HBP-W-001` | compat-warning | compatibility mode derived `.hbp` path from legacy `.pass.dat` |

No silent fallback to legacy text pass family is permitted.

## 8. Cross-File Consistency Constraints

1. Parser-local HBP acceptance is prerequisite for run-level shard-set closure;
   parser must fail closed on malformed/missing shard bytes.
2. `hillslope_id` must remain consistent with orchestration-provided expected
   hillslope identity when supplied.
3. Required state IDs in payload state snapshots must close against registry
   declarations and required-state catalog.
4. Schema2 day slices must be contiguous, non-overlapping, and complete inside
   each decompressed payload block.

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `schema_major/schema_minor` | `hbp.schema_*` | `openwepp.boundary.parser.hbp.v1.metadata` | canonical names + typed profile | schema branch observability |
| `hillslope_id,nyear,npart,nofe,max_layers,simulation_mode` | `hbp.*` | `openwepp.boundary.parser.hbp.v1.dimension` | canonical dimensional fields | downstream allocation/context closure |
| `year_table[]` | `hbp.year_entries[]` | `openwepp.boundary.parser.hbp.v1.year_table` | canonical year-entry mapping | day iterator source of truth |
| `day_directory[]` | `hbp.directory_entries[]` | `openwepp.boundary.parser.hbp.v1.directory` | schema-specific payload locator variants | payload lookup surface |
| `payload_block_table[]` | `hbp.payload_blocks[]` | `openwepp.boundary.parser.hbp.v1.payload_blocks` | schema2 block metadata | absent for schema1 |
| path/warning branch | `hbp.path_resolution`, `hbp.warnings` | `openwepp.boundary.observability.parser_warnings.v1` | deterministic warning IDs | strict/compat auditability |

## 10. Compatibility Policy

- Strict mode:
  - accepts only direct canonical `.hbp` naming,
  - rejects `.pass.dat` path inputs,
  - enforces full structural and invariant closure,
  - never falls back to text pass files.
- Compatibility mode:
  - may derive `.hbp` from `.pass.dat` path with `HBP-W-001`,
  - still requires derived `.hbp` parse success,
  - still fails closed on malformed or missing `.hbp` bytes,
  - does not enable text-pass fallback.

## 11. Guard Map and Invariant Linkage

| Guard ID | Invariant / rule | Enforcement path | Failure behavior |
| --- | --- | --- | --- |
| `G-HBP-001` | canonical magic and naming-policy closure | preamble + header parse | `HBP-E-001`/`HBP-E-002` |
| `G-HBP-002` | supported schema major/minor closure | header parse | `HBP-E-003`/`HBP-E-004` |
| `G-HBP-003` | header structural integrity and checksum closure | header parse | `HBP-E-005`/`HBP-E-006`/`HBP-E-007` |
| `G-HBP-004` | hillslope-id closure against optional expected ID | dimension parse/finalize | `HBP-E-014` |
| `G-HBP-005` | year-table cardinality/order/day-range closure | year-table validator | `HBP-E-008` |
| `G-HBP-006` | required state-registry schema closure | state-registry validator | `HBP-E-009` |
| `G-HBP-007` | day-directory ordering/key/payload-reference closure | directory validator | `HBP-E-010` |
| `G-HBP-008` | schema2 payload-block and day-slice closure | block-table + slice validators | `HBP-E-011`/`HBP-E-013` |
| `G-HBP-009` | footer and file-level CRC closure | footer validator | `HBP-E-012` |
| `G-HBP-010` | compatibility derivation observability closure | path resolver | `HBP-W-001` |

## 12. Legacy Symbol Continuity and Alias Map

Canonical symbols remain authoritative and unchanged:
`hillslope_id`, `nyear`, `npart`, `nofe`, `max_layers`, `simulation_mode`,
`year_table`, `day_directory`, `payload_block_table`.

openWEPP boundary names are aliases only (Section 3).

## 13. HOLD Gap Register

| Gap ID | Statement | Evidence | Disposition |
| --- | --- | --- | --- |
| `HBP-GAP-001` | Schema `2.x` single-storm encoding remains out-of-scope in current upstream contract. | `[DIRECT][E-WF-HBP-01]` | `NOTE (non-blocking parser-surface scope)` |

## 14. Revision History

| Date UTC | Version | Change |
| --- | --- | --- |
| `2026-05-22` | `0.1.0` | Initial HBP parser contract authored and aligned to openWEPP parser implementation surface. |
