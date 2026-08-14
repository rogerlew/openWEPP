# Surface-Liquid Operand Lineage

Evidence class: `Static`

| Operand | Unit/basis | Authoritative owner/source | Candidate consumer |
|---|---|---|---|
| beginning surface/litter liquid | `kg m^-2 tile-ground` | persistent hydrology state keyed by OFE/tile/surface class | immutable authorization snapshot |
| request `D` | `kg m^-2 OFE-ground interval` | LSE constitutive potential solve | hydrology arbiter |
| authorization `A` | same | hydrology immutable snapshot | LSE fixed-cap rebuild |
| finalized use `F` | same | accepted fixed-cap LSE solve | hydrology candidate debit |
| condensation credit | `kg m^-2 tile-ground interval` | signed accepted LSE vapor flux | same hydrology store |
| precipitation/runon/canopy ingress | typed parcel mass plus enthalpy | forcing/upstream/vegetation owners | post-solve hydrology ingress partition |
| retained ingress | typed parcel mass plus enthalpy | hydrology capacity partition | ending store and LSE enthalpy join |
| infiltration | typed parcel mass plus enthalpy | hydrology partition | soil liquid and soil-thermal owners |
| routed/outlet runoff | typed parcel mass plus enthalpy | hydrology partition | downstream OFE/outlet receipt |

Tile-local storage is never relabeled as OFE-ground storage. Conversion by
`f_t` occurs exactly once at request/final-use boundaries. Current ingress is
not available to same-interval withdrawal authorization.
