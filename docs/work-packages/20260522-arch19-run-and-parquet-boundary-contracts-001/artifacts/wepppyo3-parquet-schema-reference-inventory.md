# WEPPpyo3 Parquet Schema Reference Inventory

Static: inventory derived from `/workdir/wepppyo3` source inspection.
Ran: repository SHA capture only.
Status: `complete`.

## Provenance Pins

- `/workdir/wepppyo3` HEAD: `6c92e3fa70e45838e2a4778ee70ceae88db8e42b`
- `/workdir/wepp-forest` HEAD (HBP contract reference context):
  `028feb2317a35a9ad3e578c0e5798631fc0e61bd`

## Core Writer and Schema Infrastructure

| surface | file | notes |
|---|---|---|
| `ParquetSink` writer | `/workdir/wepppyo3/wepp_interchange/src/parquet.rs` | snappy-compressed V2 parquet, temp-file then rename/copy fallback, per-chunk row-group writes |
| version metadata normalizer | `/workdir/wepppyo3/wepp_interchange/src/schema.rs::schema_with_version` | injects `dataset_version*` + `schema_version` keys |
| py boundary exports | `/workdir/wepppyo3/wepp_interchange/src/lib.rs` | exposes `*_to_parquet` entry points and summary payload shape |

## Dataset Family Inventory

| family_id | output API | schema source | artifacts |
|---|---|---|---|
| `INV-PRQ-001` | `watershed_pass_to_parquet` | `pass.rs` (`build_event_schema`, `build_metadata_schema`) + `schema_with_version` | two files: events parquet, metadata parquet |
| `INV-PRQ-002` | `watershed_chanwb_to_parquet` | `schema.rs::watershed_chanwb_schema` | single `chanwb` parquet |
| `INV-PRQ-003` | `watershed_chnwb_to_parquet` | `schema.rs::watershed_chnwb_schema` | single `chnwb` parquet |
| `INV-PRQ-004` | `watershed_ebe_to_parquet` | `schema.rs::watershed_ebe_schema` | single `ebe` parquet |
| `INV-PRQ-005` | `watershed_soil_to_parquet` | `soil.rs::soil_schema` + `schema_with_version` | single `soil` parquet |
| `INV-PRQ-006` | `watershed_loss_to_parquet` | `loss.rs` (`hill_schemas`, `chn_schemas`, `out_schemas`, `class_schemas`) + `schema_with_version` | eight files (`average_*`, `all_years_*`) |
| `INV-PRQ-007` | `watershed_chan_peak_to_parquet` | `chan_peak.rs::chan_peak_schema` + `schema_with_version` | single `chan_peak` parquet |

## Schema Governance-Relevant Behaviors

| behavior_id | observed behavior | source |
|---|---|---|
| `INV-PRQ-B-001` | Compression support is effectively `snappy`-only at py API boundary; unsupported values return typed errors. | `lib.rs::ensure_snappy` |
| `INV-PRQ-B-002` | Empty datasets still emit schema-valid parquet files using empty chunks. | `parquet.rs::empty_chunk`, dataset writers |
| `INV-PRQ-B-003` | Writer path is fail-closed with typed parse/io errors; invalid rows are not silently defaulted. | dataset parser modules + `errors.rs` |
| `INV-PRQ-B-004` | Pass and loss schemas include extra dataset metadata (`version`, `nhill`, `npart`, `table`, `average_years`) in addition to core version keys. | `pass.rs`, `loss.rs` |
| `INV-PRQ-B-005` | Hillslope HBP parser surface is columnized through `hill_hbp.rs::hillslope_hbp_to_columns`, supplying canonical runoff/sediment symbols consumed by pass/event schema families. | `hill_hbp.rs`, `hill_pass.rs`, `schema.rs::hill_pass_schema` |

## openWEPP Authority Mapping Statements

| inventory item | openWEPP-owned authority statement |
|---|---|
| `INV-PRQ-001..007` | These imported families are the only accepted parquet output surface set for ARCH19 boundary authority. |
| `INV-PRQ-B-001` | Contract-level rule: snappy remains required until an openWEPP-approved schema/compression evolution decision updates boundary authority. |
| `INV-PRQ-B-002` | Contract-level rule: empty-output scenarios must remain schema-valid, not missing-file defaults. |
| `INV-PRQ-B-003` | Contract-level rule: parse/shape violations are typed failures, never silent replacement values. |
| `INV-PRQ-B-004` | Contract-level rule: version and dataset metadata keys are mandatory promotion checks. |
| `INV-PRQ-B-005` | Contract-level rule: HBP schema branch and warning semantics from ARCH18 must remain preserved when mapped into parquet-facing fields. |
