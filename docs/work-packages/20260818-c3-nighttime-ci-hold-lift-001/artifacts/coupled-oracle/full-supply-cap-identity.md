# Full-Supply Cap Identity

Status: `DIRECT JOIN PASS`

The frozen potential request batch and final authorization batch use the same
transaction (`41`), owner, OFE (`ofe-1`), tile (`forest`), occupancy, source
type, source ID, soil-layer ID, and stand-ground interval basis.

For each accessible root source the authorization amount is the corresponding
potential request amount and the reason is `FullSupply`:

| Occupancy | Layer | Authorization (kg m^-2 stand-ground interval) |
| --- | --- | ---: |
| `stratum-z-upper::forest` | `soil-1` | `4.138821210058509e-7` |
| `stratum-z-upper::forest` | `soil-2` | `2.536696870681022e-7` |
| `stratum-a-lower::forest` | `soil-1` | `4.123764176237719e-7` |
| `stratum-a-lower::forest` | `soil-2` | `2.527468366081183e-7` |

`soil-dry` and `soil-frozen` are identity-preserving zero-supply entries. The
ground litter authorization is `0.0046847023088664634 kg m^-2` with
`FullSupply`. Equality uses `cap <= law`, so every positive full-supply root
entry is authorization-active-or-tie at the accepted potential candidate.
The converted ground cap is one binary64 ULP above its reevaluated law; ground
therefore remains on `ConstitutiveLaw`, with law equal to final flux.

No aggregate authorization substitutes for a per-layer cap. Conversion to
rate occurs once through the existing interval/basis conversion before the
complete fixed-final evaluation.
