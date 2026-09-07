Static: fresh scientific reading of current candidate SC-LANDSURFACEENERGY-001 v32. This is the first completed answer; no implementation, executable verification, calibration, adoption, promotion, or terminal package closure is claimed. No science tests were run.

**Scope and reading decision.** I followed the entry's Surface/soil rules route, its complete shared interface, the surface-energy dependency table, and recursively applicable soil, water, support, solver, exact-energy and custody sections. The question's unspecified litter phase required expansion to the complete litter-phase mechanism: snow-free does not imply unfrozen litter. The base result below is the covered, liquid/unfrozen forest-litter case; the explicitly admitted V3 phase successor is distinguished. An arbitrary frozen mineral/soil state does not become supported because litter V3 exists.

Candidate selective reading is permitted for this exercise by the entry and corrective package, not general adoption. All normative material remains binding. External single-file contracts were read completely: SC-VEGETATION-001 (3,080 lines), SC-WATBAL-001 (2,648), SC-SURFACELIQUID-001 (2,205), SC-COUPLEDTIME-001 (1,108), and SC-SNOWFREEZE-001 (4,295). These are external owner authority relative to LSE. References to CLM5, ISBA-MEB, Napoly, FSM2 and ORCHIDEE below describe the canonical contract's retained provenance, not a claim that this reader independently read the original papers, bibliography PDFs or legacy source. No additional equation was inferred from a secondary scientific source.

**Concrete rules and ownership.** Anchors below are relative to `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/` unless an external contract is named.

| Applicable boundary | Concrete rule and anchor |
|---|---|
| Model/domain | `surface-energy.md#selected-sources-and-domain` admits OPENWEPP_SNOW_FREE_LSE_V1, imported by V2, for snow absent at both endpoints, no terminal snow payload, finite positive neutral wind, positive interval/area, complete typed forcing/state/configuration, and liquid/unfrozen water and soil. One ground class per tile is mandatory. Forest litter and bare mineral soil are separate classes; canopy coverage is the actual vegetation topology, not a guessed consequence of the class label. Calm/nonneutral, unsupported frozen/thawing soil, unknown class or missing temperature lineage fails before constitutive calculation. |
| Mass and state | `soil-coupling.md#INV-LANDSURFACEENERGY-100`, `surface-energy.md#exact-ownership-and-state`: hydrology alone owns every ponded/litter/soil liquid or frozen water amount. LSE owns one surface thermal node per tile. Soil thermal owns all ordered soil-node temperatures and enthalpies. Vegetation owns canopy processes. Immutable snapshots and exact owner/configuration/transaction/support digests join these owners; no second mutable mass, temperature or enthalpy representation is allowed. |
| Area basis | Stand-ground is one OFE's horizontal ground area, not the whole routed hillslope. Positive finite unique tile fractions sum to one under `64*epsilon*max(sum(f),1)`. Local mass/energy is multiplied by `f_t` once to become OFE-ground. Cross-OFE parcels retain source and destination IDs; upstream-to-downstream intensive areal mass and energy both receive the upstream/downstream area ratio once, with original provenance and destination basis retained. |
| State versus warm start | `soil-coupling.md#surface-humidity-surface-enthalpy-litter-and-soil-heat`, INV-103: `U_s=(C_dry+W*C_w)*(T_s-T_ref)`, with `T_ref=273.15 K`, `C_w=4218 J kg^-1 K^-1`. `W` is hydrology's immutable solve operand. Temperature is a derived physical value; a retained temperature is only a numerical warm start/diagnostic and must be bit-identical to the temperature derived using the accepted hydrology candidate's ending water and authoritative energy. It cannot be independently adjusted. |
| Finite capacity | Explicit `finite_capacity` requires `C_dry+W*C_w>0`. Beginning surface temperature comes from beginning authoritative `U_s,W,C_dry`; accepted ending enthalpy owns the thermal state. End temperature follows the ending water/capacity, including the post-solve ingress credit chronology below. |
| Equilibrium zero | Explicit `equilibrium_zero` requires `C_dry=W=U_s=0` exactly; storage difference is exact zero and current `T_s` is an algebraic energy unknown. No physical old surface temperature exists. This is not an epsilon-capacity approximation or permission to discard nonzero water/energy. Ordinary litter with positive dry capacity naturally lies in finite capacity; do not force it into equilibrium-zero. |

Forest litter uses Napoly/ISBA's single layer (`soil-coupling.md`, same thermal section):

```text
h_ul     = 0.5*(1-cos(pi*W_l/W_l,max))
q_l      = h_ul*q_sat(T_s,p) + (1-h_ul)*q_recipient
v_l      = rho_a*(q_l-q_recipient)/r_l-c
lambda_l = 0.1 + 0.03*W_l/(1000*dz_l)
C_dry    = dz_l*rho_ld*c_ld
```

Require explicit finite positive `W_l,max`, admissible thickness/density/specific heat and `0<=W_l<=W_l,max`. `r_l-c` is the covered neutral ground-to-canopy resistance for the requested covered tile. Litter blocks direct mineral-soil evaporation and upward capillary supply in this interval. Overflow belongs to hydrology. The bare-soil CLM humidity/dry-surface-layer branch is not substituted beneath litter. Positive surface water in the admitted mineral branch uses its exact surface store and saturated humidity; that branch does not override litter's humidity law.

**First-node heat transfer.** `soil-coupling.md#surface-humidity-surface-enthalpy-litter-and-soil-heat`, INV-106, and soil-custody bind the following complete lower boundary. There are `N>=1` ordered soil nodes with finite positive `dz_k,lambda_k,C_k`; surface thickness and conductivity are positive. For litter use its actual `dz_l,lambda_l`.

```text
g_s1    = 2/(dz_s/lambda_s + dz_1/lambda_1)
g_k,k+1 = 2/(dz_k/lambda_k + dz_(k+1)/lambda_(k+1))
G_s1    = g_s1*(T_s-T_1)                 [positive downward]
G_k,k+1 = g_k,k+1*(T_k-T_(k+1))
bar(G)  = 0.5*(G_begin+G_end)
C_1*(T_1,end-T_1,begin)/dt = bar(G_s1)-bar(G_1,2)
C_k*(T_k,end-T_k,begin)/dt = bar(G_k-1,k)-bar(G_k,k+1)
C_N*(T_N,end-T_N,begin)/dt = bar(G_N-1,N)
```

The bottom flux is exact zero. For `N=1`, the sole equation is `C_1*Delta(T_1)/dt=bar(G_s1)`. Surface energy contains `-bar(G_s1)` and soil contains the identical `+bar(G_s1)` once. Older interface wording about the receiving opposite sign is interpreted with the explicit positive-downward convention, not as a license to compute another flux.

Finite capacity uses the derived physical old surface temperature in `G_begin`, and current trial end temperature in `G_end`. Equilibrium-zero uses the **same current algebraic trial `T_s` at both endpoints**, paired with `T_1,begin` and `T_1,end` respectively. Its caller warm start must never enter the beginning conduction term. This distinction affects the residual and Jacobian, not just reporting.

After the thermal solve, hydrology's accepted infiltration sensible energy is credited to ordered soil node 1 exactly once and temperature is derived from that credited enthalpy. It is not a second `G`, a new soil-water mutation, a latent-fusion alias or a same-interval re-solve. Deeper conduction and subsurface phase remain the soil/frost owner's responsibility; this base LSE model does not license frozen/thawing-soil physics.

A represented-snow bottom volume is a different OFE/lane-level boundary (`soil-coupling.md#version-8-persistent-snow--soil-boundary-amendment`, INV-124/125/126). It bypasses tile thermal nodes and cannot be emulated by selecting/averaging/weighting a tile. It is excluded from the requested snow-free solve. Crossing into that physical regime requires the snow owner authority, including SC-SNOWENERGY-001, rather than importing a few snow equations into this result.

**Covered radiation and turbulent closure.** `surface-energy.md#shortwave-and-reciprocal-longwave` / INV-101 and `#neutral-turbulent-heat-and-vapor-network` / INV-102 require current trial operands throughout. SC-VEGETATION-001's V8 coupled-ground-energy amendment retains V7's two-stream VIS/NIR calculation with the ground class's albedos as the full-column lower boundary. Ground receives terminal direct/diffuse energy once, and reflected ground radiation propagates back through the canopy. It does not jump directly to the atmosphere.

Longwave emissivity is one with no reflection. For occupancy `i`, `tau_i=exp[-0.8*Omega_i*(LAI_i+SAI_i)]`; apply clumping once. Use `Ldn_(i+1)=tau_i*Ldn_i+(1-tau_i)*E_i`, `Lup_n=sigma*T_s^4`, and the reciprocal upward recurrence. Each sun leaf, shade leaf, wet surface and dry stem emits at its own current temperature, with exact emissive-area weight `w_j`. Its net is `w_j*(1-tau_i)*(Ldn_i+Lup_(i+1))-2*w_j*(1-tau_i)*sigma*T_i,j^4`; ground receives `Ldn_n-sigma*T_s^4`. Zero total emissive area implies `tau_i=1` and exact zero component terms. Bulk-canopy temperature, stale ground temperature, prescribed upward ground longwave, independently aggregated canopy fluxes and a bypassed lower boundary are prohibited.

The covered tile has one zero-storage canopy-air node `(T_c,q_c)`. Ground and every V8 component exchange with that same node. Ground heat and signed vapor are `H_s=rho_a*c_p*(T_s-T_c)/r_h` and `v_s=rho_a*(q_s-q_c)/r_v`; component conductance remains vegetation-owned. The exact node equations are `sum(H_j)+H_s-H_c->atm=0` and `sum(v_j)+v_s-v_c->atm=0`. Ground occurs once per tile, never once per occupancy. Direct reference-air transfer beneath the canopy or omission of ground from the shared node is invalid.

The ground path is the specified neutral ISBA-MEB specialization (`psi_H=f_hv=1`):

```text
Re = u_l*l_w/nu
c_d = 1.328*(2/sqrt(Re)) + 0.45*((1-chi_L)/pi)^1.6
d = 1.1*z_hv*ln(1+(c_d*LAI)^0.25)
u_hv = u_ref
u_star = 0.4*u_hv/ln((z_hv-d)/z0v)
K_hv = 0.4*u_star*(z_hv-d)
r_g-c = z_hv/(phi_v*K_hv) *
        (exp(phi_v*(1-z0g/z_hv))-exp(phi_v*(1-(d+z0v)/z_hv)))
```

Constants are `phi_v=2`, `z0g=0.007 m`, `chi_L=0.12`, `u_l=1 m/s`, `l_w=0.02 m`, `nu=1.5e-5 m2/s`. Require `LAI>0`, `z_hv>d+z0v>z0g>0`, `z_ref-d>=z_hv-d>0` and valid finite positive resistances/logarithms/exponentials. Heat and vapor have distinct semantic operands even when their neutral resistance is numerically equal. Canopy-to-reference paths use the neutral log-law with displacement and configured `z0m,z0h,z0q`; require `z_ref>d+max(z0m,z0h,z0q)`. No wind floor, alternate roughness, convective velocity, empirical attenuation or stability correction is admitted. The canonical V8 current-temperature gas/interception/component calculation remains binding; a bulk Penman-Monteith demand cannot replace it.

**Immutable water, signed vapor and current ingress.** `water-vapor.md#water`, `#vapor`, `#errors`, common-details algorithm/guards/tests and the complete SC-SURFACELIQUID-001 and SC-WATBAL-001 owner contracts impose this order:

1. Freeze complete beginning owners before **any current-interval ingress**. Solve the complete uncapped potential canopy/root and ground demands `D` from those same beginnings.
2. Hydrology authorizes each exact source once, returning `A` against beginning availability. Rebuild the complete final solve from the same immutable beginnings with those fixed caps. A positive capped flux uses `min(current constitutive law,A/(dt*f_t))` on the appropriate basis; equality takes the cap branch with zero cap derivative. Exact source/owner/key joins and `0<=F<=A<=D` hold. Accepted use is `F=f_t*q*dt`, not authorization `A`.
3. Hydrology debits final actual use once; unused authorization stays in its source store. No second authorization, release-based reauthorization, request-order priority, cross-source reallocation, beta afterthought, bulk-ET donation or use of current rain/throughfall/runon/condensation as withdrawal supply is allowed. Signed negative vapor is a named-store condensation credit, never a negative withdrawal authorization.
4. The accepted pre-ingress energy solve closes `(U_pre-U_begin)/dt=R_sw+R_lw-H-Q_v-bar(G_s1)` with post-final-use/pre-ingress water. Form local primitive ledgers first and area-weight once. Independent end-minus-begin owner storage reconstruction must close the control volume; a producer's residual is only diagnostic.
5. Apply the admitted litter phase/phase-spill step if selected. Then adopt current ingress once through hydrology's actual WB14 partition. Credit retained surface sensible energy to the surface, infiltrated sensible energy to soil node 1, and routed/outlet sensible energy to the exact destination parcel. Do not feed the newly arrived mass or energy back into this interval's potential/final exchange or add another soil heat transfer.

The liquid enthalpy reference is `h_l(T)=4218*(T-273.15) J/kg`, parcel `Q=m*h_l(T)`. Vapor transports both sensible and latent enthalpy: `Q_v=v_s*(h_l(T_s)+L_v(T_s))`, with `L_v(T)=2.501e6-2369*(T-273.15) J/kg`. Evaporation is positive outward and condensation negative; neither flux nor its energy may be clipped to zero or reduced to latent-only.

Rain uses the exact retained `hydrometeor_temperature_c+273.15` output of `openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity` on active `harder_pomeroy_hourly` (SC-SNOWFREEZE-001 INV-075, with INV-064/065 provider provenance). LSE does not recompute or partially transcribe it. This contract's later default/provider authority supersedes older opt-in wording; active-hour `hrrain+hrsnow/10` must reconstruct precipitation, and consumers do not repartition it. Positive typed snowfall is sufficient to require snow classification even on a warm-air day. Canopy throughfall, initial drainage, second drainage and stemflow carry accepted wet-component temperature under SC-VEGETATION-001 INV-114. Runon carries its accepted upstream outlet temperature and enthalpy. Missing lineage rejects; air, local soil, freezing-point or downstream-surface temperature is not a substitute. A zero-mass crossing has exact zero energy and no fictitious temperature.

For each actual chronological mixing window, `T_mix=273.15+sum(m_i*h_i)/(4218*sum(m_i))`. Infiltration consumes the resulting well-mixed enthalpy, not a preferred source temperature: `Q_inf=I*h_mix`; excess retains `Q_in-Q_inf`. Source attribution is proportional in canonical order with the prescribed final floating remainder. Do not globally blend temporally disjoint arrivals, use temperature-priority infiltration, recompute hydrology's split, or credit raw rainfall again beneath canopy.

SC-SURFACELIQUID-001 adds operationally essential guards to this seam:

- Water requests join the complete GroundWaterKey, OFE, source tile, source type and source ID. For group supply `S_k=f_t*W_begin`, raw pro-rata authorization is `R_i=D_i` if sufficient, otherwise `fl(fl(D_i*S_k)/sum(D))` in canonical complete-key order. Both OFE `sum(R_i)<=S_k` and inverse tile `sum(fl(R_i/f_t))<=W_begin` must hold exactly. Only a raw roundoff overshoot within `1e-14+64*epsilon*sum(abs(operands)) kg/m2` admits the canonical common proportional contraction. If the common ratio is not safe in both spaces, select the greatest admissible positive binary64 factor by monotone bit bisection, at most 64 decisions, preserving positive rows. No per-row nextafter, last-row repair, order-dependent priority or tolerance-based overdraw is authorized.
- Post-vapor water is beginning minus final use plus condensation. Retain to physical capacity; pre-current-ingress overflow is an explicit parcel at accepted surface temperature with explicit energy. It is not donated as supply. Covered atmospheric ingress is exactly the tile-weighted accepted four canopy release paths; raw rain may separately serve erosion forcing but is not another WB14 supply.
- All source boundaries enter one chronological, stateful production Green-Ampt/WB14 transition per OFE child, sharing retained day cumulative supply/infiltration, canonical soil parameters/storage and same-pass soil credit. Convert kg/m2 to metres using 1000 kg/m3 once. Do not copy infiltration equations, call per parcel independently, replay daily state, or retain a second depression-capacity/delta owner. Persistent surface ownership requires the legacy depression-capacity/delta shadow to be zero.
- Infiltration precedes retention. With `R_raw=f_t*(W_max-W)`, excess `E`, `M_raw=min(E,R_raw)` and `tau=1e-14+64*epsilon*(abs(f_t*W_max)+abs(f_t*W)+abs(E))`, a positive `M_raw<=tau` is routed entirely: retain zero and leave the store bits unchanged. Do not discard mass/energy or snap storage to capacity. Larger retention is ordinary; retained tile energy is `(m_ret*h_mix)/f_t`. Destination keys prevent redistribution of another tile's excess.
- Persistent parent cadence is 48 parents of 1,800 seconds, with admitted positive adaptive children on the exact 60-second grid; historical mass-based/fixed child menus do not override current adaptive authority. Child receipts advance private cumulative state; persistent cursor/transaction publication happens once at the accepted parent. Routing queues reach a downstream eligible child with exact support and provenance. The active Lane-D posture and day-zero bootstrap restrictions are not a general production-output cutover authorization.
- Mass and energy closure tolerances are respectively `1e-14+64*epsilon*sum(abs(operands)) kg/m2` and `1e-9+64*epsilon*sum(abs(operands)) J/m2`; identity and exact resource bounds are not repaired by these tolerances.

One very narrow liquid publication normalization exists in `water-vapor.md`: only binary64 temperature bits `0x4071126666666667` become `0x4071126666666666` at the named liquid ledger/release or Stage-3 terminal-temperature publication boundary (`T_ref+next_up(2^-44 K)`). All other temperatures remain untouched or fail their domain. This is not a solver clamp, general cold-liquid repair or exact-energy rounding rule.

**Exact receiving energy is physical custody, not a residual patch.** Read `soil-custody.md#soil-custody` and `surface-custody.md#surface-custody` in full. Soil receiver V2 represents `E=exact(H_hi)+R`, with canonical exact dyadic carry (sign, odd hexadecimal significand, exponent; unique zero). Decode the preexisting finite high and carry, add the original finite accepted primitive thermal/top/ingress credits exactly, round once to binary64 nearest-even and retain the exact remainder. Preserve the existing high's signed-zero conventions; reject overflow rather than silently saturating. Do not force a high-part ULP, discard a sub-ULP credit, zero a residual, flush a subnormal, introduce a producer compensation term or let carry drive a constitutive temperature/flux/phase calculation.

The WAT5 first-node example retains `H_hi=-34315.42154113602` unchanged after `Q_inf=-8.0670339832330148e-19`; carry retains the exact `-0x1dc319224e55f*2^-109` credit. The point is that an unchanged binary64 temperature/high part does not mean the accepted physical energy vanished. Primitive-source and independently reconstructed owner closure remain required.

Surface V16 similarly owns `U=exact(U_hi)+R`; older V2/V3 physical fields remain bit-identical nonauthoritative high mirrors. First execute the canonical accepted binary64 OFE group-energy and tile-area division, then decode that actual accepted operand exactly; an ideal rational division is a different operand. Phase-free, fusion, retained-ingress and negative phase-spill terms retain distinct original receipt origins. Exact companion carry supports closure, not altered physical forcing. Configuration-defined rank with opaque IDs defines order; lexical, numeric-ID or receipt-hash sorting cannot replace it.

Partial parent updates retain the original predecessor joins across surface, soil and hydrology while advancing the accepted energy receipt chain; final publication stamps once. Candidate-only soil staging is a nonowner and cannot leak into restart or live owner bytes. Only full replay from the original owner using the selected physical ending and original exact credits can atomically install one successor. Missing historical primitive operands (including a real p61 reconstruction) cannot be invented from a final residual. Restart, owner mirrors, high/carry, receipts, topology and transaction joins fail atomically on mismatch.

**Conditional frozen-litter rules.** `litter-phase.md#litter-phase` admits OPENWEPP_SNOW_FREE_LSE_V3 as a specific snow-free litter successor, importing V2 physical coupling. It does not admit frozen soil or reinterpret litter ice as represented snow. Require explicit beginning liquid `W_l` and water-equivalent ice `W_i`, phase-specific authorizations and immutable owners. Sensible enthalpy is `(C_dry+W_l*C_w+W_i*C_i)*(T-T_ref)`, with `C_i=2106 J/kg/K`, `rho_i=920 kg/m3`, `L_f=333700 J/kg`, phase time `tau=3300 s`, and `W_i,max=0.85*rho_w*dz_l`.

The coupled phase-free V2 solve precedes the prescribed phase step. Vapor uses liquid saturation for both phases, liquid/ice humidity factors, and beginning phase fraction `p_i=W_i/(W_l+W_i)` (zero for an empty store). Positive liquid and ice fluxes consume their own beginning caps; signed negative phase fluxes credit their named phases. No ice-saturation substitute, tiny-ice omission, minimum wetness floor or cross-phase donation is admitted.

After vapor, compute bounded melt and freeze from the same accepted state:

```text
m_melt = min(W_i, (dt/tau)*min(rho_i*C_i*dz_l*max(T-T_ref,0)/L_f,W_i))
m_freeze = min(W_l,W_i,max-W_i,
               (dt/tau)*min(rho_i*C_i*dz_l*max(T_ref-T,0)/L_f,W_l))
U_phase = U_star + L_f*(m_freeze-m_melt)
```

At most one direction is positive. `H=U-L_f*W_i` conserves the phase reference; derive ending temperature from ending phase capacity. Do not re-solve atmospheric fluxes or transfer fusion as soil `G`. If phase-created raw liquid exceeds `W_l,max`, spill exactly the excess at raw accepted temperature, export its `m*h_l(T)` once, reduce surface sensible energy accordingly and derive retained temperature. The typed full-child internal LitterPhaseOverflow carries the actual phase receipt as its original source. It enters the one ordinary WB14/mixing/routing path; later retention cannot cancel its original surface energy debit. Ice does not enter WB14, runoff, soil `frozwt` or a second daily WB17 debit.

In heterogeneous native/ordinary batches, native phase rows already applied are excluded from a second debit. Every unmatched ordinary surface-use row must authenticate and debit once against the phase-adjusted owner; soil rows go to the soil owner. Native phase ice/energy/spill bytes remain unchanged by the ordinary join, and ordinary debits create no invented phase-energy receipt.

**Support, solve, map and publication boundaries.** `terminal-support.md#support` and INV-116..123 plus the complete SC-COUPLEDTIME-001 define exact integer-nanosecond half-open supports independently of constitutive admissibility. Derive floating duration once and share identical bits across owners. A positive covered forest solve requires `dt>=60_000_000_000 ns`, authenticated by LseSupportAdmissibilityReceiptV1 binding parent/segment/slab, endpoint ticks, duration bits, active owner/policy set, minimum and semantic digest. Below by one nanosecond rejects with LSEB-E-041 **before Newton**; mismatched receipt returns LSEB-E-042. Exactly at the floor retains the same physics/tolerances. Stable ordinary cases must accept substantially larger supports; neither a one-nanosecond clock interval nor a longer-step result scaled down licenses subminimum physics.

All coupled owners advance from one immutable accepted slab beginning. Rejection restores every owner byte and receipt predecessor; inactive owners remain identical. Adaptive accepted children tile exactly, retain direct-before-composed and Half1-before-Half2 custody, and publish only at the accepted parent. Zero-duration events have zero physical rates/work and one custody transition. A snow-free successor requires a consumed event-boundary receipt and rebuilt **post-event** snow-free forcing on its exact remaining support. Zero remainder skips Newton. Snow temperature, albedo, roughness, vapor, sensible/longwave flux, stale terminal liquid or unallocated snow energy must not enter the successor (LSEB-E-043). Current SC-SNOWFREEZE-001 v140 binds adaptive 60-second compositional batch mechanics and supersedes old bisection/fixed-step/fallback language; it does not retroactively prove LSE qualification or cutover.

`nonlinear-solve.md#solve` and its V2 qualification sections retain the canonical coupled solve: deterministic tile and canopy component ordering, V8 coordinates, canopy-air `T_c,q_c`, surface temperature and soil nodes 1..N. Closed temperature bounds are 200..350 K with the admitted liquid-vapor lower boundary at 273.15 K; humidity is 0..0.1. Finite differences use `sqrt(epsilon)*max(abs(x),unit_scale)`, unchanged minus-then-plus centered probes, with only the specified unique inward stencil at an exact closed bound when the base is valid and one probe invalid. No adaptive probe reduction or clamped trial substitutes.

Use the canonical partial-pivot dense solve with deterministic lowest-row ties and `64*epsilon` norm singularity guard, at most 50 accepted updates and 20 backtracking halvings with strict residual decrease. Energy residual tolerance is `1e-6 W/m2 + 1e-10*scale`; water residual is `1e-12 kg/m2/s + 1e-9*scale`; step tolerances include `1e-8 K`, `1e-12` humidity, `1e-7 mm` water potential and `1e-10` beta. A small step alone is not convergence. The prescribed already-closed-point termination witness can accept the unchanged current point after the first domain-valid halved complete prospective witness passes; it does not accept that trial or skip complete residual/domain checks.

V2 retains V1 physics and the specified FullSupply positive/zero-demand branch. Accepted potential coordinates may seed the final solve only in its exact FullSupply case; the final immutable-state/fixed-cap evaluation is complete, and zero iterations require all residual/step/domain checks. Copying a potential physical candidate is not a final solve. Nonpositive/partial-root unsupported combinations fail; the admitted V10 nonpositive-potential scaling and inactive-component trial qualification do not erase active physical residuals or admit a general temperature floor.

Error precedence is canonical: schema/identity/topology/owner, nonfinite, unsupported, constitutive, resource, numerical singular/backtrack/iteration/step/residual, component/control-volume and cross-owner closure, retaining the first cause and exact rollback. Neither an alternate solver nor cleanup failure may mask it.

Applicable `map-custody.md#handoff`, `#reseal`, `#pending`, `#validation` add **custody only**, not another thermal algorithm. The snow-free provisional/final handoff may reseal exact slab identity while every nonslab semantic input/owner remains bound; it performs one physical calculation and zero/one publication. Native exact-energy candidates remain unpublishable until the complete owner join. A covered Initial map is ordinal 0, FixedPointAdjudication ordinal 1, and Multisecant(n) ordinal n+1 for n=1..5. An own-output outer closure failure yields history; an outer pass with failed authentic dependent-versus-prior closure is a typed rejection, not history. Both passing consume the **same move-only pending result** into FinalAccepted with one final constructor, no new charged physical evaluation or replay. Physical map counts are `M=N+2` (2..7, within the eight-map ceiling). A history candidate cannot be promoted or fall back to another solver.

INV-159 / OBL-C-019 validation-once proof is private semantic custody. Parent structural validation is lazy at its original guard position; exact current forcing validates before V8, which consumes only its pointer-identical proof. Dynamic V8 state is always freshly validated; ingress scheduling remains fallible before any native resident join. Native V3-LSE/V2-surface objects are distinct from structural V8 objects. Only an exact resident revision/parent/map/pointer join may omit the two specified duplicate validations. No proof admits an equal-digest/different-allocation object, caches a projected column/solver result/dynamic owner, persists or transfers across map/parent/restart, or silently falls back to full validation. Ordinary maps mint no native-resident proof. This is relevant if the actual consumer uses this admitted path; no execution of it is asserted here.

Represented snow is classified from immutable beginning state before evaluation (`terminal-support.md`, INV-154). It makes snow the sole atmospheric ground surface and keeps litter V3/V4 vapor, phase, storage, ingress, WB14 and exact-owner bytes inactive. It is not another simultaneous litter energy solve. Only an accepted terminal split permits the positive snow-free successor. Snow-specific identity-anchor and dependency replay optimizations were not selected as authority for the requested snow-free physical rules; any implementation or reuse assertion would expand those full numerical/replay sections and external snow authority. I make no such assertion.

**Required supplied inputs and independent tests.** A concrete run cannot be accepted from the phrase “covered forest tile” alone. It needs the named model/phase/capacity branch; schema and frozen definition identity; run/OFE/tile and occupancy ranks/fractions; exact parent/slab/support/transaction/predecessor joins; beginning hydrology stores and source keys; surface high/carry and its mirrors; soil topology, high/carry, temperatures, thicknesses, conductivities and areal capacities; litter capacity, thickness, density and heat capacity; canonical canopy V8/V10 configuration and state; finite radiation, pressure, humidity, moist-air properties, current component areas/temperatures and neutral wind geometry; accepted precipitation phase/provider temperature; exact canopy releases; runon parcel support/origin/destination/temperature/energy; hydrology demand/authorization/final-use/condensation receipts; WB14 continuation and production soil inputs; and ingress/retention/infiltration/routing/owner receipts. Missing, extra, unknown, duplicate, stale, nonfinite, wrong-owner, wrong-basis or domain-invalid inputs reject rather than default.

Common-details `#tests`, the selected mechanism tests/obligations, and the external owner test requirements imply at least these independent checks for this seam:

- Day/night, dry/wet litter, zero shortwave, evaporation and condensation, reciprocal canopy/ground sensitivity and sign reversals; heterogeneous columns/tiles with unequal fractions prove complete component and canopy-air closure and one weighting. Poison stale/bulk longwave, direct-air covered exchange, duplicated or missing ground terms, wrong albedo lower boundary and wind/stability substitutes.
- Equilibrium-zero with poisoned warm starts proves no physical old surface temperature; finite-capacity cases independently derive both temperatures/capacities. `N=1` and multinode CN vectors reconstruct series resistance, begin/end fluxes, exact opposite signs, zero bottom flux and first-node storage. Reject wrong node, wrong sign, duplicate transfer and use of tile aggregation for OFE snow coupling.
- Source scarcity/full supply, exact-zero requests, arbitrary request permutations, fractional-area roundoff, exact `F<=A<=D`, unused authorization and beginning-store conservation. Poison debiting authorization, second authorization, PM/beta donation, use of current ingress as supply, duplicate native/ordinary phase rows and condensation clipping.
- Rain/throughfall/drainage/stemflow/runon temperature provenance; mixed hot/cold arrivals on distinct chronological windows; zero mass/no temperature; exact-one WB14 continuation; infiltration-before-retention; tiny positive retention routing with unchanged storage bits; unequal-area OFE transfer; routed destinations; no raw-rain double ingress. Independently reconstruct mass and sensible/latent energy, not just serialized producer residuals.
- High/carry cancellation, ties, subnormals, overflow rejection, signed zero, repeated sub-ULP credits, multiple original receipts, unequal fractions and accepted binary64 division order; poison mirrors, ordering, owner joins, restart and synthesized residual carry. Include WAT5 and the real first-node/publication lineage; missing real p61 operands remain a missing-evidence result, not a guessed pass.
- For V3: both phase directions, no simultaneous conversion, tau scaling and bounds, ending heat capacity, liquid-saturation vapor in both phases, separate caps, phase-origin overflow and negative energy debit, retained-temperature reconstruction and heterogeneous ordinary joins. Poison snow/soil-ice aliases, repeated daily WB17 use and a second phase or atmospheric solve.
- Exactly at/below/above 60 seconds, mismatched support receipt, ordinary larger-step acceptance, exact zero-duration event, accepted post-event forcing, restarted/continuous byte identity and every failure-point rollback. A clock-valid but constitutively subminimum interval must fail before Newton.
- Solver failure/termination bounds, centered and admitted inward probes, immutable potential/final recomputation and correct fixed-cap derivatives; poison stale residual acceptance, invalid potential-candidate copying, partial-root branch and alternate solvers.
- For admitted map custody: forced-complete physical-prefix parity, zero nonfinal and one final construction/publication, exact source provider counts, pending/history/reject/final exclusivity and 2..7 charged-map accounting. OBL-C-016 requires authentic ordinary/native consumer tests. OBL-C-019 requires the authentic 52-map workload's exactly 1 parent-static, 52 exact forcing and 52 fresh dynamic validations, with role/path bitwise parity, ordinary zero-native execution and paired guard-order poisons. These are required execution evidence, not counts demonstrated by this reading exercise.

Calibration/identifiability is not applicable to numerical support, exact custody and validation reuse; none of these permits coefficient fitting or relaxed closures. Contract-derived vectors and genuine independent operand/real-consumer evidence are necessary. Existing empirical/production qualification holds are not discharged by this answer. No unresolved normative applicability remains for the explicitly liquid/unfrozen covered-litter base case; actual deployment still requires the typed input branch and evidence above. Frozen soil, represented-snow physics, production identity qualification and implementation-specific replay/leaf reuse require additional owner/route selection and are not claimed here.

**Expansion and isolation record.** Surface/soil equations demanded shared variables/state/algorithm/guards/aliases/constants/units/tolerances/calibration/tests, the ordered solve, signed water/ingress and both exact-storage owners. The generic litter task left phase ambiguous, so I read litter-phase completely rather than assuming its ice store absent. Canopy lower-boundary/recipient/release temperature required complete vegetation authority. Immutable water and WB14 required complete surface-liquid and water-balance authority. Physical support and accepted-map custody required complete coupled-time authority and applicable terminal/map sections. Exact rain temperature required complete snowfreeze authority; later version-140 text resolves historical time-mechanics supersession. Validation definitions expanded source-ordered current map custody. None of those expansions used a byte target.

The mandatory work-package handoff read incidentally exposed predicted selection counts on worker-handoff lines 9–10. I notified the parent and did not use the counts as a reading target or authority. This is an isolation limitation, not a claim of perfect blindness. No other exercise answer, review, rubric, intake matrix, predecessor candidate, monolith from Git, parent preservation/test artifact or other agent output was inspected. The snowfreeze contract itself contains its canonical historical snow-fidelity rubric; that was encountered solely as part of the mandatory whole external contract and is unrelated to exercise grading. Later metadata recount detected that worker-handoff had grown from the read 2,174-byte/e3545590… version to 4,408-byte/dc48700f…; its new source text was not delivered to this reader and is not scientific evidence here. The registry index was never read; the parent's notice of its last_reviewed-only change therefore adds no scientific reading request.

**Measurement and request accounting.** The JSON below records each actual source-reading request separately, including repeated and recovery reads. `bytes` means the selected inclusive source range's UTF-8 bytes, not terminal wrapper bytes or tokens. `sha256` is the whole source file hash at that request, not a selected-range hash. `full` means the complete source version bearing that hash. Ranges are one-based inclusive; a section request includes its requested lines exactly. This is finite structural reading exposure, not automatic-context or session telemetry.

Source-request exposure is 1,929,056 bytes across 103 requests: bootstrap 53,616 bytes and expansion 1,875,440 bytes. The finite unique union by `(path,source SHA256,byte position)` is 1,828,150 bytes; overlapping/repeated requested exposure is 100,906 bytes. The union includes recovery of truncations but does not double-count them. The original combined common-details/soil/water read was truncated; soil was re-requested in full and common-details 220–444 recovered, with its first 219 lines and water text already delivered. Vegetation requests 751–1100 and 1351–1550 were truncated; 960–1030 and 1470–1490 respectively recovered the omitted text. All five whole external contracts were eventually delivered completely in bounded chunks. I do not treat the originally truncated requests as fully observed deliveries.

`tools/agents/context_report.py` was run using `.venv/bin/python` after a direct executable attempt returned permission denied. Its declared selection report is structural, not a reading selector or semantic validator. It re-counted the subsequently changed handoff at current working-tree bytes, so its then-current aggregate (1,821,750 unique; 55,850 bootstrap exposure plus 1,864,744 expansion exposure, before the last two expansion requests) is not the historical request metric. The figures above instead use the per-request observed hashes/bytes and merge ranges; they retain the originally read handoff version. Tool-internal file recounts delivered counts/hashes, not source text; they are measurement operations, not additional semantic source reads.

Search and metadata requests are separate from the source-byte union: `pwd`; `tools/agents/find-agents --for` the report and canonical entry; heading/dependency `rg -n '^#|^\\|.*\\.md'` over terminal-support, nonlinear-solve, soil-custody and surface-custody (search output truncated); `wc -lc` on the five external contracts; map-custody heading/anchor `rg` for handoff/reseal/pending; `wc -l` litter-phase; heading `rg` on prompt-wording-guidance, role-review and standards AGENTS; context-report direct invocation/help/Python help; Python hash/line-count/log/union metadata and the context-report structural recount. The historical exact delivered search-output bytes/tokens are UNOBSERVED and are not silently charged as full-file scientific reading. No truncated search output is claimed complete. The context-report source itself was read in full and is listed in the JSON. Temporary request/selection/measurement JSON bookkeeping was under `/tmp`; the only repository write is this report.

Delivered input/output tokens, automatic context, compaction/replay exposure, whole workflow total and other roles' reading: **UNOBSERVED**. No token conversion, percentage saving, observed session-total or comparison to an unread predecessor is inferred from source bytes.

```json
[
  {
    "path": "AGENTS.md",
    "ranges": "full",
    "bytes": 9508,
    "sha256": "9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6",
    "phase": "bootstrap"
  },
  {
    "path": "docs/work-packages/AGENTS.md",
    "ranges": "full",
    "bytes": 6642,
    "sha256": "29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5",
    "phase": "bootstrap"
  },
  {
    "path": "docs/specifications/science-contracts/AGENTS.md",
    "ranges": "full",
    "bytes": 6532,
    "sha256": "bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7",
    "phase": "bootstrap"
  },
  {
    "path": "docs/work-packages/20260907-lse-context-adoption-correction-001/package.md",
    "ranges": "full",
    "bytes": 5304,
    "sha256": "0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b",
    "phase": "bootstrap"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
    "ranges": "full",
    "bytes": 3958,
    "sha256": "c94f30d0e4462284769d498ca31fe80e7a5b0d5026c233c85c1ddd310d2b6058",
    "phase": "bootstrap"
  },
  {
    "path": "docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md",
    "ranges": "full",
    "bytes": 2174,
    "sha256": "e35455905556092e67eaf8e4c79cc2b8747b4bcd7a1d55089e162d185498797c",
    "phase": "bootstrap"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md",
    "ranges": "full",
    "bytes": 8110,
    "sha256": "22ed48fa58a32920ddfde47c339650896c16978b06bf4a722dc1b094106e08f6",
    "phase": "bootstrap"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md",
    "ranges": "full",
    "bytes": 11388,
    "sha256": "f741c16f6dcbe37cf3a3ab5c2e7dfcb418fed6b03900b41fda483d5ec53a03e2",
    "phase": "bootstrap"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md",
    "ranges": "full",
    "bytes": 41620,
    "sha256": "a4e06643713c94fb771e87719cbafc9994cf2bd88050ff5c9e0ae9ef81cf454a",
    "phase": "expansion"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md",
    "ranges": "full",
    "bytes": 13488,
    "sha256": "9a9fc30b02736875d961ece3df220f243abc1b9ea446cb582bbbcf8d2f8b07a9",
    "phase": "expansion"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md",
    "ranges": "full",
    "bytes": 16549,
    "sha256": "6fd1c830c7d1fc9fbe465c28a50eade50d1585cdd30b8c599a5ab8e520fe64ff",
    "phase": "expansion"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md",
    "ranges": "full",
    "bytes": 13488,
    "sha256": "9a9fc30b02736875d961ece3df220f243abc1b9ea446cb582bbbcf8d2f8b07a9",
    "phase": "expansion-repeat-truncation-recovery"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md",
    "ranges": [
      [
        220,
        360
      ]
    ],
    "bytes": 17677,
    "sha256": "a4e06643713c94fb771e87719cbafc9994cf2bd88050ff5c9e0ae9ef81cf454a",
    "phase": "expansion-repeat-truncation-recovery"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md",
    "ranges": [
      [
        361,
        444
      ]
    ],
    "bytes": 5963,
    "sha256": "a4e06643713c94fb771e87719cbafc9994cf2bd88050ff5c9e0ae9ef81cf454a",
    "phase": "expansion-repeat-truncation-recovery"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md",
    "ranges": [
      [
        1,
        17
      ],
      [
        90,
        176
      ],
      [
        235,
        239
      ]
    ],
    "bytes": 9875,
    "sha256": "c74b6cfdcf3bc10bc59c4ff9ec23b022e28bb6a55fc395d847dc03790afe3f16",
    "phase": "expansion"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md",
    "ranges": [
      [
        1,
        110
      ],
      [
        440,
        445
      ]
    ],
    "bytes": 12488,
    "sha256": "a5396285816d2f8c6c1b58b314b9d583440f552437e4e09a95f4c8ef1e8f2513",
    "phase": "expansion"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md",
    "ranges": "full",
    "bytes": 15781,
    "sha256": "ce45ad698fcca35027377a51205b8441e8bcaa6f32221cf23f1569fb55e4d805",
    "phase": "expansion"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md",
    "ranges": "full",
    "bytes": 21567,
    "sha256": "5eb78614c4555bc299ceb05495ece242dd949c19277473a11a49d49b1e2c54d3",
    "phase": "expansion"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md",
    "ranges": [
      [
        111,
        319
      ],
      [
        435,
        439
      ],
      [
        446,
        449
      ]
    ],
    "bytes": 15460,
    "sha256": "a5396285816d2f8c6c1b58b314b9d583440f552437e4e09a95f4c8ef1e8f2513",
    "phase": "expansion-V2-and-domain-qualifiers"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md",
    "ranges": [
      [
        1,
        15
      ],
      [
        35,
        196
      ],
      [
        339,
        352
      ]
    ],
    "bytes": 21794,
    "sha256": "dd67c49876597d991f9aef28f83d14658d0b5ad0bc1a7b47342ead747bdf42f3",
    "phase": "expansion-receipt-origin"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md",
    "ranges": [
      [
        1,
        200
      ]
    ],
    "bytes": 10639,
    "sha256": "8da07a0f890010c8c0ed90672b4040f978498a6d6ee07dde74d35833813c5228",
    "phase": "expansion-snow-free-not-necessarily-unfrozen"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md",
    "ranges": [
      [
        201,
        428
      ]
    ],
    "bytes": 20248,
    "sha256": "8da07a0f890010c8c0ed90672b4040f978498a6d6ee07dde74d35833813c5228",
    "phase": "expansion-snow-free-not-necessarily-unfrozen"
  },
  {
    "path": "docs/standards/AGENTS.md",
    "ranges": "full",
    "bytes": 4052,
    "sha256": "b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015",
    "phase": "expansion-governance"
  },
  {
    "path": "docs/work-packages/role-review.md",
    "ranges": "full",
    "bytes": 1400,
    "sha256": "2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1",
    "phase": "expansion-governance"
  },
  {
    "path": "docs/standards/prompt-wording-guidance.md",
    "ranges": [
      [
        174,
        194
      ]
    ],
    "bytes": 1333,
    "sha256": "294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f",
    "phase": "expansion-governance"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1,
        250
      ]
    ],
    "bytes": 21653,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        251,
        450
      ]
    ],
    "bytes": 15166,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        451,
        750
      ]
    ],
    "bytes": 19894,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        751,
        1100
      ]
    ],
    "bytes": 41915,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        960,
        1030
      ]
    ],
    "bytes": 19027,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-repeat-truncation-recovery"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1101,
        1350
      ]
    ],
    "bytes": 19100,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1351,
        1550
      ]
    ],
    "bytes": 31384,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1470,
        1490
      ]
    ],
    "bytes": 7874,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-repeat-truncation-recovery"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1551,
        1850
      ]
    ],
    "bytes": 18358,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        1851,
        2170
      ]
    ],
    "bytes": 16457,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        2171,
        2490
      ]
    ],
    "bytes": 23406,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        2491,
        2800
      ]
    ],
    "bytes": 19344,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md",
    "ranges": [
      [
        2801,
        3080
      ]
    ],
    "bytes": 22152,
    "sha256": "3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        1,
        187
      ]
    ],
    "bytes": 23659,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        188,
        238
      ]
    ],
    "bytes": 23551,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        239,
        265
      ]
    ],
    "bytes": 23082,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        266,
        288
      ]
    ],
    "bytes": 23644,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        289,
        330
      ]
    ],
    "bytes": 23610,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        331,
        407
      ]
    ],
    "bytes": 23826,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        408,
        471
      ]
    ],
    "bytes": 23938,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        472,
        539
      ]
    ],
    "bytes": 23929,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        540,
        650
      ]
    ],
    "bytes": 23986,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        651,
        873
      ]
    ],
    "bytes": 23937,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        874,
        1284
      ]
    ],
    "bytes": 23942,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        1285,
        1696
      ]
    ],
    "bytes": 23994,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        1697,
        2127
      ]
    ],
    "bytes": 23978,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        2128,
        2456
      ]
    ],
    "bytes": 23910,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        2457,
        2550
      ]
    ],
    "bytes": 23978,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
    "ranges": [
      [
        2551,
        2648
      ]
    ],
    "bytes": 20673,
    "sha256": "1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
    "ranges": [
      [
        1,
        410
      ]
    ],
    "bytes": 23963,
    "sha256": "fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
    "ranges": [
      [
        411,
        704
      ]
    ],
    "bytes": 23795,
    "sha256": "fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
    "ranges": [
      [
        705,
        805
      ]
    ],
    "bytes": 23820,
    "sha256": "fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
    "ranges": [
      [
        806,
        1099
      ]
    ],
    "bytes": 23951,
    "sha256": "fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
    "ranges": [
      [
        1100,
        1499
      ]
    ],
    "bytes": 23990,
    "sha256": "fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
    "ranges": [
      [
        1500,
        1654
      ]
    ],
    "bytes": 23994,
    "sha256": "fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
    "ranges": [
      [
        1655,
        1997
      ]
    ],
    "bytes": 23956,
    "sha256": "fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
    "ranges": [
      [
        1998,
        2205
      ]
    ],
    "bytes": 16647,
    "sha256": "fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        1,
        407
      ]
    ],
    "bytes": 23980,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        408,
        593
      ]
    ],
    "bytes": 22694,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        594,
        851
      ]
    ],
    "bytes": 23995,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
    "ranges": [
      [
        852,
        1108
      ]
    ],
    "bytes": 21883,
    "sha256": "1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        1,
        97
      ]
    ],
    "bytes": 23466,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        98,
        183
      ]
    ],
    "bytes": 23833,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        184,
        256
      ]
    ],
    "bytes": 23933,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        257,
        290
      ]
    ],
    "bytes": 23667,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        291,
        309
      ]
    ],
    "bytes": 23780,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        310,
        325
      ]
    ],
    "bytes": 23720,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        326,
        338
      ]
    ],
    "bytes": 22601,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        339,
        348
      ]
    ],
    "bytes": 22456,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        349,
        360
      ]
    ],
    "bytes": 23885,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        361,
        370
      ]
    ],
    "bytes": 22718,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        371,
        490
      ]
    ],
    "bytes": 23689,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        491,
        564
      ]
    ],
    "bytes": 23931,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        565,
        666
      ]
    ],
    "bytes": 23568,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        667,
        992
      ]
    ],
    "bytes": 23949,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        993,
        1323
      ]
    ],
    "bytes": 23985,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        1324,
        1475
      ]
    ],
    "bytes": 23721,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        1476,
        1532
      ]
    ],
    "bytes": 23399,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        1533,
        1907
      ]
    ],
    "bytes": 23955,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        1908,
        2293
      ]
    ],
    "bytes": 23998,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        2294,
        2664
      ]
    ],
    "bytes": 23962,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        2665,
        3029
      ]
    ],
    "bytes": 23926,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        3030,
        3440
      ]
    ],
    "bytes": 23932,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        3441,
        3789
      ]
    ],
    "bytes": 23980,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        3790,
        4004
      ]
    ],
    "bytes": 23449,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        4005,
        4181
      ]
    ],
    "bytes": 23878,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        4182,
        4223
      ]
    ],
    "bytes": 23981,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md",
    "ranges": [
      [
        4224,
        4295
      ]
    ],
    "bytes": 19160,
    "sha256": "50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4",
    "phase": "expansion-external-whole-file-part"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md",
    "ranges": [
      [
        197,
        338
      ],
      [
        353,
        355
      ]
    ],
    "bytes": 13858,
    "sha256": "dd67c49876597d991f9aef28f83d14658d0b5ad0bc1a7b47342ead747bdf42f3",
    "phase": "expansion-validation-support"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md",
    "ranges": [
      [
        18,
        89
      ]
    ],
    "bytes": 4548,
    "sha256": "c74b6cfdcf3bc10bc59c4ff9ec23b022e28bb6a55fc395d847dc03790afe3f16",
    "phase": "expansion-validation-support"
  },
  {
    "path": "tools/agents/context_report.py",
    "ranges": "full",
    "bytes": 3544,
    "sha256": "2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde",
    "phase": "expansion-measurement-tool"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md",
    "ranges": "full",
    "bytes": 11388,
    "sha256": "f741c16f6dcbe37cf3a3ab5c2e7dfcb418fed6b03900b41fda483d5ec53a03e2",
    "phase": "expansion-anchor-recheck-repeat"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md",
    "ranges": "full",
    "bytes": 13488,
    "sha256": "9a9fc30b02736875d961ece3df220f243abc1b9ea446cb582bbbcf8d2f8b07a9",
    "phase": "expansion-anchor-recheck-repeat"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
    "ranges": "full",
    "bytes": 3958,
    "sha256": "c94f30d0e4462284769d498ca31fe80e7a5b0d5026c233c85c1ddd310d2b6058",
    "phase": "expansion-route-recheck-repeat"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md",
    "ranges": [
      [
        1,
        50
      ]
    ],
    "bytes": 3024,
    "sha256": "a4e06643713c94fb771e87719cbafc9994cf2bd88050ff5c9e0ae9ef81cf454a",
    "phase": "expansion-route-recheck-repeat"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md",
    "ranges": [
      [
        1,
        35
      ]
    ],
    "bytes": 2957,
    "sha256": "6fd1c830c7d1fc9fbe465c28a50eade50d1585cdd30b8c599a5ab8e520fe64ff",
    "phase": "expansion-route-recheck-repeat"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md",
    "ranges": [
      [
        177,
        245
      ]
    ],
    "bytes": 8869,
    "sha256": "c74b6cfdcf3bc10bc59c4ff9ec23b022e28bb6a55fc395d847dc03790afe3f16",
    "phase": "expansion-applicable-definitions"
  },
  {
    "path": "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md",
    "ranges": [
      [
        450,
        455
      ]
    ],
    "bytes": 1827,
    "sha256": "a5396285816d2f8c6c1b58b314b9d583440f552437e4e09a95f4c8ef1e8f2513",
    "phase": "expansion-applicable-definitions"
  }
]
```
