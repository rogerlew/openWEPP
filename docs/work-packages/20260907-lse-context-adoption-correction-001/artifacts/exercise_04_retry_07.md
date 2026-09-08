# Exercise 04 retry 07 — independent frozen-litter closure reconstruction

Static: requirements reconstruction from canonical accepted-primitive authority. No primitive bundle, configuration, initial/ending owner set, receipt stream, executable, or run was supplied. I did not execute a production or diagnostic physical solver, tests, comparator, or oracle. Consequently this report supplies a reconstruction procedure and required evidence, not numerical mass/energy closure, implementation correctness, successful runtime adoption, or production qualification. The source freeze was supplied as commit `9baf9ccb7bf7d2d53e24acbf1d1151d3d599e311`; I used the fixed working-tree sources and recorded their full SHA-256 identities without reading Git history. Final internal hash comparison confirmed every read source retained its recorded hash.

The applicable candidate is LSE contract v32, with selective reading authorized only for the package's bounded exercises. The neutral handoff does not promote it. Canonical process authority remains binding; package artifacts are evidence. This is my first completed answer, independently reconstructed without another reader's answer or parent validation. The only repository write is this assigned report. Root and work-package instructions were discovered with `tools/agents/find-agents --for <assigned-report>` before writing. No authority or runtime source was changed.

## Scope, anchors, and required inputs

The target is `OPENWEPP_SNOW_FREE_LSE_V3` on an explicitly snow-free `forest_litter` tile. V3 imports V2's canopy/radiation/turbulence/soil/water/solver/closure/support/rollback requirements and specializes litter liquid/ice vapor and a bounded post-solve phase operator. This does not admit frozen or thawing soil, bare-mineral frozen material, a ponded frozen model, represented snow, or a terminal-snow receiver as V3 litter. Beginning and ending regime evidence, complete surface class and owner identity, positive finite wind in the neutral domain, finite geometry, and complete configuration are prerequisites. Temperature cannot infer snow state or initialize ice. Litter blocks direct mineral-soil evaporation and upward capillary supply during this interval. A represented-snow lower boundary is an OFE/lane interface directly between bottom snow and top soil; inactive litter bytes are retained and are not a parallel heat boundary.

The retained litter anchor is R-156, `references/vendorable/gmd-10-1621-2017-isba-meb-litter.pdf`, Appendix A, A1–A4 and A7–A14, SHA-256 `2a8c14d912651457bf9205a4a963b78dd12f1aa7f243bccb025e4b81ce99716d`. Canonical litter-phase lines 35–72 extract its adjudication together with SURFEX v8 `isba_meb.F90.source.html` generated lines 1992–2159 (`0a300739b5dc660b61d29db144dd92f886e8fdf9934eac8facc022585992087a`), `isba_fluxes_meb.F90.source.html` 388–407 (`e0378bc89ee0d52cffe14841aac56de1d8d379edf18ad29f24cfdb9ea0dfdbbc`), and `ini_csts.F90.source.html` 146–157 (`f39840df4d851efc70044f9e3ad62822371ed743c3c3a8055a4c940e2f86d73a`). These are contract-recorded anchors, not a claim that I independently reread the retained PDF/F90 or adjudicated licensing/provenance. The literal printed A4 sign is not copied: the admitted sign is freeze minus melt. Surface-energy lines 33–38 separately identify CLM5, ISBA-MEB, Napoly, FSM2 and arbitrary-level ORCHIDEE longwave sources. Accepted scientific equations, rather than legacy output agreement, govern this reconstruction.

The following must be captured from the actual accepted transaction, with exact bits, units, full semantic keys, source/receiver owner, interval, and receipt origin. None is supplied here:

| Required material | Authority and role |
|---|---|
| Complete immutable beginning and accepted ending owners; model/schema/configuration/contract/state digests; restart/checkpoint version and full payloads; parent/segment/slab/predecessor IDs | LSE interface, litter-phase INV-140/147, soil/surface custody, CoupledTime, SurfaceLiquid; validates supported model and chronology before arithmetic |
| Configured OFE sequence, positive areas, topology, tile fractions, complete OFE/tile/surface/source/layer keys, vegetation mapping, occupancy rank, component area and thermal-node order | Surface-energy, solve-boundary, INV-158 and external owners; no lexicographic interpretation of opaque OFE IDs; local arithmetic and area conversion are auditable |
| Exact accepted half-open tick support, duration bits, participant set, support-admissibility receipt and policies; child chronology and accepted event receipts where applicable | Terminal-support and CoupledTime; one duration for all participants, no gap, overlap, local duration substitution, or rejected-attempt reduction |
| Litter beginning/post-vapor/raw-phase/retained/final liquid and ice; dry heat capacity, thickness, liquid capacity; surface enthalpy high/carry and thermal warm starts | LSE V3 + SurfaceLiquid V2 water owner; phase/spill and exact-energy reconstruction |
| Beginning and accepted trial temperatures for all actual canopy/ground/soil nodes; soil thickness/conductivity/capacity; current radiative interface/component primitives; air density/heat capacity, humidity, pressure, aerodynamic operands | Surface-energy + soil-coupling + Vegetation; independent radiation, turbulent and conduction reconstruction without using a producer residual |
| Separate liquid/ice raw and finalized signed vapor, phase-specific saturation and latent/sensible operands, source-specific requests/authorizations/final uses, identity and cap branch | Litter-phase INV-141/142 and water-vapor; accepted latent-function value and provenance must be present, not guessed from a snow or liquid constant |
| Original phase receipt, exact raw phase operands, typed spill companion, native-row join, ordinary finalized rows and unchanged resource provenance | INV-143–148/156/157; preserves exact-once phase, spill and heterogeneous resource custody |
| Each accepted current liquid parcel with mass, temperature, specific enthalpy, energy, exact full support, origin, routing key and upstream outlet lineage; canopy throughfall, first and second drainage and stemflow separately | Water-vapor, Vegetation V8, SurfaceLiquid; producer-admitted rain parcel is consumed as authoritative, not recalculated by LSE |
| Stateful WB14 owner parameters and beginning cumulative state; complete ingress schedule/partition/queue records; retained/infiltrated/routed/outlet receipts; original prepared soil owner and all ordered thermal credits | SC-WATBAL, SC-SURFACELIQUID, INV-150/151/155; independent water and advective heat partition, sequential soil composition |
| All accepted physical energy operands before high-term rounding; normalized dyadic carries, receipt ordinals and accepted support; final complete owner candidates and publication/outbox/restart lineage | Exact surface/soil custody and CoupledTime; exact reconstruction and atomic ending-set equality |

Hydrology exclusively owns water mass, including litter liquid and liquid-water-equivalent ice. LSE owns the one sensible-energy surface coordinate, with frozen V3/surface-V2 fields being bit-identical high mirrors on the exact successor path. Soil thermal owns the ordered soil temperatures/enthalpies. Vegetation owns canopy processes. Coupled time owns accepted support and atomic publication chronology. WB14 remains hydrology's sole ingress infiltration partition. SC-EVAP's daily WB17 is not a second owner of this subdaily litter vapor transaction.

Mass is `kg m^-2 tile-ground` locally; LSE sensible energy is `J m^-2 tile-ground`; rates are per second. A tile contribution becomes OFE-ground by exactly one multiplication by `f_t`. The stand-ground basis here is one OFE's ground, never a routed hillslope. A cross-OFE parcel conversion uses the actual source/destination area ratio on both mass and energy once. Water depth conversion uses `rho_w` with its stated metre/millimetre basis. Receipt key/order and exact identity checks precede tolerance checks.

## Ordered physical reconstruction

1. **Admit identity, regime and support.** Validate closed serialization, exact model/configuration/state/transaction, topology and sole owners, finite operands and physical domain before solve bounds. For an admitted V11 physical LSE invocation, the sealed support receipt binds parent, segment/slab, absolute support, requested ticks, the shared duration bits, beginning LSE/soil identities, policies and the exact minimum. The released covered-forest policy is `60_000_000_000 ns`: below it reject before Newton; exact zero is a custody/event transition without physical solve. Structural nanosecond chronology is not physical admission. Other profiles remain separately profiled/non-admitted as terminal-support explicitly states; importing a 60-second V3 floor is not evidence of profile or production qualification. A larger support must still converge; stable ordinary supports must accept substantially larger steps. Coupled-time nanoseconds use exact tick arithmetic and the one prescribed tick-to-binary64-seconds conversion; no owner recalculates a nominal duration.

2. **Take immutable water availability before ingress, solve potential, authorize once, rebuild fixed-final.** Current rain/runon/canopy release/overflow cannot enlarge beginning stores or lower a request. Roots and ground compete in one same-snapshot protocol, with complete identities and `0 <= F <= A <= D`. Equality `cap_rate <= q_law` selects the cap branch with zero generalized derivative; law flux is independently evaluated. Potential/final systems use current canopy, ground, shared air, hydraulics and soil temperatures together. Only final accepted state produces owner candidates. Never install authorization as use, donate canopy/root supply to ground, apply an agricultural PMET remainder, continue a potential candidate as final state, or authorize a second time.

3. **Reconstruct current radiation and turbulent exchange locally.** Ground VIS/NIR direct/diffuse shortwave is the unchanged whole-column two-stream terminal flux with actual ground albedo once; reflected ground radiation traverses all overlying occupancies. Longwave has unit emissivity and no reflection. For each occupancy `tau_i = exp(-0.8*Omega_i*(LAI_i+SAI_i))`, current component-weighted emission supplies the downward/upward recurrences; ground emits `sigma*T_s^4`. Each positive-area component gets `w_j*(1-tau_i)*(Ldn_i+Lup_(i+1)) - 2*w_j*(1-tau_i)*sigma*T_i,j^4`; zero total area gives `tau_i=1` and zero component longwave. Ground net is `Ldn_n-sigma*T_s^4`. A bulk canopy temperature, stale ground state, or prescribed upward ground flux cannot substitute.

   Sensible heat is positive outward: `H_s=rho_a*c_p*(T_s-T_recipient)/r_h`. Signed vapor is positive outward. Open tiles use reference air and the exact neutral log resistance `ln(z_ref/z0m)*ln(z_ref/z0h)/(kappa^2*u_ref)` for heat, with `z0q` for vapor; `kappa=0.4`, strictly positive wind and valid logarithmic geometry. Covered tiles use one shared canopy-air `(T_c,q_c)` and the admitted neutral canopy/ground resistances, with all canopy terms and exactly one ground term balancing the one reference-air exchange. A ground contribution is not repeated for each occupancy. No hidden calm-wind floor, attenuation substitute, nonneutral branch or separate occupancy air node is available.

4. **Keep the complete fixed-final nonlinear system phase-free.** No freezing/melting, fusion source, phase-adjusted capacity, temperature or availability may influence its Newton residual/Jacobian, branch, authorization or convergence witness. Beginning `W_l,0` and `W_i,0` are distinct immutable resources. The V3 constitutive vapor law deliberately differs from the older single-liquid humidity blend:

   ```text
   p_i = 0 if W_l,0 + W_i,0 == 0; otherwise W_i,0/(W_l,0+W_i,0)
   h_ul = 0.5*(1-cos(pi*W_l,0/W_l,max))
   h_ui = 0.5*(1-cos(pi*W_i,0/W_i,max))
   v_l,raw = (1-p_i)*rho_a*(h_ul*q_sat_liquid(T_l,p)-q_recipient)/r_l-c
   v_i,raw = p_i*rho_a*(h_ui*q_sat_liquid(T_l,p)-q_recipient)/r_l-c
   Q_v,l = v_l*[C_w*(T_l-T_ref)+L_v(T_l)]
   Q_v,i = v_i*[C_i*(T_l-T_ref)+L_s(T_l)]
   ```

   Both components use saturation over **liquid water**, even ice. Positive finalized evaporation/sublimation is bounded only by its own beginning pool. Negative liquid condensation/ice deposition has no withdrawal availability cap and credits only its named phase. Retain signed components separately through custody and reconstruction, then aggregate for the air-side consumer. `L_v(T)=2.501e6-2369*(T-T_ref) J kg^-1`; the accepted `L_s(T)` operand and its constitutive provenance are required separately. I have not invented a sublimation coefficient or equated snow fusion and litter fusion. Latent-only energy, absolute-value aggregation, total-store caps and cross-phase debit are invalid.

   After acceptance, reconstruct `W_l,*=W_l,0-dt*v_l`, `W_i,*=W_i,0-dt*v_i` and `U_*=(C_dry+C_w*W_l,*+C_i*W_i,*)(T_*-T_ref)`. Independently reconstruct the phase-free surface energy from signed radiation minus outward sensible, both complete vapor-energy terms and the one downward ground exchange. This must agree with the accepted post-vapor sensible-energy state; a producer's own residual is not an operand.

5. **Reconstruct the sole soil boundary and soil storage.** Litter has `C_dry=dz_l*rho_ld*c_ld` and `lambda_l=0.1+0.03*W_l/(rho_w*dz_l)` with the admitted immutable water operand. Every soil layer has positive finite thickness, conductivity and areal capacity. Interface conductance is `g_s1=2/(dz_s/lambda_s+dz_1/lambda_1)` and `G_s1=g_s1*(T_s-T_1)`; internal interfaces use the corresponding two layers. All G are downward. Crank–Nicolson uses `bar(G)=0.5*(G_begin+G_end)`: each layer's `C_k*(T_k,end-T_k,begin)/dt` is its entering minus leaving mean flux. The lower boundary is exactly zero; for one soil node its entire thermal rate is `bar(G_s1)`. Surface loses the same accepted top exchange once, soil gains it once. Do not independently calculate and apply a second G.

   Finite-capacity beginning surface temperature comes from beginning owned U and water/capacity, never the caller warm start. The separate `equilibrium_zero` branch is eligible only for exact zero dry capacity, water and energy and has no physical beginning surface T: its current algebraic surface trial appears at both CN endpoints. This is not a zero-capacity escape for frozen litter. Frozen/thawing soil and duplicate LSE-owned soil temperature remain unsupported.

6. **Apply the bounded kinetic phase operator once, after vapor and before ingress.** The selected litter constants are `T_ref=273.15 K`, `rho_w=1000 kg m^-3`, `rho_i=920 kg m^-3`, `C_w=4218`, `C_i=2106 J kg^-1 K^-1`, `L_f=333700 J kg^-1`, `tau_ice=3300 s`, `W_i,max=0.85*rho_w*dz_l`. The capacity is water-equivalent: using `rho_i` there, or the mineral-soil `917` ice density here, is rejected. Require `W_l,*>=0`, `0<=W_i,*<=W_i,max`, positive finite thickness/support and positive finite ending capacity.

   ```text
   M_warm = rho_i*C_i*dz_l*max(T_*-T_ref,0)/L_f
   M_cold = rho_i*C_i*dz_l*max(T_ref-T_*,0)/L_f
   m_melt = min(W_i,*, (dt/tau_ice)*min(M_warm,W_i,*))
   m_freeze = min(W_l,*, W_i,max-W_i,*, (dt/tau_ice)*min(M_cold,W_l,*))
   m_phase = m_freeze-m_melt
   W_raw = W_l,*-m_freeze+m_melt
   W_i,end = W_i,*+m_freeze-m_melt
   U_raw = U_*+L_f*m_phase
   C_raw = C_dry+C_w*W_raw+C_i*W_i,end
   T_raw = T_ref+U_raw/C_raw
   ```

   Outer donor/capacity bounds apply below, at and above `tau_ice`; the operator creates no maximum step requirement. At exact T_ref both transfers are zero. Check equal liquid/ice transfers, total water invariance and phase-only `H_phase=U-L_f*W_i` invariance. Ending T uses **ending** dry/liquid/ice capacity. Reject the printed wrong sign, instantaneous equilibrium, freeze-only logic, pre-phase-capacity temperature increment, tiny-ice deletion, `xwgmin` regularization or soil compensation. Ending phase temperature is the next-support warm start; it cannot trigger a same-support flux/phase/authorization/Newton re-solve.

7. **Split melt-created liquid overflow conservatively (INV-156/C-011).** The raw phase receipt remains unchanged. If `W_raw<=W_l,max`, spill is exact positive zero and retained equals raw. Otherwise use the checked binary64 sequence:

   ```text
   m_spill,tile = W_raw-W_l,max
   h_spill = C_w*(T_raw-T_ref)
   Q_spill,tile = m_spill,tile*h_spill
   W_retained = W_raw-m_spill,tile
   U_retained = U_raw-Q_spill,tile
   C_retained = C_dry+C_w*W_retained+C_i*W_i,end
   T_retained = T_ref+U_retained/C_retained
   ```

   All operations must be finite; positive spill, nonnegative within-capacity retained liquid and positive retained capacity must pass. The second subtraction is the authoritative remainder, not setting water to capacity. Independently reconstruct raw = retained + spill mass and sensible energy in the declared order. Preserve original phase hash, config/owner/transaction/full child support/key/raw/capacity/temperature/enthalpy identities. Debit one named negative `LitterPhaseCapacitySpillEnergy` operand after phase-free and fusion energy. Internally create one `LitterPhaseOverflow` parcel with `m_ofe=f_t*m_tile`, `Q_ofe=m_ofe*h_spill` and the complete child `[0,dt)` support. It is not caller-supplied condensation, ice, fusion-as-runoff, a rainfall-hour reassignment, or a finalized-use row. It enters ordinary WB14 once; any returned retained portion comes through a new ordinary retained-ingress receipt, never by cancelling the original spill debit.

8. **Join heterogeneous finalized resources once (INV-157/C-012).** Match each already consumed native phase vapor row by transaction, support, full source key and checked fraction/support aggregation. Finalized row F must equal receipt-derived F exactly and obey `0<=F<=A<=D`; A need not equal F. Exclude those rows from further application. All unmatched surface rows are the complete ordinary set and retain original request/authorization/final-use identities and bounds. Fold in complete `GroundWaterKey` order and debit checked `F/f_t` once from the accepted phase-adjusted V2 liquid owner. Empty ordinary set is the identity. Every finalized row is accounted exactly once as soil, native phase, or ordinary surface use. Preserve ice, phase/spill receipts, enthalpy high/carry and native LSE ending bytes; ordinary debit adds no new latent/fusion/sensible operand. No wholesale resource substitution, reconstructed legacy beginning, replayed phase row, or second ingress call is allowed.

9. **Apply final current ingress through the real stateful WB14 owner.** Accepted parcels are liquid, with `h_l=C_w*(T-T_ref)` and `Q=m*h_l`; zero mass has zero energy and no temperature. Runon uses the accepted upstream outlet temperature and enthalpy, never air/soil/freezing/downstream T. This accepted-parcel reconstruction does not rederive the rain-temperature provider. Canopy throughfall, first drainage, second drainage and stemflow use their accepted wet-surface temperature and individual mass/lineage. Covered and open precipitation ownership is selected from each explicit boundary-source mapping, not inferred from the litter label; do not admit raw rain and covered release twice. Initial drainage precedes wet evaluation; condensation can cause second drainage. Final fixed-cap column routing rebuilds from original beginning stores using final upstream release, never the potential release.

   Construct one complete ingress schedule/partition on accepted support and the original stateful Green–Ampt/WB14 parameters and cumulative beginning. Do not reset per parcel, distribute a daily result, use a proxy infiltration formula, or rerun WB14 for the spill. At each admitted ingress subinterval, `X=sum(x_p)`, `Q=sum(Q_p)`, positive X gives `h_mix=Q/X`; zero X has zero energy. WB14 gives infiltration I and excess E with `I+E=X`; `Q_inf=I*h_mix`, `Q_excess=Q-Q_inf` uses the canonical remainder. Provenance partitions follow their prescribed fractions and final remainder, not an arbitrary new arithmetic order. Retained excess is limited by actual tile capacity; only its exact named retained-ingress energy credit updates surface U. Infiltration energy credits top soil. Remaining routed/outlet runoff carries the accepted mass and enthalpy. Litter ice cannot infiltrate, drain, run off, supply WB14, enter `frozwt`, or alter soil phase.

   Retain the explicit SurfaceLiquid representational exception: when a positive raw retained mass is no larger than `1e-14 + 64*epsilon*(abs(f*capacity)+abs(f*W)+abs(E)) kg m^-2 OFE-ground`, choose zero retention, preserve store bits and route the entire associated mass/energy; do not silently discard a credit. This is not a general enthalpy tolerance. Native retention replaces, rather than duplicates, the old depression store. Topological queues preserve source/support/digest lineage and apply area conversion to mass and energy once before downstream partition.

10. **Seal exact surface/soil energy custody, then complete the envelope.** Independently check local mass, phase, radiation, vapor, surface energy, every soil layer and all advective parcels; only then weight to OFE ground. The internal surface/soil G pair, phase conversion and routed intermediate transfer cannot appear as unexplained whole-envelope gains/losses. Reconstruct outgoing outlet energy/water, remaining liquid/ice/soil storage and every canopy/root/other owner contribution separately. This enclosing check must use produced primitive records, not copy the producer's residual or compare two aggregates computed by the same alias. Every accepted child ending is the next beginning; no trial/rejected map enters accepted reductions. Commit complete owners, receipts, cursor/checkpoint and parent publication only after all joins pass.

## Exact arithmetic, permitted shortcuts, and admission checks

**Exact carry is custody, not extra physical forcing.** Soil INV-150 and surface INV-151 require `E=exact(H_hi)+R` and `U=exact(U_hi)+R_U`. Decode each finite accepted binary64 operand as its exact dyadic value, accumulate with arbitrary-precision integer arithmetic in the declared receipt set, round the exact total once to binary64 nearest-even, then retain the exact normalized remainder. Nonzero carries have sign ±1, odd nonzero lowercase hexadecimal magnitude and exponent; zero has the sole `(0,"0",0)` form. High-term signed-zero rules and finite subnormals remain intact. Nonfinite high/operand, noncanonical carry, overflow or exact reconstruction failure rejects. There is no tolerance, forced ULP, `nextafter`, compensated float sum, zero snap or residual-as-credit substitute. Carry is never injected into the temperature or flux equations; accepted physical high/mirror rules remain unchanged.

Soil receipts include the exact accepted internal, top and infiltration credits by layer, basis, source, support and ordinal. Surface receipts include each accepted phase-free radiation/sensible/separate-vapor/ground operand, fusion, the additional negative spill operand, and retained ingress. For retained surface energy, join the complete destination key, use canonical receipt order, reconstruct the accepted OFE-ground energy grouping, perform the specified checked binary64 division by tile fraction, then decode that value exactly. Dividing an exact rational first changes the admitted operand and is not equivalent. High mirrors in LSE V3 and SurfaceLiquid V2 must match bit-for-bit. Additive exact-owner adoption copies the existing physical high bits and zero carry; it does not reconstruct a high term from T. Missing exact-successor restart material or downgrade is invalid even if carry happens to be zero.

**Private child composition and finalization.** INV-153 retains persistent parent predecessor markers on strictly interior positive physical children while private physical/carry/receipt chain state advances; the final child stamps the parent successor once. A child receipt beginning marker must join its actual predecessor. INV-155 allows a typed read-only unpublished soil view only after original prepared-owner and previous private seal/support/chain validation. It is not a new owner, accepted receipt, installable candidate or restart wire. Final soil composition starts from the original prepared owner and includes all ordered child operands, exact ending and layer credits once, returning one atomic bundle. Do not normalize predecessor ownership to a convenient intermediate or increment a parent sequence at every child.

**Canonical key order.** INV-158 joins bare envelope bytes to the complete configured OFE order before declaring them installable. OFE IDs are opaque; use configured OFE position, then the relevant full within-OFE semantic key and operand kind/ordinal. A schema/digest-valid envelope with arbitrary order is not sufficient. Scalar sorted-receipt conventions only apply where the owning formula explicitly says so and do not replace semantic resource or topology order.

**Validation reuse is narrowly revision-bound (INV-159).** A private immutable in-memory proof can reuse completed full semantic validation for the exact unchanged revision and all schema/model/configuration/state/transaction/predecessor/support/owner/history bindings. A proof cannot be minted from a digest, unchecked public constructor or mutable object. Mutation, lineage normalization, owner/configuration replacement or different support invalidates it; a fresh revision needs fresh canonical digest and full validation. Cached serialized bytes are derived evidence required to equal fresh serialization. Restart/checkpoint/external/untrusted/durable boundaries fully parse, canonically reconstruct, validate complete prefix and replay chronology; proofs are not serialized/restored. Covered-parent forcing proof reuse additionally needs the same immutable parent generation/configuration/topology and pointer-identical forcing input after its first full validation. Resident owner proofs remain distinct from V8 validation and can be consumed only after the V8/ingress schedule obligations; proving the canopy does not authenticate the resident water owner. CoupledTime's trusted support append and Vegetation INV-134 have corresponding exact incarnation/revision/tail/owner bindings, never a blanket hash-skip license.

**Final reseal is identity-only (INV-160).** A single-use move-only capability follows complete validation of a provisional snow-free physical ending. Independent final slab admission may change only its accepted slab digest while preserving the same clock revision, parent/segment/ordinal/support, beginning owner and ledger identities, and every non-slab input. The final physical payload must be byte-identical. Consume once without another LSE, phase, WB14, soil, vegetation or BGC solve; provisional publication count is zero and final publication occurs once through the parent. Reuse on changed physical input or restart must fail typed. It cannot establish that an unvalidated physical result was correct.

**Pending covered map origin is checked even when reading snow-free boundaries (INV-161).** Regime and map role are independent. The native represented-snow map retains inactive litter and requires the expected resident owners and Stage-3 topology, with no active litter tile; the ordinary map retains its full LSE/surface/WB14/soil prefix. A post-initial physical map yields a non-cloneable pending result only after physical/discrete custody checks. Outer nonclosure can consume its history without a dependent-closure error; outer closure plus failed dependent closure returns adaptive refinement without constructing history. If both close, consume that same pending physical prefix once as FinalAccepted, then complete dependent owners and the private joint envelope. No extra final physical map, promotion of a completed nonfinal map, or per-map publication is admitted. Current snow authority preselects the regime from beginning owners; a snow-free state does not invoke a covered snow solver. The old long retry/recovery paths are not an alternative to the current canonical solver/adaptive response.

**Solver shortcuts retain complete eligibility and checks.** For exact FullSupply, every positive authorization must match request identity and amount and every canonical zero request retain exact zero and identity. Only coordinates may seed the final pass from the accepted potential solution; completely reevaluate fixed-final physics from immutable beginning owners and exact caps. Iteration-zero acceptance requires every residual, domain, bound, active-branch inequality, D/A/F and owner/identity check, with zero steps/backtracking and no Jacobian. Never copy potential fluxes or candidates. V10 nonpositive-assimilation partial positive root supply is typed unsupported. Exact zero absorbed PAR has `Ag=0`, `An=-Rd`, `gs=g0`, `beta=1` and direct diffusion; the daytime beta-vulnerability demand equation is unavailable for `An<=0`. Positive low light first uses the historical bracket; only its unbracketed `F(ca)<0` case can use the constructive dark bound, with its own gas-domain and root checks. No epsilon around compensation or radiation floor is permitted.

The inactive liquid-vapor temperature anchor is `T-max(T_canopy,273.15 K)=0` only for zero physical component area or the already eligible inactive wet row. Dry stem retains `T_stem-T_canopy`. Zero area contributes zero physical flux/ledger terms. A V10 nonpositive-assimilation potential wet row is eligible only on its liquid store-cap branch, preliminary store rate within water residual tolerance and unanchored physical wet energy already within energy tolerance; not condensation, constitutive-law wet flow, positive assimilation, V1 or fixed-final scaling. Liquid/energy/longwave state remains fully evaluated. Diagonal unit scaling is exclusive to the uncapped active V10 nonpositive-assimilation potential solve; not fixed-final or other models/branches.

All covered Jacobians retain the canonical perturbation and minus-then-plus evaluation. Centered differences require two valid probes. Only an exact admitted closed-bound case with a valid current iterate and exactly one invalid canonical probe uses its unique inward one-sided difference; invalid current or two invalid probes rejects, with no delta shrink or clamp. The full domain-valid Newton no-update witness is tried first. Only when passing current residuals have a domain-invalid full trial or excessive governed full-trial step may the first domain-valid halved trial witness a no-update acceptance. It must be completely evaluated and all governed prospective step norms pass; accept the unchanged current state, never the witness trial. Failure cannot skip to a smaller witness; actual updates still require strict residual decrease. Derived ci step is diagnostic, not an invented acceptance threshold.

The physical solve keeps ordered residuals and component unknowns, deterministic partial-pivot LU (lowest row on equal pivots), canonical `sqrt(epsilon)*max(abs(x),unit_scale)` perturbations, the existing singular threshold `64*epsilon*matrix_inf_norm`, at most 50 completed updates and factors `2^-b` through `b=20`. Energy residual threshold is `1e-6 W m^-2 + 1e-10*max(1,sum(abs(component operands)))`; water/vapor residual is `1e-12 kg m^-2 s^-1 + 1e-9*scale`. Governed steps are at most `1e-8 K`, `1e-12 kg kg^-1`, `1e-7 mm` hydraulic and `1e-10` beta. General numerical bounds include `200<=T<=350 K` and `0<=q<=0.1`, after the physical domain, with stricter liquid-vapor canopy domains retained. Exact identity, phase, receipt, basis and D/A/F checks cannot be repaired by these tolerances. Controller direct-versus-composed truncation bounds, event-error tolerances, nonlinear residual thresholds and exact-dyadic custody are different predicates. The legacy v1 symbolic integrated closure tolerances are not permission to invent coefficients or ignore later admitted component and exact-custody rules.

**One-ULP publication normalization is not a general shortcut.** INV-130 alone admits the exact first upward binary64 neighbor `0x4071126666666667` of reference `0x4071126666666666` to become exact T_ref before covered-canopy liquid ledger/release or mass-weighted Stage-3 terminal-liquid publication, persisting the temperature and zero h together. No below-reference value, second upward neighbor, general litter temperature, storage sum, residual, phase amount or exact credit is covered. All other domain rules remain.

SurfaceLiquid's authorization roundoff repair is also scoped: common beginning-store proportional allocation must satisfy both OFE and tile-basis totals. Only its bounded overshoot rule permits one symmetric common multiplier, selected in the prescribed bounded binary64 search while preserving positive rows. It cannot redistribute to a selected row, turn unused A into F, repair a physical invalidity, or excuse double tile weighting. The full external contract was read; these are protocol qualifications, not permission to use approximate closure.

## Failure, evidence, and actual limits

Apply first-error order: malformed serialization; model/configuration/state/transaction identity; missing/duplicate topology/owner; nonfinite operand; unsupported regime/domain; constitutive domain; request/authorization identity or bound; pivot/backtracking/iteration/accepted-step failures; component closure; control-volume closure; cross-owner join. Failure diagnostics bind model/config/state/transaction/OFE/tile/occupancy/pass/solve, ordered residuals, bounds/caps, iteration/backtracking, steps and rollback hashes. No failed iterate or partial candidate is usable.

V3 identity/domain/migration failures use `LSEB-E-045`, phase vapor/enthalpy `046`, phase/fusion/ending-capacity and spill `047`, chronology/receipt/restart/rollback `048`; exact soil custody is `049`, exact surface custody `050`, with applicable SurfaceLiquid/WATBAL/CoupledTime typed failures preserved. Failure at later ingress, soil composition, final replay, vegetation/BGC join, restart or publication restores all beginning owner, receipt, cursor, checkpoint and enclosing transaction bytes. Typed rejection is not a numerical fallback or an accepted approximate result.

The independent physical checks must distinguish: empty/all-liquid/all-ice/mixed litter; freezing/melting/exact T_ref; condensation/deposition and separately capped evaporation/sublimation; ice capacity; supports below/at/above tau and the physical support floor; radiation/ground sign reversals; local versus weighted OFE versus routed-area amounts; retained/infiltrated/routed/outlet energy; zero, exact capacity and melt-created spill; ordinary/native heterogeneous rows and no native replay; positive sub-ULP surface/soil credits with nonzero exact carry; child composition and fresh restart; wrong sign/density/fusion/kinetics/saturation/capacity; latent-only or duplicate vapor; current-ingress donation; ice entering WB14; stale snow operands; wrong key/order/basis/duration/receipt; unknown/missing/corrupt restart; and late failure with complete rollback. A positive witness must make competing wrong formulas numerically distinguishable. Digest equality, schema validity, source scanning and self-consistency alone do not establish physical closure.

INV-149 explicitly requires the canonical p61 and native-forest real consumers (`tests/integration/erosion_single_ofe_p61_sediment.rs`, `tests/integration/dff_ws1_native_forest_cli.rs`) through the production selector, persisting/reloading successor state and proving primitive liquid/ice, fusion, vapor, WB14 and whole-envelope closure. I read their contract obligations, not the forbidden test files and did not run them. Actual implementation/consumer acceptance would need complete produced primitive bundles and independent reconstruction plus real closure/magnitude evidence, with relevant A0/A1/A3 requirements. Sparse empirical data neither supplies nor waives these checks. This static exercise is not that runtime gate and does not resume, promote, or qualify production.

The applicable physical capture rule is litter-phase line 17 and its primitive/real-consumer obligations, together with P-001–004 and exact receipt custody. Replay experiment capture is explicitly separate. I read qualification in full to verify this boundary; I do not claim an EXP-R identity/capture check, historical retention test, replay coverage/parity, executable reconstruction, experimental timing/memory/scaling/teardown result, or source provenance adjudication. Therefore qualification's conditional EXP-R protocol/input/kickoff/handoff expansion is not triggered by this accepted-primitive task. The owner-paused experiment and retained production HOLD remain unchanged. Likewise, accepted authoritative rain parcels do not trigger provider-equation rederivation; checking that provider's actual physics would require the complete SC-SNOWFREEZE authority. Reviewing replay equivalence would require dependency-replay plus its complete frozen protocol; implementation would expand the evaluator/source and full applicable solver. This report makes none of those unperformed claims.

Reading expansion was necessary, not eliminated by the split: litter-phase requires complete water-vapor, soil-coupling and surface-custody and whole SurfaceLiquid; water/energy requires surface-energy and whole WATBAL; surface/soil regime selection includes all terminal-support and therefore whole SnowEnergy and CoupledTime; covered coupling requires whole Vegetation. Receipt origin/candidate composition expands soil-custody and map-custody. I additionally read the complete nonlinear chapter for the concrete branch/shortcut qualifications discussed above, qualification for physical-versus-experimental capture scope, and applicable governance. I did not read binding-index/history, old monolith/Git history, archived candidates, intake/predictions/reviews/rubric/old exercise reports/context-usability/final-disposition/gate results, parent validation, another reader answer, or another scratch directory. The source/inference distinction above is deliberate: concrete required inputs remain absent, so an executable closure verdict is still unavailable.

## Measurement method and complete source-request ledger

All counts below are UTF-8 source bytes including line endings, not displayed token counts. Unique coverage is the union of requested byte spans per `(path, full SHA-256)`. Requested exposure sums every request including rereads, recovered truncations and the conservative full SnowEnergy search-scan extent. The search's returned matching lines are separately logged because they were requested/displayed again. Internal file hashing and byte-boundary calculations do not claim source reading. The source logger's initial unavailable `python` invocation failed before a source read; subsequent commands used `.venv/bin/python`. Only the assigned scratch and report were written.

Initial source requests 1–8 are bootstrap; all later requests are expansion, including late mandatory governance. Automatically supplied runtime/user instructions, parent message text, compaction delivery, output framing, actual delivered tokens, total runtime/quota, effective model/effort and multi-agent workflow totals are UNOBSERVED here and are not inferred from these file counts. The explicit root AGENTS file read is counted; automatic copies are not silently treated as measured duplicates. Context_report's own internal counting/hashing is not another model source exposure. The full tool source read is counted as tool bytes. Its `--help`, the instruction-discovery command, and measurement execution are tooling operations, not physical runs.

Recovery requests are 10, 23, 27, 29, 33, 37, 38, 45, 46 and 51. Every truncated interval was recovered before completion; no source result relied on the omitted middle of a truncated tool response. Requests after compaction are genuine rereads and are included. The line/byte spans below preserve exact full-request extents, even when a tool delivered a truncated presentation. For explicit end requests past EOF, the ledger records the actual final line/byte boundary; relevant requested upper limits were 2400 for request30, 2670 for44, 500 for90, and140 for102. Chunk-reader EOF requests were69,73 and84. Source search S1 scanned the full already-read SnowEnergy file before requests103–109 with pattern `L_s\(|L_s =|2\.83|283[0-9]{3}`. No negative unlogged source search was used.

| Category | Unique UTF-8 bytes | Requested exposure bytes |
|---|---:|---:|
| LSE | 235583 | 354267 |
| external | 1400919 | 2032535 |
| governance | 89531 | 92517 |
| tool | 3544 | 3544 |
| total | 1729577 | 2482863 |

There were 114 source requests, including search S1. Bootstrap requested exposure was 43108 bytes; expansion requested exposure was 2439755 bytes. Repeated source exposure was 753286 bytes. The measurement utility reproduced the union and exposure totals from the recorded selections. Its report is retained at `/tmp/lse-closure07-e51j3fug/context-report.json`; raw and enriched request logs and the selection are retained in the same unique scratch. Those derived measurement artifacts do not replace this self-contained ledger.

Each path identifier below denotes the exact repository-relative path shown; hashes are full-file SHA-256. Ledger byte spans are zero-based, half-open; line spans are one-based, inclusive. `R` marks recovery, `B` bootstrap and `E` expansion. Repeated bytes count previously requested source within the same hash identity.

| Path ID | Path | Full SHA-256 |
|---|---|---|
| F1 | `AGENTS.md` | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` |
| F2 | `docs/work-packages/AGENTS.md` | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` |
| F3 | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| F4 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| F5 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `f0cacbe45c85b62fc1b78992a9048eadd8fc5857ee5913089c7225ca6873e388` |
| F6 | `docs/work-packages/role-review.md` | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` |
| F7 | `docs/specifications/science-contracts/AGENTS.md` | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` |
| F8 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | `a827f4099976f9592863b71a12c9059dadd36f8b8037dba4e0ebf8482449c0a0` |
| F9 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | `7328dac401340c283934e8e42b5eb0b376c7ed6d9a5723e9202bcd862dc2132b` |
| F10 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | `ac27766f6f329000a1b948d16006056e6d0780b00867dda7b788c4512fa99194` |
| F11 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | `5f96696b2956c157848e7215b3655fa074181b9f76ad9159264164b0598fbe85` |
| F12 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | `58465434d842511c72a5e27653bfa13ee51015f13939f3ad33f2fe8200f6287d` |
| F13 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | `055966f4d3373dba5dcb6a99a3d3c56c02bf948fa0182d8bc74bb20c26490361` |
| F14 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | `b0b5e1c60cc9ed7f316a6a3008f2ee7ea2f337805d2e0e06e82c4453d8a5216e` |
| F15 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | `342b7601176eb263ed3de199c6d9697d987e1650220a615a49c34181e9b27bbd` |
| F16 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | `fe37a1a64327aa46ffb1afbc63e00aedab67c83083a3eb3ccbd2edbc17d3174e` |
| F17 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | `e20bd57efa318ab3f82ca31dfd9712732b9d827d3cd38839295a3b570317a9f7` |
| F18 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | `4778df5b572c4a09b3dfedc1f5bba4c84d376bcfcd0021ae7e9fbb6200158b8c` |
| F19 | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` |
| F20 | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` |
| F21 | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` |
| F22 | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` |
| F23 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` |
| F24 | `docs/standards/AGENTS.md` | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` |
| F25 | `docs/work-packages/science-obligations.md` | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` |
| F26 | `docs/standards/testing-and-gate-strategy.md` | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` |
| F27 | `docs/standards/prompt-wording-guidance.md` | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` |
| F28 | `docs/specifications/correctness-authority-model.md` | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` |
| F29 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/qualification.md` | `48f4e12ab3361b324df21596a1652617004ec7fe8e1d72dcbd96c746ae5f6b24` |
| F30 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md` | `25a6f064713678895b349621034af30dc79010e5f89ee8a4d6734ba98762ec19` |
| F31 | `tools/agents/context_report.py` | `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde` |

| Request | Path ID | Lines | Byte span | UTF-8 bytes | Phase | Previously requested bytes | Recovery |
|---|---|---|---|---:|---|---:|---|
| 1 | F1 | 1–121 | [0,9508) | 9508 | B | 0 | — |
| 2 | F2 | 1–105 | [0,6642) | 6642 | B | 0 | — |
| 3 | F3 | 1–75 | [0,5304) | 5304 | B | 0 | — |
| 4 | F4 | 1–22 | [0,1586) | 1586 | B | 0 | — |
| 5 | F5 | 1–65 | [0,4149) | 4149 | B | 0 | — |
| 6 | F6 | 1–7 | [0,1400) | 1400 | B | 0 | — |
| 7 | F7 | 1–84 | [0,6532) | 6532 | B | 0 | — |
| 8 | F8 | 1–66 | [0,7987) | 7987 | B | 0 | — |
| 9 | F9 | 1–434 | [0,30838) | 30838 | E | 0 | — |
| 10 | F9 | 1–230 | [0,12319) | 12319 | E | 12319 | R |
| 11 | F10 | 1–301 | [0,23267) | 23267 | E | 0 | — |
| 12 | F11 | 1–228 | [0,16209) | 16209 | E | 0 | — |
| 13 | F12 | 1–208 | [0,13234) | 13234 | E | 0 | — |
| 14 | F13 | 1–206 | [0,11649) | 11649 | E | 0 | — |
| 15 | F14 | 1–292 | [0,25644) | 25644 | E | 0 | — |
| 16 | F15 | 1–201 | [0,15424) | 15424 | E | 0 | — |
| 17 | F16 | 1–113 | [0,8225) | 8225 | E | 0 | — |
| 18 | F17 | 1–245 | [0,20961) | 20961 | E | 0 | — |
| 19 | F18 | 1–171 | [0,19223) | 19223 | E | 0 | — |
| 20 | F19 | 1–220 | [0,14929) | 14929 | E | 0 | — |
| 21 | F19 | 221–550 | [14929,30895) | 15966 | E | 0 | — |
| 22 | F19 | 551–900 | [30895,83419) | 52524 | E | 0 | — |
| 23 | F19 | 700–745 | [44545,67927) | 23382 | E | 23382 | R |
| 24 | F19 | 901–1130 | [83419,97301) | 13882 | E | 0 | — |
| 25 | F19 | 1131–1470 | [97301,117570) | 20269 | E | 0 | — |
| 26 | F19 | 1471–1810 | [117570,154023) | 36453 | E | 0 | — |
| 27 | F19 | 1577–1631 | [131582,142366) | 10784 | E | 10784 | R |
| 28 | F19 | 1811–2080 | [154023,174265) | 20242 | E | 0 | — |
| 29 | F19 | 1931–1967 | [162754,165686) | 2932 | E | 2932 | R |
| 30 | F19 | 2081–2205 | [174265,184116) | 9851 | E | 0 | — |
| 31 | F20 | 1–200 | [0,26420) | 26420 | E | 0 | — |
| 32 | F20 | 201–280 | [26420,85328) | 58908 | E | 0 | — |
| 33 | F20 | 234–263 | [43703,68636) | 24933 | E | 24933 | R |
| 34 | F20 | 281–315 | [85328,111289) | 25961 | E | 0 | — |
| 35 | F20 | 316–420 | [111289,144322) | 33033 | E | 0 | — |
| 36 | F20 | 421–620 | [144322,209310) | 64988 | E | 0 | — |
| 37 | F20 | 466–505 | [162726,183167) | 20441 | E | 20441 | R |
| 38 | F20 | 506–546 | [183167,190930) | 7763 | E | 7763 | R |
| 39 | F20 | 621–880 | [209310,238210) | 28900 | E | 0 | — |
| 40 | F20 | 881–1220 | [238210,257527) | 19317 | E | 0 | — |
| 41 | F20 | 1221–1550 | [257527,277995) | 20468 | E | 0 | — |
| 42 | F20 | 1551–1920 | [277995,296612) | 18617 | E | 0 | — |
| 43 | F20 | 1921–2300 | [296612,318942) | 22330 | E | 0 | — |
| 44 | F20 | 2301–2648 | [318942,377637) | 58695 | E | 0 | — |
| 45 | F20 | 2470–2510 | [336664,346767) | 10103 | E | 10103 | R |
| 46 | F20 | 2511–2560 | [346767,359584) | 12817 | E | 12817 | R |
| 47 | F21 | 1–80 | [0,4091) | 4091 | E | 0 | — |
| 48 | F21 | 81–240 | [4091,14809) | 10718 | E | 0 | — |
| 49 | F21 | 241–480 | [14809,31257) | 16448 | E | 0 | — |
| 50 | F21 | 481–730 | [31257,88076) | 56819 | E | 0 | — |
| 51 | F21 | 599–632 | [50054,68763) | 18709 | E | 18709 | R |
| 52 | F21 | 731–1235 | [88076,112072) | 23996 | E | 0 | — |
| 53 | F21 | 1236–1323 | [112072,135642) | 23570 | E | 0 | — |
| 54 | F21 | 1324–1380 | [135642,159180) | 23538 | E | 0 | — |
| 55 | F21 | 1381–1418 | [159180,182957) | 23777 | E | 0 | — |
| 56 | F21 | 1419–1430 | [182957,205388) | 22431 | E | 0 | — |
| 57 | F21 | 1431–1441 | [205388,228531) | 23143 | E | 0 | — |
| 58 | F21 | 1442–1520 | [228531,252422) | 23891 | E | 0 | — |
| 59 | F21 | 1521–1566 | [252422,275296) | 22874 | E | 0 | — |
| 60 | F21 | 1567–1782 | [275296,299276) | 23980 | E | 0 | — |
| 61 | F21 | 1783–2024 | [299276,323051) | 23775 | E | 0 | — |
| 62 | F21 | 2025–2100 | [323051,346644) | 23593 | E | 0 | — |
| 63 | F21 | 2101–2238 | [346644,370623) | 23979 | E | 0 | — |
| 64 | F21 | 2239–2550 | [370623,393902) | 23279 | E | 0 | — |
| 65 | F21 | 2551–2890 | [393902,417517) | 23615 | E | 0 | — |
| 66 | F21 | 2891–2918 | [417517,441270) | 23753 | E | 0 | — |
| 67 | F21 | 2919–3130 | [441270,465223) | 23953 | E | 0 | — |
| 68 | F21 | 3131–3447 | [465223,489056) | 23833 | E | 0 | — |
| 69 | F21 | 3448–3559 | [489056,497785) | 8729 | E | 0 | — |
| 70 | F22 | 1–407 | [0,23980) | 23980 | E | 0 | — |
| 71 | F22 | 408–593 | [23980,46674) | 22694 | E | 0 | — |
| 72 | F22 | 594–851 | [46674,70669) | 23995 | E | 0 | — |
| 73 | F22 | 852–1108 | [70669,92552) | 21883 | E | 0 | — |
| 74 | F23 | 1–268 | [0,23959) | 23959 | E | 0 | — |
| 75 | F23 | 269–616 | [23959,47905) | 23946 | E | 0 | — |
| 76 | F23 | 617–962 | [47905,71833) | 23928 | E | 0 | — |
| 77 | F23 | 963–1074 | [71833,95779) | 23946 | E | 0 | — |
| 78 | F23 | 1075–1386 | [95779,119664) | 23885 | E | 0 | — |
| 79 | F23 | 1387–1511 | [119664,143477) | 23813 | E | 0 | — |
| 80 | F23 | 1512–1850 | [143477,167470) | 23993 | E | 0 | — |
| 81 | F23 | 1851–2275 | [167470,191431) | 23961 | E | 0 | — |
| 82 | F23 | 2276–2622 | [191431,215322) | 23891 | E | 0 | — |
| 83 | F23 | 2623–2981 | [215322,239320) | 23998 | E | 0 | — |
| 84 | F23 | 2982–3080 | [239320,248829) | 9509 | E | 0 | — |
| 85 | F4 | 1–22 | [0,1586) | 1586 | E | 1586 | — |
| 86 | F6 | 1–7 | [0,1400) | 1400 | E | 1400 | — |
| 87 | F24 | 1–58 | [0,4052) | 4052 | E | 0 | — |
| 88 | F25 | 1–98 | [0,6149) | 6149 | E | 0 | — |
| 89 | F26 | 1–465 | [0,23940) | 23940 | E | 0 | — |
| 90 | F26 | 466–494 | [23940,25142) | 1202 | E | 0 | — |
| 91 | F27 | 1–202 | [0,12057) | 12057 | E | 0 | — |
| 92 | F28 | 1–221 | [0,11159) | 11159 | E | 0 | — |
| 93 | F5 | 1–65 | [0,4149) | 4149 | E | 4149 | — |
| 94 | F29 | 1–248 | [0,19465) | 19465 | E | 0 | — |
| 95 | F30 | 1–267 | [0,19308) | 19308 | E | 0 | — |
| 96 | F9 | 1–230 | [0,12319) | 12319 | E | 12319 | — |
| 97 | F10 | 1–160 | [0,13007) | 13007 | E | 13007 | — |
| 98 | F10 | 161–301 | [13007,23267) | 10260 | E | 10260 | — |
| 99 | F11 | 1–125 | [0,7239) | 7239 | E | 7239 | — |
| 100 | F16 | 1–113 | [0,8225) | 8225 | E | 8225 | — |
| 101 | F11 | 123–228 | [7044,16209) | 9165 | E | 9165 | — |
| 102 | F31 | 1–79 | [0,3544) | 3544 | E | 0 | — |
| 103 | F21 | 1000–1000 | [101279,101310) | 31 | E | 31 | — |
| 104 | F21 | 1088–1088 | [105154,105178) | 24 | E | 24 | — |
| 105 | F21 | 1092–1092 | [105245,105322) | 77 | E | 77 | — |
| 106 | F21 | 1161–1161 | [107920,107992) | 72 | E | 72 | — |
| 107 | F21 | 1162–1162 | [107992,108026) | 34 | E | 34 | — |
| 108 | F21 | 1331–1331 | [141596,142830) | 1234 | E | 1234 | — |
| 109 | F21 | 1541–1541 | [257268,257763) | 495 | E | 495 | — |
| 110 | F12 | 18–155 | [1572,7861) | 6289 | E | 6289 | — |
| 111 | F17 | 1–160 | [0,10694) | 10694 | E | 10694 | — |
| 112 | F9 | 231–434 | [12319,30838) | 18519 | E | 18519 | — |
| 113 | F13 | 18–145 | [2105,8604) | 6499 | E | 6499 | — |
| S1-before-103 | F21 | 1–3559 | [0,497785) | 497785 | E | 497785 | — |

Self-check: each applicable mechanism above states its input/precondition, chronology, operands, exclusion, evidence and failure boundary; the concrete phase, spill, resource, exact-carry, support and solve rules were checked against the read sources before completion. Scope-dependent experimental capture and provider-equation reconstruction were explicitly not claimed. No numerical result was manufactured. This completed first report is frozen for independent assessment.
