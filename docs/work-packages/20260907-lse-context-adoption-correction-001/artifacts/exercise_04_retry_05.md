# Independent frozen-litter closure reading exercise — first report, retry 05

Static: current canonical candidate-v32 sources identified in the request ledger below; assigned source cut `b3b7b57f4` supplied by the parent, not independently verified through Git. Ran: source-reading and byte-accounting commands only. No production simulation, binary, comparator, numerical reconstruction on run operands, or tests ran.

Role/session: fresh independent scientific reading exercise, `/root/closure_iteration5`. Configured/requested effort and effective effort: UNOBSERVED in tool metadata. Scope: independently reconstruct **snow-free frozen-litter surface/soil physical energy and mass closure from accepted primitive receipts**. No accepted primitive bundle was supplied. Thus the result is a concrete reconstruction prescription and evidence inventory, not a numerical closure verdict. Missing forcing, coefficients, receipts, beginning/ending states, and binary64 operands remain missing. This report neither establishes implementation correctness nor changes any production, experiment, calibration, or promotion disposition.

## Selection and expansion

The entry contract's `Reading routes` selects `litter-phase.md#litter-phase` for this task and requires the full shared `interface.md#interface`. Active phase and closure require all of water-vapor, soil-coupling, and surface-custody. Physical operands expand to surface-energy, common-details, and soil-custody. Accepted-primitive rule selection requires solve-boundary; support and regime boundaries require all terminal-support; receipt origin and accepted-map provenance require map-custody. I read each selected chapter completely, including introductions, dependencies, marked definitions, guards and obligations. Truncated initial requests were recovered with bounded requests; the ledger includes repeats rather than subtracting them from actual exposure.

External routes explicitly require the **whole** SC-SURFACELIQUID-001, SC-WATBAL-001, SC-COUPLEDTIME-001, SC-SNOWENERGY-001 and SC-VEGETATION-001, not merely their initially named subsections. All five were read completely, including historical wording, supersession qualifiers and current amendments. This is why a snow-free target still requires represented-snow and post-event boundary reading. Governance reads are separated in the ledger. ADR-0044 and numerical-solver-architecture were expanded to resolve the current one-solver restriction encountered in the snow boundary.

This is accepted authoritative parcel reconstruction, not rain-temperature-provider rederivation: the water-vapor dependency expressly distinguishes those scopes. SC-SNOWFREEZE-001 is not selected on that condition. The provider's accepted output and lineage remain mandatory inputs; a future provider-rule or provider-derived ingress-enthalpy correctness claim would require its whole contract. No solver implementation or full evaluator correctness claim selects nonlinear-solve. No optimization, error-order replay optimization or executable capture claim selects dependency-replay or qualification. No source-provenance adjudication, authority-gap disposition, enforcement audit or promotion claim selects audit-details. I preserve those boundaries rather than interpreting unselected chapters as nonbinding. A new claim or uncertain mechanism would expand reading before work, not waive authority.

## Domain, ownership and support before arithmetic

`surface-energy#selected-sources-and-domain`, `#exact-ownership-and-state`, `common-details#guards`, `litter-phase#version-14-snow-free-frozen-forest-litter-successor-amendment` and the SurfaceLiquid frozen-litter V2 amendment fix the domain. `OPENWEPP_SNOW_FREE_LSE_V3` imports V2/V1 and adds frozen **forest-litter** only. Litter ice is hydrology-owned liquid-water-equivalent mass; it is neither snow nor soil `frozwt`. It does not authorize frozen/thawing soil, bare-mineral ice, frozen ponding, snow-present or terminal-snow evaluation. Snow is absent at both endpoints; no censored terminal payload is admitted. Require finite positive area and support, complete forcing/configuration/state, one ground class per tile, correct typed owners, neutral positive wind and valid roughness/log domains. Guard physical domain before imposing numerical bounds. Covered vegetation retains its own domain: frozen litter does not authorize a subfreezing unsupported canopy (`VEG-E-040`).

Hydrology owns **all water masses**, including liquid and ice litter stores and soil water. LSE owns the surface sensible-energy coordinate, with temperature reconstructed from it and current owner mass; the saved temperature is a numerical warm start, not a second mutable physical state. Soil thermal owns every one of the N soil temperatures and enthalpies. Vegetation owns canopy processes. Climate supplies accepted hydrometeor phase/temperature. No scalar daily ET, WATBAL diagnostic or independent soil-temperature copy can replace an owner. The V16 high terms mirrored into frozen V3/V2 fields are mirrors of one exact owner, not duplicate authority.

Local tile units are `kg m^-2 tile-ground`, `J m^-2 tile-ground`, and `W m^-2 tile-ground`. Multiply by the positive, configuration-joined tile fraction `f_t` **once** for OFE ground; validate the complete unique fraction set and its topology sum. “Stand-ground” means one OFE's horizontal area, not the routed hillslope. Keep source/destination OFE and unequal areas through routing. Positive radiation enters the surface; in the constitutive equations below, H, vapor-energy Qv, and downward G leave it. This converts the inward-positive generic ledger without silently reversing a flux twice.

`terminal-support#support`, INV-LSE-114/115 and support failures 041–043 require a slab-specific `LseSupportAdmissibilityReceiptV1`: model/configuration, parent/segment/slab identity, exact absolute nanosecond bounds, duration bits, immutable beginning LSE/soil owner identities and the adopted numerical/minimum-support profile. The actual LSE floor is **60,000,000,000 ns**, before Newton; one tick below rejects with byte-exact rollback. A 1 ns structural clock event is not a physical solve. Aggregate the maximum minimum of active participants only. Use the coupled-time canonical integer duration conversion once, shared by all owners; do not reconvert locally, scale a nominal result, freeze physics, discard a remainder or cap every stable support to the floor. Stable cases must accept substantially larger intervals.

SC-COUPLEDTIME governs gap-free half-open accepted supports, sequential accepted endings as next beginnings, event identity and exact-one parent publication. Zero-duration events integrate no rates; the narrowly admitted balanced zero-duration custody transition is not general positive physics. Rejected candidates and retries publish nothing or durable retry state. Event/refinement limits and typed exhaustion still apply; no private clock advance is permitted.

The boundary remains ordered: censored terminal reject, represented-snow delegate, then snow-free evaluation. Current SC-SNOWENERGY INV-082/083 plus ADR-0044 override historical numerical dispatch: immutable beginning snow absence invokes no covered solver; represented snow at or below `1 kg m^-2` uses the specifically admitted V22 terminal authority; greater snow uses the one canonical coupled snow-water/enthalpy and LSE soil boundary. Cold, frozen, mixed, thaw/refreeze and layer/event active sets do not create recovery solvers. The old 96-map and V33–V57 eligibility/rescue chain is not authority to try a second algorithm. Terminal V22 is the named non-CoE exception: the accepted microstep must actually end at zero ice and close mass, energy, topology and receipt custody, release liquid exactly once, then evaluate only the positive post-event successor as snow-free. A guessed continuous root, censored schema-v8 payload, post-event flux in the pre-event candidate, or fusion heat injected into soil as liquid sensible heat is invalid.

SurfaceLiquid INV-035 / CoupledTime INV-031 / SnowEnergy INV-087,C-055 additionally admit an authenticated already-accepted slab prefix. It must be positive, contiguous from parent start to the exact first snow-free terminal split, with complete owner and receipt chain. Consume that prefix as chronology only: no Stage 3, litter phase, WB14, routing, ledger regeneration or resealing. Preserve cumulative WB14 bits; the first new physical map ordinal is zero. Restart must prove the same continuation without replaying the accepted prefix.

## Independent physical reconstruction

`OBL-LSE-P-001..004`, `common-details#algorithm` and `water-vapor#errors` require immutable source-identified records, sealed start-to-end ledgers, pre-mutation validation, branch/support/tolerance disclosure, terminal exclusion and atomic commit. Producer residuals are never operands. Establish a bijection between the required typed sources and receipts, rejecting omissions, duplicates, wrong order, basis, support, owner or lineage before computing closure.

For radiation, INV-LSE-101/102, `surface-energy#shortwave-and-reciprocal-longwave` and full Vegetation V8/V7 require the complete current-temperature canopy/ground calculation. Shortwave is the V7 direct/diffuse VIS/NIR two-stream column with actual ground-class albedo as its lower boundary; ground reflection traverses all overlying ranks. Leaf/stem optical area, sun/shade ownership, PAR and once-only clumping must join the same trial. Neither a bulk canopy nor terminal-downward-only radiation receipt is sufficient.

For longwave, unit emissivity and no reflection apply. For occupancy i, `tau_i=exp[-0.8*Omega_i*(LAI_i+SAI_i)]`. With exact component emissive areas, `w_j=a_j/sum(a_j)` and `E_i=sum_j w_j*sigma*T_ij^4`, independently form

```
Ldn_(i+1) = tau_i*Ldn_i + (1-tau_i)*E_i
Lup_n = sigma*T_s^4
Lup_i = tau_i*Lup_(i+1) + (1-tau_i)*E_i
Rlw_ij = w_j*(1-tau_i)*(Ldn_i+Lup_(i+1))
         - 2*w_j*(1-tau_i)*sigma*T_ij^4
Rlw_s = Ldn_n - sigma*T_s^4.
```

Zero total emissive area gives exact `tau=1` and zero component terms. First check component, layer and tile closure, then one f weighting to OFE. Previous-step ground temperature, prescribed upward ground longwave, bulk component temperature, double clumping or ground-to-atmosphere bypass violates the operands even if an aggregate residual happens to vanish.

For turbulence, independently use `H=rho_a*c_p*(T-T_recipient)/r_h` and `v=rho_a*(q-q_recipient)/r_v`, retaining signs. Open neutral resistance is the product of the momentum and heat/vapor logarithms divided by `0.4^2*u_ref`, with positive wind and reference height above all positive roughness lengths. There is no wind floor or stability substitute. A covered tile has exactly one zero-storage `(T_c,q_c)` node; all component exchanges and one ground exchange join it, and sums minus the canopy-to-atmosphere flux equal zero. Under-canopy ground does not exchange directly with reference air, and its contribution is not replicated per occupancy.

The covered ground path is exactly the ISBA-MEB equations in `surface-energy#neutral-turbulent-heat-and-vapor-network`: `Re=u_l*l_w/nu`; `c_d=1.328*(2/sqrt(Re))+0.45*((1-chi_L)/pi)^1.6`; `d=1.1*z_hv*ln[1+(c_d*LAI)^0.25]`; `u_star=.4*u_ref/ln[(z_hv-d)/z0v]`; `K_hv=.4*u_star*(z_hv-d)`; `r_g-c=z_hv/(phi_v*K_hv)*{exp[phi_v*(1-z0g/z_hv)]-exp[phi_v*(1-(d+z0v)/z_hv)]}`. Constants are `phi_v=2`, `z0g=.007 m`, `chi_L=.12`, `u_l=1 m/s`, `l_w=.02 m`, `nu=1.5e-5 m²/s`, with `psi_H=f_hv=1`. Require LAI>0, `z_hv>d+z0v>z0g>0`, `z_ref-d>=z_hv-d>0` and positive finite resistance/logarithms. Heat and vapor are distinct semantic operands on that same path. Vegetation's leaf, wet and stem conductances retain their own dimensions and friction-velocity operand (`g_b=.01*sqrt(u_star/dimension)`); raw reference wind is not a substitute. V8 uses one joint current trial, not stale independently converged inner canopy output.

`soil-coupling#soil-coupling`, INV-LSE-103/106/110 and the N-layer thermal definition supply the storage and ground-transfer primitives. Litter `lambda_l=.1+.03*W_l/(rho_w*dz_l)` and `C_dry=dz_l*rho_ld*c_ld` use admitted configuration; litter water capacity is positive and liquid remains within its owner capacity after the authorized split. Litter blocks direct mineral-soil vapor and upward capillary donation. Interface conductance is `g=2/(dz_upper/lambda_upper+dz_lower/lambda_lower)` and downward flux is `G=g*(T_upper-T_lower)`. Crank–Nicolson integrates the average of beginning and ending fluxes. Surface loses the top flux; soil layer 1 receives it once; adjacent soil layers receive opposite internal interface terms; bottom flux is zero. This remains true for N=1. A finite-capacity surface reconstructs its beginning temperature from beginning enthalpy and mass. The equilibrium-zero surface has exact zero dry capacity, water and enthalpy and uses the current trial temperature at both top-boundary endpoints; an arbitrary warm start cannot become beginning physical energy. Frozen V3 litter uses the finite-capacity specialization.

### Frozen litter: vapor, phase and capacity spill

`litter-phase#retained-authority-and-adjudicated-constants`, `#v3-state-phase-free-solve-and-signed-vapor` and `#bounded-kinetic-phase-and-fusion-energy-closure` are the exact rule anchors. Their retained source anchors are Napoly/ISBA-MEB Appendix A1–A4,A7–A14 (SHA256 `2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d`), SURFEX `isba_meb` generated lines 1992–2159 (`0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a`), `isba_fluxes_meb` 388–407 (`e0378bc89ee0d52cffe14841aac56de1d8d379edf18ad29f24cfdb9ea0dfdbbc`), and `ini_csts` 146–157 (`f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a`). These are canonical-text provenance statements, not independently inspected source files or a fresh provenance adjudication.

Use `T_ref=273.15 K`, `rho_w=1000`, `rho_i=920 kg/m³`, `C_w=4218`, `C_i=2106 J/(kg K)`, `L_f=333700 J/kg`, `tau_ice=3300 s`, and **`W_i,max=.85*rho_w*dz_l`**. The ice-capacity density is rho_w, not rho_i; snow's separate fusion constant is not interchangeable. The phase sign follows the adjudicated liquid/ice conservation and retained execution order, not the conflicting printed A4 sign.

The whole nonlinear solve is first phase-free: no fusion, freeze/melt, phase-adjusted temperature/capacity or phase active branch enters any residual, Jacobian, authorization or convergence witness. Let `p_i=0` when `W_l0+W_i0=0`, otherwise `W_i0/(W_l0+W_i0)`; `h_ul=.5*(1-cos(pi*W_l0/W_l,max))`, with the analogous ice h using its own beginning mass/capacity. Then

```
v_l,raw = (1-p_i)*rho_a*(h_ul*q_sat_liquid(T_l,p)-q_recipient)/r_l-c
v_i,raw = p_i*rho_a*(h_ui*q_sat_liquid(T_l,p)-q_recipient)/r_l-c
Q_v,l = v_l*[C_w*(T_l-T_ref)+L_v(T_l)]
Q_v,i = v_i*[C_i*(T_l-T_ref)+L_s(T_l)]
L_v(T) = 2.501e6 - 2369*(T-T_ref).
```

Both phases deliberately use saturation over **liquid**, not ice. Positive liquid evaporation and ice sublimation have separate own-beginning-pool caps. Negative liquid condensation and ice deposition are uncapped withdrawals only in the sense that they are **credits**, each to its named phase. They cannot be converted into a total-store authorization, absolute value or same-interval ingress donation. Preserve each finalized signed mass and full sensible-plus-latent energy until sealing; only then aggregate air-side terms. The temperature-appropriate L_s and its accepted authority/primitive value are required; no actual value was supplied and I do not manufacture a numerical sublimation term.

From final accepted vapor only, `W_l*=W_l0-dt*v_l`, `W_i*=W_i0-dt*v_i` and `U*=(C_dry+W_l*C_w+W_i*C_i)*(T*-T_ref)` using the post-vapor masses. Independently reconstruct

```
U* - U0 = dt*(Rsw + Rlw - H - Q_v,l - Q_v,i - Gbar_s1).
```

No raw potential request or authorization may replace finalized use. Validate `W_l*>=0`, `0<=W_i*<=W_i,max`, positive finite dz and resulting heat capacities before the phase operator. Set

```
M_warm = rho_i*C_i*dz_l*max(T*-T_ref,0)/L_f
M_cold = rho_i*C_i*dz_l*max(T_ref-T*,0)/L_f
m_melt = min(W_i*, (dt/tau_ice)*min(M_warm,W_i*))
m_freeze = min(W_l*, W_i,max-W_i*,
               (dt/tau_ice)*min(M_cold,W_l*))
m_phase = m_freeze - m_melt
W_l,raw = W_l* - m_freeze + m_melt
W_i,raw = W_i* + m_freeze - m_melt
U_raw = U* + L_f*m_phase
T_raw = T_ref + U_raw/(C_dry+W_l,raw*C_w+W_i,raw*C_i).
```

The outer bounds remain mandatory for dt>tau; this is not a maximum-support restriction. At T_ref both transfers are exact zero. Reconstruct liquid-plus-ice conservation and the **phase-only** invariant `U-L_f*W_i`. Do not use the old capacity in a temperature increment, impose instantaneous equilibrium, clamp temperature or resolve fluxes on this support. T_raw is an ending state/next-support warm start. The phase-only invariant must not be misreported as a whole-interval enthalpy equation that forgets sublimating/depositing ice: for a whole interval, transforming U to `U-L_f*W_i` also requires the full actual change in W_i, including vapor.

INV-LSE-156/C-011, `litter-phase#exact-v3-litter-phase-capacity-spill-custody-amendment` and the identical SurfaceLiquid amendment extend the operand list after phase. If `W_l,raw<=W_l,max`, spill is positive zero and raw state unchanged. Otherwise `m_spill=W_l,raw-W_l,max`, `h_spill=C_w*(T_raw-T_ref)`, `Q_spill=m_spill*h_spill`; retained mass is the **second subtraction** `W_l,raw-m_spill`, not a capacity assignment. Retained U is `U_raw-Q_spill`, retained capacity is rebuilt from retained masses, and retained temperature follows its U identity. Check raw=retained+spill independently in the mandated binary64 operation order. Reject invalid mass/capacity, over-capacity retained state or missing custody; do not snap it into capacity.

The phase receipt remains immutable. A named negative `LitterPhaseCapacitySpillEnergy` exact-surface operand follows phase-free/fusion operands. Convert spill to OFE mass once; construct one internal `LitterPhaseOverflow` parcel with exact source key/phase receipt, model/configuration, transaction and full child `[0,dt)` support. Its OFE energy is computed from the converted OFE mass and h. It enters WB14 once, and any returned retention is a separate positive retained-ingress credit. Do not cancel the spill debit, relabel it condensation, treat it as caller ingress or move it to a rainfall-hour window. Split, ingress, replay or publication failure rolls back all owners and receipts. There is no second vapor/phase solve.

INV-LSE-157/C-012 and SurfaceLiquid's heterogeneous finalized-use join require exact native phase rows and ordinary rows. For every row `0<=F<=A<=D`; F is bit-identical to the accepted final-use receipt and need not equal A. Remove only native rows already consumed by the phase receipt. Apply remaining ordinary finalized-use debits once, in canonical GroundWaterKey order, to the phase-adjusted V2 owner; retain zero-use row identities. Do not replace the whole beginning resource set or accepted owner, duplicate a native/spill energy parcel, debit A, omit a mixed ordinary row, or infer native consumption from key resemblance.

### Water transaction, current ingress and soil credit

INV-LSE-104/105/107 and `water-vapor#water` require uncapped potential requests from the immutable beginning full canopy/ground/soil state. Hydrology authorizes all shared-snapshot root and ground requests **once before current ingress**. Rebuild the full system from the original beginning with fixed caps. The capped branch is selected when `cap_rate<=q_law`, including equality, with zero generalized derivative; an unperturbed branch is frozen for its numerical probes, not for future iterates. Final OFE use is `F=f_t*q*dt`; strict physical bounds `0<=F<=A<=D` have no closure-tolerance loophole. No scalar stress donation, potential-state continuation or second authorization is allowed.

SurfaceLiquid's proportional authorization is the prescribed floating operation order: full demand if available; otherwise `fl(fl(D_i*S)/sum D)`, with `S=f_t*W0`. Require both OFE sum(A)<=S and tile sum(fl(A/f))<=W0. The narrowly authorized representational overshoot envelope `1e-14+64*eps*sum_abs` allows only the common safe downscale/minimum-ratio or at-most-64-step monotone bit bisection, preserving positive shares. It does not permit priority allocation, residual donation or arbitrary nextdown. Ordinary condensation credits named storage before capacity overflow; its typed overflow is separate from V3 phase spill.

Only after accepted vapor, bounded phase, spill and the heterogeneous resource join may current liquid rain/runon/canopy release enter. Open raw precipitation and covered throughfall/drainage/stemflow are exclusive representations of the same source. Both canopy drainage terms retain actual accepted wet-surface temperature and individual mass lineage, not air, ground or previous temperature. Rain uses the exact accepted retained-provider output `hydrometeor_temperature_c+273.15`; runon requires the actual upstream accepted temperature and enthalpy. Missing temperature rejects. Positive-mass mixture is `T_ref+sum(m*h)/(C_w*sum m)` with `h=C_w*(T-T_ref)`; zero mass has zero energy and **no temperature**.

INV-LSE-130 permits only binary64 `0x4071126666666667`, the first upward neighbor of exact T_ref, to canonicalize to T_ref before named covered-canopy ledger/release or Stage 3 terminal-liquid publication. It is not a solver, general parcel, surface-temperature or carry clamp. Exact reference is unchanged, second-up is not normalized, and below reference retains its existing covered-canopy-snow rejection. Persist temperature and zero specific enthalpy together.

SC-WATBAL WB14 and SC-SURFACELIQUID own one actual stateful Green–Ampt ingress partition per OFE physical child, with source windows, admitted K/psi and soil inputs, parent support and retained cumulative infiltration. Actual child continuation is not a restarted dry solver or diagnostic scalar. Preserve the 48×1800-second parent timing authority and result-blind 60-second structural grid where applicable, parent-local cursor and exact cumulative beginning/end; advance the persistent cursor once at parent completion. Legacy depression retention and WAT5 retention bins remain zero on this path. Source rainfall/support sequences, conductivity, suction, water content and disturbance/recovery inputs must be supplied and validated, not filled with defaults.

Let X be total admitted OFE current liquid ingress, Q its summed sensible energy, and actual WB14 outputs I infiltration and E excess, with I+E=X. For X>0 `h_mix=Q/X`; for zero X the internal zero convention is not a physical parcel temperature. Soil receives `Q_inf=I*h_mix`; excess receives `Q-Q_inf` in the specified order. Distribute named source portions proportionally with the canonical final remainder and same mixture h; child split uses `Q_child=r*Q_parent` and remainder by subtraction. This preserves lineage instead of recomputing energy from a convenient downstream temperature.

Ordinary retained ingress is bounded by residual surface capacity. The explicit small-room exception is only `0<M_raw<=tau_M`, where `tau_M=1e-14+64*eps*(abs(fWmax)+abs(fW)+abs(E))`: retain zero, preserve state bits, route **all** corresponding mass and energy. It is not permission to discard a tiny energy credit or relax the post-phase capacity split. Otherwise retained tile energy is the named binary64 `(m_retained*h_mix)/f_t` **before** exact dyadic decode. Infiltration energy credits soil layer 1; runoff keeps its matching mass/energy parcel. Route unequal OFE areas with the source/destination area factor once, preserving origin while rekeying destination basis, and enqueue before downstream evaluation with queue digest joins. No post-hoc aggregate runoff can supply the downstream primitive.

Primary surface closure therefore reconstructs the phase-free equation above, the fusion addition, negative phase-spill debit, any independently required ordinary resource energy operands, and each named retained-ingress credit in order. Soil closure reconstructs each layer beginning-to-ending energy from top, equal/opposite internal, zero-bottom and first-layer infiltration operands. Combined surface-plus-soil closure cancels the **same accepted** G operands, never separately computed approximations. For surface water, reconstruct separate liquid and ice balances through vapor, phase, spill, ordinary finalized use and retained ingress; phase cancels in total liquid-plus-ice mass. Soil water change is from hydrology's actual typed water transitions, not the soil-thermal credit. Already consumed vapor rows cannot be debited again through SC-EVAP or daily WB17. WB13 daily aliases, RM that already includes irrigation, storage/frozwt conventions and geometry-normalized public Q are not interchangeable subdaily primitive operands. On a whole-hillslope balance, internal runon/runoff cancels only after converting to a common physical-area basis and proving the routing bijection.

## Exact arithmetic, receipt custody and accepted chronology

INV-LSE-150/P-005/C-005 (`soil-custody`) and INV-LSE-151/P-006/C-006 (`surface-custody`) distinguish numerical/physical closure from exact persistent accumulation. Per soil layer `E=exact(H_hi)+R`; per surface tile `U=exact(U_hi)+R_U`. Validate schema/model/configuration, ordered OFE topology/layers/keys, support, transaction/predecessor, complete owner lineage, source kind and ordinal, and exact-one operand membership first. Perform the contract's physical binary64 operations, including area conversion and OFE-to-tile division, **before** exact finite-binary64 dyadic decode. Do not replace a binary64 division with a rational operation. Sum beginning high plus carry and every independent accepted operand with arbitrary-precision integer/dyadic arithmetic; round the final total **once**, nearest-even, to a finite binary64 high term. The new carry is the exact difference. Overflow or reconstruction failure rejects.

Canonical carry wire zero is `(0,"0",0)`; a nonzero carry has sign ±1 and odd positive lowercase hexadecimal significand without leading zero, with its unique exponent and reachable-support bounds. Preserve the high term's stipulated signed-zero migration/no-op behavior; only carry has one zero form. Require frozen V2/V3 high mirrors bit-identical to exact-surface U_hi. No producer carry residual, aggregate credit without primitive sources, endpoint-difference operand, compensated floating sum, epsilon test, forced ULP, nextafter, subnormal flush, discarded small credit or carry feedback into temperatures/fluxes/phase is allowed. Exact carry equality alone can be tautological; independently reconstructed radiation, vapor, phase, transfer and advection remain required.

SurfaceLiquid dimensional closure uses its named mass `1e-14 kg m^-2+64*eps*sum_abs` and energy `1e-9 J m^-2+64*eps*sum_abs` predicates. Those do not replace exact identity, strict availability/capacity, canonical carry equality, nonlinear residuals or event support. Generic historical gap tolerances cannot be assigned guessed values. Record each applicable residual, dimensional scale and threshold separately.

Selected marked definitions retain these additional prerequisites and chronology:

- INV-LSE-153/C-008: each partial-parent surface high/carry and digest chain advances physically, while shared frozen V3/V2 accepted markers retain the parent predecessor. Only final publication stamps the accepted child transaction once across the required surfaces. Caller posture or an old wire schema cannot bypass this rule.
- INV-LSE-155/C-010: an unpublished original-prepared resident view is a typed borrowed nonowner bound to exact schema/model/run configuration, topology, transaction/predecessor, physical beginning seal and contiguous child. It has no owning, restart/checkpoint, accepted-receipt or receipt-free sealing authority, and cannot rebind outer transaction/support. Final replay uses the original prepared owner and complete accumulated canonical operands to reproduce the selected ending credit chain, then one atomic installation.
- INV-LSE-158/C-013: canonical operand order uses authenticated configuration OFE rank, then source key/kind/ordinal; opaque IDs are not numeric or lexicographic topology. Successful operand validation alone is not installation proof.
- INV-LSE-159/C-014: a validated immutable handoff is private and nonserializable, bound to exact model/config/state, transaction/predecessor/support and prefix count/head/tail/chain. No public constructor, mutation or digest-only reconstruction creates authority. An unchanged validated prefix may accompany a newly validated suffix; mutation invalidates the proof. External bytes, restore, durable publication and untrusted returns still receive full validation.
- The V30 static/forcing/resident definitions in surface-custody bind static facts to one parent generation, forcing proof to the pointer-identical original validated forcing, and native resident proof to the exact original V3/V2 revision. Consume resident evidence only after V8 and derived ingress at the original validation position. V8 does not attest the resident. Dynamic map work is never certified by a cached static fact.
- INV-LSE-160/C-015: an already fully validated snow-free provisional physical ending may be bound to its independently derived final slab identity by a private, move-only, single-use proof. Bind parent/segment/ordinal/ticks/duration, configuration/topology/forcing, owner/receipt and physical ending identities; only the authorized slab-identity delta changes. Zero provisional and one final publication, no new physics, water authorization, phase, WB14, soil, vegetation or BGC solve, and no restart representation or fallback fresh-physics path.
- INV-LSE-161/C-016 (`map-custody`) with SnowEnergy INV-086: Initial yields physical trial only. Later trials first validate complete physical/discrete identities and create a private non-Clone pending map. Outer candidate-versus-own-output nonclosure permits history continuation; outer closure with nonclosing authenticated dependent prior output is AdaptiveRefinement and allows neither history retention nor envelope construction. If both pass, consume the **same pending map** to construct FinalAccepted, then final V8/vegetation/BGC envelope once. Constructor failure is not a history/fallback candidate. Roles are Initial ordinal 0, FixedPointAdjacent ordinal 1, then multisecant n at ordinal n+1, with n≤5: accepted maps use 2–7 physical evaluations inside the canonical ceiling 8. Role/ordinal misuse rejects before physical work; identity/regime/custody leaks are identity failure; actual downstream physical errors retain their typed vocabulary. Maps do not publish; coupled-time parent publication does, with complete rollback on failure.
- SnowEnergy INV-085's pending soil-parent-finalization proof is same-tick and opaque. It binds independently finalized V11 successor, persistent vegetation predecessor, complete parent checkpoint/clock owners, LSE/BGC source and exact resident soil continuation/prepared/accepted seals. Only the preauthorized soil installation may differ before parent finalization reproduces that V11 result; no adjacency-based reconstruction, replay, nonsoil mutation or rebasing.
- Vegetation V11 INV-121–128 and 134 retain exact admitted duration bits for every duration-sensitive constitutive operation, sequential staged state/resources, once-only ordered material accumulation and one parent sequence increment. Occupancy D/A/F receipts are not shared-owner ending inventories. Water/N transitions need complete staged hydrology/BGC candidates, exact linked typed other fluxes and no overbooking; both mineral species share the BGC candidate. Apply the stratum semantic ordering/domain constraints when those rows are present. Restart V3 contains the complete unchanged V2 checkpoint plus the accepted prefix only and exactly seven complete owners per accepted slab; reconstruct and execute only runtime-supplied unaccepted suffix against an independent parent-beginning oracle. Digest-only restore, checkpoint-derived uninterrupted oracle and replaying accepted work are invalid. Trusted in-process validation-once proofs cannot cross mutation, untrusted executor, external, restart or durable publication boundaries.

`solve-boundary#solve-boundary`, INV-LSE-108–110 preserve the accepted-solve witness required to identify legitimate primitives even though this exercise does not implement the solver. Canonical unknowns follow typed tile/occupancy configuration order and V8 block, shared Tc/qc, surface and soil layer 1..N. Physical admission precedes T bounds 200–350 K and q bounds 0–0.1. Centered finite-difference probes use minus then plus with sqrt(eps) dimensional scaling and inward one-sided treatment only at the admitted bound. Deterministic LU ties choose the lowest row, with the prescribed 64*eps norm singularity test. Accepted no-update witnesses still use the first full trial; if required, bounded halving 1..20 seeks the first domain-valid witness. The ordinary strict-descent search is b=0..20, with at most 50 installed iterations. ci is diagnostic, not an added step threshold. Energy residual tolerance is `1e-6 W/m²+1e-10*max(1,sum_abs)`; water `1e-12 kg/m²/s+1e-9*scale`; accepted step thresholds include 1e-8 K, 1e-12 q, 1e-7 hydraulic mm and 1e-10 beta. No exact receipt fixed point, new solver or tolerance substitution may be inferred from an exact-state witness.

Ordered failure precedence remains malformed serialization; identity/topology/owner; missing/duplicate; nonfinite; unsupported constitutive domain; request bounds; singularity; backtracking; iteration limit; accepted-step/residual; component closure; control-volume closure; cross-owner join. Retain available ordered IDs/residuals, iteration/backtrack/step/bracket/pivot diagnostics and rollback hashes, never a failed iterate as accepted physics. Errors 030–040 distinguish domain, identity, radiation/turbulence, resource, numerical, closure, enthalpy, ground/atomic, ingress, condensation and soil; 041–044 support/event/regime/snow-soil; 045–048 V3 identity/vapor/phase/chronology; 049 exact soil and 050 exact surface. Any failure preserves vegetation, hydrology, LSE, BGC, soil, runner/envelope, WB14 parent and receipt/publication/checkpoint bytes.

## Required evidence and tests; no fabricated result

A numerical answer requires complete accepted beginning/end exact highs/carries and liquid/ice/soil water states; model/configuration and frozen schema identities; topology/fractions/areas and thermal/radiative/aerodynamic coefficients; exact support and branch/participant receipts; current-trial component T/q/areas, forcing density/cp/pressure/radiation/wind, saturation and latent primitives; every beginning/end conductance and soil flux; potential D, authorization A and finalized F with phase keys; independent phase prestate/transfers/raw state/spill; every liquid source mass, window, temperature/h/energy; actual WB14 configuration/cumulative state and I/E output; retention/infiltration/runoff/routing receipts; canonical energy operands and owner transitions; accepted map/prefix/finalization provenance; restart/publication lineage where claimed. None of this actual bundle was supplied. Neither a producer “closure=0” nor an end-state total would fill the gap.

Required independent tests are concrete, not reported passes:

1. Physical all-distinct operands, heterogeneous nonunit tile fractions, unequal OFE areas and support durations: reconstruct shortwave bands, reciprocal component longwave, shared air H/v closure, signed full vapor energy, beginning/end CN ground transfer for N=1 and N>1, bottom zero, surface/soil storage and advection. Reverse longwave/G signs, include night/zero shortwave, dry/wet and open/covered controls, finite-capacity/equilibrium-zero and alternate warm starts. Individually omit, duplicate, swap or wrong-basis every storage/flux/water lineage. Poison direct reference-air ground exchange, bulk/stale radiation, omitted ground shared-air terms, PMET/current-ingress donation and duplicated soil temperature.
2. V3 phase tests include empty/liquid/ice/mixed beginnings, evaporation/sublimation/condensation/deposition signs and separate positive caps, exact T_ref and both sides, dt below/equal/above tau, exact 60-second accepted floor and substantially larger stable supports. Reconstruct phase-only mass and `U-L_f*W_i`, ending capacity and temperature. Poison rho_i capacity, rho_i=917, T_ref=273.16, incorrect fusion constant/sign, ice saturation, latent-only energy, total-store cap, cross-pool/current-ingress donation, missing deposition, authorization-as-use, double ET debit, old-capacity temperature increment, instantaneous equilibrium and a same-support re-solve.
3. C-011 spill tests: below/at/above liquid capacity including melt-created spill, mandated two subtractions, immutable raw receipt, signed sensible enthalpy, exact named negative surface operand, full child support and checked tile/OFE conversion. Prove once-only WB14 supply and independently retained/infiltrated/routed outcomes. Poison phase/key/support/transaction/temperature/basis/receipt substitutions and condensation/caller aliases. C-012 mixed native/ordinary resources include zero use and F<A, exact row joins/order, no missing mixed debit, duplicate native use, owner replacement or extra parcel.
4. Water tests include full/partial/dry competing supplies, common-scale representational overshoot and caller-order reversal, strict own-pool limits, ordinary condensation overflow, all canopy release classes, mixed-temperature rain/runon, stateful 48-child WB14 continuation, two unequal OFEs across two accepted children, one parent cursor increment, source/remainder partitions and both sides of the narrow tiny-retention threshold. Tiny positive energy must survive carry even when high is unchanged; tiny room cannot destroy runoff. Test exact first-up liquid-reference normalization and below/reference/second-up exclusions.
5. P-005/C-005 and P-006/C-006 exact-carry tests: both signs, cancellation, sub-ULP credit, halfway nearest-even, high-boundary crossing, subnormal, overflow, canonical carry wire, malformed sign/hex/exponent, source omission/duplication/order/key/support poisons, high-mirror mismatch and restart. Independently regenerate physical operands, not the producer carry or end-state subtraction. WAT5's named high `-34315.42154113602` plus credit `-8.0670339832330148e-19` and carry `(-1,"1dc319224e55f",-109)` is a **contract test vector**, not this task's run result. The retained p61 support `176400000000000..178200000000000 ns` is a required evidence location; missing highs/credits cannot be supplied from an invented oracle.
6. C-008/C-010/C-013/C-014/C-015/C-016 tests prove partial markers and final once-only stamp, original-prepared replay, opaque configuration rank, private proof binding and invalidation, full external/restart validation, provisional/final reseal counts, no added physics, role/ordinal bounds, pending-map history versus dependent rejection versus constructor failure, exact byte rollback and no durable proof. Prefix tests prove zero repeated Stage 3/litter/WB14/routing and first new ordinal zero. Vegetation V11 tests cover unequal segmentation, event endpoints, zero remainder skip, mid-parent seven-owner restart, full-support V10 nonidentity equality, shared inventory overbooking and complete suffix equality against a frozen parent-beginning oracle.
7. Actual direct-consumer evidence remains necessary before an implementation claim: the named `erosion_single_ofe_p61_sediment.rs` and `dff_ws1_native_forest_cli.rs` workflows must execute with accepted primitives, preserve/reload and continue complete owners, independently close physical and exact ledgers, and retain typed poison rollback. A source inspection, cargo check/build, contract fixture or comparator agreement does not execute those consumers. A0/A1 and applicable A3 remain binding; no legacy-only agreement or missing measured data waives them. No implementation or test binding was changed here, so this reading report invokes no production or authority-suite execution.

Before sealing this first report I checked the selected rules and marked definitions against the domain, chronology, exclusions, arithmetic, inputs and evidence above. Static verdict: reconstruction requirements are stated; **numerical physical closure is NOT ESTABLISHED because the actual accepted primitive bundle is absent**. This is an evidence limitation, not a manufactured defect in an unexecuted implementation. Parent retains package adoption/review/verification disposition. No implementation, production execution, promotion or empirical claim follows.

## Provenance and measurement ledger

The source catalog gives a complete-source SHA256 for every path; every request row refers to that immutable catalog hash. Ranges are inclusive source lines and UTF8 bytes include original line endings. An oversized requested end is shown together with effective EOF; no nonexistent bytes are counted. Heading-only search rows count the actual returned matching source line, not a fictional complete read. Source-file bytes read internally to hash or select ranges are measurement mechanics, not additional model-visible source requests. Initial bootstrap reads were logged retrospectively from their actual selections, and the five initially parallel first-70-line requests were checked to be present after a possible ledger write race; all five are retained. Subsequent recording was sequential.

Exact union accounting counts each selected byte once per path. Requested exposure includes every bootstrap, recursive expansion, recovery and voluntary repeat/search. Repeated exposure is requested bytes minus unique union, not a discount to the actual reading cost. Some recovery requests were initially labeled expansion; their overlap and recovery annotation are explicit. The transcript's truncation warnings motivated the bounded recovery requests; they do not support a delivered-token estimate. Earlier ranges with no retained explicit per-call delivery note are not asserted to have an observed delivery count. No source requested through a truncated response was treated as read solely because its range had been requested: recoveries restored the missing selected text.

Delivered model tokens, cached tokens, total workflow/tool-envelope tokens, compaction exposure, wall-clock workflow telemetry, effective model effort, and host runtime metrics are **UNOBSERVED**. Approximate `original_token_count` output labels are not summed or promoted to measured model-context tokens. No percentage token-saving claim is made. The `context_report.py --help` command returned usage only; it did not request canonical scientific source content. Own temporary reader/ledger/report inspection and directory-name discovery are accounting/workflow operations, separately from canonical source exposure. No Git/history was consulted.

| Measure | UTF8 bytes |
|---|---:|
| external/governance requested | 1950376 |
| external/governance unique | 1488684 |
| external/governance repeated | 461692 |
| phase bootstrap | 95670 |
| LSE requested | 275823 |
| LSE unique | 196206 |
| LSE repeated | 79617 |
| phase expansion | 1879288 |
| phase recovery | 178924 |
| phase voluntary-repeat | 70153 |
| phase voluntary-search | 2164 |
| All source requested exposure | 2226199 |
| All unique source union | 1684890 |
| All repeated requested exposure | 541309 |

These are this agent's actual request bytes, not a minimum hypothetical route. The LSE source union includes all selected LSE chapters and entry; external/governance union is reported separately even where its mandatory whole-contract extent dominates the reading.

| Source ID | Path | Complete SHA256 | Unique requested bytes / complete bytes | Effective selected union |
|---|---|---|---:|---|
| S01 | `AGENTS.md` | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` | 9508 / 9508 | FULL 1–121 |
| S02 | `docs/specifications/science-contracts/AGENTS.md` | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` | 6532 / 6532 | FULL 1–84 |
| S03 | `docs/work-packages/AGENTS.md` | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` | 6642 / 6642 | FULL 1–105 |
| S04 | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` | 5304 / 5304 | FULL 1–75 |
| S05 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` | 1586 / 1586 | FULL 1–22 |
| S06 | `docs/work-packages/role-review.md` | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` | 1400 / 1400 | FULL 1–7 |
| S07 | `docs/standards/AGENTS.md` | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` | 4052 / 4052 | FULL 1–58 |
| S08 | `docs/standards/prompt-wording-guidance.md` | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` | 12057 / 12057 | FULL 1–202 |
| S09 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `930c682e5c923083683e8a030457c9e5e90e6ca9cd2f70a425ee0aa79c326c72` | 4130 / 4130 | FULL 1–65 |
| S10 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | `a827f4099976f9592863b71a12c9059dadd36f8b8037dba4e0ebf8482449c0a0` | 7987 / 7987 | FULL 1–66 |
| S11 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | `29ba7de4a93aa48d770b229fb1ea7fbbc581c2f49c2c61bf5c5697fb8f5f3042` | 30323 / 30323 | FULL 1–426 |
| S12 | `docs/work-packages/science-obligations.md` | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` | 6149 / 6149 | FULL 1–98 |
| S13 | `docs/standards/testing-and-gate-strategy.md` | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` | 9860 / 25142 | 1, 8, 17, 33, 52, 77, 129, 131, 137, 164, 172, 182, 193–346, 368, 386, 403, 417, 427, 439–474, 487 |
| S14 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | `ac27766f6f329000a1b948d16006056e6d0780b00867dda7b788c4512fa99194` | 23267 / 23267 | FULL 1–301 |
| S15 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | `5f96696b2956c157848e7215b3655fa074181b9f76ad9159264164b0598fbe85` | 16209 / 16209 | FULL 1–228 |
| S16 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | `58465434d842511c72a5e27653bfa13ee51015f13939f3ad33f2fe8200f6287d` | 13234 / 13234 | FULL 1–208 |
| S17 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | `a1327441160fad148f847c378dec2e9251bc34e81254a973361a61c5f5450190` | 25574 / 25574 | FULL 1–290 |
| S18 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | `e20bd57efa318ab3f82ca31dfd9712732b9d827d3cd38839295a3b570317a9f7` | 20961 / 20961 | FULL 1–245 |
| S19 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | `342b7601176eb263ed3de199c6d9697d987e1650220a615a49c34181e9b27bbd` | 15424 / 15424 | FULL 1–201 |
| S20 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | `055966f4d3373dba5dcb6a99a3d3c56c02bf948fa0182d8bc74bb20c26490361` | 11649 / 11649 | FULL 1–206 |
| S21 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | `fe37a1a64327aa46ffb1afbc63e00aedab67c83083a3eb3ccbd2edbc17d3174e` | 8225 / 8225 | FULL 1–113 |
| S22 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | `4778df5b572c4a09b3dfedc1f5bba4c84d376bcfcd0021ae7e9fbb6200158b8c` | 19223 / 19223 | FULL 1–171 |
| S23 | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` | 184116 / 184116 | FULL 1–2205 |
| S24 | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` | 92552 / 92552 | FULL 1–1108 |
| S25 | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` | 377637 / 377637 | FULL 1–2648 |
| S26 | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` | 497785 / 497785 | FULL 1–3559 |
| S27 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` | 248829 / 248829 | FULL 1–3080 |
| S28 | `docs/prompt_templates/assurance-findings-template.md` | `95373d3c7df1f0a7e3ad7640fa22c0e137095e6cf5c4d92e3803846eb3f6ee83` | 736 / 736 | FULL 1–12 |
| S29 | `docs/numerics/README.md` | `b87322b3b30defa57bf7365a8cc16f0719ee2e9aa14b8bc12cdcfb39455e5a8b` | 2221 / 2221 | FULL 1–45 |
| S30 | `docs/specifications/correctness-authority-model.md` | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` | 11159 / 11159 | FULL 1–221 |
| S31 | `docs/decisions/0044-prohibit-accretive-production-solver-dispatch.md` | `a597aaf802917e1ed9c21db1f4453c571dde09ad48684c0f7d0ff9062b6645e6` | 3817 / 3817 | FULL 1–79 |
| S32 | `docs/standards/numerical-solver-architecture.md` | `46089ca55bb565b7dc67ea1b2075bbcc16db6902eeba1952572b072abadd0738` | 6742 / 6742 | FULL 1–152 |

Actual source requests, in recorded order (parallel bootstrap selections retain their ledger order):

| # | Source/hash catalog ID | Inclusive requested range | Requested UTF8 bytes | Bootstrap/expansion/repeat | Repeats, truncation and recovery notes |
|---:|---|---|---:|---|---|
| 1 | S01 | 1–121 | 9508 | bootstrap | complete |
| 2 | S02 | 1–84 | 6532 | bootstrap | complete |
| 3 | S03 | 1–105 | 6642 | bootstrap | complete |
| 4 | S04 | 1–75 | 5304 | bootstrap | complete |
| 5 | S05 | 1–22 | 1586 | bootstrap | complete |
| 6 | S06 | 1–7 | 1400 | bootstrap | complete |
| 7 | S07 | 1–58 | 4052 | bootstrap | complete |
| 8 | S08 | 1–202 | 12057 | bootstrap | complete |
| 9 | S09 | 1–65 | 4130 | bootstrap | complete |
| 10 | S10 | 1–66 | 7987 | bootstrap | Combined third source response wrapper truncated; litter-phase recovery required |
| 11 | S11 | 1–426 | 30323 | bootstrap | Combined third source response wrapper truncated; litter-phase recovery required |
| 12 | S12 | 1–98 | 6149 | bootstrap | Combined third source response wrapper truncated; litter-phase recovery required |
| 13 | S13 | 1–1 | 34 | expansion | heading-only rg discovery |
| 14 | S13 | 8–8 | 14 | expansion | heading-only rg discovery |
| 15 | S13 | 17–17 | 17 | expansion | heading-only rg discovery |
| 16 | S13 | 33–33 | 35 | expansion | heading-only rg discovery |
| 17 | S13 | 52–52 | 17 | expansion | heading-only rg discovery |
| 18 | S13 | 77–77 | 30 | expansion | heading-only rg discovery |
| 19 | S13 | 129–129 | 25 | expansion | heading-only rg discovery |
| 20 | S13 | 131–131 | 18 | expansion | heading-only rg discovery |
| 21 | S13 | 137–137 | 26 | expansion | heading-only rg discovery |
| 22 | S13 | 164–164 | 28 | expansion | heading-only rg discovery |
| 23 | S13 | 172–172 | 25 | expansion | heading-only rg discovery |
| 24 | S13 | 182–182 | 30 | expansion | heading-only rg discovery |
| 25 | S13 | 193–193 | 28 | expansion | heading-only rg discovery |
| 26 | S13 | 219–219 | 26 | expansion | heading-only rg discovery |
| 27 | S13 | 224–224 | 14 | expansion | heading-only rg discovery |
| 28 | S13 | 230–230 | 22 | expansion | heading-only rg discovery |
| 29 | S13 | 235–235 | 22 | expansion | heading-only rg discovery |
| 30 | S13 | 240–240 | 13 | expansion | heading-only rg discovery |
| 31 | S13 | 274–274 | 44 | expansion | heading-only rg discovery |
| 32 | S13 | 292–292 | 26 | expansion | heading-only rg discovery |
| 33 | S13 | 314–314 | 42 | expansion | heading-only rg discovery |
| 34 | S13 | 346–346 | 25 | expansion | heading-only rg discovery |
| 35 | S13 | 368–368 | 39 | expansion | heading-only rg discovery |
| 36 | S13 | 386–386 | 17 | expansion | heading-only rg discovery |
| 37 | S13 | 403–403 | 34 | expansion | heading-only rg discovery |
| 38 | S13 | 417–417 | 35 | expansion | heading-only rg discovery |
| 39 | S13 | 427–427 | 46 | expansion | heading-only rg discovery |
| 40 | S13 | 439–439 | 33 | expansion | heading-only rg discovery |
| 41 | S13 | 457–457 | 26 | expansion | heading-only rg discovery |
| 42 | S13 | 474–474 | 18 | expansion | heading-only rg discovery |
| 43 | S13 | 487–487 | 18 | expansion | heading-only rg discovery |
| 44 | S11 | 190–300 | 6207 | expansion | Repeated overlap 6207 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 45 | S14 | 1–70 | 6001 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 46 | S15 | 1–70 | 4264 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 47 | S16 | 1–70 | 3968 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 48 | S17 | 1–70 | 4590 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 49 | S18 | 1–70 | 5039 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 50 | S14 | 71–260 | 14587 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 51 | S15 | 71–330 (EOF 228) | 11945 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 52 | S14 | 261–400 (EOF 301) | 2679 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 53 | S16 | 71–350 (EOF 208) | 9266 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 54 | S19 | 1–201 | 15424 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 55 | S17 | 71–360 (EOF 290) | 20984 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 56 | S20 | 1–206 | 11649 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 57 | S18 | 71–400 (EOF 245) | 15922 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 58 | S21 | 1–113 | 8225 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 59 | S22 | 1–180 (EOF 171) | 19223 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 60 | S21 | 45–120 (EOF 113) | 5542 | expansion | Repeated overlap 5542 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 61 | S23 | 1–400 | 23325 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 62 | S23 | 401–850 | 54928 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 63 | S23 | 665–750 | 31587 | expansion | Repeated overlap 31587 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 64 | S23 | 851–1150 | 20179 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 65 | S23 | 705–725 | 15627 | expansion | Repeated overlap 15627 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 66 | S23 | 970–995 | 1738 | expansion | Repeated overlap 1738 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 67 | S23 | 1151–1450 | 17817 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 68 | S23 | 1451–1800 | 37473 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 69 | S23 | 1801–2205 | 30394 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 70 | S24 | 1–400 | 23523 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 71 | S24 | 401–760 | 40364 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 72 | S24 | 761–1108 | 28665 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 73 | S25 | 1–500 | 179457 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 74 | S25 | 200–350 | 99704 | expansion | Repeated overlap 99704 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 75 | S25 | 175–200 | 3571 | expansion | Repeated overlap 3571 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 76 | S25 | 240–290 | 48421 | expansion | Repeated overlap 48421 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 77 | S25 | 351–410 | 16236 | expansion | Repeated overlap 16236 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 78 | S25 | 265–275 | 10917 | expansion | Repeated overlap 10917 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 79 | S25 | 291–302 | 11871 | expansion | Repeated overlap 11871 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 80 | S25 | 351–380 | 11639 | expansion | Repeated overlap 11639 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 81 | S25 | 381–420 | 6965 | expansion | Repeated overlap 6965 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 82 | S25 | 421–445 | 8915 | expansion | Repeated overlap 8915 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 83 | S25 | 446–455 | 4702 | expansion | Repeated overlap 4702 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 84 | S25 | 501–600 | 27719 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 85 | S25 | 550–565 | 6130 | expansion | Repeated overlap 6130 B.  Expansion included truncation recovery; retained as a separate actual request. |
| 86 | S25 | 601–900 | 32183 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 87 | S25 | 901–1300 | 22679 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 88 | S25 | 1301–1700 | 23263 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 89 | S25 | 1701–2000 | 16031 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 90 | S25 | 2001–2350 | 20530 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 91 | S25 | 2351–2648 | 55775 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 92 | S25 | 2490–2570 | 21373 | recovery | Repeated overlap 21373 B.  Bounded recovery of previously requested text. |
| 93 | S26 | 1–250 | 15490 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 94 | S26 | 251–500 | 17209 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 95 | S26 | 501–750 | 56594 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 96 | S26 | 590–645 | 27264 | recovery | Repeated overlap 27264 B.  Bounded recovery of previously requested text. |
| 97 | S26 | 751–1000 | 12017 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 98 | S26 | 1001–1250 | 13678 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 99 | S26 | 1251–1350 | 38260 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 100 | S26 | 1318–1328 | 6350 | recovery | Repeated overlap 6350 B.  Bounded recovery of previously requested text. |
| 101 | S26 | 1351–1450 | 92304 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 102 | S26 | 1385–1410 | 12238 | recovery | Repeated overlap 12238 B.  Bounded recovery of previously requested text. |
| 103 | S26 | 1411–1425 | 20037 | recovery | Repeated overlap 20037 B.  Bounded recovery of previously requested text. |
| 104 | S26 | 1426–1440 | 32918 | recovery | Repeated overlap 32918 B.  Bounded recovery of previously requested text. |
| 105 | S26 | 1441–1447 | 17564 | recovery | Repeated overlap 17564 B.  Bounded recovery of previously requested text. |
| 106 | S26 | 1451–1550 | 15579 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 107 | S26 | 1515–1550 | 9620 | recovery | Repeated overlap 9620 B.  Bounded recovery of previously requested text. |
| 108 | S26 | 1551–1565 | 13129 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 109 | S26 | 1566–1595 | 7124 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 110 | S26 | 1596–1790 | 18438 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 111 | S26 | 1791–2040 | 29544 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 112 | S26 | 2041–2170 | 36751 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 113 | S26 | 2171–2420 | 16018 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 114 | S26 | 2421–2770 | 24912 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 115 | S26 | 2771–3020 | 51176 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 116 | S26 | 2900–2940 | 26742 | recovery | Repeated overlap 26742 B.  Bounded recovery of previously requested text. |
| 117 | S26 | 3021–3320 | 20405 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 118 | S26 | 3321–3559 | 19157 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 119 | S27 | 1–250 | 21653 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 120 | S27 | 251–450 | 15166 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 121 | S27 | 451–650 | 13279 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 122 | S27 | 651–750 | 6615 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 123 | S27 | 751–900 | 9793 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 124 | S27 | 901–1100 | 32122 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 125 | S27 | 1101–1350 | 19100 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 126 | S27 | 1351–1600 | 33467 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 127 | S27 | 1601–1850 | 16275 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 128 | S27 | 1851–2100 | 13201 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 129 | S27 | 2101–2350 | 17527 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 130 | S27 | 2351–2600 | 15961 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 131 | S27 | 2601–2850 | 15698 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 132 | S27 | 2851–3080 | 18972 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 133 | S11 | 301–340 | 2977 | recovery | Repeated overlap 2977 B.  Bounded recovery of previously requested text. |
| 134 | S23 | 950–975 | 1841 | recovery | Repeated overlap 1841 B.  Bounded recovery of previously requested text. |
| 135 | S13 | 193–345 | 8067 | expansion | Repeated overlap 237 B.  |
| 136 | S13 | 439–473 | 1262 | expansion | Repeated overlap 59 B.  |
| 137 | S28 | 1–9999 (EOF 12) | 736 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 138 | S29 | 1–9999 (EOF 45) | 2221 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 139 | S30 | 1–9999 (EOF 221) | 11159 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 140 | S31 | 1–9999 (EOF 79) | 3817 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 141 | S04 | 1–75 | 5304 | voluntary-repeat | Repeated overlap 5304 B.  |
| 142 | S32 | 1–9999 (EOF 152) | 6742 | expansion | No per-request delivered-token telemetry; source selection recorded. |
| 143 | S09 | 1–65 | 4130 | voluntary-repeat | Repeated overlap 4130 B.  |
| 144 | S11 | 1–189 | 9542 | voluntary-repeat | Repeated overlap 9542 B.  |
| 145 | S15 | 1–130 | 7471 | voluntary-repeat | Repeated overlap 7471 B.  |
| 146 | S20 | 1–206 | 11649 | voluntary-repeat | Repeated overlap 11649 B.  |
| 147 | S11 | 112–112 | 42 | voluntary-search | Repeated overlap 42 B. exact selected matching line; not full-file exposure |
| 148 | S23 | 97–97 | 80 | voluntary-search | Repeated overlap 80 B. exact selected matching line; not full-file exposure |
| 149 | S23 | 1084–1084 | 75 | voluntary-search | Repeated overlap 75 B. exact selected matching line; not full-file exposure |
| 150 | S26 | 1000–1000 | 31 | voluntary-search | Repeated overlap 31 B. exact selected matching line; not full-file exposure |
| 151 | S26 | 1088–1088 | 24 | voluntary-search | Repeated overlap 24 B. exact selected matching line; not full-file exposure |
| 152 | S26 | 1092–1092 | 77 | voluntary-search | Repeated overlap 77 B. exact selected matching line; not full-file exposure |
| 153 | S26 | 1161–1161 | 72 | voluntary-search | Repeated overlap 72 B. exact selected matching line; not full-file exposure |
| 154 | S26 | 1162–1162 | 34 | voluntary-search | Repeated overlap 34 B. exact selected matching line; not full-file exposure |
| 155 | S26 | 1331–1331 | 1234 | voluntary-search | Repeated overlap 1234 B. exact selected matching line; not full-file exposure |
| 156 | S26 | 1541–1541 | 495 | voluntary-search | Repeated overlap 495 B. exact selected matching line; not full-file exposure |
| 157 | S14 | 1–301 | 23267 | voluntary-repeat | Repeated overlap 23267 B.  |
| 158 | S15 | 130–228 | 8790 | voluntary-repeat | Repeated overlap 8790 B.  |
