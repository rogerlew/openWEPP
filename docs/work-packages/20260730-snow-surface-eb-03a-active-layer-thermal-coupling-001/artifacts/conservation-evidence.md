# Conservation Evidence

Status: `complete / pass`

Evidence mode: `Ran`

The real-consumer evidence is
[`consumer-cells.json`](consumer-cells.json). All B absent, B empty, B, L, S,
and LS cells exited zero.

| Reconstruction or closure | Maximum observed |
| --- | ---: |
| same-substep requested `G_0` reconstruction | `3.1531e-13 W m^-2` |
| requested - applied - rejected `G_0` | `0 W m^-2` |
| active/lower resistance reconstruction | `6.2095e-17 m^2 K W^-1` |
| active energy residual | `4.8749e-10 J m^-2` |
| lower energy residual | `6.2665e-10 J m^-2` |
| internal conduction cancellation | `0 J m^-2` |
| whole Stage 3 energy residual | `6.0111e-08 J m^-2` |

Each cell exercised active snow coupling and a real lower layer. Peak
conductive-flux row counts ranged from 352 to 680. Maximum active depth was
`0.25000000000000033 m`, within the floating-point acceptance margin around
the contracted `0.25 m`. Minimum temperatures were `-0.948`, `-15.276`,
`-8.781`, and `-16.605 deg C` for B, L, S, and LS respectively; no cell
approached absolute zero.

S and LS sublimation totals were `0.1449641 m` and `0.1275792 m`. The focused
runtime suite independently verifies vapor-mass/latent-energy identity,
cold-content export, liquid non-aliasing, active/lower ledgers, and whole-pack
closure.
