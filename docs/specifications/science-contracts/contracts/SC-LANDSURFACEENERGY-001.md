---
contract_id: SC-LANDSURFACEENERGY-001
title: Land-Surface Energy-Balance Process Contract
status: approved
maturity: active
owner: openWEPP maintainers + land-surface-energy/hydrology reviewer
contract_version: 31
producer_scope:
  - Future snow-free land-surface energy control-volume evaluator
  - Future post-snow receiving-surface evaluator after an atomic handoff cutover
  - Persistent Stage 3 snow--soil lower-boundary evaluator
consumer_scope:
  - Future soil-heat/frost boundary, evaporation, infiltration/runoff, and surface-water ledgers
evidence_level: static+independent_oracle+contract_vectors
last_reviewed: 2026-09-04
supersedes: []
superseded_by: []
---

# SC-LANDSURFACEENERGY-001 Land-Surface Energy-Balance Process Contract

Status: `approved`
Maturity: `active`
Evidence mode: `static + independent oracle`

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
| `REF-LANDSURFACEENERGY-011` | IEEE 754 binary64 round-to-nearest, ties-to-even; exact integer arithmetic over finite-binary64 dyadics; `SC-SURFACELIQUID-001#INV-SURFACELIQUID-022` | Receiver-owned exact soil-layer enthalpy total, one correctly rounded high term, canonical signed-dyadic carry, and exact accepted-credit custody. | `[DIRECT][Static] + [INFERENCE][Static]` |
| `REF-LANDSURFACEENERGY-012` | IEEE 754 binary64 round-to-nearest, ties-to-even; exact integer arithmetic over finite-binary64 dyadics; physical conservation of accepted surface-energy operands; `SC-SURFACELIQUID-001#INV-SURFACELIQUID-023` | LSE-owned exact per-tile surface enthalpy, immutable high-term mirrors, exact retained-ingress credit custody, and successor restart/rollback. | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `H_hi,k` | `J m^-2 OFE-ground` | finite binary64 high term of soil-layer enthalpy | soil-thermal V2 owner |
| `R_k` | `J m^-2 OFE-ground` | exact normalized signed-dyadic soil-layer enthalpy carry | soil-thermal V2 owner |
| `E_k` | `J m^-2 OFE-ground` | exact soil-layer enthalpy `exact(H_hi,k)+R_k` | soil-thermal V2 owner |
| `Q_soil,k`, `Q_top,k`, `Q_inf,k` | `J m^-2 OFE-ground interval` | exact accepted soil-internal, top-boundary, and infiltration energy operands | typed receipt inputs |
| `U_hi,t` | `J m^-2 tile-ground` | finite binary64 high term mirrored into frozen LSE V3 and surface-owner V2 fields | LSE exact-surface owner; mirror only on the V16 successor path |
| `R_U,t` | `J m^-2 tile-ground` | exact normalized signed-dyadic surface-enthalpy carry | LSE exact-surface owner |
| `U_t` | `J m^-2 tile-ground` | authoritative exact surface enthalpy `exact(U_hi,t)+R_U,t` | LSE exact-surface owner |
| `Q_surface,t,j` | `J m^-2 tile-ground interval` | exact dyadic decode of one finite accepted phase-free, fusion, or retained-ingress energy operand | typed surface-energy receipt input |

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
| exact-carry schema is noncanonical, identity/receipt/restart/checkpoint join fails, an operand/high term is nonfinite, exact-total rounding overflows, or exact reconstruction fails | reject before installation and roll back the complete envelope | `LSEB-E-049` |
| exact surface-enthalpy successor is absent or noncanonical, a V2/V3 high mirror differs, an accepted surface operand is missing/duplicated/stale, exact rounding/reconstruction fails, or restart/rollback identity does not join | reject before installation and roll back every joined owner | `LSEB-E-050` |
| a post-phase litter-liquid candidate exceeds its configured liquid capacity and the exact typed spill mass/enthalpy cannot be constructed, joined to the phase receipt, admitted once to current ingress, or reconstructed by the exact-surface owner | reject before phase-adjusted owner sealing and roll back every joined owner | `LSEB-E-047/048/050` / `SURFACELIQUID-E-003/009..012` |

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
| `INV-LANDSURFACEENERGY-130` | A covered-canopy or Stage 3 terminal-liquid temperature represented as exactly one upward binary64 spacing from `T_ref` is canonicalized to exact `T_ref` at its named publication boundary; every other temperature remains unchanged or fails its existing domain guard. | exact reference-state representation authority | `[INFERENCE][Static]` | runtime/test | typed closure/domain failure |
| `INV-LANDSURFACEENERGY-138` | Every covered-column Jacobian uses the canonical centered binary64 difference at an interior coordinate and the unique inward one-sided difference only at an exact admitted closed bound; an invalid current iterate or two inadmissible probes rejects. | deterministic numerical-domain authority | `[INFERENCE][Static]` | runtime/test | typed constitutive-domain failure |
| `INV-LANDSURFACEENERGY-139` | When the current complete covered residual vector passes, the full Newton trial cannot satisfy the existing no-update witness because it is domain-invalid or a governed prospective step exceeds its unchanged threshold, and the first domain-valid halved trial has every governed exact prospective step norm inside those thresholds, the solver accepts the current iterate without installing the trial. | deterministic no-update termination authority | `[INFERENCE][Static]` | runtime/test | accepted current iterate or unchanged strict-decrease/fail-closed path |
| `INV-LANDSURFACEENERGY-150` | Every soil layer retains exact receiver-owned enthalpy `E=exact(H_hi)+R`. Each candidate exactly aggregates the beginning total plus every canonical accepted soil-internal, top-boundary, and infiltration operand, rounds once to finite binary64 nearest-even `H_hi`, and stores the exact normalized signed-dyadic remainder `R`; versioned identity, receipt, restart/checkpoint, downgrade refusal, independent reconstruction, and byte-exact rollback are mandatory. | `REF-LANDSURFACEENERGY-009/011` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-022` | `[DIRECT][Static] + [INFERENCE][Static]` | runtime/test/real-consumer | `LSEB-E-049` or hard `HOLD` pending real adoption |
| `INV-LANDSURFACEENERGY-151` | On the frozen-litter V16 successor path, every LSE tile retains authoritative exact surface enthalpy `U=exact(U_hi)+R_U`. The LSE V3 and surface-owner V2 binary64 fields remain byte-frozen nonauthoritative high mirrors, every accepted phase-free/fusion/retained-ingress operand is aggregated exactly, the exact total is rounded once to finite nearest-even `U_hi`, and the exact normalized signed-dyadic remainder is retained through receipt, restart/checkpoint, real-consumer adoption, and byte-exact rollback. | `REF-LANDSURFACEENERGY-009/012` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-023` | `[DIRECT][Static] + [INFERENCE][Static]` | runtime/test/real-consumer | `LSEB-E-050` or hard `HOLD` pending `p61` and native-forest adoption |
| `INV-LANDSURFACEENERGY-153` | The V16 exact-surface receipt seals a typed coupled-parent ending posture. Every accepted partial child advances exact surface energy/carry, frozen-parent digests, owner-state digest, and receipt-chain digest while retaining the exact persistent predecessor marker mirrored by LSE V3 and surface-owner V2. Only the child ending at the sealed parent endpoint uses the persistent-parent-final posture and stamps its transaction marker once across all three owners. Parent bounds, child support, posture, predecessor, mirror markers, and complete rollback are exact. | `SC-COUPLEDTIME-001#INV-COUPLEDTIME-006/024` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-012/013/014/025` | `[DIRECT][Static] + [INFERENCE][Static]` | receipt/runtime/restart/test | `LSEB-E-050` / `SURFACELIQUID-E-012` |
| `INV-LANDSURFACEENERGY-154` | Represented Stage-3 snow is a distinct native covered-LSE regime. It executes the standard Stage-3 covered-column map exactly once under native vegetation/LSE identities and retains that same map's exact optical and snow--soil lower-boundary receipts. Snow-free V3 remains snow-free only. While snow is represented, frozen-litter V3/V4 vapor, phase, storage, current-ingress, and WB14 work are inactive and their physical/exact owner bytes are retained. A second inner legacy LSE/hydrology envelope is forbidden. | `SC-SNOWENERGY-001#INV-SNOWENERGY-083` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-026` + ADR-0044 | `[INFERENCE][Static]` | typed regime/runtime/receipt/test | `LSEB-E-020/049/050` with complete rollback |
| `INV-LANDSURFACEENERGY-155` | A V2 unpublished soil continuation may enter a charged candidate-only LSE/V3 evaluation solely through its authenticated read-only `SoilThermalUnpublishedPhysicalBeginningV2`. It remains a non-owner and supplies no restart, checkpoint, accepted receipt, or publishable projection bytes. Final acceptance reconstructs once from the original prepared owner, the complete canonical accumulated operands, and the exact selected physical ending, then seals one V2 owner/receipt/restart bundle. | `INV-LANDSURFACEENERGY-150`, `SC-SURFACELIQUID-001#INV-SURFACELIQUID-027`, `SC-SNOWENERGY-001#INV-SNOWENERGY-084` | `[DIRECT][Static] + [INFERENCE][Static]` | typed candidate/runtime/projection/finalization/test | `LSEB-E-049` with complete rollback |
| `INV-LANDSURFACEENERGY-156` | After the bounded V3 litter phase operator, liquid above the configured litter-liquid capacity is one typed physical spill. The retained phase state and spill exactly partition the raw phase mass and sensible enthalpy at the raw ending temperature; the spill is debited from litter once, enters current-ingress/WB14 once with exact phase-receipt, transaction, support, key, area-basis, mass, and enthalpy custody, and contributes one named negative exact-surface-energy operand. It is neither clipping nor condensation and never causes a same-support phase or flux re-solve. | `INV-LANDSURFACEENERGY-144/151`, `SC-SURFACELIQUID-001#INV-SURFACELIQUID-028`, `SC-WATBAL-001#INV-WATBAL-103` | `[DIRECT][Static] + [INFERENCE][Static]` | phase split/receipt/current-ingress/exact-surface/rollback test | `LSEB-E-047/048/050` with complete rollback |
| `INV-LANDSURFACEENERGY-157` | A heterogeneous V3 water-protocol finalization joins the accepted native frozen-litter phase owner to ordinary surface withdrawals without replacing either custody chain. Native litter vapor rows already consumed by the phase receipt are matched and excluded exactly; every remaining ordinary finalized-use row is authenticated against its request and authorization and debits the phase-adjusted V2 liquid owner exactly once, in canonical key order, before the one current-ingress call. Native phase/spill/ice/enthalpy custody is retained unchanged and no energy operand is replayed or synthesized. | `SC-SURFACELIQUID-001#INV-SURFACELIQUID-004/018/028/029` + transaction atomicity | `[DIRECT][Static] + [INFERENCE][Static]` | typed row partition/V2 resource join/one-ingress/rollback test | `LSEB-E-047/048/050` or typed surface protocol failure with complete rollback |
| `INV-LANDSURFACEENERGY-158` | The V16 exact-surface owner and receipt order every record/operand group by authenticated configuration topology rank of `ofe_id`, then by the existing within-OFE surface key and operand kind/ordinal. OFE IDs are opaque: neither lexical comparison nor numeric parsing is topology authority. Bare envelope validation may prove schema, digest, uniqueness, and lineage, but installable canonical order is established only at the exact configuration join; stale configuration/digest, omission, duplication, substitution, or topology-relative reorder rejects atomically. | `SC-SURFACELIQUID-001#INV-SURFACELIQUID-014/023/030` + exact-owner transaction authority | `[DIRECT][Static] + [INFERENCE][Static]` | topology-ranked owner/receipt/configuration join and multi-digit OFE tests | `LSEB-E-050` / `SURFACELIQUID-E-012` with complete rollback |
| `INV-LANDSURFACEENERGY-159` | Trusted in-process LSE custody may move an already fully validated immutable resident/candidate revision through a private nonserializable typed handoff instead of serializing and reparsing it. The proof binds schema/model/configuration, complete state/envelope digest, transaction/predecessor/support, and—where retained publication history exists—the exact prefix count, head, tail, and chain digest. Within one admitted terminal parent, the same invariant permits: one private generation-bound plan for validated immutable configuration/topology/index facts; one per-map proof from the existing first exact forcing validation to the later validation of that pointer-identical forcing object; and one per-map borrowed proof sourced from the existing fully validated `FrozenLitterV3Resident` revision for the exact native V3 LSE and V2 surface objects. The resident proof is consumed at the existing native-validation position after V8 and ingress-schedule derivation; V8 does not attest to those distinct resident objects. Every map still validates support, duration, transaction, joint, all map-dynamic LSE/surface/vegetation/BGC/soil/hydrology/lower-boundary state, residuals, solver results, and outputs afresh. No ephemeral plan or per-map proof is digest-only, wire, cloneable, mutable, transferable, or restart-capable. The existing private validated revision may clone only inseparably with its exact immutable whole resident; it is never independently exposed as map authority. Every resident mutation fully validates the successor before atomically advancing its validated revision. Restart/checkpoint, external bytes, durable publication, and untrusted-executor returns always receive fresh full semantic validation and canonical reconstruction. | `INV-LANDSURFACEENERGY-151/153/154/156/157/158/161` + `SC-SURFACELIQUID-001#INV-SURFACELIQUID-031` + `SC-SNOWENERGY-001#INV-SNOWENERGY-083/086` + `SC-COUPLEDTIME-001#INV-COUPLEDTIME-030` | `[INFERENCE][Static]` | private validated resident/resource typestate, parent-static structural plan, per-map forcing proof, resident-revision-sourced native proof, append-only tail validator, mandatory boundary validators | `LSEB-E-040/047/048/050` with complete rollback |
| `INV-LANDSURFACEENERGY-160` | A fully validated snow-free V11 provisional execution may transfer its immutable LSE physical ending through one private move-only proof to the final accepted-slab transaction. The final path may reseal only slab/receipt-dependent identities and must not repeat phase-free evaluation, litter phase, current ingress, WB14, soil evaluation, or any energy equation. Exact physical operands, owner bytes, support, topology, configuration, predecessor, and non-slab lineage must match; the final LSE owner and complete owner set must equal the provisional physical ending byte-for-byte. The proof is single-use and absent from restart, checkpoint, receipt, publication, and wire. | `INV-LANDSURFACEENERGY-151/153/155/156/157/159` + `SC-COUPLEDTIME-001#INV-COUPLEDTIME-029` | `[INFERENCE][Static]` | private snow-free physical-reuse typestate, exact final reseal/reconstruction, physical-provider call counter, restart and poison gates | `LSEB-E-040/047/048/050` with complete rollback |
| `INV-LANDSURFACEENERGY-161` | Every charged covered map validates its ordinary or native physical LSE/hydrology/soil prefix and exact custody. Role and regime dispatch are orthogonal: ordinary maps execute the admitted surface/WB14 physical branch; native represented-snow maps execute the native snow/LSE/soil branch and validate byte-retained inactive litter/WB14 custody. `Initial` returns a physical-only endpoint; every later charge returns a non-Clone pending adjudication map only after custody validation. Outer nonclosure consumes it into history without error, dependent-only nonclosure into typed rejection, or full closure consumes the same physical prefix once into V8/vegetation/BGC/joint and complete-owner construction as the `FinalAccepted` disposition. No additional final physical map exists, and no completed nonfinal endpoint can be promoted. No map publishes. | `SC-SNOWENERGY-001#INV-SNOWENERGY-086` + `INV-LANDSURFACEENERGY-154/155/156/157/159` | `[INFERENCE][Static]` | custody-before-pending physical typestate, orthogonal role/regime dispatch, exclusive disposition and final-only constructors, exact differential and rollback guards | outer nonclosure -> history, no error; dependent-only nonclosure/wrong role/ordinal -> `DirectV11RealConsumerError::AdaptiveRefinement`; wrong identity/regime/topology/custody/disposition/leak -> `DirectV11RealConsumerError::Identity`; physical and final-constructor failures retain `LSEB-E-020/040/047/048/049/050`; complete rollback, no promotion/fallback |
| `INV-LANDSURFACEENERGY-162` | In a validated represented-snow covered solve, the ground and soil temperature equations are exact identity anchors and those coordinates affect no other normalized residual. Their canonical minus-then-plus Jacobian probes may therefore reuse the current complete residual vector, replacing only the probed anchor residual with the exact full-evaluator expression. Trial-domain admission, finite-difference stencil and arithmetic, dense Jacobian bits, LU/pivot/backtracking order, convergence, diagnostics, accepted result, and first-error precedence remain identical; every other coordinate and every non-Stage-3 regime uses the complete evaluator. | `INV-LANDSURFACEENERGY-108/138/154/159` | `[INFERENCE][Static]` | private same-solve validated Stage-3 anchor proof, exact full-evaluator differential oracle, evaluator-call counter, boundary/domain poisons | any unproved dependency or identity mismatch uses the complete evaluator; no analytic derivative, approximation, fallback, or error suppression |
| `INV-LANDSURFACEENERGY-163` | Within one covered-occupancy evaluation, the internal beta-one maximum-demand leaf state may reuse the already successful current leaf state only when every call operand is bit-identical because current beta is exact binary64 `1.0`, or when the returned private gas branch is `Inactive` or `ExactZeroPar` and that branch provably does not read beta. Sun-before-shade and current-before-maximum error precedence, equations, tolerances, residuals, branches, Jacobian, solver, and results remain bit-identical. | `INV-LANDSURFACEENERGY-108/138/162` + deterministic V10 leaf-gas branch authority | `[INFERENCE][Static]` | private same-evaluation `LeafTrialState`, exact beta predicate, beta-independent branch classifier, exhaustive-call differential oracle and call counter | every other branch or beta executes the complete beta-one call; no cross-evaluation cache, approximation, fallback, or suppressed error |
| `INV-LANDSURFACEENERGY-164` | Within one validated represented-snow Jacobian sweep, a canonical sun-leaf, shade-leaf, wet-surface, or dry-stem temperature probe may reuse private successful same-iteration node results only when a topology-generic static transitive dependency graph proves those nodes unreachable from the sole changed coordinate. Every reachable node executes one shared canonical evaluator node/tail implementation in the exact complete-evaluator arithmetic and source order. Canonical probe construction/admission, residual order and bits, dense Jacobian/LU/pivots/bounds/backtracking, errors, trajectory, diagnostics, and output remain bit-identical. | `INV-LANDSURFACEENERGY-101/108/138/154/162` + reciprocal-longwave and liquid-routing authority | `[INFERENCE][Static]` | immutable sweep base, single-use signed-probe capability, versioned/hashed topology graph, forced-complete oracle, scoped and aggregate counters, normative fallibility/crossability matrix | ordinary ineligibility or conservatively unknown edges select complete evaluation before replay; private integrity mismatch fails typed; any post-start error returns directly; no synthetic fault hook, duplicated physics math, analytic/AD derivative, coloring, sparse solve, cache, approximation, recovery fallback, or error suppression |

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

## Producer Obligations and Consumer Obligations

- `OBL-LANDSURFACEENERGY-P-001`: produce immutable, source-identified records
  for all water and energy operands and a sealed start-to-end ledger.
- `OBL-LANDSURFACEENERGY-P-002`: validate before mutation and commit energy and
  water state atomically.
- `OBL-LANDSURFACEENERGY-P-003`: expose branch, interval, tolerances, residuals,
  and exact component lineage without silent defaults or clamps.
- `OBL-LANDSURFACEENERGY-P-004`: reject all schema-v8 terminal payloads until a
  reviewed atomic cutover revises both snow and receiving-surface authority.
- `OBL-LANDSURFACEENERGY-P-005`: expose every accepted soil-internal,
  top-boundary, and infiltration energy operand with exact layer/support/source
  identity; never supply a rounded aggregate, residual, or carry.
- `OBL-LANDSURFACEENERGY-P-006`: expose every accepted per-tile phase-free,
  fusion, and retained-ingress energy amount with exact kind, ordinal, support,
  transaction, surface key, source receipt, unit, and tile-ground basis; never
  supply a producer residual, carry, or untyped aggregate.
- `OBL-LANDSURFACEENERGY-C-001`: ET supplies one actual evaporation debit and
  consumes no second latent debit.
- `OBL-LANDSURFACEENERGY-C-002`: infiltration/runoff consumes one water offer,
  returns sealed partition terms, and remains sole water-partition owner.
- `OBL-LANDSURFACEENERGY-C-003`: soil/frost consumes exactly `-G` once and is
  sole subsurface conduction/phase-state mutator.
- `OBL-LANDSURFACEENERGY-C-004`: a real scheduler consumer must prove that the
  new state and ledger affect the intended direct path before runtime closure.
- `OBL-LANDSURFACEENERGY-C-005`: soil thermal V2 alone owns the exact high/carry
  representation, credit receipt, restart/checkpoint state, and atomic commit.
- `OBL-LANDSURFACEENERGY-C-006`: the LSE exact-surface owner alone owns
  authoritative `U=exact(U_hi)+R_U`; frozen LSE V3 and surface-owner V2 fields
  are high mirrors only when joined through the V16 successor projection.
- `OBL-LANDSURFACEENERGY-C-009`: prove represented-snow classification invokes
  one standard Stage-3 covered-column map under native identities, retains its
  exact optical/lower-boundary receipts, invokes no frozen-litter V3/V4 phase,
  storage, ingress, or WB14 work, constructs no second inner envelope, retains
  V3/V4 owner bytes, transitions to snow-free V3/V4 only after the exact
  terminal split, and rolls back every owner on failure.
- `OBL-LANDSURFACEENERGY-C-010`: prove an unpublished V2 continuation enters
  LSE/V3 only as its typed authenticated physical read, emits no owner/restart
  bytes and performs no intermediate install, rejects support/predecessor/
  receipt substitution, and accepts only through one complete original-owner
  replay and one atomic V2 owner/receipt/restart seal with full rollback.
- `OBL-LANDSURFACEENERGY-C-011`: prove the post-phase raw litter liquid and
  sensible enthalpy split into one within-capacity retained state and one typed
  spill, with exact phase-receipt/transaction/support/key custody, one negative
  exact-surface operand, one current-ingress/WB14 handoff, independent tile/OFE
  mass and enthalpy reconstruction, no re-solve, and complete rollback.
- `OBL-LANDSURFACEENERGY-C-012`: prove a heterogeneous V3 batch classifies and
  consumes every finalized surface row exactly once: accepted native litter
  vapor rows only through their phase receipts and all remaining authenticated
  ordinary rows through one canonical debit of the phase-adjusted V2 owner
  before one ingress. Prove no phase/spill/ice/enthalpy replacement or energy
  replay, exact transaction/support/key/cardinality joins, and full rollback.
- `OBL-LANDSURFACEENERGY-C-013`: prove V16 exact-owner records and operand
  groups follow authenticated configuration-topology rank for opaque OFE IDs,
  including `ofe-9 -> ofe-10`, and reject lexical/numeric-derived order,
  duplicate, omission, substitution, within-OFE/operand reorder, stale
  configuration/digest, and cross-owner topology mismatch with full rollback.
- `OBL-LANDSURFACEENERGY-C-014`: prove a private immutable validated handoff
  moves an unchanged frozen-litter resident and surface-resource candidate
  without serialize/parse roundtrip or duplicate owner validation; binds the
  exact configuration, revision, transaction, predecessor, support, and
  publication-history prefix; validates only an authenticated appended tail;
  invalidates on mutation; and retains fresh full validation for restart,
  external/durable bytes, and untrusted-executor returns with full rollback.
- `OBL-LANDSURFACEENERGY-C-015`: prove snow-free provisional/final execution
  evaluates every LSE, litter-phase, surface-ingress/WB14, and soil physical
  operator exactly once; the final accepted-slab path reseals identities only,
  yields byte-identical final owners and one publication, rejects every changed
  non-slab operand and any reused or post-restart proof, and rolls back exactly.
- `OBL-LANDSURFACEENERGY-C-019`: on the authentic 52-map terminal-parent
  workload, prove exactly one parent-static configuration/topology/index
  validation, exactly 52 exact normalized-forcing validations, and exactly 52
  fresh dynamic-map validations. Prove full-versus-admitted bitwise physical
  and final-owner parity for every applicable Initial/history/final and
  direct/Half1/Half2 role/path in ordinary, native, and multilane regimes;
  ordinary maps must mint and consume zero native-resident proofs. Preserve
  exact source call and first-error order. Independently poison structural and
  native LSE configurations/states, structural and native surface
  configurations/owners, generation, topology, index, support, duration,
  transaction, joint, forcing pointer, same-digest/different-allocation,
  resident revision, proof second-use, cross-map, cross-parent, and restart;
  add competing-poison vectors across all ordered boundaries. Every rejection
  has zero fallback/publication and byte-exact rollback. Executable evidence
  must exercise the real carrier, first forcing validator, V8 structural seam,
  ingress schedule, resident revision, and native-V3 consumer; fabricated
  counters or source scanning alone cannot discharge this obligation.

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
| `H_hi,R,E,Q_soil,Q_top,Q_inf` | `J m^-2 OFE-ground` | successor exact-carry registry entries required before promotion | no conversion; exact binary64-to-dyadic decode and arbitrary-precision integer aggregation | internal typed amount; no floating residual exception | none |
| `U_hi,R_U,U,Q_surface` | `J m^-2 tile-ground` | successor surface-enthalpy exact-carry registry entries required before promotion | no conversion after the named retained OFE-to-tile credit is accepted; exact binary64-to-dyadic decode and arbitrary-precision integer aggregation | internal typed amount; no floating residual exception | none |

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
- Version-15 carry arithmetic has no tolerance: aggregate exactly, round once
  to nearest-even, and require exact reconstruction. Signed-zero treatment of
  the existing binary64 high term is unchanged; only the dyadic carry has one
  schema-zero form. `nextafter`, forced ULPs, zero snapping, subnormal flush,
  producer residuals, and canonical-zero changes are prohibited.
- Version-16 surface carry arithmetic likewise has no tolerance. The frozen
  V2/V3 high mirror must equal the exact owner's `U_hi` bit-for-bit; no ULP
  forcing, zero snap, tolerance envelope, discarded credit, or carry-to-flux
  feedback is authorized.

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
| exact soil-energy credit below high-term ULP | unchanged or nearest-even `H_hi`, exact nonzero `R`, and exact reconstructed `E` | `INV-150`, `LSEB-E-049` |
| exact retained surface-energy credit below high-term ULP | unchanged or nearest-even `U_hi`, exact nonzero `R_U`, and exact reconstructed `U` | `INV-151`, `LSEB-E-050` |

Poison vectors shall independently omit and duplicate precipitation water and
heat, runon water and heat, infiltration water and heat, runoff water and heat,
latent, sensible, shortwave, longwave, ground heat, storage change, and
evaporation. Additional vectors reject non-finite inputs,
invalid Kelvin/log/roughness/resistance domains, negative storage, duplicate
lineage, wrong branch priority, and every censored schema-v8 terminal payload.
Real-consumer proof remains intentionally unsatisfied in version 1.

### Child 2C invariant IDs

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-121` | A positive snow-free successor consumes the accepted event receipt and a covered-forest support-admission receipt. | typed receipt join / `LSEB-E-042` |
| `INV-LANDSURFACEENERGY-122` | The successor uses only post-event snow-free operands and its exact accepted support. | chronology/operand lineage / `LSEB-E-043` |
| `INV-LANDSURFACEENERGY-123` | Below-domain positive support rejects before Newton with exact owner rollback; zero support performs no physical solve. | preflight / `LSEB-E-041` |

`OBL-LANDSURFACEENERGY-C-016` requires ordinary/native physical-prefix
equality against a test-only forced-complete reference, zero nonfinal and one
final V8/transaction/owner construction, exact provider counts, role/ordinal/
identity/regime/topology/one-ULP poisons, unpublishability, and complete
rollback without fallback.

`OBL-LANDSURFACEENERGY-C-017` requires every represented-snow ground/soil
anchor probe residual vector and dense Jacobian column to equal the complete
covered evaluator bit-for-bit for centered and admitted inward stencils. It
must prove unchanged minus-then-plus trial construction and domain admission,
zero complete constitutive reevaluations for those anchor probes, complete
reevaluation for every other coordinate and regime, dependency-invalidating
poisons, unchanged first-error precedence, and authentic runner output/count
parity.

`OBL-LANDSURFACEENERGY-C-018` requires every field of the reused private leaf
state, the complete covered evaluation, normalized residuals, frozen branches,
and full solve outcome to equal an exhaustive beta-one-call oracle bit-for-bit.
It must cover exact beta one, exact-zero-PAR, inactive, positive-PAR beta one
ULP below one, centered and inward beta probes, call elimination only for the
admitted cases, unchanged call order and typed-error precedence, and authentic
runner output/count parity.

`OBL-LANDSURFACEENERGY-C-020` requires an exact forced-complete differential
oracle for every admitted component-temperature probe. It compares all replayed
and retained node values, raw/tolerance/normalized residual vectors, dense
Jacobian bits, branch identities, first errors, full potential/final solves,
diagnostics, accepted output and rollback. The matrix covers centered and inward
bounds, all wet/gas/zero-area branches, the normative fallibility/crossability
classes, every naturally occurring first error, a real two-occupancy/six-soil-
node Stage-3 fixture, reciprocal longwave between every component, and upper-
wet-temperature effects routed into every lower occupancy. It requires one
shared canonical evaluator tail, an independently enumerated complete direct-
edge graph oracle, exact no-proxy custody, and truthful map/solve/sweep audit
identities and lifecycle semantics. Crossable errors use source-real paired
error/rollback vectors; noncrossable fallible nodes use implication proofs and
authentic boundary successes; infallible nodes never receive synthetic errors.
That fixture's full interior centered sweep reports exactly 58 ordered logical
probes: 14 existing synthesized identity-anchor probes, 16 component dependency
replays, and 28 complete probe evaluations. The eight hydraulic, four beta, and
two shared-canopy-air columns retain complete evaluation.

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `LSE-CHILD2C-SUCCESSOR` | `docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001/` | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-121, INV-LANDSURFACEENERGY-122, INV-LANDSURFACEENERGY-123` | `flagged-binding-addition` | Accepted event receipt, post-event-only operands, and pre-Newton covered-forest support admission; no storage arithmetic change. |
| `LSE-V15-SOIL-THERMAL-EXACT-CARRY` | Version 15 Receiver-Owned Exact Soil-Enthalpy-Carry Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-150` | `flagged-binding-addition` | Exact receiver representation and custody only; no process-physics, tolerance, chronology, or temporal-floor change. |
| `LSE-V16-SURFACE-ENTHALPY-EXACT-CARRY` | Version 16 LSE Surface-Enthalpy Exact-Carry Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-151` | `flagged-binding-addition` | Exact per-tile LSE representation/custody with frozen V2/V3 high mirrors; no process-physics, phase, tolerance, chronology, or temporal-floor change. |
| `LSE-STAGE3-NATIVE-CROSS-REGIME` | Represented-Snow Native-LSE Cross-Regime Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-154, OBL-LANDSURFACEENERGY-C-009` | `flagged-binding-addition` | One standard Stage-3 covered-column map under native identities; litter V3/V4 remains inactive and byte-retained until the exact snow-free transition. |
| `LSE-V2-UNPUBLISHED-CANDIDATE-BEGINNING` | Candidate-Only V2 Soil-Beginning Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-155, OBL-LANDSURFACEENERGY-C-010` | `flagged-binding-addition` | Admits only a typed read-only unpublished soil beginning during candidate evaluation; publishable owner/restart custody is created once by complete final replay. |
| `LSE-V3-LITTER-PHASE-CAPACITY-SPILL` | Exact V3 Litter-Phase Capacity-Spill Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-156, OBL-LANDSURFACEENERGY-C-011` | `flagged-binding-addition` | Binds one conservative post-phase liquid/enthalpy spill to current ingress and exact-surface custody; capacity normalization, condensation aliasing, and duplicate WB14 supply remain forbidden. |
| `LSE-V3-HETEROGENEOUS-SURFACE-RESOURCE-JOIN` | Exact Heterogeneous V3 Surface-Resource Join Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-157, OBL-LANDSURFACEENERGY-C-012` | `flagged-binding-addition` | Applies only unmatched ordinary finalized uses to the accepted phase-adjusted V2 owner; native phase/vapor rows and spill custody are never replayed or replaced. |
| `LSE-V16-TOPOLOGY-RANKED-EXACT-OWNER` | Topology-Ranked V16 Exact-Surface Owner Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-158, OBL-LANDSURFACEENERGY-C-013` | `flagged-binding-addition` | Exact owner and receipt order follows authenticated configuration topology, not lexical or parsed-numeric OFE IDs. |
| `LSE-V24-VALIDATED-IN-MEMORY-HANDOFF` | Validated In-Memory LSE Custody Handoff Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-159, OBL-LANDSURFACEENERGY-C-014` | `flagged-binding-addition` | Private immutable revision-bound typestate may eliminate redundant trusted in-process serialization/validation; all trust boundaries retain full validation. |
| `LSE-V25-SNOW-FREE-PHYSICAL-REUSE` | Snow-Free Final-Receipt Reseal Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-160, OBL-LANDSURFACEENERGY-C-015` | `flagged-binding-addition` | One private single-use provisional physical ending may reseal final slab identities without rerunning LSE or adjacent owner physics; exact owners, publication, restart, and rollback remain unchanged. |
| `LSE-V27-PENDING-ADJUDICATION` | Covered pending-adjudication map amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-161, OBL-LANDSURFACEENERGY-C-016` | `flagged-binding-addition` | Initial and history maps remain physical-only; the converged pending map continues from its own LSE/hydrology/soil prefix into V8 and complete owner-envelope construction. |
| `LSE-V28-STAGE3-ANCHOR-JACOBIAN` | Stage-3 Identity-Anchor Jacobian Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-162, OBL-LANDSURFACEENERGY-C-017` | `flagged-binding-addition` | Exact represented-snow ground/soil identity-anchor probe residuals may reuse the current complete residual vector; all probe coordinates, finite differences, solver operations, results, and errors remain bit-identical. |
| `LSE-V29-LEAF-MAXIMUM-EXACT-REUSE` | Covered Leaf Maximum-Demand Exact-Reuse Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-163, OBL-LANDSURFACEENERGY-C-018` | `flagged-binding-addition` | A successful private current leaf state may satisfy the immediately adjacent beta-one maximum call only for bit-identical operands or a proven beta-independent branch; all other calls and all observable results remain unchanged. |
| `LSE-V30-CARRIER-PARENT-STATIC-VALIDATION-ONCE` | Carrier Parent-Static and Same-Map Validation-Once Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-159, OBL-LANDSURFACEENERGY-C-019` | `flagged-binding-addition` | One generation-bound parent plan retains only validated immutable structure; the existing first forcing validation can authorize its later pointer-identical V8 use; and the resident's validated revision can authorize omission of only the repeated native V3/V2 validation. |
| `LSE-V31-COMPONENT-TEMPERATURE-DEPENDENCY-REPLAY` | Component-Temperature Jacobian Dependency-Replay Amendment below | `active` | `maps-to-existing-INV` | `INV-LANDSURFACEENERGY-164, OBL-LANDSURFACEENERGY-C-020` | `flagged-binding-addition` | Revision 31 introduces new IDs `INV-164/C-020`; private same-iteration evidence may replay only the transitive dependents of component-temperature probes, while all other probes and every solver operation remain complete and bit-identical. |

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
the complete caller warm start, each Newton iteration uses
`delta_i=sqrt(epsilon)*max(abs(x_i),unit_scale_i)`. An interior coordinate
evaluates the minus point and then the plus point and uses the exact centered
difference. At an admitted closed bound where exactly one of those canonical
probes violates the existing covered-trial domain, the Jacobian uses the
unique inward one-sided difference between the valid current iterate and the
admitted probe. An invalid current iterate, or two inadmissible probes, rejects
with a typed constitutive-domain failure. The solve then uses deterministic
partial-pivot LU. First apply the existing full-trial no-update witness: when
the `b=0` trial is domain-valid, evaluate its exact prospective component
steps and accept the current iterate if the current complete residual vector
and every governed step norm pass. If that full-trial witness fails because
the trial is domain-invalid or any governed full-trial step exceeds its
unchanged threshold while the current complete residual vector passes, the
solver examines the same ordered factors `2^-b`, `b=1..20`, until the first
domain-valid halved trial. That trial is evaluated prospectively without
installing it. The exact applied hydraulic, beta, temperature, humidity, and
derived `ci` step norms are reconstructed; every norm with an existing
coordinate threshold must pass. The `ci` norm remains diagnostic because this
contract defines no independent `ci` step threshold. If those predicates pass, the
solver accepts the current iterate with no update. The accepted solution,
evaluation, state, branch, ledger, and owner candidates are exclusively those
of the current iterate; the diagnostic step norms and the exact examined
exponent contribution added to the existing cumulative backtracking count
record the prospective witness. No separate persisted or public exponent field
is authorized. This witness is unavailable when the full trial is
itself a passing existing no-update witness, the current residual vector does
not pass, the first domain-valid halved trial cannot be completely evaluated,
or any governed prospective coordinate step fails.

Otherwise the solve accepts the first factor `2^-b`, `b=0..20`, producing a
strict decrease in normalized infinity norm and installs that trial as the
next iterate. Equal pivot magnitudes choose the lowest row. Failure to decrease
through `b=20` is backtracking limit. A pivot below
`64*epsilon*matrix_inf_norm` is singular. The iteration limit is 50 completed
Newton updates. Unit scales are `1 K`, `0.001 kg kg^-1`, `1 Pa`, `1000 mm`,
and `1` for beta.

Each energy residual threshold is
`1e-6 W m^-2+1e-10*max(1,sum(abs(component operands)))`. Each water/vapor
residual threshold is `1e-12 kg m^-2 s^-1+1e-9*scale`. Accepted temperature
step is at most `1e-8 K`; humidity step at most `1e-12 kg kg^-1`; accepted
hydraulic step is at most `1e-7 mm`; and beta step at most `1e-10`.
Convergence requires all residual and step criteria on the same accepted
iterate or the exact no-update witness above, whose residuals come from that
accepted current iterate and whose prospective step norms come from the first
domain-valid halved trial. Active branches are evaluated in this order: typed domain; surface
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
support domain `dt >= 60000000000 ns`. A structural one-nanosecond clock interval
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

The final nonlinear termination rule is `SC-SNOWENERGY-001@25` /
`TOL-SNOWENERGY-005`: retain exactly the equal/opposite heat that both solvers
consumed, require the receipt reconstructed from installed endpoints to differ
by no more than `1e-9 J m^-2` and `1e-8 K`, then reseal the consumed receipt to
the exact installed snow and soil candidate identities. A larger or nonfinite
residual retries within the fixed-point cap and fails closed on exhaustion.
This rule does not change soil enthalpy, applied credit, or the existing
`1e-6 J m^-2` physical-ledger closure threshold.

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

## Version 9 exact liquid reference-state representation amendment

The liquid enthalpy datum remains exactly `T_ref = 273.15 K` and
`h_l(T)=4218*(T-T_ref) J kg^-1`. In the covered-canopy solver, the existing
inactive wet-surface coordinate anchor can represent its physically exact
phase-reference solution as the immediately adjacent binary64 value above
`T_ref`. At a 60-second admitted adaptive support, multiplying that representational
offset by a positive parcel mass produces a nonzero energy far below the
binary64 spacing of an otherwise valid persistent receiver ledger. Publishing
that artifact would require the receiver either to lose a positive credit or
to reject an exact-reference physical state.

Before a covered-canopy liquid ledger or release is materialized, and nowhere
else, define

```text
T_ref_bits       = 0x4071126666666666
T_ref_next_up    = 0x4071126666666667 = 273.15000000000003 K
Delta_T_ref_up   = T_ref_next_up - T_ref
                 = 2^-44 K = 5.684341886080802e-14 K.
```

If `T_wet`, or the mass-weighted Stage 3 terminal-liquid publication
temperature, is bit-identical to `T_ref_next_up`, set it to `T_ref` before
computing `h_l` and persist the canonical temperature and exact zero specific
enthalpy together. Exact `T_ref` is unchanged. A value below `T_ref` retains
the existing covered-canopy-snow rejection; a value at or above the second
upward binary64 neighbor is not normalized. This is a reference-state
representation rule, not a tolerance relaxation: it does not change liquid
mass, wet fraction, solver residuals, accepted support, storage arithmetic,
phase ownership, or any non-reference temperature.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-130` | Only the exact first upward binary64 neighbor of `273.15 K` is canonicalized to the exact liquid enthalpy reference before covered-canopy ledger/release or Stage 3 terminal-liquid publication. | exact-bit runtime guard plus below/at/above boundary vectors; existing typed closure/domain failure otherwise |

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
| `INV-LANDSURFACEENERGY-131` | Only a numerically inactive sun-leaf, shade-leaf, or wet-surface temperature coordinate uses `max(T_canopy, 273.15 K)` as its representational anchor; zero physical area and the existing inactive-wet residual predicate guarantee no physical operand or ledger interference. | cold-canopy zero-area phase-domain vector; unchanged active-component residual/closure guards and numerical rejection |

## Version 12 exact closed-bound finite-difference amendment

The covered-column nonlinear domain already contains closed coordinates,
including `0 <= beta <= 1`, `273.15 <= T_liquid-vapor <= 350 K` for canopy
components and liquid-bearing ground, the remaining temperature bounds, and
`0 <= q_canopy <= 0.1 kg kg^-1`. A valid current
iterate exactly at one of those bounds cannot admit both canonical centered
probes. This is a property of the existing domain, not constitutive failure at
the current physical state.

For every covered-column authority and for both owner-uncapped potential and
fixed-authorization final solves, retain the exact perturbation
`delta_i=sqrt(epsilon)*max(abs(x_i),unit_scale_i)`, unit scales, ordered minus
then plus evaluation, normalized residuals, and frozen active branches. When
both probes satisfy the existing covered-trial domain, use the exact centered
difference

```text
J[:,i] = (R(x + delta_i e_i) - R(x - delta_i e_i)) / (2 delta_i).
```

When the current iterate is valid and exactly one probe violates that domain,
use only the unique inward one-sided difference:

```text
lower bound: J[:,i] = (R(x + delta_i e_i) - R(x)) / delta_i
upper bound: J[:,i] = (R(x) - R(x - delta_i e_i)) / delta_i.
```

An invalid current iterate rejects before Jacobian construction. If neither
probe is admissible, Jacobian construction rejects with the covered
constitutive-domain error; it does not shrink `delta_i`, clamp a probe, infer a
derivative, or continue. Exact `beta=0`, `beta=1`, and exact `273.15 K` active
or zero-area liquid-vapor coordinates, and exact `273.15 K` liquid-bearing
ground coordinates are ordinary boundary cases under this rule. Values outside
their existing domains remain poisons.

This amendment changes no constitutive equation, closed bound, coordinate or
residual scale, branch predicate, branch-freezing order, diagonal scaling
authority, pivot rule, backtracking, convergence threshold, ledger, receipt,
event, custody, rollback, or fail-closed requirement. In particular, the
diagonal coordinate scaling admitted by `INV-LANDSURFACEENERGY-112` remains
exclusive to the uncapped active V10 nonpositive-assimilation potential solve;
the inward derivative rule is not scaling authority for any other solve.

| ID | Binding rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-138` | Every covered potential/final Jacobian retains centered differences for two valid probes and uses the unique inward one-sided difference only when a valid current iterate has exactly one inadmissible canonical probe. | beta lower/upper, active/zero-area canopy and liquid-bearing-ground vapor lower bound, interior centered, potential/final, invalid-current, and neither-probe poisons; typed `covered_jacobian_bound` rejection |

## Version 13 first-domain-valid no-update termination amendment

The covered solver already admits a no-update termination when the current
complete normalized residual vector passes and a valid full Newton trial proves
that every governed prospective component step is inside its unchanged
threshold. A closed phase bound can make that full trial invalid, while an
otherwise valid full trial can exceed one unchanged governed step threshold.
In either case a deterministic halving can produce a domain-valid governed
step too small to produce an observable strict residual decrease in binary64.

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

## Version 14 snow-free frozen forest-litter successor amendment

Version 14 admits `OPENWEPP_SNOW_FREE_LSE_V3` as an immutable successor for
the snow-free `forest_litter` surface only. It retains every V1 and V2 model,
configuration, state, receipt, restart, and serialized byte unchanged. V3
imports the complete V2 canopy, radiation, turbulence, soil-thermal,
water-authorization, numerical, closure, 60-second support-admission, event,
and rollback authority, then adds the phase-specific litter rules below. It
does not widen bare-mineral, ponded, soil-frozen, snow-present, or snow-terminal
authority.

### Retained authority and adjudicated constants

The peer-reviewed R-156 authority is
`references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf`, Appendix A,
equations A1--A4 and A7--A14, SHA-256
`2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d`.
The retained official SURFEX v8 sources are
`references/vendorable/surfex-v8/isba_meb.F90.source.html`, generated source
lines 1992--2159, SHA-256
`0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a`;
`references/vendorable/surfex-v8/isba_fluxes_meb.F90.source.html`, generated
source lines 388--407, SHA-256
`e0378bc89ee0d52cffe14841aac56de1d8d379edf18ad29f24cfdb9ea0dfdbbc`;
and `references/vendorable/surfex-v8/ini_csts.F90.source.html`, generated
source lines 146--157, SHA-256
`f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a`.
The retained source is governed by the retained CeCILL-C v1 English license,
SHA-256
`7280115e43fa03917f2f23370519be8c9fb0b57f4c86f8da5f7ac10c070f6aa0`.

The selected constants are

```text
T_ref    = 273.15 K
rho_w    = 1000 kg m^-3
rho_i    = 920 kg m^-3
C_w      = 4218 J kg^-1 K^-1
C_i      = 2106 J kg^-1 K^-1
L_f      = 333700 J kg^-1
tau_ice  = 3300 s
W_i,max  = 0.85*rho_w*dz_l kg m^-2 tile-ground.
```

R-156 supplies `T_ref`, `rho_i`, and `C_i`; the named SURFEX instantiation
supplies `tau_ice`, `L_f`, executable ordering, and the `0.85 m3 m^-3`
capacity. The capacity is liquid-water-equivalent because generated lines
2080--2081 convert both liquid and litter ice with `rho_w`. Therefore
`0.85*rho_i*dz_l` is a rejected capacity. The R-156 A4 printed sign conflicts
with A1--A3 and its melt prose; generated lines 2089--2117 and exact
liquid/ice conservation select `signed_phase = m_freeze - m_melt`.

### V3 state, phase-free solve, and signed vapor

Hydrology's versioned surface owner exclusively owns finite, nonnegative
`W_l` and `W_i` in `kg m^-2 tile-ground`; `W_i` is liquid-water-equivalent
litter ice and is neither snow nor soil `frozwt`. LSE V3 owns the surface
sensible-energy coordinate

```text
U = (C_dry + W_l*C_w + W_i*C_i)*(T_l-T_ref)   [J m^-2 tile-ground].
```

The complete V2 nonlinear system is first solved phase-free: no freeze, melt,
fusion term, phase-updated capacity, or phase-adjusted temperature appears in
any Newton residual, Jacobian, active branch, water authorization, or
convergence witness. Its immutable beginning-phase availability is
`W_l,0,W_i,0`. Define

```text
p_i = 0                                  when W_l,0 + W_i,0 = 0
p_i = W_i,0/(W_l,0 + W_i,0)             otherwise
h_ul = 0.5*(1-cos(pi*W_l,0/W_l,max))
h_ui = 0.5*(1-cos(pi*W_i,0/W_i,max))
v_l,raw = (1-p_i)*rho_a*(h_ul*q_sat_liquid(T_l,p)-q_recipient)/r_l-c
v_i,raw = p_i*rho_a*(h_ui*q_sat_liquid(T_l,p)-q_recipient)/r_l-c.
```

Both phases deliberately use saturation over liquid water, as R-156 A8
states; saturation over ice is not admitted. Positive `v_l` is liquid
evaporation and positive `v_i` is ice sublimation; negative values are liquid
condensation and ice deposition. Hydrology authorizes the two signed
components separately. Each positive finalized mass is bounded by only its
named immutable beginning pool; a negative component has no availability cap
and credits only its named phase. A liquid request cannot debit ice, an ice
request cannot debit liquid, and neither can use current ingress.

The phase-specific energy fluxes leaving the litter are

```text
Q_v,l = v_l*[C_w*(T_l-T_ref) + L_v(T_l)]
Q_v,i = v_i*[C_i*(T_l-T_ref) + L_s(T_l)].
```

They remain separately signed through authorization, owner-candidate
construction, receipt sealing, and independent reconstruction. Only then may
the air-side consumer aggregate `v_l+v_i` or `Q_v,l+Q_v,i`. Latent-only
energy, absolute-value aggregation, one total-store availability cap, or a
second SC-EVAP debit is invalid.

After fixed-authorization phase-free acceptance, install the finalized vapor
mass and energy exactly once:

```text
W_l,* = W_l,0 - dt*v_l
W_i,* = W_i,0 - dt*v_i
U_*   = (C_dry + W_l,*C_w + W_i,*C_i)*(T_*-T_ref).
```

The signed equations include condensation/deposition because negative vapor
adds mass. The accepted phase-free ledger must independently reconstruct the
same `U_*`, `T_*`, phase masses, and phase-specific vapor energy before phase
change begins. No raw request, authorization-as-use, or producer residual may
be installed.

### Bounded kinetic phase and fusion-energy closure

Require `0 <= W_i,* <= W_i,max`, `W_l,* >= 0`, positive finite `dz_l`, and a
positive finite ending heat capacity. With all operands evaluated from the
accepted post-vapor/pre-ingress state, define

```text
M_warm   = rho_i*C_i*dz_l*max(T_*-T_ref,0)/L_f
M_cold   = rho_i*C_i*dz_l*max(T_ref-T_*,0)/L_f
m_melt   = min(W_i,*, (dt/tau_ice)*min(M_warm,W_i,*))
m_freeze = min(W_l,*, W_i,max-W_i,*,
               (dt/tau_ice)*min(M_cold,W_l,*))
m_phase  = m_freeze - m_melt.
```

The outer mass/capacity bounds are binding for every admitted `dt`, including
supports greater than `tau_ice`; the phase operator never creates a maximum
step-size requirement. At `T_*=T_ref`, both transfers are exact zero. The
atomic phase candidate is

```text
W_l,phase = W_l,* - m_freeze + m_melt
W_i,phase = W_i,* + m_freeze - m_melt
U_phase   = U_* + L_f*m_phase
C_phase   = C_dry + W_l,phase*C_w + W_i,phase*C_i
T_phase   = T_ref + U_phase/C_phase.
```

Thus liquid debit equals ice credit on freezing, ice debit equals liquid
credit on melting, and the phase-only total enthalpy coordinate
`H_phase = U - L_f*W_i` is invariant exactly. Deriving `T_phase` with the
pre-phase heat capacity or applying literal `T += L_f*m_phase/C_*` is rejected
because it leaves an unowned heat-capacity-change energy term. A phase result
never triggers a same-support flux, fixed-point, water-authorization, or
Newton re-solve; it is the ending thermal state and next-support warm start.

### Ingress, WB14, identity, restart, receipts, and failure posture

Only after the vapor and phase candidates pass may current precipitation,
runon, throughfall, canopy drainage, stemflow, and litter overflow enter the
existing hydrology chronology. Every admitted current parcel is liquid and
carries the unchanged V2 liquid sensible enthalpy. The complete owner
candidate then executes the existing WB14 partition with liquid-only
availability. `W_i,phase` cannot infiltrate, run off, drain, satisfy WB14,
enter soil `frozwt`, or mutate soil. Ingress cannot retroactively donate to
the already accepted vapor or phase operation.

The immutable model tag is `OPENWEPP_SNOW_FREE_LSE_V3`; the phase receipt tag
is `OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1`. The V3 definition binds its V2
parent digest, both terminal contract digests, all retained-source hashes,
selected constants, equations, ordering, error map, and explicit refusals.
Checked V2-to-V3 LSE migration copies every V2 scientific value bit-identically
and changes only successor identity/digest material while joining an explicit
surface-owner V2 state. Checked surface-owner V1-to-V2 migration initializes
litter ice to exact zero; a new V2 seed may instead carry explicit finite
nonnegative ice. Temperature never synthesizes initial ice. Production V3-to-
V2 LSE and surface-owner V2-to-V1 downgrades are prohibited.

Every restart/checkpoint carries explicit LSE V3, surface-owner V2, model-
definition, contract, configuration, state, phase-state, and receipt tags and
digests. Missing, duplicate, stale, mixed-version, unknown, or digest-mismatched
restart material rejects before evaluation. A sealed phase receipt binds the
half-open support and exact duration bits; OFE/tile/area/owner/transaction
identities; beginning `W_l,W_i,U,T`; both raw and finalized signed vapor
components and phase-specific energies; `W_l,*,W_i,*,U_*,T_*`; all phase
constants, capacities, transfer bounds, `m_melt,m_freeze,m_phase`; the
phase-ending state; current-ingress parcel identities; liquid-only WB14
handoff; beginning and candidate owner digests; and independent mass, energy,
and `H_phase` reconstructions. Receipt serialization uses repository canonical
framing and never hashes a producer residual as proof.

Any domain, identity, availability, closure, receipt, restart, WB14, later
owner, or publication failure rolls back the LSE, surface owner, hydrology,
soil thermal, vegetation, BGC, receipts, checkpoint, and enclosing transaction
to byte-identical beginnings. The typed families are `LSEB-E-045` for V3
identity/domain/migration, `LSEB-E-046` for phase-specific vapor custody or
enthalpy, `LSEB-E-047` for phase mass/fusion/ending-capacity closure, and
`LSEB-E-048` for chronology/receipt/restart/rollback joins.

SC-EVAP-001 remains the owner of daily WB17 soil/residue/canopy ET. It neither
owns nor repeats this subdaily pre-WB14 litter liquid/ice vapor transaction.
SC-SURFACELIQUID-001 v14 owns the matching surface state/custody specialization,
and SC-WATBAL-001 retains liquid-only WB14 ownership. The exact
`60000000000 ns` physical fallback floor, support receipt, closure tolerances,
event chronology, topology, custody, and fail-closed posture are unchanged;
stable supports must still accept steps substantially larger than 60 seconds.

### V3 invariants and required production vectors

| ID | Binding V3 rule | Guard/failure |
|---|---|---|
| `INV-LANDSURFACEENERGY-140` | V3 is an immutable V2 successor; V1/V2 scientific and serialized bytes remain unchanged and mixed identities reject. | identity/migration/restart / `LSEB-E-045/048` |
| `INV-LANDSURFACEENERGY-141` | The nonlinear solve is phase-free and publishes separate signed liquid and ice vapor under liquid-water saturation with phase-specific sensible-plus-latent enthalpy. | exact component/authority joins / `LSEB-E-046` |
| `INV-LANDSURFACEENERGY-142` | Finalized vapor uses immutable beginning phase availability, installs each mass/energy component exactly once, and cannot consume current ingress. | D/A/F and owner-candidate reconstruction / `LSEB-E-046` |
| `INV-LANDSURFACEENERGY-143` | The `3300 s` kinetic operator is bounded by named liquid, ice, and liquid-water-equivalent ice-capacity operands and uses `freeze-melt`. | independent transfer reconstruction / `LSEB-E-047` |
| `INV-LANDSURFACEENERGY-144` | Equal liquid/ice transfer and `L_f*m_phase` conserve `U-L_f*W_i`; ending temperature uses ending dry/liquid/ice heat capacity. | independent mass/fusion/temperature closure / `LSEB-E-047` |
| `INV-LANDSURFACEENERGY-145` | Phase precedes current liquid ingress and liquid-only WB14; litter ice never enters soil, runoff, infiltration, or `frozwt`. | chronology and owner guard / `LSEB-E-048` |
| `INV-LANDSURFACEENERGY-146` | Phase is post-solve and cannot trigger a same-support re-solve; exact 60-second fallback and larger-support obligation remain unchanged. | solve-count/support receipt / `LSEB-E-048` |
| `INV-LANDSURFACEENERGY-147` | Successor restart and phase receipts bind every primitive operand and all owner identities; any later failure rolls back the full envelope byte-exactly. | restart/receipt/rollback / `LSEB-E-048` |
| `INV-LANDSURFACEENERGY-148` | SC-EVAP daily WB17 does not duplicate V3's subdaily pre-WB14 surface vapor debit or credit. | cross-owner exact-one guard / `LSEB-E-046/048` |
| `INV-LANDSURFACEENERGY-149` | Canonical `p61` and native-forest consumers must install and persist V3 phase state with independent mass/energy closure; producer-only or synthetic evidence cannot close runtime adoption. | real-consumer gate / hard `HOLD` until run |

Contract-derived vectors must cover exact empty, all-liquid, all-ice, mixed,
freezing, melting, exact `T_ref`, condensation, deposition, availability-capped
evaporation/sublimation, ice-capacity saturation, `dt<tau_ice`, `dt=tau_ice`,
`dt>tau_ice`, exact 60 seconds, and substantially larger stable support.
Independent poisons must distinguish wrong A4 sign, `273.16 K`, `rho_i=917`,
wrong `L_f`, wrong `tau_ice`, `rho_i` ice capacity, saturation over ice,
latent-only vapor, total-store capping, simultaneous double debit, instant
equilibrium, freeze-only logic, pre-phase capacity temperature, current-ingress
donation, ice-as-WB14 supply, same-support re-solve, implicit ice initialization,
production downgrade, `zertol` ice deletion, `xwgmin` regularization, soil
compensation, producer-residual closure, stale restart, and partial commit.

The unchanged canonical real-consumer obligations are
`tests/integration/erosion_single_ofe_p61_sediment.rs` and
`tests/integration/dff_ws1_native_forest_cli.rs`. Each must run through the
production selector, persist/reload successor state, and prove primitive-
operand liquid/ice mass, fusion-energy, vapor-energy, WB14, and whole-envelope
closure. Contract or source scanning alone is intentionally insufficient.

## Version 15 Receiver-Owned Exact Soil-Enthalpy-Carry Amendment

Version 15 corrects a representation defect only. A finite accepted energy
credit can be far smaller than the binary64 spacing of a persistent soil-layer
enthalpy high term. Discarding that credit violates exact energy custody, while
forcing a high-term ULP invents energy. Therefore each soil layer's V2 owner
stores the exact total

```text
E_k = exact(H_hi,k) + R_k                         [J m^-2 OFE-ground],
```

where `H_hi,k` is finite binary64 and `R_k` is an exact normalized signed
dyadic. This amendment changes no flux, constitutive equation, heat capacity,
temperature relation, phase behavior, closure tolerance, event, topology,
support, or solver iteration. Version 14's vapor--phase--ingress--liquid-only-
WB14 chronology and exact `60000000000 ns` fallback floor are unchanged;
ordinary stable supports remain substantially larger than 60 seconds.

The canonical wire representation is:

```text
ExactDyadicEnthalpy {
    sign: -1 | 0 | 1,
    coefficient_hex: lowercase hexadecimal nonnegative integer,
    exponent2: signed decimal integer
}
value = sign * coefficient * 2^exponent2 J m^-2 OFE-ground.
```

Zero is uniquely `(0,"0",0)`. Nonzero values require sign `-1` or `1`, a
positive odd coefficient in lowercase hexadecimal without a leading zero, and
the unique exponent remaining after every factor of two is removed. The
coefficient is arbitrary precision. Equivalent noncanonical forms, embedded
signs, uppercase digits, even coefficients, negative zero, and zero with a
nonzero exponent reject. Resource bounds may protect parsing but must accept
every carry reachable from the complete configured finite transaction.
The unique carry zero is new-schema normalization only. It cannot rewrite the
existing binary64 high-term signed-zero representation: migration and no-op
transactions preserve those high-term bits, and this amendment introduces no
new high-term canonical-zero rule.

`SoilThermalOwnerEnvelopeV2` contains ordered
`SoilThermalLayerStateV2` records with `temperature_k`, `H_hi,k`, `R_k`, and
last accepted transaction. It and `SoilThermalOwnerRestartV2`,
`SoilThermalOwnerCheckpointV2`, and `SoilThermalEnergyCreditReceiptV2` bind the
V2 schema and exact-carry definition digests, frozen V1 parent digest,
configuration/model/contract versions, ordered layer set, state digest,
transaction/predecessor, support, and receipt chain. Checked V1-to-V2 migration
copies every V1 field and binary64 bit and adds the canonical exact-zero carry;
it never reconstructs or changes temperature. Production downgrade is always
rejected, including when all carries are zero.

For each immutable candidate, the receiver executes:

1. Validate the complete V2 beginning envelope, exact layer order and
   identities, digest/version lineage, predecessor, support, and finite high
   terms. Decode every canonical `R_k` and reconstruct `E_begin,k` exactly.
2. Validate the layer-keyed credit receipt's canonical ordered list of every
   accepted soil-internal conduction/storage, surface or snow top-boundary, and
   infiltration energy operand. Each operand is decoded exactly from its finite
   binary64 physical receipt and retains source owner/kind, equal-and-opposite
   join where applicable, OFE/layer/support, transaction, units, basis, and
   ordinal. No producer aggregate, residual, or carry is accepted.
3. Compute using exact integer arithmetic
   `E_candidate,k=E_begin,k+sum(Q_soil,k)+sum(Q_top,k)+sum(Q_inf,k)`.
   Exact addition makes the result independent of machine addition order;
   canonical receipt order remains binding custody and digest identity.
4. Correctly round `E_candidate,k` exactly once to binary64 round-to-nearest,
   ties-to-even, producing `H_hi,k`. Reject a nonfinite result or overflow to
   infinity; never clamp to the largest finite value.
5. Compute exactly `R_k=E_candidate,k-exact(H_hi,k)`, normalize it to the sole
   wire form, and require exact independent reconstruction of `E_candidate,k`.
6. Seal the V2 state and credit receipt, then atomically join all owners,
   restart/checkpoint state, and enclosing transaction. Any current or later
   failure preserves all beginning V1/V2 and production bytes exactly.

The canonical WAT5 vector begins at
`H_hi=-34315.42154113602 J m^-2` and adds accepted infiltration credit
`-8.0670339832330148e-19 J m^-2`, only `1.10875e-7` ULP. Correct nearest-even
rounding leaves `H_hi` unchanged and retains the exact negative nonzero carry
`(sign=-1,coefficient_hex="1dc319224e55f",exponent2=-109)`;
independent reconstruction closes exactly. Positive vectors also cover both
signs, exact-halfway even-low and odd-low ties, crossings to adjacent high
terms, opposite-sign and exact-zero cancellation, canonical-order permutation
with identical exact totals, minimum positive/negative subnormal operands,
normal/subnormal boundary crossings, and the largest-finite rounding boundary.
Overflow refuses.

Poison vectors cover NaN, both infinities, every noncanonical dyadic encoding,
coefficient/exponent overflow/resource abuse, wrong schema/definition/parent/
configuration/state/version/owner/transaction/predecessor/support/OFE/layer/
source kind/ordinal/digest, and receipt omission/duplication/reorder/
substitution. Every poison proves byte-exact rollback. Restart tests split
before and after a nonzero credit, produce identical final state/receipt/
checkpoint bytes, and reject replay. The canonical WAT5 transaction plus
unchanged `p61` and native-forest successor consumers must read, persist,
restore, and advance the V2 total. Producer-only, schema-only, diagnostic-only,
or tolerance-only evidence cannot close adoption.

Explicitly prohibited are producer-owned carry/residual, compensated floating
state, tolerance or closure-envelope laundering, `nextafter`, forced-ULP
installation, zeroing a nonzero carry, subnormal flushing, changing high-term
signed-zero/canonical-zero semantics, a process-physics or temperature change,
production downgrade, persistent microstepping or carry diagnostics, and
partial commit. `LSEB-E-049` is the typed version-15 schema/domain/identity/
receipt/exact-reconstruction/restart/rollback failure family.

## Version 16 LSE Surface-Enthalpy Exact-Carry Amendment

Version 16 corrects the analogous representation defect at the retained LSE
surface. The frozen LSE V3 and surface-owner V2 binary64 fields cannot retain a
valid accepted per-tile energy credit below their high-term spacing. Discarding
that credit violates exact energy custody; forcing `nextafter` or one high-term
ULP invents energy. This amendment therefore adds one minimal LSE-owned
companion owner. It does not mutate or reinterpret standalone V1/V2/V3 wire
bytes. On the successor path only, each frozen binary64 field is a
nonauthoritative high mirror joined bit-for-bit to the authoritative exact
owner:

```text
LseSurfaceEnthalpyStateRecordV1 {
    surface_key,
    enthalpy_hi_j_m2_tile: U_hi,
    enthalpy_carry: R_U,
    last_accepted_transaction_id
}
U = exact(U_hi) + R_U                         [J m^-2 tile-ground].
```

`R_U` uses the unchanged canonical `ExactDyadicEnthalpy` wire definition from
version 15. Its unique carry-zero form does not canonicalize the binary64 high
term. `LseSurfaceEnthalpyOwnerEnvelopeV1`,
`LseSurfaceEnthalpyOwnerRestartV1`,
`LseSurfaceEnthalpyOwnerCheckpointV1`, and
`LseSurfaceEnthalpyEnergyCreditReceiptV1` bind the exact schema and definition
digests, frozen LSE V3 and surface-owner V2 parent digests, configuration and
ordered surface keys, transaction/predecessor, half-open support, owner/state
digests, and receipt chain. `SurfaceLiquidCompleteOwnerProjectionV4` joins the
unchanged projection-V3 bytes with this owner; projection V3 bytes remain
frozen. Production downgrade or execution without the exact owner is rejected
after V16 adoption, even when every carry is zero.

Checked adoption requires the frozen LSE V3 and surface-owner V2 high fields to
be bit-identical for every complete ordered surface key, copies those bits to
`U_hi`, and initializes only `R_U` to canonical exact zero. It derives no
enthalpy from temperature and changes no mass, phase, receipt, or high-term
bits. A mixed key set, signed-zero mismatch, stale parent, or partial adoption
rejects before candidate work.

For each immutable candidate and each surface key, the receiver executes:

1. Validate the exact beginning total, complete key order, both frozen high
   mirrors, definition/schema/configuration/state identities, predecessor,
   support, restart/checkpoint lineage, and finite `U_hi`.
2. Validate one canonical ordered list of accepted surface-energy operands.
   The exhaustive kinds are `phase_free_surface_energy`,
   `litter_fusion_energy`, and `retained_ingress_tile_credit`. Each operand is
   the exact dyadic decode of the finite binary64 amount actually accepted by
   its physical receipt and retains kind, ordinal, source owner/receipt,
   transaction, support, OFE/tile/surface key, units, and tile-ground basis.
   A phase-free operand list exposes the primitive accepted radiative,
   sensible, phase-specific vapor, and ground amounts; a producer residual or
   already-rounded aggregate is not an operand. Fusion is exactly the accepted
   binary64 `L_f*(m_frz-m_mlt)` receipt amount and occurs once.
3. For retained ingress, group all accepted retained parcel receipts by the
   complete destination surface key, sort by canonical receipt identity,
   reconstruct the existing finite binary64 OFE-ground group amount and its
   named finite binary64 OFE-to-tile result, and expose each resulting
   `retained_ingress_tile_credit` exactly once. Exact-carry arithmetic begins
   only after this existing physical basis conversion; it neither changes the
   parcel partition nor invents a rational replacement for the accepted
   binary64 tile credit.
4. Compute with exact integer arithmetic
   `U_candidate=U_begin+sum(Q_surface,j)`. Mathematical addition is
   order-independent; canonical operand order remains receipt identity. Round
   the exact total once to binary64 nearest-even `U_hi,candidate`, rejecting a
   nonfinite result or overflow. Compute and normalize
   `R_U,candidate=U_candidate-exact(U_hi,candidate)` and require exact
   independent reconstruction.
5. Require the LSE V3 candidate and surface-owner V2 candidate to carry the
   exact same `U_hi,candidate` bits as nonauthoritative mirrors. Their
   temperatures remain derived by the unchanged V14 heat-capacity equations
   from that high coordinate used by the constitutive solver. The carry is
   energy custody only: it is included in exact beginning/ending storage and
   closure, never converted to a temperature perturbation, flux, phase mass,
   residual tolerance, or solver forcing.
6. Seal the exact owner, credit receipt, both mirror joins, projection V4,
   restart/checkpoint state, and enclosing complete transaction atomically.
   Any current or later failure preserves all beginning V1/V2/V3, exact-owner,
   receipt, restart/checkpoint, and production bytes exactly.

Independent vectors must reconstruct `U_begin`, every named operand,
nearest-even high, carry, exact ending `U`, and both high mirrors without
reading a producer residual. They cover zero credit; positive and negative
sub-ULP credit; exact-halfway even-low and odd-low ties; high-term crossings;
opposite-sign cancellation to canonical carry zero; multiple retained parcels
for one tile; two tiles with distinct fractions and credits; fusion plus
retained ingress; minimum positive/negative subnormal operands; largest-finite
boundary and overflow refusal; and split restart before and after a nonzero
carry. Poisons cover omission, duplication, reorder, wrong kind/ordinal/source,
wrong OFE/tile/surface/support/transaction/predecessor, V2/V3 high-mirror
mismatch, stale projection/restart/checkpoint, noncanonical carry, producer
residual, tolerance repair, zero snap, discarded credit, and forced ULP. Every
poison proves full byte-exact rollback.

The retained `p61` failure support is exactly
`176400000000000..178200000000000 ns`. The retained run identified the
sub-ULP retained-surface-credit predicate but did not preserve the exact
beginning high bits or retained tile-credit operands. No numeric oracle is
invented from that evidence. Implementation must capture those typed operands
from the unchanged fixture, bind them into the receipt, and then prove exact
independent reconstruction plus split-restart equivalence. The unchanged
native-forest fixture carries the second real-consumer gate. Producer-only,
schema-only, synthetic-only, or tolerance-only evidence cannot close adoption.

This amendment changes no radiation, turbulence, vapor, soil heat, bounded
`3300 s` phase law, mass/fusion equation, water authorization, ingress/WB14
chronology, event, topology, physical closure tolerance, or solver rule. The
exact `60000000000 ns` fallback floor remains unchanged, and stable ordinary
supports must still accept steps substantially larger than 60 seconds.
Persistent microstepping/carry diagnostics, a forced ULP, zero snapping,
subnormal flushing, tolerance laundering, and partial commit are prohibited.
`LSEB-E-050` is the typed version-16 schema/domain/identity/operand/mirror/
exact-reconstruction/restart/rollback failure family.

## Change Log

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-09-04 | 31 | Codex | Corrected the contract-first component-temperature dependency-replay feasibility after full production revert: replaced impossible every-node poisons with a normative fallibility/crossability matrix, source-real first-error/rollback evidence, one shared canonical evaluator implementation, complete direct-edge/custody/audit obligations, and retained canonical physics, complete hydraulic/beta/shared-air probes, dense solver behavior, errors, outputs, and rollback. |
| 2026-09-04 | 30 | Codex | Extended existing `INV-LANDSURFACEENERGY-159` with one generation-bound terminal-parent structural-validation plan, one source-ordered forcing proof, and one same-map proof sourced from the native resident's validated revision; structural V8 and resident V3/V2 objects remain distinct, every map's dynamic state remains freshly validated, all trust boundaries retain full validation, and equations, tolerances, solver behavior, outputs, and wire formats are unchanged. |
| 2026-09-04 | 28 | Codex | Admitted exact dependency reuse for represented-snow ground/soil identity-anchor Jacobian probes only; canonical probe coordinates, domain admission, finite-difference arithmetic, dense Jacobian bits, solver order, results, and errors remain unchanged. |
| 2026-09-03 | 27 | Codex | Corrected final custody to consume the converged pending adjudication map's own LSE/hydrology/soil prefix once; no independently replayed final physical map or nonfinal promotion is authorized. |
| 2026-09-02 | 26 | Codex | Split nonfinal covered-map physical endpoints from the independently charged final complete owner envelope; equations, operands, exact native identity, charge budget, wire, and rollback are unchanged. |
| 2026-08-31 | 16 | Codex | Added a minimal LSE-owned exact per-tile surface-enthalpy companion `U=exact(U_hi)+R_U`, immutable V3/V2 binary64 high mirrors, exact named phase-free/fusion/retained-ingress tile-credit aggregation, one nearest-even finite high rounding, successor receipt/restart/checkpoint/projection custody, exact rollback and `p61`/native real-consumer gates, with no physics, phase, tolerance, chronology, or exact-60-floor change. |
| 2026-08-30 | 15 | Codex | Added receiver-owned exact soil-layer enthalpy `E=exact(H_hi)+R`, canonical normalized signed-dyadic carry, exact accepted operand aggregation, one nearest-even finite high-term rounding, immutable V1-to-V2 zero-carry migration, versioned credit/restart/checkpoint custody, exact rollback, WAT5 sub-ULP and numeric/identity/restart/real-consumer gates, with no process-physics, tolerance, v14 chronology, or 60-second-floor change. |
| 2026-08-30 | 14 | Codex | Admitted immutable snow-free forest-litter LSE V3 authority: phase-free V2 solve, separately finalized liquid/ice signed vapor, bounded 3300-second kinetic freeze/melt, exact equal-mass and fusion-energy closure with ending heat capacity, post-phase current ingress and liquid-only WB14, successor identities/restart/receipts/rollback, unchanged exact 60-second fallback, and mandatory p61/native real-consumer proof. |
| 2026-08-30 | 13 | Codex | Extended covered-column no-update termination only to the first domain-valid halved trial when the full trial fails the existing witness by domain invalidity or governed-step excess and the current complete residual vector plus every governed halved prospective step already pass; the accepted current iterate is unchanged and all actual updates retain strict decrease. |
| 2026-08-29 | 12 | Codex | Generalized the covered-column finite-difference stencil to use the unique inward one-sided derivative at exact admitted closed bounds for potential and final solves, while preserving the exact centered interior stencil, perturbations, scaling authority, branches, backtracking, ledgers, receipts, and rollback. |
| 2026-08-28 | 11 | Codex | Kept numerically inactive sun, shade, and wet liquid-vapor coordinates inside the existing `273.15 K` phase domain by anchoring them to `max(T_canopy, T_ref)`; zero-area physical operands, active equations, tolerances, ledgers, exact-60 fallback, events, and fail-closed behavior remain unchanged. |
| 2026-08-28 | 10 | Codex | Bound causal snow--soil receipt termination to finite `1e-9 J m^-2` energy and `1e-8 K` endpoint residuals while preserving the exact applied soil credit, exact installed owner identities, and unchanged physical-ledger closure threshold. |
| 2026-08-28 | 9 | Codex | Extended the existing exact one-ULP liquid phase-reference representation rule to Stage 3 terminal-liquid committed publication; all other temperatures and every ledger tolerance remain unchanged. |
| 2026-08-26 | 9 | Codex | Admitted exact one-ULP upward canonicalization of the covered-canopy wet-liquid phase reference before ledger and release publication, with an explicit `2^-44 K` threshold, exact-bit boundary, and no normalization of other temperatures. |
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

## Version 9 positive-support admission owner amendment

`OPENWEPP_SNOW_FREE_LSE_V1` retains nanosecond chronology at the coupled-time
wire boundary, but its physical nonlinear solver has a deterministic declared
positive-support domain. The released domain policy is
`OPENWEPP_SNOW_FREE_LSE_V1_SUPPORT_POLICY_V1` with
`minimum_support_ns = 60000000000` (60 seconds). This is the exact Stage-3
adaptive temporal floor selected by the 2026-08-27 owner amendment. It is a
conservative model-specific numerical-domain boundary; open-mineral, litter,
and other surface profiles remain prospective/non-admitted until separately
profiled. It is not
a hidden duration floor, a changed V10 cadence, or a promise that every larger
support is globally convergent under arbitrary forcing.

The amendment changes temporal admission only. It does not change any LSE
constitutive equation, conservation or liquid/energy custody, phase ownership,
active-set/topology predicate, receipt identity, rollback, or fail-closed
behavior. Stable ordinary supports must accept steps substantially larger than
60 seconds. Every earlier admission, trace, or performance result that depended
on the provisional 0.6-second floor is superseded and requires a fresh
amended-floor run; none is claimed here.

For `OPENWEPP_STAGE3_ADAPTIVE_OWNER_TOLERANCE_V1` only, direct-versus-composed
LSE energy uses `1e-6 J m^-2 + 5e-3*max_abs`, soil-thermal energy uses
`1e-6 J m^-2 + 1.5e-2*max_abs`, and LSE/soil temperature uses
`1e-2 K + 1e-8*max_abs`. These are truncation-error controller bounds, not
constitutive, residual, mass, or energy-closure tolerances; all exact owner,
topology, phase, and receipt predicates remain exact.

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
finite-capacity V11 actual-stack profile. The prior 0.6-second profiling and
execution evidence is historical and does not validate the amended floor. The
declared 60-second policy remains fail-closed; fresh fixture execution is
required before any conformance or performance claim. A future smaller or additional
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
| `LSEB-E-041` | Requested positive support is below `60000000000` ns for the declared LSE policy; typed pre-Newton rejection. |
| `LSEB-E-042` | Support receipt parent/segment/slab, absolute support, duration bits, configuration/state, tolerance, or numerical-policy binding is invalid. |

| Date | Version | Author | Change |
|---|---:|---|---|
| 2026-08-27 | 9 owner amendment | Codex | Raised the Stage-3 adaptive temporal admission floor from the provisional 0.6 seconds to exactly 60 seconds (`60_000_000_000 ns`). Constitutive equations, energy/liquid custody, phase ownership, topology, receipt, rollback, and fail-closed obligations are unchanged; stable ordinary supports must accept substantially larger steps. Prior floor-dependent evidence is superseded and awaits rerun. |
| 2026-08-20 | 6 | Codex | Prospective deterministic positive-support admission; nanosecond chronology remains coupled-time-valid, below-domain support rejects before Newton, and event-boundary coalescing is deferred to the reviewed snow/event contract. |
| 2026-08-20 | 7 | Codex | Bound the covered-forest snow-free successor to the accepted event receipt, post-event-only operands, and pre-Newton support admission without scaling, freezing, or sub-ULP storage treatment. |
It does not invoke hydraulic attenuation, conductance or vulnerability floors,
plant capacitance, or authorization donation.

Every covered potential and final solve uses the exact closed-bound derivative
rule in `INV-LANDSURFACEENERGY-138`. This general numerical-domain rule does
not broaden the V10-only coordinate-scaling authority below.

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
`T_wet - max(T_canopy, 273.15 K) = 0`. Liquid mass, enthalpy, longwave area, and every owner
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
| `INV-LANDSURFACEENERGY-113` | A store-cap-active V10 nonpositive-assimilation wet coordinate below the canonical water-rate tolerance uses the domain-valid inactive anchor defined by `INV-LANDSURFACEENERGY-131` only when its physical energy residual already passes, without changing liquid or energy ledgers. |
| `LSE-E-109` | V8/V9 vegetation identity, mixed V1/V2 receipts, or any owner alias rejects before V2 physics. |
| `LSE-E-110` | Missing, duplicated, mutated, or locally recomputed V10 nighttime gas state rejects the coupled owner envelope. |
| `LSE-E-111` | Partial or value-mutating V1-to-V2 migration rejects without a V2 state. |

This amendment remains default-off. It authorizes no production selector,
default/output change, cutover, snow handoff, deployment, calibration, or
empirical claim.

## Canonical Stage-3 Accepted-Map Boundary Amendment

`INV-LANDSURFACEENERGY-152` — When the canonical Stage-3 covered solver calls
LSE as an authentic physical map, LSE retains its own `INV-138/139` covered
column algorithm and `TOL-LSE-001/002` closure. The outer Stage-3 admission
compares continuous LSE boundary payload under `TOL-SNOWENERGY-007`; it does
not replace, relax, or select the LSE solver. The accepted payload is sealed
once. Owner/receipt envelope, exact high-plus-carry representation, exact
snow--soil debit/credit identity, topology, branch, transaction, and rollback
remain exact under `INV-LANDSURFACEENERGY-150/151`.

`OBL-LANDSURFACEENERGY-C-007` — The real covered consumer must reject a
tolerance-equivalent payload with a changed branch/topology/owner envelope,
reject any unequal or duplicated snow--soil transfer, and prove continuous
boundary sensitivity through the actual Stage-3 admission seam without a
second LSE algorithm or uncharged physical evaluation.

### Profile integration

| Profile surface | Binding |
| --- | --- |
| algorithm step | After native covered-column closure, return one authentic payload to the Stage-3 map; do not run an outer-algorithm-selected LSE successor. |
| branch/guard | Same exact LSE branch/topology/owner envelope or typed `LSEB-E-049/050`; continuous outer comparison uses only `TOL-SNOWENERGY-007`. |
| invariant guard map | `INV-LANDSURFACEENERGY-152` -> Stage-3 accepted-map boundary validator and exact snow--soil debit/credit join. |
| test vector | `OBL-LANDSURFACEENERGY-C-007`: boundary/next-above, branch poison, duplicate heat, uncharged map, rollback. |
| binding exposure | `LSE-STAGE3-CANONICAL-ACCEPTED-MAP`, active, `new-INV`, IDs `152/C-007`, dual review/verification. |
| change log | 2026-09-01, contract 17: bound the canonical Stage-3 accepted-map boundary without changing native LSE physics or solver. |

## Exact-Surface Parent-Local Chronology Amendment

`INV-LANDSURFACEENERGY-153` — `LseSurfaceEnthalpyEnergyCreditReceiptV1`
seals exact parent support bounds and one canonical ending posture:
`ParentLocalPartial` when `child_end < parent_end`, or
`PersistentParentFinal` when `child_end == parent_end`. The validator derives
the posture from support; callers cannot select it independently.

A partial child advances the authoritative exact `U_hi + R_U`, frozen-parent
digests, owner-state digest, and exact receipt-chain digest, while every exact
record retains the persistent predecessor transaction marker shared bit-for-
bit by the frozen LSE V3 and surface-owner V2 mirrors. A final child stamps the
child transaction exactly once across all three owners. Beginning exact-record
markers equal the receipt predecessor in both postures. Mixed record markers,
wrong parent bounds or posture, early physical-owner advance, final predecessor
retention, omitted/duplicated exact operands, restart substitution, or replay
rejects with `LSEB-E-050` / `SURFACELIQUID-E-012` and byte-exact rollback.

The current pre-production V1 receipt wire adds the typed posture and parent
bounds to its digest preimage and changes its schema digest. Earlier V1 receipt
bytes fail closed; no implicit migration or compatibility parse is authorized.
This is custody/chronology authority only and changes no energy equation,
tolerance, temporal floor, or physical branch.

`OBL-LANDSURFACEENERGY-C-008` — Prove first and middle partial children retain
the predecessor while exact carry/state/receipt chain advance; the final child
stamps the transaction once; mid-parent restart reproduces the same canonical
bytes; wrong posture/bounds, mixed markers, early physical advance, final
predecessor retention, and injected failure all reject with full rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Derive partial/final from sealed parent and child supports after authentic V4 physics, then advance exact carry and mirrors atomically. |
| branch/guard | Partial retains predecessor; final stamps transaction once; mirror markers and exact owner remain identical. |
| invariant guard map | `INV-LANDSURFACEENERGY-153` -> exact receipt schema/lineage validator, projection V4, restart, mirror join. |
| test vector | `OBL-LANDSURFACEENERGY-C-008`: first/middle/final, split restart, marker/bounds/posture poisons, rollback. |
| binding exposure | `LSE-V16-PARENT-LOCAL-CHRONOLOGY`, active, `new-INV`, IDs `153/C-008`, dual review/verification. |
| change log | 2026-09-01, contract 18: typed partial/final exact-surface receipt chronology; unchanged physics/tolerances. |

## Represented-Snow Native-LSE Cross-Regime Amendment

`INV-LANDSURFACEENERGY-154` — Regime classification uses the immutable
represented-snow state before any physical evaluation. `OPENWEPP_SNOW_FREE_LSE_V3`
remains exclusively snow-free, and its V3 covered-column evaluator continues to
reject a Stage-3 snow lower boundary. When represented Stage-3 snow owns the
destination surface, the standard `INV-LANDSURFACEENERGY-138/139` Stage-3
covered-column physics executes exactly once under the native vegetation and
LSE owner identities. That charged map carries its own exact typed optical
receipt and `CoveredLowerBoundaryEnergyOperands::Stage3SnowCovered` lower-
boundary receipt; neither may be copied from, inferred from, or replaced by an
independent envelope.

During that represented-snow map, snow is the sole atmospheric ground surface.
Frozen-litter V3/V4 vapor exchange, liquid/ice phase work, surface-storage
arithmetic, current-ingress adoption, and WB14 are inactive. The frozen-litter
V3 physical owner, V4 exact-energy companion, their predecessor/receipt chains,
and their canonical bytes are retained unchanged. A standard covered tile
result may be embedded in the same heterogeneous native batch only as output of
that one charged native-identity map; constructing or charging a second inner
legacy LSE/hydrology envelope is forbidden.

At the earliest accepted Stage-3 terminal event the parent is split exactly.
The represented-snow child retains inactive litter custody; only a positive
snow-free successor may invoke the snow-free V3/V4 litter path, exactly once on
post-event operands. Missing or mismatched native identity, optical or lower-
boundary receipt, any under-snow litter-phase/WB14 call or byte mutation, a
second physical envelope, or failed terminal transition rejects atomically with
the existing typed LSE/surface-custody error and byte-exact rollback. This
amendment changes no equation, continuous tolerance, eight-map budget, temporal
floor, or exact receipt comparison.

`OBL-LANDSURFACEENERGY-C-009` — Prove typed represented-snow classification,
one charged standard covered map, zero under-snow V3/V4 phase/storage/ingress/
WB14 calls, exact optical and lower-boundary receipt retention, absence of a
second constructor or charge, unchanged V3/V4 owner bytes, exact terminal
transition to one snow-free successor call, poison rejection, and full rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Classify represented snow before evaluation; run one standard Stage-3 covered map under native identities, or the snow-free V3/V4 path after an exact terminal split. |
| branch/guard | Represented snow keeps litter V3/V4 inactive and byte-retained; V3 rejects Stage-3 lower-boundary input; duplicate physical construction rejects. |
| invariant guard map | `INV-LANDSURFACEENERGY-154` -> regime classifier, charged-map identity/receipt join, inactive-owner validator, and terminal split. |
| test vector | `OBL-LANDSURFACEENERGY-C-009`: classification, zero phase calls, exact optical/lower receipt, no second envelope, unchanged bytes, transition, poison, rollback. |
| binding exposure | `LSE-STAGE3-NATIVE-CROSS-REGIME`, active, `new-INV`, IDs `154/C-009`, dual review/verification. |
| change log | 2026-09-01, contract 19: one native-identity Stage-3 covered map with inactive frozen-litter custody; unchanged physics/tolerances/budget. |

## Candidate-Only V2 Soil-Beginning Amendment

`INV-LANDSURFACEENERGY-155` — A
`DirectSoilThermalUnpublishedContinuationV2` may construct one
`SoilThermalUnpublishedPhysicalBeginningV2` only after authenticating the
installed resident against the continuation's original prepared owner, the
complete immutable owner/schema/model/run/configuration/topology identity, the
exact predecessor unpublished trial and its ending-state/seal lineage, and the
exact contiguous positive next-child support. Its transaction is the existing
prepared soil transaction. Neither support nor transaction lineage may be
rebound to the outer surface/LSE transaction.

The constructed value is a borrowed constitutive read surface for one charged
candidate-only LSE/V3 evaluation. A typed V3 soil-beginning discriminator must
keep it distinct from the ordinary publishable owner-plus-restart branch. In
the unpublished branch, no `SoilThermalOwnerEnvelopeV2`, restart, checkpoint,
accepted credit receipt, receipt-free seal, or owner-shaped bytes may be
constructed, serialized into the complete-owner projection, installed,
persisted, or exposed as accepted custody. The candidate projection may retain
only a typed non-owner custody record sufficient to replay the exact original
prepared-owner digest, predecessor-trial seal, physical beginning-state digest,
transaction/predecessor/receipt-chain identities, and child support; it cannot
be accepted or restored as an owner.

Final acceptance remains the sole promotion boundary. It must independently
replay the complete original prepared owner plus every canonically accumulated
accepted operand, require the exact selected physical ending and complete
layer-credit chain, invoke the existing canonical unpublished-composition
path, and seal exactly one `SoilThermalOwnerEnvelopeV2`, energy-credit receipt,
restart/checkpoint identity, and atomic install. A candidate-only projection is
replaced, never accepted beside the publishable projection. Support rebinding,
owner-byte synthesis, private-trial promotion, intermediate or dual acceptance,
proxy physics, receipt repair, or tolerance comparison of any identity rejects
with `LSEB-E-049` and restores every owner byte.

This amendment changes no soil constitutive equation, exact-carry arithmetic,
physical operand, temperature projection, tolerance, Stage-3 budget, temporal
floor, topology, or accepted restart wire. `SC-SOIL-001` process authority is
unchanged; exact soil-thermal representation and custody remain owned here.

`OBL-LANDSURFACEENERGY-C-010` — Prove the exact typed branch and constructor,
contiguous support and predecessor-trial authentication, absence of owner/
restart/checkpoint bytes and intermediate installation, complete accumulated-
operand/physical-ending final replay, one owner/receipt/restart seal, rejection
of rebinding/substitution/dual acceptance, and byte-exact rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Authenticate the continuation and construct a read-only unpublished physical beginning; evaluate privately; compose and seal the publishable V2 owner only at final acceptance. |
| branch/guard | Exactly one of publishable owner+restart or unpublished non-owner beginning; no conversion or fallback between branches. |
| invariant guard map | `INV-LANDSURFACEENERGY-155` -> continuation constructor, typed V3 input/projection, complete outer replay, single install. |
| test vector | `OBL-LANDSURFACEENERGY-C-010`: exact child, support/predecessor/receipt poisons, no owner bytes, final replay, one install, rollback. |
| binding exposure | `LSE-V2-UNPUBLISHED-CANDIDATE-BEGINNING`, active, `maps-to-existing-INV`, IDs `155/C-010`, dual review/verification. |
| change log | 2026-09-01, contract 20: typed candidate-only unpublished V2 soil beginning and single final owner/restart promotion; unchanged physics/tolerances/wire. |

## Exact V3 Litter-Phase Capacity-Spill Amendment

`INV-LANDSURFACEENERGY-156` closes the already-required `litter overflow`
handoff between bounded phase and current ingress. The existing V3 phase
receipt remains the immutable raw bounded-phase image; it is not rewritten or
accepted as an over-capacity surface owner. For each forest-litter tile,
reconstruct the raw phase result from the accepted post-vapor state and phase
receipt in the existing order:

```text
W_raw = W_l,* - m_freeze + m_melt
W_i,end = W_i,* + m_freeze - m_melt
U_raw = U_* + L_f*(m_freeze-m_melt)
C_raw = C_dry + C_w*W_raw + C_i*W_i,end
T_raw = T_ref + U_raw/C_raw.
```

If `W_raw<=W_l,max`, the typed spill is exact positive zero and the raw state
is the retained state. If `W_raw>W_l,max`, construct one
`LitterPhaseCapacitySpillV1` by the canonical checked binary64 operations:

```text
m_spill,tile = W_raw - W_l,max
h_spill       = C_w*(T_raw-T_ref)
Q_spill,tile  = m_spill,tile*h_spill
W_retained    = W_raw - m_spill,tile
U_retained    = U_raw - Q_spill,tile
C_retained    = C_dry + C_w*W_retained + C_i*W_i,end
T_retained    = T_ref + U_retained/C_retained.
```

Every operation must be finite, `m_spill,tile>0`, `0<=W_retained<=W_l,max`,
and `C_retained>0`. The second subtraction is the authoritative remainder;
`min`, clamp, saturation normalization, a tolerance snap, or discard is not
the spill algorithm. Independent reconstruction must reproduce the raw phase
mass from retained mass plus spill and the raw sensible enthalpy from retained
enthalpy plus `Q_spill,tile`, using the declared operation order. The spill is
liquid only; litter ice and fusion energy are not reclassified as runoff.

The companion binds the original phase-receipt SHA-256, LSE configuration and
surface-owner identity, transaction, exact child support, OFE/tile/surface/
source key, `W_raw,U_raw,T_raw`, capacity, retained state, spill mass,
`h_spill`, and `Q_spill`. It cannot be caller synthesized or labeled as a
condensation credit. The phase-adjusted surface owner seals the retained state
and its closure debits `m_spill,tile` exactly once. The V16 exact-surface owner
adds exactly one named negative
`LitterPhaseCapacitySpillEnergy` operand `-Q_spill,tile`, independently
reconstructed from companion mass and temperature. This debit is in addition
to, and ordered after, the unchanged phase-free and fusion operands.

For current-ingress custody, checked area conversion produces
`m_spill,ofe=f_t*m_spill,tile`; its parcel energy is
`Q_spill,ofe=m_spill,ofe*h_spill`. One internally constructed
`LitterPhaseOverflow` parcel uses the same transaction and full accepted child
support `[0,dt)`, retains the exact source key and phase-receipt identity, and
enters the ordinary SurfaceLiquid/WB14 mixing, infiltration, excess, retention,
runoff, and topology routing path exactly once. The full-child support is the
existing aggregate LSE-child timing authority; it may not be reassigned to a
rainfall hour or another support. Any retained portion returns through the
ordinary named retained-ingress energy receipt, never by cancelling or
omitting the spill debit.

Missing/duplicate spill, wrong phase receipt, key, capacity, transaction,
support, area basis, sign, temperature, specific enthalpy, exact-surface
operand, or WB14 receipt rejects before installation. A failed split, ingress,
final replay, owner join, or publication preserves the complete LSE,
surface-liquid, exact-enthalpy, soil, WB14 parent, receipt, and runner beginning
bytes. The spill does not invoke a second vapor/phase evaluation, fixed point,
water authorization, or same-support LSE solve and changes no phase equation,
capacity, tolerance, temporal floor, or WB14 constitutive rule.

`OBL-LANDSURFACEENERGY-C-011` — Prove zero/below/at/above-capacity cases,
melt-created positive spill, exact raw-to-retained-plus-spill mass and sensible-
enthalpy reconstruction, named negative exact-surface operand, checked tile/OFE
basis conversion, full-child timing, one WB14 call/supply, retained/infiltrated/
routed dispositions, phase/key/transaction/support/enthalpy substitution
poisons, no same-support re-solve, and byte-exact rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Evaluate bounded phase once; split an over-capacity raw liquid ending into retained state plus typed spill; debit litter energy; admit the spill once to current ingress/WB14. |
| branch/guard | `W_raw<=W_l,max` has zero spill; `W_raw>W_l,max` requires the complete typed spill and exact closure. Missing custody rejects; no clamp or condensation alias exists. |
| invariant guard map | `INV-LANDSURFACEENERGY-156` -> pure spill splitter, phase companion, surface-owner closure, V16 exact debit, ingress receipt, WB14 and rollback joins. |
| test vector | `OBL-LANDSURFACEENERGY-C-011`: capacity boundary sides, melt spill, exact closures, one ingress/WB14, substitutions, no-resolve, rollback. |
| binding exposure | `LSE-V3-LITTER-PHASE-CAPACITY-SPILL`, active, `maps-to-existing-INV`, IDs `156/C-011`, dual review/verification. |
| change log | 2026-09-01, contract 21: conservative typed post-phase litter capacity spill into current ingress; unchanged phase/WB14 equations, tolerances, floor, and prior receipt bytes. |

## Exact Heterogeneous V3 Surface-Resource Join Amendment

`INV-LANDSURFACEENERGY-157` governs a V3 batch containing both accepted native
frozen-litter tiles and ordinary legacy open or covered surface withdrawals.
The unified water protocol remains the sole authority for requests,
authorizations, and finalized uses. The accepted native phase receipt remains
the sole consumer of its phase-specific litter-vapor row. Match that row by
the exact transaction, child support, OFE, tile, surface/source identity and
checked tile-fraction/support aggregation, and remove it from further resource
application. Its receipt-derived finalized amount `F` must satisfy the same
`0<=F<=A<=D` custody relation as the unified protocol; `A` remains an upper
authorization and need not be bit-identical to `F`. The finalized-use row is
bit-identical to the receipt-derived `F`. A missing, duplicate, foreign, or
out-of-bound native row rejects; it cannot be reclassified as ordinary.

The complete unmatched surface set is the ordinary set. Every ordinary row
must retain its original request/authorization/finalized-use identity and
`0<=F<=A<=D`. Aggregate those rows in complete `GroundWaterKey` order and
apply the existing checked `F/f_t` debit exactly once to the accepted
phase-adjusted V2 liquid owner. The join is typed
`SurfaceLiquidV2HeterogeneousResourceJoinV1`; it starts from that exact owner,
not from an independently reconstructed legacy beginning, and produces the
one resource candidate consumed by the existing one current-ingress call.
Zero ordinary rows are the identity join.

This is a mass-custody join, not a second LSE energy operation. The accepted
LSE tile ending and native phase/exact-surface receipts already own vapor,
fusion, and sensible-energy effects. The join retains native litter ice,
surface enthalpy high mirrors and exact carry/receipt bytes, phase closure, and
`LitterPhaseCapacitySpillV1` unchanged. The spill remains the separate internal
`LitterPhaseOverflow` ingress parcel and never enters `finalized_uses`.
Ordinary resource debit supplies no new parcel and no second enthalpy, latent,
fusion, or exact-surface operand. Wholesale substitution of
`accepted.surface_resource`, rebuilding from legacy owner bytes, phase-row
replay, spill reclassification, capacity repair, and a second ingress are
forbidden.

The join authenticates one transaction and accepted child support, complete
request/authorization/finalized-use cardinality, exact owner/configuration and
predecessor lineage, row/store keys, source and area basis, finite amount, and
canonical debit reconstruction. Every finalized row is accounted exactly once
as soil, accepted native phase, or ordinary surface resource. Failure of the
partition, debit, native receipt join, ingress, receiver closure, final owner
join, or publication preserves every surface-liquid V1/V2, LSE V3/V16, soil,
WB14 parent, spill, receipt, cursor, and runner beginning byte.

`OBL-LANDSURFACEENERGY-C-012` — Prove a heterogeneous native/ordinary batch,
zero and positive ordinary withdrawals, canonical order independence, exact
ordinary mass closure, native-row exclusion and replay refusal, retained
phase/spill/ice/enthalpy bytes, one resource candidate and ingress, rejection
of omitted/duplicate/foreign/wrong-amount/transaction/support/key rows, and
complete rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Partition finalized surface rows by accepted native phase receipt; apply the complete unmatched ordinary set once to the phase-adjusted V2 owner; execute the existing ingress once. |
| branch/guard | Native phase rows must match and are never replayed; every other row is an authenticated ordinary finalized use. No wholesale resource replacement or untyped remainder is admitted. |
| invariant guard map | `INV-LANDSURFACEENERGY-157` -> typed heterogeneous row partition, V2 resource join, canonical ordinary debit, one-ingress and rollback validators. |
| test vector | `OBL-LANDSURFACEENERGY-C-012`: heterogeneous/identity cases, mass closure, native replay poison, row identity/cardinality poisons, retained receipt bytes, one ingress, rollback. |
| binding exposure | `LSE-V3-HETEROGENEOUS-SURFACE-RESOURCE-JOIN`, active, `maps-to-existing-INV`, IDs `157/C-012`, dual review/verification. |
| change log | 2026-09-02, contract 22: exact once-only ordinary finalized-use debit on the accepted native phase-adjusted V2 owner; unchanged LSE/phase/WB14 equations, tolerances, support, and energy receipts. |

## Topology-Ranked V16 Exact-Surface Owner Amendment

`INV-LANDSURFACEENERGY-158` binds canonical V16 exact-surface record and
accepted-energy operand order to the authenticated configuration topology.
For owner records, compare the rank of `surface_key.ofe_id` in the exact
`SurfaceLiquidConfigurationV2.ofe_topology` first, then use the existing
`DirectSurfaceLiquidStoreKey` order with `ofe_id` held equal. For operands,
compare that owner-record rank first, then the existing operand kind and
ordinal. `OfeId` remains an opaque identity. Its text is never parsed as a
number and never compared lexically to infer topology. Therefore physical
topology `ofe-9 -> ofe-10` is admitted exactly as configured, as are reverse,
nonmonotone, and nonnumeric identifiers.

`LseSurfaceEnthalpyOwnerEnvelopeV1` does not serialize a second topology; it
seals the configuration digest and ordered record bytes. Its context-free
validator therefore proves schema, digest form, unique keys, finite exact
coordinates, state/receipt lineage, and beginning/ending sequence equality,
but does not impose lexical OFE order. The existing authenticated frozen-
parent/configuration join is the canonical-order authority: it validates the
configuration and SurfaceLiquid V2 owner, requires the exact-owner record
sequence to equal their complete topology-ranked sequence, joins LSE V3 keys,
and rejects stale configuration or digest. Construction adopts that sequence;
advance, receipt, restart/checkpoint, projection, and independent replay
preserve it without sorting or rebinding.

No relaxation of cardinality or custody is admitted. Duplicate, omitted,
substituted, foreign, topology-relative reordered, or within-OFE reordered
records; reordered operand groups; LSE/SurfaceLiquid topology disagreement;
stale configuration/digest; or changed beginning/ending sequence rejects with
`LSEB-E-050` / `SURFACELIQUID-E-012`. Failure preserves all exact owner/carry,
high-mirror, physical owner, receipt, restart/checkpoint, WB14, and runner
bytes. No schema field, energy arithmetic, constitutive result, tolerance,
transaction, support, or chronology changes.

`OBL-LANDSURFACEENERGY-C-013` — Prove exact adoption, advance, receipt,
independent replay, restart/checkpoint, and projection for configured topology
`ofe-9 -> ofe-10`; reverse/nonmonotone and nonnumeric opaque identifiers;
multiple within-OFE records and operand kinds/ordinals; and unchanged exact
energy bytes. Poison lexical/numeric-derived reorder, duplicate, omission,
substitution, foreign OFE, within-OFE/operand reorder, stale configuration or
digest, cross-owner topology mismatch, and beginning/ending reorder. Every
failure proves complete rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Validate uniqueness without OFE spelling order; at the authenticated configuration join validate the complete topology-ranked owner sequence and topology-ranked operand groups. |
| branch/guard | Configuration rank is the sole OFE ordering authority. Bare validation cannot promote bytes to installable custody, and lexical/numeric OFE inference is forbidden. |
| invariant guard map | `INV-LANDSURFACEENERGY-158` -> exact-owner adoption/parser, frozen-parent/configuration join, receipt/restart/projection, independent replay, and installer. |
| test vector | `OBL-LANDSURFACEENERGY-C-013`: multi-digit/opaque topology, within-OFE and operand order, duplicate/omission/substitution/reorder/stale-config poisons, rollback. |
| binding exposure | `LSE-V16-TOPOLOGY-RANKED-EXACT-OWNER`, active, `maps-to-existing-INV`, IDs `158/C-013`, dual review/verification. |
| change log | 2026-09-02, contract 23: authenticated configuration-topology rank replaces lexical OFE ordering for V16 exact-owner records and operands; unchanged wire schema, exact arithmetic, physics, and custody. |

## Validated In-Memory LSE Custody Handoff Amendment

`INV-LANDSURFACEENERGY-159` permits a trusted in-process consumer to receive
the exact immutable object whose complete semantic validation has already
succeeded. The authority carrier is private typed state, not serialized
authority. It binds model/schema and authenticated configuration identity, the
complete state or envelope digest, transaction, predecessor and support, and,
for a retained frozen-litter publication history, the exact prefix count,
first receipt, last receipt, and chain digest. It owns or immutably borrows the
value, has no public or unchecked constructor and no mutable dereference, and
cannot be persisted or reconstructed from a digest alone.

Installing an unchanged `FrozenLitterV3Resident` may therefore move this typed
validated resident directly rather than serialize and restore its complete
publication history. An append operation validates the new support and its
predecessor against the bound tail and produces a new validated revision; it
does not revalidate the immutable prefix. Likewise, one fully validated V2
surface-resource candidate may be consumed by its trusted ingress chain
without repeatedly serializing the same three nested owner envelopes. Every
mutation or replacement consumes the proof and requires full semantic
validation of the resulting revision.

An optional canonical byte/digest cache is derived only during validation,
keyed to the exact configuration and immutable revision, must equal fresh
canonical serialization, and is discarded on mutation. It is never wire,
restart, checkpoint, receipt, or publication authority. Restart/checkpoint
restore, external bytes, durable publication, and untrusted executor outputs
still perform the existing full parse, canonical reconstruction, digest and
semantic validation. In particular, the Stage 3 frozen-litter restart chain
retains full prefix replay under `SC-SNOWENERGY-001#INV-SNOWENERGY-083`; no
receipt, history row, owner join, or rollback check is removed.

`OBL-LANDSURFACEENERGY-C-014` — Prove O(1)-with-history direct install of an
unchanged validated resident, exact prefix retention, new-tail-only validation,
and full restart history replay; resource-candidate validation once per
revision and zero duplicate nested-owner serialization at trusted ingress;
mutation/configuration/transaction/support/chain/proof-transfer poison
rejection; byte-identical outputs; and complete rollback. Prove restart,
external/durable bytes, and untrusted executor returns still receive fresh
full validation.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Mint a private handoff only after complete semantic validation; move the unchanged resident/candidate or validate one appended tail; validate every changed revision anew. |
| branch/guard | A cache is derived evidence only. Mutation invalidates it. Restart, checkpoint, external bytes, durable publication, and untrusted executor returns always take the full validator. |
| invariant guard map | `INV-LANDSURFACEENERGY-159` -> validated resident/resource typestate, publication-prefix/tail join, revision-bound cache, mandatory boundary validators, atomic rollback. |
| test vector | `OBL-LANDSURFACEENERGY-C-014`: 1/N-history O(1) install, tail append/poison, restart full replay, resource validation-once, zero repeat serialization, mutation/configuration/proof-transfer poisons, untrusted validation, equality, rollback. |
| binding exposure | `LSE-V24-VALIDATED-IN-MEMORY-HANDOFF`, active, `maps-to-existing-INV`, IDs `159/C-014`, dual review/verification. |
| change log | 2026-09-02, contract 24: admitted private immutable nonserializable validation-once handoffs while retaining complete validation at every restart, external, durable, and untrusted-executor boundary. |

## Snow-Free Final-Receipt Reseal Amendment

`INV-LANDSURFACEENERGY-160` admits no new physical result. The provisional
snow-free V11 execution must first complete all existing validation and retain
its exact immutable LSE, surface, hydrology, BGC, vegetation, and soil ending.
A private move-only proof may then authorize the final accepted-slab pass to
reconstruct and reseal only identities whose preimage contains the accepted
slab receipt. Phase-free evaluation, litter phase, resource arbitration,
current ingress, WB14, soil thermal, and every constitutive or conservation
equation are forbidden in that final reseal.

The proof binds the live beginning revision, parent/segment/slab ordinal,
support and duration, configuration/topology, forcing and support receipt,
beginning complete owners, all non-slab inputs, provisional slab identity, and
the validated physical ending. Consumption requires an independently created
final slab whose only permitted binding difference is its ending-owner-derived
accepted-slab identity. The resealed final complete-owner bytes must equal the
provisional physical ending exactly. The provisional pass publishes nothing;
the final pass publishes exactly once. Missing, reused, stale, foreign,
mutated, or restored proofs fail before owner mutation; restart always executes
fresh physics and cannot serialize or recover the proof.

`OBL-LANDSURFACEENERGY-C-015` requires forced direct-versus-reuse byte equality,
one-call counters for every physical provider and adjacent owner operation,
zero provisional and one final publication, exhaustive non-slab identity and
physical-ending poisons, single-use rejection, pre/post-restart vectors, and
complete rollback.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Fully validate one snow-free physical execution, consume one private proof to reseal final accepted-slab identities, compare exact ending owners, then publish once. |
| branch/guard | The final reseal cannot call any physical evaluator; a missing or invalid proof is a typed failure with no replay fallback. Restart invalidates the proof and requires fresh physical execution. |
| invariant guard map | `INV-LANDSURFACEENERGY-160` -> private physical-reuse typestate, accepted-slab-only delta validator, exact ending-owner comparison, provider/publication counters, restart and rollback gates. |
| test vector | `OBL-LANDSURFACEENERGY-C-015`: direct/reuse equality, physical call count one, publication 0/1, non-slab/ending poisons, duplicate use, restart refusal/fresh evaluation, rollback. |
| binding exposure | `LSE-V25-SNOW-FREE-PHYSICAL-REUSE`, active, `maps-to-existing-INV`, IDs `160/C-015`, dual review/verification. |
| change log | 2026-09-02, contract 25: admitted one private single-use snow-free physical ending for final accepted-slab identity resealing; physical equations, exact owners, publication, restart, and rollback are unchanged. |

## Covered Nonfinal Physical-Only Map Amendment

`INV-LANDSURFACEENERGY-161` — The canonical covered-map evaluator retains its
physical result in a private typed posture appropriate to the preselected
regime. Role and regime dispatch are orthogonal. Every ordinary
posture performs the unchanged projection, covered LSE, snow/soil lower-
boundary, hydrology, surface/WB14-custody, and soil-candidate calculations.
Every native represented-snow posture instead executes the existing native
snow/LSE/soil branch, performs zero snow-free surface or WB14 physics, and
validates byte-retained inactive litter/WB14 custody. Both validate their
complete physical, identity, and discrete-custody relations. `Initial` stops at that validated
endpoint. Every later charged role yields a non-Clone pending adjudication
value only after those validations. Until that value is consumed into the final disposition, it does not
project a V8 receipt, build vegetation
persistent/material candidates, build a BGC candidate or ending joint, or
construct a complete owner envelope. Outer nonclosure consumes the pending
value into iteration history; dependent-only nonclosure consumes it into typed
adaptive rejection. Full closure consumes the same pending value once as the
`FinalAccepted` disposition and continues its own already-executed physical
prefix through those custody constructors, producing a private publishable
envelope but no enqueue, exposure, live-owner install, or publication.

The represented-snow native V3/V4 branch remains distinct from the ordinary
inner envelope: native physical-only evaluation requires both residents,
nonempty Stage-3 native tiles, empty active litter tiles, exact optical and
lower-boundary receipts, and byte-retained inactive litter/WB14 custody. A
physical-only result is private, move-only, non-wire, non-owner, and has no
installation or publication path. Wrong role, ordinal, regime, native posture,
support, transaction, topology, forcing, soil predecessor, surface custody, or
physical endpoint fails typed with exact rollback; duplicate or cross-
disposition consumption and final construction failure cannot retry through a
physical-only, history, or alternate-envelope path.

| Amendment trigger | Typed failure and precedence |
| --- | --- |
| role, ordinal, or charged-map order mismatch | `DirectV11RealConsumerError::AdaptiveRefinement`, before LSE execution |
| outer nonclosure | consume the pending map into history; no error |
| dependent-only nonclosure | consume the pending map into `DirectV11RealConsumerError::AdaptiveRefinement`; no history or constructor |
| support, transaction, topology, forcing, regime, native posture, predecessor, custody, promotion, or state-leak mismatch | `DirectV11RealConsumerError::Identity`, before pending minting or the affected constructor/exposure |
| ordinary/native physical-prefix failure | retain the unchanged specific `LSEB-E-020/040/047/048/049/050` or nested hydrology/surface/soil error |
| V8, vegetation, BGC, joint, serialization, or envelope construction failure | retain its unchanged typed downstream error and construct no complete envelope |
| map-level publication attempt | coupled-time `ERR-CT-018 PublicationState`; expose nothing |

Validation follows table order; downstream LSE error precedence is unchanged.

`OBL-LANDSURFACEENERGY-C-016` — Compare every initial, history, rejected, and
final-disposition physical prefix against a test-only forced-complete path
exactly for ordinary and native represented-snow
regimes, including precipitation, LSE, soil, surface/WB14, Stage-3, branch,
topology, and receipt-custody fields. Prove final-only V8/vegetation/BGC/joint/
envelope constructor counts, absence of those calls for history or rejection
dispositions, one complete final envelope, one-ULP and identity poisons,
unpublishability, and byte-identical rollback. Apply the SnowEnergy charge,
physical-endpoint, exclusive-disposition, physical-failure, dependent-
rejection, and final-constructor failure matrix exactly. Map-level
publication is always zero; the selected composed parent publishes once only
at atomic parent commit.

| Profile surface | Binding |
| --- | --- |
| algorithm step | Execute and validate one physical prefix per charged map; consume each post-initial pending value into history, rejection, or same-map final custody. |
| branch/guard | Typed ordinary/native pending endpoints cannot cross regimes, be consumed twice, or enter owner/restart/publication APIs except through the successful final disposition. |
| invariant guard map | `INV-LANDSURFACEENERGY-161` -> orthogonal role/regime dispatcher, ordinary/native physical-prefix evaluator, native V3/V4 custody-before-pending validator, pending disposition gates, final-only V8/vegetation/BGC/envelope constructors, separate envelope/parent-publication counters, rollback gate; outer nonclosure -> history/no error, dependent-only nonclosure/role -> `AdaptiveRefinement`, identity/regime/custody/disposition/leak -> `Identity`, downstream failures retain existing typed variants. |
| test vector | `OBL-LANDSURFACEENERGY-C-016`: exact differential prefix, ordinary/native regime matrix, exclusive dispositions, success/failure constructor counts, role/identity/ULP poisons, zero map publication, parent-only publication, unpublishability, rollback. |
| binding exposure | `LSE-V27-PENDING-ADJUDICATION`, active, `new-INV`, IDs `161/C-016`, dual review/verification. |
| change log | 2026-09-03, contract 27: the converged pending adjudication map continues from its own physical prefix into complete owner custody; no final physical replay is authorized. |

## Stage-3 Identity-Anchor Jacobian Amendment

`INV-LANDSURFACEENERGY-162` applies only after the immutable covered-column
inputs and the represented-snow lower boundary have passed their complete
same-solve validation. In that regime the ground-temperature residual is
exactly

```text
(T_ground - T_snow) / 1e-9
```

and soil-temperature residual `i` is exactly

```text
(T_i - T_i,beginning) / 1e-9.
```

The represented-snow evaluator uses the boundary snow temperature—not the
ground coordinate—for reciprocal longwave, uses the boundary sensible and
vapor fluxes for the shared-air equations, and performs no ground vapor,
storage, or soil-conduction solve. Consequently each ground or soil coordinate
changes only its matching identity-anchor residual.

For one such Jacobian column, the canonical solver still constructs the exact
current-derived minus trial first and plus trial second using the unchanged
`sqrt(epsilon)*max(abs(x_i),unit_scale_i)` perturbation. It validates the
current and both trials through the existing covered-trial domain rule and
selects the unchanged centered or unique inward stencil. For each admitted
probe it copies the current complete normalized residual vector and replaces
only the matching anchor entry with the exact expression above, evaluated in
the same subtraction-then-division order as the complete evaluator. The
existing finite-difference function consumes those values; an analytic
derivative, sparse solve, changed operation order, or tolerance shortcut is
not authorized.

Every non-anchor coordinate, every ordinary or snow-free regime, every base
evaluation, and every prospective or backtracking evaluation retains the
complete evaluator. Missing or unvalidated Stage-3 boundary authority cannot
select the anchor path. If any residual dependency is introduced or cannot be
proved exactly, the implementation must use the complete evaluator rather
than approximate, cache across mutation, suppress an error, or install a
fallback. Unknown/residual ordering, dense Jacobian layout and bits, pivot
classification, LU, backtracking, convergence, diagnostics, output, receipts,
and first-error precedence remain those of `INV-LANDSURFACEENERGY-108/138`.

`OBL-LANDSURFACEENERGY-C-017` — Compare the optimized probe residual vectors
and resulting dense Jacobian columns with forced complete-evaluator results
bit-for-bit for ground and every soil coordinate, centered and admitted inward
stencils, multiple current iterates, and Stage-3 boundary/soil-anchor poisons.
Prove canonical minus-then-plus trial construction and domain rejection,
complete-evaluator call elimination only for admitted anchor probes, complete
reevaluation for all other coordinates/regimes, unchanged solver outcome and
diagnostics, and authentic runner output, closure, map-count, and publication
parity.

| Profile surface | Binding |
| --- | --- |
| algorithm step | For a validated represented-snow ground/soil column, construct and admit the canonical probes, replace only the matching exact anchor residual in the current complete vector, then use the existing finite-difference operation. |
| branch/guard | Private same-solve Stage-3 proof and exact anchor index are mandatory; all other columns/regimes use the complete evaluator, and any unproved dependency forbids reuse. |
| invariant guard map | `INV-LANDSURFACEENERGY-162` -> private validated boundary proof, ground/soil anchor classifier, exact probe-residual assembler, canonical stencil and dense-Jacobian path. |
| test vector | `OBL-LANDSURFACEENERGY-C-017`: full-evaluator residual/Jacobian bit parity, centered/inward/domain vectors, dependency poisons, evaluation-call counts, authentic runner parity. |
| binding exposure | `LSE-V28-STAGE3-ANCHOR-JACOBIAN`, active, `new-INV`, IDs `162/C-017`, dual review/verification. |
| change log | 2026-09-04, contract 28: exact represented-snow ground/soil identity-anchor probe reuse; unchanged equations, probes, Jacobian, solver, errors, outputs, and custody. |

## Covered Leaf Maximum-Demand Exact-Reuse Amendment

`INV-LANDSURFACEENERGY-163` applies only within one invocation of the covered
occupancy evaluator after its current sun or shade `leaf_trial_state` call has
succeeded. The internal maximum-demand call uses the same leaf inputs,
biochemical constants, temperature, canopy humidity, gas environment,
boundary conductance, minimum conductance, and Medlyn parameter, changing only
beta to exact binary64 `1.0`.

The successful private `LeafTrialState` may be copied as the maximum-demand
state only when either current beta is bit-identical to exact `1.0`, so every
operand is identical, or its returned gas branch is `Inactive` or
`ExactZeroPar`. Those two admitted branches complete before beta participates
in any operation and therefore return the same state for the beta-one call.
`RespirationDominated`, `PositiveAssimilation`, and every unclassified branch
retain the complete beta-one evaluation unless beta itself is exact one.

Current sun and shade calls remain first and keep their existing order. Each
maximum result remains logically ordered sun then shade; a non-reused maximum
executes the unchanged function in that position. A failed current call is
never reused. The proof is the successful private `Copy` state in that same
stack evaluation; it is not a public capability and cannot cross an evaluator,
trial, Newton iteration, mutation, restart, serialization, or publication
boundary. Leaf equations, arithmetic, gas branches, maximum demand, hydraulic
residuals and tolerance, all other normalized residuals, finite differences,
dense Jacobian, LU/backtracking/convergence, results, diagnostics, and typed
first-error precedence remain bit-identical. Approximation, tolerant beta
comparison, additional branch admission, persistent caching, and fallback are
forbidden.

`OBL-LANDSURFACEENERGY-C-018` requires a forced-exhaustive test oracle that
always performs the beta-one calls and compares every private leaf-state field,
complete covered evaluation and normalized residual, frozen branch, and full
solve result bit-for-bit. It covers exact beta one, inactive,
exact-zero-PAR, positive-PAR beta one ULP below one, centered and inward beta
probes, success and typed-error precedence, exact call counts, and authentic
release output, closure, map-count, and publication parity.

| Profile surface | Binding |
| --- | --- |
| algorithm step | After a successful current leaf call, copy its private state for the adjacent beta-one maximum only for exact beta one or an admitted beta-independent branch; otherwise execute the complete maximum call. |
| branch/guard | Exact `to_bits` beta predicate or returned `Inactive`/`ExactZeroPar` branch is mandatory; all other cases perform the existing call, with no cache or fallback. |
| invariant guard map | `INV-LANDSURFACEENERGY-163` -> private same-evaluation leaf state, exact classifier, exhaustive-call oracle, invocation audit. |
| test vector | `OBL-LANDSURFACEENERGY-C-018`: every-field and complete-evaluation/solve bit parity, exact-beta and branch matrix, centered/inward probes, call order/count, typed-error precedence, authentic runner parity. |
| binding exposure | `LSE-V29-LEAF-MAXIMUM-EXACT-REUSE`, active, `new-INV`, IDs `163/C-018`, dual review/verification. |
| change log | 2026-09-04, contract 29: exact same-evaluation reuse of already successful bit-identical or beta-independent leaf states for internal beta-one maximum demand; unchanged equations, solver, outputs, errors, and custody. |

## Carrier Parent-Static and Same-Map Validation-Once Amendment

This version extends the already admitted private validation-once custody of
`INV-LANDSURFACEENERGY-159`; it creates no new invariant and no solver version.
Within one already admitted terminal parent, one private non-Clone, non-wire,
generation-bound structural plan may retain only successful semantic
validation and deterministic indexes for immutable LSE and surface
configuration plus authenticated OFE/tile/occupancy topology. It never attests
to structural V8 state or to the distinct native resident's V3 LSE
configuration/state or V2 surface configuration/owner. It owns no mutable
runtime state, cannot be constructed from digests alone, and is absent from
restart, checkpoint, serialization, receipt, publication, and external APIs.

Plan construction is lazy at the first structural validation that an admitted
charged map reaches. If the parent has no charged map, no plan is minted and no
new validation occurs. Each plan join occurs at the exact configuration,
topology, or index check it replaces, after every existing carrier guard that
precedes that check; no plan join is hoisted ahead of support, duration,
transaction, joint, or forcing errors. On first use, the canonical full
validation at that position executes in its unchanged order. Failure returns
the same first typed error and leaves the parent and every owner byte-identical.
A successful plan binds the exact live parent generation, configuration,
topology, and index-source objects. Later maps may omit only those immutable
checks while every binding remains exact. Generation change, replacement,
equal-digest substitution, mutation, or transfer rejects at the original check
position without reconstructing a plan or falling back.

Every charged map retains the source-real order:

1. Run every existing carrier child/joint, support, duration, transaction,
   vegetation, receipt, boundary, prepared-input, and soil-read guard in its
   current position. Join the structural plan only when the first replaceable
   immutable structural check is reached.
2. At the existing forcing-validation position before V8, validate and
   canonically normalize that map's exact forcing. That first validation may
   mint a private move-only map proof bound to the live forcing allocation,
   transaction, support, generation, complete semantic digest, and normalized
   values. Equal digest with a different allocation is not authority.
3. Execute V8 projection in its current position. It freshly validates all
   current-map structural state and dynamic vegetation, LSE, surface, BGC,
   soil, hydrology, lower-boundary, and join surfaces. At V8's later validation
   of the pointer-identical forcing, consume the forcing proof instead of
   repeating only that validation. V8 neither receives nor attests to the
   distinct native resident's V3 LSE or V2 surface objects.
4. Derive the ingress schedule in its current fallible position after V8 and
   before native projection.
5. Only in a native regime, at the existing native-validation position, join
   the exact `FrozenLitterV3Resident` to its private
   `ValidatedFrozenLitterV3ResidentRevisionV1`. The revision must still match
   the resident's complete validated configuration/state/envelope digest,
   topology, transaction/predecessor/support, publication-prefix count/head/
   tail/chain, and exact V3-LSE/V2-surface references. That successful join may
   mint one borrowed, pointer-, revision-, parent-generation-, and map-bound
   proof, consumed immediately to omit only the repeated
   `lse_beginning.validate(lse_configuration)` and
   `surface_beginning.canonical_bytes(surface_configuration)` calls.
6. Continue every remaining native solver-ready, topology, rebinding,
   lower-boundary, residual, solver, output, and owner validation and every
   physical operation exactly as before. Ordinary maps mint and consume no
   resident proof.
7. Consume the final map through existing finalization and atomic parent
   commit. Rejected, history, or failed maps expose no plan/proof and mutate no
   owner. Restart discards ephemeral authority and reconstructs a fully
   validated resident revision through the canonical restore path.

The role order remains Initial, zero or more history candidates, and one final
candidate; each adaptive attempt retains direct before composed, and composed
retains Half1 before Half2 with Half2 beginning from the authenticated Half1
ending. Lazy plan creation, plan joins, forcing proof consumption, and resident
revision joins occur only at the checks they replace. Thus support, duration,
transaction, joint, forcing, V8, ingress-schedule, native-resident, subsequent
dynamic/solver, and output failures retain their present relative order, and
only the first error is returned. A stale plan paired with an earlier support,
duration, transaction, or joint poison returns that earlier error; a native
resident poison paired with an ingress-schedule poison returns the ingress
error. Malformed restart input still fails at its existing boundary.

The plan or proof must not contain or cache a
`ValidatedV8RuntimeInputProjection`, projected column, solver-ready tile,
hydrology snapshot, physical result, or dynamic owner candidate. It must not
use `Arc<DirectV10...>` or another shared owning handle to extend the lifetime
of a dynamic or complete DirectV10 input. A canonical digest may accompany
pointer/generation identity as evidence but can never independently admit an
object. The persistent resident revision is private validated custody and may
remain with an unchanged resident across maps; it is not the ephemeral map
proof. Its existing `Clone` implementation is authorized only as an inseparable
private clone of the exact whole immutable resident and never as independently
transferable admission. Every accepted resident successor is fully validated
before its revision advances atomically, and the resident may not mutate while
a borrowed proof exists. The plan, forcing proof, and resident map proof have no `Clone`,
serde, wire, public or unchecked constructor, cross-map/cross-parent transfer,
persistence, or restart restore. Second consumption and transfer are rejected;
alternate solver selection and silent full-validation fallback are prohibited.

`OBL-LANDSURFACEENERGY-C-019` requires an executable forced-full-validation
oracle against the admitted path. On the retained authentic terminal-parent
workload whose carrier performs 52 maps, audit evidence must report exactly one
parent-static validation, 52 exact normalized-forcing validations, and 52 fresh
dynamic-map validations. For each applicable regime independently, the oracle
must enumerate and compare every required Initial/history/final and
direct/Half1/Half2 role/path, with byte-for-byte physical and final-owner parity
and exact call order. Native and native-multilane maps exercise the real native
consumer; ordinary maps prove zero resident-proof mint/consume and zero native
physical execution.

Independent poisons distinguish structural versus native LSE configuration
and state, structural versus native surface configuration and owner,
generation, topology, index, support, duration, transaction, joint, forcing
pointer, same digest/different allocation, ingress schedule, resident revision,
proof second-use, cross-map, cross-parent, restart restoration, dynamic
vegetation/surface/soil-hydrology state, native solver/residual, and output
validation. Competing-poison vectors cross each ordered boundary through
dynamic validation, solver/residual, and output validation and require the same
first typed error on full and admitted paths. Every rejection has zero fallback and
publication plus byte-exact rollback. Counters and order records must originate
at the real carrier, first forcing validator, V8 projection, ingress scheduler,
resident-revision join, native V3 consumer, dynamic validators, and final owner;
fabricated outcomes, manually incremented fixture counters, or source scanning
alone cannot satisfy the obligation.

This is validation/custody architecture only. It adds no dimensional symbol,
conversion, scalar exception, constant, empirical parameter, tolerance,
equation, physical branch, solver, residual, output, publication field, or wire
format. All existing units, aliases, numeric guards, closure thresholds,
calibration posture, and constitutive-suite obligations remain unchanged.
Calibration and identifiability are therefore `CALIBRATION_NOT_APPLICABLE` for
this amendment; the contract-level fields remain unchanged.

| Profile surface | Binding |
| --- | --- |
| state surface | Private non-Clone/non-wire parent structural plan, per-map exact-forcing proof, existing resident validated revision, and borrowed non-Clone resident map proof; no cached dynamic state, result, owner candidate, restart, or publication representation. |
| algorithm step | Retain existing early carrier guards; join the lazy structural plan only at each replaced immutable check; validate forcing once before V8 and consume its proof at V8's duplicate forcing check; run V8 and fallible ingress; then join the exact resident revision and consume its proof only for the two repeated native V3/V2 validations. |
| branch/guard | Exact pointer, revision, parent generation, map, transaction, support, configuration, topology, index, and semantic identity are mandatory as applicable. Changed, reused, or transferred authority rejects at its original validation position with no fallback; restart and every trust boundary perform canonical full validation. |
| invariant guard map | `INV-LANDSURFACEENERGY-159` -> parent-static plan, exact-forcing proof, resident-revision-sourced native proof, authentic call-site audit, forced-full oracle, paired poison/error-order matrix, and rollback gate; `INV-LANDSURFACEENERGY-161` and `SC-COUPLEDTIME-001#INV-COUPLEDTIME-030` retain role/disposition custody. |
| alias/unit/constant/tolerance | No new aliases, dimensional values, conversions, constants, parameters, tolerances, or numeric normalization. Existing contract tables remain authoritative. |
| calibration | `CALIBRATION_NOT_APPLICABLE`: no parameter, observation, objective, calibration evidence, or identifiability claim changes. |
| test vector | `OBL-LANDSURFACEENERGY-C-019`: authentic 1/52/52 audit, per-applicable-regime bitwise role/path parity, ordinary zero-native proof, exact order, structural/native identity and proof-custody poisons, paired precedence, no cache/Arc/wire/fallback surface, and byte-exact rollback. |
| binding exposure | `LSE-V30-CARRIER-PARENT-STATIC-VALIDATION-ONCE`, active, `maps-to-existing-INV`, IDs `159/C-019`, dual review/verification. |
| change log | 2026-09-04, contract 30: admitted parent-static, source-ordered forcing, and resident-revision-sourced native validation-once custody only; structural V8 and resident V3/V2 objects remain distinct; no process physics, solver, tolerance, output, publication, or wire change. |

## Component-Temperature Jacobian Dependency-Replay Amendment

`INV-LANDSURFACEENERGY-164` applies only inside one already validated
represented-snow covered solve and one current Jacobian sweep. It changes no
coordinate, residual, constitutive equation, physical branch, or derivative.
The canonical coordinate order remains ten coordinates per occupancy (four
hydraulic potentials, two beta values, then sun-leaf, shade-leaf, wet-surface,
and dry-stem temperatures), followed by shared canopy-air temperature and
humidity, ground temperature, and the configured soil temperatures. Canonical
probe perturbations, minus-before-plus construction, bounds admission, and
centered or unique inward finite differences remain unchanged.

One immutable `ValidatedCoveredComponentReplaySweepBase` owns the successful
current evaluation for one Jacobian sweep. It binds the exact validated-input
generation and every referenced input, the exact potential or fixed-final caps
values and posture, frozen-branch values, current-trial bits, authentic map,
solve, Newton-iteration, and sweep identities, and the graph version/hash. It
may be borrowed by all signed probes in that sweep but is dropped before the
next iteration, solve, map, or retry. For each admitted minus or plus probe the
canonical probe constructor mints a fresh non-Clone
`ValidatedCoveredComponentProbeReplay` bound to that base, coordinate index,
sign, perturbation bits, exact probe bits, and the actually selected stencil.
The constructor, not a later whole-vector scan, proves the sole-coordinate
difference and sign/stencil/bound relation. That capability is consumed exactly
once by success or error and cannot be transferred, restored, serialized, or
reused. Dropping either object publishes and mutates nothing.

Custody is exact, not a costly proxy check. Production may use immutable borrows,
typed generations and compact canonical seals minted where the corresponding
object is already fully validated, but it must compare every bound identity and
value bit exactly. A `Debug` string, digest length, allocation-independent hash,
reconstructed approximation, whole-probe clone, or repeated full-vector scan is
not custody. A seal must include the caps, frozen branches, graph, trial,
coordinate, sign, perturbation, probe and stencil facts above and must be joined
to the same successful base evaluation. The static graph descriptor is built or
validated once for its authenticated topology/configuration generation and then
borrowed; production must not rebuild strings, ordered maps, transitive closure,
or a whole-input hash in every sweep or signed probe. This bounded descriptor is
not a cross-sweep result cache: no base evaluation, probe, branch result,
residual, solver result, or mutable map state survives its named boundary.

`CoveredComponentTemperatureDependencyGraph` uses schema version
`covered-component-temperature-dependency-v1`. Stable node IDs and direct-edge
families are normative below; `[o]` is authenticated occupancy topology rank,
`[k]` is a component in canonical `sun,shade,wet,stem` order, `[s]` is soil
rank, and `[j>o]` means every lower occupancy. Expanded node and edge records are
sorted lexically, duplicate-free, length-prefixed, and SHA-256 hashed with the
schema version, `N`, and `S` by the contract oracle. A production
representation may be a compact equivalent whose exact edge-set/version/
topology join has been established outside the probe hot path. Inclusive
transitive closure is computed from those exact directed edges in stable node
order. The sweep base and probe capability carry and compare the same
version/hash or an equally exact typed graph identity. An unrecognized
evaluator node/read or missing edge marks the affected coordinate conservatively
ineligible; unknown never proves independence.

| Stable node family | Source-order computation and owned values |
|---|---|
| `probe[o,k]` | sole signed component-temperature coordinate change |
| `route.prepare[o]` | top-to-bottom preliminary liquid preparation from exact incident rain |
| `route.wet[o]` | first/source-distinct wet-flux evaluation from `trial[o,wet]` |
| `route.finalize[o]` | first liquid finalization; throughfall, both drainages, stemflow |
| `route.incident[o+1]`, `route.stemflow[o+1]` | ordered lower-occupancy rain and stemflow-prefix custody |
| `longwave.layer[o]`, `longwave.column` | all areas/temperatures, reciprocal component nets and ground-facing net |
| `occ.leaf.current[o,sun|shade]` | current surface humidity, resistance, CI/carbon and gas branch |
| `occ.leaf.maximum[o,sun|shade]` | beta-one maximum state in exact current-before-maximum order |
| `occ.vapor[o,sun|shade]` | current/maximum vapor rates and conductances |
| `occ.wet[o]` | second/source-distinct wet-flux evaluation during occupancy evaluation |
| `occ.hydraulic[o]` | source/root loop, six hydraulic residuals and tolerances |
| `occ.sensible[o,k]`, `occ.energy[o,k]`, `occ.tolerance[o,k]` | component sensible, energy/anchor residual and exact tolerance arithmetic |
| `occ.liquid[o]`, `occ.route_match[o]` | second liquid finalization and equality with `route.finalize[o]` |
| `occ.output[o]` | all occupancy fields including CI/carbon, branches, water and component arrays |
| `shared.heat`, `shared.vapor`, `shared.tolerance` | occupancy-order reductions, lower boundary and reference air |
| `lower.ground_output` | ground-facing longwave plus represented-snow ground/soil outputs |
| `residual.raw`, `residual.tolerance`, `residual.normalized` | canonical occupancy/shared/ground/soil rows and normalization division |
| `result.ground_release`, `result.ground_stemflow`, `result.output` | terminal incident rain, stemflow sum and complete evaluation fields |

The complete direct-edge generator is normative:

| From | Required direct targets |
|---|---|
| `probe[o,k]` | `longwave.layer[o]` and the matching component families below |
| `probe[o,sun|shade]` | matching `occ.leaf.current`, `occ.leaf.maximum`, `occ.vapor`, `occ.hydraulic`, `occ.sensible`, `occ.energy`, `occ.tolerance`, `occ.output` |
| `probe[o,wet]` | `route.wet[o]`, `route.finalize[o]`, `occ.wet[o]`, matching sensible/energy/tolerance, `occ.liquid[o]`, `occ.output[o]` |
| `probe[o,stem]` | matching sensible/energy/tolerance and `occ.output[o]` |
| `route.incident[o]` | `route.prepare[o]`; `route.stemflow[o]` and `route.finalize[o]` feed `route.stemflow[o+1]` |
| `route.prepare[o]` | `route.wet/finalize[o]`, `longwave.layer[o]`, `occ.wet[o]`, and every occupancy vapor/hydraulic/sensible/energy/tolerance/liquid/output node that reads area, wet fraction, store or branch |
| `route.wet[o]` | `route.finalize[o]`; finalization feeds `route.incident/stemflow[o+1]`, `occ.route_match[o]`, and at terminal `o`, both ground-release/stemflow results |
| `route.incident[o+1]` | the same routing chain for every `j>o` by closure; no adjacent-only truncation |
| `longwave.layer[o]` | `longwave.column`; the column feeds all component energy/tolerance nodes, every `occ.output[o]`, `lower.ground_output`, and `result.output` |
| each `occ.leaf.current` | matching maximum, vapor, hydraulic, energy/tolerance and output nodes |
| each `occ.leaf.maximum` | matching vapor, hydraulic and output nodes |
| each `occ.vapor` | matching hydraulic, energy/tolerance and output plus `shared.vapor` |
| `occ.wet[o]` | hydraulic, wet energy/tolerance, liquid and output plus `shared.vapor` |
| each `occ.sensible[o,k]` | matching energy/tolerance and output plus `shared.heat` |
| `occ.hydraulic/energy/tolerance/liquid[o]` | `occ.output[o]`; both liquid finalizations feed `occ.route_match[o]` |
| `occ.route_match[o]` | `occ.output[o]`; mismatch returns here before later occupancies/shared work |
| each `occ.output[o]` | ordered raw/tolerance rows, shared reductions and final output |
| `lower.ground_output` | `shared.heat`, `shared.vapor`, `shared.tolerance`, matching raw/tolerance rows and `result.output` |
| `shared.heat` | `shared.tolerance`, matching raw/tolerance rows and `result.output` |
| `shared.vapor` | `shared.tolerance`, matching raw/tolerance rows and `result.output` |
| `shared.tolerance` | matching tolerance/normalized residual rows and `result.output` |
| `result.ground_release`, `result.ground_stemflow` | `result.output` |
| raw/tolerance rows | matching normalized row; every residual node feeds final output |

Additional conservative edges may cause more replay; omitting any edge above is
forbidden. An independent graph oracle expands the exact normative node and
direct-edge records for at least `N=1,S=1` and the real `N=2,S=6` topology,
compares every record and the golden schema hash, and proves that removing or
changing any required edge fails. Any additional conservative edge is explicit,
versioned, and present in that comparison; reachability-only tests do not close
direct-edge completeness.

The complete evaluator and replay walker call one shared canonical
implementation for every common node and evaluator tail. Factoring may expose
typed intermediate values but must not duplicate, mirror, translate, or reorder
any physical, tolerance, residual, branch, or output arithmetic. The replay
walker preserves exact source order: trial admission;
top-rain guard; top-to-bottom `route.prepare -> route.wet -> route.finalize ->
incident/stemflow`; reciprocal longwave; then per occupancy sun current, shade
current, sun maximum, shade maximum, vapor, second wet, hydraulic/root,
sensible, energy/tolerance, second liquid finalization, route-match, output;
lower/ground work; shared heat then vapor and tolerances; ground/soil rows;
normalization; result assembly. Every reachable node executes its existing
expression and operation order; only unreachable successful base nodes copy.

### Eligibility, integrity, mismatch, and error outcomes

| Trigger at its source position | Outcome |
|---|---|
| non-Stage-3, non-component coordinate, inadmissible/multi-coordinate probe, or coordinate disabled for an unknown edge | Select the canonical complete evaluator before capability creation; this is ordinary selection, not error recovery. |
| recognized component probe with every base/graph/probe join exact | Mint one per-probe capability and begin replay. |
| graph version/hash or topology mismatch found before capability creation, with identical complete-evaluator operands | Select the complete evaluator before replay and preserve its first error. |
| stale/foreign base, transfer, wrong coordinate/sign/probe binding, or second consumption | Reject directly with `LandSurfaceEnergyError::ConstitutiveDomain("covered_component_dependency_replay_integrity")`; no complete evaluation and no mutation. |
| fallible reachable node or route-match fails after replay begins | Return that existing error at its source-real position; never run the complete evaluator or another solver afterward. |
| replay succeeds | Assemble the existing result once; consume/drop the probe capability with zero publication. |

### Normative fallibility and canonical-crossability matrix

Here, *canonically crossable* means that a source-real input can produce a
successful immutable base evaluation and the unchanged canonical infinitesimal
one-coordinate component-temperature probe can then reach that existing typed
error surface. A test-only mutation, fault injector, alternate tolerance,
noncanonical perturbation, impossible branch, forged intermediate, or direct
private-node call does not establish crossability.

| Node/error family | Classification for a successful represented-snow base plus an admitted canonical component probe | Required assurance |
|---|---|---|
| `occ.leaf.current[o,sun|shade]` existing leaf-domain errors | fallible and canonically crossable | For every authentic crossable leaf error and both applicable component/occupancy positions, run the same source-real base and signed probe through replay and forced-complete modes; require the same first typed error at the same source-order position, no later node/complete fallback, and byte-exact beginning/custody rollback. |
| `occ.leaf.maximum[o,sun|shade]` existing leaf-domain errors | fallible but not currently established crossable | A successful current-leaf base, unchanged beta-one maximum operands/branch, and admitted component probe must imply maximum-call validity. Require authentic exact-beta/branch boundary successes and exact leaf/evaluation fields. Reclassify as crossable and add the paired source-real error vector only if an authentic successful-base plus admitted-probe counterexample is first established. |
| `route.prepare[o]`, `route.wet[o]`, `route.finalize[o]`, `occ.wet[o]`, `occ.liquid[o]` | fallible but noncrossable from an admitted replay | Prove from the successful base, immutable rain/store/area/caps/frozen inputs, admitted temperature bounds and unchanged branch that every existing domain precondition remains true. Exercise authentic zero/wet/dry, exact-capacity and routing boundary successes and compare all liquid, wet-flux, drainage, throughfall and stemflow fields exactly. |
| `longwave.column` | fallible but noncrossable from an admitted replay | Prove successful-base finite areas/temperatures plus admitted finite bounded component temperature imply every longwave operation remains valid. Exercise authentic reciprocal multi-occupancy/zero-area/boundary successes and compare every component and ground-facing net bit exactly. |
| `occ.hydraulic[o]` | fallible but noncrossable from an admitted replay | Prove the successful leaf/wet predecessors, immutable root/soil/caps/frozen inputs and admitted temperature bounds preserve the hydraulic/root-loop domain. Exercise authentic active/inactive, limiting-root and tolerance-boundary successes and compare all six residual/tolerance fields exactly. |
| `occ.route_match[o]` | fallible consistency guard but noncrossable from correct replay | Prove both calls consume the same immutable routing inputs and one shared canonical liquid-finalization implementation, so equality follows by construction. Exercise authentic upper-to-every-lower routing, zero/nonzero wet drainage, and terminal routes; compare both finalizations and the match fields exactly. A forged mismatch is an integrity test, not a physical poison. |
| `lower.ground_output` existing under-canopy resistance/domain errors | fallible but noncrossable from an admitted replay | Prove the successful base and unchanged lower-boundary/caps/frozen operands retain domain validity; exercise authentic represented-snow resistance and ground/soil boundary successes and compare every lower/ground/soil evaluation field exactly. |
| replay trial-shape, top-rain, coordinate/bound admission, graph/topology and capability-integrity guards | pre-admission or private integrity, not a reachable physical node error | Use separate source-real malformed/boundary selection vectors for the unchanged complete-evaluator guards and real constructor/lifetime operations across authentic generations, bases and probes for private stale/foreign/wrong-coordinate/wrong-sign/wrong-perturbation/wrong-probe/wrong-stencil/second-use vectors. Never mutate private fields to create them. Require exact first error, no replay/fallback, and byte-exact rollback. |
| `probe[o,k]`; `route.incident[o+1]`; `route.stemflow[o+1]`; `longwave.layer[o]`; `occ.vapor[o,sun|shade]`; `occ.sensible[o,k]`; `occ.energy[o,k]`; `occ.tolerance[o,k]`; `occ.output[o]`; `shared.heat`; `shared.vapor`; `shared.tolerance`; `residual.raw`; `residual.tolerance`; `residual.normalized`; `result.ground_release`; `result.ground_stemflow`; `result.output` | infallible computations or assembly under their already validated predecessors | Never invent an error. Execute or copy only as the graph authorizes, in exact source order, and compare every node value, branch, residual/tolerance, terminal route and complete evaluation/output field bit-for-bit against forced complete evaluation. |

For every fallible-but-noncrossable row, the obligation is both a reviewable
successful-base-plus-admitted-probe implication proof over each named existing
guard and executable authentic boundary/branch success vectors with exact field
parity. A generic assertion that the node was successful once is insufficient.
The differential corpus is also a catch-all: whenever any unmodified canonical
input in the branch/bound corpus naturally makes replay or forced-complete
evaluation return an error, both modes must return the identical first existing
typed error and leave beginning/custody bytes identical. Production and tests
must not add mutation seams, fault-injection hooks, synthetic error branches, or
test-only physics entry points to manufacture an otherwise unreachable error.

Every hydraulic-potential, beta, shared-canopy-air, non-Stage-3, malformed,
multi-coordinate, and unproved component probe executes the unchanged complete
evaluator. Existing `INV-LANDSURFACEENERGY-162` synthesis remains exclusive to
represented-snow ground and soil identity anchors. No analytic or automatic
derivative, graph coloring, simultaneous perturbation, sparse Jacobian or LU,
changed pivoting, approximate reuse, cross-sweep/iteration/map/retry cache,
memoization, fallback, hardcoded two-occupancy/six-soil logic, or alternative
solver path is authorized.

The contract-first structural expected-red has one deliberately narrow claim:
it classifies whether the seven named replay graph/evidence/audit/function
declarations exist as unconditional top-level Rust items. It cannot establish
dispatcher invocation, control-flow reachability, graph/evidence consumption,
counter provenance, or any numerical behavior. Empty, skeleton, token-only,
dead-code, or disconnected declarations may turn that source classifier green
but are insufficient for implementation readiness and cannot satisfy
`OBL-LANDSURFACEENERGY-C-020`. Only post-implementation executable tests that
exercise the real dispatcher, observe authentic sealed sweep/run counters, and
pass the forced-complete node/residual/Jacobian/full-solve oracle establish
connectivity, consumption, and behavior.

For `N` occupancies and `S` soil nodes, one full interior centered sweep retains
`2*(10*N+3+S)` ordered logical probes. Exactly `2*(1+S)` represented-snow
ground/soil probes use existing identity-anchor synthesis, `8*N` component-
temperature probes use dependency replay, and `12*N+4` probes use the complete
evaluator. Thus the real `N=2`, `S=6` fixture must report `58 = 14 + 16 + 28`.
Its eight hydraulic, four beta, and two shared-canopy-air columns are the 14
columns whose 28 probes remain complete. This is one named
`N=2,S=6,fully-centered-interior` sweep observation, never a release-run total.

Each sweep resets a local audit before its first column and seals it as
`Completed` only after every required column/probe finishes, or `Failed` at the
source-real first typed error with counts limited to work actually attempted.
`ShortCircuited` exists and is reported only if the unchanged canonical solver
has a real non-error path that ends an already-started sweep before all probes;
if no such path exists, the variant, counter and claimed population are absent,
not fabricated as an always-zero state. `RejectedBeforeProbe` is a per-column
stencil/admission outcome and is not a sweep short-circuit. The record binds
`Potential|FixedFinal`, `N`, `S`, every column's
`Centered|InwardLower|InwardUpper|RejectedBeforeProbe` stencil, admitted signed
logical-probe count, and disjoint anchor/replay/complete buckets. Centered
columns count two signs, inward columns only their admitted sign, and rejection
counts only probes actually attempted. Every record must satisfy
`logical=anchor+replay+complete`.

Map, solve, Newton-iteration, and sweep identities are independent authentic
identities from the real caller/lifecycle: a map ID joins all of that map's
potential/fixed-final solves; a solve ID identifies one actual solve within the
map; iteration and sweep IDs identify their actual nested events. One ordinal
copied into another field, a locally invented release label, or an address/hash
proxy is forbidden. If the library seam lacks map context, the audited path
receives a private typed map identity from the authenticated caller rather than
guessing it. Ordinary production with audit collection disabled need not
allocate records or construct histograms.

A separately reset release aggregator retains every sealed source-real sweep
record without coalescing potential/final, centered/inward, completed/failed, or
any genuinely reachable short-circuit identity. It reports the actually
supported disjoint lifecycle class counts, a per-sweep histogram, and aggregate
bucket sums; it must not require or emit a lifecycle state the canonical solver
cannot enter. Each solve/map aggregate reconciles exactly to its retained sweep
records, including failed and inward-bound sweeps, and reset boundaries forbid
cross-run accumulation. Release acceptance requires complete aggregation and at
least one authentic named fully centered `N=2,S=6` completed record with
`58/14/16/28`; the whole release aggregate does not equal those fixture values.

`OBL-LANDSURFACEENERGY-C-020` binds a test-only mode that forces the complete
evaluator for those same component probes. For potential and fixed-final
solves, the admitted and forced paths compare every node/evaluation field,
raw/tolerance/normalized residual bit, complete dense Jacobian, pivot and matrix
norm, branch, iteration/backtracking trajectory, diagnostic, accepted owner and
typed error bit-for-bit. It proves both modes consume one shared canonical
evaluator node/tail implementation, and an independently enumerated oracle
compares the complete normative direct-edge graph rather than only selected
reachability. Exact custody vectors cover input generations and fields, caps,
frozen branches, graph, trial, coordinate, sign, perturbation, probe and stencil
without `Debug`/length/hash-only or repeated whole-input/probe proxy checks.

The normative fallibility/crossability matrix governs error evidence. Every
canonically crossable typed-error surface receives a source-real paired
replay-versus-forced-complete first-error and rollback vector; currently that is
`occ.leaf.current`, while `occ.leaf.maximum` joins this class only after an
authentic counterexample proves crossability. Every fallible-but-noncrossable
family receives the stated successful-base implication proof plus authentic
boundary/branch success and exact-field parity. Infallible families receive
source-ordered value/evaluation-field parity and never a synthetic error.
Separate real pre-admission and private-integrity vectors cover trial shape,
bounds, graph/topology identity, stale/foreign evidence, wrong bindings and
second use. The unmodified differential corpus catches any naturally occurring
error in either mode and requires the same first typed error, no post-replay
complete evaluation or alternate solver fallback, and byte-exact beginning/
custody rollback. Mutation and fault-injection hooks are forbidden.

Production retention uses this exact command for baseline and candidate:

```text
timeout 1800 taskset -c 0 env RUST_MIN_STACK=67108864 CARGO_PROFILE_RELEASE_LTO=false nix develop -c cargo test --release -p openwepp-runner --lib hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile -- --ignored --exact --nocapture --test-threads=1
```

The frozen pre-v31 Rust source manifest is
`78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`
before and after all three runs. The unchanged binary is
`/workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/release/deps/openwepp_runner-fc552493dc3c6cc2`,
SHA-256 `9a91c82f1799382014c3a561e79130b5f5b665bef0667a4bdff613c91d8e573f`.
From each `STAGE3_LANED_RELEASE_PROBE` JSON record, ordered
`(run_wall_us, physical_phase_wall_us.potential, rss_kib)` values are
`(4926758,354838,70696)`, `(4903570,353374,54624)`, and
`(4896095,353431,59364)`. Sort each timing field independently and select the
middle integer: baseline medians are `run_wall_us=4903570` and
`physical_phase_wall_us.potential=353431`.

Build the candidate once; record one unchanged candidate source-manifest hash
before/after three repetitions and one unchanged candidate binary path/hash;
then run the identical command/environment three times. The same sorted-middle
median requires candidate `run_wall_us <= 4803570` and
`physical_phase_wall_us.potential <= 253431`, at least `100000 us` improvement
in both exact JSON fields. Every candidate run must retain
source `0.8488061229561478`, outlet `0.8471105124736579`, storage
`0.0016956104824910018`, clamp `0`, exact `48/56/20/32/4` workload counts, the
complete run-level sweep aggregation with a qualifying `58/14/16/28` named
sweep, full-solve bit equality, and JSON `rss_kib <= 65536`. If either median
ceiling fails, any identity differs, aggregation is incomplete, candidate
source/binary identity changes, or any RSS exceeds the bound, fully revert the
revision-31 production increment; retain no partial replay, cache, or fallback.

This amendment introduces no dimensional symbol, unit, conversion, constant,
parameter, tolerance, equation, output, publication field, or wire format.
Calibration and identifiability remain `CALIBRATION_NOT_APPLICABLE`.

| Profile surface | Binding |
| --- | --- |
| state surface | One immutable sweep base plus fresh non-Clone signed-probe capabilities, exact generation/input/caps/frozen/graph/trial/map/solve/iteration/sweep/coordinate/sign/perturbation/probe/stencil custody without costly proxy checks, and no persisted result or cross-boundary cache. |
| algorithm step | Construct/admit canonical probes unchanged; for one component-temperature coordinate replay reachable nodes through the one shared canonical evaluator node/tail implementation in complete-evaluator order and reuse only proven-unreachable successful nodes; assemble the existing ordered residual vector and finite difference unchanged. |
| branch/guard | Exact one-coordinate difference and every custody/hash join are mandatory; ordinary ineligibility chooses complete evaluation before replay, integrity violation fails typed, and any post-start error returns directly without fallback. |
| invariant guard map | `INV-LANDSURFACEENERGY-164` -> private graph/evidence types, canonical replay walker, forced-complete oracle, exact direct-edge graph tests, normative fallibility/crossability matrix, exact bucket counters, source-real error/rollback corpus, authentic release gate. |
| alias/unit/constant/tolerance | No new dimensional values, aliases, conversions, constants, parameters, tolerances, numerical normalization, or derivative rule. |
| calibration | `CALIBRATION_NOT_APPLICABLE`: no parameter, observation, objective, empirical evidence, or identifiability claim changes. |
| test vector | `OBL-LANDSURFACEENERGY-C-020`: exact forced-complete node/residual/Jacobian/full-solve parity; complete direct-edge oracle; source-real crossable first-error/rollback vectors; noncrossable implication and authentic boundary-success vectors; infallible exact-field parity; lifetime/integrity/custody; real two-occupancy/six-soil reciprocal-longwave, duplicated wet-routing and terminal-descendant fixture; truthful scoped/aggregate counters; exact release gate. |
| binding exposure | `LSE-V31-COMPONENT-TEMPERATURE-DEPENDENCY-REPLAY`, active, `maps-to-existing-INV`; revision 31 introduced new IDs `164/C-020`; dual review/verification required. |
| change log | 2026-09-04, contract 31: corrected contract-first feasibility after full production revert; exact same-sweep component-temperature dependency replay only, one shared canonical evaluator implementation, source-real error/rollback obligations, unchanged dense solver, trajectory and outputs. |
