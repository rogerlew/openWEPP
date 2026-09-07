[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| common-details.md#common-details | physical rules, solver correctness or accepted-primitive closure | shared physical symbols, state, algorithm, guards, aliases, units, tolerances and tests | common physical set listed in common-details introduction; source/guard-map/gap sections additionally for those claims |
| qualification.md#qualification | identity capture, experiment or qualification claims | frozen protocols and separate scientific/production limits | whole qualification chapter |

<a id="interface"></a>
# Shared interface

Energy/mass is positive inward; outgoing water is nonnegative. Preserve units,
interval, lineage and tile/OFE basis; physical binary64 conversion precedes exact
decode. Owners: LSE surface thermal, hydrology water, soil thermal soil state,
vegetation canopy physiology. Keep immutable-beginning/current-ingress chronology,
exactly-once opposite-sign transfers, typed failure and all-owner atomicity.

V1 is snow-free; V2 imports V1 with V10 specialization; V3 adds admitted snow-free
litter phase. Represented snow uses its separate native map and inactive litter.
Regime precedes iteration. Covered support floor: 60000000000 ns; one-nanosecond
structural chronology is not physical admission. Follow applicable support rules.

Effective rules: contract revision 3 (not model V3) supersedes only named V1
missing/future/ownership labels;
conservation/failure/owner rules survive. Litter spill adds its negative exact
operand. V27 finalizes its own pending physical prefix; V30 reuses validation
only at original positions. Detailed mechanism qualifiers remain binding.

Authority does not activate or qualify production. Retain FAIL/HOLD, expected-red
seams, frozen identities and paused EXP-R/PC1/SG1. Identity proves no scientific
result. Uncertainty expands reading; external single-file contracts require full
reading. No silent defaults, unauthorized normalization, replay fallback or duplicate owners.

Provenance shorthand `v31:Lx-Ly` means those exact original lines at commit
`b932db101cce07d0860b45b5ecaa8ddb7f455b58`, path
`docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`.
Citations remain binding; logical IDs resolve via binding-index.

<a id="canonical-invariants"></a>
## Canonical invariants
| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| <a id="INV-LANDSURFACEENERGY-001"></a> `INV-LANDSURFACEENERGY-001` | Every dimensional operand has explicit units, interval, area basis, finite domain, and provenance. | REF-009, unit governance | `[INFERENCE][Static]` | profile/test | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-002"></a> `INV-LANDSURFACEENERGY-002` | `A > 0`, `dt > 0`, temperatures satisfy their declared absolute/log domains, and resistances/conductivities satisfy admitted domains. | REF-009 | `[INFERENCE][Static]` | future runtime/test | typed failure |
| <a id="INV-LANDSURFACEENERGY-010"></a> `INV-LANDSURFACEENERGY-010` | The energy identity is independently reconstructible with every signed radiative, turbulent, advective, ground, and storage component exactly once. | REF-009 | `[INFERENCE][Static]` | future runtime/test | `LSEB-E-011` |
| <a id="INV-LANDSURFACEENERGY-011"></a> `INV-LANDSURFACEENERGY-011` | Surface-liquid start, inputs, debits, and end storage close independently. | REF-001/004/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-012` |
| <a id="INV-LANDSURFACEENERGY-012"></a> `INV-LANDSURFACEENERGY-012` | Actual liquid evaporation has one shared mass/latent-energy identity and one state debit. | REF-005/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-013` |
| <a id="INV-LANDSURFACEENERGY-013"></a> `INV-LANDSURFACEENERGY-013` | Surface `G` and soil/frost `-G` are one interface transfer, never two production fluxes. | REF-006/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-014` |
| <a id="INV-LANDSURFACEENERGY-014"></a> `INV-LANDSURFACEENERGY-014` | Each precipitation, runon, infiltration, and runoff mass crossing has exactly one linked advected-energy term with the same lineage/reference state. | REF-004/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-010/011` |
| <a id="INV-LANDSURFACEENERGY-015"></a> `INV-LANDSURFACEENERGY-015` | End liquid storage is non-negative within its admitted mass tolerance; material negative storage is never clamped. | REF-009 | `[INFERENCE][Static]` | future runtime/test | `LSEB-E-015` |
| <a id="INV-LANDSURFACEENERGY-020"></a> `INV-LANDSURFACEENERGY-020` | Snow-present, snow-terminal, and snow-free branches are mutually exclusive with the declared priority. | REF-008 | `[DIRECT][Static]` | future runtime/test | `LSEB-E-020/021` |
| <a id="INV-LANDSURFACEENERGY-021"></a> `INV-LANDSURFACEENERGY-021` | Schema-v8 terminal liquid, energy, and time are censored and cannot enter this contract. | REF-008 | `[DIRECT][Static]` | contract/runtime/test | `LSEB-E-021` |
| <a id="INV-LANDSURFACEENERGY-022"></a> `INV-LANDSURFACEENERGY-022` | LSE cannot reorder or duplicate hydrology/ET mutations; it consumes sealed handoffs. | REF-004/005/007 | `[DIRECT][Static]` | scheduler/consumer gate | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-030"></a> `INV-LANDSURFACEENERGY-030` | Climate owns forcing and phase; LSE owns surface flux evaluation, not forcing correction. | REF-007 | `[DIRECT][Static]` | ownership test | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-031"></a> `INV-LANDSURFACEENERGY-031` | Constitutive equations remain with their named owner; conservation orchestration does not confer duplicate ownership. | REF-007/009 | `[INFERENCE][Static]` | review/test | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-032"></a> `INV-LANDSURFACEENERGY-032` | Producer-only, skeleton-only, or serialized ledger evidence cannot prove a runtime path; a real downstream consumer must read and act on the result. | governance | `[DIRECT][Static]` | consumer gate | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-040"></a> `INV-LANDSURFACEENERGY-040` | No production implementation is promotable while any required family is `AUTHORITY_MISSING`. | REF-003-010 | `[DIRECT][Static] + [INFERENCE][Static]` | governance gate | `NON_PROMOTABLE` |
| <a id="INV-LANDSURFACEENERGY-041"></a> `INV-LANDSURFACEENERGY-041` | Comparator agreement, current helper code, or silent legacy clamps cannot substitute for canonical physics authority and typed guards. | ADR-0017, REF-010 | `[DIRECT][Static]` | review/gate | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-042"></a> `INV-LANDSURFACEENERGY-042` | Future vegetation radiation receipts remain recipient-specific across canopy strata, ground, litter, snow, soil, ponded water, and atmosphere; no recipient is an alias or residual bucket for another. | SC-VEGETATION-001#INV-VEGETATION-021 | `[INFERENCE][Static]` | future integration/test | hard `HOLD` |
| <a id="INV-LANDSURFACEENERGY-043"></a> `INV-LANDSURFACEENERGY-043` | Interval-integrated Stage C transpiration mass and its latent-energy debit share one transaction, stratum, area, interval, lineage, and authority-tagged `h_v`, satisfying `Q_T,s=-h_v*T_s` exactly once. | SC-VEGETATION-001#INV-VEGETATION-014 | `[INFERENCE][Static]` | future integration/test | hard `HOLD` |

<a id="canonical-obligations"></a>
## Canonical obligations
| Obligation ID | Statement | Applicability | Authority | Enforcement/failure | Test bindings |
|---|---|---|---|---|---|
| <a id="OBL-LANDSURFACEENERGY-C-004"></a> `OBL-LANDSURFACEENERGY-C-004` | a real scheduler consumer must prove that the new state and ledger affect the intended direct path before runtime closure. | All scheduler/direct-path consumers claiming runtime closure | v31:L332-L333 | Local guards; [errors](water-vapor.md#errors) | [Shared tests](common-details.md#tests) and [Mechanism tests](common-details.md#tests); named fixtures/tests and real consumers |
