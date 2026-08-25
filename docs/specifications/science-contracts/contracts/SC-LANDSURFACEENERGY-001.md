---
contract_id: SC-LANDSURFACEENERGY-001
title: Land-Surface Energy-Balance Process Contract
status: approved
maturity: active
owner: openWEPP maintainers + land-surface-energy/hydrology reviewer
contract_version: 8
released_contract_version: 8
candidate_contract_version: 9
producer_scope:
  - Future snow-free land-surface energy control-volume evaluator
  - Future post-snow receiving-surface evaluator after an atomic handoff cutover
  - Persistent Stage 3 snow--soil lower-boundary evaluator
consumer_scope:
  - Future soil-heat/frost boundary, evaporation, infiltration/runoff, and surface-water ledgers
evidence_level: static+independent_oracle
last_reviewed: 2026-08-24
supersedes: []
superseded_by: []
---

# SC-LANDSURFACEENERGY-001 Land-Surface Energy-Balance Process Contract

Status: `approved`
Maturity: `active`
Evidence mode: `static + independent oracle`

Lifecycle: version 8 remains `approved / active`; only the version-9 terminal
endpoint amendment is `in_review / draft` until its own mandatory review,
verification, implementation, and exact-head gates pass.

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

All pinned citations mean `git show
dac3c950d8b16cc73774bf5ce2e7e11f80baac70:<path>`; the mutable checkout
HEAD is not normative.

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

## Algorithm Specification with Step Sequence

The future algorithm shall execute atomically for one interval:

1. Validate state identity, units, finiteness, `A`, `dt`, branch predicates,
   lineage uniqueness, and all required inputs before mutation.
2. Select exactly one branch. Version 1 only defines the `snow_free` authority
   boundary; `snow_present` delegates without evaluation and `snow_terminal`
   fails closed.
3. Obtain, without recomputation, one authoritative record for each admitted
   component. A missing constitutive family blocks implementation.
4. Reconstruct the signed energy identity independently:

   `E_s,1 - E_s,0 = dt * (R_sw + R_lw + H + LE + Q_p + Q_runon - Q_inf - Q_runoff + G)`.

5. Reconstruct surface-liquid custody independently:

   `M_l,1 - M_l,0 = m_p + m_runon - m_evap - m_inf - m_runoff`.

6. Bind latent mass and energy exactly once when phase is liquid evaporation:

   `LE * dt = -L_v(T_s) * m_evap`.

   Other vapor/phase branches require their own admitted authority and may not
   alias this identity.
7. Bind every liquid crossing to one advected-energy record using the same
   mass, interval, temperature/enthalpy reference state, and lineage:
   precipitation to `Q_p`, runon to `Q_runon`, infiltration to `Q_inf`, and
   runoff to `Q_runoff`. The constitutive enthalpy/reference-state authority is
   missing in v1; omitting these terms is not permitted closure.
8. Bind the shared ground boundary as equal and opposite: the surface records
   `G`, while the sole soil/frost consumer records `-G`; neither side computes
   or applies a second ground flux.
9. Commit both state ledgers only after every guard passes. Any failure leaves
   all producer and consumer state unchanged and returns a typed `LSEB-E-*`
   error.

These equations define conservation and custody, not the missing constitutive
equations for `T_s`, `H`, `LE`, `Q_p`, `G`, or energy storage.

### Step-Local Preconditions, Intermediates, and Postconditions

| Step | Preconditions | Computed intermediate and domain | Postcondition |
|---|---|---|---|
| 1 validate | complete immutable request; no state mutation | finite `A > 0 m^2`, `dt > 0 s`; finite unit-typed operands; unique lineage counts are integers exactly one | validated snapshot or one typed error; state byte-identical on error |
| 2 branch | validated explicit snow-state and terminal-field presence | enumerated branch in `{snow_terminal, snow_present, snow_free}` | terminal rejected; snow delegated; only snow-free advances |
| 3 acquire | snow-free branch; named owner for every operand | sealed component/water records with matching state and interval IDs | exactly one record per required operand or `LSEB-E-010` |
| 4 energy reconstruct | sealed component records | `delta_E = E_s,1-E_s,0` and `sum_E = dt*(R_sw+R_lw+H+LE+Q_p+Q_runon-Q_inf-Q_runoff+G)`, both finite `J m^-2`; `epsilon_E=delta_E-sum_E`, finite `J m^-2` | `|epsilon_E|` satisfies `TOL-LANDSURFACEENERGY-001` or `LSEB-E-011` |
| 5 water reconstruct | sealed water records | `delta_M=M_l,1-M_l,0`, `sum_M=m_p+m_runon-m_evap-m_inf-m_runoff`, `epsilon_M=delta_M-sum_M`, all finite `kg m^-2` | mass tolerance passes and `M_l,1` is non-negative within tolerance, else `LSEB-E-012/015` |
| 6 latent join | passing ledgers; liquid-evaporation branch | `Q_LE=LE*dt` finite `J m^-2`; `Q_evap=-L_v(T_s)*m_evap` finite `J m^-2`; exact same lineage | admitted latent tolerance passes or `LSEB-E-013`; tolerance/constitutive authority currently missing |
| 7 advection join | passing ledgers; each liquid crossing identified | four mass/energy pairs with same interval, reference state, temperature/enthalpy, and lineage; energy finite `J m^-2` | every pair occurs exactly once or `LSEB-E-010/011`; constitutive authority currently missing |
| 8 ground join | passing ledger; one soil/frost consumer identified | surface `G` and consumer `-G`, finite `W m^-2`, same interval/lineage | equal/opposite pair occurs once or `LSEB-E-014` |
| 9 atomic commit | all preceding postconditions pass | sealed end state and ledger; no additional numeric intermediate | `E_s,1` and `M_l,1` commit together and consumer handoffs become visible together; otherwise no mutation |

The missing tolerances and constitutive values in steps 4-8 are precisely
`GAP-LANDSURFACEENERGY-001..004`; therefore the table makes the ledger
mechanics reproducible but does not make the runtime algorithm promotable.

## Branch and Guard Table

| Condition | Required disposition | Failure |
|---|---|---|
| snow absent and no terminal payload | `snow_free`; evaluate only after all authority families are admitted | typed failure while gaps remain |
| snow present | delegate exclusively to snow owner; no LSE mutation | `LSEB-E-020` on attempted dual evaluation |
| snow terminal/censored payload present | reject; there is no v1 recipient | `LSEB-E-021` |
| non-finite/unit/domain failure | reject before mutation | `LSEB-E-001` |
| duplicate/missing component lineage | reject | `LSEB-E-010` |
| energy or water closure exceeds tolerance | reject atomically | `LSEB-E-011` / `LSEB-E-012` |
| latent mass-energy mismatch | reject | `LSEB-E-013` |
| ground-flux dual ownership | reject | `LSEB-E-014` |
| negative end storage beyond tolerance | reject; no clamp/default | `LSEB-E-015` |
| positive successor support is below the active adopter domain | reject before Newton and restore every owner/receipt byte | `LSEB-E-041` |
| support/adopter/event receipt join, duration, policy, or owner digest is invalid | reject before physical evaluation | `LSEB-E-042` |
| post-event operand or support chronology is invalid | reject before physical evaluation | `LSEB-E-043` |

Branch priority is `snow_terminal` rejection, then `snow_present` delegation,
then `snow_free`. No temperature-only guess may override explicit snow state.

## Invariants and Invariant Guard Map

| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |
|---|---|---|---|---|---|
| `INV-LANDSURFACEENERGY-001` | Every dimensional operand has explicit units, interval, area basis, finite domain, and provenance. | REF-009, unit governance | `[INFERENCE][Static]` | profile/test | hard `HOLD` |
| `INV-LANDSURFACEENERGY-002` | `A > 0`, `dt > 0`, temperatures satisfy their declared absolute/log domains, and resistances/conductivities satisfy admitted domains. | REF-009 | `[INFERENCE][Static]` | future runtime/test | typed failure |
| `INV-LANDSURFACEENERGY-010` | The energy identity is independently reconstructible with every signed radiative, turbulent, advective, ground, and storage component exactly once. | REF-009 | `[INFERENCE][Static]` | future runtime/test | `LSEB-E-011` |
| `INV-LANDSURFACEENERGY-011` | Surface-liquid start, inputs, debits, and end storage close independently. | REF-001/004/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-012` |
| `INV-LANDSURFACEENERGY-012` | Actual liquid evaporation has one shared mass/latent-energy identity and one state debit. | REF-005/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-013` |
| `INV-LANDSURFACEENERGY-013` | Surface `G` and soil/frost `-G` are one interface transfer, never two production fluxes. | REF-006/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-014` |
| `INV-LANDSURFACEENERGY-014` | Each precipitation, runon, infiltration, and runoff mass crossing has exactly one linked advected-energy term with the same lineage/reference state. | REF-004/009 | `[DIRECT][Static] + [INFERENCE][Static]` | future runtime/test | `LSEB-E-010/011` |
| `INV-LANDSURFACEENERGY-015` | End liquid storage is non-negative within its admitted mass tolerance; material negative storage is never clamped. | REF-009 | `[INFERENCE][Static]` | future runtime/test | `LSEB-E-015` |
| `INV-LANDSURFACEENERGY-020` | Snow-present, snow-terminal, and snow-free branches are mutually exclusive with the declared priority. | REF-008 | `[DIRECT][Static]` | future runtime/test | `LSEB-E-020/021` |
| `INV-LANDSURFACEENERGY-021` | Schema-v8 terminal liquid, energy, and time are censored and cannot enter this contract. | REF-008 | `[DIRECT][Static]` | contract/runtime/test | `LSEB-E-021` |
| `INV-LANDSURFACEENERGY-022` | LSE cannot reorder or duplicate hydrology/ET mutations; it consumes sealed handoffs. | REF-004/005/007 | `[DIRECT][Static]` | scheduler/consumer gate | hard `HOLD` |
| `INV-LANDSURFACEENERGY-030` | Climate owns forcing and phase; LSE owns surface flux evaluation, not forcing correction. | REF-007 | `[DIRECT][Static]` | ownership test | hard `HOLD` |
| `INV-LANDSURFACEENERGY-031` | Constitutive equations remain with their named owner; conservation orchestration does not confer duplicate ownership. | REF-007/009 | `[INFERENCE][Static]` | review/test | hard `HOLD` |
| `INV-LANDSURFACEENERGY-032` | Producer-only, skeleton-only, or serialized ledger evidence cannot prove a runtime path; a real downstream consumer must read and act on the result. | governance | `[DIRECT][Static]` | consumer gate | hard `HOLD` |
| `INV-LANDSURFACEENERGY-040` | No production implementation is promotable while any required family is `AUTHORITY_MISSING`. | REF-003-010 | `[DIRECT][Static] + [INFERENCE][Static]` | governance gate | `NON_PROMOTABLE` |
| `INV-LANDSURFACEENERGY-041` | Comparator agreement, current helper code, or silent legacy clamps cannot substitute for canonical physics authority and typed guards. | ADR-0017, REF-010 | `[DIRECT][Static]` | review/gate | hard `HOLD` |
| `INV-LANDSURFACEENERGY-042` | Future vegetation radiation receipts remain recipient-specific across canopy strata, ground, litter, snow, soil, ponded water, and atmosphere; no recipient is an alias or residual bucket for another. | SC-VEGETATION-001#INV-VEGETATION-021 | `[INFERENCE][Static]` | future integration/test | hard `HOLD` |
| `INV-LANDSURFACEENERGY-043` | Interval-integrated Stage C transpiration mass and its latent-energy debit share one transaction, stratum, area, interval, lineage, and authority-tagged `h_v`, satisfying `Q_T,s=-h_v*T_s` exactly once. | SC-VEGETATION-001#INV-VEGETATION-014 | `[INFERENCE][Static]` | future integration/test | hard `HOLD` |
| `INV-LANDSURFACEENERGY-114` | Default-off LSE-V2 selects the actual receiver and rebuilds every flux only on `[wall_t*,wall_end)` without snow operands. | terminal receiver authority | `[INFERENCE][Static]` | runtime/test | typed receiver failure |
| `INV-LANDSURFACEENERGY-115` | The 0 C parcel enters hydrology once; fusion energy is not soil heat, zero remaining support skips LSE, and any failure rolls back all owners. | conservation/transaction authority | `[INFERENCE][Static]` | runtime/test | typed join/rollback failure |
| `INV-LANDSURFACEENERGY-127` | Candidate v9 positive terminal support uses only accepted two-half/root `q_ss` integrals; exact-zero cursor replay uses zero support/no receipt and no fabricated snow node. | released v8 interface; `SC-SNOWENERGY-001#INV-SNOWENERGY-034/051` | `[INFERENCE][Static]` | terminal support/receipt validator | `LSEB-E-045` |
| `INV-LANDSURFACEENERGY-128` | Each accepted joint trial binds start/end state, exact support, carrier and `q_ss`; terminal receipt binds ordered sum, event, soil ending, dormant snow and owners. Full/rejected trials are evidence only. | deterministic two-half/root integration; energy conservation | `[INFERENCE][Static]` | trial/event/owner reconstruction | `LSEB-E-045` |
| `INV-LANDSURFACEENERGY-129` | Terminal snow `-Q_ss` and first-soil-node `+Q_ss` use identical bits and commit atomically; installed validation never projects dormant snow. | released v8 sign/owner authority; first law | `[DIRECT][Static] + [INFERENCE][Static]` | energy/owner join | `LSEB-E-045` |

Guard-map enforcement in version 1 is the contract-derived integration test
and package review. Runtime mappings are intentionally future obligations; an
implementation package must replace each future mapping with a typed path and
evidence artifact before promotion.

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
| `INV-LANDSURFACEENERGY-041` | provenance/no-proxy review | governance | blocked promotion | baseline map + reviews |
| `INV-LANDSURFACEENERGY-042` | future recipient-specific radiation ledger and poison vectors | runtime + test | blocked promotion on omitted, duplicated, or aliased recipient | vegetation/LSE integration package |
| `INV-LANDSURFACEENERGY-043` | future latent mass-energy lineage join | runtime + test | blocked promotion on missing/mismatched `h_v`, duplicate debit, or amount/rate basis mismatch | vegetation/LSE integration package |
| `INV-LANDSURFACEENERGY-127` | candidate-v9 terminal support/receipt classifier | runtime + test | `LSEB-E-045`; exact rollback | Child-1 terminal chronology contract/tests |
| `INV-LANDSURFACEENERGY-128` | accepted joint-trial and receipt reconstruction | runtime + test | `LSEB-E-045`; exact rollback | Child-1 terminal chronology contract/tests |
| `INV-LANDSURFACEENERGY-129` | identical-bit snow/soil custody and dormant installed join | runtime + test | `LSEB-E-045`; exact rollback | Child-1 terminal chronology contract/tests |

## Producer Obligations and Consumer Obligations

- `OBL-LANDSURFACEENERGY-P-001`: produce immutable, source-identified records
  for all water and energy operands and a sealed start-to-end ledger.
- `OBL-LANDSURFACEENERGY-P-002`: validate before mutation and commit energy and
  water state atomically.
- `OBL-LANDSURFACEENERGY-P-003`: expose branch, interval, tolerances, residuals,
  and exact component lineage without silent defaults or clamps.
- `OBL-LANDSURFACEENERGY-P-004`: reject all schema-v8 terminal payloads until a
  reviewed atomic cutover revises both snow and receiving-surface authority.
- `OBL-LANDSURFACEENERGY-C-001`: ET supplies one actual evaporation debit and
  consumes no second latent debit.
- `OBL-LANDSURFACEENERGY-C-002`: infiltration/runoff consumes one water offer,
  returns sealed partition terms, and remains sole water-partition owner.
- `OBL-LANDSURFACEENERGY-C-003`: soil/frost consumes exactly `-G` once and is
  sole subsurface conduction/phase-state mutator.
- `OBL-LANDSURFACEENERGY-C-004`: a real scheduler consumer must prove that the
  new state and ledger affect the intended direct path before runtime closure.
- `OBL-LANDSURFACEENERGY-P-020`: candidate v9 emits one terminal snow--soil
  receipt per positive terminating support or the canonical zero-support
  no-receipt disposition.
- `OBL-LANDSURFACEENERGY-C-020`: candidate v9 independently reconstructs the
  ordered accepted `q_ss` sum and identical-bit snow debit/soil credit without
  a dormant snow projection.

## Symbol Alias Map

| Canonical symbol | Boundary/API name | Scope | Units check | Owner contract |
|---|---|---|---|---|
| `T_s` | none | future LSE state | gap | this contract |
| legacy `surtmp(hour)` / `Thra` | not an alias of `T_s` in v1 | pinned frost/tmpadj input in `degC` | future named `degC <-> K` conversion and atomic cutover required | `SC-SNOWFREEZE-001` |
| `R_sw`, `R_lw`, `H`, `LE`, `Q_p`, `Q_runon`, `Q_inf`, `Q_runoff`, `G` | `SurfaceEnergyBalanceTerms` members are mechanics-only candidates | meteorology helper | `W m^-2`; no runtime alias admitted | this contract |
| `m_evap` | `Es`/`Er`/`Ep` are not interchangeable aliases | ET handoff | named `m` or `kg m^-2` conversion required | `SC-EVAP-001` |
| `m_inf`, `m_runoff` | infiltration/runoff records | direct hydrology | named depth-to-area-mass conversion required | `SC-WATBAL-001`, `SC-RUNOFFPART-001` |
| `m_terminal`, `Q_terminal`, `dt_terminal` | schema-v8 terminal fields | snow trace only | prohibited | `SC-SNOWENERGY-001` |

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

## Unit-Governance Map

| Symbol | Declared units | Boundary registry entry | Conversion helper | Scalar exception | Publication metadata |
|---|---|---|---|---|---|
| `T_s` | `K` | future typed temperature | named Celsius/Kelvin helper | none | none |
| energy fluxes | `W m^-2` | future typed flux | named flux-duration integration | no final scalar exception | none |
| `E_s`, energy residual | `J m^-2` | future typed area energy | named integration only | internal guarded scalar candidate | none |
| water stores/transfers | `kg m^-2` | future typed area mass | named depth/area-mass conversion | no final scalar exception | none |
| `dt` | `s` | future typed duration | named cadence conversion | none | none |

No publication is authorized. A future implementation must update the machine
registry and pass its unit guards before a dimensional runtime surface lands.

## Tolerance and Numeric Notes

For INV-114/115, `wall_t*`, `wall_end`, support identity, receipt cardinality,
and snow-operand absence are exact. Existing LSE energy/mass tolerances apply
only after those checks and cannot repair them.

- `TOL-LANDSURFACEENERGY-001`: future energy closure shall require
  `|epsilon_E| <= max(a_E, rho_E*sum_abs_energy_operands)`; `a_E` has units
  `J m^-2`, `rho_E` is dimensionless and non-negative, and both are authority
  gaps in v1.
- `TOL-LANDSURFACEENERGY-002`: future mass closure shall use a separately
  justified scale-aware predicate `|epsilon_M| <=
  max(a_M, rho_M*sum_abs_mass_operands)`; `a_M` has units `kg m^-2`, `rho_M`
  is dimensionless and non-negative, and both are authority gaps in v1.
- Zero snapping, storage clipping, denominator substitution, or rounding into
  plausibility is prohibited unless a later contract gives a threshold, unit,
  provenance, and tests.
- Closure, solver convergence, phase-event, and representation tolerances are
  distinct and cannot be substituted for one another.

## Calibration and Identifiability

Disposition: `CALIBRATION_NOT_APPLICABLE`.

```text
science_implementation_status = NOT_IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
```

Version 1 admits no empirically estimated parameter. Fixed constants are
science authority, forcing/state is externally supplied, and missing
constitutive parameters remain gaps. Comparator agreement, synthetic recovery,
or an execution assumption is not empirical calibration.

## Test-Vector Obligations

| Vector family | Expected observable/result | Bound invariant/failure |
|---|---|---|
| bare dry snow-free; residue/canopy variants | `snow_free`; one sealed ledger, no implicit cover alias | `INV-001/020/030` |
| calm wind; night/zero shortwave | admitted zero component or typed constitutive-domain failure, never a hidden default | `INV-001/002/041` |
| freezing/thawing `T_s` | named branch/domain result with no implicit `surtmp` alias | `INV-002/031` |
| all-zero flux | unchanged energy and water state with positive `dt` | `INV-010/011` |
| rain, no snow handoff | one `m_p/Q_p` pair; terminal fields absent | `INV-014/020/021` |
| ponded/unponded | one named `M_l` state and no LSE water repartition | `INV-011/022` |
| ET latent versus vapor | one `m_evap/LE` pair; no second vapor debit | `INV-012/031` |
| ground sign reversal | surface `G` equals soil/frost `-G` | `INV-013`, `LSEB-E-014` |
| all-distinct operands | independently reconstructed `epsilon_E/epsilon_M` pass | `INV-010/011` |
| snow present | delegate with no LSE mutation | `INV-020`, `LSEB-E-020` |
| terminal schema-v8 payload | reject with no mutation | `INV-021`, `LSEB-E-021` |
| non-finite/domain/negative storage | reject with no clamp | `INV-002/015`, `LSEB-E-001/015` |
| missing/duplicate lineage | reject before mutation | `INV-010/011/014`, `LSEB-E-010/011/012` |

Poison vectors shall independently omit and duplicate precipitation water and
heat, runon water and heat, infiltration water and heat, runoff water and heat,
latent, sensible, shortwave, longwave, ground heat, storage change, and
evaporation. Additional vectors reject non-finite inputs,
invalid Kelvin/log/roughness/resistance domains, negative storage, duplicate
lineage, wrong branch priority, and every censored schema-v8 terminal payload.
Real-consumer proof remains intentionally unsatisfied in version 1.

### Child 2C test bindings

The Child 2C and candidate-v9 test vectors bind canonical invariant rows
`INV-LANDSURFACEENERGY-121..123` and `127..129`; no invariant definition is
duplicated in this test section.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `LSE-CHILD2C-SUCCESSOR` | `docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/` | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-121, INV-LANDSURFACEENERGY-122, INV-LANDSURFACEENERGY-123` | `flagged-binding-addition` | Accepted event receipt, post-event-only operands, and pre-Newton covered-forest support admission; no storage arithmetic change. |
| `LSE-V9-TERMINAL-SNOW-SOIL-ENDPOINT` | `docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/` | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-127, INV-LANDSURFACEENERGY-128, INV-LANDSURFACEENERGY-129, OBL-LANDSURFACEENERGY-P-020, OBL-LANDSURFACEENERGY-C-020` | `flagged-binding-addition` | Candidate v9 event-integrated terminal snow--soil custody without a fabricated post-event snow node; released v8 persistent custody remains active. |

## Gap Register and Promotability Labels

| Gap ID | Gap | Required closure | Label |
|---|---|---|---|
| `GAP-LANDSURFACEENERGY-001` | V1/v2 lacked a complete snow-free surface-temperature and coupled energy-storage algorithm. | Version 3 named model and independent vectors. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-002` | V1/v2 lacked jointly authorized sensible, latent, ground, liquid advection, and storage families. | Version 3 exact owner/source equations. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-003` | V1/v2 lacked complete latent heat, storage, resistance, substrate, and tolerance authority. | Version 3 strict configuration and numerical contract. | `AUTHORITY_ADMITTED`, implementation pending |
| `GAP-LANDSURFACEENERGY-004` | No first-class runtime state, ledger, domain error, scheduler span, or real downstream consumer exists. | Later scoped implementation plus real-consumer proof. | `IMPLEMENTATION_MISSING`, `NON_PROMOTABLE` |
| `GAP-LANDSURFACEENERGY-005` | Schema-v8 snow terminal liquid, energy, and remaining time are censored. | Atomic two-contract cutover with exact-one custody, rollback/defaults, and receiving-surface closure. | `AUTHORITY_MISSING`, `NON_PROMOTABLE` |
| `GAP-LANDSURFACEENERGY-006` | Legacy daily ET and frost mechanics are not complete LSE authority. | Version 3 uses the selected external stack; legacy remains unchanged comparator behavior. | authority portion admitted; runtime/cutover pending |

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

## `OPENWEPP_SNOW_FREE_LSE_V1` Constitutive Authority

Version 3 prospectively supersedes the version-2 missing-authority posture only
for this named model and supported domain. Historical conservation and custody
requirements remain binding.

### Selected sources and domain

The selected reference-model stack is CLM5 equations 2.5.72--2.5.81,
2.5.86--2.5.153 and 2.6.1--2.6.91; Boone et al. (2017) ISBA-MEB Part 1
equations 4--28 and Appendix I; Napoly et al. (2017) Appendix A equations
A1--A14; FSM2.1.1 equations 25--34; and the ORCHIDEE arbitrary-level longwave
matrix S2.16--S2.24. The exact retained bytes and rights are recorded as
R-153 and R-155--R-158 in `references/annotated_bibliography.md`.

The executable domain requires snow absent at both endpoints, no terminal snow
payload, positive finite neutral-domain wind, positive finite interval and
area, liquid/unfrozen water and soil, complete forcing/configuration/state, one
ground class per tile, and exact owner identity. `bare_mineral_soil` and
`forest_litter` are admitted. Calm wind, nonneutral stability, snow, terminal
snow, frozen or thawing material, multiple ground classes per tile and missing
liquid-temperature lineage return typed unsupported errors before calculation.

### Exact ownership and state

Hydrology exclusively owns ponded, litter-held and soil-layer water mass.
More completely, it owns every water mass: ponded/depression storage,
litter-held water, and every soil-layer liquid/frozen store. LSE owns one
surface thermal node per tile and no water amount. The surface dry body and any
positive hydrology-owned surface store are isothermal at that node. The
soil-thermal owner, not LSE, owns all `N` soil temperatures and enthalpies.
Vegetation retains all canopy processes. No model state contains two mutable
representations of the same mass, temperature, or enthalpy.

Strict configuration, state, forcing, water-protocol, diagnostics, and coupled
owner-envelope machine surfaces are the six package artifacts named
`lse_v1_*_schema.json`; every frozen digest is part of the model definition.
Configuration supplies OFE/tile
topology, one surface class per
tile, VIS/NIR albedo, neutral aerodynamic geometry, surface dry heat capacity,
surface thickness/conductivity, soil-layer geometry and thermal properties,
and litter properties where applicable. LSE persistent state supplies one
physical `surface_enthalpy_j_m2_tile` per tile plus numerical warm starts,
transaction lineage, and digests. `surface_temperature_k` is retained only as
a solver warm start/diagnostic and is reconstructed from enthalpy, immutable
hydrology-owned `W`, and `C_dry` before use; it is not a second physical state.
Hydrology supplies all beginning water amounts, and soil thermal supplies its
`N` temperatures/enthalpies, through immutable owner snapshots with exact
digest lineage. No executable default supplies a scientific value. Unknown,
missing, extra, duplicate, stale, nonfinite, wrong-owner, or out-of-domain
fields reject.

For one OFE, tile fractions are finite, positive, unique by tile ID, and sum
to one under the topology tolerance. `stand-ground` in this contract means
that one OFE's horizontal ground area; it never means an entire routed
hillslope. Tile-local amounts become OFE-ground amounts by multiplication by
`f_t` exactly once. Cross-OFE runon retains source and destination OFE IDs.

### Shortwave and reciprocal longwave

V8 performs the unchanged V7 two-stream shortwave solve using ground
surface-class VIS/NIR albedos as the full-column lower boundary. Ground receives
terminal direct/diffuse VIS/NIR exactly once; reflected radiation traverses
the overlying column and is never sent directly to the atmosphere.

V1 selects unit longwave emissivity and no longwave reflection. For top-to-
bottom occupancies `i=0..n-1`, `P_i=LAI_i+SAI_i` on tile-ground basis and

```text
tau_i = exp[-0.8*Omega_i*P_i].
```

The coefficient `0.8` has units `m2 plant area m-2 ground`; clumping
`0<Omega_i<=1` is applied exactly once. With atmospheric boundary `Ldn_0` and
current trial surface temperature `T_s`,

```text
Ldn_(i+1) = tau_i*Ldn_i + (1-tau_i)*E_i
Lup_n     = sigma*T_s^4
Lup_i     = tau_i*Lup_(i+1) + (1-tau_i)*E_i.
```

For component `j` in the ordered set `{sun_leaf, shade_leaf, wet_surface,
dry_stem}`, let `a_j>=0` be its exact tile-ground emissive area and
`w_j=a_j/sum(a_j)`. If `sum(a_j)=0`, `tau_i=1` and every component longwave
term is exact zero. Otherwise the component net is

```text
R_lw,i,j = w_j*(1-tau_i)*(Ldn_i+Lup_(i+1))
             - 2*w_j*(1-tau_i)*sigma*T_i,j^4.
```

Thus `sum_j R_lw,i,j` equals the layer net and each physical temperature enters
its own emission. The ground term is `R_lw,s=Ldn_n-sigma*T_s^4`. All terms are
recomputed from current nonlinear trial temperatures. A bulk canopy
temperature, prescribed upward ground longwave, stale previous-step ground
temperature, or direct ground-to-atmosphere bypass is noncanonical.

### Neutral turbulent heat and vapor network

Flux signs in this section are positive away from the surface/component and
toward its air recipient. Let `rho_a` and `c_p` be the V8 forcing-derived moist
air density and heat capacity and let `q_s` be the admitted surface humidity.
For any surface-to-air resistance `r_h,r_v>0`,

```text
H_s = rho_a*c_p*(T_s-T_recipient)/r_h
v_s = rho_a*(q_s-q_recipient)/r_v.
```

The signed vapor flux `v_s>0` is evaporation and `v_s<0` is condensation. It
is never clipped to zero.

An open tile has `d=0` and uses the exact neutral log-law

```text
r_h = ln(z_ref/z0m)*ln(z_ref/z0h)/(kappa^2*u_ref)
r_v = ln(z_ref/z0m)*ln(z_ref/z0q)/(kappa^2*u_ref),
```

where `kappa=0.4`, `u_ref>0`, and
`z_ref>max(z0m,z0h,z0q)>0`. No wind floor, stability correction, convective
velocity, or alternate roughness is admitted.

A covered tile owns one zero-storage canopy-air node `(T_c,q_c)`. Every V8
component uses its unchanged component boundary/stomatal conductance to that
shared node. Ground/litter exchange uses ISBA-MEB equations 54--63 specialized
exactly to `psi_H=1` and `f_hv=1`:

```text
Re       = u_l*l_w/nu
c_d      = 1.328*(2/sqrt(Re)) + 0.45*((1-chi_L)/pi)^1.6
d        = 1.1*z_hv*ln[1+(c_d*LAI)^0.25]
u_hv     = u_ref
u_star   = kappa*u_hv/ln[(z_hv-d)/z0v]
K_hv     = kappa*u_star*(z_hv-d)
r_gn     = z_hv/(phi_v*K_hv)
           * {exp[phi_v*(1-z0g/z_hv)]
              - exp[phi_v*(1-(d+z0v)/z_hv)]}
r_g-c    = r_gn/psi_H = r_gn
z_r      = z_ref-d
phi_z    = (z_hv-d)/z_r <= 1.
```

The frozen constants are `phi_v=2`, `z0g=0.007 m`, `chi_L=0.12`,
`u_l=1 m s-1`, `l_w=0.02 m`, and `nu=1.5e-5 m2 s-1`. Configuration supplies
`z_hv,z0v,z_ref,LAI`; it must satisfy `LAI>0`, `z_hv>d+z0v>z0g>0`,
`z_ref-d>=z_hv-d>0`, and every logarithm/exponential/resistance must be finite
and positive. Heat and vapor use this same neutral ground-to-canopy path as
distinct semantic operands. The canopy-to-reference paths use the open neutral
log-law above with configured canopy `d,z0m,z0h,z0q` and
`z_ref>d+max(z0m,z0h,z0q)`.

The exact shared zero-storage residuals are

```text
R_Tc = sum_j H_j + H_s - H_c->atm
R_qc = sum_j v_j + v_s - v_c->atm.
```

Ground terms occur exactly once per tile, not once per occupancy. Reference-air
ground transfer beneath a canopy, a ground term omitted from the canopy-air
node, or a producer-aggregated canopy flux is invalid.

### Surface humidity, surface enthalpy, litter, and soil heat

One surface node is used for both admitted classes. With hydrology-owned
surface water `W>=0`, configured dry areal heat capacity
`C_dry>=0 J m^-2 K^-1`, and `T_ref=273.15 K`, its authoritative enthalpy is

```text
U_s = (C_dry + W*C_w)*(T_s-T_ref).
```

`W` is an immutable hydrology operand during a solve, never an LSE state field.
The configured `finite_capacity` branch requires `C_dry+W*C_w>0`; its accepted
ending enthalpy is the sole physical LSE state. Temperature is derived using
the hydrology candidate's ending `W`; a retained temperature warm start must be
bit-identical to that derived candidate value and cannot be independently
adjusted. The
configured `equilibrium_zero` branch requires `C_dry=0`, `W=0`, and `U_s=0`
exactly and replaces the storage difference by exact zero while retaining
`T_s` as the algebraic surface-energy unknown. No other zero-capacity branch is
admitted.

For bare mineral soil without positive surface water, V1 transcribes CLM5
equations 5.72--5.81. With top-layer hydrology operands
`W_liq,1,W_ice,1,dz_1,theta_sat,psi_sat,B,Phi`:

```text
s_1       = min(1,max(0.01,(W_liq,1/rho_w + W_ice,1/rho_i)
                          /(dz_1*theta_sat)))
theta_1   = W_liq,1/(rho_w*dz_1)
psi_1     = max(-1e8, psi_sat*s_1^(-B))                 [mm]
alpha     = exp(psi_1*g/(1000*R_wv*T_s))
theta_air = Phi*(psi_sat/psi_air)^(1/B)
DSL       = D_max*(theta_init-theta_1)/(theta_init-theta_air)
            when theta_1<theta_init, otherwise 0
Phi_air   = Phi-theta_air
tau_pore  = Phi_air^2*(Phi_air/Phi)^(3/B)
D_v       = 2.12e-5*(T_s/T_ref)^1.75
r_soil    = DSL/(D_v*tau_pore)
q_soil    = alpha*q_sat(T_s,p).
```

The constants are `D_max=0.015 m`, `psi_air=-1e7 mm`,
`R_wv=461.5 J kg^-1 K^-1`,
`rho_w=1000 kg m^-3`, `rho_i=917 kg m^-3`, and `g=9.80665 m s^-2`.
If `q_sat(T_s)>q_recipient>q_soil`, V1 selects the CLM branch
`q_soil=q_recipient`, yielding exact zero vapor flux. Otherwise
`v_s=rho_a*(q_soil-q_recipient)/(r_v+r_soil)`. All denominators and powers
must be defined; this bounded CLM branch is the only admitted humidity
normalization. Positive surface water instead selects
`q_s=q_sat(T_s,p)`, `r_soil=0`, and its request source is that exact store.
Snow-free frozen/thawing states reject before these equations.

Forest litter is the Napoly/ISBA single layer. With hydrology-owned liquid
`W_l`, configured capacity `W_l,max`, thickness `dz_l`, dry density `rho_ld`,
and dry specific heat `c_ld`,

```text
h_ul     = 0.5*(1-cos(pi*W_l/W_l,max))
q_l      = h_ul*q_sat(T_s,p) + (1-h_ul)*q_recipient
v_l      = rho_a*(q_l-q_recipient)/r_l-c
lambda_l = 0.1 + 0.03*W_l/(rho_w*dz_l)
C_dry    = dz_l*rho_ld*c_ld.
```

`W_l,max>0` and `0<=W_l<=W_l,max`. Litter blocks direct mineral-soil
evaporation and upward capillary supply during this interval. `r_l-c` is the
covered neutral `r_g-c` or open neutral `r_v` as topology requires. Overflow
remains hydrology-owned ingress.

The soil-thermal owner has exact ordered nodes `k=1..N`, each with
`dz_k>0`, `lambda_k>0`, and areal heat capacity `C_k>0`. The surface has
`dz_s>0,lambda_s>0`; for litter these are `dz_l,lambda_l`, and bare soil has
its own configured mineral skin. Define

```text
g_s1     = 2/(dz_s/lambda_s + dz_1/lambda_1)
g_k,k+1  = 2/(dz_k/lambda_k + dz_(k+1)/lambda_(k+1))
G_s1     = g_s1*(T_s-T_1)
G_k,k+1  = g_k,k+1*(T_k-T_(k+1)).
```

All `G` here are positive downward. The surface residual contains `-G_s1`;
the soil-thermal candidate contains `+G_s1` exactly once. For Crank--Nicolson,
`bar(G)=0.5*(G_begin+G_end)` and

```text
C_1*(T_1,1-T_1,0)/dt = bar(G_s1)-bar(G_1,2)
C_k*(T_k,1-T_k,0)/dt = bar(G_k-1,k)-bar(G_k,k+1)
C_N*(T_N,1-T_N,0)/dt = bar(G_N-1,N),
```

with exact zero lower flux. `N>=1`; for `N=1` this reduces to
`C_1*(T_1,1-T_1,0)/dt=bar(G_s1)`. Phase change, frozen or thawing soil, an
LSE-owned duplicate soil temperature, and a second independently calculated
surface/soil flux are unsupported.

For `finite_capacity`, `G_s1,begin` uses the beginning surface temperature
derived from authoritative beginning `U_s,W,C_dry`, and `G_s1,end` uses the
trial ending surface temperature. For `equilibrium_zero`, no physical
beginning surface temperature exists: the current algebraic trial `T_s` is the
surface-side operand at both Crank--Nicolson endpoints,
`G_s1,begin=g_s1*(T_s-T_1,0)` and
`G_s1,end=g_s1*(T_s-T_1,1)`. The caller temperature warm start is numerical
only and must never enter `G_s1,begin` as a physical state operand.

### Signed vapor and liquid enthalpy

The liquid reference state is `T_ref=273.15 K`; `C_w=4218 J kg^-1 K^-1`.
Each parcel carries `h_l(T)=C_w*(T-T_ref)` and `Q=m*h_l(T)`. Positive-mass
mixing conserves enthalpy exactly:
`T_mix=T_ref+sum(m_i*h_i)/(C_w*sum(m_i))`. A zero-mass crossing has neither
temperature nor energy. Rain temperature is the exact retained output
`hydrometeor_temperature_c+273.15` from
`openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity`
on the active `harder_pomeroy_hourly` provider required by
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-075`; LSE neither recomputes nor partially
transcribes that solver. Runon must carry the typed temperature and enthalpy of
the accepted upstream outlet parcel. Missing runon temperature rejects; air,
soil, freezing, or downstream surface temperature is never substituted.
Infiltration and runoff carry the exact accepted source-parcel or conservative
mixture temperature selected by hydrology.

The selected liquid vaporization enthalpy is
`L_v(T)=2.501e6-2369*(T-T_ref) J kg^-1`. The signed vapor-energy flux leaving
the surface is

```text
Q_v = v_s*[h_l(T_s)+L_v(T_s)]                 [W m^-2].
```

Thus evaporation (`v_s>0`) removes both liquid sensible enthalpy and latent
energy, while condensation (`v_s<0`) credits both. Evaporation produces a
positive water request `D=max(v_s,0)*dt`; only finalized positive use is
debited. Condensation produces no withdrawal request and hydrology credits the
exact amount `-v_s*dt` to the typed surface store. Any clipping, absolute
value, latent-only record, authorization-as-use, or missing condensation mass
credit fails.

The interval surface equation before current-ingress advection is

```text
(U_pre-U_0)/dt = R_sw + R_lw - H_s - Q_v - G_s1.
```

`U_pre` uses the hydrology-owned pre-ingress mass after finalized evaporation
debit or condensation credit. Every term is evaluated at the accepted trial
surface state and occurs exactly once.

### Immutable-beginning water transaction and current ingress

The water snapshot precedes all current-interval rain, runon, and canopy liquid
release. Only immutable beginning surface/litter and soil-layer liquid stores
are available to root uptake and ground evaporation. Current precipitation,
runon, throughfall, both canopy drainage terms, stemflow, and litter overflow
cannot satisfy or reduce a same-interval withdrawal request.

From immutable beginning vegetation/LSE/soil-thermal state and that immutable
hydrology snapshot, solve the complete current-temperature canopy--ground
system without owner caps. Publish root, surface/litter, and soil-layer
requests with transaction/OFE/tile/occupancy/surface/source/layer/basis
identity. Hydrology authorizes all same-snapshot requests exactly once, before
current ingress. Therefore a changed final canopy release cannot shrink or
enlarge authorized supply.

Rebuild the complete system from the original beginning state with fixed
source-specific caps. The cap is active iff `cap_rate<=q_law`; equality selects
the cap branch and zero generalized derivative. Gas, canopy energy, surface
energy, shared air, hydraulics and soil thermal state re-solve together. Final
use is `F=f_tile*q*dt`, independently checked as `0<=F<=A<=D`. No second
authorization, donation, scalar stress or potential-state continuation exists.

After the capped solve accepts, hydrology applies finalized beginning-store
debits and the explicit condensation credit, then accepts final current
precipitation, runon, and canopy release. Hydrology partitions that ingress
exactly once into retained surface/litter store, infiltration, routed runoff,
and outlet runoff. This is owner candidate construction, not a second
authorization. Ingress parcels retain their individual enthalpies. Retained
ingress updates surface `U`; infiltration energy credits soil thermal node 1;
routed runoff transfers the same accepted parcel mass and enthalpy to the
downstream OFE; outlet runoff transfers them out. The ending surface
temperature is obtained only from the authoritative ending `U_s,W,C_dry`
identity. Current ingress does not feed the already accepted same-interval
H/LE/G flux evaluation.

```text
U_s,1       = U_pre + sum(Q_retained_ingress)
E_soil,1,1  = E_soil,1,pre + sum(Q_infiltration)
Q_runon,dst = Q_routed_runoff,src
Q_outlet    = sum(Q_outlet_runoff).
```

Each equality uses the identical accepted mass/enthalpy parcel; zero mass has
zero energy and no temperature. The soil owner reconstructs its first-layer
temperature from the credited enthalpy. Hydrology may combine parcels only by
the conservative mixing equation above.

Potential and final passes use identical operator ordering. Only the final
pass constructs owner candidates and applies the once-only ingress partition.

### Ordered numerical algorithm, active branches, and error precedence

For each OFE, tiles sort by typed tile ID. Within each tile, canopy occupancies
sort top-to-bottom by rank and then typed occupancy ID. The exact unknown order
is: every V8 occupancy unknown block in that order (with the V7 within-block
order), the shared tile `(T_c,q_c)` when covered, `T_s`, and soil thermal
`T_1..T_N`. The exact residual order is: matching V8 gas/component-energy/
hydraulic blocks, shared canopy-air heat, shared canopy-air vapor, surface
energy, then soil thermal layers `1..N`. Open tiles omit only the two shared-air
unknowns/residuals and use reference-air fluxes.

The nonlinear unknown bounds are `200<=T<=350 K`,
`0<=q_c<=0.1 kg kg^-1`, and the unchanged V8 bounds for `ci`, hydraulic
potentials, and `beta`. Configuration/domain validation precedes numerical
bounds; an invalid physical input is not reported as nonconvergence.

The joint system may be monolithic or a mathematically equivalent nested solve
only when every inner solve converges at every outer residual evaluation and
the complete ordered residual vector passes. Residuals are normalized by the
dimensional thresholds below before the infinity norm is formed. Starting from
the complete caller warm start, each Newton iteration uses centered finite
differences `delta_i=sqrt(epsilon)*max(abs(x_i),unit_scale_i)`, evaluating the
minus point and then the plus point, solves with deterministic partial-pivot
LU, and accepts the first factor `2^-b`, `b=0..20`, producing a strict decrease
in normalized infinity norm. Equal pivot magnitudes choose the lowest row.
Failure to decrease through `b=20` is backtracking limit. A pivot below
`64*epsilon*matrix_inf_norm` is singular. The iteration limit is 50 completed
Newton updates. Unit scales are `1 K`, `0.001 kg kg^-1`, `1 Pa`, `1000 mm`,
and `1` for beta.

Each energy residual threshold is
`1e-6 W m^-2+1e-10*max(1,sum(abs(component operands)))`. Each water/vapor
residual threshold is `1e-12 kg m^-2 s^-1+1e-9*scale`. Accepted temperature
step is at most `1e-8 K`; humidity step at most `1e-12 kg kg^-1`; accepted
hydraulic step is at most `1e-7 mm`; and beta step at most `1e-10`.
Convergence requires all residual and step criteria on the same accepted
iterate. Active branches are evaluated in this order: typed domain; surface
class; covered/open turbulence; positive-surface-water versus dry-soil vapor;
vapor sign; water cap (`cap<=law`, equality cap-active); then numerical solve.
Identity, unit, basis, owner, layer, band, direction and D/A/F inequalities are
exact and cannot be tolerance-repaired.

Error precedence is: malformed serialization; model/configuration/state/
transaction identity; missing/duplicate topology or owner; nonfinite operand;
unsupported snow/frozen/thawing/calm/nonneutral/domain branch; constitutive
domain; request/authorization identity or bound; singular pivot;
backtracking limit; iteration limit; accepted-step/residual failure;
component closure; control-volume closure; cross-owner join. Only the first
error is returned, with diagnostics accumulated up to that point.

Every failure includes typed model/configuration/state/transaction/OFE/tile/
occupancy/pass/solve identity, ordered residuals, iteration and backtracking
counts, step, bounds/caps, bracket/pivot/matrix evidence and rollback hashes.
No failed iterate or partial owner candidate is usable.

### Independent closure and errors

Independent validators reconstruct local then weighted OFE-ground shortwave,
longwave, sensible, signed vapor enthalpy, surface storage, soil storage,
ground and advected energy; hydrology mass;
latent mass/energy; and equal/opposite ground heat. They consume primitive
operands, never producer residuals. Required new error families are:

- `LSEB-E-030` unsupported domain;
- `LSEB-E-031` strict configuration/state/identity;
- `LSEB-E-032` radiation or turbulent ownership;
- `LSEB-E-033` source-water D/A/F;
- `LSEB-E-034` numerical convergence;
- `LSEB-E-035` component/control-volume closure;
- `LSEB-E-036` liquid enthalpy/latent join; and
- `LSEB-E-037` ground-heat or atomic-owner join.

Additional typed failures are `LSEB-E-038` current-ingress ordering or
same-interval availability, `LSEB-E-039` condensation mass/energy credit, and
`LSEB-E-040` soil-thermal owner/state/enthalpy mismatch.

Any error preserves vegetation, hydrology, LSE, BGC, soil-thermal and envelope
bytes. `GAP-LANDSURFACEENERGY-001..003` and the authority portion of 006 are
`AUTHORITY_ADMITTED`; gap 004 remains `IMPLEMENTATION_MISSING`; gap 005 remains
outside the snow-free model.

### V1 invariants and independent fixtures

| ID | Binding V1 rule |
|---|---|
| `INV-LANDSURFACEENERGY-100` | Hydrology owns every water mass; LSE owns one surface thermal node per tile; soil thermal owns all `N` soil temperatures and enthalpies. |
| `INV-LANDSURFACEENERGY-101` | Component longwave uses current component temperature and `tau=exp[-0.8*Omega*(LAI+SAI)]`; no bulk-canopy or stale-ground operand is accepted. |
| `INV-LANDSURFACEENERGY-102` | Neutral open and covered turbulent paths use only the exact equations/constants/domains above; no wind floor or stability substitute exists. |
| `INV-LANDSURFACEENERGY-103` | Surface enthalpy is `(C_dry+W*C_w)*(T_s-T_ref)` using hydrology's exact mass, while every soil enthalpy remains independently owned. |
| `INV-LANDSURFACEENERGY-104` | Signed vapor mass and `v*[h_l+L_v]` energy preserve sign; condensation has one explicit hydrology credit. |
| `INV-LANDSURFACEENERGY-105` | Root and ground requests share beginning stores before ingress; accepted current ingress is partitioned exactly once after the capped solve. |
| `INV-LANDSURFACEENERGY-106` | Surface `-G_s1` and soil `+G_s1`, infiltration energy, and routed runoff enthalpy join once by exact OFE/tile/interval identity. |
| `INV-LANDSURFACEENERGY-107` | Local tile closure precedes one `f_t` weighting to OFE ground; no cross-OFE aggregate is called stand ground. |
| `INV-LANDSURFACEENERGY-108` | Ordered unknowns, residuals, branches, finite differences, pivots, backtracking, tolerances, and error precedence are deterministic. |

The digest-bound independent fixture family must include open bare-soil day and
night, dry and wet litter, covered and open tiles, two heterogeneous columns,
zero shortwave, longwave and ground-heat sign reversal, ground feedback to
canopy air, evaporation and condensation, full and partial surface/top-layer
authorization, concurrent root/ground scarcity, dry source, rain/runon/
infiltration/runoff advection, equilibrium-zero and finite heat storage,
alternate warm starts, singular/backtracking/iteration failures, and exact
rollback. Frozen, snow, terminal-snow, calm, and nonneutral inputs are typed
rejection vectors.

Required poisons independently distinguish bulk/repartitioned longwave,
prescribed upward ground longwave, reference-air ground exchange beneath
canopy, omitted ground canopy-air H/v, agricultural PMET donation, current
ingress counted as available water, final canopy release shrinking
authorization, missing/doubled `f_t`, authorization as finalized use, vapor
zero-clipping, condensation sign reversal or missing mass credit, latent-only
vapor energy, omitted/swapped liquid advection, duplicated `G`, an LSE-owned
soil temperature, hidden wind floor, and producer-supplied residual.

### Scope

This version authorizes contract-derived fixtures and later default-off shadow
implementation. It authorizes no production selector/default/output, snow
handoff, calibration, empirical validation or transferability claim, or
cutover.

## Terminal Receiver Remaining-Support Amendment

`INV-LANDSURFACEENERGY-114` admits LSE-V2 as the energy owner inside only the
default-off `terminal_receiver_v1` transaction. It receives exact
`dt_remaining`, actual vegetation/litter/mineral/frost/water owner state, and a
single 0 C terminal-liquid parcel. It must select the actual surface and
rebuild shortwave/albedo, longwave, aerodynamic geometry/roughness,
turbulence, evaporation/condensation, precipitation heat, soil heat, and
storage terms over `[wall_t*,wall_end)`. No snow temperature, albedo, roughness,
flux, residual, or terminal unallocated energy may enter any LSE operand.

`INV-LANDSURFACEENERGY-115` binds one candidate-only continuation: liquid
ingress is passed to hydrology exactly once; LSE neither repartitions it nor
relabels latent fusion as soil heat. `dt_remaining=0` suppresses the LSE solve.
Any invalid surface selection, support, receipt, owner join, or LSE solve
returns its typed error without mutation; the encompassing transaction restores
all beginning owners. CoE/default behavior and every turbulent-carrier,
efficacy, qualification, production, and cutover hold remain unchanged.

Vectors must distinguish litter/mineral/ponded and frozen/thawing receivers,
make snow and receiver fluxes unequal, cover rain and cross-midnight support,
and poison stale snow operands, full-interval LSE execution, liquid duplication,
and partial commit.

| Canonical surface | INV-114/115 binding |
|---|---|
| Algorithm | select actual receiver from beginning owners; rebuild every forcing/flux on half-open receiver support; solve LSE-V2; join hydrology/soil receipts |
| Branch/guard | zero duration skips solve; invalid/ambiguous receiver or any snow operand rejects before mutation |
| Alias/unit | absolute wall support is distinct from transaction order; 0 C parcel sensible enthalpy is zero and fusion energy is not `G` |
| Tolerance | existing LSE closure tolerances apply after exact support/identity validation; none repairs stale snow operands |
| Tests/gap | unequal snow/receiver fluxes, endpoint rain, actual surface matrix, zero remainder, rollback; carrier/efficacy/production remain held |

## Child 2C shared-carrier and successor-support amendment

The covered-forest snow-free receiver remains a default-off physical adopter.
For any positive successor segment it consumes the coupled-time
`LseSupportAdmissibilityReceiptV1` and admits only the declared physical
support domain `dt >= 600000000 ns`. A structural one-nanosecond clock interval
is not a valid constitutive LSE solve. The guard runs before Newton and leaves
the LSE, snow, surface-liquid, hydrology, soil-thermal, BGC, V11, and coupled
time owners unchanged on rejection.

### Snow-free successor chronology

The snow-free receiver may be called only after a consumed
`EventBoundaryCoalescingReceiptV1` proves that the accepted event tick is a
valid successor boundary. It reconstructs all fluxes on
`[accepted_event_tick,parent_end_tick)` using the successor forcing and owner
set. It must not consume snow albedo, snow temperature, snow roughness, snow
vapor, snow sensible heat, snow longwave, or terminal snow liquid after the
accepted tick. If the remaining support is zero, the receiver records a
zero-duration custody transition and does not execute Newton. If it is
positive but below the support receipt domain, it returns `LSEB-E-041` before
Newton; it never scales a longer result or freezes a state.

### Child 2C guards

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-121` | A positive snow-free successor consumes the accepted event receipt and a support-admission receipt whose active adopter is the covered-forest LSE policy. | typed receipt join / `LSEB-E-042` |
| `INV-LANDSURFACEENERGY-122` | The successor uses only post-event snow-free operands and its exact accepted support. | chronology/operand lineage / `LSEB-E-043` |
| `INV-LANDSURFACEENERGY-123` | Below-domain positive support rejects before Newton with exact owner rollback; zero support performs no physical solve. | preflight / `LSEB-E-041` |

`LSEB-E-043` is the wrong-regime operand failure. The existing exact energy,
liquid, and equal/opposite `G` ledgers remain binding; Child 2C adds no new
storage arithmetic and does not admit compensated or sub-ULP increments.

## Version 8 persistent snow--soil boundary amendment

This amendment admits one lower boundary for a persistent Stage 3 column. The
boundary is OFE/lane-level because the snow owner and the existing
soil-thermal snapshot are each OFE-ground owners. It couples the bottom
represented Stage 3 thermal volume directly to ordered soil node `k=1`. Tile
surface temperatures and tile LSE thermal nodes do not participate. Selecting
one tile, averaging any tile subset, weighting the OFE soil node by tile
fractions, or applying one lane flux separately to every tile is invalid.

Pinned `frostn.for` lines 476--607 and `tmpadj.for` lines 266--353 establish
additive layer resistance and harmonic snow/soil conduction. They do not
define the current node-owner interface and their silent zero-flux fallbacks,
calibrated conductivity factors, frost-front paths, and prescribed surface
temperature are not imported. The current LSE node-centered authority supplies
the exact interface specialization. Let the bottom snow volume and top OFE
soil node have positive finite `(dz_sb,lambda_sb)` and `(dz_1,lambda_1)`:

```text
R_ss       = dz_sb/(2*lambda_sb) + dz_1/(2*lambda_1)
g_ss       = 1/R_ss = 2/(dz_sb/lambda_sb + dz_1/lambda_1)
G_ss,0     = g_ss*(T_sb,0-T_1,0)
G_ss,1     = g_ss*(T_sb,1-T_1,1)
bar(G_ss)  = 0.5*(G_ss,0+G_ss,1).
```

`G_ss` is positive downward from snow to soil and has units `W m^-2
OFE-ground`. The beginning endpoint uses only the sealed beginning Stage 3 and
soil-thermal owners. The ending endpoint uses the current candidate bottom
snow and first-soil-node temperatures. Consequently `bar(G_ss)` participates
inside the existing covered fixed point and cannot be calculated after it or
held fixed from a stale trial. The Stage 3 energy candidate contains
`-bar(G_ss)` exactly once; the first soil-node Crank--Nicolson equation and
candidate enthalpy contain `+bar(G_ss)` exactly once. Deeper node conduction
and the zero lower boundary retain the existing LSE equations unchanged.

One sealed `SnowSoilHeatReceiptV1` binds schema/model identity, exact half-open
support and duration, lane/OFE and OFE-ground basis, ordered topology and
configuration digests, beginning Stage 3 and soil-thermal owner IDs/digests,
bottom-snow and first-soil layer IDs, all four positive resistance operands,
both endpoint temperature pairs, both endpoint fluxes, accepted
`bar(G_ss)`, and both candidate-ending owner IDs/digests. Its digest uses the
repository canonical framed encoding and is reconstructed from those semantic
fields; receipt-hash order is not operand order.

Independent validation recomputes the resistance, endpoint fluxes,
Crank--Nicolson flux, Stage 3 debit, first-soil-node credit/storage equation,
receipt digest, and beginning/candidate owner joins from primitive operands.
Producer residuals are diagnostic only. Any missing, duplicate, stale,
nonfinite, nonpositive-resistance, wrong-node, wrong-sign, wrong-basis,
substituted-receipt, convergence, reconstruction, or later transaction failure
returns `LSEB-E-044` / `SNOWENERGY-E-SOIL-HEAT-001` and leaves Stage 3,
soil-thermal, transaction, and receipt owners byte-identical.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-124` | The persistent snow boundary joins the bottom Stage 3 volume to first ordered OFE soil node only; no tile selection, aggregation, weighting, or duplication is allowed. | topology/node/area guard / `LSEB-E-044` |
| `INV-LANDSURFACEENERGY-125` | Half-snow plus half-soil series resistance and beginning/ending Crank--Nicolson evaluation produce one positive-downward `bar(G_ss)` inside the covered fixed point. | operand/endpoint/convergence guard / `LSEB-E-044` |
| `INV-LANDSURFACEENERGY-126` | Exact `-bar(G_ss)` snow custody and `+bar(G_ss)` first-soil-node custody share one reconstructable receipt and commit or roll back together. | independent receipt/owner transaction guard / `LSEB-E-044` |

## Version 9 Terminal Snow--Soil Endpoint Amendment

Terminal adaptive/root evaluation carries the first-soil-node candidate inside
`SC-SNOWENERGY-001#CoveredTerminalJointTrialStateV1`. Accepted second halves
begin from the first half's ending joint state; they never reuse the immutable
accepted beginning soil node. Full/coarse, rejected and discarded root
alternatives do not advance that joint state. The accepted endpoint installs
only the ending candidate proven by the accepted fine chain. This is the
trial-state realization of `INV-LANDSURFACEENERGY-127..129`; stale-soil reuse
fails `LSEB-E-043` and all accepted owners remain unchanged on failure.

Version 8 remains the persistent resolved-to-resolved receipt. A support whose
accepted Stage-3 endpoint is dormant uses the separate
`TerminalSnowSoilHeatReceiptV1`; it cannot be validated as a persistent receipt
and has no post-event snow temperature or bottom snow volume.

The terminal receipt integrates only over the evaluated snow support through
the accepted event. The terminal enthalpy solver's transition callback is a
joint snow/first-soil-node transition. For a trial beginning at state `x_k`
over exact duration `h`, the covered provider computes the current-trial
snow--soil flux from the real trial-start bottom snow volume and soil node,
advances the soil node over the same trial, and returns `q_ss,k` in `J m^-2`
positive snow-to-soil. One-full and two-half trials each begin from the same
immutable `x_k`; the accepted step is the two sequential half transitions, so
only `q_ss,k,1+q_ss,k,2` enters accepted custody. The full trial and every
rejected retry are evidence only. Root localization replays from the immutable
pretrial state at each bisection duration; only the transition chain ending at
the accepted upper root endpoint contributes. Thus
`Q_ss=sum(q_ss,k,accepted)` in chronological order, snow energy includes
`-Q_ss`, and first-soil-node storage includes `+Q_ss` using the identical bits.

The callback evaluates flux only from a positive-ice trial-start state and
never requests an ending dormant snow temperature. The accepted upper root may
be dormant because `q_ss,k` is already the integrated output of the exact
first-order trial map authorized by
`SC-SNOWENERGY-001#INV-SNOWENERGY-034/051`; no limiting temperature or
post-event node is constructed. Each accepted trial binds its start/end joint
state, absolute support, carrier receipt and `q_ss,k`. The terminal receipt
binds the ordered accepted trial receipts, event tick, evaluated support,
exact `Q_ss`, ending soil owner, dormant snow endpoint, event-result digest,
and complete beginning/ending owner digests. `[INFERENCE][Static]` from the
released terminal first-order/step-doubling algorithm, released version-8
interface physics, and equal/opposite energy conservation.

No `273.15 K` fallback, melting-point substitute, or other fabricated dormant
snow temperature is admitted by this receipt.

When the event is exactly at the current cursor, evaluated snow support is
zero, `Q_ss=0`, no `TerminalSnowSoilHeatReceiptV1` is emitted, and the terminal
physical ledger records the explicit zero-support disposition. Any positive
support terminating lane requires exactly one terminal receipt.

Required-lane and installed-owner validation classify lanes consistently:
resolved-to-resolved lanes require the version-8 persistent receipt;
resolved-to-dormant and terminal-domain-to-dormant lanes require the version-9
terminal receipt when evaluated support is positive, while a sealed cursor-root
event requires no snow--soil receipt and validates `Q_ss=0`. Installed terminal validation reconstructs the event-integral
and owner joins without calling a resolved-state bottom-volume projector on the
dormant endpoint. Missing, duplicate, cross-class, post-event, fabricated-node,
or unequal snow/soil custody fails atomically.

Canonical `INV-LANDSURFACEENERGY-127..129` rows are integrated with the Child
2C invariant IDs, and `OBL-LANDSURFACEENERGY-P-020/C-020` are integrated in
the producer/consumer obligations section. This algorithm detail is not a
second invariant or obligation surface.

Required vectors cover terminal-domain and resolved-crossing beginnings,
nonzero soil heat, endpoint/support and event-result poisons, persistent versus
terminal receipt substitution, no fabricated-temperature reachability,
dormant installed-owner validation, and rollback before owner publication.

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-24 | 9 | Codex | Added a terminal-specific snow--soil receipt integrated only through the accepted event, with limiting pre-event snow-state custody, exact soil credit, dormant endpoint validation, and no fabricated terminal snow temperature. |
| 2026-08-24 | 8 | Codex | Admitted the persistent Stage 3 OFE/lane snow--soil interface as a specialization of the existing node-centered Crank--Nicolson soil authority: bottom snow volume to first OFE soil node, half-layer series resistance, positive-downward exact snow debit/soil credit, sealed receipt, independent reconstruction, and atomic rollback; no tile averaging or duplicated flux. |
| 2026-08-19 | 5 | Codex | Admitted default-off terminal remaining-support LSE-V2 authority (`INV-LANDSURFACEENERGY-114/115`) with actual receiver selection, complete flux rebuild, no post-event snow operands, and atomic rollback. |
| 2026-08-18 | 4 | Codex | Admitted `OPENWEPP_SNOW_FREE_LSE_V2`, binding V10 exact-zero-PAR physiology and deterministic FullSupply iteration-zero final reevaluation; V1 remains immutable. |
| 2026-08-14 | 3 | Codex | Admitted `OPENWEPP_SNOW_FREE_LSE_V1`: explicit bare-soil and forest-litter thermal state, reciprocal V8 canopy-ground radiation/turbulence, hydrology-owned water mass, liquid enthalpy, coupled potential/final solve, strict numerics and independent closure; no runtime or cutover. |
| 2026-08-08 | 2 | Codex | VEGETATION-BOUNDARY-AUTHORITY amendment: separated canopy/ground/litter/snow/soil radiation lineage and bound actual transpiration to one independently reconstructed latent-energy debit without admitting constitutive physics. |
| 2026-08-08 | 1 | Codex | Initial control-volume, custody, conservation, ownership, guard, test-vector, and non-promotable authority-gap contract. |

## `OPENWEPP_SNOW_FREE_LSE_V2` V10 Coupling Amendment

V2 imports every V1 control volume, tolerance, owner, rollback rule, and
positive-PAR accepted result. It requires the V10 vegetation identity and
admits its exact-zero-PAR and respiration-dominated positive-low-light
branches. It does not recompute, clamp, or relabel V10 gas states.

When every positive final water authorization is identity- and amount-equal to
its potential request with `FullSupply`, and every canonical zero request
retains its exact identity and zero amount, V2 uses the accepted potential
coordinates as the deterministic fixed-final initial iterate. It rebuilds the
complete fixed-final evaluation from immutable beginning owners and exact
per-resource caps. No potential flux, candidate state, branch, receipt, or
diagnostic is copied.

If that initial evaluation satisfies every residual tolerance, active-branch
inequality, domain and bound, `F<=A<=D`, identity, and owner check, V2 accepts
at iteration zero with exact-zero step norms and zero backtracking without
constructing a Jacobian. Every actual solver step retains V1 strict-decrease
and convergence rules. Copying the potential candidate without complete final
reevaluation is forbidden. A residual outside tolerance or any branch,
identity, owner, or amount mismatch cannot use this acceptance path.

Nonpositive-assimilation partial positive root authorization is typed unsupported in V2.

## Version 6 prospective positive-support admission amendment

`OPENWEPP_SNOW_FREE_LSE_V1` retains nanosecond chronology at the coupled-time
wire boundary, but its physical nonlinear solver has a deterministic declared
positive-support domain. The released domain policy is
`OPENWEPP_SNOW_FREE_LSE_V1_SUPPORT_POLICY_V1` with
`minimum_support_ns = 600000000` (0.6 seconds). This is a conservative,
model-specific numerical-domain boundary selected from the frozen V11 actual
covered-forest stack sweep and representation analysis; open-mineral, litter,
and other surface profiles remain prospective/non-admitted until separately
profiled. It is not
a hidden duration floor, a changed V10 cadence, or a promise that every larger
support is globally convergent under arbitrary forcing.

Every physical V11 LSE invocation must first admit a sealed
`LseSupportAdmissibilityReceiptV1` binding parent transaction, segment, slab,
absolute half-open support, requested support ticks, exact binary64 duration
bits, model/configuration identity, beginning LSE/soil state identity,
tolerance-policy identity, numerical-policy identity, the exact minimum, and
its digest. Requested support below the declared minimum rejects
before Newton/nonlinear execution with `LSEB-E-041`; the caller receives no
candidate and all owner state remains byte-identical. A support exactly at the
minimum is an ordinary positive-support solve and must pass the declared
fixture; one nanosecond below it is a typed domain rejection. Coupled-time
identity, ordering, event receipts, and restart chronology remain valid at one
nanosecond; they do not force the physical LSE solver to execute below its
admitted domain.

The admission receipt does not canonicalize, round, or silently replace the
requested duration. A support which is rejected is not retried at the minimum.
Terminal events may coalesce an otherwise below-domain LSE segment only at an
admitted event boundary and only when exact event-time, mass, and energy error
bounds in the event contract are satisfied; no flux from the wrong surface
regime may be applied and no time gap may be created. This amendment does not
change the frozen V10 configuration, V10 behavior, selectors, defaults,
coupled-time V2 bytes, or DirectV10 restart V1 bytes.

The independent representation analysis records adjacent binary64 temperature
spacing, finite-capacity energy quanta, configured absolute/relative energy
tolerances, and the necessary storage-resolution support for every declared
finite-capacity V11 actual-stack profile. The observed
transition is solver-domain behavior beyond that representational floor; the
declared 0.6-second policy is therefore fail-closed and deterministic across
the admitted V11 actual-stack fixture domain. A future smaller or additional
surface/state-qualified domain
requires a new contract cycle; it may not be inferred from a successful
individual run.

The receipt digest preimage is the compact canonical JSON object with the
`receipt_sha256` field replaced by the empty string, prefixed by the exact
domain tag `OPENWEPP_LSE_SUPPORT_ADMISSION_V1\\0`. Parent, segment, slab, and
owner chronology use the 64-lowercase-hex coupled-time identity projection;
slab ordinal and tick bounds use canonical decimal strings with no leading
zero. The baseline receipt and 12 independent identity/digest/rollback cases
are frozen in the package artifact set.

| New guard | Required result |
|---|---|
| `INV-LANDSURFACEENERGY-116` | Coupled-time nanosecond identity is independent of the LSE physical support domain. |
| `INV-LANDSURFACEENERGY-117` | Every physical solve carries one sealed support-admission receipt with exact identity and policy joins. |
| `INV-LANDSURFACEENERGY-118` | Below-minimum support rejects before Newton and leaves all staged/committed owners unchanged. |
| `INV-LANDSURFACEENERGY-119` | Support exactly at the declared minimum uses the unchanged constitutive equations and existing tolerances. |
| `INV-LANDSURFACEENERGY-120` | No hidden floor, longer-step scaling, tolerance relaxation, frozen state, or V10 mutation is admitted. |

| Error | Meaning |
|---|---|
| `LSEB-E-041` | Requested positive support is below `600000000` ns for the declared LSE policy; typed pre-Newton rejection. |
| `LSEB-E-042` | Support receipt parent/segment/slab, absolute support, duration bits, configuration/state, tolerance, or numerical-policy binding is invalid. |

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-20 | 6 | Codex | Prospective deterministic positive-support admission; nanosecond chronology remains coupled-time-valid, below-domain support rejects before Newton, and event-boundary coalescing is deferred to the reviewed snow/event contract. |
| 2026-08-20 | 7 | Codex | Bound the covered-forest snow-free successor to the accepted event receipt, post-event-only operands, and pre-Newton support admission without scaling, freezing, or sub-ULP storage treatment. |
It does not invoke hydraulic attenuation, conductance or vulnerability floors,
plant capacitance, or authorization donation.

For the V10 nonpositive-assimilation potential solve only, a coordinate at an admitted
closed bound uses the inward one-sided binary64 finite difference when the
opposite perturbation is outside the existing trial domain. Interior
coordinates retain the V1 centered difference. Both perturbations outside the
domain reject. This bound-aware derivative rule is prospective V2 numerics and
does not alter any V1/V8/V9 execution.

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

When the uncapped V10 nonpositive-assimilation potential evaluation selects the
canopy-liquid store-cap branch and the preliminary store rate is no larger
than the canonical water residual tolerance, the wet-surface temperature is a
numerically inactive coordinate: the wet-energy residual is already below its
admitted closure scale and cannot determine that temperature. V2 replaces
that one row with the existing inactive-component anchor
`T_wet - T_canopy = 0`. Liquid mass, enthalpy, longwave area, and every owner
ledger remain evaluated normally; no liquid amount is clamped or discarded.
The predicate additionally requires the unanchored physical wet-energy
residual already satisfy its canonical energy tolerance. It is unavailable to
V1, positive-assimilation V10, condensation, or a constitutive-law wet flux.

V1-to-V2 migration validates complete V1 and V10 owner identities, copies all
LSE scientific values bit-identically, and changes only the LSE identity and
transitively derived receipts. V1 remains immutable and is not a V2 alias.

| ID | Binding rule |
|---|---|
| `INV-LANDSURFACEENERGY-109` | LSE-V2 imports exact LSE-V1 physics and accepts only the V10 vegetation owner at the coupled boundary. |
| `INV-LANDSURFACEENERGY-110` | Exact FullSupply finalization seeds only coordinates, then reevaluates the complete fixed-final system from immutable beginning owners; a passing initial evaluation accepts at iteration zero without a Jacobian. |
| `INV-LANDSURFACEENERGY-111` | V1-to-V2 migration copies every LSE scientific value bit-identically and derives only V2 identity receipts; partial nonpositive-assimilation root supply is unsupported. |
| `INV-LANDSURFACEENERGY-112` | Only the uncapped active V10 nonpositive-assimilation potential solve uses the declared diagonal coordinate scaling for Jacobian pivot classification and dimensionless Newton solution; physical residuals and steps are unchanged. |
| `INV-LANDSURFACEENERGY-113` | A store-cap-active V10 nonpositive-assimilation wet coordinate below the canonical water-rate tolerance uses the existing canopy-temperature inactive anchor only when its physical energy residual already passes, without changing liquid or energy ledgers. |
| `LSE-E-109` | V8/V9 vegetation identity, mixed V1/V2 receipts, or any owner alias rejects before V2 physics. |
| `LSE-E-110` | Missing, duplicated, mutated, or locally recomputed V10 nighttime gas state rejects the coupled owner envelope. |
| `LSE-E-111` | Partial or value-mutating V1-to-V2 migration rejects without a V2 state. |

This amendment remains default-off. It authorizes no production selector,
default/output change, cutover, snow handoff, deployment, calibration, or
empirical claim.
