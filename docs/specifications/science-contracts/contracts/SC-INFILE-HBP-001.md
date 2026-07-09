---
contract_id: SC-INFILE-HBP-001
title: Hillslope Binary Pass Input Parser Contract (H<hillslope_id>.hbp)
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.2.3
evidence_mode: Static
last_updated_utc: 2026-07-09T00:00:00Z
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

Implementation authority note:
- openWEPP HBP parser/serialization behavior is still implemented based on
  `/workdir/wepp-forest` HBP contract surfaces (`E-WF-HBP-01`,
  `E-WF-HBP-02`); the ADR-0012 pinned baseline does not replace this contract
  authority.

## 1. Scope and Version Applicability

### 1.1 Scope

This contract governs parser behavior for HBP shard surface `infile-hillslope-binary-pass-hbp`
(`H<hillslope_id>.hbp`) and parse-to-runtime handoff of typed HBP metadata,
directory mappings, and payload-block metadata.

### 1.2 Version/Schema Applicability Matrix

| Case | Input schema | Source-model stance | Simulation-model stance | Evidence |
| --- | --- | --- | --- | --- |
| A | `schema_major=1`, `schema_minor<=1` | Accept. | Parse and validate schema `1.x` daily payload layout; runoff-EVENT payloads carry the minor-gated field set (Section 3a). | `[DIRECT][E-SPEC-HBP-01]`, `[DIRECT][E-WF-HBP-01]` |
| B | `schema_major=2`, `schema_minor<=1` | Accept. | Parse and validate schema `2.x` block-directory layout; day-slice payload encoding shares the Section 3a minor gating. | `[DIRECT][E-SPEC-HBP-01]`, `[DIRECT][E-WF-HBP-01]` |
| C | unsupported major | Reject. | Emit typed unsupported-schema failure. | `[DIRECT][E-WF-HBP-01]` |
| D | supported major but higher unsupported minor (file header **or** per-payload `payload_schema_minor`) | Reject. | Emit typed unsupported-minor failure — a newer payload is rejected loudly, never silently mis-parsed. | `[DIRECT][E-WF-HBP-01]` |

Minor `1` is the ADR-0036/E.2 additive EVENT extension (Section 3a). Minor-`0`
payloads remain fully parseable (the minor-1 fields are absent by definition);
minor-`1` payloads are rejected by pre-E.2 readers via Case D.

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
  - path-resolution mode (`direct` only),
  - dimensional metadata (`hillslope_id`, `nyear`, `npart`, `nofe`, `max_layers`),
  - directory and payload-block metadata arrays,
  - warning surface (must remain empty for valid inputs).
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
| derived `path_resolution` | naming/policy branch | `hbp.path_resolution` | enum | string | 1 | yes | all | direct canonical `.hbp` only | `path_resolution` |
| derived `warnings[]` | naming/policy branch | `hbp.warnings[]` | list | warning[] | 0..n | yes | all | must be empty; no compatibility warnings allowed | `warnings` |

## 3a. Runoff-EVENT Payload Field Block (minor-gated)

The runoff-EVENT payload (`event_kind = 2`) is strict-consumption: every field
must be explicitly read in writer order and the cursor must land exactly at
the payload end (no silent skip of unknown bytes). Fields by
`payload_schema_minor`:

| Canonical symbol | Minor | Units | Type / encoding | Semantics |
| --- | --- | --- | --- | --- |
| `event.duration_seconds` | >=0 | s | f64 | event/storm duration |
| `event.time_of_concentration_hours` | >=0 | h | f64 | reserved/fixed on the direct writer |
| `event.overland_flow_alpha` | >=0 | none | f64 | reserved/fixed on the direct writer |
| `event.peak_runoff_m3_s` | >=0 | m³/s | f64 | **minor >= 1: true volumetric discharge** (`depth_rate × hillslope area`). Minor-0 payloads from the direct writer carried the WB16 depth-rate basis (m/s) under this name — a labeled legacy-basis caveat for minor-0 consumers. |
| `event.total_detachment_kg` / `event.total_deposition_kg` | >=0 | kg | i64 ×1e9 | event totals (true kg per `SC-SED-001` E.1) |
| `event.sediment_concentration_kg_m3[npart]` | >=0 | kg/m³ | u32 count + f64[] | per-class exit concentration; count must equal `npart` (production writes `npart = 5` from minor 1; earlier direct shards wrote `npart = 1`) |
| `event.particle_flow_fraction[npart]` | >=0 | none | u32 count + f64[] | per-class exiting fractions (`SC-SED-001` GAP-SED-007 basis) |
| `event.hourly_runoff_volume_m3[24]` | >=1 | m³ | u32 count (= 24) + f64[24] | hour-integrated runoff volume at the hillslope exit; `Σ = ` event runoff volume (`SC-SED-001#INV-SED-014`) |
| `event.hourly_sediment_mass_kg[24]` | >=1 | kg | u32 count (= 24) + f64[24] | hour-integrated exported sediment mass on the same time base; `Σ = ` event exported mass |
| `event.baseflow_volume_m3` / `gwbfv` | >=0 | m³ | i64 ×1e9 | generated groundwater-reservoir baseflow volume for the day; zero when the reservoir branch is disabled or produces true zero |
| `event.deep_seepage_volume_m3` / `gwdsv` | >=0 | m³ | i64 ×1e9 | generated groundwater-reservoir deep-seepage volume for the day; zero when the reservoir branch is disabled or produces true zero |

Minor-1 fields are inserted **before** the final groundwater/baseflow
`2 × i64` pair, identically in writer and parser (strict consumption makes any
divergence a typed `HBP-E-013`/`HBP-E-015` failure, not a silent shift).
Structural validation for the hourly arrays: count exactly `24`, every element
finite and non-negative. Integral-closure checks against runoff volume /
sediment mass are **run-level intake validation** (Section 8), not
parser-local.
The final two scaled integers are fixed-position groundwater/baseflow fields
owned by `SC-GWBASEFLOW-001`, not skippable padding.

## 3b. Latest-Day NoEvent / Non-Runoff Payload State

The parser must expose the latest represented day as typed state, not as an
optional runoff payload whose absence can be confused with missing data.
Source event kinds:

| Source `event_kind` | Source name | Simulation state | Required payload fields | Watershed routing consequence |
| --- | --- | --- | --- | --- |
| `0` | `NO_EVENT` | `HbpLatestEventState::NoEvent` with `source_event_kind = NoEvent` | `baseflow_volume_m3`, `dissolved_storage_volume_m3`, process-state snapshots | No surface runoff/sediment event; watershed may zero-fill surface runoff/sediment fields only as typed no-event consequence. |
| `1` | `SUBEVENT` | `HbpLatestEventState::NoEvent` with `source_event_kind = Subevent` | `subsurface_flow_depth_m`, `subsurface_flow_volume_m3`, `tile_drainage_depth_m`, `tile_drainage_volume_m3`, `baseflow_volume_m3`, `dissolved_storage_volume_m3`, process-state snapshots | No full runoff/sediment event; current watershed routing may zero-fill surface runoff/sediment fields while preserving parsed non-runoff fields for future consumers. |
| `2` | `EVENT` | `HbpLatestEventState::EventPayload` | Section 3a runoff-EVENT block and process-state snapshots | Existing runoff/sediment watershed routing payload. |

`NO_EVENT` and `SUBEVENT` do not authorize fallback from missing payload bytes or
from a missing pass file. They are valid only after the payload key, required
state snapshots, fixed fields, and non-negative scaled volume/depth fields have
passed parser validation. The latest represented day must overwrite prior state:
an earlier runoff `EVENT` must not remain visible when a later directory record
is `NO_EVENT` or `SUBEVENT`.

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
| derived `path_resolution` | `derived.path_resolution` | `hbp.path_resolution` | `input::hbp` | init | immutable | naming-policy observability | `G-HBP-010` |
| derived `warnings` | `derived.warnings` | `hbp.warnings` | `input::hbp` | init | immutable | reserved warning surface (must be empty) | `G-HBP-010` |
| latest-day payload state | `HbpLatestEventState` | watershed pass inventory | `input::hbp` -> `runner::watershed` | watershed routing | immutable handoff | `PassInventory`, `HillslopeContribution` | `G-HBP-013` |

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
| `D-HBP-001` | Derive canonical direct path-resolution branch from strict `.hbp` input naming. | parse preamble | `C-HBP-001` |
| `D-HBP-002` | Derive schema profile (`schema1x`/`schema2x`) from validated major/minor. | parse header | `C-HBP-002` |
| `D-HBP-003` | Derive record/block counts from validated directory/block tables. | parse finalize | `C-HBP-003` |

Closure hooks:
- `C-HBP-001`: naming policy is deterministic and no-fallback-safe.
- `C-HBP-002`: schema branch selection must be unambiguous and typed.
- `C-HBP-003`: directory/block cardinality and ordering closure must be explicit.

## 7. Validation and Error Taxonomy

| Error ID | Class | Trigger |
| --- | --- | --- |
| `HBP-E-000` | io | direct `.hbp` open/read failure |
| `HBP-E-001` | naming-policy | invalid process HBP name family (must be canonical `H*.hbp`) |
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
| `HBP-E-015` | semantic | minor-1 hourly-surface structural violation (count != 24, non-finite, or negative element) |
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
5. Minor-1 intake closure (run-level, ADR-0036 D4 / `SC-SED-001#INV-SED-014`,
   chain form per `SC-SED-001#INV-SED-016` (e)): the shard-set intake
   validator must check the sediment-side telescoping identity
   `Σ event.hourly_sediment_mass_kg = total_detachment_kg −
   total_deposition_kg` within the declared tolerance, failing closed on
   material violation. On multi-OFE producers the EVENT totals are
   CHAIN-AGGREGATED (Σ across the hillslope's OFEs for the event day) and
   the hourly sediment surface is EXIT-scoped, so the same identity holds
   with the chain-internal inflows telescoped out — one intake rule for
   single- and multi-OFE shards. The
   water-side closure `Σ hourly_runoff_volume_m3 = runvol` is a
   **producer-side** (writer) obligation on the same `runvol` basis the pass
   parquet publishes; a concentration × volume reconstruction is NOT a valid
   intake gate (it would embed the producer's `efflen`/`slplen` geometry).
   Parser-local validation stays structural (`HBP-E-015`).
6. Latest-day no-event intake (run-level, WSHED-W9): watershed pass inventory
   must consume `HbpLatestEventState`, not `Option<HbpLatestEventPayload>`, so
   valid `NO_EVENT`/`SUBEVENT` records are distinguishable from missing or
   malformed payload state. Surface runoff/sediment zeros are valid only when
   derived from a validated typed no-event state.

## 9. Boundary Export Mapping

| Canonical symbol(s) | Internal runtime field | Boundary surface | Boundary field mapping | Notes |
| --- | --- | --- | --- | --- |
| `schema_major/schema_minor` | `hbp.schema_*` | `openwepp.boundary.parser.hbp.v1.metadata` | canonical names + typed profile | schema branch observability |
| `hillslope_id,nyear,npart,nofe,max_layers,simulation_mode` | `hbp.*` | `openwepp.boundary.parser.hbp.v1.dimension` | canonical dimensional fields | downstream allocation/context closure |
| `year_table[]` | `hbp.year_entries[]` | `openwepp.boundary.parser.hbp.v1.year_table` | canonical year-entry mapping | day iterator source of truth |
| `day_directory[]` | `hbp.directory_entries[]` | `openwepp.boundary.parser.hbp.v1.directory` | schema-specific payload locator variants | payload lookup surface |
| `payload_block_table[]` | `hbp.payload_blocks[]` | `openwepp.boundary.parser.hbp.v1.payload_blocks` | schema2 block metadata | absent for schema1 |
| `event.baseflow_volume_m3` / `event.deep_seepage_volume_m3` | `HbpLatestEventPayload.baseflow_volume_m3` / `HbpLatestEventPayload.deep_seepage_volume_m3` | `openwepp.boundary.parser.hbp.v1.latest_event_payload` | `gwbfv`/`gwdsv` pass handoff | `SC-GWBASEFLOW-001`; non-negative scaled volumes |
| `NO_EVENT` / `SUBEVENT` latest-day state | `HbpLatestEventState::NoEvent` | `openwepp.boundary.parser.hbp.v1.latest_event_state` | source event kind plus parsed non-runoff fields | WSHED-W9; no stale prior `EVENT` reuse |
| path/warning branch | `hbp.path_resolution`, `hbp.warnings` | `openwepp.boundary.observability.parser_warnings.v1` | deterministic path observability; warning list must be empty | strict-mode auditability |

## 10. Naming Policy

- Parser input naming is strict-only:
  - accepts only direct canonical `.hbp` naming,
  - rejects `.pass.dat` path inputs,
  - rejects forbidden legacy suffix families (`.pass.hbp`, `.pass.dat.hbp`),
  - enforces full structural and invariant closure,
  - never falls back to text pass files.

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
| `G-HBP-010` | strict naming/path observability closure | path resolver | `HBP-E-001` |
| `G-HBP-011` | minor-1 hourly-surface structural closure (count = 24, finite, non-negative) | runoff-EVENT payload validator | `HBP-E-015` |
| `G-HBP-012` | groundwater/baseflow pass handoff fields are fixed-position scaled non-negative volumes | runoff-EVENT payload validator + watershed pass inventory | `HBP-E-013` or run-level inventory failure |
| `G-HBP-013` | latest represented day state is typed as runoff `EVENT` or validated no-event/non-runoff state; stale prior `EVENT` must not survive later `NO_EVENT`/`SUBEVENT` records | payload validator + parser finalization + watershed pass inventory | `HBP-E-013` or `CLIWAT-E-045` |

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
| `2026-07-09` | `0.2.3` | WSHED-W9 amendment: added typed latest-day no-event/non-runoff parser state, source event-kind preservation, no stale prior-`EVENT` reuse after later `NO_EVENT`/`SUBEVENT`, and `G-HBP-013` watershed inventory handoff authority. |
| `2026-07-09` | `0.2.2` | M-T2 baseflow export closure: named the existing final runoff-EVENT scaled integer pair as `event.baseflow_volume_m3` (`gwbfv`) and `event.deep_seepage_volume_m3` (`gwdsv`) under `SC-GWBASEFLOW-001`, added parser boundary mapping and `G-HBP-012`; layout/order unchanged. |
| `2026-07-04` | `0.2.1` | E.3 chain-form amendment: Section 8.5 intake closure generalized — multi-OFE EVENT totals are chain-aggregated (Σ across OFEs, event day) with the EXIT-scoped hourly sediment surface, keeping the single identity `Σ S_h = tdet − tdep` valid for both single- and multi-OFE shards (`SC-SED-001#INV-SED-016` (e)). |
| `2026-07-04` | `0.2.0` | E.2/ADR-0036 minor-1 EVENT extension: schema/payload minor `<=1` accepted (Section 1.2), new Section 3a runoff-EVENT payload field block (paired `hourly_runoff_volume_m3[24]` m³ + `hourly_sediment_mass_kg[24]` kg before the reserved trailing i64s; `npart = 5` per-class production from minor 1; `peak_runoff_m3_s` true-volumetric from minor 1 with the minor-0 depth-rate caveat labeled), `HBP-E-015`/`G-HBP-011` structural validation, and the Section 8.5 run-level integral-closure intake rule. |
| `2026-05-29` | `0.1.1` | WSHEDIMPL43 amendment: retired `.pass.dat` compatibility derivation and warning branch; parser naming policy is strict canonical `.hbp` only with no ASCII fallback support. |
| `2026-05-22` | `0.1.0` | Initial HBP parser contract authored and aligned to openWEPP parser implementation surface. |
