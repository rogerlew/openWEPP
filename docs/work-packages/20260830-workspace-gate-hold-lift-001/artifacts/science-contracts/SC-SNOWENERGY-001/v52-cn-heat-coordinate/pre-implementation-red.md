# V52 CN heat coordinate pre-implementation red

Evidence mode: `Static + Ran`

Status: `EXPECTED RED`

R133 advanced through V51 and failed at exact-floor support `2100..2160 s`
inside authentic receipt stabilization. R134 proved exact
`ReceiptOscillation` at used `94/96`: ordinary solve 61 evaluations, polish
18 with `ReceiptEntryReserve`, two probes, and no replay. The exact two-cycle
changes ending snow temperature `263.2042297771622 -> 263.20422977716225 K`,
ending flux `-88.24563334437782 -> -88.24563334437724 W m^-2`, and
positive-into-snow candidate heat
`5340.494294593449 -> 5340.494294593433 J m^-2`.

Static tracing proves the private map algebraically computes CN heat from
proposed endpoint temperatures, whereas the authentic map consumes the sealed
receipt heat and reconstructs its successor from the physical endpoint. The
continuous receipt-governing heat is absent from the solved coordinate and
residual vector. V52 contract/source tests are authored before production and
must fail only for the missing Q coordinate/residual and five behavior seams.

Retained evidence:

- r133 SHA-256 `6291dab02a435a46c4f13646fe8898ade184029ec9cbca75bb7739bab4b2ebcb`;
- r134 SHA-256 `cf276951616c509f71bf2f33dc2192e096d5367768ee43062e66f9e37a8d39f0`.

No receipt averaging, interpolation, canonical repair, digest/bit-distance or
map-difference residual, tolerance/cap/floor change, uncharged physics,
private publication, or replay/finalization bypass is authorized.
