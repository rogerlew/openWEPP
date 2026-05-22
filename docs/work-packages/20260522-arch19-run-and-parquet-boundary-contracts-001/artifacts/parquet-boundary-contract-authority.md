# Parquet Boundary Contract Authority

Static: parquet boundary authority authored from openWEPP ADRs/contracts and
`/workdir/wepppyo3` writer/schema surfaces.
Ran: none.
Status: `complete-with-hold`.
Boundary ID: `openwepp.boundary.parquet.v1`.

## Scope

This contract defines openWEPP authority over parquet output boundary behavior,
including schema-governance rules for imported `wepppyo3` interchange schemas.

In scope:
- governance rules for schema versioning, metadata, and evolution control,
- allowed writer surfaces and output artifact families,
- authority split between openWEPP and `wepppyo3` implementation details,
- closure requirements for parser/runtime -> parquet propagation.

Out of scope:
- direct Rust implementation in openWEPP crates for parquet emission,
- rewriting `wepppyo3` schema definitions in ARCH19.

## Authority Stack

| precedence | authority | role |
|---|---|---|
| 1 | `/home/workdir/openWEPP/docs/decisions/0005-parquet-via-wepppyo3-interchange.md` | ratifies schema inheritance from `wepppy`/`wepppyo3` |
| 2 | `/home/workdir/openWEPP/docs/contracts/README.md` | declares parquet as interface contract surface |
| 3 | `/home/workdir/openWEPP/docs/architecture/README.md` | defines hillslope/watershed parquet as process-boundary outputs |
| 4 | `/home/workdir/openWEPP/docs/work-packages/20260522-arch18-hbp-authority-and-convergence-closure-001/artifacts/arch19-parquet-schema-handoff.md` | required HBP->parquet carry-forward constraints |
| 5 | `/workdir/wepppyo3/wepp_interchange/src/{parquet.rs,schema.rs,pass.rs,chanwb.rs,chnwb.rs,ebe.rs,soil.rs,loss.rs,chan_peak.rs,lib.rs}` | implemented writer/schema surfaces inventoried by ARCH19 |

## Canonical Governance Rules

| rule_id | rule |
|---|---|
| `PRQ-R-001` | openWEPP owns boundary contract authority; imported schemas are implementation references, not authority replacement. |
| `PRQ-R-002` | Schema changes that rename/remove/retype columns require coordinated change across openWEPP contract docs and `wepppyo3` writer/schema code with explicit version metadata update. |
| `PRQ-R-003` | Boundary metadata keys from `schema_with_version` (`dataset_version`, `dataset_version_major`, `dataset_version_minor`, `schema_version`) are required for promotable outputs. |
| `PRQ-R-004` | Compression is constrained to `snappy` for currently supported writer surfaces; unsupported compression values are typed errors. |
| `PRQ-R-005` | Writer behavior must remain fail-closed and atomic-at-destination (`*.tmp` then rename/copy fallback), with no silent truncation/drop on parse or schema errors. |
| `PRQ-R-006` | Schema1/schema2 HBP branch distinctions and compatibility warnings required by ARCH18 handoff must remain representable in parquet-facing payload columns/metadata. |
| `PRQ-R-007` | OpenWEPP may not declare parity closure based only on watershed/hourly parquet deltas; comparator confidence-tier policy from ADR-0011 applies. |

## Allowed Surface Families (Current Inventory)

| family_id | producer function family | output kind |
|---|---|---|
| `PRQ-F-001` | `watershed_pass_to_parquet` | watershed pass events + metadata parquet pair |
| `PRQ-F-002` | `watershed_chanwb_to_parquet` | watershed channel water-balance parquet |
| `PRQ-F-003` | `watershed_chnwb_to_parquet` | watershed chnwb parquet |
| `PRQ-F-004` | `watershed_ebe_to_parquet` | watershed event-by-event parquet |
| `PRQ-F-005` | `watershed_soil_to_parquet` | watershed soil parquet |
| `PRQ-F-006` | `watershed_loss_to_parquet` | watershed loss parquet family (8 tables) |
| `PRQ-F-007` | `watershed_chan_peak_to_parquet` | watershed channel-peak parquet |

## Required Metadata and Version Governance

| metadata key | source surface | governance meaning |
|---|---|---|
| `dataset_version` | `schema::schema_with_version` | canonical dataset semantic version string |
| `dataset_version_major` | `schema::schema_with_version` | major version axis for compatibility gating |
| `dataset_version_minor` | `schema::schema_with_version` | minor version axis for additive evolution |
| `schema_version` | `schema::schema_with_version` | schema version broadcast key used in downstream tooling |
| table-specific keys (`version`, `nhill`, `max_years`, `begin_year`, `npart`, `table`, `average_years`) | `pass.rs`, `loss.rs` | dataset-family specific closure metadata |

## Canonical Alias/Continuity Policy

Canonical WEPP symbols remain primary for contractual meaning (`runoff`,
`runvol`, `sbrunf`, `peakro`, etc.). Parquet column names in imported schemas
are boundary aliases and must remain explicitly mapped in contract artifacts;
symbol substitution without alias mapping is non-compliant.

## HOLD Register

| hold_id | gap | impact | closure owner |
|---|---|---|---|
| `PRQ-HOLD-001` | openWEPP canonical parquet boundary contract is authored, but no openWEPP-local executable writer pipeline currently enforces these rules end-to-end. | governance-only closure; implementation closure pending | openWEPP runtime/output owners |
| `PRQ-HOLD-002` | No openWEPP-owned automated gate currently validates full parquet schema metadata contract against produced files in this repo. | schema drift can evade local CI | follow-on validation package owner |
| `PRQ-HOLD-003` | Cross-file `.run` selector -> parquet dataset family routing is not yet encoded in a typed openWEPP run model. | boundary completeness remains partial | run-boundary follow-on owner |

Promotion state remains `HOLD` until `PRQ-HOLD-*` closure evidence exists.
