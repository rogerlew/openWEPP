# Conservation And No-Drift Audit

Evidence mode: `Ran`

Status: `focused closure and no-drift checks pass`

The standalone canopy contract tests independently reconstruct

```text
prior foliar mass + leaf-on allocation - leaf-off litter = realized foliar mass
```

for daily transitions. Cold start uses the first realized foliar mass as its
prior boundary, so neither transfer term is fabricated. A three-year replay of
the same 365-day forcing compares years 2 and 3 bit-for-bit for foliar state,
canopy, annual allocation, and annual litter totals. The wrapped NH/SH test uses
the same complete 365-day climate, a 182-day cyclic shift, and negated latitude;
it compares transformed canopy trajectories and both phenological limbs within
the contract's one-day tolerance.

The runner proof separately verifies that nonzero leaf-off litter is the exact
same-day decomposition input and that the resulting surface residue and depth
equal the independently projected values consumed downstream.

Ran: `cargo test -p openwepp-plant-phenology` and
`cargo test -p openwepp-runner --lib`; both passed.
