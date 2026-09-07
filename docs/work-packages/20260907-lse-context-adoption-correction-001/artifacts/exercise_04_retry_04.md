# Independent frozen-litter closure reconstruction — first completed report

**Static:** This is a source-based, symbolic reconstruction of the frozen task, “Independently reconstruct snow-free frozen-litter surface/soil physical energy and mass closure from accepted primitive receipts.” No primitive receipt bundle, binary, run input, accepted state, or executable-input identity was supplied. No numerical closure, runtime correctness, actual consumer behavior, test pass, or adoption result is established. **Ran:** only source discovery, bounded source reads, SHA-256/UTF-8 measurement, and report construction. No production execution, comparator, solver, or package validation workflow ran.

The assigned frozen source is `current7627c01e2`; I did not query Git or history. Complete actual source hashes and requests appear below. I did not consult the old LSE monolith, other exercise reports, archived candidates, predictions, rubrics, prior reviews, parent results, or other agents' answers. Historical text encountered inside mandatory current external contracts was read as context and was not promoted over current amendments.

## Reading route and scope decision

The v32 entry's frozen-litter route starts at `litter-phase.md#litter-phase`. Its closure dependencies require all of water-vapor, soil-coupling and surface-custody; shared interface requires all common-details. Recursion adds surface-energy, solve-boundary, soil-custody, physical support and complete terminal-support, plus physical receipt-origin handoff/pending/reseal rules in map-custody. I read each of those chapters completely. The interface's full external-contract rule required complete SC-SURFACELIQUID-001, SC-WATBAL-001, SC-VEGETATION-001, SC-SNOWENERGY-001 and SC-COUPLEDTIME-001. This expansion is necessary even for a snow-free target: regime selection precedes evaluation, the snow/soil boundary determines inactivity, and accepted receipt origins include time and complete-owner custody.

I also read audit-details completely to check retained scope, supersession, source anchors, guard-map duties and GAP007/008 rather than infer that relocation resolved them. Qualification was read completely to preserve existing HOLD and distinguish historical retention from prospective EXP-R/PC1/SG1. I make no experimental identity/capture/qualification claim; the protocol/input/kickoff/handoff routes conditional on those claims were not invoked. I do not assert evaluator/reuse equivalence, implement or review V10/V11–13 solver branches, or audit optimization error-order equivalence, so the conditional full nonlinear-solve and dependency-replay routes are not selected. Their absence is not a waiver for a later implementation/equivalence claim. Source quotations identifying literature provenance are contract-reported anchors, not a claim that I independently inspected those PDF/F90 source bytes. Accepted authoritative rain parcels are consumed; their upstream rain-temperature provider is not rederived, so water-vapor's conditional whole SC-SNOWFREEZE provider route is not selected. No actual restart, terminal event, routing lane, or public-output bundle exists to select further downstream implementation protocols.

Root/science/work-package governance, package and worker handoff were read before the exercise. Role-review, science-obligations, prompt-wording, testing-and-gate strategy, correctness-authority model, local gate selection and numerics policy were additional governance reads. The exercise is documentation-only; no Rust or science production change is made. TESTGATE is not invoked. Neither a textual test nor this report discharges runtime A0/A1/A3, real-consumer, independent operand, or closure obligations.

## Exact owner and admission boundary

References below abbreviate `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/` as **LSE/**. Logical IDs retain the `LANDSURFACEENERGY` namespace unless another contract is named.

`LSE/interface.md#interface`, INV001/002/010–015/020–022/030–032/040–043 and `LSE/audit-details.md#purpose/#scope/#gaps` require one explicit horizontal control volume, interval, provenance and area basis for every dimensional primitive. LSE owns surface thermal state; hydrology owns all water; soil thermal owns soil temperature/enthalpy; vegetation owns canopy physiology; climate owns forcing/phase. Conservation bookkeeping confers no second constitutive owner. Preserve one actual water/latent identity, one source/recipient energy pair and immutable owner lineage. A real scheduler consumer must read and act on new state under C004 before runtime closure; producer-only or serialized-ledger consistency does not suffice.

`LSE/litter-phase.md#version-14-snow-free-frozen-forest-litter-successor-amendment` selects immutable `OPENWEPP_SNOW_FREE_LSE_V3`, importing complete unchanged V2/V1 authority and specializing only snow-free `forest_litter`. Hydrology's surface-owner V2 stores liquid `W_l` and liquid-water-equivalent litter ice `W_i`, both kg/m² tile-ground. Litter ice is neither snow nor soil `frozwt`; temperature cannot synthesize it. V3 does not admit bare/ponded frozen surfaces, frozen mineral soil, represented snow, or terminal snow merely because litter freezing is admitted. V2-to-V3 migration copies scientific bits and changes identity only while joining explicit surface-owner V2; V1-to-V2 water migration initializes ice to exact zero, whereas a new explicit V2 seed may contain finite nonnegative ice. Production downgrades are prohibited.

Regime is selected from immutable owner state before iteration: terminal, represented snow, then snow-free base classification are exclusive. `LSE/terminal-support.md#terminal-support`, INV114/115/154 and C009, and current SC-SNOWENERGY's ADR-0044 amendment require the standard native represented-snow map with exact optical/lower-boundary receipts; frozen litter remains inactive. There are zero litter V3/V4 vapor/phase/storage/ingress/WB14 calls, no second inner envelope, and unchanged litter high/carry/markers/receipt bytes. Only an accepted exact terminal split enables a fresh positive snow-free successor. The older snow numerical dispatch descriptions are not alternate production solvers. A legacy schema-v8 censored terminal liquid/energy/time payload is refused (P004), not reconstructed by proxy.

A valid support capability binds parent/segment/slab, absolute half-open integer-nanosecond bounds, exact shared binary64 duration, participants, model/configuration, beginning LSE/soil owners, tolerance/numerical policy and physical minimum. Positive physical LSE support requires 60,000,000,000 ns, not the structural 1 ns quantum; one tick below fails LSEB-E-041 before Newton, preserving every owner. Zero support is an event/custody transition or skip, with no rate integration. No shortened-result scaling, frozen-state result or retry below the floor is permitted; ordinary stable cases must also accept substantially larger steps. The obsolete 0.6-second-dependent evidence is superseded, not replacement evidence. SC-COUPLEDTIME exact decimal u128 ticks, one correct-rounded tick-to-seconds conversion and shared duration bits prohibit independent participant conversions.

Required admission inputs are complete typed model/config/state and source identities, explicit configured OFE topology and areas, tile fractions, canopy mapping/occupancies/ranks, beginning owners, support and predecessor receipts, validated forcing (including positive finite neutral wind and valid logarithmic geometry), atmospheric pressure/humidity/radiation, surface geometry/capacity/conductivity and unfrozen soil layers. Missing/nonfinite/domain-invalid, duplicate/stale/wrong-owner or wrong-basis operands reject before mutation; no calm-wind floor, implicit parameter default, alias, or canonicalize-and-proceed is authorized.

## Primitive physical reconstruction

Use the equation-specific convention below: net radiation is inward; `H_s`, signed vapor energy `Q_v`, and `G_s1` are positive leaving the surface. This is the negative of the generic inward H/G ledger labels in common-details. Signs must be explicitly translated, never silently mixed. Integrate fluxes with the one admitted `dt` before exact-energy decoding.

**Radiation and turbulence** — `LSE/surface-energy.md#surface-energy`, SC-VEGETATION V3 mixed-optics and V8 amendments, INV101–103/VEGETATION080/110–114. Reconstruct the complete VIS/NIR, direct/diffuse two-stream column, ground reflection and upward transfer, then physical leaf/stem/wet recipient absorption. Do not sum direct reflection separately, average nonlinear occupancies, omit clumping or apply it twice, or give stem absorption to leaf PAR. Each canopy occupancy has `tau_i=exp[-0.8*Omega_i*(LAI_i+SAI_i)]`; the unit-emissivity, no-reflection arbitrary-rank longwave recurrence uses current component temperatures. The component contribution is `w_j*(1-tau_i)*(Ldn_i+Lup_(i+1))-2*w_j*(1-tau_i)*sigma*T_j^4`; bottom emission is `sigma*T_s^4`. Zero area contributes zero; empty area transmits with tau one. Bulk canopy temperature, prescribed stale upward ground radiation, and repartitioning bulk net radiation are prohibited.

`H_s=rho_a*c_p*(T_s-T_recipient)/r_h`; signed vapor uses the matching current humidity and vapor resistance. Open tiles use the admitted neutral reference-air log law, kappa 0.4. Covered tiles use the selected ISBA-MEB neutral ground-to-canopy resistance and one common tile `T_c,q_c`; all canopy components and exactly one ground H/v enter the zero-storage node, with one reference-air exchange. The independent node residuals are `sum(H_j)+H_s-H_atm` and `sum(v_j)+v_s-v_atm`. No per-occupancy ground duplicate, reference-air shortcut under canopy, resistance floor or agricultural PMET demand donation is allowed. Accepted primitive receipts must expose the resistance/geometry/air operands and current temperatures/humidities, not only a supplied net residual.

**Litter thermal and vapor state** — `LSE/soil-coupling.md#surface-humidity-surface-enthalpy-litter-and-soil-heat` and `LSE/litter-phase.md#retained-authority-and-adjudicated-constants/#v3-state-phase-free-solve-and-signed-vapor`. `C_dry=dz_l*rho_l,d*c_l,d`; admitted litter conductivity is `0.1+0.03*W_l/(rho_w*dz_l)`. Litter blocks direct mineral evaporation and upward capillary supply. Its sensible coordinate is `U=(C_dry+C_w W_l+C_i W_i)*(T_l-T_ref)`. Selected constants are T_ref=273.15 K, rho_w=1000, rho_i=920 kg/m³, C_w=4218, C_i=2106 J/(kg K), L_f=333700 J/kg, tau_ice=3300 s and `W_i,max=0.85*rho_w*dz_l`. Using rho_i in ice capacity is wrong. The contract anchors this choice to R-156 Appendix A A1–A4/A7–A14 and retained SURFEX generated lines 1992–2159, 388–407 and 146–157 with source hashes in litter-phase; the A4 sign conflict is resolved by conservation and selected executable ordering. Snow's separate 333600 J/kg fusion datum must not replace litter's constant.

Let `p_i=0` for an exactly empty beginning total, otherwise `W_i0/(W_l0+W_i0)`. Let `h_ul=.5*(1-cos(pi*W_l0/W_lmax))`, `h_ui=.5*(1-cos(pi*W_i0/W_imax))`. The raw signed phase fluxes are `(1-p_i)*rho_a*(h_ul*q_sat_liquid(T_l,p)-q_recipient)/r_l-c` and `p_i*rho_a*(h_ui*q_sat_liquid(T_l,p)-q_recipient)/r_l-c`. Both deliberately use saturation over liquid, not ice. Positive liquid evaporation and ice sublimation are authorized separately against only their immutable beginning phase stores; negative condensation/deposition is uncapped supply into its own phase. No cross-phase availability or current-ingress supply is allowed.

Reconstruct separate signed leaving energies `Q_vl=v_l*[C_w*(T_l-T_ref)+L_v(T_l)]` and `Q_vi=v_i*[C_i*(T_l-T_ref)+L_s(T_l)]`. `L_v=2.501e6-2369*(T_l-T_ref)` follows the selected liquid relation. Actual authority-tagged L_s values/definition and evaluation operands are required; this exercise has none and does not invent an L_s relation or replace it with another regime's constant. Keep the two phases distinct through authorization, state construction, sealing and independent checks; only then may the air consumer aggregate them. Latent-only energy, absolute-value aggregation, a total-store cap and a second SC-EVAP debit fail.

**Potential/fixed-final acceptance** — `LSE/solve-boundary.md#solve` INV108–110, water-vapor INV104/105/107, and SC-SURFACELIQUID/SC-VEGETATION fixed-cap authority. The complete potential canopy/surface/soil solve starts from immutable beginnings, publishes root and ground requests together, and gets one hydrology authorization. The complete capped pass rebuilds from those same beginnings and forcing; it cannot continue the potential candidate, reauthorize, or transfer unused caps. Positive finalized use is `F=f_t*q*dt`, with exact `0<=F<=A<=D`; only F debits inventory. Equality is cap-active with zero cap derivative; fully supplied values do not waive fresh complete residual/step acceptance. In V10 nonpositive-assimilation canopy branches, every positive root request must have identity- and amount-equal FullSupply and zero requests retain exact identity; partial positive root authorization is VEG-E-119 unsupported. Ground scarcity is not permission to alter that canopy rule.

For this V3 solve, freeze/melt/fusion, phase-updated capacity and phase-adjusted temperatures are absent from every residual, Jacobian, active branch, authorization and convergence witness. All radiation/H/signed-vapor/G components, surface storage and every soil layer must close; a vapor-only receipt is insufficient. After fixed-final acceptance, `W_l*=W_l0-dt*v_l`, `W_i*=W_i0-dt*v_i`, and `U*=(C_dry+C_w W_l*+C_i W_i*)*(T*-T_ref)`. Independently derive the same U*, T* and masses from beginning states and complete accepted flux primitives, not D, A or a producer zero residual.

**Ground and soil heat** — `LSE/soil-coupling.md#soil-coupling`, INV106 and soil-custody. Use the half-layer resistance `K_s1=2/(dz_s/lambda_s+dz_1/lambda_1)` and `G_s1=K_s1*(T_s-T_1)`, with the admitted Crank–Nicolson endpoint average. Each interior interface similarly has the harmonic two-half-layer conductance. For every ordered soil layer k, `C_k*(T_k1-T_k0)/dt=G_in-G_out`; bottom flux is zero. N=1 has only its top exchange. Integrating the common top transfer gives exactly one surface debit and opposite soil credit. Soilthermal owns every T_k; no duplicate LSE soil temperature, frost-front solver, hidden bottom boundary or fusion injection is admitted. Generic equilibrium-zero surfaces have C_dry=W=U=0 and algebraic endpoint temperatures; a numerical warm start cannot become physical beginning storage. Frozen litter instead requires its finite positive ending phase heat capacity.

The phase-free surface relation, in the explicit leaving convention, is `U*-U0=dt*(R_sw+R_lw-H_s-Q_vl-Q_vi-G_s1)` using the accepted temporal quadrature for each physical operand. Soil interior G terms cancel only after independently joining both sides. Do not use a global residual to manufacture either side of an interface.

## Phase, capacity spill and current ingress

`LSE/litter-phase.md#bounded-kinetic-phase-and-fusion-energy-closure` requires finite positive dz and ending capacity, `W_l*>=0` and `0<=W_i*<=W_imax`. From accepted post-vapor/pre-ingress state:

```
M_warm = rho_i*C_i*dz_l*max(T*-T_ref,0)/L_f
M_cold = rho_i*C_i*dz_l*max(T_ref-T*,0)/L_f
m_melt = min(W_i*, (dt/tau_ice)*min(M_warm,W_i*))
m_freeze = min(W_l*, W_imax-W_i*, (dt/tau_ice)*min(M_cold,W_l*))
m_phase = m_freeze-m_melt
W_l,phase = W_l*-m_freeze+m_melt
W_i,phase = W_i*+m_freeze-m_melt
U_phase = U*+L_f*m_phase
T_phase = T_ref+U_phase/(C_dry+C_w*W_l,phase+C_i*W_i,phase)
```

The outer mass/capacity limits remain mandatory for dt>tau_ice and create no maximum-step requirement. At exact T_ref both phase transfers are zero. Independently check liquid debit=ice credit and the reverse, unchanged total water, and phase-only `H=U-L_f*W_i` invariance. Do not evaluate T with pre-phase capacity or apply literal `T+=L_f*m_phase/C*`; both leave an unowned capacity-change energy term. The phase ending is the next-support warm start; there is no same-support flux/Newton/fixed-point/authorization resolve.

**Spill** — INV156/C011, `LSE/litter-phase.md` exact capacity-spill amendment and SC-SURFACELIQUID matching amendment. After phase, before ingress, use raw phase W/U/T. If `W_l,raw<=W_lmax` (including exact equality), preserve bits and emit zero spill. Otherwise perform the stated checked binary64 order: `m_spill=W_l,raw-W_lmax`; `h_spill=C_w*(T_raw-T_ref)`; `Q_spill=m_spill*h_spill`; `W_l,ret=W_l,raw-m_spill`; `U_ret=U_raw-Q_spill`; derive retained capacity and T from retained U and masses. The second subtraction, not the capacity constant or a clamp, is authoritative retained water. Ice is unchanged. Independently reconstruct raw=retained+spill mass and sensible energy with this operation order.

`LitterPhaseCapacitySpillV1` binds model/config, owner/transaction, exact full child support/key, phase source, raw/capacity/retained state and m/h/Q. It generates exactly one negative exact-surface `LitterPhaseCapacitySpillEnergy` operand after phase-free and fusion operands; applying f once gives one internal `LitterPhaseOverflow` ingress parcel on the full child `[0,dt)`. It is not caller rain, condensation or a rainfall-hour alias. WB14 consumes it once. A later retained return is a separately named positive credit, never cancellation of the spill debit. No ice runoff, latent-fusion runoff, discarded overflow, min/clamp/snap, or duplicate ingress is allowed.

**Heterogeneous finalized resources** — INV157/C012. Match each native vapor row by exact phase receipt, transaction/child/key, area and finalized F (not A). Exclude exactly those already consumed native rows. Apply every unmatched ordinary finalized row in canonical GroundWaterKey order, one checked F/f conversion, to the same phase-adjusted V2 owner; all rows are partitioned exclusively and completely among soil/native/ordinary responsibilities. Zero ordinary rows preserve identity and produce no invented parcel/energy. No wholesale legacy owner replacement, native row replay, omitted/foreign row or dropping phase ice/high-carry receipts. Current ingress follows once after this joined owner.

**Ingress and WB14** — water-vapor#vapor/#water, P001–004/C001–002; SC-WATBAL WB14 and SC-SURFACELIQUID algorithms. Only after accepted vapor, phase and spill may liquid rain, runon, same-tile throughfall, first/second canopy drainage, stemflow and overflow enter. Litter ice cannot infiltrate, run off, drain, satisfy WB14, enter soil frozwt or mutate soil. No ingress can retroactively supply the accepted vapor or phase operation. Daily WB17 SC-EVAP remains separate and cannot repeat this subdaily litter debit.

Each positive liquid parcel carries `h=C_w*(T-T_ref)`, `Q=m*h` with exact source/recipient/support/reference lineage. Mix by `T_mix=T_ref+sum(m*h)/(C_w*sum m)` only for positive mass; zero crossing has neither temperature nor energy. Covered canopy releases use accepted wet-surface T and separate release masses/lineage; stemflow bypasses lower canopy to same-tile ground, while throughfall/drainage traverse lower ranks. Open raw rainfall and covered final releases are exclusive ground inputs. Accepted rain carries the retained hydrometeor-provider output plus273.15; accepted runon carries actual outlet T, never ambient or soil fallback.

INV130's only temperature normalization is at the named covered-canopy ledger/release or Stage3 terminal-liquid publication boundary: exact bits 0x4071126666666667 (first upward neighbor of273.15, difference2^-44 K) become0x4071126666666666 before h is formed. Exact reference stays unchanged; below-reference canopy liquid retains its typed rejection; second upward neighbor and beyond are not snapped. This does not normalize frozen-litter T, solver residuals, storage or arbitrary parcels.

WB14 alone executes nonlinear infiltration continuation; no copied formula, per-parcel proxy, whole-day replay or alternative partition. For each chronological source window, independently reconstruct offered X and sensible Q; I+E=X, mixture h=Q/X ifpositive otherwise0, `Q_inf=I*h`, `Q_excess=Q-Q_inf`. Source attribution retains canonical order and its explicitly admitted final floating remainder; that exception does not authorize water-authorization last-row repair. Surface excess retention uses available f(Wmax-W) and named capacity rule; only a positive retained mass <=`1e-14+64eps*(|fWmax|+|fW|+|E|)` may become exact zero retention with unchanged store bits, sending the complete mass and enthalpy onward. Larger retention follows normal checked arithmetic. Retained Q is grouped in canonical receipt order on OFE basis then checked binary64-divided by f exactly once. Routing source to destination uses Au/Ad once for both m and Q. Without actual routing regime receipts, do not infer LaneD or DC01, reinterpret runon as an external hillslope source, or claim full WAT/PASS/HBP publication closure.

## Exact energy custody and independently testable closure

`LSE/soil-custody.md#exact-soil`, INV150/P005/C005, and `LSE/surface-custody.md#exact-surface`, INV151: decode each already accepted finite binary64 integrated physical energy only after its physical units/area conversion. For a soil layer, exact total is `E=exact(H_hi)+R`; for a surface tile, `U=exact(U_hi)+R_U`. Accumulate beginning high+carry and each named accepted physical Q with arbitrary-precision dyadic integer arithmetic, round once to nearest-even finite high, then retain the exact difference as canonical dyadic carry. Never form an energy from a residual, regrouped aggregate or carry.

ExactDyadic has sign -1/0/1, odd positive lowercase hexadecimal coefficient without leading zeros and integer exponent. The sole zero is `(0,"0",0)`. Equivalent but noncanonical even/uppercase/embedded-sign/negative-zero/wrong-zero-exponent forms reject. Frozen binary64 high signed-zero policy is unchanged: migration/no-op copies high bits; dyadic zero does not authorize canonicalizing that high. Migration soil V1→V2 copies all old bits with zero carry and no thermal/physical change; production downgrade is forbidden even if carry is zero. Surface exact ownership requires every matching LSE V3/surface-owner V2 high mirror equal bitwise, exact key set and explicit companion; projectionV4 composes requiredV3 data, not a missing-companion or downgrade alias.

The surface exact operand list is complete phase-free shortwave, longwave, sensible, phase-specific vapor and ground conduction; accepted binary64 fusion `L_f*(freeze-melt)`; the later named negative spill; and canonical retained-ingress group credits. Retained OFE Q is folded in its canonical receipt order and divided by f in checked binary64 before decoding: exact rational division is not the admitted physical conversion. Temperature derives from the physical high and ending capacity; carry is custody, never feedback to T, flux, humidity, phase or Newton residual.

With source/recipient joins proved independently, reconstruct these separate predicates:

1. Phase-free water: `W_l* - W_l0 + dt*v_l=0` and `W_i* - W_i0 + dt*v_i=0`, preserving signed condensation/deposition and each real finalized row.
2. Phase-only total water and H: `W_l,phase+W_i,phase=W_l*+W_i*`; `U_phase-L_f W_i,phase=U*-L_f W_i*` under selected accepted arithmetic/evidence. Fusion is internal, not external energy.
3. Raw/retained/spill water and sensible energy with the exact prescribed subtraction order; native and ordinary finalized-row coverage separately; complete ingress offered=inf+retained+forwarded with all begin/end stores.
4. Each surface exact high+carry ending equals exact beginning plus every named accepted Q once; each soil exact high+carry ending equals exact beginning plus its accepted conduction/top/infiltration credits once. Require matching opposite interface receipt bodies independently from both owners.
5. The combined litter+soil total uses physical `H_litter=U-L_f W_i` and soil exact enthalpy; interior conduction and internal parcel transfers cancel by joined identities, leaving only explicitly identified atmospheric and external advective terms. Do not call fusion an external source or count runoff at both an internal route and the hillslope outlet. Material/vegetation controls remain their distinct owners; a surface/soil subtotal cannot establish whole-system C/N or watershed-output closure.

These are algorithms for independent checks, not fabricated numeric zeros. Actual ending highs/carries, signed energies, parcels, temperatures, masses, caps, and tolerances are absent. Even a small source-reported fixture cannot supply them: the WAT5 below-ULP example is a distinct source oracle, not this run; p61's exact failing high/credit operands require unchanged-fixture capture and are not manufactured here. C005/P005 and the surface consumer obligations require real WAT5/p61/native-forest evidence, split-restart equivalence and exact reconstruction before promotion.

SC-SURFACELIQUID mass closure uses `1e-14 kg/m² +64eps*sum(abs operands)` and liquid energy closure `1e-9 J/m²+64eps*sum(abs operands)`; identity, cardinality, capacity and F<=A remain exact. Its proportional authorization has its own precisely bounded representational exception: only overshoot within `1e-14+64eps*(abs rawsum+abs supply)`, with one common maximal safe positive binary64 factor found by bounded bit search, preserving positive rows and both OFE and inverse-tile supply checks. No independent last-row correction is permitted there. Exact high/carry custody has **no tolerance**. Solver residual/step, phase-event, adaptive truncation and physical ledger thresholds are distinct; neither generic historical unassigned TOL001/002 nor a snow outer-map tolerance can replace the selected current check. The physical solve requires all component residuals and governed steps, not merely an aggregate zero, and supports typed numerical failure with no last-iterate publication. The selected boundary uses energy residual threshold `1e-6 W/m²+1e-10*max(1,sum(abs component operands))`, water-flux threshold `1e-12+1e-9*owning water scale`, and governed temperature/humidity/hydraulic/beta step bounds `1e-8 K`, `1e-12 kg/kg`, `1e-7 mm H2O`, and `1e-10`. Require the complete current residual plus an admitted prospective-step witness even for a no-update acceptance; a mere reused coordinate vector cannot accept iteration zero. The boundary retains deterministic lowest-index pivot ties, typed singularity rather than inverse/default substitution, bounded Newton/backtracking and strict descent for updates. Detailed branch/stencil implementation equivalence would require the conditional full nonlinear-solve reading and execution, which this accepted-primitive exercise does not claim.

## Receipt origin, failure chronology and complete-owner evidence

The phase receipt `OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1` must contain model/config/state/phase tags/digests, exact support/duration, owner/OFE/tile/area/transaction, beginning W_l/W_i/U/T, both raw/final signed vapor and energies, post-vapor U*/T*/masses, all constants/capacities/phase bounds/transfers, ending phase state, ingress identities/WB14 handoff, beginning/candidate owners, and independent mass/energy/H reconstructions. Producer residuals are not hashable proof. P001 requires immutable source-identified primitives and sealed start/end ledger; P002 validation before atomic energy/water mutation; P003 explicit branch/interval/tolerances/residual/component lineage; P004 continued refusal of censored schema-v8 data.

INV153/C008: child ending before parent end is Partial and preserves all three predecessor transaction markers while advancing exact energy receipt chain; child ending at parent end is Final and stamps the child transaction once. Parent start<=child start<child end<=parent end. Wrong posture, mixed markers, early advance or retained predecessor at final reject; historical receipt bytes are not aliases. INV158/C013: configured ofe_topology rank then canonical within-OFE key order controls exact owner/operand records. Every operand binds owner/rank/kind/ordinal; OFE IDs are opaque, not lexically/numerically ordered. Unique digests alone do not prove full configured membership/order.

INV155/C010: an unpublished soil continuation may be read only from the original authenticated prepared resident, exact predecessor unpublished state/seal and positive contiguous child support. Its prepared transaction is not rebound to an outer ID. The borrowed continuation is a non-owner and has **absent**, not empty, owner/restart/checkpoint/accepted-receipt/seal byte surfaces. Promotion consumes the original prepared owner plus all accumulated canonical operations, replays the selected exact ending credit chain once and installs/seals once; no dual intermediate/final owner.

INV159's full handoff definition: private immutable live-revision proof binds schema/model/configuration/topology/digest, transaction/predecessor/support and exact history prefix count/head/tail/chain. No mutable/unchecked constructor, digest-only proof or wire-restored proof. Trusted append validates tail predecessor; mutation consumes proof and requires full new validation. External/restart/durable/untrusted input receives full canonical validation. V30's extension changes only original validation positions: parent-generation static topology plan, pointer-identical per-map forcing validation at its original position, and native proof sourced from the already validated resident revision and consumed after V8+ingress in native validation. V8 cannot attest a distinct native resident; all forcing/support/duration/joint/residual/output and other map-dynamic surfaces stay fresh. This report checks receipt-origin requirements, not implementation optimization equivalence or C019 discharge.

INV160/C015 reseal requires a fully validated provisional physical ending and private move-only live proof binding parent/segment/slab/support/config/topology/forcing/beginning owners/support receipt and all non-slab physical data. A final independent slab may change only ending-derived slab identity; it consumes the same physical ending byte-for-byte without physics replay. There are zero provisional publications and one final publication; no restart proof, fallback replay or non-slab substitution.

INV161/C016 pending: ordinary/native role and ordinal are chosen before physical evaluation. Each ordinary physical prefix executes its surface/WB14 work once; represented snow executes no litter work. After physical+exact-custody completion, pending evaluation precedes V8/vegetation/BGC/joint/envelope constructors. Outer nonclosure retains history without manufacturing a constructor error; outer closure with only dependent nonclosure returns AdaptiveRefinement and no accepted history; complete closure consumes that SAME pending prefix into FinalAccepted private envelope construction with no install/publication or physics replay. Existing typed physical failures keep precedence. Coupled time owns final atomic publication (CT-E-018); finalization occurs only after exact parent coverage, active owner/event/scheduled receipt joins, all material/resource ledgers and accepted-only reductions pass. Slab advancement does not bump persistent parent sequence; parent finalization does so once. Rejection preserves all beginning owners, receipts, checkpoints, publication/outbox and accepted controller history; diagnostic attempts are not accepted physical IDs.

Physical solve error precedence is malformed input → identity → topology/owner → nonfinite → unsupported regime → constitutive domain → demand/authorization/final-use → singular Jacobian → exhausted backtracking → iteration limit → accepted residual/step → component closure → control-volume closure → owner validation. V3 families are LSEB-E-045 identity/domain/migration, -046 signed-phase vapor custody/enthalpy, -047 phase mass/fusion/capacity (and spill), -048 chronology/receipt/restart/rollback, -049 exact soil, -050 exact surface/mirror/custody. A later WB14, owner or publication failure still rolls back LSE, water, soilthermal, hydrology, vegetation, BGC and enclosing transaction byte-for-byte. I have not exercised any failure seam.

For restart evidence, SC-COUPLEDTIME and vegetation V11 require complete accepted prefix and typed owner states, not opaque digest aliases. V11 restartV3 composes unchanged requiredV2 semantic admission and adds resource-owner custody; V1 and V2 alone are nonproduction for that continuation. Every occupancy debit preserves D/A/F and separate shared owner transitions; occupancy post-use never becomes shared hydrology inventory. Canonical accepted-child order and full seven-owner continuation must match independent uninterrupted parent-beginning+prefix+suffix execution. The audit-only one-ULP terminal BGC regrouping check cannot change the sequential BGC ending or relax water/surface/soil exact custody. The global BGC inventory is not silently split over OFEs: nonempty mineral-N protocols require explicit unique one-bearing-OFE mapping. This is a conditional whole-owner requirement, not evidence that a BGC or restart transaction was supplied.

## Required input and test evidence; first-report self-check

To perform actual reconstruction, supply independent beginning and accepted ending owner surfaces plus the complete source-identified primitive receipt set: model/config/source hashes; topology/area maps; support, parent/child/regime and predecessor chains; radiation directional/component operands; atmospheric/canopy/ground temperatures/humidities/resistances; soil geometry/capacities/conductivities and both endpoint interface fluxes; separate D/A/F and raw/final phase vapor with authority-tagged vapor enthalpies; complete phase/spill/ordinary-resource receipts; every external/internal ingress, retained/infiltration/runoff energy and area conversion; exact highs/carries/mirrors; pending/reseal/owner promotion and accepted publication origins. Nothing in this report fills an absent numerical member with zero or a source fixture.

The self-check of selected rule families is:

| Selected anchors | Preconditions/exclusions checked in this report | Concrete required independent evidence/tests, not run |
|---|---|---|
| interface INV001/002/010–015/020–022/030–032/040–043; P001–004/C001–004 | units/basis/lineage; correct mutually exclusive regime; one owner, validation-before-mutation; no proxy production/calibration claim | all-distinct begin/input/output/ending operands; omit/duplicate/swapped radiation, latent, sensible, G, advection, storage and water; missing/stale receipt; real scheduler consumer reads/acts; nonfinite and domain poison with rollback |
| surface-energy and soil-coupling; INV101–107; vegetation080/110–114 | current-component reciprocal column and one tile air node; unfrozen soil; actual half-layer/CN boundary | mixed two-rank unequal tiles, VIS/NIR/direct/diffuse and leaf/stem/wet poisons, zero area/light, open/covered day/night, ground-feedback and sign reversal, N=1/multilayer, finite/equilibrium-zero storage, no duplicated G/T owner |
| solve-boundary INV108–110; phase-free V3; vegetation118/119 | immutable original beginnings; one authorization; all residual/step witnesses; V10 nonpositive leaf FullSupply restriction | potential/final distinct values, partial/full surface/root scarcity, exact cap equality and zero derivative, unused A preserved, no ingress donation, alternative warm starts, singular/backtracking/iteration refusal, no potential-candidate continuation, all-owner byte equality |
| litter-phase signed vapor, bounded phase and phase receipt | separate beginning-phase caps; liquid saturation for both; correct sensible+latent; ending capacity; no same-support resolve | liquid-only/ice-only/mixed/empty; evaporation/condensation/sublimation/deposition; negative-sign and wrong-pool poisons; warm/cold/exact-reference; dt below/at/above tau; ice-cap limit; exact H and mass, wrong Lf/sign/rho capacity/prephase capacity poisons |
| INV156/C011 and INV157/C012 | postphase/preingress spill; checked operation order; one native/ordinary resource partition | below/at/above capacity and melt-created spill, full-child support, unequal f, zero/nonzero ordinary rows, duplicate/omitted/foreign/native replay, wrong area or transaction, explicit negative spill plus separate retained return, one WB14 call, no flux resolve |
| water-vapor INV130/104/105/107; WB14/SurfaceLiquid | exact first-upward-neighbor boundary only; no arbitrary snapping; canonical parcel chronology and sole hydrology partition | below/reference/first/second neighbor, zero/positive parcel mass, distinct release temperatures and runon, two-rank first+second drainage/stemflow, rain/root/ground competition, source-time windows, internal Au/Ad once, bounded retention threshold just inside/outside, all external/internal conservation |
| soil INV150/P005/C005; surface INV151 and exact dyadic definition | finite accepted physical f64 then exact decode; round once; no carry feedback; frozen high mirrors and typed companion | positive/negative/zero/subnormal and cancellation credits, below-ULP credit, tie-even, noncanonical dyadic poisons, signed high zero preserved, wrong mirror/key/version, full exact reconstruction, WAT5/p61/native-forest real consumers and missing actual capture fields |
| INV153/C008; INV158/C013 | partial vs final marker chronology; authenticated configured topology not ID sorting | first/middle/final child, premature/late stamp, gap/overlap, midparent restore, ofe-9→ofe-10 and reverse/nonnumeric IDs, duplicate/omitted/reordered operands, semantic order/amount substitution |
| INV155/C010; INV159 marked handoff; INV160/C015; INV161/C016 | non-owner byte absence; original prepared authority; immutable revision proof; original validation positions; same pending prefix consumed; no replay/publication | wrong predecessor/prepared transaction/support; owner-byte presence; exact final replay once/install once; cross-revision/config/pointer/proof poison; fresh untrusted/restart validation; same physical end bytes and provider/phase/ingress/WB14/soil call counters; pending ordinary/native/failure/refinement outcomes and CT-owned publication |
| terminal-support INV114/115/154/C009 and physical support definition | represented snow inactive litter; post-event fresh receiver only; zero remainder no solve; >=60s physical support | native optical/lower-boundary receipt retention, zero litter calls/no second envelope, exact terminal handoff and fresh successor forcing, stale snow/duplicate liquid/fusion-as-G/fullinterval poisons; floor and floor-minus1 ns pre-Newton rejection; stable larger step; rollback/restore |
| common-details tolerances/tests and audit gaps; qualification retention | exact custody vs physical ledger vs solver/adaptive tolerances separate; no gap or HOLD discharged by format/readability | truthful source/operand/test/run class; no producer-selected expected residual; independent calculator/frozen fixture bytes; required authority gates and real consumers before promotion; no identity-only science/performance claim |

I checked the applicable marked mechanism definitions against the report before completion: phase-free input/exclusion and signed vapor; bounded phase and postphase ending capacity; spill and heterogeneous join; one-ULP liquid publication boundary; exact dyadic/high-mirror and physical-before-exact conversion; parent partial/final and configured topology; unpublished non-owner; handoff/reseal/pending chronology; complete support and represented-snow exclusion. All have concrete preconditions, prohibited alternatives and required evidence above. Conditional optimizer/solver/executable experiment definitions were read only where present in mandatory scope or qualification context and are not claimed satisfied. Numerical physical closure and real consumer acceptance remain unestablished because actual operands and execution are absent; no further reading can turn those missing values into evidence.

## Provenance and measurement ledger

Measurement convention: a source request is each actual selected source-body read. SHA-256 covers complete source file bytes; UTF-8 requested bytes count exactly the selected existing source lines (including their original newlines), excluding numbered display prefixes and FILE/NEXT framing. A requested range past EOF is recorded as requested, with actual extent separately. Reading complete bytes internally to hash does not mean their unrequested body was delivered to the agent. Unique source union counts the union of requested existing lines by path; repeated requested exposure is total requested bytes minus that union, including repeated reads for route self-checks and recovery. Requests whose output was truncated still count their complete requested source bytes. This is requested source exposure, not delivered semantic/token telemetry.

Bootstrap = first root/science/workpackage/package/handoff requests. Every later read is expansion; voluntary rereads are included. Source metadata discovery (pwd, applicable-instruction finder and source wc inventory) exposed paths/counts only, no additional source-body ranges; /tmp helper/ledger reads and this report's own writes are measurement work, not canonical source exposure. Delivered-token counts, workflow/tokenizer totals and total workflow context are **UNOBSERVED**; tool-reported approximate token counts are not silently treated as exact telemetry.

Truncation/recovery record: initial multi-file interface/litter/science-obligation/prompt/testing request exceeded the outer output budget; litter's tail and testing's beginning plus affected governance reads were recovered in the subsequent recorded bounded ranges. A litter recovery batch again lost its final lines;404–426 was recovered. SurfaceLiquid541–840 lost the703–718 middle and that interval was reread. WATBAL1–280 lost its middle and125–200,201–259,260–280 were recovered. Subsequent22KB whole-line chunks avoided truncation until the qualification+audit batch outer limit clipped qualification's tail and audit's beginning; qualification230–280 (actual230–246) and audit1–16 recover that overlap. The request ledger below identifies all repetitions; no truncated normative text was intentionally left unrecovered.


| Exposure category | Unique requested UTF-8 bytes | Total requested UTF-8 bytes | Repeated requested bytes |
|---|---:|---:|---:|
| LSE | 235188 | 329692 | 94504 |
| external/governance | 1499545 | 1609557 | 110012 |
| Total | 1734733 | 1939249 | 204516 |

The unique requested union is the complete file for every source in the following table; no additional, unlisted normative source-body request was made. All hashes were rechecked unchanged during report assembly.

| Source ID | Full source path | Unique inclusive lines | Complete-source SHA-256 |
|---|---|---|---|
| S01 | `AGENTS.md` | 1–121 (full) | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` |
| S02 | `docs/specifications/science-contracts/AGENTS.md` | 1–84 (full) | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` |
| S03 | `docs/work-packages/AGENTS.md` | 1–105 (full) | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` |
| S04 | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | 1–75 (full) | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| S05 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | 1–22 (full) | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| S06 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 1–65 (full) | `930c682e5c923083683e8a030457c9e5e90e6ca9cd2f70a425ee0aa79c326c72` |
| S07 | `docs/work-packages/role-review.md` | 1–7 (full) | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` |
| S08 | `docs/standards/AGENTS.md` | 1–58 (full) | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` |
| S09 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | 1–66 (full) | `0335d3b03acab1bdd711b3be589b429dcd63b7023e516561e269700006fbb2ff` |
| S10 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | 1–426 (full) | `29ba7de4a93aa48d770b229fb1ea7fbbc581c2f49c2c61bf5c5697fb8f5f3042` |
| S11 | `docs/work-packages/science-obligations.md` | 1–98 (full) | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` |
| S12 | `docs/standards/prompt-wording-guidance.md` | 1–202 (full) | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` |
| S13 | `docs/standards/testing-and-gate-strategy.md` | 1–494 (full) | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` |
| S14 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | 1–301 (full) | `6b60c3d5ca01cf798ad3f98c6ee0d03c181019a24d6fde300c9f66cbac78142d` |
| S15 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | 1–228 (full) | `09bfcda49d5181dd8022d798f0d21caadcbb9da236650e902019dc885a8ecd62` |
| S16 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | 1–208 (full) | `58465434d842511c72a5e27653bfa13ee51015f13939f3ad33f2fe8200f6287d` |
| S17 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | 1–290 (full) | `a1327441160fad148f847c378dec2e9251bc34e81254a973361a61c5f5450190` |
| S18 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | 1–201 (full) | `342b7601176eb263ed3de199c6d9697d987e1650220a615a49c34181e9b27bbd` |
| S19 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | 1–245 (full) | `e20bd57efa318ab3f82ca31dfd9712732b9d827d3cd38839295a3b570317a9f7` |
| S20 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | 1–171 (full) | `4778df5b572c4a09b3dfedc1f5bba4c84d376bcfcd0021ae7e9fbb6200158b8c` |
| S21 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | 1–206 (full) | `055966f4d3373dba5dcb6a99a3d3c56c02bf948fa0182d8bc74bb20c26490361` |
| S22 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | 1–113 (full) | `fe37a1a64327aa46ffb1afbc63e00aedab67c83083a3eb3ccbd2edbc17d3174e` |
| S23 | `docs/specifications/correctness-authority-model.md` | 1–221 (full) | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` |
| S24 | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 1–2205 (full) | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` |
| S25 | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 1–2648 (full) | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` |
| S26 | `docs/standards/local-ci-gate-selection.md` | 1–129 (full) | `9b2cd01f06d433f99b9dc9ae3d596c4a348c23b9260ef4ecbe197efa84f60e8e` |
| S27 | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 1–1108 (full) | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` |
| S28 | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | 1–3559 (full) | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` |
| S29 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 1–3080 (full) | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` |
| S30 | `docs/numerics/README.md` | 1–45 (full) | `b87322b3b30defa57bf7365a8cc16f0719ee2e9aa14b8bc12cdcfb39455e5a8b` |
| S31 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/qualification.md` | 1–246 (full) | `be7b5b41d11e39e5adb3fb7af18233d9e567938936345194c7b92eca76472ebe` |
| S32 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/audit-details.md` | 1–159 (full) | `32ee0cd90d9df038b5596a47e699b91a06c16af8aee0e980622fe9f7053a084d` |

Every request below inherits the complete path and complete-source hash from its Source ID above. Repeated bytes are measured against earlier requests, so a recovery request counts as repeated even when the original output was not fully delivered. Ranges are inclusive. The truncation/recovery narrative above is delivery metadata, separate from the requested-byte accounting.

| Request | Source | Requested range | UTF-8 bytes | Bootstrap/expansion | Repeated requested bytes |
|---|---|---|---:|---|---:|
| R001 | S01 | 1–121 | 9508 | bootstrap | 0 |
| R002 | S02 | 1–84 | 6532 | bootstrap | 0 |
| R003 | S03 | 1–105 | 6642 | bootstrap | 0 |
| R004 | S04 | 1–75 | 5304 | bootstrap | 0 |
| R005 | S05 | 1–22 | 1586 | bootstrap | 0 |
| R006 | S06 | 1–65 | 4130 | expansion | 0 |
| R007 | S07 | 1–7 | 1400 | expansion | 0 |
| R008 | S08 | 1–58 | 4052 | expansion | 0 |
| R009 | S09 | 1–66 | 7976 | expansion | 0 |
| R010 | S10 | 1–426 | 30323 | expansion | 0 |
| R011 | S11 | 1–98 | 6149 | expansion | 0 |
| R012 | S12 | 1–202 | 12057 | expansion | 0 |
| R013 | S13 | 1–494 | 25142 | expansion | 0 |
| R014 | S10 | 174–426 | 21799 | expansion | 21799 |
| R015 | S11 | 1–98 | 6149 | expansion | 6149 |
| R016 | S12 | 1–202 | 12057 | expansion | 12057 |
| R017 | S10 | 404–426 | 7070 | expansion | 7070 |
| R018 | S14 | 1–200 | 16853 | expansion | 0 |
| R019 | S14 | 201–365 (actual to 301) | 6285 | expansion | 0 |
| R020 | S15 | 1–190 | 10516 | expansion | 0 |
| R021 | S15 | 185–400 (actual to 228) | 5856 | expansion | 199 |
| R022 | S16 | 1–200 | 11890 | expansion | 0 |
| R023 | S16 | 201–230 (actual to 208) | 1344 | expansion | 0 |
| R024 | S17 | 1–290 | 25574 | expansion | 0 |
| R025 | S18 | 1–201 | 15424 | expansion | 0 |
| R026 | S19 | 1–30 | 2452 | expansion | 0 |
| R027 | S20 | 1–30 | 2131 | expansion | 0 |
| R028 | S19 | 31–240 | 17595 | expansion | 0 |
| R029 | S20 | 31–200 (actual to 171) | 17092 | expansion | 0 |
| R030 | S21 | 1–210 (actual to 206) | 11649 | expansion | 0 |
| R031 | S22 | 1–113 | 8225 | expansion | 0 |
| R032 | S13 | 1–192 | 11066 | expansion | 11066 |
| R033 | S23 | 1–180 | 9607 | expansion | 0 |
| R034 | S24 | 1–260 | 16631 | expansion | 0 |
| R035 | S24 | 261–540 | 13938 | expansion | 0 |
| R036 | S24 | 541–840 | 46774 | expansion | 0 |
| R037 | S24 | 703–718 | 12805 | expansion | 12805 |
| R038 | S24 | 841–1040 | 15051 | expansion | 0 |
| R039 | S24 | 1041–1340 | 16716 | expansion | 0 |
| R040 | S24 | 1341–1650 | 34334 | expansion | 0 |
| R041 | S24 | 1651–1920 | 18630 | expansion | 0 |
| R042 | S24 | 1921–2205 | 22042 | expansion | 0 |
| R043 | S25 | 1–280 | 85328 | expansion | 0 |
| R044 | S25 | 125–200 | 9027 | expansion | 9027 |
| R045 | S25 | 201–259 | 38757 | expansion | 38757 |
| R046 | S25 | 281–297 | 20676 | expansion | 0 |
| R047 | S25 | 298–355 | 21731 | expansion | 0 |
| R048 | S25 | 356–435 | 21701 | expansion | 0 |
| R049 | S25 | 436–484 | 21672 | expansion | 0 |
| R050 | S25 | 485–553 | 21827 | expansion | 0 |
| R051 | S25 | 554–674 | 21981 | expansion | 0 |
| R052 | S25 | 675–872 | 21853 | expansion | 0 |
| R053 | S25 | 873–1245 | 21900 | expansion | 0 |
| R054 | S25 | 1246–1606 | 21936 | expansion | 0 |
| R055 | S25 | 1607–2022 | 21947 | expansion | 0 |
| R056 | S25 | 2023–2397 | 21993 | expansion | 0 |
| R057 | S25 | 2398–2509 | 21949 | expansion | 0 |
| R058 | S25 | 2510–2592 | 21922 | expansion | 0 |
| R059 | S25 | 2593–2648 | 9221 | expansion | 0 |
| R060 | S25 | 260–280 | 20151 | expansion | 20151 |
| R061 | S23 | 181–221 | 1552 | expansion | 0 |
| R062 | S26 | 1–129 | 6874 | expansion | 0 |
| R063 | S27 | 1–377 | 21981 | expansion | 0 |
| R064 | S27 | 378–591 | 21768 | expansion | 0 |
| R065 | S27 | 592–765 | 21956 | expansion | 0 |
| R066 | S27 | 766–1046 | 21994 | expansion | 0 |
| R067 | S27 | 1047–1108 | 4853 | expansion | 0 |
| R068 | S28 | 1–339 | 21940 | expansion | 0 |
| R069 | S28 | 340–589 | 21592 | expansion | 0 |
| R070 | S28 | 590–613 | 21730 | expansion | 0 |
| R071 | S28 | 614–720 | 21905 | expansion | 0 |
| R072 | S28 | 721–1185 | 21971 | expansion | 0 |
| R073 | S28 | 1186–1309 | 21838 | expansion | 0 |
| R074 | S28 | 1310–1348 | 21997 | expansion | 0 |
| R075 | S28 | 1349–1412 | 21995 | expansion | 0 |
| R076 | S28 | 1413–1426 | 20615 | expansion | 0 |
| R077 | S28 | 1427–1435 | 20616 | expansion | 0 |
| R078 | S28 | 1436–1445 | 21876 | expansion | 0 |
| R079 | S28 | 1446–1547 | 21810 | expansion | 0 |
| R080 | S28 | 1548–1603 | 21921 | expansion | 0 |
| R081 | S28 | 1604–1848 | 21949 | expansion | 0 |
| R082 | S28 | 1849–2031 | 21746 | expansion | 0 |
| R083 | S28 | 2032–2101 | 21647 | expansion | 0 |
| R084 | S28 | 2102–2216 | 21962 | expansion | 0 |
| R085 | S28 | 2217–2544 | 21898 | expansion | 0 |
| R086 | S28 | 2545–2829 | 21867 | expansion | 0 |
| R087 | S28 | 2830–2911 | 21971 | expansion | 0 |
| R088 | S28 | 2912–3000 | 21933 | expansion | 0 |
| R089 | S28 | 3001–3322 | 21998 | expansion | 0 |
| R090 | S28 | 3323–3559 | 19008 | expansion | 0 |
| R091 | S29 | 1–253 | 21946 | expansion | 0 |
| R092 | S29 | 254–558 | 21972 | expansion | 0 |
| R093 | S29 | 559–889 | 21954 | expansion | 0 |
| R094 | S29 | 890–1023 | 21793 | expansion | 0 |
| R095 | S29 | 1024–1245 | 21972 | expansion | 0 |
| R096 | S29 | 1246–1473 | 21856 | expansion | 0 |
| R097 | S29 | 1474–1642 | 21955 | expansion | 0 |
| R098 | S29 | 1643–1989 | 21915 | expansion | 0 |
| R099 | S29 | 1990–2334 | 21947 | expansion | 0 |
| R100 | S29 | 2335–2679 | 21967 | expansion | 0 |
| R101 | S29 | 2680–3004 | 21925 | expansion | 0 |
| R102 | S29 | 3005–3080 | 7627 | expansion | 0 |
| R103 | S19 | 241–245 | 914 | expansion | 0 |
| R104 | S30 | 1–45 | 2221 | expansion | 0 |
| R105 | S06 | 1–65 | 4130 | expansion | 4130 |
| R106 | S10 | 1–90 | 4951 | expansion | 4951 |
| R107 | S19 | 1–75 | 5396 | expansion | 5396 |
| R108 | S09 | 1–66 | 7976 | expansion | 7976 |
| R109 | S14 | 1–40 | 3426 | expansion | 3426 |
| R110 | S17 | 1–30 | 2638 | expansion | 2638 |
| R111 | S20 | 1–30 | 2131 | expansion | 2131 |
| R112 | S18 | 1–30 | 2186 | expansion | 2186 |
| R113 | S16 | 1–30 | 2039 | expansion | 2039 |
| R114 | S15 | 1–30 | 2429 | expansion | 2429 |
| R115 | S22 | 1–30 | 1814 | expansion | 1814 |
| R116 | S21 | 1–30 | 3049 | expansion | 3049 |
| R117 | S31 | 1–246 | 19322 | expansion | 0 |
| R118 | S32 | 1–159 | 19836 | expansion | 0 |
| R119 | S31 | 230–280 (actual to 246) | 1154 | expansion | 1154 |
| R120 | S32 | 1–16 | 711 | expansion | 711 |
| R121 | S10 | 90–220 | 6671 | expansion | 6671 |
| R122 | S15 | 145–228 | 8026 | expansion | 8026 |
| R123 | S14 | 195–301 | 6709 | expansion | 6709 |
