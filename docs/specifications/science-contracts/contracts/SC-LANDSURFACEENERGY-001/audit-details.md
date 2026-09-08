[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
none beyond entry

<a id="audit-details"></a>
# Authority and enforcement audit details

All sections remain normative. Read the relevant complete sections for source provenance, enforcement-path, historical applicability or promotability claims. Current physical rules and unresolved authority limits remain in interface and common-details; this relocation does not resolve or demote any gap.

<a id="purpose"></a>
## Purpose

Define the first-class control-volume, conservation, custody, constitutive and
failure authority for the default-off `OPENWEPP_SNOW_FREE_LSE_V1` model.
Version 3 releases implementation authority for snow-free bare-mineral and
forest-litter surfaces coupled to `OPENWEPP_C3_WOODY_V8`; it authorizes no
production selector, cutover, calibration or snow-terminal recipient.

Sections from "Scientific Scope" through the gap register preserve the
version-2 conservation and missing-authority baseline as historical context.
Where those sections say `future`, `missing`, `gap`, `NON_PROMOTABLE`, or
describe `M_l` as LSE-mutated, the named Version-3 constitutive authority below
prospectively supersedes that statement for `OPENWEPP_SNOW_FREE_LSE_V1` only.
All exact-one conservation, failure, adjacent-owner and no-real-consumer rules
not expressly superseded remain binding.

<a id="scope"></a>
<a id="scientific-scope-and-explicit-out-of-scope-boundaries"></a>
## Scientific Scope and Explicit Out-of-Scope Boundaries

In scope is one horizontal-area-normalized surface control volume over one
explicit interval. It owns the surface temperature/energy state, surface
liquid energy state, flux sign convention, exact-one component lineage, and
the coupled water/energy bookkeeping at its boundary.

Out of scope in version 1:

- production Rust, scheduling, selectors, defaults, publication, or cutover;
- any snow-present thermodynamics, which remain owned by
  `SC-SNOWENERGY-001` and `SC-SNOWFREEZE-001`;
- consumption of schema-v8 terminal liquid, energy, or unevaluated time;
- recomputation of ET, infiltration, runoff, percolation, soil-water
  withdrawal, frost fronts, or subsurface phase change;
- a new empirical parameter, calibration result, remote-sensing input, or
  wepppy-owned climate/GIS/run-state concern; and
- provisional, surrogate, heuristic, or comparator-targeted physics.

<a id="sources"></a>
<a id="authority-anchors-with-top-down-citations"></a>
## Authority Anchors with Top-Down Citations

| Anchor | Source | Binding use | Evidence |
|---|---|---|---|
| `REF-LANDSURFACEENERGY-001` | `references/50201000/chap5.pdf`, §§5.1-5.3 | Daily water closure and modified-Ritchie ET context; it does not specify a complete prognostic surface-energy solver. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-002` | `references/50201000/chap4.pdf`, §4.2 | Green-Ampt/Mein-Larson infiltration and rainfall-excess ownership boundary. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-003` | pinned `dac3c950...:src/contin.for`, lines 839-882 and 907-922 | Winter branch produces `wmelt`; rain plus melt updates the `r5` antecedent-history lineage. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-004` | pinned `dac3c950...:src/watbal.for`, lines 331-344, 431-498 | Liquid ingress precedes percolation and ET; water state is mutated by the owning hydrology routines. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-005` | pinned `dac3c950...:src/evap.for`, lines 171-360 and 428-680; `src/evappm.for`, lines 178-430 | Legacy radiation/ET demand and soil/residue/plant water withdrawal; not an hourly surface enthalpy solver. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-006` | pinned `dac3c950...:src/frostn.for`, lines 383-686; `src/frzng.for`, lines 331-381; `src/frznw.for`, lines 62-113 | Soil/residue/snow thermal resistance and frozen-soil phase mechanics; `surtmp` is an input rather than a coupled solved surface state. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-007` | `SC-CLIMATE-001`, `SC-EVAP-001`, `SC-WATBAL-001`, `SC-RUNOFFPART-001`, `SC-SOIL-001`, `SC-SUBHYD-001` | Existing forcing, ET, water, runoff, soil, and subsurface ownership. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-008` | `SC-SNOWENERGY-001#INV-SNOWENERGY-034` and `GAP-SNOWENERGY-011` | Schema-v8 terminal payload is censored and has no receiving-surface authority. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-009` | physical conservation of mass and first-law energy accounting | Exact ledger closure and equal/opposite shared-boundary fluxes. | `[INFERENCE][Static]` |
| `REF-LANDSURFACEENERGY-010` | `crates/openwepp-meteorology/src/surface_energy.rs` | Existing typed flux algebra is reusable mechanics only; it is not proof of a complete runtime owner or consumer. | `[DIRECT][Static]` |
| `REF-LANDSURFACEENERGY-011` | IEEE 754 binary64 round-to-nearest, ties-to-even; exact integer arithmetic over finite-binary64 dyadics; `SC-SURFACELIQUID-001#INV-SURFACELIQUID-022` | Receiver-owned exact soil-layer enthalpy total, one correctly rounded high term, canonical signed-dyadic carry, and exact accepted-credit custody. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-LANDSURFACEENERGY-012` | IEEE 754 binary64 round-to-nearest, ties-to-even; exact integer arithmetic over finite-binary64 dyadics; physical conservation of accepted surface-energy operands; `SC-SURFACELIQUID-001#INV-SURFACELIQUID-023` | LSE-owned exact per-tile surface enthalpy, immutable high-term mirrors, exact retained-ingress credit custody, and successor restart/rollback. | `[DIRECT][Static] + [INFERENCE][Static]` |

All pinned citations mean `git show
dac3c950d8b16cc73774bf5ce2e7e11f80baac70:<path>`; the mutable checkout
HEAD is not normative.

<a id="enforcement"></a>
<a id="invariants-and-invariant-guard-map"></a>
## Invariants and Invariant Guard Map

| Binding reference | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|

Guard-map enforcement in version 1 is the contract-derived integration test
and package review. Runtime mappings are intentionally future obligations; an
implementation package must replace each future mapping with a typed path and
evidence artifact before promotion.

<a id="invariant-guard-map"></a>
### Invariant Guard Map

| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |
|---|---|---|---|---|
| `INV-LANDSURFACEENERGY-001` | focused contract schema/unit assertions | test | blocked promotion | package contract-test evidence |
| `INV-LANDSURFACEENERGY-002` | future LSE input validator | runtime | `LSEB-E-001`; currently `HOLD` | `GAP-LANDSURFACEENERGY-003/004` |
| `INV-LANDSURFACEENERGY-010` | future independent energy reconstruction | runtime | `LSEB-E-011`; currently `HOLD` | `GAP-LANDSURFACEENERGY-001/004` |
| `INV-LANDSURFACEENERGY-011` | future independent water reconstruction | runtime | `LSEB-E-012`; currently `HOLD` | `GAP-LANDSURFACEENERGY-004` |
| `INV-LANDSURFACEENERGY-012` | future latent mass/energy identity | runtime | `LSEB-E-013`; currently `HOLD` | `GAP-LANDSURFACEENERGY-002/003` |
| `INV-LANDSURFACEENERGY-013` | future equal/opposite ground-flux consumer | runtime | `LSEB-E-014`; currently `HOLD` | `GAP-LANDSURFACEENERGY-002/004` |
| `INV-LANDSURFACEENERGY-014` | future liquid mass/advected-energy lineage join | runtime | `LSEB-E-010/011`; currently `HOLD` | `GAP-LANDSURFACEENERGY-002/003` |
| `INV-LANDSURFACEENERGY-015` | future end-storage domain validator | runtime | `LSEB-E-015`; currently `HOLD` | `GAP-LANDSURFACEENERGY-003/004` |
| `INV-LANDSURFACEENERGY-020` | future branch selector plus poison vectors | runtime | `LSEB-E-020/021`; currently `HOLD` | `GAP-LANDSURFACEENERGY-004/005` |
| `INV-LANDSURFACEENERGY-021` | terminal-field rejection assertion | test | `LSEB-E-021` / blocked promotion | focused test + snow contract |
| `INV-LANDSURFACEENERGY-022` | scheduler-order and real-consumer gate | governance | blocked promotion | `GAP-LANDSURFACEENERGY-004` |
| `INV-LANDSURFACEENERGY-030` | owner-boundary assertions | test | blocked promotion | focused test + adjacent contracts |
| `INV-LANDSURFACEENERGY-031` | dual science review and owner assertions | governance | blocked promotion | package review artifacts |
| `INV-LANDSURFACEENERGY-032` | real-consumer reachability gate | governance | blocked promotion | `GAP-LANDSURFACEENERGY-004` |
| `INV-LANDSURFACEENERGY-040` | gap-label assertion and package disposition | governance | `NON_PROMOTABLE` | focused test + disposition |
| `INV-LANDSURFACEENERGY-114` | terminal support/operand validator and receiver selection | default-off runtime/test | typed reject/rollback | terminal handoff package |
| `INV-LANDSURFACEENERGY-115` | liquid-energy join and atomic envelope | default-off runtime/test | typed reject/rollback | terminal handoff package |
| `INV-LANDSURFACEENERGY-130` | covered-liquid finalization before ledger materialization | runtime/test | exact reference-state canonicalization or existing typed reject | adaptive microstepping package |
| `INV-LANDSURFACEENERGY-138` | covered-column Jacobian probe-domain validator | runtime/test | centered interior, unique inward closed-bound derivative, or typed reject | covered solver contract/unit/runtime vectors |
| `INV-LANDSURFACEENERGY-139` | covered-column first-domain-valid-halving no-update witness | runtime/test | after the full trial fails the existing no-update witness by domain or governed-step refusal, accept the unchanged current iterate only when the complete current residual vector and every governed first-halved prospective step pass; otherwise retain strict-decrease update or typed numerical rejection | covered solver contract/unit/runtime vectors plus interior-terminal real consumer |
| `INV-LANDSURFACEENERGY-150` | `SoilThermalOwnerEnvelopeV2`, exact aggregation/rounding, typed energy-credit receipt, restart/checkpoint replay, complete-owner join, and real consumers | runtime/test/governance | canonical finite exact total and atomic install, or `LSEB-E-049`; blocked promotion until WAT5, `p61`, and native-forest consumers pass | exact-carry focused vectors, split-restart equivalence, rollback hashes, and three real-consumer gates |
| `INV-LANDSURFACEENERGY-151` | `LseSurfaceEnthalpyOwnerEnvelopeV1`, exact surface-energy operand aggregation, immutable V2/V3 high-mirror join, restart/checkpoint replay, `SurfaceLiquidCompleteOwnerProjectionV4`, and real consumers | runtime/test/governance | canonical finite exact total and atomic install, or `LSEB-E-050`; blocked promotion until `p61` and native-forest consumers pass | exact surface-carry vectors, split-restart equivalence, rollback hashes, and two real-consumer gates |
| `INV-LANDSURFACEENERGY-153` | exact-surface receipt ending-posture and parent/child-support validator plus V3/V2 mirror join | runtime/test/governance | deterministic partial/final lineage or `LSEB-E-050` / `SURFACELIQUID-E-012` with byte-exact rollback | first/middle/final child, mid-parent restart, wrong posture/bounds/mixed marker/early advance, rollback |
| `INV-LANDSURFACEENERGY-154` | immutable represented-snow classifier, standard Stage-3 covered-column charge, native identity/receipt join, inactive-litter custody validator, and terminal split | runtime/test/governance | one charged standard map or typed rejection with byte-exact owner rollback | typed classification, zero litter-phase calls, exact optical/lower-boundary retention, no second envelope, unchanged V3/V4 bytes, snow-free transition |
| `INV-LANDSURFACEENERGY-155` | unpublished-continuation constructor, typed V3 candidate-only soil-beginning branch, non-owner projection guard, complete outer replay, and single owner/restart install | runtime/test/governance | authenticated private read or `LSEB-E-049`; no intermediate owner; byte-exact rollback | contiguous support, predecessor-trial substitution, owner/restart-byte absence, exact final replay, one install, rollback |
| `INV-LANDSURFACEENERGY-156` | bounded-phase raw/retained/spill splitter, typed phase-source receipt, V3 current-ingress handoff, named exact-surface debit operand, and independent mass/enthalpy replay | runtime/test/governance | exact spill and one WB14 handoff or typed `LSEB-E-047/048/050`; no owner mutation on failure | zero/below/at/above capacity, melt-created spill, mass/enthalpy and area-basis closure, receipt/support/transaction/key substitution, no-resolve, one ingress, rollback |
| `INV-LANDSURFACEENERGY-157` | typed native-phase versus ordinary-finalized-use partition, phase-adjusted V2 resource join, canonical ordinary debit, and single current-ingress continuation | runtime/test/governance | every finalized row consumed exactly once under its original custody or typed rejection; no resource replacement or owner mutation on failure | heterogeneous open/covered batch, zero/nonzero ordinary rows, native-row replay, duplicate/omitted/foreign row, canonical order, one ingress, rollback |
| `INV-LANDSURFACEENERGY-158` | topology-ranked V16 exact-surface owner records and accepted operand groups at the authenticated configuration join | runtime/test/governance | accept opaque multi-digit OFE identities in configuration order; reject duplicate/omitted/substituted/stale/reordered custody with rollback | topology `ofe-9 -> ofe-10`, reverse/nonmonotone IDs, within-OFE order, duplicate/omission/substitution, stale digest, rollback |
| `INV-LANDSURFACEENERGY-159` | private immutable validated resident/resource handoff, parent-static structural plan, per-map forcing proof, and resident-revision-sourced native proof with exact pointer, revision, generation, configuration, topology, and lineage binding | runtime/test/governance | join each proof only at the validation position it replaces; every map freshly validates exact forcing once and all map-dynamic surfaces; every resident successor and every restart/external/durable/untrusted boundary receives full validation | O(1) resident install, append-tail validation, one parent-static plus one forcing/dynamic validation per map, native proof consumption only in native regimes, structural/native object poisons, proof reuse/transfer, combined first-error poisons, restart replay, rollback |
| `INV-LANDSURFACEENERGY-160` | private snow-free provisional-physical ending consumed by the exact final accepted-slab reseal | runtime/test/governance | execute energy and coupled owner physics once; reseal only final receipt identities or reject atomically | exact direct-versus-reuse owner bytes, one provider/phase/ingress/WB14/soil call, non-slab poisons, single-use/restart rejection, rollback |
| `INV-LANDSURFACEENERGY-162` | represented-snow ground/soil anchor classifier and exact probe-residual assembler | runtime/test | bit-identical full-evaluator residual vectors and Jacobian or complete-evaluator path; typed existing domain/error precedence | full-evaluator differential oracle, centered/inward stencil vectors, dependency poisons, evaluation-call counts, authentic runner parity |
| `INV-LANDSURFACEENERGY-163` | private covered-leaf maximum-demand exact-reuse classifier | runtime/test | reuse only an identical successful leaf result or execute the unchanged complete beta-one call | per-branch and exact-beta differential oracle, call counts, boundary probes, typed-error precedence, authentic runner parity |
| `INV-LANDSURFACEENERGY-164` | versioned/hashed topology-generic component-temperature graph, immutable sweep base and single-use signed-probe replay through one shared canonical evaluator tail | runtime/test | select complete evaluation before replay on ordinary ineligibility; otherwise replay reachable nodes in source order or fail directly on integrity/post-start error | forced-complete oracle, exact direct-edge graph, normative fallibility/crossability matrix, source-real error/rollback corpus, scoped/aggregate buckets, reciprocal-longwave, duplicated wet routing, terminal descendants, full-solve/release parity |
| `INV-LANDSURFACEENERGY-041` | provenance/no-proxy review | governance | blocked promotion | baseline map + reviews |
| `INV-LANDSURFACEENERGY-042` | future recipient-specific radiation ledger and poison vectors | runtime + test | blocked promotion on omitted, duplicated, or aliased recipient | vegetation/LSE integration package |
| `INV-LANDSURFACEENERGY-043` | future latent mass-energy lineage join | runtime + test | blocked promotion on missing/mismatched `h_v`, duplicate debit, or amount/rate basis mismatch | vegetation/LSE integration package |

<a id="producer-obligations-and-consumer-obligations"></a>
## Producer Obligations and Consumer Obligations


<a id="gaps"></a>
<a id="gap-register-and-promotability-labels"></a>
## Gap Register and Promotability Labels

| Gap ID | Gap | Required closure | Label |
|---|---|---|---|
| `GAP-LANDSURFACEENERGY-001` | V1/v2 lacked a complete snow-free surface-temperature and coupled energy-storage algorithm. | Version 3 named model and independent vectors. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-002` | V1/v2 lacked jointly authorized sensible, latent, ground, liquid advection, and storage families. | Version 3 exact owner/source equations. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-003` | V1/v2 lacked complete latent heat, storage, resistance, substrate, and tolerance authority. | Version 3 strict configuration and numerical contract. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-004` | No first-class runtime state, ledger, domain error, scheduler span, or real downstream consumer exists. | Later scoped implementation plus real-consumer proof. | `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-LANDSURFACEENERGY-005` | Schema-v8 snow terminal liquid, energy, and remaining time are censored. | Atomic two-contract cutover with exact-one custody, rollback/defaults, and receiving-surface closure. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-LANDSURFACEENERGY-006` | Legacy daily ET and frost mechanics are not complete LSE authority. | Version 3 uses the selected external stack; legacy remains unchanged comparator behavior. | authority portion admitted; runtime/cutover pending |
| `GAP-LANDSURFACEENERGY-007` | V1 soil-thermal binary64 state cannot retain accepted energy below the high-term ULP. | Version 15 V2 owner/receipt/restart/checkpoint exact carry, exact reconstruction, rollback, and real WAT5/`p61`/native-forest adoption. | `AUTHORITY_ADMITTED`, implementation pending; `NON_PROMOTABLE` |
| `GAP-LANDSURFACEENERGY-008` | Frozen LSE V3 and surface-owner V2 binary64 surface enthalpy cannot retain an accepted per-tile energy credit below the high-term ULP. | Version 16 LSE exact-surface owner/receipt/restart/checkpoint, immutable high-mirror join, exact reconstruction, rollback, and real `p61`/native-forest adoption. | `AUTHORITY_ADMITTED`, implementation pending; `NON_PROMOTABLE` |

The first safe later implementation slice is a default-off, snow-free-only
typed request/state/component-ledger/result using only admitted operands,
explicit duration, atomic validation, and an independent internal ledger
consumer. It must reject snow and terminal handoffs and must not mutate ET,
runoff, infiltration, soil, or frost. That slice is evaluation mechanics only;
production coupling requires later authority and a real process consumer.

The immutable definition identity is
`sha256:e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`
for
`docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_snow_free_lse_v1_definition.json`.

<a id="current-schema-reference"></a>
# Current schema reference

Original normative presentation; no obligation is historical or demoted.

<a id="variables"></a>
<a id="variables-and-units-using-canonical-symbols-first"></a>
## Variables and Units Using Canonical Symbols First

Positive energy is into the surface control volume. Positive water mass is
into surface storage; named outgoing water terms remain non-negative.

| Symbol | Units | Meaning | Owner/status |
|---|---|---|---|
| `A` | `m^2` | represented horizontal area, strictly positive | geometry input |
| `dt` | `s` | exact interval duration, strictly positive | scheduler input |
| `T_s` | `K` | land-surface temperature at an identified state point | future LSE state |
| `E_s,0`, `E_s,1` | `J m^-2` | start/end surface-control-volume energy | future LSE state |
| `M_l,0`, `M_l,1` | `kg m^-2` | start/end liquid held at the surface | future LSE state |
| `R_sw`, `R_lw` | `W m^-2` | signed net shortwave and net longwave into surface | future exact-one ledger |
| `H` | `W m^-2` | signed sensible heat into surface | future exact-one ledger |
| `LE` | `W m^-2` | signed latent heat into surface; evaporation is negative | future exact-one ledger |
| `Q_p`, `Q_runon` | `W m^-2` | precipitation/runon advected heat into surface | future exact-one ledger |
| `Q_inf`, `Q_runoff` | `W m^-2` | non-negative energy advected out with infiltration/runoff | future exact-one ledger |
| `G` | `W m^-2` | ground heat into surface; soil sees exactly `-G` | shared boundary |
| `m_p`, `m_runon` | `kg m^-2` | liquid precipitation and runon admitted during interval | climate/hydrology inputs |
| `m_evap` | `kg m^-2` | actual surface-water evaporation debit | `SC-EVAP-001` handoff |
| `m_inf`, `m_runoff` | `kg m^-2` | infiltration and runoff debits | hydrology handoffs |
| `m_terminal`, `Q_terminal`, `dt_terminal` | contract units | censored schema-v8 snow-terminal payload | prohibited v1 input |
| `epsilon_E`, `epsilon_M` | `J m^-2`, `kg m^-2` | independently reconstructed closure residuals | guards only |
| `rho_E`, `rho_M` | `dimensionless` | non-negative relative closure coefficients | authority gap |
| `H_hi,k` | `J m^-2 OFE-ground` | finite binary64 high term of soil-layer enthalpy | soil-thermal V2 owner |
| `R_k` | `J m^-2 OFE-ground` | exact normalized signed-dyadic soil-layer enthalpy carry | soil-thermal V2 owner |
| `E_k` | `J m^-2 OFE-ground` | exact soil-layer enthalpy `exact(H_hi,k)+R_k` | soil-thermal V2 owner |
| `Q_soil,k`, `Q_top,k`, `Q_inf,k` | `J m^-2 OFE-ground interval` | exact accepted soil-internal, top-boundary, and infiltration energy operands | typed receipt inputs |
| `U_hi,t` | `J m^-2 tile-ground` | finite binary64 high term mirrored into frozen LSE V3 and surface-owner V2 fields | LSE exact-surface owner; mirror only on the V16 successor path |
| `R_U,t` | `J m^-2 tile-ground` | exact normalized signed-dyadic surface-enthalpy carry | LSE exact-surface owner |
| `U_t` | `J m^-2 tile-ground` | authoritative exact surface enthalpy `exact(U_hi,t)+R_U,t` | LSE exact-surface owner |
| `Q_surface,t,j` | `J m^-2 tile-ground interval` | exact dyadic decode of one finite accepted phase-free, fusion, or retained-ingress energy operand | typed surface-energy receipt input |

<a id="state"></a>
<a id="algorithm-state-surfaces"></a>
## Algorithm State Surfaces

Required future inputs are one state identity, area, interval, forcing lineage,
start energy and liquid storage, authoritative component records, and
authoritative water-transfer records. Required outputs are end state, a sealed
component ledger, closure residuals/tolerances, branch identity, and provenance
for every operand. Mutated state is limited to `E_s` and `M_l`; adjacent owners
mutate their own soil, frost, ET, infiltration, and runoff states.

No current `DirectDayFrame` field satisfies this state surface. No production
consumer currently reads a land-surface energy result. A stateless sum of
fluxes is diagnostic mechanics, not this state or consumer proof.

Allowed degenerate states include zero flux components and zero surface liquid
when every closure identity remains defined. Invalid states include non-finite
values, `A <= 0`, `dt <= 0`, negative storage beyond tolerance, ambiguous
lineage, snow presence, and any terminal schema-v8 payload.

Exact future precondition: authoritative upstream state reports zero represented
snow at the interval start, no snow-terminal event or payload exists, every
required lineage ID occurs once, and all domain guards pass. Exact future
postcondition: the sealed end state shares the input state/interval identity,
both independently reconstructed residuals pass their own tolerance, and every
adjacent-owner handoff is accounted once without an uncommitted mutation.

<a id="aliases"></a>
<a id="symbol-alias-map"></a>
## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `T_s` | none | future LSE state | gap | this contract |
| legacy `surtmp(hour)` / `Thra` | not an alias of `T_s` in v1 | pinned frost/tmpadj input in `degC` | future named `degC <-> K` conversion and atomic cutover required | `SC-SNOWFREEZE-001` |
| `R_sw`, `R_lw`, `H`, `LE`, `Q_p`, `Q_runon`, `Q_inf`, `Q_runoff`, `G` | `SurfaceEnergyBalanceTerms` members are mechanics-only candidates | meteorology helper | `W m^-2`; no runtime alias admitted | this contract |
| `m_evap` | `Es`/`Er`/`Ep` are not interchangeable aliases | ET handoff | named `m` or `kg m^-2` conversion required | `SC-EVAP-001` |
| `m_inf`, `m_runoff` | infiltration/runoff records | direct hydrology | named depth-to-area-mass conversion required | `SC-WATBAL-001`, `SC-RUNOFFPART-001` |
| `m_terminal`, `Q_terminal`, `dt_terminal` | schema-v8 terminal fields | snow trace only | prohibited | `SC-SNOWENERGY-001` |
| `H_hi` | `SoilThermalLayerStateV2.enthalpy_hi_j_m2_ofe_ground` | soil-thermal persistent state | `J m^-2 OFE-ground`; finite binary64 nearest-even from exact total | this contract |
| `R` | `SoilThermalLayerStateV2.enthalpy_carry` / `ExactDyadicEnthalpy` | soil-thermal persistent state | `J m^-2 OFE-ground`; canonical normalized signed dyadic | this contract |
| `Q_soil/Q_top/Q_inf` | `SoilThermalEnergyCreditReceiptV2.accepted_operands` | candidate credit receipt | `J m^-2 OFE-ground interval`; exact finite-binary64 dyadics | this contract + `SC-SURFACELIQUID-001` |
| `U_hi` | `LseSurfaceEnthalpyStateRecordV1.enthalpy_hi_j_m2_tile` and the bit-identical frozen V3/V2 `surface_enthalpy_j_m2_tile[_ground]` fields | successor persistent LSE high term; frozen fields are mirrors | `J m^-2 tile-ground`; finite binary64 nearest-even from exact total | this contract |
| `R_U` | `LseSurfaceEnthalpyStateRecordV1.enthalpy_carry` / `ExactDyadicEnthalpy` | successor persistent LSE carry | `J m^-2 tile-ground`; canonical normalized signed dyadic | this contract |
| `Q_surface` | `LseSurfaceEnthalpyEnergyCreditReceiptV1.accepted_operands` | candidate surface-energy receipt | `J m^-2 tile-ground interval`; exact finite-binary64 dyadics with typed chronology and source | this contract + `SC-SURFACELIQUID-001` |

<a id="constants"></a>
<a id="constants-and-parameters-with-provenance-anchors"></a>
## Constants and Parameters with Provenance Anchors

| Constant/parameter | Units | Status | Authority |
|---|---|---|---|
| water density `rho_w` | `1000 kg m^-3` | fixed conversion constant | unit governance / adjacent water contracts |
| latent heat `L_v(T_s)` | `J kg^-1` | required constitutive function; exact authority missing | `GAP-LANDSURFACEENERGY-003` |
| all roughness, resistance, conductivity, heat-capacity, emissivity, and albedo inputs | contract-declared | no new value admitted by v1 | owning contracts/future authority |

Legacy fallback constants and clamps in `tmpadj.for`, `evap.for`, or
`evappm.for` are not silently admitted as LSE parameters.
Legacy `surtmp`/`Thra` remains an `SC-SNOWFREEZE-001` boundary and is neither
consumed nor superseded here. A future package must define whether `T_s`
replaces or supplies it, prove the named Celsius/Kelvin conversion, and cut
over both owners atomically; until then the seam is `NON_PROMOTABLE`.

<a id="units"></a>
<a id="unit-governance-map"></a>
## Unit-Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `T_s` | `K` | future typed temperature | named Celsius/Kelvin helper | none | none |
| energy fluxes | `W m^-2` | future typed flux | named flux-duration integration | no final scalar exception | none |
| `E_s`, energy residual | `J m^-2` | future typed area energy | named integration only | internal guarded scalar candidate | none |
| water stores/transfers | `kg m^-2` | future typed area mass | named depth/area-mass conversion | no final scalar exception | none |
| `dt` | `s` | future typed duration | named cadence conversion | none | none |
| `H_hi,R,E,Q_soil,Q_top,Q_inf` | `J m^-2 OFE-ground` | successor exact-carry registry entries required before promotion | no conversion; exact binary64-to-dyadic decode and arbitrary-precision integer aggregation | internal typed amount; no floating residual exception | none |
| `U_hi,R_U,U,Q_surface` | `J m^-2 tile-ground` | successor surface-enthalpy exact-carry registry entries required before promotion | no conversion after the named retained OFE-to-tile credit is accepted; exact binary64-to-dyadic decode and arbitrary-precision integer aggregation | internal typed amount; no floating residual exception | none |

No publication is authorized. A future implementation must update the machine
registry and pass its unit guards before a dimensional runtime surface lands.

<a id="step-local-preconditions-intermediates-and-postconditions"></a>
### Step-Local Preconditions, Intermediates, and Postconditions

| Step | Preconditions | Computed intermediate and domain | Postcondition |
|---|---|---|---|
| 1 validate | complete immutable request; no state mutation | finite `A > 0 m^2`, `dt > 0 s`; finite unit-typed operands; unique lineage counts are integers exactly one | validated snapshot or one typed error; state byte-identical on error |
| 2 branch | validated explicit snow-state and terminal-field presence | enumerated branch in `{snow_terminal, snow_present, snow_free}` | terminal rejected; snow delegated; only snow-free advances |
| 3 acquire | snow-free branch; named owner for every operand | sealed component/water records with matching state and interval IDs | exactly one record per required operand or `LSEB-E-010` |
| 4 energy reconstruct | sealed component records | `delta_E = E_s,1-E_s,0` and `sum_E = dt*(R_sw+R_lw+H+LE+Q_p+Q_runon-Q_inf-Q_runoff+G)`, both finite `J m^-2`; `epsilon_E=delta_E-sum_E`, finite `J m^-2` | `&#124;epsilon_E&#124;` satisfies `TOL-LANDSURFACEENERGY-001` or `LSEB-E-011` |
| 5 water reconstruct | sealed water records | `delta_M=M_l,1-M_l,0`, `sum_M=m_p+m_runon-m_evap-m_inf-m_runoff`, `epsilon_M=delta_M-sum_M`, all finite `kg m^-2` | mass tolerance passes and `M_l,1` is non-negative within tolerance, else `LSEB-E-012/015` |
| 6 latent join | passing ledgers; liquid-evaporation branch | `Q_LE=LE*dt` finite `J m^-2`; `Q_evap=-L_v(T_s)*m_evap` finite `J m^-2`; exact same lineage | admitted latent tolerance passes or `LSEB-E-013`; tolerance/constitutive authority currently missing |
| 7 advection join | passing ledgers; each liquid crossing identified | four mass/energy pairs with same interval, reference state, temperature/enthalpy, and lineage; energy finite `J m^-2` | every pair occurs exactly once or `LSEB-E-010/011`; constitutive authority currently missing |
| 8 ground join | passing ledger; one soil/frost consumer identified | surface `G` and consumer `-G`, finite `W m^-2`, same interval/lineage | equal/opposite pair occurs once or `LSEB-E-014` |
| 9 atomic commit | all preceding postconditions pass | sealed end state and ledger; no additional numeric intermediate | `E_s,1` and `M_l,1` commit together and consumer handoffs become visible together; otherwise no mutation |

The missing tolerances and constitutive values in steps 4-8 are precisely
`GAP-LANDSURFACEENERGY-001..004`; therefore the table makes the ledger
mechanics reproducible but does not make the runtime algorithm promotable.

<a id="numerical-presentation-reference"></a>
# Numerical presentation reference

Original normative V11/V13/V2 wording is retained below. The complete operative
rules and evidence duties are required through numerical-methods and its admission
dependency. No requirement is historical or waived; source/interpretation
adjudication reads this whole section.

<a id="version-11-inactive-liquid-vapor-coordinate-domain-amendment"></a>
## Version 11 inactive liquid-vapor coordinate domain amendment

The sun-leaf, shade-leaf, and wet-surface temperature coordinates use the
covered solver's liquid-vapor saturation law, whose admitted phase domain
begins at `T_ref = 273.15 K`. When one of those component coordinates is
numerically inactive because its physical component area is exactly zero, or
when the existing `INV-LANDSURFACEENERGY-113` wet-coordinate predicate has
already proven its physical energy residual inside the canonical tolerance,
its deterministic representational anchor is

```text
T_inactive_liquid_vapor - max(T_canopy, T_ref) = 0.
```

The dry-stem inactive anchor remains exactly `T_stem - T_canopy = 0` because
that coordinate does not invoke the liquid-vapor law. The amended target only
keeps an otherwise unconstrained numerical coordinate inside its existing
constitutive domain. For an exactly zero-area component the Newton row is the
direct deterministic anchor row; it does not inherit the nondifferentiable
phase-boundary slope from a finite difference through `max`. An exactly
zero-area component contributes no physical
radiative, sensible, latent, mass, enthalpy, or ledger operand, and the
existing V10 inactive-wet predicate still requires the unanchored physical
wet-energy residual to pass before the row substitution. Active components,
physical residual equations, tolerances, ledgers, receipts, events, the exact
60-second raw fallback, backtracking limits, rollback, and fail-closed behavior
are unchanged.

| ID | Binding rule | Guard/failure |
|---|---|---|

<a id="version-13-first-domain-valid-no-update-termination-amendment"></a>
## Version 13 first-domain-valid no-update termination amendment

The covered solver already admits a no-update termination when the current
complete normalized residual vector passes and a valid full Newton trial proves
that every governed prospective component step is inside its unchanged
threshold. A closed phase bound can make that full trial invalid, while an
otherwise valid full trial can exceed one unchanged governed step threshold.
In either case a deterministic halving can produce a domain-valid governed
step too small to produce an observable strict residual decrease in binary64.

### Witness coordinates and public/persisted diagnostics

For both owner-uncapped potential and fixed-authorization final solves, first
retain the existing no-update witness on a domain-valid full `b=0` Newton
trial. If and only if the current complete residual vector passes and that
full-trial witness cannot accept because the full trial is outside the existing
covered-trial domain or any governed full-trial prospective step exceeds its
unchanged threshold, examine the existing ordered backtracking sequence from
`b=1` until its first domain-valid halved trial. A
complete prospective evaluation of that trial supplies the exact applied
hydraulic, beta, temperature and humidity norms and the diagnostic derived
`ci` norm. When all four governed norms pass their unchanged thresholds,
accept the current iterate without applying, projecting, or publishing any
part of the trial. Record the prospective norms and add the exact examined
exponent to the existing cumulative backtracking-count diagnostic; do not add a
separate public or persisted field. Retain the current solution, evaluation,
active branches, state, water requests/uses, ledgers, owner candidates, and
closure operands exactly.

The halved witness is refused when the current residual vector has any nonfinite or
out-of-tolerance member; when the full trial itself passes the existing
no-update witness; when the first domain-valid halved trial cannot be
completely evaluated; or when any governed prospective step norm fails. After
refusal, that same first domain-valid trial
and all later factors remain eligible only under the unchanged strict-residual-
decrease rule for an actual installed update. Exhaustion remains the existing
typed backtracking-limit failure with exact rollback. The solver may not skip
the first domain-valid candidate to obtain a smaller no-update witness.

This amendment changes no closed bound, constitutive equation, residual or
step threshold, finite-difference rule, pivot rule, iteration/backtracking
limit, active branch, event chronology, 60-second floor, mass/energy ledger,
receipt, custody, topology, publication, or rollback rule. It admits no trial
clamp or projected state and does not turn strict decrease into a tolerance;
strict decrease remains mandatory for every actual update.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-139` | After the full Newton trial fails the existing no-update witness by domain invalidity or a governed step excess, a passing current residual vector and passing governed step norms on the first domain-valid halved trial accept only the unchanged current iterate. | positive full-outside and full-step-excess/first-valid witnesses; residual/nonfinite, already-passing-full, per-thresholded-coordinate, prospective-evaluation and skip-first poisons; unchanged strict-decrease update/backtracking-limit/rollback vectors |

Required real-consumer vectors are the two interior terminal-event paths that
previously reached `FinalFixedCap` iteration 4 and exhausted 20 halvings. They
must complete with current-state acceptance, unchanged owner closure, and no
trial installation. Existing oracle backtracking-limit and genuine strict-
decrease vectors remain required and must retain their prior disposition.

<a id="openwepp_snow_free_lse_v2-v10-coupling-amendment"></a>
## `OPENWEPP_SNOW_FREE_LSE_V2` V10 Coupling Amendment

V2 imports every V1 control volume, tolerance, owner, rollback rule, and
positive-PAR accepted result. It requires the V10 vegetation identity and
admits its exact-zero-PAR and respiration-dominated positive-low-light
branches. It does not recompute, clamp, or relabel V10 gas states.

### Final-solve initialization and diagnostic accounting

#### Full-supply initialization

When every positive final water authorization is identity- and amount-equal to
its potential request with `FullSupply`, and every canonical zero request
retains its exact identity and zero amount, V2 uses the accepted potential
coordinates as the deterministic fixed-final initial iterate. It rebuilds the
complete fixed-final evaluation from immutable beginning owners and exact
per-resource caps. No potential flux, candidate state, branch, receipt, or
diagnostic is copied.

#### Iteration-zero acceptance

If that initial evaluation satisfies every residual tolerance, active-branch
inequality, domain and bound, `F<=A<=D`, identity, and owner check, V2 accepts
at iteration zero with exact-zero step norms and zero backtracking without
constructing a Jacobian. Every actual solver step retains V1 strict-decrease
and convergence rules. Copying the potential candidate without complete final
reevaluation is forbidden. A residual outside tolerance or any branch,
identity, owner, or amount mismatch cannot use this acceptance path.

### V10 domain and numerical restrictions

#### Authorization exclusions

Nonpositive-assimilation partial positive root authorization is typed unsupported in V2.

It does not invoke hydraulic attenuation, conductance or vulnerability floors,
plant capacitance, or authorization donation.

#### Derivative scope

Every covered potential and final solve uses the exact closed-bound derivative
rule in `INV-LANDSURFACEENERGY-138`. This general numerical-domain rule does
not broaden the V10-only coordinate-scaling authority below.

#### V10 unit scaling

For that same uncapped active V10 nonpositive-assimilation potential solve only, the
Newton linear system is expressed in the declared coordinate units. With
`x = D y`, where `D` is the exact finite-difference unit-scale diagonal, the
solver forms `J_y[:,j] = J_x[:,j] * D[j]`, applies the canonical pivot test to
`J_y`, solves `J_y delta_y = -r`, and maps
`delta_x[j] = D[j] * delta_y[j]`. This deterministic nondimensionalization
changes neither the residual equations nor the represented physical Newton
direction. It is forbidden for V1, V8/V9-derived behavior, positive-PAR V10,
and fixed-final solves. Reducing the pivot multiplier, accepting a rejected
pivot, regularization, larger iteration/trust bounds, or physiological floors
is not an equivalent implementation.

#### Potential wet-store eligibility

When the uncapped V10 nonpositive-assimilation potential evaluation selects the
canopy-liquid store-cap branch and the preliminary store rate is no larger
than the canonical water residual tolerance, the wet-surface temperature is a
numerically inactive coordinate: the wet-energy residual is already below its
admitted closure scale and cannot determine that temperature. V2 replaces
that one row with the existing inactive-component anchor
`T_wet - max(T_canopy, 273.15 K) = 0`. Liquid mass, enthalpy, longwave area, and every owner
ledger remain evaluated normally; no liquid amount is clamped or discarded.
The predicate additionally requires the unanchored physical wet-energy
residual already satisfy its canonical energy tolerance. It is unavailable to
V1, positive-assimilation V10, condensation, or a constitutive-law wet flux.

#### Identity migration

V1-to-V2 migration validates complete V1 and V10 owner identities, copies all
LSE scientific values bit-identically, and changes only the LSE identity and
transitively derived receipts. V1 remains immutable and is not a V2 alias.

| ID | Binding rule |
|---|---|
| `LSE-E-109` | V8/V9 vegetation identity, mixed V1/V2 receipts, or any owner alias rejects before V2 physics. |
| `LSE-E-110` | Missing, duplicated, mutated, or locally recomputed V10 nighttime gas state rejects the coupled owner envelope. |
| `LSE-E-111` | Partial or value-mutating V1-to-V2 migration rejects without a V2 state. |

This amendment remains default-off. It authorizes no production selector,
default/output change, cutover, snow handoff, deployment, calibration, or
empirical claim.

