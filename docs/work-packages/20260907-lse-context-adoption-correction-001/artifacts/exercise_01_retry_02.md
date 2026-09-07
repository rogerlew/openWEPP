# Snow-free covered forest surface/soil reading — first attempt

Static: independent scientific reading of the frozen current candidate v32 authority. No implementation, simulation, comparator, Rust test, or real-consumer workflow was run. The answer identifies obligations; it does not certify production implementation, qualification, calibration, cutover, or package closure.

## Answer and authority anchors

The snow-free covered tile is one coupled canopy–air–surface–soil problem. Its water amounts remain hydrology-owned immutable inputs during each physical solve. The surface thermal storage branch determines whether there is a physical beginning surface temperature; the first soil node receives exactly the heat that leaves the surface. Current precipitation/canopy release/runon energy enters only after the fixed-authorization solve, through hydrology's actual partition, and cannot retroactively change its accepted turbulent or conductive fluxes.

Anchor abbreviations below are files inside `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/`; external contract names are the current canonical files in their parent directory. Section fragments and invariant IDs are current anchors, not references to the prohibited monolith.

### Owner, regime and basis boundaries

- `surface-energy.md#exact-ownership-and-state`, `soil-coupling.md#INV-LANDSURFACEENERGY-100`, and `interface.md#interface`: hydrology exclusively owns all surface/litter/soil water masses, including liquid/frozen state. LSE owns one surface thermal node per tile. Soil thermal exclusively owns every ordered soil temperature and enthalpy, and vegetation owns canopy components. Neither soil temperature nor water may be duplicated as independent LSE mutable state. Accepted source identity must join exact run/OFE/tile/surface/source/layer/model/configuration/transaction/support lineage.
- `surface-energy.md#selected-sources-and-domain`: V1 is snow-free at both support endpoints, without terminal-snow payload, with unfrozen surface/soil liquid and positive finite neutral wind. It admits bare mineral soil and forest litter with complete configured thermal and hydraulic operands. Snow, frozen/thawing soil or bare surface, calm/nonneutral conditions, or missing domains fail before numerical iteration. Snow-free does not imply that every frozen-litter successor is V1: V3 phase authority is separately selected, and may not silently enter the unfrozen branch. LSE-V2 imports V1 physics and the exact V10 vegetation owner (`solve-boundary.md#INV-LANDSURFACEENERGY-109`).
- Tile storage/fluxes are per tile-ground; LSE “stand-ground” means this OFE's horizontal ground, never hillslope-ground. Close each local tile first and apply `f_t` exactly once to OFE amounts (`water-vapor.md#INV-LANDSURFACEENERGY-107`). SurfaceLiquid requires positive finite tile fractions/capacities/areas, exact production-lane/layer bindings, and one configured `covered_canopy_release` ingress mode for a covered tile. Surface class alone cannot infer canopy exposure.
- The represented-snow boundary in `soil-coupling.md#version-8-persistent-snow--soil-boundary-amendment`, INV-124–126, is a separate OFE/lane bottom-snow-volume → first-soil-node receipt, not tile surface `G_s1`. It cannot be selected, averaged, weighted or duplicated over tiles. Under represented snow the litter/WB14 owners are inactive byte-identical custody (SURFACELIQUID INV-026); the present answer does not claim native snow execution correctness or terminal adoption.

### Surface storage and first-node heat transfer

`soil-coupling.md#surface-humidity-surface-enthalpy-litter-and-soil-heat`, INV-103/106, supplies the exact rule:

```text
T_ref = 273.15 K; C_w = 4218 J kg^-1 K^-1
U_s = (C_dry + W*C_w)*(T_s - T_ref)
```

`W` is the exact hydrology operand and stays immutable during the solve.

| Selected branch | Physical storage/temperature rule | Surface operand in Crank–Nicolson beginning flux |
| --- | --- | --- |
| `finite_capacity` | Require `C_dry+W*C_w>0`. Accepted ending enthalpy is the sole physical surface state. Derive ending temperature with hydrology candidate ending `W`; any retained warm start must be bit-identical to that derived value. | Derive physical beginning `T_s,0` from authoritative beginning `U_s,W,C_dry`. Ending endpoint uses trial ending `T_s`. |
| `equilibrium_zero` | Require `C_dry=W=U_s=0` exactly. Storage difference is exact zero, but `T_s` remains an algebraic energy unknown. No other zero-capacity branch exists. | There is no physical beginning surface temperature: use the **current algebraic trial `T_s` at both endpoints**, against `T_1,0` and `T_1,1`. A caller warm start is never a beginning physical operand. |

For exact ordered soil nodes `k=1..N`, require finite positive `dz_k,lambda_k,C_k` and positive surface `dz_s,lambda_s`:

```text
g_s1 = 2/(dz_s/lambda_s + dz_1/lambda_1)
g_k,k+1 = 2/(dz_k/lambda_k + dz_(k+1)/lambda_(k+1))
G_s1 = g_s1*(T_s-T_1)
G_k,k+1 = g_k,k+1*(T_k-T_(k+1))
bar(G) = (G_begin+G_end)/2
C_1*(T_1,1-T_1,0)/dt = bar(G_s1)-bar(G_1,2)
C_k*(T_k,1-T_k,0)/dt = bar(G_k-1,k)-bar(G_k,k+1)
C_N*(T_N,1-T_N,0)/dt = bar(G_N-1,N)
```

Bottom flux is exact zero. For `N=1`, only `C_1*DeltaT_1/dt=bar(G_s1)` remains. These `G` are positive downward: surface loses `bar(G_s1)` and first soil node gains that same operand once. Do not mistake the generic common-contract signed `G`/consumer `-G` terminology for an additional negative sign in the explicitly positive-downward V1 equations. A separately recomputed transfer, second soil temperature owner, phase-change term or frozen-soil extension is prohibited here.

For forest litter (`soil-coupling.md`, same section),

```text
h_ul = (1-cos(pi*W_l/W_l,max))/2
q_l = h_ul*q_sat(T_s,p) + (1-h_ul)*q_recipient
v_l = rho_a*(q_l-q_recipient)/r_l-c
lambda_l = 0.1 + 0.03*W_l/(rho_w*dz_l)
C_dry = dz_l*rho_ld*c_ld
```

Require `W_l,max>0`, `0<=W_l<=W_l,max`; use `dz_s=dz_l`, `lambda_s=lambda_l`. Litter blocks both direct mineral-soil evaporation and upward capillary supply for the interval. Overflow remains hydrology-owned ingress. A generic “equilibrium” choice cannot discard configured litter capacity.

### Covered energy network and signed vapor

`surface-energy.md#shortwave-and-reciprocal-longwave`, INV-101/102, plus the full current `SC-VEGETATION-001` V7/V8 authority:

- Shortwave retains the complete band-resolved V7 two-stream column and ground VIS/NIR albedo lower boundary. Terminal direct/diffuse flux reaches ground once; reflected shortwave returns through the canopy.
- Longwave uses unit-emissivity/no-reflection ground and component sources, current component temperatures and `tau=exp[-0.8*Omega*(LAI+SAI)]`; ground upward longwave is `sigma*T_s^4`. Each component emission is weighted by its actual emissive area; zero emissive area contributes zero. Prescribed/stale/bulk ground or canopy longwave is not an equivalent boundary.
- `surface-energy.md#neutral-turbulent-heat-and-vapor-network` gives one shared zero-storage `(T_c,q_c)` per covered tile. Ground sensible and signed vapor fluxes are `rho_a*c_p*(T_s-T_c)/r_h` and `rho_a*(q_s-q_c)/r_v`. Canopy residuals are `sum H_j+H_s-H_c->atm=0` and `sum v_j+v_s-v_c->atm=0`. Ground occurs once per tile, not per occupancy. A beneath-canopy reference-air ground bypass, omitted ground term or producer-aggregated canopy total is invalid.
- The ground path is the exact neutral ISBA-MEB 54–63 specialization, `psi_H=f_hv=1`, with `phi_v=2`, `z0g=.007 m`, `chi_L=.12`, `u_l=1 m/s`, `l_w=.02 m`, `nu=1.5e-5 m²/s`, and `kappa=.4`. Configuration must supply `z_hv,z0v,z_ref,LAI`; require `LAI>0`, `z_hv>d+z0v>z0g>0`, `z_ref-d>=z_hv-d>0`, and finite positive logs/exponentials/resistances. Heat and vapor use the same ground-to-canopy resistance as distinct semantic operands. Canopy-reference exchange has configured `d,z0m,z0h,z0q` and the displaced neutral log-law. No wind floor, stability/convective substitute or alternate roughness is admitted.

`water-vapor.md#vapor`, INV-104:

```text
h_l(T) = C_w*(T-T_ref)
L_v(T) = 2.501e6 - 2369*(T-T_ref)
Q_v = v*[h_l(T_s)+L_v(T_s)]
(U_pre-ingress-U_0)/dt = R_sw+R_lw-H_s-Q_v-bar(G_s1)
```

Evaporation is positive away from surface and withdraws only final authorized use. Condensation is negative vapor, with a positive hydrology mass credit and matching accepted temperature/enthalpy. Neither clipping/absolute value nor latent-only energy is valid; condensation cannot vanish at a storage capacity boundary.

### Immutable beginning, fixed authorization, and ingress energy

`water-vapor.md#water`, INV-105; current `SC-WATBAL-001` Stage-B ownership; `SC-SURFACELIQUID-001` algorithm 1–8 and INV-003/004/006–009:

1. Validate and freeze the complete accepted beginning before current rain, runon, throughfall, initial/second drainage, stemflow, infiltration or runoff. Concurrent root and ground demand compete against these same beginning stores. First solve the uncapped coupled potential system, publish exact source-keyed requests, and obtain one hydrology authorization batch.
2. Rebuild from original beginning owners with those source caps fixed; solve the complete gas/energy/shared-air/hydraulic/surface/soil system. `cap<=law` is cap-active, including equality, with zero cap derivative. Final interval use obeys exact `0<=F<=A<=D`; for a tile flux, `F=f_t*q*dt`. Debit `F`, not `A`. Unused authorization remains in storage; no donation, second authorization, current-ingress availability, scalar stress repair, or potential-state continuation is permitted. Exact FullSupply can seed coordinates but must reevaluate the complete fixed-final system from the original owners (INV-110), not accept potential results by identity alone.
3. SurfaceLiquid's source authorization uses canonical key order and both OFE and inverse tile-basis safety predicates. A bounded representational overshoot permits only the named common symmetric downward binary64 scale, at most 64 bit decisions, with every positive row still positive and exact safe sums. No per-row/last-row/priority repair or general clamp is permitted. The explicit envelope is `1e-14 kg/m² + 64*epsilon*(abs(sum)+abs(supply))` in the tested basis; larger inconsistency rejects.
4. After accepted capped physics, finalize start-store debits and signed condensation credits, then admit current ingress exactly once. Covered tiles contribute only accepted canopy throughfall plus initial drainage, second drainage and stemflow; **do not add raw precipitation again**. Add typed upstream runon and any condensation overflow. Every parcel retains exact half-open timing, source/destination and OFE area basis. Same-support ingress cannot change the already accepted `H`, vapor energy or `G_s1`, or supply same-support evapotranspiration.
5. Hydrology runs the actual shared chronological WB14/Green–Ampt transition once per OFE per accepted child, preserving cumulative supply/infiltration and immutable production parameters. It is not a per-parcel or daily scalar partition. Native persistent depression storage replaces legacy depression retention: shadow WB14 depression capacity/delta and WAT5 retention bins are zero. Production soil receives the same-pass infiltration mass once through the bound production transition.
6. Mix all source parcels over each chronological subinterval before partition: `X=sum m`, `Q=sum Q_p`, `h_mix=Q/X` for positive `X` (zero-mass carries exact zero energy). Actual WB14 supplies infiltration `I` and excess `E`, `I+E=X`; `Q_inf=I*h_mix`, `Q_excess=Q-Q_inf`. Preserve proportional source attribution and canonical final-source floating remainders. Retain only post-infiltration excess in its exact tile/source capacity and route the remainder with mass and enthalpy together. Source type may not choose which temperature infiltrates. The specific bounded unrepresentable retained-mass rule routes the entire parcel onward without changing storage; it never deletes mass/energy or rounds storage to capacity.
7. Infiltration energy credits the actual named first receiving soil-thermal layer; retained energy credits the exact LSE tile once after OFE-to-tile conversion. Thus `U_end=U_pre+sum Q_retained` and soil energy gains `sum Q_inf`, with temperature reconstructed from the accepted ending state. Outlet energy is the sum of outlet parcel energies. If native DC01 owns routing, recipient mass and energy both multiply by `A_up/A_down` once and re-key basis; Lane-D-local posture instead gives Lane D sole inter-OFE ownership. Mixed or duplicate routing is invalid.

Rain temperature is the retained accepted Harder–Pomeroy hydrometeor output in Celsius plus 273.15, not a new solve. Runon preserves its exact upstream outlet temperature/specific enthalpy. Canopy releases use their accepted wet-surface temperature. Air/soil/freezing-temperature fallback, guessed temperature, enthalpy-free mass, or mismatched destinations are invalid. Mixture temperature is `T_ref+sum(m*h)/(C_w*sum m)` for positive mass.

One narrow exception must survive a “no normalization” summary: `water-vapor.md#version-9-exact-liquid-reference-state-representation-amendment`, INV-130, canonicalizes only exact bits `0x4071126666666667` (first upward neighbor of `273.15 K`, offset `2^-44 K`) to `0x4071126666666666` before covered-canopy liquid ledger/release or Stage-3 terminal-liquid publication, and persists exact zero specific enthalpy with it. It is not a solver/storage/general temperature tolerance; second-up and other values are not normalized and below-reference values retain existing branch guards.

### Exact accepted energy custody and acceptance

`soil-custody.md#exact-soil`, INV-150, and `surface-custody.md#surface-custody`, INV-151/153, distinguish physical equations from successor storage representation. Soil receiver V2 owns `E_k=exact(H_hi,k)+R_k`; the V16 LSE companion owns per-tile `U_t=exact(U_hi,t)+R_U,t`. Decode each named finite accepted physical amount only **after its existing physical basis conversion**, aggregate beginning exact total plus all canonical soil-internal/top-boundary/infiltration or surface phase-free/fusion/retained-ingress credits using exact integer arithmetic, round once nearest-even to a finite high term, and retain/reconstruct the normalized signed-dyadic remainder exactly. Surface V2/LSE V3 frozen enthalpy fields are bit-identical high mirrors only on that successor. Carry never feeds temperature, phase or flux physics. No rounded producer aggregate/residual/carry, forced ULP, `nextafter`, tolerance repair, rational replacement of physical conversion, discarded sub-ULP credit, zero snap or subnormal flush is allowed. Zero-carry adoption preserves prior bits; production downgrade is forbidden.

All keys, source receipt, ordering, support, model/configuration/schema, transaction/predecessor and restart/checkpoint joins are exact. Parent-local partial children advance exact energy/receipts but retain persistent predecessor markers; only a parent-final child stamps the transaction once. Authenticated unpublished soil beginning is non-owner/non-wire candidate custody; it cannot synthesize owner/restart bytes or install. Final promotion replays original owner plus accumulated operands and seals one complete owner/receipt/restart/install; private trial support cannot be rebound (`soil-custody.md`, INV-155).

`map-custody.md#handoff/#pending`, INV-159/161, preserve physical origin: a pending covered map first validates its physical prefix/custody; outer nonclosure becomes history, dependent-only nonclosure becomes typed rejection, and full closure consumes that same pending prefix once into final private custody. No completed nonfinal promotion, extra final physical map, physical replay fallback, or map publication. The separately admitted snow-free final identity reseal cannot replace this canonical covered-map path. A private validation proof applies only to the exact immutable revision; mutation invalidates it, and restart/external/untrusted boundaries still fully validate.

`terminal-support.md#support` and full `SC-COUPLEDTIME-001` require exact common half-open support/duration bits and the maximum active-participant physical minimum. LSE minimum is `60_000_000_000 ns`, checked before Newton; zero duration is an event, not physics. A structural 1 ns identity does not admit physical stepping. Grid/floor failures cannot be fixed by rounding, scaling a longer result, dropping remainder or sub-floor retries. Stable ordinary supports must accept substantially larger steps. Same canonical solver retries smaller admitted support from immutable beginning; exact-floor failure stays typed. Only complete all-owner parent acceptance publishes; every rejected candidate preserves complete beginning bytes.

## Required inputs and tests; missing evidence

An actual run still needs exact configured owner/topology/model/schema identities; tile fractions and OFE areas; ordered canopy occupancies, LAI/SAI/Omega, bands/directions/albedos and component states; forcing-derived moist-air properties, humidity/pressure/radiation/wind; configured neutral geometry/roughness; explicit surface branch, dry capacity, depth/conductivity and authoritative beginning enthalpy; immutable surface/litter and root-source water, capacities and requests; ordered soil depth/conductivity/areal capacity and beginning temperature/enthalpy; exact support/duration and predecessor receipts; fixed authorizations; complete ingress timing/mass/temperature/enthalpy; and actual immutable WB14 parameters, lane/layer bindings and continuation. None were supplied as numerical fixture operands here. No example numeric result, test success or production readiness is inferred.

Required discriminating vectors are `common-details.md#tests`, `water-vapor.md#v1-invariants-and-independent-fixtures`, each selected chapter's canonical obligations, and full SURFACELIQUID test obligations. They include:

- Covered wet/dry litter; finite-capacity versus equilibrium-zero; alternate caller warm starts proving equilibrium `G_begin` independence; one/multiple soil nodes, zero lower flux, both `G` signs, heterogeneous tile fractions, day/night and zero shortwave, current ground/canopy radiative and shared-air feedback.
- Evaporation/condensation, dry zero source, full/partial authorization and concurrent root/ground scarcity, cap equality, exact D/A/F identity, unused authorization, no current-ingress donation, and full-system fixed-cap re-solve.
- Each covered release type, mutually exclusive raw/covered precipitation, mixed-temperature rain/runon/overflow/infiltration/retention/outlet, unequal OFE area routing, capacity and representational boundaries, one actual WB14 call/child, stateful parent chronology, independent liquid and enthalpy operands.
- Poison duplicate/missing `f_t`, wrong OFE/tile/layer/source/support, ground bypass or duplicate transfer, stale/bulk LW, clipped condensation, latent-only energy, temperature fallback, producer residuals, mutable/duplicated water or soil state, alternative solver/wind floors, frozen/snow unsupported branches, singular/backtracking/iteration/closure failure and atomic rollback at every boundary. Local closure precedes weighted closure. `solve-boundary.md#solve` retains complete residual and step acceptance, deterministic order and first-error precedence; identity/basis/resource inequalities never borrow numerical tolerances. V1 errors are the typed LSEB-E-030–040 families, support failures E-041/042, and successor custody uses its named E-049/050 families.
- Where exact-carry successor custody is under test: positive/negative sub-ULP credits, exact-halfway even/odd ties, adjacent-high crossings, opposite-sign cancellation, multiple same-tile receipts and unequal tiles, subnormal/largest-finite limits and overflow rejection, canonical carry wire poisons, mirror/receipt/restart identity, split restart and exact rollback. The WAT5 soil input recorded by authority is `H_hi=-34315.42154113602` and `Q_inf=-8.0670339832330148e-19 J/m²`; high stays unchanged and carry retains the exact negative credit. The historical p61 support `176400000000000..178200000000000 ns` did **not** retain exact high/credit bits; they must be captured from the unchanged fixture, not invented. Both p61 and native-forest real consumers must persist, restore and advance the successor before adoption can close.

These are required tests, not tests executed in this exercise. Governing authority ranks A0/A1 and applicable A3 above legacy comparator agreement. Unresolved FAIL/HOLD/expected-red/paused EXP-R/PC1/SG1 and successor adoption limits remain in force. Sparse empirical data does not authorize heuristic physics or an empirical-validation claim.

## Reading scope and expansion decisions

Bootstrap consumed the applicable instructions/package/handoff, current canonical entry, interface and routed surface-energy chapter. Physical surface/soil reconstruction required whole soil-coupling, water-vapor and solve-boundary; interface required whole common-details. Soil/surface energy storage required whole soil-custody and surface-custody. Those in turn required physical support (terminal-support dependencies/introduction plus the complete `support` section), and exact physical receipt origin required complete `handoff` and `pending` sections with their attached invariant/obligation rows. Mandatory external single-file extent was honored for **all** current SC-VEGETATION-001, SC-WATBAL-001, SC-SURFACELIQUID-001 and SC-COUPLEDTIME-001, including embedded current-file legacy/history material; no external contract was silently reduced to its cited anchor. Those documents substantially dominate this task's exposure.

Ordinary citations to a process reference do not turn this snow-free physical-rule question into implementation/review of every cited process. I did not cross into native snow physics, terminal receiver execution, frozen-litter phase law implementation, nonlinear V10/V11–13 algorithm eligibility/implementation, dependency-replay optimization correctness, experiment identity/qualification or publication adoption. Accordingly the conditional whole SnowEnergy, full terminal-support, litter-phase, nonlinear-solve, dependency-replay, qualification, audit-details and historical LSE chapter routes were not triggered. Governing common invariants and linked guard/test obligations were retained. Map custody was read to establish the accepted physical operand origin, not to claim native-map solver or optimization correctness. Instructions, role-review, science-obligations, standards measurement/gate extents and the correctness authority model were separate governance exposure. The context measurement utility was read to use its declared-selection schema and executed without a revision/history argument.

Large combined requests and long single-file tables produced actual truncation notices. The ledger retains **all requested ranges**, including unseen portions of truncated requests; recovery ranges are additional exposure, not replacements. Subsequent smaller ranges covered the missing parts, with conservative overlaps around uncertain truncation boundaries. Current sources were not edited. No original monolith, Git history, archive/predecessor candidate, other exercise/report/review/rubric/intake/prediction, parent test output or agent answer was read. No science hints were requested.

## Structural source measurement

Measured with `.venv/bin/python tools/agents/context_report.py /tmp/surface_selection.json --root /workdir/openWEPP` against the frozen working-tree files. The tool ran; no science workflow ran. Inclusive line ranges count complete original UTF-8 source lines with original line endings, excluding tool wrappers, prefixes and truncation markers. Heading searches count only matched source lines. Hashes identify each full source, even for partial reads. `R` numbers below are ledger identifiers, not a claim about tool-call numbering. The ledger retains repeated requests and recovery overlaps. A request beyond EOF counts only extant source bytes (soil-custody request180–260 ends201; solve-boundary request1–115 ends113).

| Material | Bootstrap unique bytes | Bootstrap requested exposure bytes | Expansion unique bytes | Expansion requested exposure bytes | Combined unique bytes | Total requested exposure bytes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| governance | 29,572 | 29,572 | 35,599 | 36,069 | 63,585 | 65,641 |
| LSE | 23,175 | 23,175 | 131,524 | 214,434 | 150,614 | 237,609 |
| external | 0 | 0 | 903,134 | 1,243,861 | 903,134 | 1,243,861 |
| tooling | 0 | 0 | 3,544 | 3,544 | 3,544 | 3,544 |
| Total, disjoint material categories | — | — | — | — | 1,120,877 | 1,550,655 |

Repeat/recovery exposure above the unique union is **429,778 bytes**. This is not delivered text or model tokens. Delivered tokens, hidden/automatic context bytes, workflow total, and causal context-reduction savings are **UNOBSERVED**. No token estimate or token-to-byte conversion is used. No baseline-monolith comparison was made. Filesystem searches, line counts, hash/measurement execution, tool-help output, task messages and compaction summaries are not source-file selected bytes and have no observed token/workflow total. Source-selection counts do not imply that truncated text was delivered; complete reading relies on the explicitly retained recovery requests.

### Full-source identities

| ID | Path | Full lines | Full UTF-8 bytes | Full-source SHA-256 |
| --- | --- | ---: | ---: | --- |
| S01 | `AGENTS.md` | 121 | 9508 | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` |
| S02 | `docs/specifications/science-contracts/AGENTS.md` | 84 | 6532 | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` |
| S03 | `docs/work-packages/AGENTS.md` | 105 | 6642 | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` |
| S04 | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | 75 | 5304 | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` |
| S05 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | 22 | 1586 | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` |
| S06 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | 65 | 4130 | `930c682e5c923083683e8a030457c9e5e90e6ca9cd2f70a425ee0aa79c326c72` |
| S07 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | 66 | 7976 | `0335d3b03acab1bdd711b3be589b429dcd63b7023e516561e269700006fbb2ff` |
| S08 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | 203 | 11069 | `3adaa50cb49f8f2895a481df4c2be6d87311b68a262d3adef5c2638c1aca025f` |
| S09 | `docs/work-packages/role-review.md` | 7 | 1400 | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` |
| S10 | `docs/standards/AGENTS.md` | 58 | 4052 | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` |
| S11 | `docs/standards/prompt-wording-guidance.md` | 202 | 12057 | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` |
| S12 | `docs/standards/testing-and-gate-strategy.md` | 494 | 25142 | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` |
| S13 | `docs/work-packages/science-obligations.md` | 98 | 6149 | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` |
| S14 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | 208 | 13249 | `582c8591001247342a1a0fa919dcd6e3cd267f03ceec04101c87890a6b0ec094` |
| S15 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | 113 | 8225 | `fe37a1a64327aa46ffb1afbc63e00aedab67c83083a3eb3ccbd2edbc17d3174e` |
| S16 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | 303 | 22406 | `7c83a0a492c5b6c4c8c8588f9ebd592b83f1f7955c99a812cbcdbad76f5ad037` |
| S17 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | 227 | 15924 | `7d7e0816257f51d2878450c7cbf5cb5d69e9e5fb061c0a49f52bae5a0beef3ce` |
| S18 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | 201 | 15484 | `3585c1a3713dfe4224fd03f72a88133530d1e14810b34ec5587c05aab2dc5922` |
| S19 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | 290 | 25619 | `a3169e84ce8e828dc0b51c6fad51d558860f0a51692cfe72975a9c61cc20fc03` |
| S20 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | 245 | 20940 | `9eb3f77d55de9ed404f89bff9c313194780dd816bb43fb087b7e4927ca0e0007` |
| S21 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | 3080 | 248829 | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` |
| S22 | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | 2648 | 377637 | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` |
| S23 | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | 2205 | 184116 | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` |
| S24 | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | 1108 | 92552 | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` |
| S25 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | 171 | 19223 | `4778df5b572c4a09b3dfedc1f5bba4c84d376bcfcd0021ae7e9fbb6200158b8c` |
| S26 | `docs/specifications/correctness-authority-model.md` | 221 | 11159 | `64275bad708f353c2c3b66f5b1475c070254591e26278d0a4ec442cad6e6eb0f` |
| S27 | `tools/agents/context_report.py` | 79 | 3544 | `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde` |

### Every requested source range

Each source ID resolves to the exact path and full-source hash above. All rows are one exposure of the shown inclusive range. Repeated/overlapping rows count again. Searches emit one row per matched source line; search prefixes and unselected lines are excluded.

| Request | Source | Inclusive range | Requested UTF-8 bytes | Phase | Reason / recovery |
| --- | --- | --- | ---: | --- | --- |
| R001 | S01 | 1–121 | 9508 | bootstrap | instructions/package/assignment bootstrap |
| R002 | S02 | 1–84 | 6532 | bootstrap | instructions/package/assignment bootstrap |
| R003 | S03 | 1–105 | 6642 | bootstrap | instructions/package/assignment bootstrap |
| R004 | S04 | 1–75 | 5304 | bootstrap | instructions/package/assignment bootstrap |
| R005 | S05 | 1–22 | 1586 | bootstrap | instructions/package/assignment bootstrap |
| R006 | S06 | 1–65 | 4130 | bootstrap | canonical entry routing |
| R007 | S07 | 1–66 | 7976 | bootstrap | full governed read |
| R008 | S08 | 1–203 | 11069 | bootstrap | full governed read |
| R009 | S09 | 1–7 | 1400 | expansion | full governed read |
| R010 | S10 | 1–58 | 4052 | expansion | full governed read |
| R011 | S11 | 1–1 | 26 | expansion | heading/anchor search exposure |
| R012 | S11 | 7–7 | 12 | expansion | heading/anchor search exposure |
| R013 | S11 | 174–174 | 24 | expansion | heading/anchor search exposure |
| R014 | S11 | 195–195 | 22 | expansion | heading/anchor search exposure |
| R015 | S12 | 1–1 | 34 | expansion | heading/anchor search exposure |
| R016 | S12 | 8–8 | 14 | expansion | heading/anchor search exposure |
| R017 | S12 | 17–17 | 17 | expansion | heading/anchor search exposure |
| R018 | S12 | 33–33 | 35 | expansion | heading/anchor search exposure |
| R019 | S12 | 52–52 | 17 | expansion | heading/anchor search exposure |
| R020 | S12 | 77–77 | 30 | expansion | heading/anchor search exposure |
| R021 | S12 | 129–129 | 25 | expansion | heading/anchor search exposure |
| R022 | S12 | 131–131 | 18 | expansion | heading/anchor search exposure |
| R023 | S12 | 137–137 | 26 | expansion | heading/anchor search exposure |
| R024 | S12 | 164–164 | 28 | expansion | heading/anchor search exposure |
| R025 | S12 | 172–172 | 25 | expansion | heading/anchor search exposure |
| R026 | S12 | 182–182 | 30 | expansion | heading/anchor search exposure |
| R027 | S12 | 193–193 | 28 | expansion | heading/anchor search exposure |
| R028 | S12 | 219–219 | 26 | expansion | heading/anchor search exposure |
| R029 | S12 | 224–224 | 14 | expansion | heading/anchor search exposure |
| R030 | S12 | 230–230 | 22 | expansion | heading/anchor search exposure |
| R031 | S12 | 235–235 | 22 | expansion | heading/anchor search exposure |
| R032 | S12 | 240–240 | 13 | expansion | heading/anchor search exposure |
| R033 | S12 | 274–274 | 44 | expansion | heading/anchor search exposure |
| R034 | S12 | 292–292 | 26 | expansion | heading/anchor search exposure |
| R035 | S12 | 314–314 | 42 | expansion | heading/anchor search exposure |
| R036 | S12 | 346–346 | 25 | expansion | heading/anchor search exposure |
| R037 | S12 | 368–368 | 39 | expansion | heading/anchor search exposure |
| R038 | S12 | 386–386 | 17 | expansion | heading/anchor search exposure |
| R039 | S12 | 403–403 | 34 | expansion | heading/anchor search exposure |
| R040 | S12 | 417–417 | 35 | expansion | heading/anchor search exposure |
| R041 | S12 | 427–427 | 46 | expansion | heading/anchor search exposure |
| R042 | S12 | 439–439 | 33 | expansion | heading/anchor search exposure |
| R043 | S12 | 457–457 | 26 | expansion | heading/anchor search exposure |
| R044 | S12 | 474–474 | 18 | expansion | heading/anchor search exposure |
| R045 | S12 | 487–487 | 18 | expansion | heading/anchor search exposure |
| R046 | S13 | 1–1 | 22 | expansion | heading/anchor search exposure |
| R047 | S13 | 5–5 | 52 | expansion | heading/anchor search exposure |
| R048 | S13 | 52–52 | 30 | expansion | heading/anchor search exposure |
| R049 | S13 | 80–80 | 46 | expansion | heading/anchor search exposure |
| R050 | S11 | 174–194 | 1333 | expansion | full governed read |
| R051 | S12 | 193–345 | 8067 | expansion | full governed read |
| R052 | S12 | 439–473 | 1262 | expansion | full governed read |
| R053 | S13 | 1–98 | 6149 | expansion | full governed read |
| R054 | S14 | 1–208 | 13249 | expansion | combined request truncated; covered/recovered below |
| R055 | S15 | 1–113 | 8225 | expansion | combined request truncated; covered/recovered below |
| R056 | S16 | 1–303 | 22406 | expansion | combined request truncated; covered/recovered below |
| R057 | S17 | 1–227 | 15924 | expansion | combined request truncated; covered/recovered below |
| R058 | S16 | 1–303 | 22406 | expansion | truncation recovery, repeat |
| R059 | S15 | 1–113 | 8225 | expansion | combined request truncated; covered/recovered below |
| R060 | S18 | 1–201 | 15484 | expansion | combined request truncated; covered/recovered below |
| R061 | S19 | 1–290 | 25619 | expansion | combined request truncated; covered/recovered below |
| R062 | S19 | 1–170 | 10977 | expansion | truncation recovery |
| R063 | S18 | 180–201 | 4376 | expansion | truncation recovery; request endpoint260 exceeds EOF201, measured through EOF |
| R064 | S20 | 3–3 | 16 | expansion | heading/anchor search exposure |
| R065 | S20 | 16–16 | 19 | expansion | heading/anchor search exposure |
| R066 | S20 | 20–20 | 49 | expansion | heading/anchor search exposure |
| R067 | S20 | 53–53 | 59 | expansion | heading/anchor search exposure |
| R068 | S20 | 64–64 | 35 | expansion | heading/anchor search exposure |
| R069 | S20 | 78–78 | 20 | expansion | heading/anchor search exposure |
| R070 | S20 | 90–90 | 21 | expansion | heading/anchor search exposure |
| R071 | S20 | 92–92 | 56 | expansion | heading/anchor search exposure |
| R072 | S20 | 177–177 | 54 | expansion | heading/anchor search exposure |
| R073 | S20 | 226–226 | 24 | expansion | heading/anchor search exposure |
| R074 | S20 | 242–242 | 25 | expansion | heading/anchor search exposure |
| R075 | S20 | 1–18 | 1728 | expansion | support dependencies/intro and complete selected support section (anchor176 recovered later) |
| R076 | S20 | 90–175 | 5776 | expansion | support dependencies/intro and complete selected support section (anchor176 recovered later) |
| R077 | S21 | 1–400 | 33697 | expansion | full external read |
| R078 | S21 | 401–750 | 23016 | expansion | full external read |
| R079 | S21 | 751–1150 | 45678 | expansion | truncated |
| R080 | S21 | 990–1040 | 14866 | expansion | truncation recovery |
| R081 | S21 | 1151–1450 | 26602 | expansion | truncated combined request |
| R082 | S21 | 975–1000 | 6778 | expansion | truncation recovery |
| R083 | S21 | 1195–1270 | 4574 | expansion | truncation recovery |
| R084 | S21 | 1451–1950 | 43700 | expansion | full external read |
| R085 | S21 | 1951–2500 | 35270 | expansion | full external read |
| R086 | S21 | 2501–3080 | 40866 | expansion | full external read |
| R087 | S22 | 1–450 | 155593 | expansion | truncated full-external first chunk |
| R088 | S22 | 220–320 | 78432 | expansion | truncated recovery |
| R089 | S22 | 199–294 | 76794 | expansion | truncated recovery |
| R090 | S22 | 245–271 | 25201 | expansion | recovery |
| R091 | S22 | 295–357 | 25810 | expansion | recovery |
| R092 | S22 | 175–220 | 12919 | expansion | recovery combined |
| R093 | S22 | 448–501 | 25750 | expansion | truncated recovery combined |
| R094 | S22 | 502–588 | 25986 | expansion | external read |
| R095 | S22 | 589–841 | 25831 | expansion | external read |
| R096 | S22 | 842–1224 | 25986 | expansion | external read |
| R097 | S22 | 1225–1666 | 25944 | expansion | external read |
| R098 | S22 | 1667–2135 | 25976 | expansion | external read |
| R099 | S22 | 2136–2465 | 25801 | expansion | external read |
| R100 | S22 | 272–294 | 25818 | expansion | combined recovery truncated |
| R101 | S22 | 358–447 | 25789 | expansion | combined recovery truncated |
| R102 | S22 | 460–465 | 2791 | expansion | combined recovery tail visible |
| R103 | S22 | 358–401 | 12788 | expansion | recovery |
| R104 | S22 | 292–294 | 4439 | expansion | recovery |
| R105 | S22 | 402–447 | 13001 | expansion | recovery |
| R106 | S22 | 2466–2565 | 25772 | expansion | combined tail truncated |
| R107 | S22 | 2470–2500 | 7666 | expansion | truncation recovery |
| R108 | S22 | 2566–2648 | 16432 | expansion | external tail |
| R109 | S23 | 1–446 | 25932 | expansion | complete full-external chunk |
| R110 | S23 | 447–709 | 25090 | expansion | complete full-external chunk |
| R111 | S23 | 710–834 | 25982 | expansion | complete full-external chunk |
| R112 | S23 | 835–1235 | 25947 | expansion | complete full-external chunk |
| R113 | S23 | 1236–1568 | 25862 | expansion | complete full-external chunk |
| R114 | S23 | 1569–1822 | 25969 | expansion | complete full-external chunk |
| R115 | S23 | 1823–2163 | 25977 | expansion | complete full-external chunk |
| R116 | S23 | 2164–2205 | 3357 | expansion | complete full-external chunk |
| R117 | S24 | 1–437 | 25976 | expansion | complete full-external chunk |
| R118 | S24 | 438–620 | 25560 | expansion | complete full-external chunk |
| R119 | S24 | 621–944 | 25570 | expansion | complete full-external chunk |
| R120 | S24 | 945–1108 | 15446 | expansion | complete full-external chunk |
| R121 | S25 | 1–30 | 2131 | expansion | dependency/intro search expansion for physical receipt origin |
| R122 | S18 | 150–190 | 2928 | expansion | conservative truncation-gap recovery |
| R123 | S20 | 176–176 | 64 | expansion | selected section terminal anchor boundary |
| R124 | S25 | 3–3 | 16 | expansion | heading/anchor search exposure |
| R125 | S25 | 14–14 | 25 | expansion | heading/anchor search exposure |
| R126 | S25 | 15–15 | 14 | expansion | heading/anchor search exposure |
| R127 | S25 | 18–18 | 26 | expansion | heading/anchor search exposure |
| R128 | S25 | 19–19 | 63 | expansion | heading/anchor search exposure |
| R129 | S25 | 20–20 | 53 | expansion | heading/anchor search exposure |
| R130 | S25 | 24–24 | 33 | expansion | heading/anchor search exposure |
| R131 | S25 | 25–25 | 24 | expansion | heading/anchor search exposure |
| R132 | S25 | 36–36 | 21 | expansion | heading/anchor search exposure |
| R133 | S25 | 37–37 | 63 | expansion | heading/anchor search exposure |
| R134 | S25 | 38–38 | 53 | expansion | heading/anchor search exposure |
| R135 | S25 | 88–88 | 21 | expansion | heading/anchor search exposure |
| R136 | S25 | 89–89 | 58 | expansion | heading/anchor search exposure |
| R137 | S25 | 90–90 | 48 | expansion | heading/anchor search exposure |
| R138 | S25 | 157–157 | 34 | expansion | heading/anchor search exposure |
| R139 | S25 | 158–158 | 24 | expansion | heading/anchor search exposure |
| R140 | S25 | 161–161 | 2299 | expansion | heading/anchor search exposure |
| R141 | S25 | 162–162 | 1564 | expansion | heading/anchor search exposure |
| R142 | S25 | 163–163 | 765 | expansion | heading/anchor search exposure |
| R143 | S25 | 165–165 | 35 | expansion | heading/anchor search exposure |
| R144 | S25 | 166–166 | 25 | expansion | heading/anchor search exposure |
| R145 | S25 | 169–169 | 768 | expansion | heading/anchor search exposure |
| R146 | S25 | 170–170 | 585 | expansion | heading/anchor search exposure |
| R147 | S25 | 171–171 | 605 | expansion | heading/anchor search exposure |
| R148 | S25 | 36–171 | 16567 | expansion | handoff/pending entire sections plus linked invariant/obligation rows |
| R149 | S26 | 1–221 | 11159 | expansion | testing strategy7 required authority ranking |
| R150 | S27 | 1–79 | 3544 | expansion | utility schema needed for mandated measurement |
| R151 | S18 | 1–22 | 1902 | expansion | dependency extent check repeat |
| R152 | S21 | 1185–1210 | 1627 | expansion | conservative truncation-gap recovery |
| R153 | S08 | 3–3 | 16 | expansion | heading/anchor search exposure |
| R154 | S08 | 16–16 | 28 | expansion | heading/anchor search exposure |
| R155 | S08 | 17–17 | 17 | expansion | heading/anchor search exposure |
| R156 | S08 | 20–20 | 62 | expansion | heading/anchor search exposure |
| R157 | S08 | 21–21 | 54 | expansion | heading/anchor search exposure |
| R158 | S08 | 27–27 | 41 | expansion | heading/anchor search exposure |
| R159 | S08 | 28–28 | 32 | expansion | heading/anchor search exposure |
| R160 | S08 | 45–45 | 39 | expansion | heading/anchor search exposure |
| R161 | S08 | 46–46 | 30 | expansion | heading/anchor search exposure |
| R162 | S08 | 81–81 | 47 | expansion | heading/anchor search exposure |
| R163 | S08 | 82–82 | 38 | expansion | heading/anchor search exposure |
| R164 | S08 | 122–122 | 54 | expansion | heading/anchor search exposure |
| R165 | S08 | 123–123 | 45 | expansion | heading/anchor search exposure |
| R166 | S08 | 189–189 | 19 | expansion | heading/anchor search exposure |
| R167 | S08 | 190–190 | 10 | expansion | heading/anchor search exposure |
| R168 | S08 | 198–198 | 34 | expansion | heading/anchor search exposure |
| R169 | S08 | 199–199 | 24 | expansion | heading/anchor search exposure |
| R170 | S08 | 202–202 | 456 | expansion | heading/anchor search exposure |
| R171 | S08 | 203–203 | 454 | expansion | heading/anchor search exposure |
| R172 | S14 | 3–3 | 16 | expansion | heading/anchor search exposure |
| R173 | S14 | 15–15 | 27 | expansion | heading/anchor search exposure |
| R174 | S14 | 16–16 | 16 | expansion | heading/anchor search exposure |
| R175 | S14 | 19–19 | 68 | expansion | heading/anchor search exposure |
| R176 | S14 | 20–20 | 62 | expansion | heading/anchor search exposure |
| R177 | S14 | 125–125 | 64 | expansion | heading/anchor search exposure |
| R178 | S14 | 126–126 | 54 | expansion | heading/anchor search exposure |
| R179 | S14 | 193–193 | 34 | expansion | heading/anchor search exposure |
| R180 | S14 | 194–194 | 24 | expansion | heading/anchor search exposure |
| R181 | S14 | 197–197 | 450 | expansion | heading/anchor search exposure |
| R182 | S14 | 198–198 | 446 | expansion | heading/anchor search exposure |
| R183 | S14 | 199–199 | 442 | expansion | heading/anchor search exposure |
| R184 | S14 | 200–200 | 365 | expansion | heading/anchor search exposure |
| R185 | S14 | 201–201 | 382 | expansion | heading/anchor search exposure |
| R186 | S14 | 202–202 | 376 | expansion | heading/anchor search exposure |
| R187 | S14 | 204–204 | 35 | expansion | heading/anchor search exposure |
| R188 | S14 | 205–205 | 25 | expansion | heading/anchor search exposure |
| R189 | S14 | 208–208 | 403 | expansion | heading/anchor search exposure |
| R190 | S17 | 3–3 | 16 | expansion | heading/anchor search exposure |
| R191 | S17 | 16–16 | 25 | expansion | heading/anchor search exposure |
| R192 | S17 | 17–17 | 14 | expansion | heading/anchor search exposure |
| R193 | S17 | 20–20 | 19 | expansion | heading/anchor search exposure |
| R194 | S17 | 21–21 | 46 | expansion | heading/anchor search exposure |
| R195 | S17 | 22–22 | 37 | expansion | heading/anchor search exposure |
| R196 | S17 | 65–65 | 19 | expansion | heading/anchor search exposure |
| R197 | S17 | 66–66 | 71 | expansion | heading/anchor search exposure |
| R198 | S17 | 67–67 | 62 | expansion | heading/anchor search exposure |
| R199 | S17 | 118–118 | 20 | expansion | heading/anchor search exposure |
| R200 | S17 | 119–119 | 44 | expansion | heading/anchor search exposure |
| R201 | S17 | 120–120 | 35 | expansion | heading/anchor search exposure |
| R202 | S17 | 146–146 | 52 | expansion | heading/anchor search exposure |
| R203 | S17 | 147–147 | 43 | expansion | heading/anchor search exposure |
| R204 | S17 | 171–171 | 77 | expansion | heading/anchor search exposure |
| R205 | S17 | 172–172 | 67 | expansion | heading/anchor search exposure |
| R206 | S17 | 209–209 | 34 | expansion | heading/anchor search exposure |
| R207 | S17 | 210–210 | 24 | expansion | heading/anchor search exposure |
| R208 | S17 | 213–213 | 459 | expansion | heading/anchor search exposure |
| R209 | S17 | 214–214 | 391 | expansion | heading/anchor search exposure |
| R210 | S17 | 215–215 | 426 | expansion | heading/anchor search exposure |
| R211 | S17 | 216–216 | 395 | expansion | heading/anchor search exposure |
| R212 | S17 | 218–218 | 35 | expansion | heading/anchor search exposure |
| R213 | S17 | 219–219 | 25 | expansion | heading/anchor search exposure |
| R214 | S17 | 222–222 | 439 | expansion | heading/anchor search exposure |
| R215 | S17 | 223–223 | 397 | expansion | heading/anchor search exposure |
| R216 | S17 | 224–224 | 437 | expansion | heading/anchor search exposure |
| R217 | S17 | 225–225 | 448 | expansion | heading/anchor search exposure |
| R218 | S17 | 226–226 | 370 | expansion | heading/anchor search exposure |
| R219 | S17 | 227–227 | 410 | expansion | heading/anchor search exposure |
| R220 | S17 | 172–208 | 2055 | expansion | reference-state precise exception extraction repeat |
| R221 | S14 | 1–124 | 6182 | expansion | equilibrium endpoint formula extraction repeat |
| R222 | S05 | 1–22 | 1586 | expansion | report requirements recheck repeat |
| R223 | S08 | 123–197 | 2659 | expansion | covered turbulent inputs extraction repeat |
| R224 | S15 | 1–113 | 8225 | expansion | physical boundary scope extraction repeat; requested115, EOF113 |

The report is complete as a first-attempt answer. Any later parent assessment is separate evidence; no parent hints or answer-repair cycle supplied this reading.
