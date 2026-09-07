# Surface-energy/soil coupling reading exercise — first completed report

Static: canonical routed authority and applicable governance only. Ran: source reads, source-byte/SHA-256 accounting and `tools/agents/context_report.py`; no model, binary, production workflow, numerical fixture, Cargo test or comparator was run. This is a requirements answer, not implementation acceptance, production qualification, calibration, or adoption approval.

Reader: `/root/surface_iteration5`. Assigned source freeze: `b3b7b57f4` (parent-provided identity; no Git/history consulted). Actual consulted source identities are the complete SHA-256 values in the ledger below. Requested/configured/effective runtime model, effort, delivered tokens, quota and workflow-wide telemetry are UNOBSERVED except for the supplied role/task text. I wrote only this assigned report and my own `/tmp/lse5*` measurement files. The package and handoff authorize selective candidate-v32 reading for this exercise only.

The question is: identify surface-energy/soil coupling rules for a snow-free covered forest tile, including equilibrium-zero versus finite-capacity, first-node transfer, immutable water, and ingress energy. Snow-free does not establish that litter is unfrozen. Therefore the answer distinguishes the liquid V1/V2 domain from the admitted V3 frozen-litter successor and also reads the represented-snow/terminal boundaries that delimit them. No actual tile configuration, forcing, water snapshot, state, receipt chain, binary, run inputs or accepted flux operands was supplied. All required concrete execution evidence remains missing; no numerical result is inferred from a named fixture.

## Authority route and scope

All chapter references below are relative to `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/`; external `SC-*` references name their complete canonical contract in the parent contracts directory. `INV-100`, for example, means `INV-LANDSURFACEENERGY-100`, not a source-line alias.

The entry's **Surface/soil rules → science review → surface-energy** route selected complete `interface`, `common-details`, `surface-energy`, `soil-coupling`, `water-vapor`, `solve-boundary`, `terminal-support`, and `litter-phase`. Exact soil/retained-ingress storage and accepted receipt origin expanded to complete `soil-custody`, `surface-custody`, and the `map-custody#handoff`/`#pending` joins; I voluntarily read the entire map chapter. All selected chapter introductions/dependency tables were inspected. Complete external contracts were required and read: SC-VEGETATION (canopy owner), SC-COUPLEDTIME (support/transaction), SC-SNOWENERGY (represented-snow and terminal boundary), SC-SURFACELIQUID (water, phase, exact energy custody), SC-WATBAL (sole WB14 partition), and SC-SNOWFREEZE (retained rain-temperature provider). External whole-contract costs were not silently narrowed to the first referenced invariant.

This is physical rule selection and accepted-operand custody, not full evaluator implementation, V10/V11–13 numerical branch review, optimization-equivalence review, source-provenance adjudication, enforcement mapping, gap/promotability adjudication, or executable capture/experiment qualification. Those would additionally trigger `nonlinear-solve`, `dependency-replay`, the applicable complete `audit-details` sections, and/or `qualification` plus its frozen protocols. Their absence is not a waiver for those tasks. This answer preserves their no-promotion boundary and does not claim their predicates have passed. No prior exercise, intake, review artifact, archived candidate, old LSE monolith, parent validation result or other agent answer was consulted. Embedded authority, including the historical qualifications and amendments in required complete external contracts, was read as canonical source, not as an exercise answer.

## Physical rules and owner boundaries

**1. Select the regime before evaluating physics.** `surface-energy#selected-sources-and-domain`, `#exact-ownership-and-state`, `soil-coupling#INV-LANDSURFACEENERGY-100/103`, and `terminal-support#INV-LANDSURFACEENERGY-154` require explicit immutable snow state, one ground class per tile, complete authenticated configuration/forcing/state, positive finite area, interval and neutral-domain wind. V1 requires snow absent at both endpoints, no terminal-snow payload, and liquid/unfrozen surface water and soil. Calm/nonneutral wind, snow, terminal snow, frozen or thawing material, missing liquid-temperature lineage and multiple surface classes reject before constitutive calculation. V2 imports V1 with its V10 canopy specialization; a covered forest tile uses its actual `forest_litter` class and vegetation owner, not a generic bare-soil surrogate. V3 admits only the specified snow-free forest-litter phase extension; it does not extend bare-mineral, ponded, frozen-soil, represented-snow or terminal-snow physics.

Hydrology exclusively owns ponded/depression water, litter liquid/ice and every soil-layer liquid/frozen mass. LSE owns one surface thermal node per tile and no water amount. The dry surface body and positive surface water are isothermal at that node. Soil thermal exclusively owns all ordered soil temperatures and enthalpies. Vegetation owns canopy physiology. The older generic `M_l` bookkeeping labels in `common-details` do not create a second mutable LSE water store on the explicit V1/V2/V3 owner path. Strict six `lse_v1_*_schema.json` machine surfaces and model/configuration/state/transaction digests are part of model identity; an executable default cannot fill a missing scientific value. Unknown, extra, duplicate, stale, wrong-owner, nonfinite or invalid fields reject.

Fractions are positive, finite, unique by tile ID and sum to one under the topology tolerance. A local tile amount becomes one OFE's horizontal-ground amount by multiplying by `f_t` once (`INV-107`). “Stand-ground” is that OFE, never a routed-hillslope sum. Preserve source and destination OFE and actual area bases on routing. Generic positive-inward energy uses surface `G` and soil `-G`; the explicit conduction equations below instead name **positive-downward** `G_s1`, so surface `-G_s1` and soil `+G_s1` are the same exchange, not conflicting signs.

**2. Finite heat storage and equilibrium-zero are distinct exact branches.** `soil-coupling#surface-humidity-surface-enthalpy-litter-and-soil-heat`, `INV-103`:

```text
T_ref = 273.15 K; C_w = 4218 J kg^-1 K^-1
U_s = (C_dry + W*C_w)*(T_s - T_ref)
```

`C_dry >= 0` is configured areal dry heat capacity and `W >= 0` is the immutable hydrology operand during a solve. `finite_capacity` requires `C_dry+W*C_w > 0`. Accepted ending enthalpy is the physical LSE state; ending temperature is derived with the hydrology candidate's ending water mass. Any retained temperature warm start must be bit-identical to that derived candidate value and is not an adjustable second physical state. `equilibrium_zero` requires exactly `C_dry=W=U_s=0`; storage difference is exactly zero but `T_s` remains an algebraic energy unknown. No other zero-capacity case is admitted, and an arbitrary small capacity or a retained temperature cannot replace this branch.

For forest litter, the admitted liquid equations are:

```text
h_ul = 0.5*(1-cos(pi*W_l/W_l,max))
q_l = h_ul*q_sat(T_s,p) + (1-h_ul)*q_recipient
v_l = rho_a*(q_l-q_recipient)/r_l-c
lambda_l = 0.1 + 0.03*W_l/(rho_w*dz_l)
C_dry = dz_l*rho_ld*c_ld; rho_w = 1000 kg m^-3
```

Require `W_l,max>0`, `0<=W_l<=W_l,max`, and finite valid thickness/dry density/specific heat and all downstream thermal domains. For this covered tile `r_l-c` is the admitted ground-to-canopy resistance. Litter blocks direct mineral-soil evaporation and upward capillary supply during the interval. Overflow stays hydrology-owned current ingress. A wet litter surface is not permission to use the positive-ponded bare-mineral humidity branch. The CLM bare-soil formulas and their narrowly bounded humidity normalization apply only to that separate admitted class, not beneath this litter.

**3. First-node transfer is one current coupled Crank–Nicolson exchange.** `soil-coupling` same marked definition; `INV-100/103/106`; `OBL-C-003`:

```text
g_s1 = 2/(dz_s/lambda_s + dz_1/lambda_1)
g_k,k+1 = 2/(dz_k/lambda_k + dz_(k+1)/lambda_(k+1))
G_s1 = g_s1*(T_s-T_1)
G_k,k+1 = g_k,k+1*(T_k-T_(k+1))
bar(G) = 0.5*(G_begin+G_end)
C_1*(T_1,end-T_1,begin)/dt = bar(G_s1)-bar(G_1,2)
C_k*(T_k,end-T_k,begin)/dt = bar(G_k-1,k)-bar(G_k,k+1)
C_N*(T_N,end-T_N,begin)/dt = bar(G_N-1,N)
```

Require `N>=1`, exact ordered nodes, finite positive layer thickness, conductivity and areal heat capacity; surface thickness/conductivity are positive and use litter `dz_l,lambda_l`. The lower flux is exactly zero. With `N=1`, only `bar(G_s1)` appears on the right. Surface loss and first-soil gain use the identical accepted flux once; deeper fluxes cancel internally. No independently calculated second flux, duplicated LSE soil temperature, soil phase change, frozen/thawing-soil calculation, tile-to-OFE alias or post-solve temperature replacement is permitted.

The beginning endpoint is branch-sensitive. Finite capacity derives beginning surface temperature from authoritative beginning `U_s,W,C_dry`; the end uses the current trial ending surface temperature. Equilibrium-zero has no physical beginning surface temperature: use the **current algebraic trial `T_s` at both endpoints**, pairing it with beginning and current ending soil temperatures respectively. A caller's numerical warm start must never become the beginning physical boundary. Alternate-warm-start tests must expose that distinction.

**4. Covered radiative and turbulent exchange uses current component temperatures and one shared canopy-air node.** `surface-energy#shortwave-and-reciprocal-longwave`, `#neutral-turbulent-heat-and-vapor-network`, `INV-101/102`, together with SC-VEGETATION's V8 coupled-ground amendment and V10 specialization:

Ground-class VIS/NIR albedos are the full-column lower boundary of the unchanged V7/V8 two-stream shortwave solution. Terminal direct/diffuse VIS/NIR reaches ground once; reflection traverses the canopy, not directly atmosphere. Unit longwave emissivity/no longwave reflection is the selected model. For each occupancy, `tau=exp[-0.8*Omega*(LAI+SAI)]`, with `0<Omega<=1` applied once. The downward/upward recurrences use the current component emissions and ground `Lup_n=sigma*T_s^4`. Components are ordered sun leaf, shade leaf, wet surface, dry stem; with exact emissive areas, `w_j=a_j/sum(a_j)`, their net is `w_j*(1-tau)*(Ldn+Lup_below)-2*w_j*(1-tau)*sigma*T_j^4`. Zero total emissive area has `tau=1` and exact-zero component terms. Ground net is `Ldn_n-sigma*T_s^4`. Bulk canopy temperature, repartitioning bulk longwave, prescribed upward ground longwave and stale ground temperature are rejected aliases.

The zero-storage covered air node `(T_c,q_c)` receives each actual canopy component and ground once:

```text
H_s = rho_a*c_p*(T_s-T_c)/r_h
v_s = rho_a*(q_s-q_c)/r_v
R_Tc = sum(H_j)+H_s-H_c->atm
R_qc = sum(v_j)+v_s-v_c->atm
```

`rho_a,c_p` are the V8 forcing-derived moist-air quantities. Ground/litter uses ISBA-MEB's exact neutral `r_g-c`, with `psi_H=f_hv=1`; its frozen constants are `phi_v=2`, `z0g=.007 m`, `chi_L=.12`, `u_l=1 m/s`, `l_w=.02 m`, `nu=1.5e-5 m2/s`, `kappa=.4`. In particular `Re=u_l*l_w/nu`, `c_d=1.328*(2/sqrt(Re))+.45*((1-chi_L)/pi)^1.6`, `d=1.1*z_hv*ln(1+(c_d*LAI)^.25)`, `u_star=kappa*u_ref/ln((z_hv-d)/z0v)`, `K_hv=kappa*u_star*(z_hv-d)`, and `r_g-c=z_hv/(phi_v*K_hv)*[exp(phi_v*(1-z0g/z_hv))-exp(phi_v*(1-(d+z0v)/z_hv))]`. Require `LAI>0`, `z_hv>d+z0v>z0g>0`, `z_ref-d>=z_hv-d>0`, and finite positive logs/exponentials/resistances. Canopy-to-reference uses the configured displaced neutral log paths, with `z_ref>d+max(z0m,z0h,z0q)` and positive roughnesses. Heat and vapor retain separate semantic operands even when resistance values coincide. No reference-air ground exchange beneath canopy, per-occupancy duplication of ground, omitted ground feedback, bulk producer aggregate, hidden wind floor, stability substitute or agricultural PMET donation is admissible.

## Immutable water, signed vapor and ingress energy

**5. The two-pass water transaction uses original beginnings and a single authorization.** `water-vapor#immutable-beginning-water-transaction-and-current-ingress`, `INV-105/107`, `OBL-P-001..004`, `OBL-C-001..003`, SC-SURFACELIQUID's algorithm and SC-VEGETATION water protocol:

1. Freeze the hydrology snapshot **before** all current rain, runon, throughfall, initial and secondary canopy drainage, stemflow and litter overflow. Root and ground demands share only immutable beginning surface/litter and soil-layer pools. “Uncapped” means no hydrology-owner caps; it does not remove canonical physiology or constitutive restrictions.
2. Solve the complete current-temperature canopy/ground/soil system from immutable beginning vegetation, LSE and soil-thermal state. Emit root/surface/litter/soil requests with transaction, OFE, tile, occupancy, surface, source/layer and basis identity. Hydrology authorizes the complete same-snapshot request set once, before ingress.
3. Rebuild the **complete** system from original beginnings with those fixed source-specific caps. `cap_rate<=q_law`, including equality, selects the cap branch and zero generalized derivative. Gas exchange, canopy/surface energy, shared air, hydraulics and soil thermal all re-solve. Check finalized use `F=f_t*q*dt` independently as `0<=F<=A<=D`; authorization is not actual use. A full-supply numerical seed still requires re-evaluation on original final-pass physics and fixed caps, not potential-state continuation.
4. Only after final acceptance, construct owner candidates: debit actual finalized beginning-store use and credit condensation, then partition final current ingress once. Final canopy release may neither shrink nor enlarge the prior authorization. There is no second authorization, current-ingress donation, scalar stress repair or later potential-pass water installation.

SC-SURFACELIQUID's explicitly bounded floating proportional-allocation normalization is not a general clamp: raw joint OFE/inverse-tile allocation must satisfy supply; only its finite roundoff envelope `1e-14 + 64*epsilon*sum_abs` admits a common monotone downscale (bounded bisection, at most 64 iterations), keeping each positive amount positive. Per-row fixes, enlarged supply and material overdraw remain invalid. Similarly its retained-capacity dust rule applies only to the declared positive raw remainder bounded by `1e-14+64*epsilon*(|f_t*W_max|+|f_t*W|+|excess|)`: retain zero and route the complete corresponding mass and enthalpy, preserving storage, rather than silently deleting water or heat.

**6. Every signed vapor exchange carries sensible plus latent energy and the matching phase mass.** `water-vapor#signed-vapor-and-liquid-enthalpy`, `INV-104`, `OBL-C-001`:

```text
h_l(T) = 4218*(T-273.15) J/kg
L_v(T) = 2.501e6 - 2369*(T-273.15) J/kg
Q_v = v_s*[h_l(T_s)+L_v(T_s)] W/m2 (positive outward)
(U_pre-U_0)/dt = R_sw+R_lw-H_s-Q_v-G_s1
```

Positive evaporation requests `max(v_s,0)*dt` from the named source, and only finalized use debits it. Negative `v_s` produces no withdrawal request: hydrology credits exactly `-v_s*dt` and the energy sign credits the surface. `U_pre` is post-final-vapor/pre-current-ingress and uses the corresponding hydrology-owned mass. No clipping, absolute value, latent-only energy, condensation without mass, authorization-as-use or second SC-EVAP latent/mass debit is valid. The generic `LE*dt=-L_v*m_evap` is the latent-only join for its liquid-evaporation branch; it cannot replace this total vapor-enthalpy term or erase sensible transport.

**7. Current ingress carries actual upstream parcel energy to its actual recipient.** Same `#vapor`/`#water` definitions, `INV-106`, `OBL-C-002/003`:

```text
Q_i = m_i*h_l(T_i)
T_mix = T_ref + sum(m_i*h_i)/(C_w*sum(m_i))  [positive total mass]
U_s,end = U_pre + sum(Q_retained_ingress)
E_soil,1,end = E_soil,1,pre + sum(Q_infiltration)
```

A zero-mass crossing has zero energy and **no temperature**. Rain temperature is exactly the retained `hydrometeor_temperature_c+273.15` from `openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity` on active `harder_pomeroy_hourly`; LSE neither reruns nor partially transcribes it. SC-SNOWFREEZE `INV-064/065/075` retains finite humidity guards, hourly coefficient choice, explicit nonconvergence, and phase closure `hrrain+hrsnow/10=active hourly precipitation`. Only documented dewpoint-derived supersaturation may normalize to exactly RH=1; negative/nonfinite/zero-saturation or otherwise invalid humidity rejects. Absent/empty and explicit Harder–Pomeroy select the active phase default; unknown selectors fail, and legacy RST remains explicit rollback/test context, not a substituted rain-temperature source.

Runon carries the typed temperature and enthalpy of the accepted upstream outlet parcel. Air, soil, freezing-point and destination-surface temperatures are prohibited missing-value substitutes. Hydrology chooses the exact source parcel or conservative mixture for infiltration/runoff; it remains the sole partition owner. Soil thermal credits first-node infiltration energy and derives its new temperature. Retained ingress updates surface enthalpy and ending temperature with ending capacity. Neither updated temperature feeds back into already accepted same-interval H/LE/G. Routed upstream and downstream mass/energy are the same transfer on their stated bases; where both are per OFE ground, convert source amounts by `A_src/A_dst` once and rekey rather than asserting unequal-area per-area numbers are equal. Outlet runoff is the named external sink. No heat-only crossing, dropped infiltration heat, independently reconstructed runoff temperature, duplicate tile fraction or cross-OFE aggregate masquerading as one stand is acceptable.

The physical conversion precedes exact energy decoding: for retained groups, the prescribed binary64 OFE-group sum and division by `f_t` occur before exact dyadic decoding; for a spill, OFE mass is `f_t*m_tile` and OFE energy is that accepted mass times its specific enthalpy. Do not replace those accepted operations with an exact-rational alternative and call it the same operand.

SC-WATBAL's **WB14 Infiltration and Hyetograph Coupling Addendum** and SC-SURFACELIQUID continuation require the real single Green–Ampt/Mein–Larsen owner per OFE child for the combined ingress, preserving parent cumulative state/cursor. No copied proxy, supplied infiltration number, per-parcel partition, or whole-day replay replaces it. Inputs include `ninten`, `nbrkpt`, breakpoint times/intensities, `ssc`, `dg`, `thetdr`, `thetfc` and the active disturbed-forest `solwpv`, `ksatadj`, factor/recovery, `lkeff`, current first-two-layer water/upper-limit geometry. Default Ke is `ssc`; only the active canonical forest selection applies its saturation-dependent 9001 or >=9002 SR/9003 branches. Invalid active domains, nonmonotone/negative hyetograph, nonfinite values, or integrated-liquid mismatch yield WB14 typed missing/nonfinite/domain failure before writeback, not seeded infiltration or parameter repair. Shared parents are 1800 s with 48/day and exact child support/cursor continuity; source attribution preserves canonical remainders, and last-child completion advances once. Under represented snow, an authenticated inactive-prefix chronology proof cannot run WB14, create a physical receipt, reseal state or increment the physical child cursor; the first actual snow-free child has ordinal zero (SC-SNOWENERGY `INV-087`).

The only LSE reference-temperature publication normalization is `INV-130`, `water-vapor#version-9-exact-liquid-reference-state-representation-amendment`: before covered-canopy liquid ledger/release or mass-weighted Stage-3 terminal-liquid publication, exact bits `0x4071126666666667` map to `0x4071126666666666` (273.15 K), a `2^-44 K` offset, before computing/persisting zero specific enthalpy. Exact reference is unchanged; below-reference retains its covered-canopy-snow rejection; second upward neighbor and higher are not normalized. It changes no mass, wet fraction, residual, support, storage or phase rule and is not general temperature snapping.

## If litter phase is present or unspecified

**8. V3 phase is a post-solve, phase-specific operation, not snow or frozen-soil physics.** `litter-phase#v3-state-phase-free-solve-and-signed-vapor`, `#bounded-kinetic-phase-and-fusion-energy-closure`, `#ingress-wb14-identity-restart-receipts-and-failure-posture`, `INV-140..149`:

The surface owner holds nonnegative litter `W_l,W_i`; `W_i` is water-equivalent ice, neither represented snow nor soil `frozwt`. `U=(C_dry+W_l*C_w+W_i*C_i)*(T_l-T_ref)` is sensible energy. Constants are exactly `T_ref=273.15 K`, `rho_w=1000`, `rho_i=920 kg/m3`, `C_w=4218`, `C_i=2106 J/kg/K`, `L_f=333700 J/kg`, `tau_ice=3300 s`, `W_i,max=.85*rho_w*dz_l`. R-156 Appendix A and retained SURFEX v8 select `freeze-melt`; `.85*rho_i*dz_l`, 917, 273.16, wrong latent heat or fitted timescale are rejected.

The full V2 solve first runs phase-free: no fusion, phase-updated capacity/temperature or phase transfer enters Newton, active branches, authorization or convergence witnesses. Use immutable beginning phase masses. `p_i=0` for exact empty total, otherwise `W_i,0/(W_l,0+W_i,0)`; `h_ul` and `h_ui` are the half-cosine pool/capacity functions. The V3 vapor formulas are specifically

```text
v_l,raw = (1-p_i)*rho_a*(h_ul*q_sat_liquid(T_l,p)-q_recipient)/r_l-c
v_i,raw = p_i*rho_a*(h_ui*q_sat_liquid(T_l,p)-q_recipient)/r_l-c
Q_v,l = v_l*[C_w*(T_l-T_ref)+L_v(T_l)]
Q_v,i = v_i*[C_i*(T_l-T_ref)+L_s(T_l)]
```

Both use saturation over **liquid** water. These are not V1's interpolated `q_l` formula. Distinct positive components use only their named beginning pool and distinct negative components credit only their own phase without an availability cap. Preserve raw/final signed component and energy lineage through receipts before any air-side aggregation. No total-store cap, phase borrowing, ice saturation substitute or daily WB17 duplicate is allowed.

After phase-free final acceptance, install `W_l,*=W_l,0-dt*v_l`, `W_i,*=W_i,0-dt*v_i` and the accepted `U_*,T_*` once; independently reconstruct their sensible-plus-latent ledger. Require `0<=W_i,*<=W_i,max`, `W_l,*>=0`, positive finite thickness and ending heat capacity. Then

```text
M_warm/cold = rho_i*C_i*dz_l*max(±(T_*-T_ref),0)/L_f
m_melt = min(W_i,*, (dt/tau_ice)*min(M_warm,W_i,*))
m_freeze = min(W_l,*, W_i,max-W_i,*, (dt/tau_ice)*min(M_cold,W_l,*))
m_phase = m_freeze-m_melt
W_l,phase = W_l,*-m_freeze+m_melt
W_i,phase = W_i,*+m_freeze-m_melt
U_phase = U_*+L_f*m_phase
T_phase = T_ref+U_phase/(C_dry+C_w*W_l,phase+C_i*W_i,phase)
```

Outer availability/capacity bounds apply even when `dt>tau_ice`; there is no maximum support introduced by the kinetic rule. At exact reference both phase transfers are zero. Phase-only `U-L_f*W_i` is conserved, with equal/opposite liquid/ice transfer. Pre-phase capacity or an additive `T += L_f*m/C_*` leaves unowned energy and rejects. This ending state only seeds the next support: no same-support re-solve, reauthorization or instantaneous equilibrium substitution. Only afterward may liquid current ingress and liquid-only WB14 proceed; ice cannot drain/infiltrate/run off/satisfy WB14 or mutate soil.

Checked V2→V3 migration copies scientific values bit-identically and joins explicit surface-owner V2; checked surface V1→V2 migration sets ice exactly zero, whereas a new V2 seed may carry explicitly supplied valid ice. Temperature cannot invent initial ice. Production downgrades and mixed/stale identities reject. Model tag is `OPENWEPP_SNOW_FREE_LSE_V3`; phase receipt tag is `OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1`. Receipts bind exact support/duration, keys/bases/owners, beginning state, separate raw/final vapor and energies, post-vapor state, constants/bounds/transfers, phase end, ingress/WB14 joins and independently reconstructed mass, sensible/fusion energy and `U-L_f*W_i`. Restart requires all model/contract/config/state/phase/receipt tags and digests before evaluation. Error order preserves `LSEB-E-045` identity/domain/migration, `046` phase-vapor, `047` phase/fusion/capacity and `048` chronology/receipt/restart/rollback, including rollback for any later owner/publication failure.

**9. Excess post-phase liquid must become an authentic spill before owner sealing.** `litter-phase#spill`, `INV-156`, `OBL-C-011`; SC-SURFACELIQUID `INV-028`:

Keep the raw phase receipt immutable. Reconstruct `W_raw,U_raw,W_i,end,C_raw,T_raw` in its defined order. At/below `W_l,max`, spill is exact positive zero and raw=retained. Above capacity, checked finite binary64 operations are `m=W_raw-W_l,max`, `h=C_w*(T_raw-T_ref)`, `Q=m*h`, `W_retained=W_raw-m`, `U_retained=U_raw-Q`, and temperature from the retained ending capacity. Require positive spill, bounded retained liquid and positive capacity. The **second subtraction**, not `min`, clamp, tolerance snap or deletion, defines the remainder. Reconstruct raw mass and sensible enthalpy from retained plus spill.

`LitterPhaseCapacitySpillV1` binds phase-receipt SHA, config/owner/transaction, child support/key/basis, capacity, raw/retained endpoints and spill operands. Exact surface energy adds one negative `LitterPhaseCapacitySpillEnergy` after phase-free and fusion operands. Checked `f_t*m` becomes one internally produced `LitterPhaseOverflow` parcel on full child `[0,dt)`, with energy from its OFE mass and the same `h`; it enters ordinary ingress/WB14 once. Do not assign it to a rain hour, label it condensation, reclassify ice/fusion as runoff, or cancel the debit if some spill is retained: retention returns by a separate normal retained-ingress receipt. Missing/duplicate/wrong phase, key, support, basis, mass, sign, temperature, enthalpy or WB14 join fails before installation and rolls back the complete owner set; it never causes a second phase/flux solve.

**10. Heterogeneous finalized uses must not replay native vapor.** `litter-phase#resource-join`, `INV-157`, `OBL-C-012`; SC-SURFACELIQUID `INV-029`: if the batch also contains ordinary surface withdrawals, match each native receipt-derived finalized `F` by full transaction/support/OFE/tile/source/basis identity and exclude it from ordinary resource application. The final row must be bit-identical to receipt-derived **F**, not necessarily authorization A; require `0<=F<=A<=D`. Every unmatched ordinary row must be authenticated, aggregated in complete `GroundWaterKey` order and debited once as checked `F/f_t` from the exact phase-adjusted V2 owner. Zero ordinary rows is identity. Produce one `SurfaceLiquidV2HeterogeneousResourceJoinV1` resource candidate and one ingress. Preserve native ice, enthalpy high/carry/receipts, phase and spill bytes. No wholesale resource replacement, legacy-beginning rebuild, ordinary new energy operand, spill-as-finalized-use, phase replay, capacity repair or second ingress is valid. Failed partition/receipt/debit/ingress/final join/publication restores all owner, cursor and receipt beginnings.

## Exact storage, accepted-map custody and chronology

**11. Small real energy credits are stored exactly, not rounded away or forced into a ULP.** `soil-custody#exact-soil`, `surface-custody#surface-custody`, `INV-150/151`, the associated exact-storage obligations: soil energy is exact `E_k=exact(H_hi,k)+R_k`; surface energy is `U_t=exact(U_hi,t)+R_U,t`. High terms are finite binary64 and carry is a normalized signed dyadic: nonzero sign ±1, odd hexadecimal significand without leading zero, integer power of two; zero has the sole carry form `(0,"0",0)`. Decode each finite **accepted physical** energy operand exactly, accumulate with arbitrary-precision integer arithmetic, round the exact total **once** nearest-even to a finite high, and retain exact residual carry. Reject overflow/noncanonical wire form/identity/receipt/reconstruction failure (`LSEB-E-049/050`). Keep the existing high's signed-zero treatment, including no-op migration; do not normalize it merely because carry has one zero form.

Soil credits name internal, top-boundary and infiltration energy with correct ordered layer/source receipt. Surface credits name accepted phase-free, fusion, retained-ingress, and the later explicit negative spill. Missing, duplicated, stale or fabricated operands cannot be repaired by a producer residual. Surface V2/V3 frozen fields mirror `U_hi` bit-for-bit. Carry is persistent accounting, never a feedback term for a second flux/temperature/phase correction; physical temperature uses the high term under the admitted capacity convention. No compensated floating sum, repeated rounded addition, subnormal flush, tolerance deletion, `nextafter`, residual redistribution or rational substitution for prescribed physical conversion is admitted.

Migration into exact soil state copies its V1 high bit-identically with zero carry; no reconstructed temperature/high or production downgrade. Exact surface identity joins its frozen physical/mirror owner and authenticated topology. `INV-153/158` retain parent-local receipts and topology ordering: partial children evolve accepted high/carry and receipt lineage while resident predecessor identity is retained; only final parent stamping/transaction installs the joined owners once. OFE IDs are opaque: configured topology rank orders the complete vector, never numeric/lexicographic ID or receipt-hash order. Restart and prefix/head/tail/chain joins must reproduce all exact owners and receipts.

**12. A candidate beginning is not a published soil owner, and final resealing does not rerun physics.** `soil-custody` candidate-beginning amendment (`INV-155`, `OBL-C-010`) permits a private read-only candidate continuation tied to the original prepared resident predecessor/next support. It cannot mint a live owner, restart record or accepted credit receipt. Final preparation independently replays all ordered accepted operands and exact ending credits, validates the candidate continuation and selected ending, then seals/installs once; an outer transaction rebind cannot launder a foreign predecessor.

`surface-custody#reseal`, `INV-160`, `OBL-C-015` permits the fully validated provisional envelope to produce a private move-only, live-revision proof of exact non-slab bindings. The final envelope may differ only in the admitted slab identity. It consumes that proof once without physical re-execution; actual physical owner bytes remain identical, zero provisional owner construction/publication and exactly one final construction are independently demonstrated. The proof is non-wire/non-restart; fresh restored bytes receive full validation/physics as governed. Any changed endpoint, owner, support, config, transaction, stale proof or forbidden difference fails typed, with no fallback reconstruction path.

`map-custody#handoff`, `INV-159`, `OBL-C-014` permits an unchanged fully validated immutable resident/resource revision to move through a private nonserializable handoff bound to schema/model/configuration, complete digest, transaction/predecessor/support and exact history prefix/head/tail/chain. Appending validates the new tail; mutation consumes proof and fully validates the successor. A byte/digest cache is revision-bound derived evidence only. Restart/checkpoint, external/durable bytes and untrusted-executor returns still perform full parse, canonical reconstruction and semantic validation, including required frozen-litter prefix replay. This answer does not claim V30 dependency-replay optimization correctness.

`map-custody#pending`, `INV-161`, `OBL-C-016` preserves one physical prefix per charged map, with role/regime orthogonal. Ordinary maps execute actual LSE/hydrology/soil/surface/WB14 physics; native represented-snow maps execute snow/LSE/soil and validate byte-retained inactive litter/WB14. `Initial` stops after complete physical/custody validation; later roles yield non-Clone pending values. Outer nonclosure consumes pending into history without error; dependent-only nonclosure consumes it into typed adaptive rejection without history/constructors. Full closure consumes the **same** pending physical prefix once into V8, persistent/material vegetation, BGC, ending joint and complete-envelope construction as `FinalAccepted`. No additional final physical map, promotion of completed nonfinal output, map publication, duplicate disposition or constructor-error fallback exists. Wrong role/ordinal/order fails AdaptiveRefinement before LSE; wrong identity/regime/support/topology/forcing/predecessor/custody fails Identity before mint/exposure; physical and downstream errors retain their specific typed cause. Map-level publication fails `ERR-CT-018 PublicationState`; only selected composed parent atomic commit publishes. Test-only forced-complete comparisons must establish physical-prefix equality and zero nonfinal/one final constructors, not simply count calls without validating endpoints.

**13. Represented snow and terminal transition are required boundary checks, not alternate snow-free shortcuts.** `soil-coupling#version-8-persistent-snow--soil-boundary-amendment`, `INV-124..126`; `terminal-support` `INV-114..123/154`; `map-custody#accepted-map`, `INV-152`, `OBL-C-007/009`:

While snow is represented, the bottom Stage-3 thermal volume joins **first ordered OFE soil node**, with `R=dz_sb/(2*lambda_sb)+dz_1/(2*lambda_1)`, `g=1/R`, and beginning/current-ending CN flux positive snow→soil. It is an OFE/lane interface: no tile selection/subset averaging/fraction weighting or per-tile repetition. Beginning temperatures come from sealed beginning snow/soil; ending temperatures come from the current coupled candidate, not stale or post-solve evaluation. One reconstructable `SnowSoilHeatReceiptV1` binds support/duration, lane/OFE/basis, topology/config, node IDs, all four resistance operands, endpoint pairs/fluxes, mean flux and beginning/ending owners. Snow consumes its negative and soil its positive once. Retain the actual consumed heat; installed-endpoint reconstruction must meet `TOL-SNOWENERGY-005` (`1e-9 J/m2`, `1e-8 K`) before resealing to installed identities. Retry within the canonical fixed-point cap then typed `LSEB-E-044`/`SNOWENERGY-E-SOIL-HEAT-001`; do not replace applied heat or relax the distinct `1e-6 J/m2` physical ledger tolerance.

Native represented-snow dispatch has both residents, nonempty native Stage-3 tile set, empty active litter set, and its own exact optical plus `Stage3SnowCovered` lower-boundary receipts. Snow is the sole atmospheric ground surface. Litter V3/V4 vapor, phase, storage arithmetic, ingress and WB14 are inactive and their bytes/predecessor/receipt chains remain unchanged. A second inner legacy physical envelope is forbidden. Outer Stage-3 continuous admission uses `TOL-SNOWENERGY-007`; it never replaces LSE's own algorithm or relaxes exact branch/topology/owner/high-carry/heat-transfer identity.

The older default-off `terminal_receiver_v1` text describes actual-surface selection and a single 0 C sensible-zero parcel on `[wall_t*,wall_end)`; zero remainder performs no physical solve, fusion is not soil G, and censored schema-v8 payloads remain inadmissible to V1 (`OBL-P-004`). Its old bisection/continuous-root, evaluation-only and lasting-CoE ownership statements are not current production-model mechanics: SC-SNOWFREEZE **Adaptive Stage-3 lifecycle and batch successor**, `INV-103..106`, and SC-SNOWENERGY's active adaptive amendment supersede those named mechanics. Current disappearance is exact zero ice at an accepted microstep endpoint, with one topology-bound produced/unconsumed liquid parcel per terminating lane, one event group for same-tick lanes, one consumption through SurfaceLiquid/WB14 and actual snow-free remainder. Authorized solid precipitation may reappear as canonical Stage-3 state; atmospheric vapor cannot recreate snow. Preserve active same-step phase/deposition custody, complete net energy and liquid closure; no CoE/legacy-terminal/alternate-model fallback or independent lane integration followed by owner merge.

All receiver radiation/albedo, geometry/roughness, turbulence, evaporation/condensation, rain heat, soil heat and storage are rebuilt for actual successor owners/forcing. No snow temperature/albedo/roughness/flux/residual, terminal unallocated energy, or post-tick snow liquid is reused. Cross-midnight support closes/opens days once; absolute support differs from transaction order. General terminal error chronology is schema/identity → snow forcing/domain → integration/closure → terminal receipt debit/credit/consumed join → actual receiver selection/forcing → LSE → surface/WB14 → soil/frost → routing → cross-owner commit → rollback validation. The first cause remains primary; a rollback diagnostic cannot mask it and poisons an uncommittable transaction. Active adaptive restart occurs at sealed operation boundaries, retains accepted controller/support/cursor/complete-owner/topology/pending parcel/provider/clock/receipt history, stores no rejected trial, and must equal uninterrupted execution exactly.

**14. Positive support admission is an exact pre-Newton domain check.** `terminal-support#support`, `INV-116..123`; SC-COUPLEDTIME support/event algorithm:

For the declared covered-forest adopter, `OPENWEPP_SNOW_FREE_LSE_V1_SUPPORT_POLICY_V1` has exact minimum `60_000_000_000 ns`. Every physical V11 invocation first validates `LseSupportAdmissibilityReceiptV1`: parent/segment/slab, absolute half-open support, requested ticks, exact duration bits, model/config, beginning LSE/soil identity, tolerance/numerical policy, exact minimum and digest. Identity uses canonical coupled-time projection; ordinal/ticks are canonical decimal, and the digest is canonical JSON with empty self-hash under its specified domain tag. Wrong join is `LSEB-E-042`; below minimum is `041` before Newton with no candidate and exact rollback. Exactly minimum is an ordinary solve; one ns less rejects. Zero successor support is a custody transition with no Newton. No retry-at-minimum, silent floor, longer-result scaling, frozen state, relaxed tolerance or V10 mutation is allowed. Nanosecond wire validity is not physical solver admission. The policy does not license every other surface profile or guarantee global convergence for every larger support.

A successor also requires consumed event-boundary coalescing custody and post-event-only operands (`043` on wrong regime). Coupled-time uses the maximum required minimum of active participants; admissible coalescing must satisfy event-time and independent snow/liquid mass and energy bounds and leave both neighboring supports zero or at least the relevant minimum. Deterministic ranking is displacement, normalized-error sum, then earliest tick; exact-zero tolerances require exact equality. No admitted candidate means typed coalescing failure/rollback, not a time gap or wrong-regime flux. Current Stage-3 positive supports are exact 60-second quanta: direct/composed children exactly tile the candidate, rejection restarts complete immutable beginnings, acceptance installs composed child-2 ending, and the one-quantum floor has no fabricated split. Stable ordinary supports must also accept substantially larger steps. The controller-only LSE energy bound `1e-6+5e-3*max_abs`, soil-energy bound `1e-6+1.5e-2*max_abs` (J/m2), and temperature bound `1e-2+1e-8*max_abs` K are not local residual, physical-closure, phase or exact-representation tolerances. Prior 0.6-second-dependent evidence does not validate this floor.

## Ordered solve, evidence requirements and first-answer check

`solve-boundary#solve`, `INV-108..110` fixes physical unknown/residual ordering by tile ID, occupancy rank/ID, complete vegetation blocks, shared air, surface and soil nodes 1..N. Joint or nested evaluation must converge the complete inner physics at every evaluation; incomplete inner state cannot be a residual. Temperature bounds are 200–350 K and specific humidity 0–0.1 before physics. Its prescribed numerical boundary uses deterministic finite-difference scaling/stencils, LU pivot tie order and cutoff, strict residual descent backtracking and finite iteration cap; it admits no alternate solver after failure. In the selected boundary description the finite-difference scale is `sqrt(epsilon)*max(abs(x),unit scale)`, interior minus/plus or inward boundary sampling; LU cutoff is `64*epsilon*||J||inf`, ties use lowest row; backtracks are `2^-b`, b=0..20, and at most 50 accepted updates. These describe the selected physical-boundary obligations, not verification of additional V10/V11–13 eligibility/scaling branches.

Acceptance requires current residuals plus the prospective step witness: a residual-small point with an oversized/invalid full step may use only the first domain-valid halved prospective step, checking every governed norm; accept the current state, not an applied trial. If that first valid step fails, later smaller steps cannot be used to manufacture acceptance. Intercellular CO2 is not an added step threshold. `TOL-LSE-001/002` require energy residual `1e-6 W/m2+1e-10*max(1,sum_abs)` and water residual `1e-12+1e-9*scale`; governed steps are T `1e-8 K`, q `1e-12`, hydraulic `1e-7 mm`, beta `1e-10`. Small step alone, small residual alone, a stale seed or producer total does not prove acceptance. Full solver implementation/equivalence would require the unselected nonlinear/replay chapters.

Failure precedence is malformed serialization → model/config/state/transaction mismatch → missing/duplicate owner/topology → nonfinite → unsupported regime → constitutive domain → D/A/F → singular Jacobian → backtracking → iteration limit → step/residual acceptance → component closure → control-volume closure → cross-owner join. Retain identity, iteration counts, residual/step norms, active bounds and matrix/failure diagnostics. `water-vapor#errors` maps unsupported domain 030, strict identity 031, radiation/turbulent ownership 032, water D/A/F 033, convergence 034, closure 035, liquid/latent join 036, ground/atomic join 037, ingress ordering 038, condensation 039 and soil owner 040. Later exact/phase/support errors retain their more specific families described above. Every failure preserves complete joined vegetation/hydrology/LSE/BGC/soil/surface/snow/time/receipt/checkpoint state as applicable, including later commit failure. A partial successful producer is not a successful transaction.

The required **inputs and evidence**, concretely, are:

- Exact selector/model/schema/contract/configuration identities, complete OFE/tile/occupancy/node topology and fractions/areas, parent/segment/slab/event/support policies, accepted time/duration bits, predecessor/owner digests and full restart/carry/receipt state. Configuration must supply VIS/NIR albedos, emissive/component areas and clumping, actual V10 physiology/hydraulic parameters, neutral canopy/reference geometry, litter capacity/thickness/dry properties, soil thickness/conductivity/heat capacity, and the explicitly selected thermal branch. None is synthesized here.
- Forcing-derived air density/heat capacity, atmospheric longwave, direct/diffuse VIS/NIR, pressure/humidity/temperature/positive neutral wind with source/exposure lineage; actual immutable beginning water/phase/soil state; original U/T warm starts with their constrained role; retained rain-provider result; upstream runon parcels; final request/authorization/use rows; raw and accepted signed phase vapor; accepted CN endpoints/flux; actual ingress/mixing/WB14 continuation/partition/retention/routing operands. Exact energy reconstruction additionally needs every finite accepted physical credit in its prescribed order, high bits/carry, and receipt hashes, not decimal summaries or producer residuals.
- Independent local then one-fraction-weighted OFE radiation, signed sensible/vapor, surface/soil storage, ground exchange, advection, water and phase reconstruction, with distinct operands chosen so omitted/swapped/doubled aliases cannot pass. Tests must cover dry/wet litter, covered/open heterogeneity, zero shortwave, night, longwave/ground sign reversal, canopy-air ground feedback, equilibrium-zero versus finite storage, alternate warm starts, evaporation/condensation, full/partial/capped and zero-source allocation, concurrent root/ground scarcity, equality-at-cap derivative, rain/runon/infiltration/runoff energy, singular/backtrack/iteration/first-valid-step failures and exact rollback.
- V3 vectors must include empty/all-liquid/all-ice/mixed phases, freezing/melting/exact reference, liquid condensation/ice deposition, separate pool exhaustion/capping, ice-capacity boundary, dt below/equal/above 3300 s, exact 60 s and substantially larger stable support. Reject wrong constants/sign/saturation/capacity, instant-equilibrium or freeze-only substitutes, pre-phase heat capacity, phase donation/current ingress, ice as WB14/soil water, implicit ice initialization, downgrade, ice deletion/regularization, re-solve and partial commit. Spill tests require at/below/above capacity, melt-created spill, exact raw=retained+spill mass/energy, one negative carry operand, checked area/full-child support, retained/infiltrated/routed dispositions and all receipt/identity poisons. Heterogeneous tests prove exact native exclusion, canonical ordinary order and unchanged native/spill/energy bytes.
- Exact-storage vectors need sub-ULP positive/negative credits, cancellation, zero and signed-high-zero, nearest-even ties, canonical dyadics, malformed/nonfinite/overflow/omission/duplication, no-op migration, final replay and restart, partial/final parent chronology and opaque topology permutations. Same-map tests need initial/history/rejection/final physical-prefix equality, final-only constructors, exclusive dispositions, role/order/one-ULP/identity poisons, unpublishability and complete rollback. Native snow tests prove zero inactive litter/WB14 calls/changes, exact optical/lower receipts and one successor at the accepted event; terminal/support tests include start/interior/end/zero remainder, min/one-ns-below, coalescing/refinement rejection, multi-lane simultaneous/different events, cross-midnight, rain on both sides, reappearance and every permitted restart boundary.
- `litter-phase#v3-invariants-and-required-production-vectors`, `INV-149`, and the exact-custody obligations name real consumers `tests/integration/erosion_single_ofe_p61_sediment.rs` and `tests/integration/dff_ws1_native_forest_cli.rs`. Their production selector, installation, persistence/reload and next-support continuation must consume the new owner and prove independent liquid/ice/fusion/vapor/WB14/whole-envelope closure. A synthetic unit vector, source marker, reported residual or constructor counter cannot replace those runs. No p61 actual beginning high/carry/credit operands for support `[176400000000000,178200000000000) ns`, binary/runfile or actual output was supplied, so no p61 oracle, runtime adoption verdict, consumer PASS or amended-floor conformance/performance claim is made.

Before completing this first report I checked each selected applicable marked definition against the answer: state/regime/units and owner separation (1); thermal-capacity/humidity/CN beginning distinction (2–3); current radiative and shared-air operands (4); request→authorization→original-state capped solve→final use→ingress and total vapor energy (5–7); phase-free→vapor→bounded phase→spill→heterogeneous resource→liquid WB14 ordering and exclusions (8–10); high/carry, accepted operand conversion, parent/restart/map-origin and final-only custody (11–12); represented-snow/terminal active-versus-historical mechanics, actual surface reconstruction and exact support (13–14); and local failures, independent evidence and named real consumers above. Every selected universal P001–004 duty is retained: immutable source records/sealed ledger, validation before atomic mutation, branch/support/tolerance/residual lineage without defaults, and no unauthorized schema-v8 terminal import. Nothing here promotes missing execution evidence or waives a required current-scope test. Rust line-count, implementation/regression, external-suite mutation and production qualification gates are NOT APPLICABLE to this report-only write; the scientific tests listed above are NOT RUN, not passed or deferred adoption evidence.

## Complete provenance and measurement ledger

Measurement is structural requested UTF-8 source bytes, including original line terminators, without rendering prefixes. Inclusive line ranges are authoritative; full-source SHA256 is carried by the source identity table and joined to every request by source ID. All full hashes were checked again against the frozen working files when assembling this ledger; no Git/history was consulted. “New” is incremental union coverage and “repeat” is requested exposure overlapping an earlier request, not a claim about delivered tokens. Bootstrap means the initial nine full-file requests; all later requests are expansions.

| Category | Unique requested content bytes | Total requested exposure bytes | Repeated requested bytes |
|---|---:|---:|---:|
| governance/tool | 74148 | 95612 | 21464 |
| LSE | 196206 | 381672 | 185466 |
| external_contract | 2035441 | 2341522 | 306081 |
| All 150 content requests | 2305795 | 2818806 | 513011 |

The LSE union above is the current entry and 11 routed chapter files, not the old monolith. External canonical contracts and governance/tool sources are separately accounted; external expansions are not evidence of reduced total workflow context. The content-request table includes voluntary full chapter reads and all overlap/recovery, even when larger than the minimal scientific answer. The automatic user-provided root AGENTS content is one additional exposure, distinct from the explicit R001 file request; its surrounding prompt, system/developer messages, tool schemas, compaction and message delivery are not measured. Runtime delivered-token, model-token, latency and complete workflow telemetry are **UNOBSERVED**. No ratio from structural UTF-8 bytes is promoted to token or workflow savings.

### Source identities

| Source | Path | Complete source SHA256 | Full UTF-8 bytes | Lines |
|---|---|---|---:|---:|
| S01 | `AGENTS.md` | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` | 9508 | 121 |
| S02 | `docs/specifications/science-contracts/AGENTS.md` | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` | 6532 | 84 |
| S03 | `docs/work-packages/AGENTS.md` | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` | 6642 | 105 |
| S04 | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` | 5304 | 75 |
| S05 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` | 1586 | 22 |
| S06 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `930c682e5c923083683e8a030457c9e5e90e6ca9cd2f70a425ee0aa79c326c72` | 4130 | 65 |
| S07 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | `a827f4099976f9592863b71a12c9059dadd36f8b8037dba4e0ebf8482449c0a0` | 7987 | 66 |
| S08 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | `055966f4d3373dba5dcb6a99a3d3c56c02bf948fa0182d8bc74bb20c26490361` | 11649 | 206 |
| S09 | `docs/work-packages/role-review.md` | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` | 1400 | 7 |
| S10 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | `ac27766f6f329000a1b948d16006056e6d0780b00867dda7b788c4512fa99194` | 23267 | 301 |
| S11 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | `58465434d842511c72a5e27653bfa13ee51015f13939f3ad33f2fe8200f6287d` | 13234 | 208 |
| S12 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | `e20bd57efa318ab3f82ca31dfd9712732b9d827d3cd38839295a3b570317a9f7` | 20961 | 245 |
| S13 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | `29ba7de4a93aa48d770b229fb1ea7fbbc581c2f49c2c61bf5c5697fb8f5f3042` | 30323 | 426 |
| S14 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | `fe37a1a64327aa46ffb1afbc63e00aedab67c83083a3eb3ccbd2edbc17d3174e` | 8225 | 113 |
| S15 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | `5f96696b2956c157848e7215b3655fa074181b9f76ad9159264164b0598fbe85` | 16209 | 228 |
| S16 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | `342b7601176eb263ed3de199c6d9697d987e1650220a615a49c34181e9b27bbd` | 15424 | 201 |
| S17 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | `a1327441160fad148f847c378dec2e9251bc34e81254a973361a61c5f5450190` | 25574 | 290 |
| S18 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` | 248829 | 3080 |
| S19 | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` | 92552 | 1108 |
| S20 | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` | 184116 | 2205 |
| S21 | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` | 497785 | 3559 |
| S22 | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` | 377637 | 2648 |
| S23 | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4` | 634522 | 4295 |
| S24 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | `4778df5b572c4a09b3dfedc1f5bba4c84d376bcfcd0021ae7e9fbb6200158b8c` | 19223 | 171 |
| S25 | `docs/standards/AGENTS.md` | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` | 4052 | 58 |
| S26 | `docs/standards/prompt-wording-guidance.md` | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` | 12057 | 202 |
| S27 | `docs/work-packages/science-obligations.md` | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` | 6149 | 98 |
| S28 | `docs/standards/testing-and-gate-strategy.md` | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` | 25142 | 494 |
| S29 | `docs/prompt_templates/assurance-findings-template.md` | `95373d3c7df1f0a7e3ad7640fa22c0e137095e6cf5c4d92e3803846eb3f6ee83` | 736 | 12 |
| S30 | `tools/agents/context_report.py` | `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde` | 3544 | 79 |
| S31 | `docs/specifications/correctness-authority-model.md` | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` | 11159 | 221 |
| S32 | `docs/standards/local-ci-gate-selection.md` | `9b2cd01f06d433f99b9dc9ae3d596c4a348c23b9260ef4ecbe197efa84f60e8e` | 6874 | 129 |

### Every explicit content request

Recovery notes use the instrumented helper’s original logged indices: logged index N maps to request R(N+9), with three-digit request numbering. For example, logged 7 is R016. Initial R001–R009 predate that helper.

| Request | Source | Inclusive lines | Requested bytes | New bytes | Repeat bytes | Phase | Delivery/recovery note |
|---|---|---|---:|---:|---:|---|---|
| R001 | S01 | 1–121 | 9508 | 9508 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R002 | S02 | 1–84 | 6532 | 6532 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R003 | S03 | 1–105 | 6642 | 6642 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R004 | S04 | 1–75 | 5304 | 5304 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R005 | S05 | 1–22 | 1586 | 1586 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R006 | S06 | 1–65 | 4130 | 4130 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R007 | S07 | 1–66 | 7987 | 7987 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R008 | S08 | 1–206 | 11649 | 11649 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R009 | S09 | 1–7 | 1400 | 1400 | 0 | bootstrap | Initial full content request; backfilled from retained call record. |
| R010 | S10 | 1–301 | 23267 | 23267 | 0 | expansion | Complete delivered request. |
| R011 | S11 | 1–208 | 13234 | 13234 | 0 | expansion | Complete delivered request. |
| R012 | S12 | 1–245 | 20961 | 20961 | 0 | expansion | TRUNCATED REQUEST: Combined output truncation; full recovery at logged 7. |
| R013 | S13 | 1–426 | 30323 | 30323 | 0 | expansion | TRUNCATED REQUEST: Combined output truncation; full recovery at logged 8. |
| R014 | S14 | 1–113 | 8225 | 8225 | 0 | expansion | TRUNCATED REQUEST: Combined output truncation; full recovery at logged 9. |
| R015 | S15 | 1–228 | 16209 | 16209 | 0 | expansion | Complete delivered request. |
| R016 | S12 | 1–245 | 20961 | 0 | 20961 | expansion | Truncation recovery. |
| R017 | S13 | 1–426 | 30323 | 0 | 30323 | expansion | Truncation recovery. |
| R018 | S14 | 1–113 | 8225 | 0 | 8225 | expansion | Truncation recovery. |
| R019 | S16 | 1–201 | 15424 | 15424 | 0 | expansion | Complete delivered request. |
| R020 | S17 | 1–290 | 25574 | 25574 | 0 | expansion | Complete delivered request. |
| R021 | S18 | 1–400 | 33697 | 33697 | 0 | expansion | Complete delivered request. |
| R022 | S18 | 401–800 | 26155 | 26155 | 0 | expansion | Complete delivered request. |
| R023 | S18 | 801–1200 | 46986 | 46986 | 0 | expansion | TRUNCATED REQUEST: Output truncation; recovery logged 15 (990-1080). |
| R024 | S18 | 990–1080 | 19984 | 0 | 19984 | expansion | Truncation recovery. |
| R025 | S18 | 1201–1550 | 42274 | 42274 | 0 | expansion | TRUNCATED REQUEST: Output truncation; recovery logged 17 (1420-1490). |
| R026 | S18 | 1420–1490 | 13690 | 0 | 13690 | expansion | Truncation recovery. |
| R027 | S18 | 1551–1900 | 21115 | 21115 | 0 | expansion | Complete delivered request. |
| R028 | S18 | 1901–2250 | 18634 | 18634 | 0 | expansion | Complete delivered request. |
| R029 | S18 | 2251–2650 | 28540 | 28540 | 0 | expansion | Complete delivered request. |
| R030 | S18 | 2651–3080 | 31428 | 31428 | 0 | expansion | Complete delivered request. |
| R031 | S19 | 1–350 | 20398 | 20398 | 0 | expansion | Complete delivered request. |
| R032 | S19 | 351–700 | 39027 | 39027 | 0 | expansion | Complete delivered request. |
| R033 | S19 | 701–1108 | 33127 | 33127 | 0 | expansion | Complete delivered request. |
| R034 | S20 | 1–400 | 23325 | 23325 | 0 | expansion | Complete delivered request. |
| R035 | S20 | 401–800 | 47854 | 47854 | 0 | expansion | TRUNCATED REQUEST: Output truncation; recovery logged 27 (670-725). |
| R036 | S20 | 670–725 | 25803 | 0 | 25803 | expansion | Truncation recovery. |
| R037 | S20 | 801–1150 | 27253 | 27253 | 0 | expansion | Complete delivered request. |
| R038 | S20 | 1151–1550 | 24913 | 24913 | 0 | expansion | Complete delivered request. |
| R039 | S20 | 1551–1900 | 37106 | 37106 | 0 | expansion | Complete delivered request. |
| R040 | S20 | 1901–2205 | 23665 | 23665 | 0 | expansion | Complete delivered request. |
| R041 | S21 | 1–350 | 22717 | 22717 | 0 | expansion | Complete delivered request. |
| R042 | S21 | 351–700 | 60063 | 60063 | 0 | expansion | TRUNCATED REQUEST: Output truncation; recovery logged 34 (575-640). |
| R043 | S21 | 575–640 | 32251 | 0 | 32251 | expansion | Truncation recovery. |
| R044 | S21 | 701–1050 | 20578 | 20578 | 0 | expansion | Complete delivered request. |
| R045 | S21 | 1051–1350 | 49890 | 49890 | 0 | expansion | TRUNCATED REQUEST: Output truncation; recoveries logged 37-39 (1190-1320). |
| R046 | S21 | 1190–1260 | 7439 | 0 | 7439 | expansion | Truncation recovery. |
| R047 | S21 | 1261–1290 | 11504 | 0 | 11504 | expansion | Truncation recovery. |
| R048 | S21 | 1291–1320 | 6317 | 0 | 6317 | expansion | Truncation recovery. |
| R049 | S21 | 1351–1500 | 97469 | 97469 | 0 | expansion | TRUNCATED REQUEST: Output truncation; recovery logged 41 and its recoveries 42-44; retained tail includes 1461-1500. |
| R050 | S21 | 1390–1460 | 84970 | 0 | 84970 | expansion | TRUNCATED REQUEST: Recovery itself truncated; recoveries logged 42-44 (1423-1444), remaining head/tail delivered. |
| R051 | S21 | 1424–1433 | 23912 | 0 | 23912 | expansion | Truncation recovery. |
| R052 | S21 | 1434–1444 | 22762 | 0 | 22762 | expansion | Truncation recovery. |
| R053 | S21 | 1423–1423 | 3211 | 0 | 3211 | expansion | Truncation recovery. |
| R054 | S21 | 1501–1730 | 45931 | 45931 | 0 | expansion | TRUNCATED REQUEST: Output truncation; recovery logged 46-48 (1554-1630). |
| R055 | S21 | 1557–1580 | 12502 | 0 | 12502 | expansion | Truncation recovery. |
| R056 | S21 | 1554–1556 | 3783 | 0 | 3783 | expansion | Truncation recovery. |
| R057 | S21 | 1581–1630 | 6645 | 0 | 6645 | expansion | Truncation recovery. |
| R058 | S21 | 1731–1850 | 7195 | 7195 | 0 | expansion | Complete delivered request. |
| R059 | S21 | 1851–2150 | 61412 | 61412 | 0 | expansion | TRUNCATED REQUEST: Output truncation; recoveries logged 51-52 and54 (2020-2112). |
| R060 | S21 | 2020–2060 | 17239 | 0 | 17239 | expansion | Truncation recovery. |
| R061 | S21 | 2061–2095 | 5310 | 0 | 5310 | expansion | Truncation recovery. |
| R062 | S21 | 2151–2350 | 12995 | 12995 | 0 | expansion | Complete delivered request. |
| R063 | S21 | 2096–2112 | 8759 | 0 | 8759 | expansion | Truncation recovery. |
| R064 | S21 | 2351–2600 | 19348 | 19348 | 0 | expansion | Complete delivered request. |
| R065 | S21 | 2601–2900 | 28747 | 28747 | 0 | expansion | Complete delivered request. |
| R066 | S21 | 2901–2940 | 25850 | 25850 | 0 | expansion | Complete delivered request. |
| R067 | S21 | 2941–3190 | 17231 | 17231 | 0 | expansion | Complete delivered request. |
| R068 | S21 | 3191–3559 | 28359 | 28359 | 0 | expansion | Complete delivered request. |
| R069 | S22 | 1–157 | 21934 | 21934 | 0 | expansion | Complete delivered request. |
| R070 | S22 | 158–233 | 21769 | 21769 | 0 | expansion | Complete delivered request. |
| R071 | S22 | 234–259 | 21474 | 21474 | 0 | expansion | Complete delivered request. |
| R072 | S22 | 260–281 | 21553 | 21553 | 0 | expansion | Complete delivered request. |
| R073 | S22 | 282–309 | 21991 | 21991 | 0 | expansion | Complete delivered request. |
| R074 | S22 | 310–362 | 21823 | 21823 | 0 | expansion | Complete delivered request. |
| R075 | S22 | 363–443 | 21864 | 21864 | 0 | expansion | Complete delivered request. |
| R076 | S22 | 444–490 | 21657 | 21657 | 0 | expansion | Complete delivered request. |
| R077 | S22 | 491–561 | 21820 | 21820 | 0 | expansion | Complete delivered request. |
| R078 | S22 | 562–719 | 21957 | 21957 | 0 | expansion | Complete delivered request. |
| R079 | S22 | 720–907 | 21858 | 21858 | 0 | expansion | Complete delivered request. |
| R080 | S22 | 908–1294 | 21939 | 21939 | 0 | expansion | Complete delivered request. |
| R081 | S22 | 1295–1664 | 21976 | 21976 | 0 | expansion | Complete delivered request. |
| R082 | S22 | 1665–2073 | 21994 | 21994 | 0 | expansion | Complete delivered request. |
| R083 | S22 | 2074–2438 | 21669 | 21669 | 0 | expansion | Complete delivered request. |
| R084 | S22 | 2439–2519 | 21818 | 21818 | 0 | expansion | Complete delivered request. |
| R085 | S22 | 2520–2601 | 21746 | 21746 | 0 | expansion | Complete delivered request. |
| R086 | S22 | 2602–2648 | 6795 | 6795 | 0 | expansion | Complete delivered request. |
| R087 | S23 | 1–93 | 21553 | 21553 | 0 | expansion | Complete delivered request. |
| R088 | S23 | 94–161 | 21931 | 21931 | 0 | expansion | Complete delivered request. |
| R089 | S23 | 162–233 | 21956 | 21956 | 0 | expansion | Complete delivered request. |
| R090 | S23 | 234–284 | 21376 | 21376 | 0 | expansion | Complete delivered request. |
| R091 | S23 | 285–301 | 20692 | 20692 | 0 | expansion | Complete delivered request. |
| R092 | S23 | 302–317 | 20148 | 20148 | 0 | expansion | Complete delivered request. |
| R093 | S23 | 318–329 | 21946 | 21946 | 0 | expansion | Complete delivered request. |
| R094 | S23 | 330–341 | 21752 | 21752 | 0 | expansion | Complete delivered request. |
| R095 | S23 | 342–350 | 21277 | 21277 | 0 | expansion | Complete delivered request. |
| R096 | S23 | 351–361 | 20610 | 20610 | 0 | expansion | Complete delivered request. |
| R097 | S23 | 362–370 | 20818 | 20818 | 0 | expansion | Complete delivered request. |
| R098 | S23 | 371–487 | 21969 | 21969 | 0 | expansion | Complete delivered request. |
| R099 | S23 | 488–554 | 21913 | 21913 | 0 | expansion | Complete delivered request. |
| R100 | S23 | 555–652 | 21839 | 21839 | 0 | expansion | Complete delivered request. |
| R101 | S23 | 653–891 | 21954 | 21954 | 0 | expansion | Complete delivered request. |
| R102 | S23 | 892–1191 | 21985 | 21985 | 0 | expansion | Complete delivered request. |
| R103 | S23 | 1192–1455 | 22000 | 22000 | 0 | expansion | Complete delivered request. |
| R104 | S23 | 1456–1497 | 21617 | 21617 | 0 | expansion | Complete delivered request. |
| R105 | S23 | 1498–1656 | 21941 | 21941 | 0 | expansion | Complete delivered request. |
| R106 | S23 | 1657–2022 | 21971 | 21971 | 0 | expansion | Complete delivered request. |
| R107 | S23 | 2023–2371 | 21960 | 21960 | 0 | expansion | Complete delivered request. |
| R108 | S23 | 2372–2708 | 21951 | 21951 | 0 | expansion | Complete delivered request. |
| R109 | S23 | 2709–3046 | 21958 | 21958 | 0 | expansion | Complete delivered request. |
| R110 | S23 | 3047–3422 | 21999 | 21999 | 0 | expansion | Complete delivered request. |
| R111 | S23 | 3423–3742 | 22000 | 22000 | 0 | expansion | Complete delivered request. |
| R112 | S23 | 3743–3997 | 21614 | 21614 | 0 | expansion | Complete delivered request. |
| R113 | S23 | 3998–4168 | 21382 | 21382 | 0 | expansion | Complete delivered request. |
| R114 | S23 | 4169–4203 | 21953 | 21953 | 0 | expansion | Complete delivered request. |
| R115 | S23 | 4204–4270 | 21972 | 21972 | 0 | expansion | Complete delivered request. |
| R116 | S23 | 4271–4295 | 6485 | 6485 | 0 | expansion | Complete delivered request. |
| R117 | S24 | 1–80 | 5357 | 5357 | 0 | expansion | Complete delivered request. |
| R118 | S25 | 1–58 | 4052 | 4052 | 0 | expansion | Complete delivered request. |
| R119 | S24 | 81–171 | 13866 | 13866 | 0 | expansion | Complete delivered request. |
| R120 | S26 | 174–194 | 1333 | 1333 | 0 | expansion | Complete delivered request. |
| R121 | S09 | 1–7 | 1400 | 0 | 1400 | expansion | Complete delivered request. |
| R122 | S27 | 1–98 | 6149 | 6149 | 0 | expansion | Complete delivered request. |
| R123 | S03 | 1–105 | 6642 | 0 | 6642 | expansion | Complete delivered request. |
| R124 | S28 | 193–345 | 8067 | 8067 | 0 | expansion | Complete delivered request. |
| R125 | S28 | 439–473 | 1262 | 1262 | 0 | expansion | Complete delivered request. |
| R126 | S29 | 1–12 | 736 | 736 | 0 | expansion | Complete delivered request. |
| R127 | S30 | 1–79 | 3544 | 3544 | 0 | expansion | Complete delivered request. |
| R128 | S31 | 1–221 | 11159 | 11159 | 0 | expansion | Complete delivered request. |
| R129 | S32 | 1–129 | 6874 | 6874 | 0 | expansion | Complete delivered request. |
| R130 | S02 | 1–84 | 6532 | 0 | 6532 | expansion | Complete delivered request. |
| R131 | S05 | 1–22 | 1586 | 0 | 1586 | expansion | Complete delivered request. |
| R132 | S06 | 1–65 | 4130 | 0 | 4130 | expansion | Complete delivered request. |
| R133 | S07 | 1–20 | 1234 | 0 | 1234 | expansion | Complete delivered request. |
| R134 | S08 | 1–20 | 2404 | 0 | 2404 | expansion | Complete delivered request. |
| R135 | S11 | 1–20 | 1703 | 0 | 1703 | expansion | Complete delivered request. |
| R136 | S15 | 1–20 | 1993 | 0 | 1993 | expansion | Complete delivered request. |
| R137 | S14 | 1–20 | 1151 | 0 | 1151 | expansion | Complete delivered request. |
| R138 | S12 | 1–20 | 1857 | 0 | 1857 | expansion | Complete delivered request. |
| R139 | S13 | 1–20 | 1841 | 0 | 1841 | expansion | Complete delivered request. |
| R140 | S16 | 1–20 | 1702 | 0 | 1702 | expansion | Complete delivered request. |
| R141 | S17 | 1–20 | 2097 | 0 | 2097 | expansion | Complete delivered request. |
| R142 | S11 | 21–208 | 11531 | 0 | 11531 | expansion | Complete delivered request. |
| R143 | S15 | 21–228 | 14216 | 0 | 14216 | expansion | Complete delivered request. |
| R144 | S10 | 1–281 | 21956 | 0 | 21956 | expansion | Complete delivered request. |
| R145 | S10 | 282–301 | 1311 | 0 | 1311 | expansion | Complete delivered request. |
| R146 | S08 | 21–206 | 9245 | 0 | 9245 | expansion | Complete delivered request. |
| R147 | S13 | 21–407 | 21574 | 0 | 21574 | expansion | Complete delivered request. |
| R148 | S13 | 408–426 | 6908 | 0 | 6908 | expansion | Complete delivered request. |
| R149 | S04 | 1–75 | 5304 | 0 | 5304 | expansion | Complete delivered request. |
| R150 | S12 | 21–245 | 19104 | 0 | 19104 | expansion | Complete delivered request. |

### Discovery scans and non-content operations

Two voluntary source search requests were also made. These scanned complete files but delivered only matched lines; neither is represented as having read the unmatched authority. Their matching content overlaps the content union above; the exact additional union is stated below. The search exposure below is additional to the 150-request exposure, while full scanned bytes are a separate tooling metric.

| Scan | Source/path | Pattern | Matched inclusive lines | Matched UTF-8 bytes | Full scanned bytes | Complete SHA256 |
|---|---|---|---|---:|---:|---|
| Q1 | `docs/standards/prompt-wording-guidance.md` | `context-measurements or ^#` | 1, 7, 174, 195 | 84 | 12057 | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` |
| Q2 | `docs/standards/testing-and-gate-strategy.md` | `^## ` | 8, 17, 33, 52, 77, 129, 193, 219, 274, 292, 346, 368, 386, 403, 417, 427, 439, 457, 474, 487 | 553 | 25142 | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` |

Scan matches add 430 unique bytes and 637 requested matched-line bytes: content-plus-search union 2306225, exposure 2819443, repeated requested exposure 513218. A complete-file scan is not counted as complete delivered reading. No within-line span requests were needed after switching to bounded UTF-8 chunks; original truncations were recovered with overlapping inclusive line ranges, as recorded above.

Metadata/tool operations: `pwd`; root-to-nearest instruction discovery with `tools/agents/find-agents --for` the assigned report; `wc -l` on the six routed external contracts; own `/tmp/lse5*` filename discovery; own helper/selection/ledger JSON processing; and `tools/agents/context_report.py --help` and the structural reporter invocation. These were not additional manual authority-content requests. Hashing/range selection programmatically loads full source bytes to compute complete SHA256 and selected-range measurements; those internal reads are not claimed to be model-delivered content. The context-report implementation itself was voluntarily read and is included in the table. The reporter command used `/tmp/lse5selection.json`, root `/workdir/openWEPP`, no revision/Git argument, and wrote `/tmp/lse5context-report.json`; it exited 0. This validates declared structural ranges/bytes only. Own report inspection (`wc`, bounded tail and heading/phrase search) is output-artifact QA, not another authority request; its first tail output was truncated and was not used as source authority. No production binary, model, science test or external-source web lookup was run.

### Why the expansion was necessary

The entry/interface routed the snow-free covered surface answer to common numerical/physical rules, surface energy, soil, water and solve boundary. Their mandatory ownership/dependency chains required complete current vegetation V10, coupled-time, surface-liquid, snow energy, water balance and snow/freeze contracts. Snow-free describes the current physical regime; it does not erase phase, represented-snow onset, terminal-event, support/restart or ingress-owner exclusions exposed by those contracts. Litter-phase and exact soil/surface/map custody were expanded because an unspecified covered forest tile cannot be silently treated as phase-free, and retained liquid, sensible storage, phase spill and native accepted-map restart evidence share these owner boundaries. Terminal-support was read for the explicit older-versus-current admission boundary and 60 s policy; map-custody was voluntarily read in full for precise provisional/final chronology. Governance expansion established report-only scope, complete authority consumption, evidence/claim posture, independent-role rules and structural measurement requirements.

Recovery requests exist solely to replace missing portions of oversized tool output; no truncated middle was treated as read. The request table preserves the original requested bytes as exposure and counts recovery overlap again. The initial nine requests were backfilled from retained actual call records, while later requests were instrumented before delivery. The full external contract reading obligation materially expanded the external/governance union; neither desired context budget nor anticipated compaction was used as an authority waiver. This is a first completed report; it uses no previous exercise answer, score, intake prediction, old monolith or parent result.


Final ledger check: all 150 declared requests have valid inclusive source ranges and recomputed requested-byte totals; every complete-source SHA256 still matches its frozen file. The 32-source identity table joins every request. Category sums and incremental overlap are calculated from actual selected line bytes, not estimated tokens. Scan-match additions are explicitly outside the 150-request totals. No empirical science acceptance is inferred from this structural verification.
