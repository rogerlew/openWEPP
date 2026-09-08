# Exercise 01, independent retry 09 — surface-energy / soil coupling

Static: requirements reading only at `c2dd02bc33aa5dace9013970d1001cdcd6e8d72a`, independently confirmed by `git rev-parse HEAD`. No production code, simulation, numerical solver, test, comparator, restart workflow or downstream consumer was executed or examined. Ran: source readers, instruction discovery, hashes and declared source-byte measurement only. The findings below are requirements, not evidence that an implementation or actual run satisfies them. Candidate v32 adoption, production qualification, inherited FAIL/HOLD and paused EXP-R/PC1/SG1 remain unchanged.

## Answer and scope

A snow-free covered forest tile exchanges heat and vapor with its shared canopy-air node. LSE owns its surface enthalpy, hydrology owns every water mass, and soil thermal owns every ordered soil temperature and enthalpy. The single surface-to-first-soil-node transfer is equal and opposite in the two owners. Beginning water is immutable through potential solving, one authorization, and complete fixed-cap solving. Current ingress follows those operations; its retained energy enters surface enthalpy, its infiltration energy enters soil node 1, and routed water retains its upstream energy identity. It does not feed the already accepted same-interval H/LE/G solve.

“Snow-free covered forest” does **not** supply the actual model, capacity branch, phase state or run inputs. V1/V2 admit liquid/unfrozen litter and soil; V2 adds exact V10 vegetation specialization. V3 alone adds the admitted snow-free frozen-litter successor. It imports V2 and does not authorize frozen/thawing soil, bare/ponded V3 physics, represented snow or an ordinary unsplit terminal-snow payload. Contract revision 3 is not model V3. This report includes conditional litter-phase and snow-boundary rules because those exclusions, first-node custody, physical support and ingress cannot be resolved by assuming an unspecified run is V1. It does not review an unseen numerical implementation or optimization/replay equivalence.

All LSE links below are relative to `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/` (abbreviated **LSE/**); external names denote the current sibling `SC-…-001.md` files. File identities and every selected range are recorded in the ledger appendix. Stable headings and invariant/obligation IDs provide the rule anchors; none requires reading historical provenance to understand this answer.

## R01 — Control volumes, owners, admission and inputs

[LSE/surface-energy.md, “Selected sources and domain” and “Exact ownership and state,” lines 30–81; LSE/interface.md, INV-001/002/020/021/022/030/031/042/043; LSE/soil-coupling.md, INV-100/103/106.]

- Require snow absent at both endpoints and no terminal payload, positive finite interval/area and neutral-domain wind, complete forcing/configuration/state/lineage, one ground class per tile, and the admitted unfrozen soil/material branch. Calm wind, nonneutral stability, missing temperature lineage and frozen/thawing V1/V2 material reject before numerical calculation; no wind floor or inferred scientific default.
- One tile has one surface thermal node. Its dry body and positive hydrology-owned surface store are isothermal. Hydrology owns ponded/depression, litter liquid and ice, and every soil liquid/frozen store. Soil thermal alone owns all `N` soil temperatures/enthalpies; vegetation retains canopy physiology and component processes. Neither mass nor temperature nor enthalpy has a second mutable owner.
- A stand-ground basis is one OFE's horizontal area, not a routed hillslope. Tile fractions are finite, positive and unique; they sum to one under the canonical topology tolerance (`64*epsilon*max(1,sum(f))` in the surface-liquid topology). Convert a tile-local mass/energy to OFE-ground by `f_t` once. Preserve source/destination OFE identity and configured topological rank; opaque OFE IDs or digest order do not supply physical order.
- Require the frozen, strict configuration/state/forcing/water-protocol/diagnostic/owner-envelope identities and schema digests. Unknown, extra, duplicate, stale, nonfinite, wrong-owner or out-of-domain fields reject. An authority tag, units and actual provenance accompany every dimensional operand. LSE does not correct climate forcing or recompute phase.

## R02 — Equilibrium-zero is not a zeroed finite-capacity history

[LSE/soil-coupling.md, “Surface humidity, surface enthalpy, litter, and soil heat,” lines 31–134; INV-100/103/106.]

With `T_ref=273.15 K`, `C_w=4218 J kg^-1 K^-1`, and exact hydrology mass `W`,

`U_s = (C_dry + W*C_w)*(T_s-T_ref)`.

The configured **finite_capacity** branch requires `C_dry+W*C_w>0`. Authoritative beginning `U_s,W,C_dry` determine its physical beginning temperature. Its accepted ending enthalpy is the sole physical LSE state; derive ending temperature with the **ending hydrology candidate's** `W`. Any retained temperature warm start must be bit-identical to this derived candidate temperature and cannot be independently adjusted.

The configured **equilibrium_zero** branch requires `C_dry=0`, `W=0`, and `U_s=0` exactly. Its storage difference is exactly zero; `T_s` remains the algebraic energy unknown. There is no physical beginning surface temperature. Use the **current trial `T_s` at both surface-side Crank–Nicolson endpoints**, even when the caller provided a different warm start. No other zero-capacity branch, epsilon capacity, remembered physical temperature or warm-start storage is authorized. The actual configured litter properties must satisfy the selected branch; “forest” alone does not establish that equilibrium-zero is admissible.

## R03 — Covered radiation and air exchange are component-resolved

[LSE/surface-energy.md, “Shortwave and reciprocal longwave,” lines 84–122, and “Neutral turbulent heat and vapor network,” lines 125–190; SC-VEGETATION-001 whole contract.]

Ground receives terminal direct/diffuse VIS and NIR from the unchanged vegetation two-stream solve exactly once. Its configured class albedos are the full-column lower boundary; reflection returns through the overlying column. Longwave selects unit emissivity and no reflection. For each ordered occupancy, `P=LAI+SAI`, `tau=exp[-0.8*Omega*P]`, with `0<Omega<=1` applied once. Use current trial temperatures in both directional recursions, `Lup_ground=sigma*T_s^4`, and each sun-leaf/shade-leaf/wet/dry-stem component's own fourth-power emission. Exact zero component area supplies zero physical radiation. No bulk canopy temperature, stale surface temperature, imposed upward ground longwave or ground-to-atmosphere bypass.

A covered tile has one **zero-storage** `(T_c,q_c)` node. Component boundary/stomatal conductances connect to it; litter connects through the admitted neutral MEB ground-to-canopy resistance. In the positive-outgoing convention,

`H_s=rho_a*c_p*(T_s-T_c)/r_h`, `v_s=rho_a*(q_s-q_c)/r_v`,

`R_Tc=sum(H_components)+H_s-H_c->atm`, `R_qc=sum(v_components)+v_s-v_c->atm`.

Ground occurs once per tile, not once per occupancy. Heat and vapor are distinct operands even when their resistance is equal. Direct reference-air ground transfer beneath the canopy, omitted ground terms, and producer-aggregated canopy flux are invalid.

The exact neutral MEB equations/geometry in the cited section bind, including `phi_v=2`, `z0g=0.007 m`, `chi_L=0.12`, `u_l=1 m/s`, `l_w=0.02 m`, `nu=1.5e-5 m2/s`, `psi_H=f_hv=1`, `kappa=0.4`, and forcing `u_hv=u_ref`. Require `LAI>0`, `z_hv>d+z0v>z0g>0`, `z_ref-d>=z_hv-d>0`, and finite positive logarithms, exponentials and resistances. Canopy-to-reference exchange uses the configured displaced neutral log-law and its roughness-domain constraints. The open-tile law is a separate topology branch, not a covered fallback.

## R04 — Liquid litter and the one first-node conduction transfer

[LSE/soil-coupling.md, lines 80–134; INV-100/103/106; LSE/interface.md INV-013.]

For V1/V2 forest litter, `W_l,max>0`, `0<=W_l<=W_l,max`,

`h_ul=.5*(1-cos(pi*W_l/W_l,max))`,
`q_l=h_ul*q_sat(T_s,p)+(1-h_ul)*q_recipient`,
`v_l=rho_a*(q_l-q_recipient)/r_l-c`,
`lambda_l=.1+.03*W_l/(rho_w*dz_l)`, `C_dry=dz_l*rho_ld*c_ld`.

Litter blocks direct mineral-soil evaporation and upward capillary supply during this interval. Overflow belongs to hydrology's ingress, not an extra evaporative supply. Do not import the bare-mineral dry-soil humidity/DSL branch into a litter tile; positive surface-water and dry-mineral source selection remain distinct guarded classes.

For the ordered soil nodes `1..N`, require `N>=1`, positive `dz_k,lambda_k,C_k`, and positive surface `dz_s,lambda_s`. The half-cell series resistances give

`g_s1=2/(dz_s/lambda_s+dz_1/lambda_1)`,
`g_k,k+1=2/(dz_k/lambda_k+dz_(k+1)/lambda_(k+1))`,
`G_s1=g_s1*(T_s-T_1)`, `G_k,k+1=g_k,k+1*(T_k-T_(k+1))`.

These specialized `G` values are **positive downward**. Surface has `-G_s1`; first soil node has `+G_s1` exactly once. The generic interface's inward-positive surface `G` and soil `-G` use the opposite surface sign convention; they do not authorize negating the specialized downward soil credit.

With `bar(G)=.5*(G_begin+G_end)`, first-node storage is `C_1*DeltaT_1/dt=bar(G_s1)-bar(G_1,2)`, internal nodes use incoming minus outgoing, and the bottom flux is exactly zero. For `N=1`, use `C_1*DeltaT_1/dt=bar(G_s1)` only. Finite-capacity beginning conduction uses the derived beginning surface temperature. Equilibrium-zero uses `g_s1*(T_s_trial-T_1,begin)` and `g_s1*(T_s_trial-T_1,end)`. Soil phase change and an independently recalculated second interface flux are not admitted by this snow-free conduction law.

## R05 — Immutable beginning water, once-only authorization, fixed final solve

[LSE/water-vapor.md, “Immutable-beginning water transaction and current ingress,” lines 67–116; LSE/solve-boundary.md INV-108–110; LSE/nonlinear-solve.md; SC-SURFACELIQUID-001 and SC-WATBAL-001 INV-101.]

1. Snapshot beginning vegetation/LSE/soil thermal/hydrology **before** current rain, runon, throughfall, both canopy drainage terms, stemflow and overflow. Only named beginning surface/litter and soil-layer liquid stores can supply same-interval root uptake and ground evaporation.
2. Solve the complete potential system without owner caps. Publish source/layer-specific requests with exact transaction/OFE/tile/occupancy/surface/basis identity. Hydrology alone authorizes all competing requests on that snapshot **once**, before ingress. A changed final canopy release cannot enlarge or shrink supply.
3. Rebuild the **whole** final system from original beginning owners with those fixed caps: gas, hydraulic state, canopy/component energy, shared air, surface energy and soil thermal all reevaluate together. A cap is active for `cap_rate<=q_law`, including equality, with zero generalized derivative. Final use is `F=f_t*q*dt`; independently require `0<=F<=A<=D` and resource-wide availability.
4. Only accepted final uses—not requests or authorizations—construct owner debits/credits. There is no second authorization, supply donation, scalar stress substitute, or continuation of potential owner candidates/fluxes/branches/receipts.

Surface-liquid proportional authorization retains its prescribed binary64 operation order `fl(fl(D*S)/sum(D))` and **both** OFE-scale `sum(R)<=fW` and independently reconstructed tile-scale `sum(R/f)<=W` bounds. Its narrowly admitted E-003 common-factor correction uses `c=min(S/sum(R),W/sum(R/f))` and the greatest feasible binary64 factor at or below it, within the bounded search, preserving every positive row; otherwise typed failure. It is not permission for arbitrary clamps, row priority, last-row remainder or per-row `nextdown` repair. This authorization rule is distinct from downstream parcel attribution's allowed final-parcel remainder.

## R06 — Signed vapor carries liquid sensible plus latent enthalpy

[LSE/water-vapor.md, lines 20–63 and 171–207; INV-104/105/107; LSE/interface.md INV-012/014/043.]

`h_l(T)=4218*(T-273.15)`, `L_v(T)=2.501e6-2369*(T-273.15) J/kg`, and the positive-outgoing vapor energy is

`Q_v=v_s*[h_l(T_s)+L_v(T_s)]`.

Evaporation (`v_s>0`) removes both terms and requests positive water; condensation (`v_s<0`) credits both and has **no withdrawal request**, but must credit the exact named-store mass `-v_s*dt`. No zero-clipping, absolute values, latent-only energy, authorization-as-use or missing condensation mass. Interval-integrated root transpiration likewise retains one mass/energy transaction, stratum, area and authority-tagged vapor enthalpy; PMET remainder cannot supply canopy authority or a second ET debit.

Before current-ingress advection,

`(U_pre-U_0)/dt=R_sw+R_lw-H_s-Q_v-G_s1`,

where `U_pre` uses finalized pre-ingress hydrology mass. Every accepted current-trial primitive appears once. The energy identity is independently reconstructed, not accepted from a producer residual.

## R07 — Current-ingress energy and actual hydrology routing

[LSE/water-vapor.md, lines 20–37, 89–116; INV-106/107/128; SC-SURFACELIQUID-001 whole; SC-WATBAL-001 INV-105, P-032/C-007; SC-VEGETATION-001 accepted wet-release authority.]

After fixed-cap acceptance, apply finalized beginning-store uses and condensation; for V3, first complete R09's phase/spill operations. Then hydrology partitions final current ingress once into retained surface/litter, infiltration, routed runoff and outlet runoff. Ingress does not alter same-interval availability or cause another H/LE/G evaluation.

- Each liquid parcel has linked mass, `h_l`, `Q=m*h_l`, half-open support, reference state and lineage. A zero-mass crossing has **no temperature and zero energy**. For positive mass, `T_mix=T_ref+sum(m_i*h_i)/(C_w*sum(m_i))`; do not average temperatures without mass/enthalpy weights.
- Rain temperature is the exact retained `hydrometeor_temperature_c+273.15` output of `openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity` under active `harder_pomeroy_hourly`. LSE must not rerun or partially transcribe that phase solver. Runon uses the accepted upstream outlet's typed temperature/enthalpy; missing values reject, with no air/soil/freezing/downstream-temperature substitution.
- Covered throughfall, both drainage terms and stemflow use their accepted canopy wet-component release temperature/enthalpy without inventing a persistent temperature lane. `ground_ingress_mode` explicitly selects `open_raw` versus `covered_canopy_release`, independently of litter/mineral class; raw rain plus a covered release is double ingress. Erosion's rainfall input remains a distinct quantity.
- At each chronological source boundary, mix mass `X` and energy `Q` **before** one WB14 partition: `I+E=X`, `h=Q/X`, `Q_inf=I*h`, `Q_excess=Q-Q_inf`. Source attribution is proportional with its canonical final-parcel remainder. Existing configuration supplies actual infiltration/soil/tillage operands and cumulative state; no seeded infiltration, proxy or per-parcel copied WB14 call.
- Retention uses `R_raw=f*(Wmax-W)`, `M_raw=min(E,R_raw)`. Only the specified subresolution case `0<M_raw<=tau_M`, `tau_M=1e-14+64*epsilon*(abs(fWmax)+abs(fW)+abs(E))`, retains zero, preserves `W` bits, and routes the **whole mass and energy** onward. This is not a general mass/energy discard or tolerance repair.
- `U_s,end=U_pre+sum(Q_retained)` (with V3's phase/spill base); infiltration credits `E_soil,1` exactly once, and soil thermal derives the first-layer temperature from credited enthalpy. Routed mass and energy cross to the next OFE with `A_source/A_destination` once and explicit basis/identity; outlet exports leave the modeled domain. No lane-wide averaging, wrong layer, duplicated `f`, or scalar temperature correction.

WB14 is the one actual stateful partition per accepted child, with the canonical 48-by-1800-second parent context and current adaptive support rules, not a second full-parent replay. Same-pass infiltration changes the owning soil layers before the subsequent water-balance work; native depression capacity and associated WAT5 depression bins retain their admitted zero behavior. Parent cumulative cursors, ordered queues and before/after joins must match; local direct projection versus the Lane-D-local path is explicitly selected and exclusive after day zero. An inactive snow prefix advances only its authenticated cursor, not WB14 physical state; its first physical invocation remains ordinal zero.

Water and energy closure include independent per-OFE before/after operands and adjacent-area routing joins. Surface-liquid tolerances are `1e-14+64*epsilon*sum(abs(mass operands))` and `1e-9+64*epsilon*sum(abs(energy operands))`, with exact identities outside tolerance. Water-balance storage aliases are not interchangeable; litter ice is not soil frozen water. Native interval ET is not a second daily classic debit, and runon is supply, not runoff appended afterward. Real closure requires the actual Stage-3/surface/RUNOFFPART-WB14/Lane-D/per-OFE/hillslope/public WAT-PASS-HBP consumer chain; producer serialization or equal public `Q` conventions alone do not prove it.

## R08 — V2 fixed-final admission and inactive coordinates

[LSE/nonlinear-solve.md entire; LSE/solve-boundary.md entire; LSE/numerical-methods.md selected dependencies and canonical definitions INV-111/112/113/131/138/139; SC-VEGETATION-001 whole.]

V2 requires exact V10 vegetation identity and imports V1 physics, including its positive-PAR accepted results and V10's exact-zero-PAR/nonpositive-assimilation low-light branches. It cannot recompute, clamp or relabel the gas state. Nonpositive-assimilation partial **positive ROOT** authorization is typed unsupported; no attenuation, conductance/vulnerability floor, capacitance or donation.

FullSupply initialization requires **each** positive final authorization's identity and amount equal its potential request and its disposition be `FullSupply`; every zero request retains its exact identity and zero amount. Only coordinates may seed a complete original-owner fixed-final reevaluation. Iteration-zero acceptance has zero steps/backtracking and no Jacobian **only if** all residuals, domains, active inequalities, owners, identities and D/A/F checks pass there. It is not copying an accepted potential candidate. Checked V1-to-V2 migration copies scientific bits and changes only the admitted identity and derived receipts; V1 remains immutable. Mixed V8/V9 owners, absent/mutated nighttime gas and partial/value-changing migration retain E-109/110/111 failure families.

Exactly zero-area sun/shade/wet temperature coordinates anchor to `T-max(T_c,T_ref)` and contribute zero physical mass/heat/radiation; inactive dry stem anchors to `T_stem-T_c`. A nonzero wet area has the special anchor only under **all** INV-113 predicates: uncapped V10 nonpositive-assimilation potential, selected STORE-CAP, preliminary store rate no larger than canonical water residual tolerance, and already-passing unanchored wet-energy residual. A small positive rate can qualify. Mass, energy and longwave still evaluate normally; no clamp. It is unavailable to V1, positive assimilation, condensation or a constitutive-law wet-flux branch.

Accepted physical solve ordering is typed tile, ranked occupancy, component blocks, covered shared air, surface, then ordered soil; residual order matches. Bounds include `200<=T<=350 K`, `0<=q_c<=0.1 kg/kg`, plus vegetation's exact hydraulic/gas bounds. Energy residual thresholds are `1e-6 W/m2+1e-10*max(1,sum(abs(operands)))`; water/vapor `1e-12 kg/m2/s+1e-9*scale`; governed steps are `1e-8 K`, `1e-12 kg/kg`, `1e-7 mm` hydraulic and `1e-10` beta. A nested formulation requires every inner solve converged at every outer evaluation and the full ordered residual vector passing. Identity/unit/basis/owner/layer/band/direction and D/A/F predicates are exact.

No-update admission first prefers the existing full-trial witness. Only if **all current complete residuals pass** and the full trial is domain-invalid or exceeds a governed step may the first domain-valid halved trial supply fully evaluated prospective hydraulic/beta/T/q norms (`ci` diagnostic). Accept only the unchanged current iterate, never any trial state. Do not skip that first valid trial for a smaller one. Record its exponent contribution in the existing cumulative backtracking count, without a new persisted/public field. Real updates still require strict residual decrease; the existing 50-update/20-halving limits, domain failures, exact rollback and genuine limit-test dispositions remain. These are acceptance requirements, not a finding that any unseen solver followed its stencil, pivot or witness algorithm; such an implementation/equivalence review requires the whole numerical-methods route.

## R09 — V3 litter phase is a later, bounded operator

[LSE/litter-phase.md entire, especially lines 31–173, “spill,” “resource-join,” INV-140–149/151/153/154; SC-SNOWFREEZE-001 and SC-SURFACELIQUID-001 whole.]

Hydrology owns nonnegative liquid `W_l` and liquid-water-equivalent litter ice `W_i`; LSE V3 owns sensible `U=(C_dry+W_l*C_w+W_i*C_i)*(T_l-T_ref)`. Litter ice is neither snow nor soil `frozwt`. The complete V2 system is solved **phase-free** with original beginning phases: no fusion, phase-adjusted capacity/temperature, freeze/melt or reauthorization enters its residuals, derivatives, branches or witness.

Use immutable `p_i=0` when both pools are zero, otherwise `W_i,0/(W_l,0+W_i,0)`, and cosine humidity factors for each original named pool/capacity. The raw V3 laws are

`v_l,raw=(1-p_i)*rho_a*(h_ul*q_sat_liquid(T_l,p)-q_recipient)/r_l-c`,
`v_i,raw=p_i*rho_a*(h_ui*q_sat_liquid(T_l,p)-q_recipient)/r_l-c`.

**Both use saturation over liquid water**, and these raw expressions are not V1's recipient-humidity blend. Positive liquid/ice flux is authorized against only its own immutable beginning pool; negative flux credits only its named phase without an availability cap. Keep signed liquid and ice masses and energies separate through authorization, construction, sealing and independent reconstruction; only then may an air consumer aggregate. Liquid uses `v_l*[C_w*(T-T_ref)+L_v(T)]`, ice `v_i*[C_i*(T-T_ref)+L_s(T)]`; no latent-only record, total-store cap, cross-phase donation or second daily SC-EVAP debit.

After fixed-cap vapor acceptance, `W_l,*=W_l,0-dt*v_l`, `W_i,*=W_i,0-dt*v_i`, and accepted primitives must independently reproduce `U_*` and `T_*`. The phase constants are `tau_ice=3300 s`, `rho_i=920 kg/m3`, `C_i=2106 J/kg/K`, `L_f=333700 J/kg`, `T_ref=273.15 K`, and **`W_i,max=.85*rho_w*dz_l`** (water density, not ice density). Require valid post-vapor masses, ice capacity and positive finite ending heat capacity. Define

`M_warm=rho_i*C_i*dz_l*max(T_*-T_ref,0)/L_f`,
`M_cold=rho_i*C_i*dz_l*max(T_ref-T_*,0)/L_f`,
`m_melt=min(W_i,*,(dt/tau_ice)*min(M_warm,W_i,*))`,
`m_freeze=min(W_l,*,W_i,max-W_i,*,(dt/tau_ice)*min(M_cold,W_l,*))`.

The outer bounds remain mandatory for `dt>tau_ice`; this creates no maximum step size. At `T_*=T_ref`, both transfers are exactly zero. Set liquid/ice masses by equal-opposite transfer, `U_phase=U_*+L_f*(m_freeze-m_melt)`, and derive `T_phase` with the **ending** capacity. `H=U-L_f*W_i` is invariant for phase alone. Adding `L_f*m/C_old` to temperature, or rerunning fluxes/fixed point/Newton/authorization on the same support, is invalid. Phase is the ending thermal state and next-support warm start. WB14 can use only admitted liquid; ice never becomes infiltration/runoff/depression or soil frozen-water supply by alias.

If phase creates liquid over capacity, preserve the original phase receipt; remove the exact excess mass at the **raw phase temperature**, its linked sensible energy, and derive retained mass through the specified second subtraction—not a `min`/clamp rewrite. The spill is a named **negative exact surface-energy operand after fusion**, with tile-to-OFE conversion once, represented as one full-child `[0,dt)` internal Overflow parcel for ordinary WB14 routing. Any later retained spill portion is an ordinary positive retained-ingress credit; it does not cancel the original spill receipt or disappear into tolerance.

For heterogeneous final resource joins, match each native phase receipt to its D/A/F row and exclude that already-applied row exactly once. Other canonical ordinary rows debit final `F/f` once from the phase-adjusted V2 water owner, including exact zero-row identity. Preserve phase, spill, ice and exact-energy custody. This mass join supplies no second energy or ingress operation and cannot replace an entire phase-adjusted owner with an older candidate.

## R10 — Exact accepted energy belongs to each receiver

[LSE/soil-custody.md, “exact-soil,” INV-150, P-005/C-005; LSE/surface-custody.md, “exact-surface,” P-006/C-006; SC-SURFACELIQUID-001 exact-soil/surface amendments.]

Soil layer `k` stores `E_k=exact(H_hi,k)+R_k` on OFE-ground; the surface owner similarly stores exact `U` on its own declared basis. `H_hi` is finite binary64; `R` is a normalized signed arbitrary-precision dyadic. Zero has sole `(sign=0,coefficient_hex="0",exponent2=0)` form; nonzero has sign ±1, positive **odd** lowercase hexadecimal coefficient without leading zero and the unique exponent. Reject equivalent noncanonical forms, overflow/resource abuse and nonfinite highs; parser resource limits must still accept all carries reachable from the configured finite transaction. Carry-zero normalization does not rewrite the high term's signed-zero bits.

The receiver validates complete schema/definition/parent/model/configuration/topology/owner/state/transaction/predecessor/support/layer/receipt identities, then independently decodes the beginning exact total and **every accepted finite binary64 primitive**: soil internal conduction, surface/snow top boundary and infiltration; or surface phase-free energy, fusion, exact spill debit and retained ingress. Required physical binary64 area conversion and canonical grouping occur **before** exact decode. Source owner/kind, basis, layer, support and ordinal remain binding even though exact addition is numerically order-independent.

Aggregate with exact integer arithmetic, round the total once to binary64 nearest-even, reject nonfinite/overflow rather than clamp, and store the exact difference as normalized carry. No producer aggregate/residual/carry, compensated floating state, tolerance laundering, `nextafter`/forced ULP, nonzero-carry deletion, subnormal flushing, persistent microstepping or new diagnostic owner. Carries do not feed constitutive temperature/flux/phase laws or change heat capacity or tolerances. Native V3 and V2 surface high/carry mirrors match bitwise. Checked migration copies all scientific/high signed-zero bits and adds exact-zero carry; it never infers a temperature, and production downgrade is rejected even when carries are zero.

The canonical **specified fixture**, not an observed run here, starts soil `H_hi=-34315.42154113602 J/m2` and adds `-8.0670339832330148e-19 J/m2`. Its unchanged nearest-even high and carry `(-1,"1dc319224e55f",-109)` retain the small negative credit exactly. The actual p61 high, primitive credits and run are missing; even the specified p61 support `176400000000000..178200000000000 ns` supplies no missing energy operand.

## R11 — Candidate/read custody cannot become an accepted owner

[LSE/soil-custody.md “soil-beginning,” INV-155/C-010; LSE/surface-custody.md parent-custody/reseal amendments; LSE/map-custody.md “handoff” and “pending”; SC-SNOWENERGY-001 INV-084/085 and C-031.]

A partial parent may advance exact physical totals/digests while retaining its predecessor last-accepted marker. Final acceptance stamps the selected child once across the complete owner/carry/receipt surfaces. Typed configured OFE order, complete canonical sources/layers and receipt sequence bind; lexical/numeric ID sorting cannot replace topology.

An unpublished soil continuation may supply one charged candidate evaluation only through the authenticated **borrowed non-owner** `SoilThermalUnpublishedPhysicalBeginningV2`, with original prepared owner, immutable identity, exact predecessor-trial seal and contiguous positive child support. Its existing prepared soil transaction must not be rebound to the outer LSE/surface transaction. It cannot construct, serialize, install, expose or restore owner-shaped, restart, checkpoint or accepted-credit bytes. Final acceptance independently replays the original prepared owner plus **all** accumulated canonical operands and exact selected ending, then produces one owner/receipt/restart/checkpoint seal and atomic install. No intermediate owner synthesis, dual acceptance, proxy physics, receipt repair or tolerance comparison of identity.

Soil-energy operand **source** and prepared-soil **target** transactions are separate exact fields: authenticate ingress/physical receipts against source and prepared soil against target, bind both in the digest, and reject swaps, inference or support rebinding. The pending native-map soil-close branch authenticates live checkpoint clock/end-owner/predecessor and its independent vegetation finalization/LSE-BGC source joins; any admitted soil-only same-tick close is single-use and the later vegetation finalization must match its exact successor.

In-process immutable validation proofs are scoped to their exact object/configuration/support/identity and original validation position; mutation invalidates them. External durable/restart bytes require full validation. Snow-free final resealing may change only admitted slab-dependent identity/receipt fields, preserving the same ending physics and consumed operands; it is single-use, nonserializable proof custody, not a replacement physical solve. A distinct pending covered/native branch cannot fall back through it. This report does not claim evaluator/reuse equivalence; that claim requires the unselected dependency-replay route.

## R12 — Snow exclusion, terminal ingress and real physical support

[LSE/terminal-support.md entire; LSE/soil-coupling.md “Version 8 persistent snow--soil boundary amendment,” INV-124–126; LSE/map-custody.md entire; SC-COUPLEDTIME-001, SC-SNOWENERGY-001 and SC-SNOWFREEZE-001 whole.]

Classify represented snow before entering V3 litter physics. A represented-snow map owns its own optical/lower-boundary receipts; litter vapor, phase, energy/ingress/WB14 state remain inactive and byte-preserved there. Covered does not mean snow-covered. At an actual accepted terminal event, remove only terminating lanes and deliver the produced zero-degree-C liquid parcel exactly once to the post-event surface/WB14 receiver; snow fusion energy remains with snow and is not a second sensible/G credit. Rebuild receiver physics on actual post-event surfaces for the positive remaining half-open support; zero remaining support means no solve. No old snow flux reuse, guessed terminal energy, schema-v8 censored terminal payload or vapor-created snow reappearance.

For a **represented** snow lower boundary, couple the bottom snow thermal volume directly to **ordered OFE soil node 1**. Tile nodes do not participate: no tile selection, averaging, fraction weighting or broadcast. Use the same half-snow/half-soil harmonic resistance and immutable beginning/current candidate ending temperatures **inside** the covered fixed point. Snow gets `-bar(G_ss)` and first soil gets `+bar(G_ss)` once; internal soil conduction and zero bottom remain. A `SnowSoilHeatReceiptV1` binds the exact support, node/owner identities, four resistance operands, endpoint pairs/fluxes and both candidate ends. Reconstruct independently from primitives. TOL-SNOWENERGY-005 endpoint resealing retains the heat actually consumed and requires `1e-9 J/m2` and `1e-8 K` endpoint agreement; it must separately satisfy `1e-6 J/m2` physical-ledger closure. No heat/enthalpy repair. Failure is LSEB-E-044/SNOWENERGY-E-SOIL-HEAT-001 with all owners unchanged.

Every positive covered physical invocation, including receipt-only admission, is subject to the exact **60-second = 60000000000-ns** floor before Newton; subfloor supports reject without candidate construction. Exact 1-ns chronology is not physical permission. Ordinary stable supports must remain substantially larger than 60 seconds; the old 600-ms fallback does not survive. Coupled minimum is the maximum across active participants, with no inactive-profile inflation. Event coalescence requires both neighboring supports zero or at least the minimum and all four endpoint tolerance checks, then ranks displacement, normalized error and earliest tick. No feasible event means atomic retry, not lost support or a changed tolerance.

Current SnowFreeze INV-103–106's v140 adaptive lifecycle supersedes INV-101/102 and rejected v137–139 lifecycle proposals. Positive supports use integer 60-second quanta; where at least two quanta permit comparison, evaluate the full proposal and two positive children from immutable beginnings and accept the composed child ending only under the joint estimator/exact guards. One quantum has no further split and still needs exact/discrete/closure acceptance. All active lanes and shared vegetation/LSE/hydrology/surface/WB14/soil/BGC/provider/clock owners advance in one joint candidate with maximum error and exact discrete decisions; independent per-lane commits cannot be merged afterward. An exact-zero-ice event is a zero-duration topology transition with one produced/unconsumed parcel per terminating lane. Only admitted solid precipitation can create a new snow state with its full density/enthalpy/layer/custody identity.

Restart binds accepted adaptive operation boundary, parent/support cursor, next proposal, accepted history, all owner/topology state, pending event parcels, forcing/provider/GSI receipts and clock—not rejected trials. Require byte-identical continuation across disappearance, consumption, reappearance and midnight. No CoE/fixed-step/legacy terminal/mass-threshold/model fallback or permanent 60-second season stepping as production qualification.

For covered snow's conditional native-map custody, roles are Initial ordinal 0, FixedPoint ordinal 1, then the at-most-five admitted Multisecant roles: canonical 2–7 maps under the shared ceiling of 8. Pending history is its own exact charged physical prefix; invalid histories fail rather than using another map. TOL-SNOWENERGY-007 compares fresh `F(x_k)` with its charged `x_k`, plus exact identity/branch/topology and each physical closure, not prior `F` equality. Finalization promotes that same prefix with no extra physical map or nonfinal owner publication. Historical alternate solver/root-polish/96-call recovery schemes are not current fallback authority. Snow constitutive fusion `333600 J/kg` is not V3 litter's `333700 J/kg`. These conditional exclusions/custody rules do not assert execution of a snow solver on this unspecified snow-free tile.

Phase routing precedes the “inactive” shortcut. SC-SNOWFREEZE INV-075 retains the active hourly Harder–Pomeroy provider and INV-089 requires resolution of material precipitation `>1e-12 m` before inactivity: material hourly solid precipitation can activate snow despite warm mean temperature and zero initial pack. All projected snow guards precede inactivity; fractional rain/snow phase and `hrrain+hrsnow/10=active hourly precipitation` remain. The provider uses typed Celsius and RH with its narrow documented upper-one normalization where authorized; negative/nonfinite RH or invalid saturation inputs fail. LSE cannot infer rain-only from mean temperature, normalize provided phase masses away or reconstruct retained hydrometeor temperature. A warm all-rain zero-pack may be inactive **after** phase admission; warm dry bypass is separately guarded.

CoupledTime exact checked `u128` nanosecond/string chronology, once-converted shared floating duration bits and canonical rational event rounding remain; attempted maps do not advance accepted time. Parent sequence advances once, scheduled work occurs once, sequential phenology keeps its sequence, and rejected/nonfinal owners are unpublished.

Legacy `surtmp` and `Thra` Celsius surfaces are not aliases for LSE Kelvin surface temperature. Any admitted migration requires explicit authority-tagged conversion and atomic cutover with complete owner/receipt lineage; no silent temperature alias or mixed old/new path. This is a requirement, not inspected migration evidence.

## R13 — Required evidence and typed failure

[LSE/common-details.md “Test Vector Obligations,” “Tolerances,” “Algorithm,” and producer obligations P-001–004; LSE/water-vapor.md “errors”/“v1-invariants-and-independent-fixtures”; phase/custody/admission leaf obligations; six external whole-contract tests.]

No actual input record or run result was supplied. To apply these requirements, obtain the exact selected model/capacity/regime/ground-ingress path, schema/configuration/contract digests, OFE/tile/topology/areas, forcing/provider receipts and interval/clock, VIS/NIR albedos and radiation, neutral geometry/wind/pressure/humidity, canopy occupancies/component areas/V10 gas/hydraulic/wet-release state, surface dry/litter properties and capacities, original hydrology phase/layer masses, surface U/high/carry and warm starts, ordered soil geometry/properties/U/T/high/carry, all prepared/previous/candidate owner/support identities, root/ground D/A/F rows, positive-parcel temperatures/enthalpies, queue/WB14/Lane-D/output consumer state and any terminal/phase/restart receipts. Formula constants are not missing actual operands. No finite-capacity history, p61 credit, water availability, phase flag or temperature may be manufactured.

Required evidence includes:

- Digest-bound independent physical fixtures: open bare day/night, covered/open dry/wet litter, two heterogeneous columns, zero shortwave, longwave/ground-heat sign reversal, canopy ground feedback, evaporation/condensation, full/partial caps and equality, concurrent root/ground scarcity, dry source, all rain/runon/infiltration/runoff paths, equilibrium-zero/finite storage, alternate warm starts and `N=1`/multi-node conduction. Reconstruct **local first, then f-weighted OFE** energy/mass closure from all distinct signed primitive radiation, turbulent, vapor sensible/latent, storage, ground and advective operands.
- Poisons for each omitted/doubled/wrong-sign component; bulk/stale/repartitioned radiation; direct ground-to-reference beneath canopy; omitted shared-air terms; PMET donation; current ingress availability; final-release authorization changes; missing/doubled fraction or cross-area conversion; authorization as use; clipping/latent-only/missing condensation; wrong parcel temperature/reference state; wrong soil node/owner; independent duplicate G; wind floors and producer residuals. Reject snow/frozen/thawing/calm/nonneutral/terminal domains and prove singular/backtracking/iteration/closure and later-transaction **byte-exact all-owner rollback**.
- V2 zero/low-light FullSupply and exact zero-row identity, iteration-zero reevaluation, all inactive-anchor predicates, cold zero-area phase-domain tests, no-update full/first-valid success and each residual/nonfinite/coordinate/incomplete/skip-first poison. Preserve genuine strict-decrease/limit vectors. The two real interior terminal paths named by the admission contract are required evidence, not runs performed here.
- V3 empty/liquid/ice/mixed pools; freeze/melt/exact-reference temperature; condensation/deposition; separate caps and vapor energy; both sides of `tau_ice`, exact floor and ordinary larger support; no phase/ice donation; exact fusion/ending capacity; spill/retention/heterogeneous row joins; no re-solve; all units/sign/saturation/capacity/chronology poisons. Real p61 and native-forest production-selector, persist/restore and next-support consumers must expose primitive vapor, phase, fusion, WB14 mass and energy closures.
- Exact soil/surface carry positive/negative sub-ULP credits, even/odd halfway ties, adjacent-high crossings, opposite-sign and exact-zero cancellation, subnormals and normal boundaries, largest-finite boundary/overflow, high signed zero, noncanonical encodings and finite resource bounds, source/layer/transaction/support/order/receipt omission/duplication/substitution, exact mirrors, real WAT5 and p61/native-forest consumers, restart split before/after nonzero credit and replay rejection. No producer-only, schema-only or tolerance-only adoption evidence.
- Conditional snow first-node primitive resistance/endpoints/signs/receipt and physical closure, minimum/support/coalescence, same-map/borrowed-soil/final-seal custody, adaptive all-lane rollback and event/restart continuation, including independent publication-path operands, unequal OFE areas and factor-1000/basis swaps. A public `QOFE==Q` convention is not independent proof of mass provenance.

Producer P-001–004 duties remain universal: expose every signed primitive in sealed handoffs, prevalidate and commit atomically, and retain branch/support/lineage/tolerance/residual evidence; censored terminal values are inadmissible. P-005/P-006 expose every accepted soil/surface primitive. Consumers retain one ET debit (C-001), one WB14 ingress partition (C-002), one opposite-sign soil transfer and sole subsurface mutation (C-003), an actual direct scheduler consumer (C-004), and receiver-owned exact custody (C-005/C-006), plus the specialized final/read-custody duties. A cloned output, serialized ledger, helper agreement, comparator match or implementation skeleton cannot replace an acting real consumer.

First-error ordering is serialization; model/configuration/state/transaction identity; topology/owner; nonfinite; unsupported branch; constitutive domain; D/A/F; numerical singular/backtracking/iteration/accepted-step failures; component closure; control-volume closure; cross-owner join. Typed diagnostics retain the applicable identity, ordered residual/count/step/bound/cap/pivot evidence and rollback hashes up to that error. Water-vapor error families E-030–040, boundary E-044, exact-soil E-049 and specialized phase/surface/custody families remain; no failed iterate or partial candidate is usable. Preserve vegetation, hydrology, LSE, BGC, soil thermal and envelopes on any failure. No generic clamp, silent fallback or tolerance repair of exact identity is authorized.

## Reading expansion and limits

The entry route required the shared interface, then common physical details for this physical task. Surface-energy, soil-coupling, water-vapor and solve-boundary supply its constitutive and chronological answer. Terminal-support is required even for snow-free physical/receipt support; unspecified litter phase and its current-ingress/spill obligations require the full litter-phase chapter. Accepted first-node/retained credits require soil- and surface-custody; their same-map, pending, borrowed-beginning and reseal dependencies require map-custody. Accepted-primitive admission requires nonlinear-solve plus the selected unique numerical-methods definitions, guards and tests. These are independent obligations, not reading shortcuts justified by the question's wording.

The interface explicitly requires **every selected external single-file contract in full**. Vegetation was required for component radiation, canopy air, V10 physiology/requests/releases; CoupledTime for physical/zero/accepted support; SurfaceLiquid for authoritative water/ingress/WB14 and exact receiver credits; SnowEnergy for represented-boundary, native/pending and first-node receipt custody; WATBAL for one real water-partition/downstream closure; SnowFreeze for the mandatory actual phase-provider source, phase/inactive priority, physical support and adaptive terminal lifecycle. All six were read from first through last line, including their current superseding amendments. Historical prose encountered inside a mandatory whole current file did not license following archives or applying a superseded algorithm.

No old LSE, Git history, archive, intake/prediction/rubric/review record, earlier exercise, other answer or parent validation evidence was read. The current whole external files contain their own historical/rubric-like material; it was unavoidable within the explicitly mandated whole-file reading, and no separate forbidden artifact was opened. No agent-listing or waiting was used. Current commit metadata was read without Git history/source retrieval.

The unselected whole numerical-methods algorithm and dependency-replay routes would be mandatory for solver/branch implementation, evaluator/reuse equivalence or optimization claims. Replay-evidence would be required for executable identity capture/experiment/replay-retention qualification. Audit-details would be required for a provenance/gap/enforcement/adoption adjudication beyond the physical-rule question. This report makes none of those claims, does not alter a gap or promotion disposition, and does not claim frozen protocol/runtime qualification. Existing required scientific/production limits still bind. Detailed RUNOFFPART constitutive implementation review or a code/test claim would require its own additional authority and source; merely naming the already-read SurfaceLiquid/WATBAL sole-consumer requirement does not prove that implementation.

## Private omission and claim audit

Before this report, I created an independent source-derived inventory v1, then retained it and expanded v2 after all whole external readings. V2 has 60 numbered inventory items (with compound qualifiers), covering equations, ownership, units/signs, exclusions, chronology, actual inputs, conditional boundaries and evidence duties. It is a private checklist, not authority and not permission to omit reading. I compared all items against R01–R13 for omission and separately checked written claims against the selected current sources. The audit covers requirements only; it cannot certify an unseen solver or run. Exact private helper/inventory versions and their hashes/bytes are retained below. This is the first completed report; no completed-report repair follows it.

## Measurement and exposure record
Measurement is UTF-8 **selected source bytes**, not model tokens, runtime, cost, delivered output bytes or whole-workflow telemetry. `tools/agents/context_report.py` was run against the hash-verified working tree, without `--revision` or Git source retrieval. Its explicit selection lists every source request and the separately declared source-equivalent search/automatic exposures. Configured/effective model, reasoning-effort setting, delivered-token counts and automatic context beyond the known root AGENTS are **UNOBSERVED**. No runtime telemetry is inferred from bytes.
The explicit-reader ledger contains **158 requests**, **34 source identities**, **2,735,953 requested bytes**, **2,313,472 unique bytes**, and **422,481 repeated bytes within that ledger**. The eight search/path-pattern records expose another **8,838 matched source bytes**, of which **771** lie outside the explicit-read union. The known automatic root AGENTS contributes **9,508** source-equivalent exposure bytes, already in the union. Combined declared exposure is **2,754,299 bytes**; finite source union including searches is **2,314,243 bytes**. Cross-category/global repetition is their difference; new/repeat columns below deliberately mean prior explicit-reader requests, not an invented timestamp ordering against automatic or early search exposure.
| Class | Explicit unique bytes | Explicit requested bytes | Explicit repeat bytes | Search-only added union |
|---|---:|---:|---:|---:|
| Governance/package | 70,604 | 89,556 | 18,952 | 591 |
| LSE | 203,883 | 417,548 | 213,665 | 180 |
| External | 2,035,441 | 2,221,761 | 186,320 | 0 |
| Measurement helper source | 3,544 | 7,088 | 3,544 | 0 |

LSE entry plus the always-required interface is **12,021 bytes**. This exercise's finite LSE reading union including source-equivalent search headings is **204,063 bytes**; versus the package's stated 264,863-byte comparison figure this is a structural reduction of **22.9553%**. No old source was opened to calculate that denominator, and this arithmetic is not a semantic-preservation or adoption verdict. Mandatory external whole reading is **2,035,441 unique bytes** and is not hidden in the LSE number.
The context reporter declares bootstrap unique/exposure **66,940/76,448** and expansion unique/exposure **2,285,412/2,677,851** bytes. Bootstrap contains initial instruction/package/entry/shared requests plus the known automatic root instructions; later mechanism, external, governance, verification and search requests are expansion. Stage unions overlap; only the combined union is deduplicated across stages. Every fine-grained recorded phase is retained in the request table.

### Source identities and selected coverage

A source ID resolves every request/search row to this exact complete-file SHA-256. Coverage is the explicit-reader line union; selected numerical/standards sections are intentionally bounded and search-only headings are accounted separately. All six external files and every selected whole LSE chapter have complete first-to-last coverage. Source hashes were rechecked at final audit and none changed.

| ID | Source path | Complete-file SHA-256 | Selected inclusive lines | Unique selected UTF-8 bytes |
|---|---|---|---|---:|
| S01 | [AGENTS.md](/workdir/openWEPP/AGENTS.md) | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` | 1–121 / 121 | 9508 |
| S02 | [docs/work-packages/AGENTS.md](/workdir/openWEPP/docs/work-packages/AGENTS.md) | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` | 1–105 / 105 | 6642 |
| S03 | [docs/work-packages/20260907-lse-context-adoption-correction-001/package.md](/workdir/openWEPP/docs/work-packages/20260907-lse-context-adoption-correction-001/package.md) | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` | 1–75 / 75 | 5304 |
| S04 | [docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md](/workdir/openWEPP/docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md) | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` | 1–22 / 22 | 1586 |
| S05 | [docs/work-packages/role-review.md](/workdir/openWEPP/docs/work-packages/role-review.md) | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` | 1–7 / 7 | 1400 |
| S06 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md) | `391772bfc9345357a654552ae84203809a91e0280f5579693ae73da68119cc53` | 1–67 / 67 | 4090 |
| S07 | [docs/specifications/science-contracts/AGENTS.md](/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md) | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` | 1–84 / 84 | 6532 |
| S08 | [docs/work-packages/science-obligations.md](/workdir/openWEPP/docs/work-packages/science-obligations.md) | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` | 1–98 / 98 | 6149 |
| S09 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md) | `758873659d360c8edfc7bcec7aa6cb58d065d44e221e890166390252810ed481` | 1–66 / 66 | 7931 |
| S10 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md) | `2c3afa3f25827508ab9bd4826249d6557d08aa44b8f4ba7f0ecd5c988a900bb8` | 1–205 / 205 | 11494 |
| S11 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md) | `b930bbd0f0ce2c16d826be6a6d1d0bb4e6fe0155e611232047e0b4205e2dbfc4` | 1–221 / 221 | 17798 |
| S12 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md) | `a85b0aa76a404143b049a56a25225250b4eddeabdf8ac2fc895c1aee27a4434d` | 1–219 / 219 | 13780 |
| S13 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md) | `2222800b0e5fa73d281ba68ec21961a6fcc8b38401f9c59d045139e2671dd709` | 1–244 / 244 | 20914 |
| S14 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md) | `4fdd656cc3884fafff85fe5d5d3ee30deb9cb5e015c5e6c0ca245c967d4a99e3` | 1–433 / 433 | 30863 |
| S15 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md) | `62e13854fe0bdddb71a4e5aae874aa13c89431bef3d47526b9af973ec8014b54` | 1–114 / 114 | 8368 |
| S16 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md) | `fda2350a1d4bf826dd3ad6f622e4ee0f41d708860aef6005e8948c27c3d729f8` | 1–227 / 227 | 15949 |
| S17 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md) | `eac50350e934d03e47022dcc51d0c55013c5a5d975f1beceb6f6b7196cc03b18` | 1–200 / 200 | 15323 |
| S18 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md) | `4acd7ddf7a20d9c1198a41ee26d4caa8c57361728eb893ae365c31c669fe66e3` | 1–291 / 291 | 25523 |
| S19 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md) | `bb75c9248557ae704992c1f5629c1b0ab563d764cef193f6a8e220907671dc43` | 1–94 / 94 | 6549 |
| S20 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md) | `8508b3ceaa49f081ce8c2df1d7cce39249d8f3875fea5a017404c0816cdae2c1` | 1–170 / 170 | 19110 |
| S21 | [docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md) | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` | 1–3080 / 3080 | 248829 |
| S22 | [docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md) | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` | 1–1108 / 1108 | 92552 |
| S23 | [docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md) | `172438a4196f13e3b9260845680abc81b98c55d3af044dbda744f5a1ca6932f8` | 1–26, 93–107 / 107 | 6191 |
| S24 | [docs/standards/testing-and-gate-strategy.md](/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md) | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` | 193–345, 439–473 / 494 | 9329 |
| S25 | [docs/standards/prompt-wording-guidance.md](/workdir/openWEPP/docs/standards/prompt-wording-guidance.md) | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` | 174–194 / 202 | 1333 |
| S26 | [docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md) | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` | 1–2205 / 2205 | 184116 |
| S27 | [docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md) | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` | 1–3559 / 3559 | 497785 |
| S28 | [docs/specifications/science-contracts/contracts/SC-WATBAL-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md) | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` | 1–2648 / 2648 | 377637 |
| S29 | [docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md](/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md) | `50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4` | 1–4295 / 4295 | 634522 |
| S30 | [docs/prompt_templates/assurance-findings-template.md](/workdir/openWEPP/docs/prompt_templates/assurance-findings-template.md) | `95373d3c7df1f0a7e3ad7640fa22c0e137095e6cf5c4d92e3803846eb3f6ee83` | 1–12 / 12 | 736 |
| S31 | [tools/agents/context_report.py](/workdir/openWEPP/tools/agents/context_report.py) | `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde` | 1–79 / 79 | 3544 |
| S32 | [docs/specifications/correctness-authority-model.md](/workdir/openWEPP/docs/specifications/correctness-authority-model.md) | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` | 1–221 / 221 | 11159 |
| S33 | [docs/standards/local-ci-gate-selection.md](/workdir/openWEPP/docs/standards/local-ci-gate-selection.md) | `9b2cd01f06d433f99b9dc9ae3d596c4a348c23b9260ef4ecbe197efa84f60e8e` | 1–129 / 129 | 6874 |
| S34 | [docs/standards/AGENTS.md](/workdir/openWEPP/docs/standards/AGENTS.md) | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` | 1–58 / 58 | 4052 |

### Complete explicit source-request ledger

Inclusive ranges are the reader's exact selected source extent. V3 clips a requested upper bound to the file end and then to a 28,000-byte whole-line output budget; the ledger records that effective requested/emitted selection, not unselected suffix bytes. The Python helper necessarily reads the file to hash/select it; unexposed file-loading bytes are not counted as model source exposure. Original wide upper requests for the SnowFreeze whole-file continuation were 4295 and for the WATBAL V3 continuation 2648; continuation starts follow the prior recorded effective end. The selected litter audit request 26–433 ended at 432 under the budget, with line 433 already read. Original failed out-of-file numerical-methods 93–109/108 requests returned only helper tracebacks and **zero source body**, before the successful 93–107 read. One unavailable bare-`python` attempt likewise exposed no source.

The early output renderer truncated some broad source requests; requested-byte counts still describe those selected requests, not a claim all their bytes were delivered then. Every missing selected extent was reread. Recovery rows below, including conservative boundary repeats, record those bytes again. Soil/terminal/surface-custody recoveries and Vegetation, SnowEnergy and WATBAL recoveries are retained; final whole coverage is complete. Exact delivered-byte/token telemetry for those rendered outputs is unavailable.

| Request | Source | Inclusive range | Phase | Requested UTF-8 bytes | New within explicit ledger | Repeated within explicit ledger |
|---:|---|---|---|---:|---:|---:|
| 1 | S01 | 1–121 | instructions | 9508 | 9508 | 0 |
| 2 | S02 | 1–105 | instructions | 6642 | 6642 | 0 |
| 3 | S03 | 1–75 | bootstrap | 5304 | 5304 | 0 |
| 4 | S04 | 1–22 | bootstrap | 1586 | 1586 | 0 |
| 5 | S05 | 1–7 | instructions | 1400 | 1400 | 0 |
| 6 | S06 | 1–67 | entry | 4090 | 4090 | 0 |
| 7 | S07 | 1–84 | instructions | 6532 | 6532 | 0 |
| 8 | S08 | 1–98 | instructions | 6149 | 6149 | 0 |
| 9 | S09 | 1–66 | shared | 7931 | 7931 | 0 |
| 10 | S10 | 1–205 | surface | 11494 | 11494 | 0 |
| 11 | S11 | 1–221 | shared | 17798 | 17798 | 0 |
| 12 | S12 | 1–219 | mechanism | 13780 | 13780 | 0 |
| 13 | S13 | 1–244 | boundary | 20914 | 20914 | 0 |
| 14 | S12 | 1–219 | truncation-recovery | 13780 | 0 | 13780 |
| 15 | S13 | 1–23 | truncation-recovery | 2260 | 0 | 2260 |
| 16 | S14 | 1–433 | mechanism | 30863 | 30863 | 0 |
| 17 | S15 | 1–114 | mechanism | 8368 | 8368 | 0 |
| 18 | S16 | 1–227 | mechanism | 15949 | 15949 | 0 |
| 19 | S17 | 1–200 | mechanism | 15323 | 15323 | 0 |
| 20 | S18 | 1–291 | mechanism | 25523 | 25523 | 0 |
| 21 | S18 | 1–143 | truncation-recovery | 9369 | 0 | 9369 |
| 22 | S19 | 1–94 | admission | 6549 | 6549 | 0 |
| 23 | S20 | 1–170 | custody | 19110 | 19110 | 0 |
| 24 | S21 | 1–300 | external | 27772 | 27772 | 0 |
| 25 | S21 | 301–600 | external | 18970 | 18970 | 0 |
| 26 | S21 | 601–1000 | external | 33395 | 33395 | 0 |
| 27 | S21 | 1001–1200 | external | 26701 | 26701 | 0 |
| 28 | S21 | 1201–1550 | external | 42274 | 42274 | 0 |
| 29 | S21 | 1370–1450 | truncation-recovery | 10328 | 0 | 10328 |
| 30 | S21 | 1551–1800 | external | 14551 | 14551 | 0 |
| 31 | S21 | 1801–2150 | external | 19284 | 19284 | 0 |
| 32 | S21 | 2151–2500 | external | 25016 | 25016 | 0 |
| 33 | S21 | 2501–2800 | external | 18714 | 18714 | 0 |
| 34 | S21 | 2801–3080 | external | 22152 | 22152 | 0 |
| 35 | S22 | 1–220 | external | 12837 | 12837 | 0 |
| 36 | S22 | 221–500 | external | 17017 | 17017 | 0 |
| 37 | S22 | 501–700 | external | 29571 | 29571 | 0 |
| 38 | S22 | 701–950 | external | 18618 | 18618 | 0 |
| 39 | S22 | 951–1108 | external | 14509 | 14509 | 0 |
| 40 | S23 | 1–26 | definitions | 2659 | 2659 | 0 |
| 41 | S24 | 193–345 | governance | 8067 | 8067 | 0 |
| 42 | S24 | 439–473 | governance | 1262 | 1262 | 0 |
| 43 | S25 | 174–194 | governance | 1333 | 1333 | 0 |
| 44 | S23 | 93–107 | definitions | 3532 | 3532 | 0 |
| 45 | S26 | 1–280 | external | 17637 | 17637 | 0 |
| 46 | S26 | 281–580 | external | 14604 | 14604 | 0 |
| 47 | S26 | 581–800 | external | 38938 | 38938 | 0 |
| 48 | S26 | 801–1030 | external | 20862 | 20862 | 0 |
| 49 | S26 | 1031–1350 | external | 17727 | 17727 | 0 |
| 50 | S26 | 1351–1650 | external | 33676 | 33676 | 0 |
| 51 | S26 | 1651–1950 | external | 20578 | 20578 | 0 |
| 52 | S26 | 1951–2205 | external | 20094 | 20094 | 0 |
| 53 | S27 | 1–180 | external | 10795 | 10795 | 0 |
| 54 | S27 | 181–380 | external | 13899 | 13899 | 0 |
| 55 | S27 | 381–600 | external | 27022 | 27022 | 0 |
| 56 | S27 | 601–750 | external-whole | 37577 | 37577 | 0 |
| 57 | S27 | 751–880 | external-whole | 6954 | 6954 | 0 |
| 58 | S27 | 881–1060 | external-whole | 7458 | 7458 | 0 |
| 59 | S27 | 1061–1220 | external-whole | 7625 | 7625 | 0 |
| 60 | S27 | 1221–1370 | external-whole | 45197 | 45197 | 0 |
| 61 | S27 | 1300–1340 | truncation-recovery | 23449 | 0 | 23449 |
| 62 | S27 | 1371–1510 | external-whole | 94906 | 94906 | 0 |
| 63 | S27 | 1410–1425 | truncation-recovery | 21144 | 0 | 21144 |
| 64 | S27 | 1426–1438 | truncation-recovery | 28750 | 0 | 28750 |
| 65 | S27 | 1439–1444 | truncation-recovery | 12559 | 0 | 12559 |
| 66 | S27 | 1511–1700 | external-whole | 41395 | 41395 | 0 |
| 67 | S27 | 1564–1575 | truncation-recovery | 4232 | 0 | 4232 |
| 68 | S27 | 1561–1563 | truncation-recovery | 3109 | 0 | 3109 |
| 69 | S27 | 1701–1860 | external-whole | 11766 | 11766 | 0 |
| 70 | S27 | 1861–2100 | external-whole | 42050 | 42050 | 0 |
| 71 | S27 | 2020–2030 | truncation-recovery | 3652 | 0 | 3652 |
| 72 | S27 | 2101–2200 | external-whole | 21339 | 21339 | 0 |
| 73 | S27 | 2201–2400 | external-whole | 12746 | 12746 | 0 |
| 74 | S27 | 2401–2600 | external-whole | 16869 | 16869 | 0 |
| 75 | S27 | 2601–2780 | external-whole | 12050 | 12050 | 0 |
| 76 | S27 | 2781–3000 | external-whole | 47131 | 47131 | 0 |
| 77 | S27 | 2900–2925 | truncation-recovery | 20361 | 0 | 20361 |
| 78 | S27 | 3001–3190 | external-whole | 12647 | 12647 | 0 |
| 79 | S27 | 3191–3370 | external-whole | 13862 | 13862 | 0 |
| 80 | S27 | 3371–3559 | external-whole | 14497 | 14497 | 0 |
| 81 | S28 | 1–160 | external-whole | 22093 | 22093 | 0 |
| 82 | S28 | 161–300 | external-whole | 85999 | 85999 | 0 |
| 83 | S28 | 229–262 | truncation-recovery | 27626 | 0 | 27626 |
| 84 | S28 | 263–289 | truncation-recovery | 27233 | 0 | 27233 |
| 85 | S28 | 301–377 | external-whole | 27931 | 27931 | 0 |
| 86 | S28 | 378–467 | external-whole | 27685 | 27685 | 0 |
| 87 | S28 | 468–547 | external-whole | 27780 | 27780 | 0 |
| 88 | S28 | 548–743 | external-whole | 27948 | 27948 | 0 |
| 89 | S28 | 744–1039 | external-whole | 27948 | 27948 | 0 |
| 90 | S28 | 1040–1498 | external-whole | 27954 | 27954 | 0 |
| 91 | S28 | 1499–2033 | external-whole | 27943 | 27943 | 0 |
| 92 | S28 | 2034–2451 | external-whole | 27532 | 27532 | 0 |
| 93 | S28 | 2452–2556 | external-whole | 27769 | 27769 | 0 |
| 94 | S28 | 2557–2648 | external-whole | 19055 | 19055 | 0 |
| 95 | S29 | 1–104 | external-whole | 27590 | 27590 | 0 |
| 96 | S29 | 105–209 | external-whole | 27806 | 27806 | 0 |
| 97 | S29 | 210–279 | external-whole | 27576 | 27576 | 0 |
| 98 | S29 | 280–303 | external-whole | 27140 | 27140 | 0 |
| 99 | S29 | 304–322 | external-whole | 27410 | 27410 | 0 |
| 100 | S29 | 323–338 | external-whole | 27478 | 27478 | 0 |
| 101 | S29 | 339–350 | external-whole | 27631 | 27631 | 0 |
| 102 | S29 | 351–364 | external-whole | 27572 | 27572 | 0 |
| 103 | S29 | 365–470 | external-whole | 27449 | 27449 | 0 |
| 104 | S29 | 471–545 | external-whole | 27945 | 27945 | 0 |
| 105 | S29 | 546–662 | external-whole | 27638 | 27638 | 0 |
| 106 | S29 | 663–1020 | external-whole | 27974 | 27974 | 0 |
| 107 | S29 | 1021–1392 | external-whole | 27947 | 27947 | 0 |
| 108 | S29 | 1393–1496 | external-whole | 27470 | 27470 | 0 |
| 109 | S29 | 1497–1745 | external-whole | 27961 | 27961 | 0 |
| 110 | S29 | 1746–2203 | external-whole | 27985 | 27985 | 0 |
| 111 | S29 | 2204–2636 | external-whole | 27935 | 27935 | 0 |
| 112 | S29 | 2637–3065 | external-whole | 27936 | 27936 | 0 |
| 113 | S29 | 3066–3539 | external-whole | 27812 | 27812 | 0 |
| 114 | S29 | 3540–3967 | external-whole | 27937 | 27937 | 0 |
| 115 | S29 | 3968–4165 | external-whole | 27954 | 27954 | 0 |
| 116 | S29 | 4166–4212 | external-whole | 27935 | 27935 | 0 |
| 117 | S29 | 4213–4295 | external-whole | 24441 | 24441 | 0 |
| 118 | S30 | 1–12 | governance | 736 | 736 | 0 |
| 119 | S06 | 1–67 | verification | 4090 | 0 | 4090 |
| 120 | S09 | 1–66 | verification | 7931 | 0 | 7931 |
| 121 | S10 | 1–205 | verification | 11494 | 0 | 11494 |
| 122 | S12 | 1–25 | dependencies-audit | 2016 | 0 | 2016 |
| 123 | S16 | 1–25 | dependencies-audit | 2173 | 0 | 2173 |
| 124 | S14 | 1–25 | dependencies-audit | 2581 | 0 | 2581 |
| 125 | S17 | 1–25 | dependencies-audit | 2006 | 0 | 2006 |
| 126 | S18 | 1–25 | dependencies-audit | 2255 | 0 | 2255 |
| 127 | S20 | 1–25 | dependencies-audit | 1703 | 0 | 1703 |
| 128 | S13 | 1–25 | dependencies-audit | 2401 | 0 | 2401 |
| 129 | S15 | 1–25 | dependencies-audit | 1655 | 0 | 1655 |
| 130 | S19 | 1–25 | dependencies-audit | 1615 | 0 | 1615 |
| 131 | S11 | 1–221 | verification | 17798 | 0 | 17798 |
| 132 | S05 | 1–7 | governance-recheck | 1400 | 0 | 1400 |
| 133 | S25 | 174–194 | governance-recheck | 1333 | 0 | 1333 |
| 134 | S31 | 1–79 | helper-inspection | 3544 | 3544 | 0 |
| 135 | S12 | 26–219 | claim-audit | 11764 | 0 | 11764 |
| 136 | S16 | 26–227 | claim-audit | 13776 | 0 | 13776 |
| 137 | S14 | 26–432 | claim-audit | 27562 | 0 | 27562 |
| 138 | S24 | 193–345 | governance-audit | 8067 | 0 | 8067 |
| 139 | S24 | 439–473 | governance-audit | 1262 | 0 | 1262 |
| 140 | S27 | 2031–2034 | truncation-recovery | 1735 | 0 | 1735 |
| 141 | S21 | 1451–1470 | truncation-recovery | 1091 | 0 | 1091 |
| 142 | S32 | 1–221 | governance-expansion | 11159 | 11159 | 0 |
| 143 | S33 | 1–129 | governance-expansion | 6874 | 6874 | 0 |
| 144 | S03 | 1–75 | final-requirements-check | 5304 | 0 | 5304 |
| 145 | S04 | 1–22 | final-requirements-check | 1586 | 0 | 1586 |
| 146 | S19 | 1–94 | final-claim-audit | 6549 | 0 | 6549 |
| 147 | S31 | 1–79 | measurement-procedure-reread | 3544 | 0 | 3544 |
| 148 | S10 | 30–190 | final-claim-audit | 7262 | 0 | 7262 |
| 149 | S12 | 18–219 | final-claim-audit | 12278 | 0 | 12278 |
| 150 | S15 | 1–114 | final-claim-audit | 8368 | 0 | 8368 |
| 151 | S34 | 1–58 | governance-final-expansion | 4052 | 4052 | 0 |
| 152 | S27 | 2035–2036 | truncation-recovery | 1051 | 0 | 1051 |
| 153 | S17 | 1–200 | final-claim-audit | 15323 | 0 | 15323 |
| 154 | S09 | 1–66 | final-route-audit | 7931 | 0 | 7931 |
| 155 | S14 | 73–173 | final-claim-audit | 4262 | 0 | 4262 |
| 156 | S16 | 20–170 | final-claim-audit | 7567 | 0 | 7567 |
| 157 | S14 | 31–72 | final-claim-audit | 1806 | 0 | 1806 |
| 158 | S06 | 1–67 | final-route-audit | 4090 | 0 | 4090 |

### Searches, automatic/source-equivalent and derived exposure

The initial `rg -n` navigation matched `^##|^#|<a id=` in the three sources below. Its exact matched source lines/bytes were reconstructed from verified unchanged source; renderer prefixes, the early command's complete presentation bytes and its interleaving timestamp are unavailable. Later anchor searches used the retained search helper and their actual emitted source matches are logged. Search scan bytes are metadata about file traversal, **not** source exposure; only matched complete source lines are counted. No search content from another answer or forbidden artifact was used.

| Search | Source | Pattern | Matched inclusive lines | Exposed source UTF-8 bytes | Phase/accounting |
|---:|---|---|---|---:|---|
| 1 | S23 | `^##&#124;^#&#124;<a id=` | 3, 19–20, 27–28, 77, 79, 93–94, 97–102, 104–105 | 3558 | initial-navigation-reconstructed |
| 2 | S24 | `^##&#124;^#&#124;<a id=` | 1, 8, 17, 33, 52, 77, 129, 131, 137, 164, 172, 182, 193, 219, 224, 230, 235, 240, 274, 292, 314, 346, 368, 386, 403, 417, 427, 439, 457, 474, 487 | 827 | initial-navigation-reconstructed |
| 3 | S25 | `^##&#124;^#&#124;<a id=` | 1, 7, 174, 195 | 84 | initial-navigation-reconstructed |
| 4 | S10 | `^#&#124;^<a` | 3, 17–18, 22–23, 29–30, 47–48, 83–84, 124–125, 191–192, 200–201 | 590 | anchor-audit |
| 5 | S12 | `^#&#124;^<a` | 3, 14–15, 18, 30–31, 136–137, 204–205, 215–216 | 465 | anchor-audit |
| 6 | S16 | `^#&#124;^<a` | 3, 16–17, 20–22, 65–67, 118–120, 146–147, 171–172, 209–210, 218–219 | 765 | anchor-audit |
| 7 | S14 | `^#&#124;^<a` | 3, 13–14, 19–20, 31–32, 73–74, 137–138, 174–175, 177, 188, 230–231, 255–257, 344–346, 371, 411–412, 428–429 | 1285 | anchor-audit |
| 8 | S11 | `^#&#124;^<a` | 3, 13–14, 19–21, 27–29, 33–35, 75–76, 96–98, 121–123, 127–129, 133–135, 139–141, 170–172, 187–189, 217–218 | 1264 | anchor-audit |

The user-provided root AGENTS is the one quantified automatic source-equivalent exposure (9,508 bytes, same S01 identity). Other system/developer/task instructions and compaction summaries exist, but their complete delivered-byte totals are **UNOBSERVED**; compaction summaries are derived context, not additional canonical source authority. Instruction-discovery path listings (initial and final `find-agents --for`), `wc`/file metadata, `git rev-parse HEAD`, helper/version listings and computed hashes/counts expose metadata rather than source bodies. Their renderer/control text is not silently counted as scientific source or token telemetry.

Private source-derived inventory v1 was displayed once by `cat`: **9,142 UTF-8 bytes** of derived notes, separately from source reads. Private v2 inventory, draft bodies, audit JSON, manifests and helper source versions are retained below; their generation/write sizes are artifact bytes, not claims of model-token exposure. Source-derived draft text was generated privately and not reread through a tool as a substitute for authority. Helper code generated in tool inputs, statistical JSON outputs and failure tracebacks are derived/helper/control exposure; their complete conversation-delivery byte total is **UNOBSERVED**. The exact retained helper versions make the source-byte procedure reviewable despite that telemetry limit.

### Private procedure, omission audit and exact versions

Inventory-to-report coverage was reviewed for all **60/60** numbered items, with conditional algorithm/optimization detail explicitly scoped out of implementation claims. The separate written-claim audit found no remaining requirement omission within this task; this self-check is not independent adoption evidence. Complete mappings and scope dispositions are in `audit-v2.json`. All source hashes stayed fixed. No execution claim was upgraded by the audit.

| Private file | UTF-8/file bytes | SHA-256 |
|---|---:|---|
| `/tmp/lse-surface09-reader.py` | 849 | `01e685c9fccdda2840b35b13d6c293f3551ec588e41854d16d1f1ab2905cf0f6` |
| `/tmp/lse-surface09-reader-v1.py` | 849 | `01e685c9fccdda2840b35b13d6c293f3551ec588e41854d16d1f1ab2905cf0f6` |
| `/tmp/lse-surface09-reader-v2.py` | 870 | `9f5ab473e8fe42366daf8a7ef7f09ad4b243e84af212e57da0bd6fc977be0563` |
| `/tmp/lse-surface09-reader-v3.py` | 1023 | `22f3d943c88b5bbc9001ed08b62282407009746ceef61badef5127bd54e907b0` |
| `/tmp/lse-surface09-search-v1.py` | 622 | `ec137bf3a82f3ba608c1cf39db81aa26ff56e6ae4f50a0d499f110da1f1c409e` |
| `/tmp/lse-surface09-finalize-v1.py` | 5656 | `c1a1c45b2cef31702a915868cb19884054ae279c78bc341c1f5ad6e348b73399` |
| `/tmp/lse-surface09-finalize-v2.py` | 5884 | `bd72dbff7045ebdaff861508c7fda7a2ec1d683cb63417be5e75749695639f16` |
| `/tmp/lse-surface09-build-report-v1.py` | 13209 | `d241aaaa57b567ecf844c1adca2c6a9171322331514a3945b63cb96f65c927db` |
| `/tmp/lse-surface09-ledger.jsonl` | 45954 | `fb9a3edd6e17179fa0616f7d81c1af40e14abb458fb1bb7ec7d24df46a1c862f` |
| `/tmp/lse-surface09-search-ledger.jsonl` | 2079 | `4ce4a37f1ed784e125be5b290e1f077de92b6a5fa5e88089e1f9988ecbf461b8` |
| `/tmp/lse-surface09-initial-search-reconstruction-v1.json` | 1989 | `2cdf9ce330707dce1445659b10789b4d7960f8a43a852d294164d923c027e4e5` |
| `/tmp/lse-surface09-inventory-v1.md` | 9142 | `297b9e050a904b8c7d828f49d6be7b48874273c3ba18270f137c68519b525ea5` |
| `/tmp/lse-surface09-inventory-additions-v2.md` | 9966 | `d59562468f0e5016b1ac0bc2b17ba3cf5b52346ceaac0cf808038111190b7c0c` |
| `/tmp/lse-surface09-inventory-v2.md` | 19109 | `7f31fa3d72e5a47af315047ae649a1b8db3caaf7256462f975257fcc2020dd77` |
| `/tmp/lse-surface09-body-v1.md` | 46117 | `21ba62c35084e4bc84fb6dc21ee0e90e47b588094970572a4f167b4496d2e337` |
| `/tmp/lse-surface09-body-v2.md` | 46447 | `8b38a4b7a03e16614284ed07262e955f9266e90bc035b3488ea8133f7425a1f0` |
| `/tmp/lse-surface09-audit-v1.json` | 11447 | `1641d4b4b6825eed43d01777da2abef5694aabb1e493d5637756d83613cd76ca` |
| `/tmp/lse-surface09-audit-v2.json` | 11732 | `bdf083f578c588f11c2676a0800f0dc80cc52069a984409b130a4a38376ee1ee` |
| `/tmp/lse-surface09-context-selection-v2.json` | 83681 | `21046e5b1ec9c2e52da19dffaedcec7e6493f7654bcdcfe11e8af8ecdce5b623` |
| `/tmp/lse-surface09-context-report-v2.json` | 132094 | `831420d7cd6273c500cb1e516c077a46179a774c9fc3101c482a094f6207e4ee` |
| `/tmp/lse-surface09-measurement-summary-v1.json` | 9494 | `e1ef4068df0df35e12a9c4071c2c2de8fb35812cfb607f143a8a4bb88a3b07c5` |

The original reader and v1 are identical; v2 adds safe end clipping; v3 adds the whole-line output budget. Finalize v1 and v2 are both retained; v2 adds search exposure to the context selection. Context-selection/report v2 are the final search-inclusive manifests. Private draft v2 adds the explicit legacy Celsius/LSE-Kelvin alias prohibition before completion. The owned report is written only after this audit and is then immutable.

**Disposition: READY — static requirements report only. Actual run inputs remain missing; production/test/workflow evidence remains NOT RUN.**
