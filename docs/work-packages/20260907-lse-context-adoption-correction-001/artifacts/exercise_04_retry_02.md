# Exercise 04, fresh retry 02 — independent frozen-litter closure reading

Static: first-attempt reconstruction from the current canonical candidate route. Ran: file/range reads and structural source measurement only. No physical model, numerical oracle, production test, comparator, or primitive-receipt reconstruction was executed. No run-specific primitive bundle was supplied. This report specifies the reconstruction that such a bundle must permit; it does not report a measured residual or a passing physical closure.

The exact assignment was to independently reconstruct snow-free frozen-litter surface/soil physical energy and mass closure from accepted primitive receipts, identify rules/anchors, owner/regime boundaries, prohibited shortcuts, required inputs/tests, and explain reading expansion. I read no original monolith, Git history, predecessor/archive candidate, intake/prediction/rubric/review, previous exercise answer, parent test, or other agent material. No scientific hints were requested. Canonical source hashes were checked unchanged during measurement. Only this report is a repository write.

## Authority route and expansion decisions

The entry contract is candidate revision 32, directory-v1. Its authorization for selective reading is expressly limited to this package's bounded candidate exercises; the whole normative set remains binding. I selected its **Frozen-litter closure / closure review** route, reading the full interface and litter-phase chapter, then each selected chapter's dependency table and introduction. References below resolve in `docs/specifications/science-contracts/contracts/`; `LSE/` denotes `SC-LANDSURFACEENERGY-001/`. Original `v31:L…` provenance annotations were not followed into historical bytes.

| Expansion | Reason and extent |
|---|---|
| `LSE/common-details.md` | Interface mandates the whole shared physical chapter for accepted-primitive closure: symbols, state, signs, algorithms, guards, tolerances, tests. |
| `LSE/water-vapor.md`, `soil-coupling.md`, `surface-custody.md` | Litter phase expressly requires these whole chapters: reconstruct vapor, current-ingress advection, opposite soil flux, and exact surface receipts. |
| `LSE/surface-energy.md`, `solve-boundary.md`, `soil-custody.md` | Physical operands and accepted-state origin require current radiation/turbulence, potential/fixed-cap chronology and soil exact-energy receipt custody. Whole chapters read. |
| `LSE/terminal-support.md` | Its dependency/introduction and complete `#support` section, plus applicable marked invariant/guard links, bind physical interval admission, zero support and the pre-Newton floor even on snow-free work. Selected ranges listed below. Terminal snow handoff is not this task. |
| `LSE/map-custody.md` | Receipt-origin and trusted-handoff dependencies require full chapter: a digest or completed nonfinal map is not accepted physical custody. |
| `LSE/audit-details.md` | Expanded to the whole chapter to distinguish current superseding rules from retained future/missing labels and to check exact-carry enforcement and consumer limits. |
| `SC-SURFACELIQUID-001.md` | Whole external single-file contract, not only linked frozen-litter/spill/join sections. It owns both phase masses, partition/retention, exact recipient custody and rollback. |
| `SC-WATBAL-001.md` | Whole external single-file contract. Actual WB14 infiltration, runoff and hydrology ownership are required for surface/soil mass closure. |
| `SC-VEGETATION-001.md` | Whole external single-file contract. Covered surfaces require current shared canopy air, reciprocal longwave, accepted wet-release enthalpy and potential/final water chronology. |
| `SC-COUPLEDTIME-001.md` | Whole external single-file contract. Support bits, accepted parent/slab chain, physical-map disposition, reseal, publication and restart custody must join. |
| Governance expansion | Review role requires testing strategy §§7–10,17–18 and applicable science obligations; §7 directs the correctness-authority model, read fully. Science-obligations read fully for anti-tautology, consumer and empirical-claim boundaries. |

No implementation or full nonlinear-solver correctness review was requested; `nonlinear-solve.md` therefore was not expanded. No optimization equivalence claim was made; `dependency-replay.md` was not expanded. No executable-identity capture, qualification experiment or production-qualification claim was performed; source-byte measurement is not that workflow, so `qualification.md` was not expanded. No snow-present/terminal native physical reconstruction was claimed; the snow external contracts were not expanded merely because whole selected chapters preserve conditional snow text. References to earlier work-package definitions and source provenance do not make their historic artifacts an independent acceptance oracle for this exercise.

## Regime, owner and temporal boundaries

`LSE/interface.md#interface`, `common-details.md`, `solve-boundary.md`, and `litter-phase.md#version-14-snow-free-frozen-forest-litter-successor-amendment` govern the domain. `OPENWEPP_SNOW_FREE_LSE_V3` is the frozen-forest-litter successor; it imports V2, including V1 physics and V10 vegetation specialization, and changes only the admitted litter phase. V1/V2 historical model/configuration/state/receipt/restart bytes remain immutable. V3 does not extend bare-mineral, ponded, frozen-soil, snow-present or terminal authority. Litter ice is water equivalent in the litter owner, not ground snow SWE or soil `frozwt` and not WB14 liquid supply. The represented-snow classifier selects the separate native map and retains inactive litter/WB14 state; no duplicate LSE envelope or litter phase executes there. Terminal input is rejected unless its separate authority applies.

LSE owns surface thermal state and exact surface energy; SurfaceLiquid/hydrology owns litter liquid and ice and real water partition; soil thermal owns the ordered soil temperatures and exact layer energy; vegetation owns canopy physiology/interception; meteorology owns forcing and rain-phase temperature lineage. None may invent another owner's state. Soil hydrology alone authorizes/debits its liquid inventory. Existing daily ET must not debit the accepted subdaily vapor again.

`LSE/terminal-support.md#support` and `SC-COUPLEDTIME-001#INV-COUPLEDTIME-001/002/004/005/006/017` require exact half-open integer-nanosecond support, one common duration-bit operand, immutable beginning owners and accepted-only chronology. The LSE floor is exactly **60,000,000,000 ns**; one tick below rejects before Newton (`LSEB-E-041`), not via scaled/frozen fluxes. Zero duration is event/skip, never a physical solve. Active common minimum is the maximum of the actual active participants' admitted minima. Stable ordinary supports must accept substantially larger steps; the old 0.6-second evidence does not qualify this boundary. Retry uses the canonical smaller-support response from immutable owners, with typed failure at the floor, never a replacement solver or floor split.

`LSE/solve-boundary.md` requires: beginning snapshot before current rain/runon/canopy release; joint potential solve and immutable requests; one hydrologic authorization; complete final solve rebuilt from the original beginning under fixed caps; final-use validation; then current ingress. Potential is owner-uncapped, not hydraulically unstressed. For a valid capped law, cap equality selects the cap-active branch and zero generalized derivative. Actual use, not demand or authorization, is debited, with exact `0 <= F <= A <= D`. A converged aggregate residual does not excuse a failed component or mismatched owner receipt.

## Reconstruction from physical primitive operands

Every amount must retain its typed owner, OFE/tile/source, phase, area basis, support, duration bits, model/configuration, beginning/ending state and receipt lineage. Use local tile-ground equations before exactly one area weighting. Inward energy is positive; outward mass is nonnegative. Distinguish the general inward `G` convention from the constitutive downward `G_s1`: the surface debits exactly what soil credits.

### Surface energy and soil conduction

`LSE/common-details.md` gives the component balance

`E1-E0 = dt*(R_sw + R_lw + H + LE + Q_p + Q_runon - Q_inf - Q_runoff + G)`.

This is a ledger identity, not permission to reuse a universal net-energy scalar. The phase-free constitutive surface equation in `LSE/surface-energy.md` and `soil-coupling.md` is

`(U_pre-U0)/dt = R_sw + R_lw - H_s - Q_v - Gbar_s1`.

Current ingress is applied later as its independently named sensible-energy receipts. Do not add rain/runon/infiltration/runoff again if already partitioned into those later owner terms. Signed sensible liquid enthalpy relative to 273.15 K is not required to be numerically nonnegative merely because outward liquid mass is nonnegative.

Shortwave retains each VIS/NIR and direct/diffuse component. Actual ground albedo is the two-stream lower boundary; terminal transmitted radiation reaches ground once, and reflected radiation traverses the overlying column. Covered longwave uses current component temperatures, exact unit emissivity/no reflection, `tau_i=exp(-0.8*Omega_i*(LAI_i+SAI_i))`, component emissive-area weights, and reciprocal up/down interface recurrence. Ground net longwave is `Ldn_ground-sigma*T_s^4`; a bulk or stale canopy/ground temperature is not an alias. Empty component area has zero owned flux.

Sensible heat is `H_s=rho_a*c_p*(T_s-T_recipient)/r_h`. Open ground exchanges with reference air through the admitted neutral logarithmic resistance; covered ground exchanges once with the one shared tile canopy-air `(T_c,q_c)` node, whose heat and vapor residuals include every canopy component and exactly one ground term. No occupancy-local node or reference-air bypass can substitute. Radiation, gas/energy, hydraulic and node operands must all refer to the accepted common current state. Vegetation V10 exact-zero/respiration-dominated light branches retain their specific full-supply-only root-authorization restriction; blanket V5 capped behavior is not permission to bypass it.

For soil layer `k`, use its own thickness/conductivity/capacity and temperature. The surface-to-top-layer conductance is `g_s1=2/(dz_s/lambda_s+dz_1/lambda_1)` and `G_s1=g_s1*(T_s-T_1)` positive down. Interior interfaces use the corresponding harmonic conductance. `Gbar=0.5*(G_begin+G_end)` and

`C_k*(T_k,end-T_k,begin)/dt = Gbar_in,k-Gbar_out,k`.

The bottom boundary is zero; the single-layer case has no invented interior face. All interior exchanges cancel only after each adjacent pair is checked equal/opposite. A finite-capacity beginning surface temperature is derived from its physical energy/masses/capacity; the exact dry zero-capacity equilibrium branch uses the current algebraic surface temperature at both endpoints, not an old warm start. Forest litter conductivity is `0.1+0.03*W_l/(rho_w*dz_l)` and dry heat capacity is `dz_l*rho_ld*c_ld`. Litter blocks mineral-soil evaporation and capillary upward supply in this model. None of this authorizes frozen-soil phase change.

### Separate signed liquid and ice vapor

`LSE/litter-phase.md#v3-state-phase-free-solve-and-signed-vapor` and `SC-SURFACELIQUID-001#INV-SURFACELIQUID-017/018` require the immutable beginning frozen fraction

`p_i=0` for the empty pool, otherwise `W_i0/(W_l0+W_i0)`.

The humidity factors use their own beginning phase stores: `h_ul=0.5*(1-cos(pi*W_l0/W_l,max))`, and the analogous `h_ui` with `W_i0/W_i,max`. The uncapped laws are

`v_l=(1-p_i)*rho_a*(h_ul*q_sat_liquid(T_l,p)-q_recipient)/r_v`,

`v_i=p_i*rho_a*(h_ui*q_sat_liquid(T_l,p)-q_recipient)/r_v`.

Both use the same admitted **liquid-water saturation** expression, including for ice. Positive liquid evaporation and ice sublimation receive separate own-beginning-pool caps; neither phase borrows the other's mass or current ingress. Negative values are uncapped named-phase condensation/deposition credits. Keep their mass/energy receipts separate through acceptance, then aggregate only for the shared-air residual.

With `T_ref=273.15 K`, the separate outward energy rates are

`Q_v,l=v_l*(C_w*(T_l-T_ref)+L_v(T_l))`,

`Q_v,i=v_i*(C_i*(T_l-T_ref)+L_s(T_l))`.

`L_v(T)=2.501e6-2369*(T-T_ref)` in J/kg is the imported liquid law. An actual reconstruction also needs the authority-tagged `L_s(T)` operand/law and exact saturation/configuration inputs: no run value was supplied, and I do not invent a sublimation constant or evaluate an unprovided receipt. The selected phase chapter explicitly names `L_s(T)` rather than supplying a replacement numerical default.

Reconstruct `W_l*=W_l0-dt*v_l`, `W_i*=W_i0-dt*v_i` and the phase-free sensible energy `U*=(C_dry+C_w*W_l*+C_i*W_i*)*(T*-T_ref)`. Independently reconstruct `U*` against **all** accepted radiation, sensible, signed liquid/ice vapor and soil-flux operands. SurfaceLiquid's vapor-only custody subledger is not a whole-surface energy equation. A producer-supplied residual, phase receipt alone or an exact carry alone cannot prove this step.

### Bounded phase and fusion

`LSE/litter-phase.md#bounded-kinetic-phase-and-fusion-energy-closure` selects `rho_w=1000`, `rho_i=920 kg/m3`, `C_w=4218`, `C_i=2106 J/(kg K)`, `L_f=333700 J/kg`, `tau_ice=3300 s`, and `W_i,max=0.85*rho_w*dz_l`. R-156 Appendix A A1–A4/A7–A14 and the retained SURFEX instantiation are the contract's provenance. The printed R-156 A4 sign is adjudicated by mass conservation and executable melt ordering; the selected signed amount is freeze minus melt. I cite that canonical adjudication, not an independently performed legacy/source audit.

From the post-vapor, pre-ingress state:

```text
M_warm = rho_i*C_i*dz_l*max(T*-T_ref,0)/L_f
M_cold = rho_i*C_i*dz_l*max(T_ref-T*,0)/L_f
m_melt = min(W_i*, (dt/tau_ice)*min(M_warm,W_i*))
m_freeze = min(W_l*, W_i,max-W_i*, (dt/tau_ice)*min(M_cold,W_l*))
m_phase = m_freeze-m_melt
W_l,phase = W_l*-m_freeze+m_melt
W_i,phase = W_i*+m_freeze-m_melt
U_phase = U*+L_f*m_phase
C_phase = C_dry+C_w*W_l,phase+C_i*W_i,phase
T_phase = T_ref+U_phase/C_phase
```

At `T*=T_ref` both transfers are zero. The outer donor/capacity limits remain binding when `dt>tau_ice`; there is no maximum-step substitution or instantaneous-equilibrium branch. Prove phase-only mass invariance and `U-L_f*W_i` invariance independently. The changed phase heat capacity owns the ending temperature; applying a literal old-capacity temperature increment is wrong. This operator changes no already accepted same-support vapor/radiation/conduction solve and does not trigger a second Newton, phase or WB14 evaluation.

### Post-phase liquid-capacity spill

`LSE/litter-phase.md#spill`, `INV-LANDSURFACEENERGY-156`, `OBL-LANDSURFACEENERGY-C-011`, and SurfaceLiquid's Exact V3 Litter-Phase Capacity-Spill amendment require preserving raw `(W_raw,W_i,end,U_raw,C_raw,T_raw)` separately. When `W_raw<=W_l,max`, emit the typed positive-zero spill and retain raw state. Otherwise:

```text
m_spill = W_raw-W_l,max
h_spill = 4218*(T_raw-273.15)
Q_spill = m_spill*h_spill
W_retained = W_raw-m_spill
U_retained = U_raw-Q_spill
```

That **second subtraction** is authoritative; assigning/clamping the retained mass directly to capacity is not the same operation. Reconstruct retained heat capacity and temperature. Check positive finite spill, finite energy, in-domain retained liquid, positive heat capacity and exact raw/retained/spill lineage. Reconstruct raw mass and sensible energy as retained plus spill in the specified operation order. Ice and fusion do not become runoff.

The companion receipt binds raw phase digest, configuration, owner, transaction, support, source key, raw/retained states, capacity, temperature, spill mass, specific enthalpy and total energy. The surface exact-energy operand set gains `LitterPhaseCapacitySpillEnergy=-Q_spill` **after** phase-free and fusion operands. Construct OFE spill mass `f_t*m_spill`, then its binary64 energy `m_ofe*h_spill`, and only then exact-decode. The internal `LitterPhaseOverflow` parcel occupies the actual child `[0,dt)` support, enters the single ordinary ingress call once, and cannot be forged as caller rain/condensation or reassigned to another rainfall hour. Any retained return from ingress is a distinct positive receipt, never cancellation/removal of the spill debit.

### Heterogeneous use, ingress, infiltration and routing

`INV-LANDSURFACEENERGY-157` / `C-012`, SurfaceLiquid `INV-004/018/028/029` require joining the accepted phase-adjusted V2 water owner to every finalized row. Match native litter vapor rows exactly to the native phase receipt (transaction/support/OFE/tile/source, `f_t`, `dt`, checked aggregation and finalized-use bits); exclude those rows from ordinary debit because already consumed. Authorization may exceed finalized use. Authenticate and canonically sum every unmatched ordinary row, then debit its checked `F/f_t` once from the **phase-adjusted** owner, not a reconstructed legacy owner. Zero ordinary rows preserve identity. Preserve ice, phase/spill, energy highs/carries and receipts; synthesize no energy operand. No row may be omitted, double-used, reassigned or replaced. The complete row partition covers soil/native/ordinary resources.

`LSE/water-vapor.md`, SurfaceLiquid's hydrologic-ingress specification and WATBAL WB14 authority require ingress only after that join. Open raw rain and covered accepted canopy releases are mutually exclusive per tile. Covered throughfall, initial drainage, second drainage and stemflow retain same-tile routing and their accepted wet-surface temperature; stemflow bypasses lower foliage. Rain uses the retained Harder–Pomeroy hydrometeor temperature, with its named Celsius-to-Kelvin conversion; runon uses exact upstream temperature/enthalpy lineage. Do not substitute air, ground, stale canopy or default temperature.

Every positive parcel carries `h_l=4218*(T_l-273.15)` and `Q=m*h_l`; exact zero mass has exact zero energy and no invented temperature. Mixture `T=273.15+sum(m*h)/(4218*sum(m))` uses actual admitted parcels. Current ingress cannot feed the already finalized same-support ET/phase solve.

WB14 is one real shared OFE infiltration partition per physical child, using the production Green–Ampt/Mein–Larson lineage and persisted parent context. Not one solve per parcel, a scaled daily result, a proxy capacity or an under-snow shadow solve. For a WB14 subinterval with input mass `X` and sensible energy `Q`, use `h_mix=Q/X` (exact zero branch when `X=0`), infiltration `I`, excess `E`, `I+E=X`; `Q_inf=I*h_mix`, `Q_excess=Q-Q_inf`. Preserve source provenance with canonical final-source remainder. Soil receives infiltrated liquid/enthalpy once; surface excess partitions into its own keyed available retention and routed runoff with matching sensible energy. Legacy native depression storage has zero capacity/delta/retention so it cannot duplicate explicit surface storage.

The narrow retention exception is explicit: `tau_M=1e-14+64*epsilon*(abs(f_t*W_max)+abs(f_t*W)+abs(E))`; if `0<M_raw<=tau_M`, retain zero, keep the store unchanged and route the **entire** mass and enthalpy. It is not disappearance. Otherwise retention follows the admitted `min(E,f_t*(W_max-W))`, with `Q_retained_tile=(m_retained*h_mix)/f_t`. Lateral handoff converts upstream to downstream OFE basis exactly once by `A_up/A_down` for both mass and energy and preserves source/support; outlet terms remain external losses. Lane-D/native/local route-owner identities cannot alias.

For each tile, first reconstruct beginning phase water minus signed vapor, internal freeze/melt, spill, ordinary debit and retained ingress as ending phase water; reconstruct each parcel's input = infiltration + retention + runoff separately. Then sum with one tile weight and cancel internal OFE/soil/adjacent transfers, leaving rain/runon/atmospheric exchange/outlet and ending storage. For energy, reconstruct phase-free storage, fusion, spill debit, retained credit and soil credit separately before cancellation. Only the phase-only `U-L_f*W_i` identity has zero phase change; whole-step total-phase energy must also account for vapor phase mass and all external operands. Do not incorrectly assert sensible `U` alone is conserved through freezing.

## Exact high-plus-carry custody is additional to physical closure

`LSE/soil-custody.md#INV-LANDSURFACEENERGY-150` (`P-005/C-005`) and `surface-custody.md#INV-LANDSURFACEENERGY-151` require independent physical reconstruction **and** exact accepted-credit retention. Soil `SoilThermalOwnerEnvelopeV2` and surface `LseSurfaceEnthalpyOwnerEnvelopeV1` carry

`E_exact = exact(binary64_high) + canonical_signed_dyadic_carry`.

A nonzero dyadic has sign ±1, odd positive magnitude in canonical lowercase hexadecimal without a leading zero, and exponent; zero is exactly `(0,"0",0)`. Decode each already-formed finite binary64 physical operand after its prescribed rate/time/area/source-group arithmetic. Add those dyadics to beginning exact storage with unbounded exact integer arithmetic, round **once** to finite binary64 ties-to-even, then carry the exact difference. Overflow rejects. No tolerance, compensated floating sum, arbitrary limited residual width, `nextafter`, sub-ULP credit dropping, or rounding each addition substitutes for exact reconstruction.

Soil sums its accepted soil-energy, opposite top flux and infiltration credits with exact layer/order/receipt joins. Surface sums the separately named accepted phase-free radiation, sensible, liquid-vapor, ice-vapor and ground terms, accepted fusion, the named negative spill, and grouped retained-ingress credits. Retained ingress grouping follows canonical receipt order on OFE basis before `/f_t` and exact decode; exact rational multiplication before the required binary64 conversion is not the contract calculation. Fusion is the admitted binary64 `L_f*(m_freeze-m_melt)`, not a reused aggregate residual.

High fields in frozen V3/V2 are nonauthoritative mirrors that must match the successor high term bit-for-bit. Adoption copies high bits and initializes zero carry; it does not rederive energy from temperature. V1→V2 soil migration preserves high bits and zero carry; downgrade is forbidden even for zero carry. Signed-zero no-op/migration posture is preserved. Carries are custody, not extra temperature, phase mass, latent heat, forcing, tolerance adjustment or feedback into constitutive laws. Complete-owner projections, candidate/receipt hashes, restart and rollback retain them.

`INV-LANDSURFACEENERGY-153` / `C-008` distinguishes a partial child (`end<parent_end`) from final child. Partial children advance exact energy/digests while all required persistent predecessor markers remain the parent's predecessor; only the final child stamps the transaction once. `INV-155` / `C-010` admits only an authenticated read-only unpublished soil continuation from the original prepared owner, authentic predecessor trial and contiguous support. It is not an intermediate owner/restart state or a promotable candidate. Final replay starts at the original owner, includes the complete accumulated physical operand set and seals/installs once.

`INV-158` / `C-013` requires authenticated configuration OFE topology order, then within-OFE key and kind order. IDs are opaque (`ofe-9 -> ofe-10` is not lexical order); parser uniqueness without configuration is insufficient to establish installable order.

## Accepted receipt origin and publication limits

`LSE/map-custody.md#handoff`, `INV-159`, permits private immutable revision-bound validation reuse only on exactly the same live object and lineage. Configuration, topology, transaction, predecessor, support, owner set and resident revision remain exact; a digest alone or wrapper reconstructed from wire is no proof. Static plans and current-map forcing/native proofs apply only at their original validation positions, so error precedence is unchanged. Any mutation invalidates proof. Restart, external/untrusted return and durable boundary require fresh full semantic validation and canonical reconstruction.

`LSE/surface-custody.md#reseal`, `INV-160/C-015`, and coupled-time `INV-029/OBL-012` allow one fully validated provisional snow-free physical execution followed by identity-only final accepted-slab reseal via a private move-only single-use proof. All non-slab inputs and physical endings must remain exact. Zero provisional publication, one final publication; no second phase/ingress/WB14/soil/vegetation physics and no replay fallback. Cross-support, stale, mutated, reused or pre-restart proof rejects atomically; fresh restore executes physical work anew.

`LSE/map-custody.md#pending`, `INV-161/C-016`, and coupled-time `INV-030/OBL-013` distinguish charged map, validated physical endpoint, pending adjudication and final custody. `Initial@0`, then `FixedPointAdjudication@1`, then contiguous `MultisecantAdjudication(n)@(n+1)` give `M=N+2`, `0<=N<=5`, hence two through seven charges under the unchanged eight-map ceiling. Post-initial physical custody must validate before a private pending value exists. Outer candidate-versus-own-output nonclosure consumes it into history without error; outer closure followed by dependent-output instability against the preceding authentic map consumes it into typed adaptive rejection, not history. Full closure consumes that same pending map once as FinalAccepted and constructs one complete private owner envelope. FinalAccepted is a disposition, never an extra physical charge or promotion of a completed nonfinal endpoint. Constructor failure cannot reinterpret it. Ordinary snow-free maps retain LSE, soil, vapor/phase and WB14 physical work; native inactive-litter shortcuts do not apply. Every map/direct/rejected/unselected candidate publishes zero; only accepted composed-parent commit publishes once. Counters alone cannot prove the physical prefix; independent operand and exact differential evidence remain required.

Coupled-time parent acceptance must join complete owners, exact ordered slab/event/scheduled receipts, support coverage and local/global ledgers before the one persistent transaction increment. Restart restores canonical complete owner bytes, exact carries, accepted chronology and outbox state, never rejected iterates or digest-only passengers. A trusted publication-append capability likewise cannot replace independent validation on wire/archive/restart. This reading exercise neither proves those runtime paths nor changes existing production HOLD/qualification status.

## Required run inputs, independent tests and disposition

A genuine run reconstruction requires the complete immutable configuration/model/support/tolerance identities; topology/order/fractions/areas; forcing and canopy/rain/runon temperature lineage; beginning surface phase masses/dry heat capacity, surface high+carry and soil layer high+carry/temperature/grid/material data; final accepted current surface/canopy temperatures and humidity/resistances; distinct shortwave/longwave/sensible/liquid-vapor/ice-vapor/ground rates and integrated primitive amounts; exact request/authorization/finalized-use rows; raw phase, fusion, spill, retained state and matching water-owner receipts; every ingress/mixture/WB14 infiltration/retention/runoff/adjacent/outlet parcel; exact source groups and operation order; final full owner states/mirrors/carries; accepted parent/slab/receipt chain; and restart/rollback/real-consumer surfaces if those claims are made. Missing `L_s(T)`, saturation-law configuration, physical operands or authentic lineage cannot be replaced with guessed defaults or zero residuals.

The reconstruction uses each owning tolerance, not one generous global epsilon. SurfaceLiquid's general mass envelope is `1e-14+64*epsilon*sum(abs(operands))`; its sensible parcel-energy envelope is `1e-9+64*epsilon*sum(abs(operands))`. These do not repair categorical identities, exact `F<=A<=D`, canonical high+carry equalities or wrong area/unit/phase. Adaptive direct/composed truncation tolerances are a separate acceptance axis from physical closure and exact custody. Likewise, a contract's specifically authorized symmetric common-factor roundoff correction for shared authorization does not authorize arbitrary per-row repair, phase-mass clipping or spill snapping.

Required independent populations include:

- Distinct nonzero every-component operands; omit/duplicate/swap/negate each term; local-before-area weighting; unequal tile/OFE areas; adjacent cancellation; surface/soil equal-opposite top flux; bottom-zero and one/multiple soil layers; current versus stale canopy/ground/node temperatures; covered versus open recipients.
- Empty, all-liquid, all-ice and mixed litter; evaporation/sublimation and condensation/deposition separately; donor caps and unused authorization; `T*=T_ref`, warming/cooling, ice-capacity bound; `dt<tau_ice`, equal and greater; exact 60-second floor, one tick below pre-Newton refusal and stable larger supports. Poisons for ice saturation instead of liquid saturation, wrong phase/capacity density, cross-pool/current-ingress donation, printed wrong fusion sign, old capacity temperature and same-support re-solve.
- Below/at/above liquid capacity, melt-created positive spill, zero spill, spill later retained/infiltrated/routed, mass/enthalpy independent reconstruction, raw/retained/receipt/config/source/support/transaction substitutions, wrong area conversion, duplicate ingress and replay. Mixed native-vapor and ordinary finalized rows with omitted/duplicate/foreign rows and zero-ordinary identity.
- Exact positive/negative sub-ULP credits, halfway ties with even/odd high mantissas, cancellation and zero crossing, minimum subnormal, largest finite and overflow, mixed signed energies, multiple unequal tiles/parcel groups, fusion/spill/ingress, omitted/duplicated/misordered operands, stale or noncanonical carry and high-mirror poisons. Exact mathematical sums may be order-independent while physical binary64 grouping and receipt custody order remain binding.
- Partial first/middle/final child markers; contiguous unpublished soil continuation; stale predecessor trial; absence of intermediate owner/restart bytes; one final install; independent final replay. Fresh-object split restart and next-parent continuation must equal uninterrupted complete-owner/carry/receipt/publication bytes; late owner failure must preserve all beginning bytes.
- Private proof nontransfer/reuse/mutation/restart poisons, combined first-error cases, pending exclusive dispositions, no hidden map charge, physical/history/dependent/constructor failure cases, exact physical-prefix differential evidence, zero map publication and one parent publication. Successful counters do not substitute for constitutive tests or closure.

`INV-150/151` and their consumer obligations retain the real WAT5/p61/native-forest paths. Contract-listed examples and stored scalar operands are not a run of this exercise. In particular, p61 and `tests/integration/dff_ws1_native_forest_cli.rs` must use the actual production selector, persist/reload accepted owners and advance a successor with independently reconstructed physical primitive and carry closure. The named p61 failed support `176400000000000..178200000000000 ns` is contextual authority evidence only; the missing actual receipt bundle prevents calculating its residual here. I did not read or run parent tests.

Disposition: **scientific reconstruction specified; numerical physical closure UNEXECUTED / not established**. The reading did not establish implementation correctness, successful production execution, qualification, calibration, identifiability or transferability. A0/A1 and applicable A3, real-consumer proof, exact custody and anti-tautology remain required when implementation/execution is claimed. No data limitation is being used to waive admitted science. The report itself is a bounded static exercise, not permission to repair science, alter thresholds, invent a fixture, reopen history or activate production.

## Reading measurement and recovery ledger

All numbers below are **requested structural source bytes**, encoded UTF-8 including original newlines, measured from working-tree full-file SHA-256 identities. A requested source range is not a delivered-token count. Whole external contracts retain their full cost even when the scientific assignment only applies to a bounded portion. Inclusive line ranges were read through a local recorder, with the initial bootstrap reads entered retrospectively from the actual requested full files. The measurement command was `.venv/bin/python tools/agents/context_report.py /tmp/lse04selection.json --root /workdir/openWEPP` (exit 0); no revision argument or Git/history access was used. A supplemental exact union calculation supplies category totals and the request ledger below.

The first eight bootstrap files are governing instructions/package handoff; entry/interface/litter are the initial scientific route. Every later mandatory dependency is expansion, not eliminated cost. Source requests that returned truncated text still count their full requested ranges in exposure; those ranges were recovered by smaller overlapping reads. Bootstrap is a phase label rather than a claim to fit the 32–64 KiB aspiration.

| Phase | Unique union bytes within phase | Requested exposure bytes |
|---|---:|---:|
| bootstrap | 89525 | 89525 |
| expansion | 1103307 | 1327366 |
| Combined | 1182184 | 1416891 |

| Category | Unique union bytes | Requested exposure bytes |
|---|---:|---:|
| LSE canonical | 205332 | 215980 |
| External canonical | 903134 | 1127193 |
| Governance/package | 73718 | 73718 |

**Delivered tokens: UNOBSERVED. Workflow-total context: UNOBSERVED. Runtime automatic context: UNOBSERVED beyond the known supplied repository/role instructions.** No token/byte heuristic is used. The automatically supplied root instruction text, parent task, system/developer material, tool schemas, intermediate summaries/compaction and orchestration are not represented as measured source-byte delivery. This report measures this reader's explicit source requests, not other agents or a workflow total.

Truncation recovery: SurfaceLiquid 551–1100 was requested twice after an outer limit, and the second request still had an inner truncation; 700–729 and 730–860 explicitly recovered its missing middle. SurfaceLiquid 1401–1800 had a truncation, recovered by 1578–1589 and 1590–1640. The combined SurfaceLiquid-tail/WATBAL-first request was truncated; WATBAL 1–320 was re-read completely in bounded chunks, followed by the full remaining contract. Full visible SurfaceLiquid tail 2051–2205 remained retained. Later source requests used whole-line chunks below 22,000 source bytes to avoid hidden omissions. The ledger retains every repeat and recovery as paid exposure. No omitted output is asserted read solely because a command requested it.

Navigation-only searches were additional operations, not substitutes for the full reads. They scanned the stated files to locate headings, dependency/mandatory-reading words, `L_s`/sublimation and context-report instructions; heading/matching-line output is not counted as a second full-file read. `wc -l`, `find-agents`, hashing, `/tmp` measurement-log inspection and context-report `--help` were metadata/tool operations. The following scan inventory separately measures one **full-source scan union**, not output delivery or a claim that all scanned lines were exposed. Repeated search passes remain separately described here: initial headings on terminal-support/map-custody; combined headings/reading/Ls search on review/science-obligations/testing/litter; Ls/context search on common/water/litter/interface/prompt; Ls/sublimation/dependency search on SurfaceLiquid/surface-energy/audit/litter.

Navigation scanned-source union: 376493 bytes; it overlaps the source inventory below and is **not** added to unique reading bytes. Search-output bytes/tokens were not captured as exact delivery telemetry.

Tooling source read separately: `tools/agents/context_report.py`, requested lines 1–190 (file has 79 lines), 3544 actual source bytes, full SHA-256 `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde`. This tool-source read is excluded from scientific/governance totals; it explains structural measurement and calls no forbidden history route without `--revision`. Locally authored measurement scripts are not external scientific evidence.

### Full-source identity and union inventory

| Source | Class | Full lines | Full UTF-8 bytes | Read union bytes | Full-source SHA-256 |
|---|---|---:|---:|---:|---|
| `AGENTS.md` | Governance/package | 121 | 9508 | 9508 | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` |
| `docs/specifications/science-contracts/AGENTS.md` | Governance/package | 84 | 6532 | 6532 | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` |
| `docs/work-packages/AGENTS.md` | Governance/package | 105 | 6642 | 6642 | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` |
| `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | Governance/package | 75 | 5304 | 5304 | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | Governance/package | 22 | 1586 | 1586 | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| `docs/work-packages/role-review.md` | Governance/package | 7 | 1400 | 1400 | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` |
| `docs/standards/AGENTS.md` | Governance/package | 58 | 4052 | 4052 | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` |
| `docs/standards/prompt-wording-guidance.md` | Governance/package | 202 | 12057 | 12057 | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | LSE canonical | 65 | 4130 | 4130 | `930c682e5c923083683e8a030457c9e5e90e6ca9cd2f70a425ee0aa79c326c72` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | LSE canonical | 66 | 7976 | 7976 | `0335d3b03acab1bdd711b3be589b429dcd63b7023e516561e269700006fbb2ff` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | LSE canonical | 426 | 30338 | 30338 | `787558481951bc98d78e6c65d6127218e11fcd9fa9cf5c31ccea7f3e649c757c` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | LSE canonical | 303 | 22406 | 22406 | `7c83a0a492c5b6c4c8c8588f9ebd592b83f1f7955c99a812cbcdbad76f5ad037` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | LSE canonical | 227 | 15924 | 15924 | `7d7e0816257f51d2878450c7cbf5cb5d69e9e5fb061c0a49f52bae5a0beef3ce` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | LSE canonical | 208 | 13249 | 13249 | `582c8591001247342a1a0fa919dcd6e3cd267f03ceec04101c87890a6b0ec094` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | LSE canonical | 290 | 25619 | 25619 | `a3169e84ce8e828dc0b51c6fad51d558860f0a51692cfe72975a9c61cc20fc03` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | LSE canonical | 201 | 15484 | 15484 | `3585c1a3713dfe4224fd03f72a88133530d1e14810b34ec5587c05aab2dc5922` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | LSE canonical | 203 | 11069 | 11069 | `3adaa50cb49f8f2895a481df4c2be6d87311b68a262d3adef5c2638c1aca025f` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | LSE canonical | 113 | 8225 | 8225 | `fe37a1a64327aa46ffb1afbc63e00aedab67c83083a3eb3ccbd2edbc17d3174e` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | LSE canonical | 245 | 20940 | 11936 | `9eb3f77d55de9ed404f89bff9c313194780dd816bb43fb087b7e4927ca0e0007` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | LSE canonical | 171 | 19223 | 19223 | `4778df5b572c4a09b3dfedc1f5bba4c84d376bcfcd0021ae7e9fbb6200158b8c` |
| `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | External canonical | 2205 | 184116 | 184116 | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` |
| `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | External canonical | 2648 | 377637 | 377637 | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` |
| `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | External canonical | 3080 | 248829 | 248829 | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` |
| `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | External canonical | 1108 | 92552 | 92552 | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` |
| `docs/work-packages/science-obligations.md` | Governance/package | 98 | 6149 | 6149 | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` |
| `docs/standards/testing-and-gate-strategy.md` | Governance/package | 494 | 25142 | 9329 | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` |
| `docs/specifications/correctness-authority-model.md` | Governance/package | 221 | 11159 | 11159 | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` |
| `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/audit-details.md` | LSE canonical | 155 | 19753 | 19753 | `e3311b94e58cc04dff37cedaa54a742d56b9c4fb9bf4d89c62a904b7d94e624f` |

### Inclusive requested ranges and exposures

Each row is one requested source read and one exposure; repeated ranges are deliberately retained. Resolve full SHA-256 through the identity table above.

| # | Phase | Source | Inclusive lines | UTF-8 requested bytes |
|---:|---|---|---|---:|
| 1 | bootstrap | `AGENTS.md` | 1–121 | 9508 |
| 2 | bootstrap | `docs/specifications/science-contracts/AGENTS.md` | 1–84 | 6532 |
| 3 | bootstrap | `docs/work-packages/AGENTS.md` | 1–105 | 6642 |
| 4 | bootstrap | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | 1–75 | 5304 |
| 5 | bootstrap | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | 1–22 | 1586 |
| 6 | bootstrap | `docs/work-packages/role-review.md` | 1–7 | 1400 |
| 7 | bootstrap | `docs/standards/AGENTS.md` | 1–58 | 4052 |
| 8 | bootstrap | `docs/standards/prompt-wording-guidance.md` | 1–202 | 12057 |
| 9 | bootstrap | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 1–65 | 4130 |
| 10 | bootstrap | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | 1–66 | 7976 |
| 11 | bootstrap | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | 1–426 | 30338 |
| 12 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | 1–303 | 22406 |
| 13 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | 1–227 | 15924 |
| 14 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | 1–208 | 13249 |
| 15 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | 1–290 | 25619 |
| 16 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | 1–201 | 15484 |
| 17 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | 1–203 | 11069 |
| 18 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | 1–113 | 8225 |
| 19 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | 1–18 | 1728 |
| 20 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | 90–175 | 5776 |
| 21 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | 227–239 | 4432 |
| 22 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | 1–171 | 19223 |
| 23 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 1–550 | 30895 |
| 24 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 551–1100 | 64712 |
| 25 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 551–1100 | 64712 |
| 26 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 730–860 | 14670 |
| 27 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 700–729 | 19701 |
| 28 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 1101–1400 | 17427 |
| 29 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 1401–1800 | 40688 |
| 30 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 1590–1640 | 9135 |
| 31 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 1801–2050 | 18287 |
| 32 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 1578–1589 | 2176 |
| 33 | expansion | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 2051–2205 | 12107 |
| 34 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 1–320 | 113665 |
| 35 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 1–157 | 21934 |
| 36 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 158–233 | 21769 |
| 37 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 234–259 | 21474 |
| 38 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 260–281 | 21553 |
| 39 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 282–309 | 21991 |
| 40 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 310–362 | 21823 |
| 41 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 363–443 | 21864 |
| 42 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 444–490 | 21657 |
| 43 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 491–561 | 21820 |
| 44 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 562–719 | 21957 |
| 45 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 720–907 | 21858 |
| 46 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 908–1294 | 21939 |
| 47 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 1295–1664 | 21976 |
| 48 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 1665–2073 | 21994 |
| 49 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 2074–2438 | 21669 |
| 50 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 2439–2519 | 21818 |
| 51 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 2520–2601 | 21746 |
| 52 | expansion | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 2602–2648 | 6795 |
| 53 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 1–253 | 21946 |
| 54 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 254–558 | 21972 |
| 55 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 559–889 | 21954 |
| 56 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 890–1023 | 21793 |
| 57 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 1024–1245 | 21972 |
| 58 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 1246–1473 | 21856 |
| 59 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 1474–1642 | 21955 |
| 60 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 1643–1989 | 21915 |
| 61 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 1990–2334 | 21947 |
| 62 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 2335–2679 | 21967 |
| 63 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 2680–3004 | 21925 |
| 64 | expansion | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 3005–3080 | 7627 |
| 65 | expansion | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 1–377 | 21981 |
| 66 | expansion | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 378–591 | 21768 |
| 67 | expansion | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 592–765 | 21956 |
| 68 | expansion | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 766–1046 | 21994 |
| 69 | expansion | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 1047–1108 | 4853 |
| 70 | expansion | `docs/work-packages/science-obligations.md` | 1–98 | 6149 |
| 71 | expansion | `docs/standards/testing-and-gate-strategy.md` | 193–345 | 8067 |
| 72 | expansion | `docs/standards/testing-and-gate-strategy.md` | 439–473 | 1262 |
| 73 | expansion | `docs/specifications/correctness-authority-model.md` | 1–221 | 11159 |
| 74 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | 1–72 | 4261 |
| 75 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/audit-details.md` | 1–155 | 19753 |
| 76 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | 1–38 | 2257 |
| 77 | expansion | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 1–65 | 4130 |

No production executable/test/authority fixture was changed. The report is the first scientific answer from this reader and has not incorporated a follow-up answer repair.
