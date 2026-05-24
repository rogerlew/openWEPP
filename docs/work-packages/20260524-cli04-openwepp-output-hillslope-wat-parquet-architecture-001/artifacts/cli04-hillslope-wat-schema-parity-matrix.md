# CLI04 Hillslope WAT Schema Parity Matrix

Status: completed
Evidence mode: Static + Ran

## Static
Authority references used for parity:
- `/home/workdir/wepppy/wepppy/wepp/interchange/hill_wat_interchange.py`
- `/home/workdir/wepppy/wepppy/wepp/interchange/versioning.py`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`

Dataset metadata parity requirements:

| Key | Expected value source | openWEPP writer behavior | Parity |
| --- | --- | --- | --- |
| `dataset_version` | WEPPpy `schema_with_version` | set from `InterchangeVersion` (`major.minor`) | pass |
| `dataset_version_major` | WEPPpy `schema_with_version` | set from `InterchangeVersion.major` | pass |
| `dataset_version_minor` | WEPPpy `schema_with_version` | set from `InterchangeVersion.minor` | pass |
| `schema_version` | WEPPpy `schema_with_version` | set from `InterchangeVersion.major` | pass |

Field-level parity matrix:

| Field | Type | Nullable | Units metadata | Description metadata | Parity status |
| --- | --- | --- | --- | --- | --- |
| `wepp_id` | `Int32` | no | n/a | n/a | pass |
| `ofe_id` | `Int16` | no | n/a | n/a | pass |
| `year` | `Int16` | no | n/a | n/a | pass |
| `sim_day_index` | `Int32` | no | n/a | `1-indexed simulation day` | pass |
| `julian` | `Int16` | no | n/a | n/a | pass |
| `month` | `Int8` | no | n/a | n/a | pass |
| `day_of_month` | `Int8` | no | n/a | n/a | pass |
| `water_year` | `Int16` | no | n/a | n/a | pass |
| `OFE` | `Int16` | no | n/a | n/a | pass |
| `P` | `Float64` | no | `mm` | `Precipitation` | pass |
| `RM` | `Float64` | no | `mm` | `Rainfall+Irrigation+Snowmelt` | pass |
| `Q` | `Float64` | no | `mm` | `Daily runoff over eff length` | pass |
| `Ep` | `Float64` | no | `mm` | `Plant transpiration` | pass |
| `Es` | `Float64` | no | `mm` | `Soil evaporation` | pass |
| `Er` | `Float64` | no | `mm` | `Residue evaporation` | pass |
| `Dp` | `Float64` | no | `mm` | `Deep percolation` | pass |
| `UpStrmQ` | `Float64` | no | `mm` | `Runon added to OFE` | pass |
| `SubRIn` | `Float64` | no | `mm` | `Subsurface runon added to OFE` | pass |
| `latqcc` | `Float64` | no | `mm` | `Lateral subsurface flow` | pass |
| `Total-Soil Water` | `Float64` | no | `mm` | `Unfrozen water in soil profile` | pass |
| `frozwt` | `Float64` | no | `mm` | `Frozen water in soil profile` | pass |
| `Snow-Water` | `Float64` | no | `mm` | `Water in surface snow` | pass |
| `QOFE` | `Float64` | no | `mm` | `Daily runoff scaled to single OFE` | pass |
| `Tile` | `Float64` | no | `mm` | `Tile drainage` | pass |
| `Irr` | `Float64` | no | `mm` | `Irrigation` | pass |
| `Area` | `Float64` | no | `m^2` | `Area that depths apply over` | pass |
| `SoilWaterTotal` | `Float64` | yes | `mm` | optional producer-authoritative term | pass |
| `ProfileDepth` | `Float64` | yes | `mm` | optional producer-authoritative term | pass |
| `ProfilePorosityCap` | `Float64` | yes | `mm` | optional producer-authoritative term | pass |
| `ProfileFCStore` | `Float64` | yes | `mm` | optional producer-authoritative term | pass |
| `ProfileWPStore` | `Float64` | yes | `mm` | optional producer-authoritative term | pass |
| `InterceptionStorage` | `Float64` | yes | `mm` | optional producer-authoritative term | pass |

Notes:
- CLI04 preserves the post-`wepp_260430` WAT authority exception for
  optional `InterceptionStorage` semantics.
- Shared-boundary transition posture remains explicit:
  `crates/openwepp-output/` target, `crates/openwepp-hillslope-output/`
  transition predecessor.

## Ran
- `cargo test -p openwepp-hillslope-output`
  - pass (`14 passed; 0 failed`).
  - includes schema metadata assertions in `hillslope_wat` unit tests.
- `cargo test --test cli04_runner_wat_parquet_contract_derived_tests`
  - pass (`2 passed; 0 failed`).
  - verifies emitted `H1.wat.parquet` is valid parquet and contains required
    dataset metadata keys and `P`/`InterceptionStorage` field metadata.
