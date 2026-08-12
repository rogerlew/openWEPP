# Lift Heterogeneous-Tile Canopy-Liquid Authority and Resume the Coupled C3 Vegetation Implementation

Scope: local openWEPP repository scientific-authority and kernel implementation work in `/home/workdir/openWEPP`. Flat-file edits and local command execution only. Read-only scientific-reference access is permitted only when required to close the authority package. No deployment, external publication, runtime activation, production cutover, remote branch creation, PR, or external message. Do not push unless separately directed by the user.

Starting point:

`main` at commit `02631ae92af6b073ed7957592fef4bad68dcf77f`

The worktree must be clean and synchronized before edits.

This directive authorizes two sequential stages:

1. Create and execute a narrow contract-first authority-lift package for heterogeneous-tile canopy-liquid state and routing.
2. After that package completes and releases a new digest-bound model identity, resume and complete the existing implementation package:

   `docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/`

Do not create a second vegetation implementation package. The current implementation package remains the owner of production Rust, remediation findings, diagnostic execution, reviews, heavy validation, and final implementation disposition.

## Current Boundary

The implementation package is truthfully:

`EXECUTED-HOLD / canonical E04 topology authority missing`

The legitimate blocker is narrow but load-bearing. The existing authority permits one stratum to occupy multiple nonoverlapping topology tiles while defining only one stand-ground canopy-liquid store for that stratum. It does not define how that nonlinear persistent state is distributed, updated, recombined, or routed through heterogeneous descendant columns.

The current implementation must continue to fail closed for ambiguous heterogeneous topology until the replacement authority is approved and implemented.

Before any other work, move the heterogeneous-E04 guard so it executes before any call to `rain_by_stratum`, `liquid_interception`, wet-fraction calculation, or other E04-derived intermediate. Unsupported topology must fail before the unauthorized mechanism is evaluated, not merely before commit.

Preserve all existing HOLD evidence, failed reviews, accepted findings, command logs, and checkpoint dispositions. Do not rewrite history.

# Stage A: Authority Lift

Create and execute:

`docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001/`

Execution mode: package-end-to-end.

Plan class: Critical contract-first scientific-authority amendment.

The authority package contains no production Rust other than contract-derived tests, independent reference/oracle code, digest fixtures, and bounded authority-suite bindings.

## Authority-Package Objective

Select and canonically admit an exact heterogeneous-topology rule for:

* persistent canopy-liquid state;
* tile-local incident liquid;
* interception;
* storage capacity;
* wet fraction;
* wet-canopy evaporation;
* condensation;
* second drainage;
* throughfall;
* stemflow;
* drainage;
* descendant-column routing;
* tile-local energy ownership;
* tile-local water demand;
* stand aggregation;
* state initialization and migration;
* exact conservation and poison vectors.

The selected rule must close every downstream consumer of E04 state. Do not amend only `S_liq` while leaving wet-surface energy, gas exchange, hydraulics, resource requests, or persistent numerical state ambiguous.

## Required Reading for Stage A

### Core

* `/home/workdir/openWEPP/AGENTS.md`
* `/home/workdir/openWEPP/docs/codex_exec_plans.md`
* `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
* `/home/workdir/openWEPP/docs/work-packages/README.md`
* `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
* `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
* `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
* `/home/workdir/openWEPP/docs/specifications/science-contract-spec.md`
* `/home/workdir/openWEPP/docs/specifications/unit-governance.md`
* `/home/workdir/openWEPP/docs/specifications/correctness-authority-model.md`
* `/home/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
* `/home/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md`
* `/home/workdir/openWEPP/docs/governance/reference-vendoring-policy.md`

### Existing authority and HOLD evidence

* `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md`
* `docs/specifications/science-contracts/contracts/SC-BIOGEOCHEM-001.md`
* `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
* `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
* `docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/package.md`
* its equation, numerical, parameter, state-ownership, model-definition, and test-vector artifacts;
* the current implementation package;
* `artifacts/hold-legitimacy-audit.md`;
* `artifacts/review_agent_b_remediation_repeat.md`;
* `artifacts/review_agent_a_remediation_repeat.md`;
* `artifacts/final-disposition.md`;
* `artifacts/gate-results.md`.

### Source and reference triggers

Read the exact CLM5, interception, canopy-energy, and multistratum reference material already captured in the repository when their mechanisms are invoked. Acquire another primary or established-model source only if it is load-bearing and not already locally identified. Follow the repository rights policy and retain exact reviewed-byte checksums.

Run:

```
tools/agents/find-agents --for \
  docs/work-packages/20260811-c3-woody-tile-liquid-topology-authority-001 \
  docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md \
  docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md \
  docs/specifications/science-contracts/contracts/SC-WATBAL-001.md \
  docs/specifications/science-contracts/index.md \
  tests/integration
```

Record the instruction chain and exact local reading-byte budget.

## Selected Scientific Direction

The default and preferred canonical selection is **tile-resolved occupancy state**.

An occupancy is the exact pair:

`o = (stratum_id, tile_id)`

where the stratum is present in that tile.

The authority package may select a different aggregate-state rule only if both independent science reviewers conclude that it fully closes nonlinear storage, descendant routing, wet-surface energy, resource requests, and persistent-state semantics without hidden lateral mixing or information loss. Merely preserving the old scalar state is not sufficient.

Unless that reviewed exception occurs, admit the following model.

## Topology and Area Basis

For tile `t`:

* `f_t` is its positive stand-area fraction.
* Tiles are nonoverlapping.
* `sum_t f_t = 1` within the admitted representation tolerance.

For stratum `s` occupying tile set `T_s`:

```
C_s = sum(f_t for t in T_s)
```

`LAI_s` and `WAI_s` remain shared stratum state on the stand-ground basis.

The conditional plant-area density on every occupied tile is selected as:

```
LAI_s,t = LAI_s / C_s
WAI_s,t = WAI_s / C_s
```

This is an explicit `OPENWEPP_CANONICAL_SELECTION`: a named stratum has uniform conditional leaf and woody area density across its occupied tiles. Do not leave this as an implementation inference.

Reject:

* `C_s <= 0`;
* missing occupancy state;
* duplicate `(s,t)` state;
* a tile state for a tile not occupied by the stratum;
* stand-area LAI used directly as tile-area LAI;
* silent normalization of inconsistent topology.

## New Model Identity

Do not mutate the existing `OPENWEPP_C3_WOODY_V1` definition while retaining its digest or identity.

The heterogeneous-tile amendment changes persistent state, supported topology, routing, resource identity, and numerical execution. Issue a new immutable identity:

`OPENWEPP_C3_WOODY_V2`

unless an established repository version-naming rule requires an equivalent explicit successor name.

Required changes include:

* bump `SC-VEGETATION-001` to the next contract version;

* mark V1 historical and superseded before production completion;

* create:

  `openwepp_c3_woody_v2_definition.json`

* bind the exact canonical contract-section hashes;

* bind all fixed constants and algorithms;

* bind occupancy-state schema;

* bind unsupported branches;

* bind routing and aggregation policy;

* bind numerical tolerances;

* compute and record a new SHA-256;

* preserve the V1 JSON and digest unchanged.

No implementation may claim V2 until its bytes and section hashes match exactly.

## Persistent State Partition

### Shared stratum state

Keep these shared by stratum:

* identity and parameters;
* leaf, fine-root, stem, coarse-root, storage, transfer, and retranslocation C/N;
* `NSC_C`;
* maintenance reserve;
* phenology phase and timers;
* mortality and turnover state;
* stand-ground `LAI_s` and `WAI_s`;
* root profile and stoichiometric parameters;
* material-transfer state.

### Occupancy-local state

Define persistent state for every occupied `(s,t)` pair:

* canopy-liquid store `S_liq,s,t`;
* wet-fraction state derived from that store;
* tile-local accepted canopy temperature/wet-node numerical warm starts when required;
* sunlit/shaded leaf-temperature warm starts when required;
* canopy-air temperature and humidity warm starts when required;
* hydraulic leaf/stem/root potential warm starts when the occupancy solve uses them;
* last accepted occupancy transaction identity.

Warm starts are numerical state, not water storage, but their ownership and serialization must be deterministic. Do not broadcast one accepted occupancy solution to all tiles after they have diverged.

## Units and Basis

Define:

* `S_liq,s,t`: `kg H2O m^-2 tile-ground`;
* `P_liq,s,t`: interval-integrated `kg H2O m^-2 tile-ground`;
* `E_int,s,t`: interval-integrated `kg H2O m^-2 tile-ground`;
* `R_through,s,t`, `R_stem,s,t`, `R_drain,s,t`: interval-integrated `kg H2O m^-2 tile-ground`;
* tile-local energy: `J m^-2 tile-ground`;
* tile-local photosynthesis and transpiration before weighting: declared tile-ground or leaf basis;
* resource requests submitted to stand-level owners: stand-ground interval amounts.

Stand-ground aggregates are exact weighted sums:

```
S_liq,s,agg = sum_t(f_t * S_liq,s,t)
X_s,agg     = sum_t(f_t * X_s,t)
```

Do not store both local and aggregate state as independent mutable sources. Aggregates are derived diagnostics or ledgers.

## Tile-Local E04

For each occupancy `(s,t)`, execute the complete admitted E04 sequence independently using:

* local beginning store `S_liq,s,t`;
* local incident amount `P_liq,s,t`;
* `LAI_s,t`;
* `WAI_s,t`;
* stratum interception parameters;
* tile-local canopy temperature.

The exact ordered branch remains:

```
fint = alpha_liq * tanh(L + S)
Pint = fint * P
Pfree = P - Pint
stemflow = fstem * Pfree
throughfall = (1 - fstem) * Pfree
Sstar = S0 + Pint
Smax = pliq * (L + S)
initial drainage = max(0, Sstar - Smax)
wet fraction = (S / Smax)^(2/3), or exact zero when Smax = 0
positive vapor removes no more than stored liquid
negative vapor is condensation
condensation reapplies capacity
second drainage is exposed explicitly
accepted S1 follows the complete ordered sequence
```

Subfreezing state remains typed unsupported.

No lateral water redistribution among tiles is permitted.

## Column Routing

For every tile, process occupancies in deterministic top-to-bottom rank order.

Select the following routing:

* free throughfall proceeds to the next lower occupancy in the same tile;
* initial and second drainage proceed to the next lower occupancy in the same tile;
* stemflow bypasses lower foliage and routes directly to the tile’s ground liquid recipient;
* terminal throughfall and drainage from the lowest occupancy route to that tile’s ground recipient;
* rainfall on an empty tile routes directly to the ground recipient;
* no release from tile `t` may enter tile `u`;
* no release may be aggregated to stand scale before descendant-column routing is complete.

If primary or established-model evidence requires a different stemflow route, record it explicitly and have both reviewers adjudicate it. Do not leave stemflow routing implicit.

## Tile-Local Wet-Surface Energy and Physiology

Trace every consumer of:

* `S_liq`;
* wet fraction;
* wet leaf area;
* wet stem area;
* dry leaf area;
* local radiation;
* canopy-air state;
* condensation;
* wet-canopy evaporation.

For V2:

* radiation remains solved per complete topology column;
* wet/dry area partition is occupancy-local;
* wet-canopy evaporation and condensation are occupancy-local;
* leaf and canopy-air energy equations consume occupancy-local radiation and wetness;
* sunlit/shaded gas exchange is occupancy-local;
* the coupled gas/energy/hydraulic solve is occupancy-local when its forcing or wetness differs by tile;
* occupancy-local outputs are weighted by `f_t` before entering shared stratum C/N state.

Do not average PAR, wet fraction, temperature, or conductance before solving a nonlinear leaf/energy equation unless the contract explicitly derives that reduction.

## Occupancy Water Requests

Extend water resource identity to include occupancy:

```
D_W,s,t,l
A_W,s,t,l
F_W,s,t,l
```

The occupancy solve produces a tile-ground interval amount:

```
D_tile,s,t,l
```

Convert exactly once to the stand-ground request:

```
D_W,s,t,l = f_t * D_tile,s,t,l
```

Hydrology arbitrates stand-ground amounts against the same soil-layer snapshot.

For the authorization-constrained occupancy re-solve, convert back exactly:

```
A_tile,s,t,l = A_W,s,t,l / f_t
```

Finalized use returns to stand basis:

```
F_W,s,t,l = f_t * F_tile,s,t,l
```

Require:

```
0 <= F_W,s,t,l <= A_W,s,t,l <= D_W,s,t,l
```

Aggregate stratum transpiration:

```
T_s = sum_t sum_l F_W,s,t,l
```

Reject:

* missing tile factor;
* applying `f_t` twice;
* authorization treated as tile-ground without division;
* occupancy swap;
* layer swap;
* stale transaction identity;
* duplicate request identity.

Amend `SC-WATBAL-001` only as much as required to admit occupancy-preserving water identity and arbitration. Hydrology remains the sole soil-store mutator.

## Shared Carbon and Nitrogen State

Tile-local physiology yields tile-local interval fluxes. Convert to stand-ground and aggregate:

```
GPP_s = sum_t(f_t * GPP_s,t)
Rm_s  = sum_t(f_t * Rm_s,t)
water and energy fluxes similarly aggregate
```

The shared stratum C/N transition then executes once using those exact aggregates.

Mineral-N demand may remain stratum/layer/species-level after tile aggregation unless the selected physiology requires tile-specific N demand. State the decision explicitly.

Do not duplicate the shared C/N pool into every tile.

Material transfers remain stratum-level and retain exact donor/receiver C/N/dry-matter identity.

## Canopy-Air and Hydraulic Warm Starts

The authority package must explicitly decide and test warm-start ownership.

Preferred selection:

* occupancy-local leaf, stem, root, canopy-air, and wet-node warm starts;
* warm starts affect numerical initialization only;
* accepted physical fluxes do not depend on arbitrary occupancy iteration order;
* alternate valid warm starts converge to the same accepted state within canonical tolerance;
* failure preserves every warm-start byte.

Do not define an unreviewed weighted-average potential as persistent state.

## Local and Global Closure

Require tile-local canopy-liquid closure:

```
S0_s,t
+ P_s,t
+ condensation_s,t
=
S1_s,t
+ wet_evaporation_s,t
+ throughfall_s,t
+ stemflow_s,t
+ initial_drainage_s,t
+ second_drainage_s,t
```

Require tile-column closure after internal transfers cancel:

```
incident rain at tile top
+ sum beginning canopy stores
+ sum condensation
=
ground liquid receipt
+ sum ending canopy stores
+ sum wet evaporation
```

Require stand closure:

```
sum_t f_t * tile_inputs
=
sum_t f_t * tile_outputs
```

Require separate energy closure per occupancy and at stand aggregation.

No producer-supplied zero residual is acceptance evidence.

## Initial State and Migration

The V2 initial state must contain exactly one liquid-state lane for every occupied `(s,t)` pair.

Automatic migration from V1 is permitted only where the mapping is unique:

### Zero store

If the old aggregate store is exact zero:

```
S_liq,s,t = 0 for every occupied tile
```

### Single occupied tile

For one occupied tile with coverage `C_s`:

```
S_liq,s,t = S_liq,s,V1 / C_s
```

because:

```
S_liq,s,V1 = C_s * S_liq,s,t
```

### Multiple occupied tiles with nonzero store

Do not invent a distribution.

Require caller-supplied V2 occupancy stores, or return an exhaustive unresolved-state report.

An optional explicit migration operation such as “uniform local wetness” may be admitted only if the authority package independently reviews it, names it as a migration assumption, records provenance, and never executes silently.

The RHESSys migration adapter must report the new occupancy-state requirements and cannot fill them from parser defaults.

## Required Authority Artifacts

Create at least:

* `artifacts/required-reading-map.md`
* `artifacts/topology-state-problem-statement.md`
* `artifacts/candidate-rule-comparison.md`
* `artifacts/selected-tile-liquid-rule.md`
* `artifacts/area-and-unit-ledger.md`
* `artifacts/state-schema-amendment.md`
* `artifacts/column-routing-contract.md`
* `artifacts/wet-energy-coupling-contract.md`
* `artifacts/occupancy-water-transaction-contract.md`
* `artifacts/migration-and-compatibility-disposition.md`
* `artifacts/equation-authority-addendum.md`
* `artifacts/model-identity-and-digest.md`
* `artifacts/test-vector-ledger.md`
* `artifacts/reference-acquisition-ledger.md`
* `artifacts/contract-amendment-evidence.md`
* `artifacts/contract-test-evidence.md`
* `artifacts/pre-implementation-authority-gate.md`
* `artifacts/gate-results.md`
* `artifacts/review_agent_a.md`
* `artifacts/review_agent_b.md`
* `artifacts/review-finding-disposition.md`
* `artifacts/verification_agent_a.md`
* `artifacts/verification_agent_b.md`
* `artifacts/final-disposition.md`
* `artifacts/worker-handoff.md`

## Required Canonical Amendments

At minimum:

* `SC-VEGETATION-001`;
* science-contract index;
* new V2 model-definition JSON;
* model-stack authority lifecycle records;
* current implementation package’s frozen authority reference and HOLD-lift condition.

Amend these only if required by the selected interfaces:

* `SC-LANDSURFACEENERGY-001`;
* `SC-WATBAL-001`;
* unit-governance or typed resource-contract surfaces.

Do not amend unrelated contracts.

## Independent Oracle Vectors

Add independent vectors for:

1. One stratum occupying two tiles with unequal `f_t`.
2. Different incident rain in the two tiles.
3. Different beginning stores in the two tiles.
4. Same stratum beneath different upper-canopy columns.
5. Two vertical ranks with tile-local throughfall and drainage.
6. Stemflow bypass to ground.
7. Condensation and second drainage in only one tile.
8. One empty tile.
9. Single-tile reduction to the existing result.
10. Homogeneous two-tile case reducing to the expected weighted result.
11. Tile-order permutation invariance.
12. Occupancy request weighting and authorization back-conversion.
13. Local closure in every tile.
14. Stand closure after weighting.
15. Exact rollback after a tile-local failure.

Required poisons:

* replicate the complete stand store into every tile;
* divide by the wrong area basis;
* aggregate incident rain before E04;
* average tile-local E04 outputs and treat that as local routing;
* route drainage into the wrong tile;
* send stemflow through lower foliage;
* omit second drainage;
* apply `f_t` twice;
* omit `f_t`;
* average wet fraction before energy solve;
* aggregate PAR before nonlinear photosynthesis;
* use one occupancy’s authorization for another;
* duplicate or omit an occupancy state lane.

Include a deliberately nonlinear vector proving:

```
sum_t f_t * E04(P_t, S_t)
!=
E04(sum_t f_t * P_t, sum_t f_t * S_t)
```

for at least one accepted operand.

## Stage-A Reviews and Gates

Subagent authorization: this directive explicitly authorizes and requires:

* one canopy-interception/topology/energy science reviewer;
* one coupled-state/resource-transaction science reviewer;
* one comparator suite runner for heavy contract/full-workspace gates;
* two independent terminal verifiers.

Both science reviewers must inspect:

* local versus stand basis;
* persistent state;
* routing;
* wet-energy coupling;
* occupancy water transactions;
* migration;
* model identity;
* all poison vectors.

Every finding is dispositioned and accepted findings are fixed.

Run all applicable:

```
bash tools/release/check_science_contract_admission.sh \
  --base-ref 02631ae92af6b073ed7957592fef4bad68dcf77f \
  --worktree

bash tools/release/check_authority_suite_antievasion.sh

cargo nextest run \
  --test auth11_required_suite_obligation_guards_contract

cargo nextest run \
  --test vegetation_boundary_authority_contract \
  --profile quick

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo test --doc --workspace
cargo deny check
git diff --check
```

Also run all affected contract unit, schema, Markdown, digest, oracle, and ownership checks.

Use external scratch outside the checkout for heavy runs.

The authority package may complete only when:

* V2 authority is exact and digest-bound;
* V1 remains immutable;
* every heterogeneous-tile join is defined;
* both independent science reviews pass;
* both terminal verifiers pass;
* all required gates pass;
* the implementation HOLD has an exact released lift condition.

# Stage B: Resume the Existing Implementation Package

After the authority package reaches:

`COMPLETE / OPENWEPP_C3_WOODY_V2 implementation authority released`

resume:

`docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/`

Do not create a new implementation package.

Change its status to:

`executing / V2 tile-liquid authority admitted / remediation active`

Preserve the historical V1 and HOLD records.

## Stage-B Intake

Before production edits:

1. Freeze the authority-package terminal commit and V2 digest.
2. Update the implementation package’s authority predecessor.
3. Update its model identity from V1 to V2.
4. Update the equation-module map and state-ownership map.
5. Update the write set if occupancy water DTOs require bounded changes.
6. Rerun Milestone 0 authority, digest, unit, and anti-evasion gates.
7. Keep the fail-closed heterogeneous guard until the complete V2 path is implemented and tested.
8. Do not delete old V1 checkpoint tests; add explicit V1 historical/nonexecution checks.

## Implement the V2 State Schema

Update configuration and state:

* exact occupancy IDs;
* one tile-liquid lane per `(s,t)`;
* occupancy-local warm starts selected by authority;
* strict missing/duplicate/extra lane rejection;
* exact local/stand basis;
* deterministic serialization;
* state digest over every occupancy lane;
* V2 model-definition digest checks;
* V1 state rejection except through the explicit migration tool.

Implement migration:

* zero-store expansion;
* single-tile area conversion;
* unresolved nonzero multi-tile state report;
* no silent uniform distribution.

## Implement Tile-Local E04 and Column Routing

Replace the temporary unsupported branch with the complete V2 mechanism.

Execution order for every tile:

1. Read tile-local rain at the column top.
2. Traverse strata by rank.
3. Use occupancy-local store and conditional plant area.
4. Execute full E04.
5. Route throughfall and drainage to the next lower rank.
6. Route stemflow directly to ground.
7. Carry local wet fraction into energy.
8. Preserve occupancy-local accepted state.
9. Route terminal releases to the tile ground recipient.
10. Weight only after local routing is complete.

No call may read or write a shared mutable aggregate store.

## Implement Occupancy-Local Coupled Solves

Use tile-local:

* radiation;
* wet/dry surface area;
* leaf and stem energy;
* canopy-air state;
* FvCB and Medlyn state;
* hydraulic warm starts;
* water requests.

Submit stand-ground occupancy requests and convert authorizations back to local basis exactly once.

Aggregate accepted tile fluxes to shared stratum carbon and nitrogen state.

Add explicit tests for:

* different tile radiation but common stratum C/N;
* different tile wetness;
* different water caps;
* aggregate GPP;
* aggregate transpiration;
* shared LAI update;
* no C/N duplication.

## Complete Remaining Accepted Rust Findings

The authority lift does not waive the remaining review findings.

### Numerical errors and diagnostics

Typed numerical failures must carry:

* solve identity;
* iteration count;
* residual vector or norm;
* step norm;
* active bounds;
* backtracking count;
* pivot/singularity detail where applicable.

Aggregate diagnostics must include:

* `ci` solve;
* canopy energy solve;
* outer coupled solve;
* hydraulic solve;
* tile/stratum identity;
* unit-normalized acceptance status.

Do not combine residuals with incompatible units without the canonical scale.

### Multirank final-liquid handoff

After condensation or active wet-store re-solve, recompute final drainage and route it through the correct lower occupancy in the same tile.

Do not preserve a pre-energy throughfall result after final store/condensation changed.

### Independent energy reconstruction

The energy owner must not clone vegetation’s proposed ledger and compare it with itself.

Expose immutable component operands:

* direct/diffuse VIS/NIR;
* absorbed leaf and stem shortwave;
* longwave incident/emitted;
* sensible heat;
* transpiration amount;
* wet evaporation/condensation amount;
* dry-stem term;
* ground term;
* storage/conductive term;
* interval;
* authority-tagged latent enthalpy.

The energy owner independently computes its candidate and receipt, including:

```
Q_T = -h_v * finalized transpiration
```

It must reject:

* authorization substituted for finalized use;
* duplicate latent debit;
* omitted stem energy;
* canopy/ground substitution;
* rate/amount substitution;
* wrong interval;
* wrong tile;
* wrong model digest.

### Independent BGC reconstruction

BGC must independently validate:

* `(layer, NH4|NO3)` inventory;
* request, authorization, finalized use;
* exact debit;
* material proposal identity;
* donor class;
* receiver class;
* C;
* N;
* dry matter;
* transaction identity.

Do not accept a producer-consistent copy as independent proof.

### All-owner commit API

Remove or make inaccessible any public vegetation-only commit API capable of committing outside the complete owner set.

The only closure-eligible commit must include:

* vegetation;
* diagnostic water owner;
* biogeochemistry;
* energy owner;
* immutable transaction identity.

All candidates validate before one atomic commit.

### Duplicate request identity

Reject duplicate identity before deterministic proportional summation.

Identity includes:

* transaction;
* owner;
* occupancy where applicable;
* layer;
* resource species/type;
* amount basis.

Sorting and compensated summation do not make duplicates valid.

### Stem shortwave roundoff

Remove unconditional `.max(0.0)` behavior.

Use only a contract-authorized, provenance-bound floating representation tolerance. Values below the negative tolerance fail typed. Values within the exact admitted roundoff interval may normalize to zero with diagnostics and tests.

### File decomposition

`transaction.rs` exceeds the 2,000-line warning threshold.

Split by responsibility, for example:

* `transaction/mod.rs`
* `transaction/validation.rs`
* `transaction/radiation.rs`
* `transaction/liquid.rs`
* `transaction/physical.rs`
* `transaction/carbon_nitrogen.rs`
* `transaction/ledgers.rs`
* `transaction/commit.rs`
* `transaction/diagnostics.rs`

Do not hide semantic changes inside the split. Record line counts and ownership.

## Stage-B Test Families

Maintain distinct:

1. A0 authority tests.
2. V2 model-definition and schema tests.
3. Pure E01–E22 conformance tests.
4. Tile-local E04 and routing tests.
5. Occupancy resource transaction tests.
6. Independent energy/BGC reconstruction tests.
7. Whole-candidate transaction tests.
8. Phase-injection byte-identical rollback tests.
9. Diagnostic consumer tests.
10. Legacy selector and V1 nonactivation tests.

Expected values must come from committed digest-bound fixtures independently regenerated by the Python oracle.

## Focused Review Gate

Before any heavy gate, require fresh science and Rust reviews.

Science reviewer must prove:

* V2 state semantics;
* local/stand basis;
* nonlinear routing;
* E01–E22 causal path;
* occupancy water requests;
* shared C/N update;
* no hidden lateral mixing;
* independent five-ledger closure.

Rust reviewer must prove:

* typed identity;
* deterministic serialization;
* numerical diagnostics;
* no partial commit;
* all-owner rollback;
* no producer-self-validation;
* no V1/V2 digest confusion;
* no duplicate request;
* line-count governance.

Fix all accepted findings and repeat both reviews on stable bytes.

## Stage-B Heavy and Terminal Gates

Only after focused reviews pass, spawn the comparator runner for:

```
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile full
cargo test --doc --workspace
cargo deny check
cargo fmt --all -- --check
git diff --check
```

Run all package benchmarks with recorded hardware and fixture identity.

Complete exact terminal-diff reconciliation.

Spawn two independent terminal verifiers.

Archive the active implementation kickoff prompt only after both verifiers pass.

## Completion Status

The implementation package may close as:

`COMPLETE / OPENWEPP_C3_WOODY_V2 science implementation complete / default-off diagnostic only`

Only if:

* V2 authority package is complete;
* all heterogeneous-tile state and routing is implemented;
* all prior science and Rust findings are corrected;
* public execution invokes the complete V2 state machine;
* every owner independently reconstructs its ledger;
* every failure rolls all owners back byte-identically;
* diagnostic execution uses the real public path;
* heavy gates pass;
* both reviews and both terminal verifiers pass;
* no finding remains undispositioned.

The final disposition must still state:

* no runtime activation;
* no production consumer cutover;
* no calibration;
* no independent validation;
* no canopy snow;
* no soil biogeochemical transformations;
* no transferability claim;
* `calibration_evidence_status=NOT_CALIBRATION_READY`;
* `identifiability_status=NOT_ASSESSED`.

Autonomy: execute both stages through their truthful terminal dispositions without requesting additional user direction unless a new, exact, independently supported authority or external-tool blocker remains after all safe in-scope routes are exhausted.
