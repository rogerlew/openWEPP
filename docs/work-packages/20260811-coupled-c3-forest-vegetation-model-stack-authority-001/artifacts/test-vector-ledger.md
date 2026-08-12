# Test Vector Ledger

Status: `reference oracle passing`

Evidence mode: `Ran` (`2026-08-11`, `.venv/bin/python artifacts/reference_calculator.py`)

The package-local calculator returned `"all_pass": true`. It contains
nondegenerate numeric operands for zero/saturated light, Rubisco/electron
limits and their transition; Medlyn coupling; wet/dry finite interception;
mixed strata; distinct root profiles; competing water and N demand; respiration
and allocation; leaf litter C/N/dry matter and retranslocation; LAI ownership;
floor non-donation; and rollback.

| Required family | Oracle/test binding | Poison distinguished |
|---|---|---|
| zero leaf C/LAI | `radiation_zero_lai` and coupled-leaf zero branch | invented LAI or assimilation |
| light/FvCB limits | `zero`, `rubisco`, `electron`, `saturated`, `transition` | raw `min`, wrong quadratic root, lost respiration |
| coupled leaf state | nested `coupled_leaf` FvCB--Medlyn--`cs`/`ci`--canopy-air--leaf-temperature--`beta_hyd`--hydraulic fixed point plus fixed energy values | one-pass diagnostic hydraulics, omitted boundary resistance, ambient-for-surface VPD, pressure/unit error |
| interception and wet/dry energy | one `dt`-bound store-limited wet leaf/stem, dry-leaf, and dry-stem energy/water ledger plus condensation/second drainage and subfreezing rejection | missing stem energy owner, rate/amount or area alias, wrong sign/phase |
| multistratum | `vertical_mixed_radiation`, `mixed_strata_not_averaged` | cover/parameter averaging or simultaneous illumination |
| roots/water | only the selected four-node/gravity/layer path for distinct profiles, active interval-amount cap re-solve, dry/frozen exclusion, competition, and typed hydraulic-redistribution rejection | single depth, layer swap, amount-as-rate, silent negative-flux zeroing, authorization debit, overbooking |
| C cycle | fixed `cn_tissue_allocation` expected values | omitted growth respiration/double allocation |
| N cycle | proportional receipts plus request/authorization/finalized-use vectors | unlimited source, authorization-as-use, dropped competitor |
| turnover | deciduous trajectory, evergreen bounded turnover, litter C/N/retranslocation/DM | C-as-DM, lost retranslocation, receiver mismatch |
| canopy/floor independence | `floor_not_donation_target` | agricultural complementary PMET donation |
| failure | `rollback` and Rust invalid-input/nonconvergence cases | last-iterate/partial commit |

The current oracle includes adaptive-quadrature radiation reconstruction,
fixed independent radiation, dry/wet energy, hydraulic and
C/N expected values, poison alternatives, multi-interval phenology, receiver
reconstruction, and an exception raised after candidate mutation followed by
byte-identical rollback. It remains an authority oracle, not production-code
evidence. The successor must reproduce these vectors independently in Rust and
add real downstream-consumer closure before any cutover.
