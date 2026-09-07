Static: first-attempt independent scientific reading of the frozen candidate-v32 directory. No implementation, numerical workflow, comparator, production consumer, or scientific test was run. Commands below only read source or compute source/report identities. This report does not establish adoption, implementation correctness, calibration, qualification, or production readiness.

# Exercise 01, fresh reader retry 03

Assigned question: identify surface-energy/soil coupling rules for a snow-free covered forest tile, including equilibrium-zero versus finite-capacity, first-node transfer, immutable water, and ingress energy. The answer is conditional on actual configuration: “forest” does not establish the surface class, phase state, canopy authority tuple, or support. Those inputs were not supplied. I read the unspecified-litter phase expansion instead of assuming all-liquid litter.

## Exact physical rules and authority boundaries

The governing entry is `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` version 32. Anchors below are relative to its chapter directory unless an external contract is named. Source identities and complete request ranges appear later.

1. **Owners and admitted regime.** `surface-energy.md#exact-ownership-and-state`, `#selected-sources-and-domain`, `soil-coupling.md#INV-LANDSURFACEENERGY-100`, and `interface.md#interface` distinguish hydrology-owned liquid/frozen masses, the LSE surface thermal node per tile, and the soil-thermal owner's ordered `N` temperatures/enthalpies. Vegetation retains canopy physiology, component temperatures, optics, hydraulics and water-protocol participation. LSE cannot mutate or reconstruct an independent water pool or duplicate soil temperatures. V1 is snow absent at both endpoints, no terminal payload, liquid/unfrozen soil, positive finite neutral wind/support/area, complete typed forcing/configuration/state and one admitted surface class per tile. Bare mineral and forest litter have separate configured properties. Calm, nonneutral, frozen/thawing soil, snow, missing thermal lineage or unsupported topology reject before physical evaluation. V2 imports V1 physics and its exact vegetation-owner boundary; V3 is the separately tagged snow-free forest-litter phase successor. Frozen litter does not authorize soil freeze/thaw, bare-soil phase, or represented-snow physics. Mixed successor identities and silent downgrade are invalid.

2. **Covered radiation and turbulent recipient.** `surface-energy.md#surface-energy` retains the V7/V8 full-column VIS/NIR shortwave operator: ground receives the terminal downward transmission once and reflected ground shortwave traverses the canopy again. Longwave has unit emissivity/no reflection, `tau_i=exp[-0.8*Omega_i*(LAI_i+SAI_i)]`, with clumping applied once and emission from the current trial component temperatures. Ground net longwave is `L_down,n-sigma*T_s^4`. For canopy component `j` in layer `i`, the net contribution is `w_j*(1-tau_i)*(L_down,i+L_up,i+1)-2*w_j*(1-tau_i)*sigma*T_j^4`; zero component area gives exact zero contribution and zero layer area gives `tau_i=1`. No stale prescribed ground-upward longwave or canopy-bypassing reflection is admissible.

   One zero-storage canopy-air pair `(T_c,q_c)` couples the covered tile. Ground `H_s=rho_a*c_p*(T_s-T_c)/r_g-c` and signed vapor `v_s=rho_a*(q_s-q_c)/r_g-c` use canopy air, not reference air. The covered neutral ISBA-MEB resistance uses the declared geometry and logarithm domains, with `psi_H=f_hv=1`; the canopy-air-to-atmosphere log-law is a separate transfer. Canopy heat/vapor equations sum all occupancy contributions and add the ground contribution exactly once per tile. Multiplying the ground term by occupancy count or using an open-ground reference-air formula under cover is prohibited. The required full `SC-VEGETATION-001.md` V8 coupled-ground amendment supplies the canopy side; later admitted V10/V11 tuples do not permit arbitrary identity substitution.

3. **Finite capacity and exact equilibrium zero are different state models.** `soil-coupling.md#surface-humidity-surface-enthalpy-litter-and-soil-heat` and `#INV-LANDSURFACEENERGY-103` define

   ```text
   T_ref = 273.15 K; C_w = 4218 J kg^-1 K^-1
   U_s = (C_dry + W*C_w)*(T_s-T_ref).
   ```

   The mass `W` is the immutable hydrology operand during a solve. For `finite_capacity`, `C_dry+W*C_w>0`, beginning temperature derives from authoritative beginning enthalpy, mass and dry capacity, and ending temperature derives from ending enthalpy and the ending hydrology mass. A retained temperature warm start must be bit-identical to its required reconstruction. For `equilibrium_zero`, `C_dry=0`, `W=0`, `U_s=0` exactly; storage change is exactly zero and `T_s` is an algebraic unknown. There is **no physical beginning surface temperature**. The current algebraic trial `T_s` is used in both Crank–Nicolson surface-side endpoint operands. A caller warm-start temperature cannot become a physical beginning-state operand, and a small invented heat capacity is not an equivalent branch.

4. **One first-node exchange, harmonic half-cell resistance, positive downward.** The same soil-coupling anchor defines positive finite surface skin thickness/conductivity and every node's `dz_k,lambda_k,C_k`, exact ordered `k=1..N`, `N>=1`:

   ```text
   g_s1 = 2/(dz_s/lambda_s + dz_1/lambda_1)
   g_k,k+1 = 2/(dz_k/lambda_k + dz_(k+1)/lambda_(k+1))
   G_s1 = g_s1*(T_s-T_1)
   G_k,k+1 = g_k,k+1*(T_k-T_(k+1))
   bar(G) = 0.5*(G_begin+G_end)
   C_1*(T_1,1-T_1,0)/dt = bar(G_s1)-bar(G_1,2)
   C_k*(T_k,1-T_k,0)/dt = bar(G_k-1,k)-bar(G_k,k+1)
   C_N*(T_N,1-T_N,0)/dt = bar(G_N-1,N).
   ```

   Lower-boundary flux is exactly zero. For `N=1`, the sole equation is `C_1*Delta(T_1)/dt=bar(G_s1)`. The surface loses this one exchange and the first soil node receives it once; no separate independently calculated “matching” flux is permitted. Finite capacity uses derived beginning `T_s,0` and current trial `T_s,1`. Equilibrium zero uses `g_s1*(T_s,trial-T_1,0)` and `g_s1*(T_s,trial-T_1,1)`. `INV-LANDSURFACEENERGY-106` binds exchange, infiltration energy and routed enthalpy by exact OFE/tile/support identity. The common ledger's `G` uses its into-surface sign; its C-003 phrase “soil consumes -G” must not be read as reversing the explicit positive-downward `G_s1` equations above.

   The represented-snow boundary in this chapter is a distinct OFE/lane bottom-snow-volume-to-first-soil-node interface. It excludes tile LSE thermal nodes, selected/averaged tiles, tile weighting and duplication. It is not a substitute for the snow-free covered tile law. I did not adjudicate or implement the snow evaluator.

5. **Litter is an actual thermal/moisture layer.** The soil-coupling anchor gives

   ```text
   h_ul = 0.5*(1-cos(pi*W_l/W_l,max))
   q_l = h_ul*q_sat(T_s,p)+(1-h_ul)*q_recipient
   v_l = rho_a*(q_l-q_recipient)/r_l-c
   lambda_l = 0.1+0.03*W_l/(rho_w*dz_l)
   C_dry = dz_l*rho_ld*c_ld.
   ```

   Capacity is strictly positive and `0<=W_l<=W_l,max`. Covered litter uses the covered resistance and canopy-air humidity. Litter blocks direct mineral-soil evaporation and upward capillary supply during the interval. Overflow belongs to hydrology ingress. The class is not a dry-soil evaporation multiplier.

6. **Immutable beginning water, one authorization, complete fixed-cap solve.** `water-vapor.md#water`, `#vapor`, its canonical invariants/obligations, `solve-boundary.md#solve`, and `SC-SURFACELIQUID-001` algorithm/invariants 004/017/018 require: freeze all beginning source stores before current rain, runon, canopy release, or other current ingress; solve the complete potential coupled system without owner caps; issue positive demands; hydrology authorizes all competing requests against that same beginning availability once; then rebuild from original beginning owners with fixed authorizations and solve all gas, energy, canopy-air, hydraulics, surface and soil equations again. FullSupply may seed coordinates but must reevaluate the complete fixed-final system; a passing initial evaluation can accept at iteration zero. Potential-state continuation without rebuilding, scalar stress repair, new authorization after final canopy release, or ingress donation is prohibited.

   `D=max(v,0)*dt` in the proper request basis, and the finalized OFE-ground amount obeys `F=f_tile*q_final*dt`, exactly `0<=F<=A<=D`; `A` is an upper authorization, not actual use. Equality selects the cap-active branch with zero derivative. Only finalized positive use debits the named source. Negative signed vapor is a condensation credit of `-v*dt`, not a negative demand. Soil and surface resource rows retain their own keys, beginning-store identity, support and area basis. Aggregate same-layer/source overbooking, row omission and duplicate application reject atomically.

7. **Signed vapor transports sensible plus latent enthalpy.** `water-vapor.md#vapor` defines `h_l(T)=C_w*(T-T_ref)`, parcel `Q=m*h_l(T)`, and `L_v(T)=2.501e6-2369*(T-T_ref)`. Positive-mass mixing conserves enthalpy, with `T_mix=T_ref+sum(m*h_l)/(C_w*sum(m))`; zero mass has no temperature or energy operand. Surface vapor energy away from the surface is `Q_v=v_s*(h_l(T_s)+L_v(T_s))`. Evaporation removes both terms; condensation adds both. Latent-only exchange is wrong. Before current ingress, the surface equation is

   ```text
   (U_pre_ingress-U_begin)/dt = R_sw+R_lw-H_s-Q_v-G_s1
   ```

   with the accepted conduction time discretization above. Primitive components are reconstructed locally, then converted/weighted to the OFE basis once. A producer-supplied residual is diagnostic, never acceptance authority.

8. **Ingress occurs after accepted capped fluxes and carries its own energy.** `water-vapor.md#water`, `#vapor`, `SC-SURFACELIQUID-001` algorithm/producer obligations and `SC-WATBAL-001#wb14-infiltration-and-hyetograph-coupling-addendum` require finalized debits and condensation, then current rain/runon/throughfall/first and second canopy drain/stemflow/litter overflow, then one attributed mixing/infiltration/excess/retention/routing operation. Under cover, finalized canopy-ground releases are supply; raw rain cannot also be added. Rain temperature is the retained provider hydrometeor temperature converted to K, not independently recomputed. Runon uses the accepted upstream parcel temperature/enthalpy; air, soil, reference, downstream-surface and stale temperatures cannot substitute. Infiltration/runoff carry the accepted source or enthalpy-conserving mixed liquid temperature.

   ```text
   U_surface,end = U_pre_ingress + Q_actually_retained
   E_soil,1,end = E_soil,1,pre_ingress + Q_infiltration
   ```

   The thermal receiver derives its temperature from its ending enthalpy; hydrology independently owns the attributed infiltrated mass. Retained energy must not be credited when liquid was infiltrated or routed. Routed energy stays with its physical parcel. Unequal OFE areas require the named `A_up/A_down` conversion of areal mass and enthalpy once; parcel basis is rekeyed while source lineage persists. “Same parcel” does not mean numerically identical per-area depths at unequal areas. No current ingress alters same-support `H`, `LE`, or `G`; no second LSE solve or WB14 call is permitted. WB14 is the shared stateful Green–Ampt owner once per OFE/accepted child, not a copied formula, per-parcel solve, full-day replay or proportional infiltration proxy. Small unrepresentable persistent retention follows SurfaceLiquid's explicit whole-parcel onward-routing rule, never mass/energy discard.

9. **Unspecified litter phase required the V3 expansion.** `litter-phase.md#litter-phase`, `#spill`, `#resource-join`, invariants 140–149/156/157, and SurfaceLiquid 016–021/028/029 specialize only snow-free litter. The full nonlinear evaluation is phase-free; liquid evaporation/condensation and ice sublimation/deposition have separate signed energy and beginning-availability custody. The frozen fraction is zero for an exactly empty pool, otherwise `W_i,0/(W_l,0+W_i,0)`. Both phase vapor laws use saturation over liquid water. Constants are `rho_i=920`, `C_i=2106`, `L_f=333700`, `tau_ice=3300 s`, and ice capacity `0.85*rho_w*dz_l`, with `rho_w=1000`. The bounded post-vapor kinetic freeze/melt operator uses the exact post-vapor state; equal/opposite phase mass changes and `U_phase=U_*+L_f*(m_freeze-m_melt)` conserve `U-L_f*W_i`. Ending temperature uses ending dry/liquid/ice heat capacity. Instant equilibrium, wrong fusion sign, old capacity, current-ingress donation, saturation-over-ice substitution, phase inferred from temperature, soil/frozwt compensation, and same-support phase/flux re-solve are prohibited.

   If raw bounded-phase liquid exceeds liquid capacity, one typed spill uses `m_spill=W_raw-W_l,max`, `h_spill=C_w*(T_raw-T_ref)`, `Q_spill=m_spill*h_spill`, then authoritative subtractions `W_retained=W_raw-m_spill`, `U_retained=U_raw-Q_spill`. Recompute retained temperature from ending capacity. The spill is liquid only, binds the phase receipt and full `[0,dt)` child support, adds one negative `LitterPhaseCapacitySpillEnergy` operand after phase-free/fusion operands, and enters current-ingress/WB14 once. Retained spill portions return only via ordinary retained-ingress credits. Clamp, tolerance snap, condensation alias and discarded excess are invalid. A heterogeneous batch matches native phase rows exactly and excludes them from replay; all unmatched ordinary finalized rows debit the phase-adjusted liquid owner once in canonical key order with no additional energy operation or second ingress.

10. **Exact energy custody and non-owner candidates.** `soil-custody.md#exact-soil`, `surface-custody.md#exact-surface`, their canonical invariants, and SurfaceLiquid 022/023 preserve accepted finite energy smaller than one storage ULP. Authoritative soil `E=exact(H_hi)+R` and surface `U=exact(U_hi)+R_U` aggregate the exact beginning total and every named accepted finite-binary64 physical operand, round once to finite nearest-even high, and retain normalized signed-dyadic carry. Surface arithmetic begins after the named accepted OFE-to-tile conversion. Frozen LSE-V3/surface-V2 fields are bit-identical high mirrors, not alternate authorities. Carry is not a new temperature, flux, phase, producer residual or tolerance repair. No nextafter, forced ULP, zero snap, flush or lost sub-ULP credit is admitted. Receipt/restart/checkpoint and complete-owner joins retain exact carries.

   Parent-local partial children advance exact carry and receipt chain while retaining the persistent predecessor marker; only the child ending at the sealed parent endpoint stamps the persistent-parent-final marker once. Order uses authenticated configuration topology rank for opaque OFE IDs, then within-OFE keys and operand order. Unpublished soil beginnings may enter only a typed private non-owner candidate posture, never forged soil owner/restart bytes. Final acceptance requires the original-owner replay and complete atomic seal.

11. **Support and final acceptance are real boundaries.** `terminal-support.md#support`, LSE invariants 116–123 and full `SC-COUPLEDTIME-001` require exact shared half-open integer-nanosecond support and one derived binary64 duration shared bit-identically. The covered-forest minimum is `60_000_000_000 ns`; below-floor positive support rejects before Newton (`LSEB-E-041`) and preserves all owners; exact floor uses unchanged physics/tolerances. Zero support performs no physical solve. The active-participant common minimum is the maximum of their admitted minima. A one-nanosecond clock identity is not a physical support promise. No longer-step scaling, frozen successor, hidden floor, relaxed tolerance, fabricated remainder or subfloor split is admissible; stable supports must still admit substantially larger steps.

   The solve boundary orders domain, class, covered/open recipient, wet/dry vapor, vapor sign, cap branch, then numerical work. Energy residual threshold is `1e-6 W m^-2+1e-10*max(1,sum_abs_operands)`; water/vapor is `1e-12 kg m^-2 s^-1+1e-9*scale`; accepted temperature/humidity/hydraulic/beta step limits are `1e-8 K`, `1e-12 kg kg^-1`, `1e-7 mm`, `1e-10`. Physical bounds and exact identity/D-A-F relations are not tolerance-repaired. Error precedence and complete failure context/rollback remain mandatory. This is accepted-primitive/rule selection, not a review of all V10–13 solver algorithms.

   `map-custody.md#handoff` and `#pending` were expanded to establish physical receipt origin. A validated unchanged private object can move through exact revision-bound non-wire custody; mutation or untrusted/restart reconstruction needs full validation. Canonical covered maps charge Initial, then pending adjudications: outer nonclosure consumes a pending value into history; dependent-only failure consumes it into typed adaptive rejection; full closure consumes that same already-executed physical prefix into final custody once. Nonfinal endpoints cannot be promoted; no additional final physical replay exists; maps publish zero, accepted composed parent commit publishes once. Separate snow-free provisional identity-only reseal is single-use and cannot be used to promote a canonical covered nonfinal map. This report does not certify optimization equivalence or the represented-snow native implementation.

## Required inputs and tests, without invented run evidence

Actual run values were not supplied. A concrete calculation needs exact model/schema/configuration/owner/transaction/predecessor identities; OFE topology and areas, tile fractions, surface class and explicit phase state; full covered vegetation occupancy/component/hydraulic/optical state; current forcing (band/direction radiation, longwave, air pressure/temperature/humidity/density and positive neutral wind); canopy and skin geometry/roughness; ordered soil layer thickness/conductivity/areal capacities and beginning temperatures/enthalpies; authoritative surface enthalpy, water masses and capacity/dry-litter properties; complete warm-start coordinates; exact support and admission receipts; all D/A/F source rows and condensation receipts; and actual attributed ingress parcel timing, mass, temperature/enthalpy and source/destination identities. Exact high/carry successors additionally require their full lineage and receipt chains. Missing values remain missing and reject; this answer supplies no fabricated forcing, parameter default, thermal state or result.

Required acceptance evidence comes from `common-details.md#tests`, `water-vapor.md` V1 independent fixtures and P001–004, `soil-coupling.md` C003/INV100/103/106, `litter-phase.md` C011/C012 and real-consumer vectors, `surface-custody.md`/`soil-custody.md` and the corresponding external contract tests. At minimum:

- Finite-capacity and equilibrium-zero cases with distinct caller warm starts; invariant zero-branch physical output, correct two-endpoint CN use, `N=1` and multilayer closure, and two-sided heat-flow reversal. Wrong beginning warm temperature, a duplicate first-node credit, wrong half-cell resistance, wrong sign/node and absent lower boundary must fail.
- Covered versus open recipient separation, day/night and zero shortwave, longwave/ground-flux reversal, canopy-ground feedback, dry/wet litter and heterogeneous tiles. Poison stale temperatures, unattenuated ground reflection, repeated ground-per-occupancy terms and reference-air use under cover.
- Evaporation/condensation, full/partial surface and top-layer authorization, root scarcity/dry source, exact cap equality, no same-support rain/runon/canopy-release donation, original-beginning fixed-cap reconstruction and D/A/F identities.
- Independently sourced retained/infiltrated/routed liquid mass and sensible enthalpy; warm/cold ingress, mixed temperatures, unequal OFE areas and factor-of-1000/unit traps; one WB14 child operation and source-complete receipts. Producer residuals or both sides rebuilt from one alias do not close a gate.
- Exact 60-second support and its one-nanosecond-below poison, zero support/no solve, larger stable support, shared duration bits and support receipts; all rejected paths retain complete owner/clock/receipt/publication bytes.
- V3 empty/all-liquid/all-ice/mixed, freezing/melting/exact reference, condensation/deposition, independent phase caps, ice-capacity saturation, `dt` below/equal/above 3300 s; wrong constants/saturation/fusion/old-capacity/no-ingress-donation poisons. Below/at/above liquid capacity, melt-created spill, exact raw=retained+spill energy/mass, negative exact-surface spill operand, full-child timing and one WB14 custody. Heterogeneous native/ordinary rows require no replay, exact complete partition and no energy duplication.
- Negative, sub-ULP and subnormal exact energy credits, tie-even rounding and normalized carry; frozen high mirrors; authenticated topology ordering including opaque multi-digit OFE IDs; partial/final markers; restart replay, wrong receipt/owner/support/configuration and full-envelope rollback poisons. Trusted custody and pending-map dispositions additionally require exact physical-prefix differential proof, counters, one-use and no-promotion/no-publication tests at their governed boundary.

The phase successor names actual production consumers `tests/integration/erosion_single_ofe_p61_sediment.rs` and `tests/integration/dff_ws1_native_forest_cli.rs`, through production selection and successor persist/reload, with primitive water/ice/fusion/vapor/WB14/whole-envelope closure. I did not open or run them: they are future evidence requirements, not results. No inherited `[Ran]` assertion in a source is represented as an exercise run. The narrowly specified one-upward-binary64-neighbor publication normalization at 273.15 K applies only at its named wet-liquid/Stage-3 boundary; it supplies no general domain tolerance.

## Expansion rationale and retained limits

Bootstrap was root/science/work-package governance, package/handoff, role/standard guidance, then the LSE entry, full interface and surface-energy route. Physical expansion followed surface-energy to full common details, soil coupling, solve boundary, water/vapor, complete terminal support and unspecified-litter phase. Ingress and exact accepted operand custody required full surface/soil custody and physical map handoff/pending origin. Covered canopy required full Vegetation; ingress required full WATBAL and SurfaceLiquid; support/custody required full CoupledTime. These four single-file external contracts were read to EOF, including dense registers, guards, obligations and current appendices; an anchor was not treated as permission to sample them.

No legacy source, original monolith, Git history, archived/predecessor candidate, intake/prediction/rubric/review, earlier answer, parent test output or other reader answer was consulted. References embedded in allowed current authority are recorded as provenance, not a claim that the referred historical artifact was inspected. I did not inspect the LSE historical chapter. The source's prohibited prior evidence was not opened.

The deferred LSE modules remain binding when their triggers occur: `nonlinear-solve` for solver implementation/full evaluator or numerical branch review; `dependency-replay` for optimization/error-order/evaluator-reuse equivalence; `qualification` for executable identity/frozen capture protocol/qualification; `audit-details` for source-provenance, enforcement-path or historical promotability adjudication; and binding index/full normative expansion for unresolved route selection. None of those adjudications is claimed here. Native represented-snow evaluator/transition/publication work would additionally trigger full SnowEnergy and other named snow authority. This task establishes snow-free rules and the boundary exclusion, not a native snow-map or transition review. General cross-contract citations in external histories do not silently broaden this exercise into all hydrology/erosion/forcing implementation. The expressly selected whole external contracts remained binding and were completed.

## Measurement method and interpretation

Measurements are exact UTF-8 source bytes, inclusive line ranges with original line endings; full-file SHA-256 identifies every requested source even when only a section was requested. The ledger was reconstructed from this reader's own read calls, then measured against frozen current files with a temporary local Python script. `full` becomes actual line 1 through EOF. A requested end beyond EOF is retained with its actual end; nonexistent lines contribute zero bytes. Metadata-only `find-agents`, `wc` and heading-only `rg` queries are disclosed separately below and do not inflate source-content range bytes. Source identity hashing performed for arithmetic reads the files in the measurement process, but did not deliver their text and is not another model source exposure.

A request's bytes count the entire source range requested even when tool delivery was truncated. Repeated bytes count overlap with all earlier requested ranges of the same file. Unique union is the finite set of distinct requested lines, not a sum of overlapping requests, an imagined context window, or delivered tokens. Truncation recovery explicitly rereads gaps/uncertain boundaries and remains repeated exposure. Tool output showed both inner command and outer tool truncation on some large dense requests; exact truncated-away delivered bytes/tokens were not available. Conservative recovery ranges are marked, and all required extents were ultimately read. Automatically injected root AGENTS text, prompts, parent messages, source summaries and compaction are runtime exposure outside this file-range metric; their encoded/token costs are UNOBSERVED rather than silently set to zero.

`delivered_source_tokens=UNOBSERVED`; `delivered_context_tokens=UNOBSERVED`; `total_workflow_tokens=UNOBSERVED`; `workflow_cost=UNOBSERVED`; no tokenizer, billing or delivery telemetry was available. No claimed wall-clock/token efficiency follows from file bytes.

### Byte totals

| Category / phase | Requested bytes | Repeated requested bytes | Newly added unique bytes |
|---|---:|---:|---:|
| governance | 81836 | 8118 | 73718 |
| governance/bootstrap | 47081 | 0 | 47081 |
| LSE | 276041 | 81107 | 194934 |
| LSE/bootstrap | 23531 | 0 | 23531 |
| LSE/expansion | 252510 | 81107 | 171403 |
| external | 1232891 | 329757 | 903134 |
| external/expansion | 1232891 | 329757 | 903134 |
| governance/expansion | 34755 | 8118 | 26637 |
| All categories | 1590768 | 418982 | 1171786 |

The LSE finite unique source union is 194934 bytes, 26.401951% below the supplied 264863-byte comparator size. The LSE requested exposure is 276041 bytes, including 81107 repeated bytes; it is not the unique-union result. The always-read entry plus complete shared interface total 12106 bytes; route bootstrap additionally includes surface-energy. External unique bytes (903134) and governance unique bytes (73718) remain separately visible and are not disguised as LSE savings. These are arithmetic observations only; the parent owns independent acceptance/adoption assessment.

### Source identities

| ID | Source path | Full lines | Full UTF-8 bytes | Full-file SHA-256 |
|---|---|---:|---:|---|
| S01 | `AGENTS.md` | 121 | 9508 | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` |
| S02 | `docs/specifications/science-contracts/AGENTS.md` | 84 | 6532 | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` |
| S03 | `docs/work-packages/AGENTS.md` | 105 | 6642 | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` |
| S04 | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | 75 | 5304 | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| S05 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | 22 | 1586 | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| S06 | `docs/work-packages/role-review.md` | 7 | 1400 | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` |
| S07 | `docs/standards/AGENTS.md` | 58 | 4052 | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` |
| S08 | `docs/standards/prompt-wording-guidance.md` | 202 | 12057 | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` |
| S09 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 65 | 4130 | `930c682e5c923083683e8a030457c9e5e90e6ca9cd2f70a425ee0aa79c326c72` |
| S10 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | 66 | 7976 | `0335d3b03acab1bdd711b3be589b429dcd63b7023e516561e269700006fbb2ff` |
| S11 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | 205 | 11425 | `7b77973b1d8be2a700b56063d097ac8f0ab8ae46b1ea265bc28c99f9ddca7495` |
| S12 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | 208 | 13249 | `582c8591001247342a1a0fa919dcd6e3cd267f03ceec04101c87890a6b0ec094` |
| S13 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | 113 | 8225 | `fe37a1a64327aa46ffb1afbc63e00aedab67c83083a3eb3ccbd2edbc17d3174e` |
| S14 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | 298 | 22401 | `20d4bfdd94cce60ed9ee13e1ded2c1d8d6c1f99cbf69b74483e10d4efd041e2a` |
| S15 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | 227 | 15924 | `7d7e0816257f51d2878450c7cbf5cb5d69e9e5fb061c0a49f52bae5a0beef3ce` |
| S16 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | 245 | 20940 | `9eb3f77d55de9ed404f89bff9c313194780dd816bb43fb087b7e4927ca0e0007` |
| S17 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | 426 | 30338 | `787558481951bc98d78e6c65d6127218e11fcd9fa9cf5c31ccea7f3e649c757c` |
| S18 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | 290 | 25619 | `a3169e84ce8e828dc0b51c6fad51d558860f0a51692cfe72975a9c61cc20fc03` |
| S19 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | 201 | 15484 | `3585c1a3713dfe4224fd03f72a88133530d1e14810b34ec5587c05aab2dc5922` |
| S20 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 3080 | 248829 | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` |
| S21 | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 2205 | 184116 | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` |
| S22 | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 2648 | 377637 | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` |
| S23 | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 1108 | 92552 | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` |
| S24 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | 171 | 19223 | `4778df5b572c4a09b3dfedc1f5bba4c84d376bcfcd0021ae7e9fbb6200158b8c` |
| S25 | `docs/standards/testing-and-gate-strategy.md` | 494 | 25142 | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` |
| S26 | `docs/work-packages/science-obligations.md` | 98 | 6149 | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` |
| S27 | `docs/specifications/correctness-authority-model.md` | 221 | 11159 | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` |

### Every content-range request

Source ID resolves to the exact path and full-source SHA-256 above. Ranges are inclusive, in source-request order. Phase `B` is bootstrap and `E` is expansion. Repetition is measured even where the earlier delivery was truncated. Requests made in one tool call are separate rows; this does not imply separate workflow operations.

| Request | Source | Requested lines | Actual end | Phase | Requested bytes | Repeated bytes | Read/recovery note |
|---:|---|---|---:|---|---:|---:|---|
| 1 | S01 | 1–121 | 121 | B | 9508 | 0 | source reading |
| 2 | S02 | 1–84 | 84 | B | 6532 | 0 | source reading |
| 3 | S03 | 1–105 | 105 | B | 6642 | 0 | source reading |
| 4 | S04 | 1–75 | 75 | B | 5304 | 0 | source reading |
| 5 | S05 | 1–22 | 22 | B | 1586 | 0 | source reading |
| 6 | S06 | 1–7 | 7 | B | 1400 | 0 | source reading |
| 7 | S07 | 1–58 | 58 | B | 4052 | 0 | source reading |
| 8 | S08 | 1–202 | 202 | B | 12057 | 0 | source reading |
| 9 | S09 | 1–65 | 65 | B | 4130 | 0 | source reading |
| 10 | S10 | 1–66 | 66 | B | 7976 | 0 | source reading |
| 11 | S11 | 1–205 | 205 | B | 11425 | 0 | source reading |
| 12 | S12 | 1–208 | 208 | E | 13249 | 0 | source reading |
| 13 | S13 | 1–113 | 113 | E | 8225 | 0 | source reading |
| 14 | S14 | 1–298 | 298 | E | 22401 | 0 | source reading |
| 15 | S15 | 1–227 | 227 | E | 15924 | 0 | source reading |
| 16 | S16 | 1–245 | 245 | E | 20940 | 0 | combined response truncated; explicit recovery later |
| 17 | S17 | 1–426 | 426 | E | 30338 | 0 | combined response truncated; explicit recovery later |
| 18 | S17 | 1–220 | 220 | E | 11629 | 11629 | truncation recovery |
| 19 | S18 | 1–290 | 290 | E | 25619 | 0 | combined response truncated; surface recovery later; soil tail visible |
| 20 | S19 | 1–201 | 201 | E | 15484 | 0 | combined response truncated; surface recovery later; soil tail visible |
| 21 | S18 | 285–310 | 290 | E | 2963 | 2963 | truncation recovery; end requested beyond EOF |
| 22 | S16 | 236–245 | 245 | E | 2575 | 2575 | truncation recovery |
| 23 | S20 | 1–350 | 350 | E | 30263 | 0 | source reading |
| 24 | S20 | 351–650 | 650 | E | 19835 | 0 | source reading |
| 25 | S20 | 651–1000 | 1000 | E | 30039 | 0 | source reading |
| 26 | S20 | 1001–1300 | 1300 | E | 32982 | 0 | source reading |
| 27 | S20 | 1301–1650 | 1650 | E | 40710 | 0 | truncated; later recovery |
| 28 | S20 | 1480–1520 | 1520 | E | 12564 | 12564 | truncation recovery |
| 29 | S20 | 1651–1950 | 1950 | E | 18864 | 0 | source reading |
| 30 | S20 | 1951–2250 | 2250 | E | 16168 | 0 | source reading |
| 31 | S20 | 2251–2550 | 2550 | E | 21817 | 0 | source reading |
| 32 | S20 | 2551–2820 | 2820 | E | 17217 | 0 | source reading |
| 33 | S20 | 2821–3080 | 3080 | E | 20934 | 0 | source reading |
| 34 | S20 | 1465–1482 | 1482 | E | 5922 | 5922 | truncation recovery |
| 35 | S21 | 1–400 | 400 | E | 23325 | 0 | source reading |
| 36 | S21 | 401–800 | 800 | E | 47854 | 0 | truncated; later recovery |
| 37 | S21 | 660–720 | 720 | E | 25983 | 25983 | truncation recovery |
| 38 | S21 | 801–1100 | 1100 | E | 24428 | 0 | combined response truncated; later recovery |
| 39 | S21 | 701–830 | 830 | E | 30865 | 30865 | recovery; combined response truncated |
| 40 | S21 | 1101–1500 | 1500 | E | 23990 | 0 | combined response truncated; later recovery |
| 41 | S21 | 781–840 | 840 | E | 7370 | 7370 | truncation recovery |
| 42 | S21 | 1101–1120 | 1120 | E | 1119 | 1119 | truncation recovery |
| 43 | S21 | 1501–1750 | 1750 | E | 30734 | 0 | combined response truncated; later recovery |
| 44 | S21 | 1560–1600 | 1600 | E | 9570 | 9570 | truncation recovery |
| 45 | S21 | 1751–1950 | 1950 | E | 13691 | 0 | source reading |
| 46 | S21 | 1951–2205 | 2205 | E | 20094 | 0 | source reading |
| 47 | S22 | 1–300 | 300 | E | 108092 | 0 | truncated; later recovery |
| 48 | S22 | 121–190 | 190 | E | 7963 | 7963 | truncation recovery |
| 49 | S22 | 191–240 | 240 | E | 23738 | 23738 | truncation recovery |
| 50 | S22 | 241–280 | 280 | E | 36774 | 36774 | truncation recovery |
| 51 | S22 | 281–310 | 310 | E | 23822 | 22764 | combined response truncated; later recovery |
| 52 | S22 | 311–410 | 410 | E | 32804 | 0 | combined response truncated; later recovery |
| 53 | S22 | 298–355 | 355 | E | 21731 | 21731 | truncation recovery |
| 54 | S22 | 356–510 | 510 | E | 57921 | 14219 | truncated; later recovery |
| 55 | S22 | 411–460 | 460 | E | 18411 | 18411 | truncation recovery; 461-510 visible in original tail |
| 56 | S22 | 511–690 | 690 | E | 30306 | 0 | source reading |
| 57 | S22 | 691–1000 | 1000 | E | 29522 | 0 | source reading |
| 58 | S22 | 1001–1330 | 1330 | E | 17233 | 0 | source reading |
| 59 | S22 | 1331–1640 | 1640 | E | 19698 | 0 | source reading |
| 60 | S22 | 1641–1990 | 1990 | E | 18368 | 0 | source reading |
| 61 | S22 | 1991–2320 | 2320 | E | 19383 | 0 | source reading |
| 62 | S22 | 2321–2648 | 2648 | E | 57471 | 0 | truncated; later recovery |
| 63 | S22 | 2490–2580 | 2580 | E | 24200 | 24200 | truncation recovery |
| 64 | S22 | 2470–2495 | 2495 | E | 6180 | 6180 | truncation recovery |
| 65 | S23 | 1–240 | 240 | E | 13862 | 0 | source reading |
| 66 | S23 | 241–530 | 530 | E | 18404 | 0 | source reading |
| 67 | S23 | 531–780 | 780 | E | 35501 | 0 | source reading |
| 68 | S23 | 781–1108 | 1108 | E | 24785 | 0 | source reading |
| 69 | S17 | 221–426 | 426 | E | 18709 | 18709 | explicit recovery of uncertain truncation boundary |
| 70 | S18 | 265–285 | 285 | E | 5151 | 5151 | explicit recovery of uncertain truncation boundary |
| 71 | S24 | 1–171 | 171 | E | 19223 | 0 | source reading |
| 72 | S25 | 193–345 | 345 | E | 8067 | 0 | source reading |
| 73 | S25 | 439–473 | 473 | E | 1262 | 0 | source reading |
| 74 | S26 | 1–98 | 98 | E | 6149 | 0 | source reading |
| 75 | S09 | 1–65 | 65 | E | 4130 | 4130 | repeat to check exact dependency applicability |
| 76 | S15 | 1–26 | 26 | E | 1994 | 1994 | repeat dependency applicability |
| 77 | S11 | 1–23 | 23 | E | 2594 | 2594 | repeat dependency applicability |
| 78 | S27 | 1–221 | 221 | E | 11159 | 0 | source reading |
| 79 | S19 | 1–24 | 24 | E | 1978 | 1978 | repeat dependency applicability |
| 80 | S17 | 1–23 | 23 | E | 2084 | 2084 | repeat dependency applicability |
| 81 | S02 | 1–84 | 84 | E | 6532 | 6532 | repeat to verify mandatory governance extent |
| 82 | S21 | 700–780 | 780 | E | 25428 | 25428 | explicit recovery of uncertain truncation boundary |
| 83 | S21 | 831–940 | 940 | E | 9665 | 9665 | explicit recovery of uncertain truncation boundary |
| 84 | S16 | 225–245 | 245 | E | 5405 | 5405 | explicit recovery of uncertain truncation boundary |
| 85 | S05 | 1–22 | 22 | E | 1586 | 1586 | repeat report requirements |
| 86 | S14 | 1–30 | 30 | E | 1970 | 1970 | repeat dependency applicability |
| 87 | S18 | 1–26 | 26 | E | 2371 | 2371 | repeat dependency applicability |
| 88 | S12 | 70–208 | 208 | E | 9329 | 9329 | repeat exact formula/sign/anchor inspection |
| 89 | S13 | 1–113 | 113 | E | 8225 | 8225 | repeat exact tolerances and solve boundary |
| 90 | S22 | 461–510 | 510 | E | 25291 | 25291 | final explicit dense-table truncation recovery |

### Additional operation disclosure

- Metadata only: `tools/agents/find-agents --for` the assigned report path; line-count queries for selected LSE chapters and the four external contracts; `rg -n '^## '` over testing-and-gate-strategy and science-obligations to select required governance sections. These returned instruction routing, sizes or headings, not uncounted whole-source bodies.
- Temporary measurement files: `/tmp/lse_surface_iteration3_measure.py`, `/tmp/lse_surface_iteration3_measure.json`, and `/tmp/lse_surface_iteration3_append.py`. Arithmetic ran with repository `.venv/bin/python`; one script-text replacement used system `python3`, without imports or execution of repository science. No production/test workflow was invoked.
- Local report writing and final source/report hash/link checks are artifact preparation. They do not establish physical results. No canonical contract or another worker artifact was modified.
