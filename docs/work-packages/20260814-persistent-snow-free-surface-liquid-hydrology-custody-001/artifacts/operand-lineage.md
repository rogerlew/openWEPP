# Surface-Liquid Operand Lineage

Evidence class: `Static`

| Operand | Unit/basis | Authoritative owner/source | Candidate consumer |
|---|---|---|---|
| beginning surface/litter liquid | `kg m^-2 tile-ground` | persistent hydrology state keyed by OFE/tile/surface class | immutable authorization snapshot |
| request `D` | `kg m^-2 OFE-ground interval` | LSE constitutive potential solve | hydrology arbiter |
| authorization `A` | same | hydrology immutable snapshot | LSE fixed-cap rebuild |
| finalized use `F` | same | accepted fixed-cap LSE solve | hydrology candidate debit |
| condensation credit | `kg m^-2 OFE-ground interval` (the LSE DTO's `stand-ground`) | signed accepted LSE vapor flux | exact configured tile/source store after one division by `f_t` |
| precipitation/runon/canopy ingress | typed parcel mass plus enthalpy | forcing/upstream/vegetation owners | post-solve hydrology ingress partition |
| retained ingress | typed parcel mass plus enthalpy | hydrology capacity partition | ending store and LSE enthalpy join |
| infiltration | typed parcel mass plus enthalpy | hydrology partition | soil liquid and soil-thermal owners |
| routed/outlet runoff | typed parcel mass plus enthalpy | hydrology partition | downstream OFE/outlet receipt |

Tile-local storage is never relabeled as OFE-ground storage. Conversion by
`f_t` occurs exactly once at request, finalized-use, and condensation-credit
boundaries. Current ingress is not available to same-interval withdrawal
authorization. In the native shadow, the persistent bare-surface store replaces
WB14 depression retention; WB14 runs once per OFE with zero legacy depression
capacity and the persistent store receives only post-infiltration excess.
