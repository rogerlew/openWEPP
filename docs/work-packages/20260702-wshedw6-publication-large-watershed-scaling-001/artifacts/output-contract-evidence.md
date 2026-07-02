# Output Contract Evidence

Status: `passed`

Evidence mode: `Ran:` produced parquet schema/row checks and focused writer
test.

## Required Outputs

Both W6 scaling fixtures emitted all `14` required watershed parquet outputs:

- `ebe_pw0.parquet`
- `chan.out.parquet`
- `chanwb.parquet`
- `chnwb.parquet`
- `soil_pw0.parquet`
- `totalwatsed3.parquet`
- `loss_pw0.hill.parquet`
- `loss_pw0.chn.parquet`
- `loss_pw0.out.parquet`
- `loss_pw0.class_data.parquet`
- `loss_pw0.all_years.hill.parquet`
- `loss_pw0.all_years.chn.parquet`
- `loss_pw0.all_years.out.parquet`
- `loss_pw0.all_years.class_data.parquet`

Evidence artifacts:

- `artifacts/scaling/onshore-xenophobia-scaling-summary.json`
- `artifacts/scaling/carnivorous-adobo-scaling-summary.json`

## Schema Preservation

The W6 writer change routes `WatershedPublicationFrame` through the same schema
builders used by the retained row-seed writer tests. Output identity checks read
each parquet file with `pyarrow.parquet`, compare schemas, and compare row
content across job counts.

Result:

- `onshore-xenophobia`: all `14` schemas matched between `--jobs 1` and
  `--jobs 48`.
- `carnivorous-adobo`: all `14` schemas matched between `--jobs 1` and
  `--jobs 32`.

## Contract Gate

W6 is schema-preserving and physics-preserving. No canonical science-contract
amendment was required; unavailable typed publication operands are emitted as
null instead of filled with compatibility-stage zeroes, and W6 area
normalization uses committed source slope geometry. See
`artifacts/pre-implementation-contract-gate.md` and
`artifacts/publication-operand-lineage.md`.
