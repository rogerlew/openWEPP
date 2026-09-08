# Surface/soil requirements — isolated exercise 01, retry 08

Static: requirements review of the frozen candidate source assigned as commit `645b816b5`. Candidate LSE v32 remains adoption-pending. No implementation, production run, comparator, solver, scientific test, or acceptance campaign was executed. The report is this reader's first completed answer; it does not assert that a described evidence obligation has been satisfied.

The answer is that a snow-free covered forest tile needs a jointly solved canopy–ground–soil energy system, with hydrology owning its immutable beginning water supply and its eventual water partition. “Forest” does not choose a thermal-capacity branch, authorize a bulk prescribed surface temperature, or make current rain available to evaporation. First-node conduction is an equal-and-opposite internal transfer; infiltration sensible heat is a separate, later first-node credit.

## Exact authority and scope

`LSE/` below means `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/`; external `SC-*` names mean files in its parent `contracts/` directory. Source identities and every read are recorded below.

The entry `SC-LANDSURFACEENERGY-001.md#reading-routes` selects the surface/soil science-review route, requires the entire `interface.md#interface`, then selected chapters' introductions, dependencies, definitions, guards and tests. Its directory inventory is not permission to ignore conditional dependencies. The surface route expanded into the complete physical shared set and the selected custody and numerical mechanisms. None of these requirements supersedes the scientific owners in the external contracts.

The immediate owner rules are `LSE/soil-coupling.md#INV-LANDSURFACEENERGY-100`, `#INV-LANDSURFACEENERGY-103`, `#INV-LANDSURFACEENERGY-106`, `LSE/interface.md#interface`, and `LSE/water-vapor.md#immutable-beginning-water-transaction-and-current-ingress`:

- LSE owns the tile surface thermal state. Hydrology owns all surface/litter, soil-layer and root-withdrawal water masses; LSE cannot manufacture a separately mutable water reservoir.
- Soil thermal owns the ordered soil-node temperatures and enthalpies. The surface node does not replace the first soil node. Vegetation owns canopy physiology and component states; atmosphere/forcing and exact coupled support come from their respective sealed owners.
- The covered tile uses current vegetation/radiation, canopy component temperatures and one shared zero-storage canopy-air temperature/humidity balance. One atmosphere exchange joins the full covered system; separate independent ground/canopy atmosphere fluxes or an averaged prescribed canopy temperature are not an equivalent interface.
- The base snow-free, liquid/unfrozen evaluator admits its declared single bare/litter surface class, positive finite physical support, neutral-transfer geometry and wind, complete configuration and lineage. Calm or nonneutral conditions, frozen soil/thaw branches, represented snow, multiple undeclared surface classes, and missing inputs require their own admitted authority or typed rejection. They do not authorize defaults. Frozen litter is a separately admitted amendment, not permission to run a frozen-soil model.
- Tile fractions are positive, unique and normalized on the declared OFE-ground basis; tile-to-OFE conversion occurs exactly once. OFE or hillslope areas cannot be exchanged silently.

The generic ledger uses inward-positive energy. The specific coupling convention is `G_s1 > 0` downward: subtract it from surface energy and add it to soil energy. Thus the generic invariant “surface `G` equals soil `-G`” and the specific surface `-G_s1` / soil `+G_s1` equations agree. Naming the sign convention matters.

## Surface storage, humidity and first-node conduction

The exact branch definitions and equations are in `LSE/soil-coupling.md#surface-humidity-surface-enthalpy-litter-and-soil-heat` and its `INV-LANDSURFACEENERGY-100/103/106` bindings.

For the liquid branch,

```
T_ref = 273.15 K
U_s = (C_dry + W*C_w)*(T_s - T_ref)
C_w = 4218 J kg^-1 K^-1.
```

`W` is the exact hydrology operand. A finite-capacity node requires strictly positive effective capacity; ending enthalpy is authoritative and temperature is reconstructed using the matching hydrology candidate ending mass. A caller's temperature is only a numerical warm start. Changing it cannot change authoritative beginning state or the accepted physical result merely by changing the presumed initial storage.

`equilibrium_zero` is the exact branch `C_dry = W = U_s = 0`: no surface storage change exists and `T_s` is algebraic. It is not an epsilon-sized heat capacity, a “nearly empty” bucket, or an inherited beginning temperature. Branch selection must be made from the exact governed state, not forest cover, air temperature or numerical convenience. The ending storage identity must still be checked after ingress; ingress cannot provide the beginning-state operands of the already accepted physical solve.

For admitted liquid litter the cosine water-availability humidity relation is

```
h_ul = 0.5*(1 - cos(pi*W/W_max))
q_l  = h_ul*q_sat(T_l) + (1-h_ul)*q_recipient
v_l  = rho_air*(q_l-q_recipient)/r_v
lambda_l = 0.1 + 0.03*W/(rho_w*dz_l)
C_dry = dz_l*rho_l,dry*c_l,dry.
```

`W_max > 0`, the admitted water bounds, and positive layer geometry/conductivity are preconditions, not clamping opportunities. Litter blocks underlying mineral-soil evaporation and upward capillary supply in this regime; overflow is hydrology-owned. Dry or low-capacity litter is not a reason to substitute the bare-soil evaporation branch.

For ordered soil nodes, the surface-to-first-node and interior interface conductances use half-layer series resistance:

```
g_s1 = 2/(dz_s/lambda_s + dz_1/lambda_1)
G_s1 = g_s1*(T_s-T_1)
bar(G) = 0.5*(G_begin+G_end).
```

The analogous interior conductance uses adjacent soil layers. Required depths, conductivities and soil heat capacities are positive finite; there is at least one ordered soil node. Soil Crank–Nicolson storage is incoming minus outgoing interface heat. The first node receives `+bar(G_s1)`; the last node has the declared zero lower heat-flux boundary. For one soil node, there is no fabricated second-node flux: the first-node equation contains the surface input and zero lower boundary.

For finite capacity, beginning `T_s` is reconstructed from authoritative beginning `U_s,W,C_dry`, and the ending flux uses the candidate ending temperature. For `equilibrium_zero`, the *current algebraic trial* `T_s` is used at both surface endpoints:

```
G_s1,begin = g_s1*(T_s-T_1,0)
G_s1,end   = g_s1*(T_s-T_1,1).
```

Putting a caller warm start into `G_s1,begin` is expressly wrong. Solving ground first and then attaching a stale soil flux, using a prescribed soil/surface temperature, copying surface enthalpy into soil, or crediting conduction twice violates the coupled owner equations.

## Covered energy and signed water transport

`LSE/surface-energy.md#surface-energy`, the complete current `SC-VEGETATION-001`, and `LSE/water-vapor.md#signed-vapor-and-liquid-enthalpy` govern the physical operands. Shortwave uses the admitted vegetation VIS/NIR two-stream treatment with its current lower boundary; longwave uses current component temperatures and the layer recurrences, including emission. Zero-area canopy components contribute no physical flux. A stale or bulk temperature, duplicated reflected shortwave, or an independent ground radiation shortcut is not authorized.

Sensible and vapor exchange use the declared neutral resistances and current ground/recipient states. Signed vapor must remain signed: evaporation removes mass and energy; condensation adds both. With liquid specific enthalpy and vaporization enthalpy,

```
h_l(T) = C_w*(T-T_ref)
L_v(T) = 2.501e6 - 2369*(T-T_ref)  [J kg^-1]
Q_v    = v_s*(h_l(T_s)+L_v(T_s)) [W m^-2, outward positive]
(U_pre-U_0)/dt = R_sw + R_lw - H_s - Q_v - G_s1.
```

The complete vapor energy includes liquid sensible enthalpy and latent energy. Taking an absolute value, clipping condensation, booking latent energy only, or debiting the water once in hydrology and again through an ET alias is prohibited. `U_pre` uses the finalized pre-ingress hydrology mass; every physical term is evaluated at the accepted trial state and enters once. The canopy's transpiration energy is likewise governed by its vapor enthalpy and actual use, not a duplicate ground latent debit.

## The required chronology: one authorization, then ingress

`LSE/water-vapor.md#immutable-beginning-water-transaction-and-current-ingress` is decisive:

1. Seal beginning vegetation, LSE, soil-thermal and hydrology state before this interval's precipitation, runon, throughfall, canopy drainage, stemflow or litter overflow. Only beginning stores may supply current root uptake, surface/litter evaporation and soil-layer evaporation.
2. Solve the complete potential canopy–ground system from that beginning state. Publish source-specific root, surface/litter and soil-layer requests with transaction, support, OFE, tile, occupancy, source/layer and area-basis identities.
3. Hydrology authorizes the same-snapshot requests once. Rebuild the complete system from the original beginning states using fixed source-specific caps. At `cap_rate <= q_law`, equality belongs to the capped branch with zero generalized derivative. Gas exchange, canopy energy, shared air, hydraulics, surface energy and soil thermal state re-solve together.
4. Independently check actual use `F = f_tile*q*dt` against `0 <= F <= A <= D`. Authorization is a ceiling, not actual use. A changed final canopy release cannot alter authorization. No second authorization, current-ingress donation, scalar stress adjustment, or continuation from mutated potential owner state is allowed.
5. After the final capped solve accepts, hydrology applies actual beginning-store debits and explicit condensation credits. It then accepts final current ingress and partitions it once among retained surface/litter water, infiltration, routed runoff and outlet runoff.
6. Retained ingress sensible heat credits surface enthalpy; infiltration sensible heat credits *soil node 1*; routed runoff carries the accepted parcel mass and heat to the downstream OFE; outlet runoff exports them. Derive ending temperatures from the credited owners. Current ingress cannot feed the already accepted same-interval sensible, vapor or ground-conduction evaluation.

Potential and final passes retain the same operator ordering; only the final pass constructs publishable owner candidates and performs the once-only ingress partition. This is not a second supply authorization.

For a parcel `Q=m*h_l(T)`. Mixing is conservative:

```
T_mix = T_ref + sum(m_i*h_i)/(C_w*sum(m_i))
U_s,1      = U_pre + sum(Q_retained_ingress)
E_soil,1,1 = E_soil,1,pre + sum(Q_infiltration).
```

Zero mass has zero energy and no temperature. Runon requires the accepted upstream typed temperature/enthalpy; air, soil, freezing-point or downstream temperature substitution is forbidden. Transfers must keep the declared source and destination area bases, including the real upstream/downstream area conversion, rather than equating incompatible intensive quantities.

Rain temperature is the exact retained `hydrometeor_temperature_c + 273.15` output of `openwepp_meteorology::phase::hydrometeor_temperature_from_relative_humidity` under active `harder_pomeroy_hourly`, `SC-SNOWFREEZE-001#INV-SNOWFREEZE-075`. LSE does not transcribe or rerun part of that provider. Provider review required the complete external contract: its humidity-domain, phase/mass and typed-failure rules remain binding. Covered-ground canopy release instead carries accepted canopy wet-component source temperature/enthalpy under `INV-VEGETATION-114`; raw precipitation cannot be routed around the canopy as an invented equivalent parcel.

The one-upward-binary64-neighbor normalization at 273.15 K in `INV-LANDSURFACEENERGY-130` is restricted to its named covered-canopy-release and Stage 3 terminal publication boundaries. It is not a general temperature clamp, a solver repair, or authority to erase small energy increments.

The external partition owner is `SC-SURFACELIQUID-001` together with `SC-WATBAL-001#wb14-infiltration-and-hyetograph-coupling-addendum`. Native accepted child supports preserve the canonical WB14 infiltration continuation; no per-parcel restart, daily aggregation, duplicate infiltration solve, residual reconstruction or legacy capacity shortcut can replace it. Native water, infiltration, runoff and actual ET must reach their real WAT/PASS consumers with the declared timing and areas (`SC-WATBAL-001#INV-WATBAL-105` and `P-032/C-007`). A closed LSE-only ledger is insufficient evidence of that downstream consumption.

## Frozen litter, represented snow and support are distinct boundaries

The question supplies no frozen-litter mode or actual phase pools. I therefore read the complete conditional litter chapter rather than assuming phase is off. `LSE/litter-phase.md#litter-phase` specializes the liquid equations when the admitted frozen-litter mode is active:

- Liquid and ice are distinct hydrology pools. Surface sensible enthalpy uses `C_dry + W_l*C_w + W_i*C_i`; phase-inclusive enthalpy is `H = U - L_f*W_i`. Litter's fusion constant is its own authority; a snow-process constant is not interchangeable.
- The phase-free coupled solve uses beginning phase pools and the specified mixed humidity, phase-specific vapor demands, caps and signed enthalpies. No netting of unlike liquid/ice latent terms before their separate energy calculation and no current-ingress donation are admitted.
- After accepted vapor debits/credits, apply the declared finite-rate, bounded freeze/melt update, update sensible enthalpy with fusion heat, and reconstruct temperature using the *ending* phase-dependent capacity. Preserve `U-L_f*W_i` through the phase transfer. Instantaneous equilibrium, an old-capacity temperature increment, another phase-free physical solve, or freezing water not present in the governed pool is not equivalent.
- Then compute actual liquid spill above liquid capacity, remove exactly its sensible heat at the raw post-phase temperature, and send the named receipt to hydrology/WB14 once. Do not clamp away water or energy. Ice does not become `frozwt`, runoff or infiltration by alias.
- `INV-LANDSURFACEENERGY-156` / `OBL-LANDSURFACEENERGY-C-011` preserve the explicit return/retention credit separately from the spill. `INV-LANDSURFACEENERGY-157` / `C-012` require heterogeneous authorized-use rows to join their matching phase receipts without a second debit, duplicate energy or blanket resource replacement. Restart and schema migration must preserve the admitted ice owner; undeclared downgrade or implicit ice erasure is invalid.

A snow-free tile's snow exclusion is an owner/regime rule, not a temperature test. `INV-LANDSURFACEENERGY-020/021` excludes represented snow and censored schema-v8 terminal aliases from the snow-free evaluator. If the interval is a post-snow remainder, `LSE/terminal-support.md#support` and the current external snow lifecycle must determine its exact positive support and one real receiver handoff; holding its surface temperature at 0 °C or donating fusion energy to soil is prohibited. Zero remainder means no physical receiver solve.

The complete external reading exposed a version-ordering trap: historical terminal bisection/default-off descriptions are not the current adaptive Stage 3 lifecycle. The current `SC-SNOWFREEZE-001#INV-SNOWFREEZE-103` through `106` govern positive physical supports on 60-second integer quanta, joint immutable-beginning candidate refinement, exact-zero accepted ice disappearance, one zero-duration topology event and exact-once terminal liquid routing, shared active-lane owners, and complete adaptive restart custody. `SC-COUPLEDTIME-001` supplies exact half-open integer support, one shared transaction and admissible boundary coalescing. Its fine chronology representation is not permission for an arbitrarily short physical solve. No obsolete sub-floor sample, fixed-step seasonal surrogate or selected-lane merge proves the current adaptive obligation.

If represented snow actually exists, this question leaves the snow-free regime. `LSE/soil-coupling.md#version-8-persistent-snow--soil-boundary-amendment` and `INV-LANDSURFACEENERGY-124/125/126` instead join the bottom represented Stage 3 thermal volume to *first ordered OFE soil node*, not a tile surface node. Half-snow plus half-soil series resistance and both current Crank–Nicolson endpoints belong inside the covered coupled solve. One receipt carries exact opposite snow debit and first-soil credit, support, owner, topology, operand and area identities. Selecting a tile, tile averaging, applying tile fractions to that OFE transfer, duplicating it across tiles, or adding it after convergence violates that boundary. Current snow solver/lifecycle amendments, rather than an obsolete historical fixed-point cap, govern numerical acceptance.

`INV-LANDSURFACEENERGY-154` keeps represented-snow ground ownership in the charged Stage 3 map; inactive litter state is retained, not opportunistically evolved or routed through WB14 beneath that represented cover. The first actual positive snow-free child must obey the post-terminal chronology. This explains why terminal, snow-energy and snow-freeze authorities were necessary expansions even though the requested physical answer is snow-free.

## Exact custody, numerical admission and atomic publication

These rules are not alternatives to the preceding physics; they protect its state and receipt boundaries.

`LSE/soil-custody.md#INV-LANDSURFACEENERGY-150` (`P-005/C-005`) and `LSE/surface-custody.md#INV-LANDSURFACEENERGY-151` (`P-006/C-006`) require exact high-plus-signed-dyadic carry custody. Decode the accepted primitive finite-binary64 energy operands exactly, aggregate against exact beginning custody, round the persistent high term once to nearest-even and retain the exact residual. Soil conduction and infiltration remain distinct real credits. Surface phase-free, fusion, retained ingress and spill contributions remain distinct real operands. The named OFE-to-tile floating conversion precedes exact surface-credit decoding; an invented all-rational replacement is not the same protocol. High mirrors preserve their required bits; carry has its canonical normalized representation. Tiny credits must not vanish. `nextafter`, ULP forcing, discarded carry, producer residual substitution, or carry feedback into physical flux/phase/temperature is prohibited.

`INV-LANDSURFACEENERGY-155` admits candidate-only contiguous soil continuation with original beginning custody and real accepted operands; it does not permit premature owner installation, fabricated predecessor identities, or bypass of final replay/reconstruction. `INV-LANDSURFACEENERGY-153/158/160` preserve partial-parent chronology, configured OFE ordering and identity-only final reseal; they do not authorize a fresh physical rerun during publication.

The bounded nonlinear admission rules were checked in `LSE/nonlinear-solve.md` and their `LSE/numerical-methods.md` definitions. `INV-LANDSURFACEENERGY-138` allows the unique inward finite-difference probe only at an exactly admitted closed bound; it does not shrink a failed perturbation or repair an invalid current point. `INV-LANDSURFACEENERGY-139` can accept the *unchanged* already residual-admissible current iterate under its first-domain-valid halved no-update witness conditions; it cannot install the witness or bypass stricter update decrease rules. `INV-LANDSURFACEENERGY-131` governs inactive component anchors without giving them physical area or flux. The full-supply warm-start exception provides coordinates for a fresh fixed-cap final evaluation, not copied potential state. The `INV-111/112/113` migrations, narrowly scoped unit scaling and wet-cap anchors retain their specific V10 branch predicates; they are not generic forest-ground fixes. Closure, convergence, phase, representation and exact receipt predicates are distinct.

`LSE/map-custody.md#INV-LANDSURFACEENERGY-159` permits only its validated immutable in-memory proof scope. Restart/untrusted wire state still needs complete canonical semantic validation; dynamic map obligations remain fresh. `#INV-LANDSURFACEENERGY-161` separates ordinary/native request identity from the physical ground regime and requires each charged physical map prefix to validate. A later pending map can be completed as its own final accepted prefix without extra physical reevaluation; wrong identity/regime/custody rejects before a capability is exposed. This is not generic dependency replay permission. Current `SC-SNOWENERGY-001` solver and pending-prefix amendments supersede historical alternate accelerators/fallbacks.

`LSE/solve-boundary.md#solve`, `LSE/water-vapor.md#errors`, the coupled-time contract and the custody chapters require deterministic typed admission and atomic rollback. A failure in any later owner/receipt/consumer check must leave authoritative owners unchanged; successful numerical convergence alone does not authorize publication.

## Inputs and tests actually required

The question provides no numerical fixture, mode selection or accepted owner snapshots. The following inputs are required to instantiate the rules and are **missing here**:

- Exact model/schema/configuration identities; tile/OFE/occupancy topology and fractions, OFE areas, ordered soil-node identities and layer parameters; selected liquid or frozen-litter mode; explicit snow exclusion or the real terminal history; admissible exact support and parent/child continuation identity.
- Authoritative beginning surface enthalpy and exact carry where active, hydrology liquid/ice and source-layer stores, litter dry capacity/depth/conductivity/capacity parameters, beginning ordered soil enthalpies/temperatures and their exact custody. Warm starts must be labeled separately.
- The sealed meteorological forcing with required temperatures, pressure/humidity, radiation and admissible wind; neutral canopy/ground geometry and resistance inputs; vegetation component area/optical/physiology state and the current shared-air coupling inputs. Missing constitutive parameters or a legacy scalar cannot stand in for the required schemas.
- Complete potential requests, one hydrology authorization and final actual-use receipts with exact source/area/support identities; phase and spill receipts if active; final accepted rain-provider, canopy-release and upstream-runon parcel masses and enthalpies; actual hydrology partition, first-node heat/infiltration receipts, runoff destinations, and candidate ending owner identities.
- The declared applicable tolerance and solver configurations, exact restart/migration and publication schemas, and primitive real-consumer outputs needed to reconstruct conservation independently. No run identifier, fixture results, qualified physical capture or observed residual was supplied.

The mandatory evidence duties are requirements, **not results of this exercise**. `LSE/common-details.md#tests`, selected chapter obligations and external contract tests require at least:

1. Finite-capacity enthalpy/temperature inversion with changing hydrology-owned mass; exact equilibrium-zero and nearby positive-capacity distinctions; equilibrium-zero beginning-endpoint flux sensitivity to actual soil endpoints and invariance to caller warm-start changes; no fabricated storage. Include one-node and multi-node soil, sign reversal, invalid resistance/topology and independent first-node storage reconstruction.
2. Current covered shortwave/longwave and shared-air balances, inactive-area behavior, night/all-zero forcing where admitted, and typed calm/nonneutral/Kelvin/log/roughness-domain rejection rather than hidden defaults. Snow-present and every censored schema-v8 payload reject without mutation.
3. All-distinct primitive energy and water operands with independent reconstruction, plus omission and duplication of precipitation, runon, infiltration, runoff, latent and sensible heat, shortwave/longwave, ground heat, storage and vapor mass. Producer-reported residuals are diagnostic only; all-zero residual fixtures alone do not establish real coupling.
4. Actual-use `F <= A <= D`, exact equality cap branch and condensation mass-plus-energy credit; poison current-ingress donation and second authorization; final canopy release changes without supply changes; verify fresh fixed-cap physics from immutable beginning. Test no ingestion feedback into already accepted H/LE/G and no duplicate ET debit.
5. Rain provider domain/reference and nonconvergence tests under `INV-SNOWFREEZE-075`/`P-039`, accepted canopy-source temperature, missing/stale runon temperature, conservative mixed-parcel heat, zero-mass/no-temperature, unequal-area routed energy, retained heat, first-node infiltration heat and actual outlet export. Exercise real WB14 continuation and WATBAL `P-032/C-007` consumer obligations, including hourly timing and real native ET, not a diagnostic alias or residual proxy.
6. If frozen litter is selected: separate liquid/ice vapor and caps, bounded finite-rate phase transfer, conservation of `U-L_f*W_i`, ending-capacity reconstruction, spill sensible heat and exactly-once WB14 receipt/return, heterogeneous phase-aware actual use, persistent restart and invalid migration. The declared native p61 closure duties require actual persisted/reloaded production operands and independent closure, which are absent here.
7. Exact below-high-ULP surface and soil credit custody, round-to-even ties, normalization, cancellation, subnormal and overflow boundaries, high-mirror and malformed-restart poisons, primitive reconstruction and real consumer carry preservation under `P-005/C-005` and `P-006/C-006`. A synthetic arithmetic pass is not native production/restart evidence.
8. Selected numerical admission boundary poisons and unchanged-current witness checks, fresh final-pass checks, exact support and receiver cardinality, represented-snow first-node receipt reconstruction if that boundary becomes active, complete owner rollback on late failure, partial-parent/restart ordering, proof invalidation and wrong-identity pending-map poisons. `C-014` and `C-016` require their actual custody/work-counter/prefix evidence if those mechanisms are claimed; this report supplies none.

Kernel implementation work would additionally retain the applicable A0/A1/A3 contract, typed-guard, independent-closure and lifecycle gates selected under `docs/standards/testing-and-gate-strategy.md` and `docs/work-packages/science-obligations.md`. This read-only exercise does not implement a kernel change and does not pretend that running a validator would satisfy those workflows. Generic v1 future gaps, calibration labels, historic HOLDs and later admitted amendments must retain their stated scopes; comparator agreement cannot create missing authority or erase current production HOLD/protocol obligations.

## Why reading expanded, and where it stopped

The initial route required all of the shared physical surface, water, support, soil and solve definitions, rather than only the few lines spelling `G_s1`. Ground-water immutability and ingress made the complete SURFACELIQUID and WATBAL contracts mandatory. Covered radiation/canopy coupling made the complete VEGETATION contract mandatory. Exact support, terminal exclusion and represented-soil boundaries made COUPLEDTIME and SNOWENERGY mandatory. Explicit provider-derived ingress temperature, not merely accepting an opaque parcel, made complete SNOWFREEZE mandatory. Its later adaptive lifecycle was necessary to avoid treating a historical terminal method as current authority.

Unknown litter mode required complete litter-phase reading. Exact surface/soil receipt and storage claims required their complete custody chapters. Selected admission exceptions required nonlinear-solve and their routed numerical definitions. I voluntarily expanded the selected numerical-methods and map-custody extents to their whole chapters to check branch predicates, immutable-proof limits and pending-prefix semantics; that expansion is retained in cost accounting. Governance expanded into the complete testing standard, science obligations, review format, prompt wording and the context measurement tool. This was a costly reading decision, not evidence that the omitted chapters were semantically empty.

No code optimization, error-order audit, executable-identity verification, frozen experimental run, full API/registry-conformance audit, gap adjudication or promotion decision was performed. Accordingly, the separate conditional dependency-replay, qualification/replay-evidence and full audit-details routes were not activated. No old monolith, Git history, archived candidate, forbidden package artifact, other exercise answer or parent validation evidence was inspected. References to legacy provenance or other frozen experimental procedures in the admitted contracts were not turned into permissions to read forbidden evidence or execute a new experiment. The report does not claim a full-contract conformance audit.

## Reading accounting

The next tables describe source requests and finite source-byte unions, not model tokens or total runtime cost. Inclusive line ranges retain the requested endpoint; an EOF-clipped actual extent is shown where needed. File SHA-256 is the exact identity for each request, mapped through the source key. The assigned frozen commit identity is supplied by the exercise; no Git history was accessed. Mechanical current-file hashing/range accounting verified identical source identities without adding a model-visible scientific excerpt.

EOF over-requests request no nonexistent content; their byte count is the actual available slice. All original truncated requests are retained in requested exposure, along with their complete recovery reads. The union is computed over `(path, whole-file SHA-256, byte position)` and is not reduced by output truncation. Repeated bytes include ordinary rereads, voluntary source checks and truncation recovery. A truncated request cannot itself certify the full content was read; recovery establishes that coverage.

| Source class | Finite unique bytes | Requested excerpt bytes | Repeated bytes |
|---|---:|---:|---:|
| LSE (completed logged requests) | 215,666 | 298,616 | 82,950 |
| external_protocol (completed logged requests) | 2,035,441 | 2,362,062 | 326,621 |
| governance_tool (completed logged requests) | 82,652 | 90,768 | 8,116 |
| Failed existing-source request F2 (additional exposure, no new union) | 0 | 1,586 | 1,586 |
| Private measurement-helper source (two displays) | 733 | 1,466 | 733 |
| Total observed/declared source-request accounting | 2,334,492 | 2,754,498 | 420,006 |

There were **140 completed logged repository-source requests**, **two failed source requests**, and **two private-helper source displays**. Source-content searches: **zero**. Instruction discovery (`tools/agents/find-agents --for`), path/line/byte inventory, current-file hashing, ledger inspection, helper editing and arithmetic are metadata/instrumentation operations, not additional scientific excerpts. The context-report function mechanically reread file bytes to calculate the declared ranges; those filesystem I/O bytes are not double-counted as model-visible source requests. Its emitted report labels workflow total `UNOBSERVED`.

The finite LSE union is **215,666 bytes**, versus the package's **264,863-byte** baseline: **18.57450833072192% reduction**, so **the requested at-least-20% reduction is NOT met**. No repeats or voluntary expansion have been removed to improve that outcome. The entry plus complete shared interface is **12,021 bytes**, within the **12,288-byte** always-on limit; it is only the declared LSE bootstrap, not the full session bootstrap. Initial governance/handoff bootstrap contributes 24,440 additional requested unique bytes. All of the external/protocol expansion and later governance reading remain separately visible.

Root AGENTS instructions were also automatically exposed in the task. Their current file-equivalent content is 9,508 bytes; this automatic occurrence is reported separately from the explicit file reread and is **not** inserted as if an observed tool request. Exact serialized system/developer/user exposure, other automatic instruction tokens, tool-description exposure, context-compaction summaries/replay, model-token counts, quotas, and end-to-end runtime cost are **UNOBSERVED**. This exercise continued as the same reader after context compaction and a usage interruption; the private ledger/draft persisted. No fresh-reader reset is claimed for that continuation, and no ledger can recover unobserved delivered-token or replay telemetry. Requested source bytes must not be described as the actual delivered model-token context.

The private original helper at `/tmp/lse-surface08-Mh5yLy/read.py`, lines 1–8, was displayed twice: once during earlier instrumentation inspection and once while recovering F2. Each original display requested 733 bytes with SHA-256 `2c7b9021f0472d3163989625415a956b8510269303aa1ecf0191dbf37d66c124`; the second repeats all 733. Its corrected helper is 749 bytes, SHA-256 `1e003ae70b9281a2eeb74dfcf9fdcfdcdf750c9c24689c49ebff47b2c16c3dea`; the correction only clips repeat-byte lookup at EOF. The modified source was not displayed as a further excerpt. Earlier instrumentation used the environment interpreter for bootstrap; subsequent repository tooling and final counting used `.venv/bin/python`.

Failed requests, retained rather than silently erased:

| ID | Path / requested inclusive range | Identity / requested bytes | Phase and outcome |
|---|---|---|---|
| F1 | `docs/specifications/science-contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md`, 116–207 | Missing path; no source identity or available bytes | Final source-check; `FileNotFoundError`, zero source content delivered; corrected path recovered in request 135. |
| F2 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md`, 1–100 (actual 1–22) | S01; 1,586 bytes requested, all previously exposed | Final source-check; helper repeat-count lookup exceeded EOF before it printed or logged the excerpt; no source content delivered. Recovery is request 138 after helper correction. This conservative requested-exposure accounting includes the failed request's available slice. |

The recorded phase `reading` includes ordinary expansion, recovery and voluntary source-check rereads. Requests 134–140 are the final explicit source-check sequence (including the tool/neutral-handoff checks); initial request 1 has phase `handoff`. The interruption did not discard earlier requests. All source identities below were mechanically rechecked against the current frozen files; no mismatch was found.

### Source identity index

| Key | Exact path | Whole-file SHA-256 | File lines | File bytes |
|---|---|---|---:|---:|

| S01 | `docs/work-packages/20260907-lse-context-adoption-correction-001/artifacts/worker-handoff.md` | `00f80e082f41b65a25b6864340cd89b6daffbf0d16ba54d6df5612e31d25d6fc` | 22 | 1586 |
| S02 | `AGENTS.md` | `9bd7887bcae731debc64689a4774ac222d529bcd527ebea2293e3ffd600289f6` | 121 | 9508 |
| S03 | `docs/work-packages/AGENTS.md` | `29e2f48a0b69fd9f43eb8d3a6dc2b72d30792337f90a2141b89df105726282a5` | 105 | 6642 |
| S04 | `docs/work-packages/20260907-lse-context-adoption-correction-001/package.md` | `0cb2505989026d5e911d696eb7eda8d96c5ebd5498da421e55a2f7682290929b` | 75 | 5304 |
| S05 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md` | `391772bfc9345357a654552ae84203809a91e0280f5579693ae73da68119cc53` | 67 | 4090 |
| S06 | `docs/work-packages/role-review.md` | `2193e9538ad22dad5678fc5bb59080f686a543f697fec4983208214a73cc5eb1` | 7 | 1400 |
| S07 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/interface.md` | `758873659d360c8edfc7bcec7aa6cb58d065d44e221e890166390252810ed481` | 66 | 7931 |
| S08 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-energy.md` | `2c3afa3f25827508ab9bd4826249d6557d08aa44b8f4ba7f0ecd5c988a900bb8` | 205 | 11494 |
| S09 | `docs/specifications/science-contracts/AGENTS.md` | `bace4ab3e33119e0239f700f92c53707d884f297e92d2454a86c477758c930c7` | 84 | 6532 |
| S10 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/common-details.md` | `b930bbd0f0ce2c16d826be6a6d1d0bb4e6fe0155e611232047e0b4205e2dbfc4` | 221 | 17798 |
| S11 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-coupling.md` | `042668b12c9bd0c207ab999a63531ccfe69936418555adc1f7917fffb68d9731` | 207 | 13024 |
| S12 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/terminal-support.md` | `c42f6fad39dd7b1a549b83ff867aa54af9b86baf7971060e849784880ea9773d` | 244 | 20665 |
| S13 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/litter-phase.md` | `4fdd656cc3884fafff85fe5d5d3ee30deb9cb5e015c5e6c0ca245c967d4a99e3` | 433 | 30863 |
| S14 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/solve-boundary.md` | `f4ef16b1459c728583634e0110ef31f17a4da19865fee39229c74b5588e4eba7` | 114 | 8329 |
| S15 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/water-vapor.md` | `fda2350a1d4bf826dd3ad6f622e4ee0f41d708860aef6005e8948c27c3d729f8` | 227 | 15949 |
| S16 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/soil-custody.md` | `eac50350e934d03e47022dcc51d0c55013c5a5d975f1beceb6f6b7196cc03b18` | 200 | 15323 |
| S17 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/surface-custody.md` | `4acd7ddf7a20d9c1198a41ee26d4caa8c57361728eb893ae365c31c669fe66e3` | 291 | 25523 |
| S18 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/nonlinear-solve.md` | `a358e2acece25dc440adc8b1d35c732c829b2f8e2c3fbe24e8667c1635bed58e` | 93 | 6459 |
| S19 | `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` | `3b1c8525e6d09d7afae6e27d5fa59f90347d55d8bf34bc6292d0d767d3bfc5fa` | 3080 | 248829 |
| S20 | `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md` | `1d10611b5143be201381f283ec5594d9f65c99f8252e944dfae57076a8a699f7` | 1108 | 92552 |
| S21 | `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md` | `fbb27d07e4459dabd37330d3292564ccb94789fdc0530fcde07262c00adfcbd4` | 2205 | 184116 |
| S22 | `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md` | `94eb7a73ed2d0de50f5a354a200467809154c2e4ecfa0792677942343cc4943d` | 3559 | 497785 |
| S23 | `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` | `1d60a1c5e879bd28551b3fd73091b108597fa4172d67c35d1957826406c4cd1c` | 2648 | 377637 |
| S24 | `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` | `50e285aa00bcf7313a232da915e0ec033590e5db2b7c5694b7574fb0b2d508f4` | 4295 | 634522 |
| S25 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/map-custody.md` | `8508b3ceaa49f081ce8c2df1d7cce39249d8f3875fea5a017404c0816cdae2c1` | 170 | 19110 |
| S26 | `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/numerical-methods.md` | `fe3f8fb74ec1143cc69b5ead71949d2549757edb4bbac46b9bc77e175ab00f87` | 266 | 19108 |
| S27 | `docs/standards/AGENTS.md` | `b62a33d9a70a3d3f45e7eccdcf05ca3a194486fcdbfc7bd439371cf8f0ae3015` | 58 | 4052 |
| S28 | `docs/standards/testing-and-gate-strategy.md` | `56277d9848d80b0672ebeaaa8277ac2ad9888c90e54b4f31aa7c5b8ca9b04230` | 494 | 25142 |
| S29 | `docs/work-packages/science-obligations.md` | `1a0377a3362a6c34d783796b6756854f8630e1f05e994dc3d963eaafd16c542e` | 98 | 6149 |
| S30 | `docs/prompt_templates/assurance-findings-template.md` | `95373d3c7df1f0a7e3ad7640fa22c0e137095e6cf5c4d92e3803846eb3f6ee83` | 12 | 736 |
| S31 | `docs/standards/prompt-wording-guidance.md` | `294385612b565fe8be3e9e530caa0792c901406ecf53abc9402649e5568ffe3f` | 202 | 12057 |
| S32 | `tools/agents/context_report.py` | `2aa217325da4b279926aa0780d1aa9427fb21d1e80a533314cbe277474d2bbde` | 79 | 3544 |

### Every completed source request

`new = bytes - repeat`; repeat is against preceding requests for the same exact file identity. `OK` means no recorded truncation. The recovery column lists the later untruncated requests covering the entire truncated available range. Requested EOF overreach is shown explicitly, without calling the nonexistent tail an unobserved source obligation.

| # | Source | Inclusive requested lines (actual if clipped) | Bytes | New | Repeat | Recorded phase | Truncation/recovery |
|---:|---|---|---:|---:|---:|---|---|

| 1 | S01 | 1–22 | 1586 | 1586 | 0 | handoff | OK |
| 2 | S02 | 1–121 | 9508 | 9508 | 0 | reading | OK |
| 3 | S03 | 1–105 | 6642 | 6642 | 0 | reading | OK |
| 4 | S04 | 1–75 | 5304 | 5304 | 0 | reading | OK |
| 5 | S05 | 1–67 | 4090 | 4090 | 0 | reading | OK |
| 6 | S06 | 1–7 | 1400 | 1400 | 0 | reading | OK |
| 7 | S07 | 1–66 | 7931 | 7931 | 0 | reading | OK |
| 8 | S08 | 1–205 | 11494 | 11494 | 0 | reading | OK |
| 9 | S09 | 1–84 | 6532 | 6532 | 0 | reading | OK |
| 10 | S10 | 1–221 | 17798 | 17798 | 0 | reading | OK |
| 11 | S11 | 1–207 | 13024 | 13024 | 0 | reading | OK |
| 12 | S12 | 1–244 | 20665 | 20665 | 0 | reading | Truncated; fully recovered 14,15 |
| 13 | S13 | 1–433 | 30863 | 30863 | 0 | reading | Truncated; fully recovered 16,17,18 |
| 14 | S12 | 1–125 | 8437 | 0 | 8437 | reading | OK |
| 15 | S12 | 126–244 | 12228 | 0 | 12228 | reading | OK |
| 16 | S13 | 1–145 | 7695 | 0 | 7695 | reading | OK |
| 17 | S13 | 146–290 | 7552 | 0 | 7552 | reading | OK |
| 18 | S13 | 291–443 (actual 291–433) | 15616 | 0 | 15616 | reading | OK |
| 19 | S14 | 1–114 | 8329 | 8329 | 0 | reading | OK |
| 20 | S15 | 1–227 | 15949 | 15949 | 0 | reading | OK |
| 21 | S16 | 1–200 | 15323 | 15323 | 0 | reading | OK |
| 22 | S17 | 1–291 | 25523 | 25523 | 0 | reading | OK |
| 23 | S18 | 1–93 | 6459 | 6459 | 0 | reading | OK |
| 24 | S19 | 1–320 | 28754 | 28754 | 0 | reading | OK |
| 25 | S19 | 321–650 | 21344 | 21344 | 0 | reading | OK |
| 26 | S19 | 651–980 | 24113 | 24113 | 0 | reading | OK |
| 27 | S19 | 981–1140 | 27149 | 27149 | 0 | reading | OK |
| 28 | S19 | 1141–1470 | 28724 | 28724 | 0 | reading | OK |
| 29 | S19 | 1471–1780 | 32323 | 32323 | 0 | reading | OK |
| 30 | S19 | 1781–2130 | 19902 | 19902 | 0 | reading | OK |
| 31 | S19 | 2131–2480 | 23701 | 23701 | 0 | reading | OK |
| 32 | S19 | 2481–2800 | 20667 | 20667 | 0 | reading | OK |
| 33 | S19 | 2801–3080 | 22152 | 22152 | 0 | reading | OK |
| 34 | S20 | 1–350 | 20398 | 20398 | 0 | reading | OK |
| 35 | S20 | 351–700 | 39027 | 39027 | 0 | reading | OK |
| 36 | S20 | 701–920 | 16082 | 16082 | 0 | reading | OK |
| 37 | S20 | 921–1108 | 17045 | 17045 | 0 | reading | OK |
| 38 | S21 | 1–330 | 20384 | 20384 | 0 | reading | OK |
| 39 | S21 | 331–650 | 15362 | 15362 | 0 | reading | OK |
| 40 | S21 | 651–950 | 51100 | 51100 | 0 | reading | Truncated; fully recovered 41,42 |
| 41 | S21 | 651–770 | 33612 | 0 | 33612 | reading | OK |
| 42 | S21 | 771–950 | 17488 | 0 | 17488 | reading | OK |
| 43 | S21 | 951–1250 | 17079 | 17079 | 0 | reading | OK |
| 44 | S21 | 1251–1540 | 18082 | 18082 | 0 | reading | OK |
| 45 | S21 | 1541–1820 | 32644 | 32644 | 0 | reading | OK |
| 46 | S21 | 1821–2205 | 29465 | 29465 | 0 | reading | OK |
| 47 | S22 | 1–300 | 19256 | 19256 | 0 | reading | OK |
| 48 | S22 | 301–590 | 24868 | 24868 | 0 | reading | OK |
| 49 | S22 | 591–880 | 52123 | 52123 | 0 | reading | Truncated; fully recovered 51,52,53 |
| 50 | S22 | 591–730 | 43952 | 0 | 43952 | reading | Truncated; fully recovered 51,52 |
| 51 | S22 | 591–625 | 23838 | 0 | 23838 | reading | OK |
| 52 | S22 | 626–760 | 22076 | 0 | 22076 | reading | OK |
| 53 | S22 | 761–940 | 8742 | 2533 | 6209 | reading | OK |
| 54 | S22 | 941–1210 | 11977 | 11977 | 0 | reading | OK |
| 55 | S22 | 1211–1450 | 134795 | 134795 | 0 | reading | Truncated; fully recovered 56,57,58,59,60,61,62 |
| 56 | S22 | 1211–1308 | 19984 | 0 | 19984 | reading | OK |
| 57 | S22 | 1309–1336 | 19644 | 0 | 19644 | reading | OK |
| 58 | S22 | 1337–1402 | 19743 | 0 | 19743 | reading | OK |
| 59 | S22 | 1403–1423 | 18050 | 0 | 18050 | reading | OK |
| 60 | S22 | 1424–1431 | 19781 | 0 | 19781 | reading | OK |
| 61 | S22 | 1432–1440 | 18502 | 0 | 18502 | reading | OK |
| 62 | S22 | 1441–1450 | 19091 | 0 | 19091 | reading | OK |
| 63 | S22 | 1451–1555 | 18481 | 18481 | 0 | reading | OK |
| 64 | S22 | 1556–1622 | 19881 | 19881 | 0 | reading | OK |
| 65 | S22 | 1623–1850 | 19929 | 19929 | 0 | reading | OK |
| 66 | S22 | 1851–2026 | 19970 | 19970 | 0 | reading | OK |
| 67 | S22 | 2027–2094 | 19580 | 19580 | 0 | reading | OK |
| 68 | S22 | 2095–2135 | 19923 | 19923 | 0 | reading | OK |
| 69 | S22 | 2136–2443 | 19936 | 19936 | 0 | reading | OK |
| 70 | S22 | 2444–2699 | 19982 | 19982 | 0 | reading | OK |
| 71 | S22 | 2700–2896 | 19986 | 19986 | 0 | reading | OK |
| 72 | S22 | 2897–2920 | 19533 | 19533 | 0 | reading | OK |
| 73 | S22 | 2921–3093 | 19928 | 19928 | 0 | reading | OK |
| 74 | S22 | 3094–3360 | 19907 | 19907 | 0 | reading | OK |
| 75 | S22 | 3361–3559 | 15197 | 15197 | 0 | reading | OK |
| 76 | S23 | 1–187 | 23659 | 23659 | 0 | reading | OK |
| 77 | S23 | 188–238 | 23551 | 23551 | 0 | reading | OK |
| 78 | S23 | 239–265 | 23082 | 23082 | 0 | reading | OK |
| 79 | S23 | 266–288 | 23644 | 23644 | 0 | reading | OK |
| 80 | S23 | 289–330 | 23610 | 23610 | 0 | reading | OK |
| 81 | S23 | 331–407 | 23826 | 23826 | 0 | reading | OK |
| 82 | S23 | 408–471 | 23938 | 23938 | 0 | reading | OK |
| 83 | S23 | 472–539 | 23929 | 23929 | 0 | reading | OK |
| 84 | S23 | 540–650 | 23986 | 23986 | 0 | reading | OK |
| 85 | S23 | 651–873 | 23937 | 23937 | 0 | reading | OK |
| 86 | S23 | 874–1284 | 23942 | 23942 | 0 | reading | OK |
| 87 | S23 | 1285–1696 | 23994 | 23994 | 0 | reading | OK |
| 88 | S23 | 1697–2127 | 23978 | 23978 | 0 | reading | OK |
| 89 | S23 | 2128–2456 | 23910 | 23910 | 0 | reading | OK |
| 90 | S23 | 2457–2550 | 23978 | 23978 | 0 | reading | Truncated; fully recovered 92 |
| 91 | S23 | 2551–2648 | 20673 | 20673 | 0 | reading | Truncated; fully recovered 93 |
| 92 | S23 | 2457–2550 | 23978 | 0 | 23978 | reading | OK |
| 93 | S23 | 2551–2648 | 20673 | 0 | 20673 | reading | OK |
| 94 | S24 | 1–97 | 23466 | 23466 | 0 | reading | OK |
| 95 | S24 | 98–183 | 23833 | 23833 | 0 | reading | OK |
| 96 | S24 | 184–256 | 23933 | 23933 | 0 | reading | OK |
| 97 | S24 | 257–290 | 23667 | 23667 | 0 | reading | OK |
| 98 | S24 | 291–309 | 23780 | 23780 | 0 | reading | OK |
| 99 | S24 | 310–325 | 23720 | 23720 | 0 | reading | OK |
| 100 | S24 | 326–338 | 22601 | 22601 | 0 | reading | OK |
| 101 | S24 | 339–348 | 22456 | 22456 | 0 | reading | OK |
| 102 | S24 | 349–360 | 23885 | 23885 | 0 | reading | OK |
| 103 | S24 | 361–370 | 22718 | 22718 | 0 | reading | OK |
| 104 | S24 | 371–490 | 23689 | 23689 | 0 | reading | OK |
| 105 | S24 | 491–564 | 23931 | 23931 | 0 | reading | OK |
| 106 | S24 | 565–666 | 23568 | 23568 | 0 | reading | OK |
| 107 | S24 | 667–992 | 23949 | 23949 | 0 | reading | OK |
| 108 | S24 | 993–1323 | 23985 | 23985 | 0 | reading | OK |
| 109 | S24 | 1324–1475 | 23721 | 23721 | 0 | reading | OK |
| 110 | S24 | 1476–1532 | 23399 | 23399 | 0 | reading | OK |
| 111 | S24 | 1533–1907 | 23955 | 23955 | 0 | reading | OK |
| 112 | S24 | 1908–2293 | 23998 | 23998 | 0 | reading | OK |
| 113 | S24 | 2294–2664 | 23962 | 23962 | 0 | reading | OK |
| 114 | S24 | 2665–3029 | 23926 | 23926 | 0 | reading | OK |
| 115 | S24 | 3030–3440 | 23932 | 23932 | 0 | reading | OK |
| 116 | S24 | 3441–3789 | 23980 | 23980 | 0 | reading | OK |
| 117 | S24 | 3790–4004 | 23449 | 23449 | 0 | reading | OK |
| 118 | S24 | 4005–4181 | 23878 | 23878 | 0 | reading | OK |
| 119 | S24 | 4182–4223 | 23981 | 23981 | 0 | reading | OK |
| 120 | S24 | 4224–4295 | 19160 | 19160 | 0 | reading | OK |
| 121 | S25 | 1–100 | 7374 | 7374 | 0 | reading | OK |
| 122 | S25 | 101–220 (actual 101–170) | 11736 | 11736 | 0 | reading | OK |
| 123 | S26 | 1–180 | 11999 | 11999 | 0 | reading | OK |
| 124 | S26 | 181–360 (actual 181–266) | 7109 | 7109 | 0 | reading | OK |
| 125 | S06 | 1–7 | 1400 | 0 | 1400 | reading | OK |
| 126 | S27 | 1–120 (actual 1–58) | 4052 | 4052 | 0 | reading | OK |
| 127 | S28 | 1–240 | 12948 | 12948 | 0 | reading | OK |
| 128 | S28 | 241–494 | 12194 | 12194 | 0 | reading | OK |
| 129 | S29 | 1–98 | 6149 | 6149 | 0 | reading | OK |
| 130 | S30 | 1–150 (actual 1–12) | 736 | 736 | 0 | reading | OK |
| 131 | S31 | 1–202 | 12057 | 12057 | 0 | reading | OK |
| 132 | S01 | 1–200 (actual 1–22) | 1586 | 0 | 1586 | reading | OK |
| 133 | S32 | 1–79 | 3544 | 3544 | 0 | reading | OK |
| 134 | S11 | 1–115 | 5636 | 0 | 5636 | reading | OK |
| 135 | S11 | 116–207 | 7388 | 0 | 7388 | reading | OK |
| 136 | S05 | 1–67 | 4090 | 0 | 4090 | reading | OK |
| 137 | S15 | 1–120 | 6972 | 0 | 6972 | reading | OK |
| 138 | S01 | 1–100 (actual 1–22) | 1586 | 0 | 1586 | reading | OK |
| 139 | S32 | 1–79 | 3544 | 0 | 3544 | reading | OK |
| 140 | S10 | 120–221 | 7336 | 0 | 7336 | reading | OK |

All eight recorded truncations have complete later untruncated recovery. A mechanical line-union check additionally found no unread line in any selected whole source. This verifies declared reading coverage and source identity; it is not scientific execution evidence, validation of a production implementation, or evidence that unobserved model context was retained perfectly through interruption.

The only repository write for this exercise is this assigned report. Private draft, log, corrected helper, measurement specification and output remain under the unique `/tmp/lse-surface08-Mh5yLy/` prefix. The report will be hashed after its sole publication and then left immutable; the parent independently determines exercise/adoption disposition. The actual size miss and all missing run evidence remain open facts.
