# Forest Lateral-Flow Authority Candidate Fixtures

Source-native external hydrology datasets acquired as candidates for
`docs/backlog/20260618-forest-lateral-flow-absolute-magnitude-authority.md`.
Downloaded on 2026-07-01 PDT.

These are not WEPP input fixtures and do not yet define an acceptance envelope.
They provide provenance-bearing observed data for future work to derive metrics,
units, aggregation windows, uncertainty, and H2637 applicability. Legacy output
must not be used as the target.

## Source Catalog

| Source | Local directory | Primary use | Metric / units | Period | Authority role |
|---|---|---|---|---|---|
| H.J. Andrews WS10 HF024 | `hjandrews_ws10_hf024/` | Wet Pacific Northwest hillslope throughflow | trench/hillslope discharge, L/s; rainfall depth, mm | 2001-2003 | Primary lateral-flow magnitude candidate |
| Panola Mountain Research Watershed | `panola_pmrw_2002/` | Threshold lateral subsurface stormflow in a well-drained forest hillslope | trenchflow, L/15 min and mm/15 min; rainfall, mm/15 min | 2002 Jan-Jun | Contrasting event-scale lateral-flow candidate |
| Maimai M8 Experimental Watershed | `maimai_m8/` | Steep, wet hillslope trench and catchment runoff benchmark | trench discharge workbooks; rainfall/runoff workbooks | 1975-2015 selected files | Wet-hillslope comparator and bracket |
| Coweeta Hydrologic Laboratory | `coweeta_streamflow/` | Long-term forested catchment water-yield context | daily streamflow CSVs | 1935-2022 by watershed | Secondary context only; not direct lateral-flow authority |

## Contents

- `data/` contains source-native CSV/XLS/XLSX files selected for lateral-flow or
  water-yield analysis.
- `metadata/` contains landing pages, file indexes, EML, quality reports, or
  provider metadata captured at acquisition time.
- `archives/` is present only where the provider ships a ZIP archive.
- Each source directory has a `SHA256SUMS` file generated from installed files
  excluding `SHA256SUMS` itself.
- `source-manifest.csv` records the provider URL and local path for each acquired
  file group.

## Verification

Checksum verification from the repository root:

```sh
for d in tests/fixtures/forest_lateral_flow_authority/*/SHA256SUMS; do
  sha256sum -c "$d"
done
```

The Coweeta archives were also checked against the Forest Service Research Data
Archive published SHA-256 values:

- `RDS-2016-0025-2.zip`: `f4391c60d352457f0318f9b4efcd53a9193d5c0a2f76395e36a1c8b15a387d49`
- `RDS-2016-0025-2_Metadata_Fileindex.zip`: `7f71b1fda37e61bac4c5d468da5af09844db5f4e170b9215ef6cc57f33d40b76`

## Use Limits

Do not promote these files into an authority suite until a work package defines:

1. the judged openWEPP output and matching observed metric;
2. temporal aggregation and unit conversions;
3. uncertainty and acceptance envelope;
4. site-to-H2637 applicability limits;
5. closure-first openWEPP evidence prerequisites.

Coweeta is included for hydroclimatic and catchment water-yield context. It does
not observe hillslope lateral flow directly and should not carry a direct
`latqcc` magnitude verdict.
