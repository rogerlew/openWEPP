# Independent Review Finding Disposition

Status: `PASS / every accepted finding corrected and independently verified`.

The initial climate/radiation and vegetation/LSE reviews both returned HOLD.
Every finding below is accepted; none is deferred, waived, or rejected.

| Finding | Severity | Disposition |
| --- | --- | --- |
| Parent-hour solar zenith cosine lacked an exact repository derivation. | P0 | Accepted. The contract now derives the representative cosine from the same bounded parent-hour `radcur` hour-angle support and binds it in each receipt. |
| The legacy near-isothermal `radmj/24` path could create positive nighttime shortwave. | P0 | Accepted. The new horizontal provider always uses the admitted `radcur/rpoth` lineage; the legacy winter branch is explicitly not this provider. |
| Breakpoint chronology dropped `stmstr` and could not preserve midnight carry. | P0 | Accepted. The source contract now requires `stmstr_h`, defines absolute day-clock half-open support, and carries the next-day suffix explicitly. |
| Weiss--Norman low-transmissivity behavior produced negative direct components. | P0 | Accepted. The selected original-method nonnegative direct-fraction branch is explicit and has a low-positive-irradiance vector. |
| The calculator did not construct the complete 24-parent to 48-receipt operator. | P1 | Accepted. The calculator now emits one complete digest-bound 48-interval day receipt, derives the parent radiation and solar-angle operands, closes daily energy and precipitation, and regenerates byte-exactly. |
| The forcing schema omitted required physical and identity fields. | P1 | Accepted. The closed schema now requires day/provider/OFE/tile/support/transaction identities and every atmospheric, parcel, CO2, GSI, reference-height, and WB14 operand. |
| Coupled handling at nonpositive VPD was transactionally ambiguous. | P1 | Accepted. A physically valid LSE atmospheric receipt may be constructed, but the coupled vegetation interval fails atomically before any owner advances. |
| Snow and mixed-phase precipitation admission was underdefined. | P1 | Accepted. Snow or mixed snow mass is typed unsupported; admitted liquid parcels use the exact Harder--Pomeroy hydrometeor-temperature parent. |
| Visible and PAR recipient aliases were not bound. | P2 | Accepted. Visible and PAR are the same 400--700 nm energy-flux bytes, with no photon conversion. |
| Daily GSI custody could permit provider recomputation or double advancement. | P2 | Accepted. The live owner computes and advances GSI once; the forcing provider holds its accepted receipt for all 48 intervals. |
| SIMIMPL28 custody used an abbreviated digest. | P2 | Accepted. The custody ledger and contract now carry the full source SHA-256. |
| The complete receipt used air temperature instead of the Harder--Pomeroy hydrometeor temperature. | P1 | Accepted. The independent calculator now reconstructs the selected hourly Harder--Pomeroy solution and uses its temperature for parcel enthalpy; the complete rainy receipt asserts non-aliasing from air temperature. |
| Snow/mixed-phase and midnight carry were not executable complete parcel evidence. | P1 | Accepted. Cold mixed phase is an executed typed rejection, while midnight carry retains a schema-valid full parcel identity, support, temperature, mass, and enthalpy. |
| Receipt poisons were literal labels instead of validator executions. | P1 | Accepted. The calculator now independently validates 48 supports, provider/source joins, interval/day hashes, parcel enthalpy, and global homogeneity; missing, duplicate, mixed-provider, one-bit physical, and heterogeneous cases execute those failures. A generated matrix changes every interval digest operand. |
| Condensation vectors returned before constructing the LSE atmospheric receipt. | P2 | Accepted. Nonpositive-VPD cases now retain complete humidity, shortwave, and atmospheric-longwave evidence before the coupled atomic unsupported disposition. |
| Current-mechanics prose retained the disallowed near-isothermal radiation branch. | P2 | Accepted. The map now distinguishes the legacy behavior and binds the new provider exclusively to `radcur/rpoth`. |
| Contract GSI/WB14 identity prose was malformed and duplicated. | P2 | Accepted. The paragraph is repaired without changing ownership. |
| Vector-ledger case count was stale. | P2 | Accepted. The ledger no longer hardcodes a stale count; executable inventory is authoritative. |

Focused exact-current evidence before re-review:

- independent calculator regeneration: `PASS`, twice byte-identical;
- complete receipt JSON-schema validation: `PASS`;
- contract authority target: `4/4 PASS`;
- `cargo fmt --all -- --check`: `PASS`;
- `git diff --check`: `PASS`.

Both independent reviewers verified the remediated scientific bytes before
admission and later re-froze the approved/active lifecycle and exact model
bindings. Terminal-verifier findings about schema and provider-definition
identity were also accepted: the model now binds the schema and a separate
noncircular semantic provider descriptor; receipts bind the exact descriptor
hash, and the focused test independently recomputes every bound artifact hash.
