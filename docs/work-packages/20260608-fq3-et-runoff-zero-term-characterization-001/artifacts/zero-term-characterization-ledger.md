# Zero-Term Characterization Ledger

Evidence mode: Ran (tables) + Static (defect grouping interpretation)

## Management Group Inventory

From `management_group_map.csv`:

- Corn: 36 prefixes
- Tah_4899: 6 prefixes

## Group x Term Aggregates

Source: `group_term_aggregate.csv`.

| Group | Term | openWEPP sum | legacy sum | Classification summary |
|---|---|---:|---:|---|
| Corn | Ep | 0.000000 | 65975.66 | 36/36 `defect-openwepp-zero-legacy-nonzero` |
| Corn | Es | 170418.787964 | 98054.99 | 36/36 `both-nonzero` |
| Corn | Er | 0.000000 | 0.00 | 36/36 `expected-config-zero` |
| Corn | Interception | 0.000000 | N/A | 36/36 `legacy-term-unavailable` |
| Corn | Q | 26.811885 | 27021.753956 | 29 defect, 7 nonzero/nonzero |
| Corn | QOFE | 26.811885 | 27021.753956 | 29 defect, 7 nonzero/nonzero |
| Tah_4899 | Ep | 32774.097207 | 34722.37 | 6/6 `both-nonzero` |
| Tah_4899 | Es | 127.531464 | 169.36 | 6/6 `both-nonzero` |
| Tah_4899 | Er | 0.000000 | 0.00 | 6/6 `expected-config-zero` |
| Tah_4899 | Interception | 3858.476933 | N/A | 6/6 `legacy-term-unavailable` |
| Tah_4899 | Q | ~0.0 | 1067.987877 | 6/6 `defect-openwepp-zero-legacy-nonzero` |
| Tah_4899 | QOFE | ~0.0 | 1067.987877 | 6/6 `defect-openwepp-zero-legacy-nonzero` |

## Key Verdicts

1. Annual-crop ET hypothesis: confirmed as a defect signal.
   - Corn `Ep` is zero for all 36 prefixes in openWEPP while legacy is materially nonzero for all 36.
   - Tah_4899 `Ep` is nonzero on both engines, so this is not a universal ET collapse.
2. Interception comparator availability: unavailable in legacy WAT output.
   - Comparator does not publish an `Interception` term in this surface, so this package cannot comparator-label interception as defect/config.
3. Runoff (`Q`, `QOFE`) zero-term question: largely defect-shaped, not universal.
   - 35/42 prefixes show `defect-openwepp-zero-legacy-nonzero` for both `Q` and `QOFE`.
   - 7 Corn prefixes remain `both-nonzero` with openWEPP much smaller than legacy (`p4,p25,p33,p35,p38,p40,p42`).

## Defect Shaping

- Root-cause family A (crop ET engagement): Corn-only `Ep=0` while legacy transpires.
- Root-cause family B (runoff partition/output): near-zero/zero `Q` and `QOFE` versus legacy across most prefixes, including all Tah_4899 cases.
- Interception requires a non-WAT comparator surface before defect/config labeling.
