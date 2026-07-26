# Target Ledger Schema

Evidence class: `Executed classification contract`

`target-ledger.csv` is UTF-8 CSV with one scalar or bounded quantity per row.
Empty `value` is allowed only when `value_low` and `value_high` define a range.
Empty bounds mean a point value. Full return-period series are normalized in
`comparison-scale-ledger.md` and represented in this ledger by their individual
series authority rows.

| Column | Meaning |
| --- | --- |
| `target_id` | Stable package-local identifier. |
| `site` | `HUBBARD_BROOK`, `SANTEE`, `MICA_CREEK`, or `GENERAL_WEPP`. |
| `quantity` | Normalized quantity name. |
| `value`, `value_low`, `value_high` | Point or inclusive range. |
| `unit` | Explicit physical or dimensionless unit. |
| `temporal_basis` | Instant, annual, long-term annual mean, equilibrium, simulation mean, or return period. |
| `spatial_basis` | Plot/stand, hillslope/OFE, watershed, channel network, or regional proxy. |
| `process_boundary` | The physical/model boundary represented. |
| `material_class` | `LIVE_ABOVEGROUND`, `FOLIAGE`, `NEEDLES`, `WOODY_LITTER`, `AGGREGATE_LITTER`, `FOREST_FLOOR`, `TOTAL_FUEL`, `NOT_APPLICABLE`, or `UNRESOLVED_MATERIAL_CLASS`. |
| `evidence_role` | One required package enum. |
| `source_file`, `source_locator` | Retained source and page/line/table/field. |
| `reported_source` | Authority named by Bill or the retained source. |
| `uncertainty` | Measurement, range, chart resolution, derivation, or unknown status. |
| `admissibility` | `ADMISSIBLE_TARGET`, `REPRODUCTION_TARGET`, `CONTEXT_ONLY`, `BRANCH_REQUIRED`, `EXCLUDED_MISCLASSIFIED`, or `EXCLUDED_UNSOURCED`. |
| `notes` | Scale, material, or interpretation limitation. |

Allowed `evidence_role` values are exactly:

- `FIELD_OBSERVATION`
- `LITERATURE_CONTEXT`
- `BILL_DERIVED_ASSUMPTION`
- `MANAGEMENT_OPERAND`
- `MODEL_OUTPUT`
- `CHART_DIGITIZED`
- `UNSOURCED_CONTEXT`

`CHART_DIGITIZED` values are approximate even when a single decimal is
recorded. `MODEL_OUTPUT` rows extracted from screenshots are reproduction
targets, not exact machine-output identities.
