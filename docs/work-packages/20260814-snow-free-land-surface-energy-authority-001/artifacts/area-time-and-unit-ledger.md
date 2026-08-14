# Area, Time, And Unit Ledger

All constitutive solves use `W m^-2 tile-ground`, `kg m^-2 tile-ground`, and
seconds. Every `f_tile` is **OFE-local**: for each `ofe_id`, its configured tile
fractions are positive and sum to one under the frozen representation-only sum
rule. A tile fraction is never a hillslope-wide or cross-OFE stand fraction.

Water requests and final uses cross to hydrology on OFE-ground basis:

```text
D_ofe = f_tile * D_tile
A_tile = A_ofe / f_tile
F_ofe = f_tile * F_tile.
```

Each conversion occurs once. Energy rates close on tile basis, are integrated
over the same `dt` once, and are then weighted to OFE basis once. OFE extensive
mass/energy multiplies local depth/amount by `A_ofe=fwidth*slplen` only at the
routing boundary. Downstream OFE depth/amount divides by the downstream area;
it never reuses an upstream tile fraction.

Temperatures are Kelvin; humidity is `kg kg^-1`; pressure is Pa; resistance is
`s m^-1`; conductivity is `W m^-1 K^-1`; areal heat capacity is
`J m^-2 K^-1`; mass enthalpy is `J kg^-1`. Identity, unit, interval, and basis
mismatches are exact failures. A numerical tolerance cannot repair a wrong
OFE, tile, layer, source, amount basis, or missing/double area conversion.
