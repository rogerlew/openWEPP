# Numeric/output equivalence

Status: PASS
Evidence mode: Ran

The characterization suite independently reconstructs published operands from
literal source rows. Distinct WAT areas, columns, hillslopes, OFEs, and
unmatched optional rows make these incorrect alternatives observably unequal:
WAT `Q` as runoff, WAT `QOFE` as runoff, all-OFE lateral flow, unweighted
optional means, total-area optional denominators, wrong date/OFE joins, and
adjacent-column substitutions.

The test proves PASS `runvol` remains the runoff-volume authority, WAT fields
retain ordered `depth*Area` aggregation, `latqcc` retains outlet-only numerator
selection with the full WAT area denominator, `QOFE` retains all rows, and
TSMF/QRain/QSnow retain matched-WAT-area weighting. PASS sediment class mass
continues to use the same row's concentration and runoff volume; zero runoff
emits finite zero sediment volume concentration.

Post-decomposition test
`two_day_water_storage_and_sediment_oracle_rejects_wrong_aliases` reconstructs
every published water/storage/profile field, row order, schema type and
nullability, `sbrunv`, `tdet`, `tdep`, all five sediment class masses,
`sed_del`, and class-density `sed_vol_conc` from literal source rows. Its two
days use different total areas. It independently reconstructs storage delta
and a deliberately nonzero primary water residual, and rejects Q/QOFE runoff,
all-OFE lateral, common-density, concentration-sum, storage, interception, and
adjacent-column aliases. A separate partial-coverage test proves the matched
optional-area denominator and current last-duplicate-WAT-key overwrite rule.
The final focused suite passes 17/17.

The accepted WSHED01 T-B2-REDO2 real-consumer cohort ran against commit
`1a4d6cd6`, whose `totalwatsed3.rs` SHA-256 is exactly the pre-refactor hash
`1b9d8d124bf34a3d5f9189eb901a2ac87ff89d51076a58632c596ec878e47ac9`.
It reconstructed 78,912 PASS/WAT rows with zero maximum runoff pairing delta,
emitted 2,192 totalwatsed3 rows, observed only
`-4.0978193283081055e-08 m3` total PASS/output runoff sum delta, and reported
`-0.409175395336963 mm` basic-storage residual excluding initialization over
2,191 days with zero days above 1 mm. The current decomposition changes no
operand, ordering, formula, schema, or output mapping; the comprehensive
current-source oracle binds that accepted real-cohort evidence across the
mechanical refactor.
