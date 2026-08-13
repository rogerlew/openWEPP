# Implement the Coupled C3 Forest Vegetation State Machine

Status: `executing / V3 potential-pass remediation active / shared C/N authority hold scoped`

Package ID: `20260811-coupled-c3-forest-vegetation-state-machine-implementation-001`

Plan class: `Critical contract-backed kernel implementation campaign`

This ExecPlan is a living document maintained under `docs/codex_exec_plans.md`.
The sections `Progress`, `Surprises & Discoveries`, `Decision Log`, and
`Outcomes & Retrospective` must remain current throughout execution.

## Purpose / Big Picture

Implement the approved `OPENWEPP_C3_WOODY_V3` scientific definition as a
deterministic, default-off Rust state machine. At completion, a caller can load
one digest-bound canonical configuration and complete initial state, execute the
entire multistratum radiation--interception--gas-exchange--energy--hydraulic--C/N
transaction through an explicit diagnostic harness, and obtain independently
reconstructible candidate state and transfer ledgers. Invalid inputs,
unsupported branches, resource denial, and solver failure leave all beginning
state byte-identical.

This package implements no production selector, default behavior change,
cutover, calibration, empirical validation, canopy snow, or soil C/N
transformation. The package is indivisible scientific implementation authority:
an internal milestone may be stable and committed, but no partial water-only,
phenology-only, diagnostic-photosynthesis, immutable-N, or final-LAI model may
close independently.

## Authority And Frozen Identity

The implementation must conform exactly to:

- `SC-VEGETATION-001` v7, approved/active;
- `SC-VEGETATIONTRANSACTION-001` v1, approved/active;
- `SC-BIOGEOCHEM-001` v1, approved/active;
- `OPENWEPP_C3_WOODY_V3`, SHA-256
  `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`;
- V3 potential-pass authority terminal commit
  `94a4c99dc1228aa0399c01f4cc9590742960028f`;
- the equation, numerical, schema, ownership, and vector artifacts in
  `20260811-coupled-c3-forest-vegetation-model-stack-authority-001/`.

The model-definition JSON has exactly one production identity. Embed the
authoritative bytes with `include_bytes!` from a stable tracked registry path,
and require byte equality and SHA-256 equality with the admitted package copy.
Do not maintain two editable model definitions. Any equation, constant,
algorithm, supported branch, or digest-bound section change is out of scope and
requires contract-first authority plus a new model identity.

## Implementation Intent And Claims

Intent is `science implementation + deterministic execution + implementation
readiness evidence`. Calibration-readiness, empirical calibration, independent
validation, runtime activation, and consumer cutover are not claimed. The
terminal readiness vocabulary is:

- `science_implementation_status=IMPLEMENTED` only after every E01--E22 family,
  typed guard, ownership boundary, and transaction path passes;
- `calibration_evidence_status=NOT_CALIBRATION_READY`;
- `identifiability_status=NOT_ASSESSED`.

Demonstration values are `ASSUMED_FOR_EXECUTION`, digest-bound in fixtures, and
are not defaults, observations, priors, calibrated values, physiological
bounds, or transferability evidence.

## Included Scope

- New dependency-neutral resource transaction DTOs in
  `openwepp-kernel-contract`.
- New `openwepp-vegetation` crate containing strict schema, configuration,
  state, topology, radiation, interception, gas exchange, energy, hydraulics,
  carbon/nitrogen, phenology, transaction, diagnostics, and typed errors.
- New `openwepp-biogeochemistry` crate containing the admitted mineral-N
  arbitration, litter/CWD receiving state, atomic candidate transaction, and
  typed errors. Decomposition and other transformations remain absent.
- Bounded coordination in `openwepp-hillslope-orchestrator` for immutable
  transaction identity, candidate execution, owner validation, and atomic
  commit against typed arbiter traits.
- A default-off diagnostic executable or test harness that is not reachable
  from the production runtime selector and proves the complete coupled state
  machine with deterministic fixtures.
- An offline, versioned RHESSys definition migration tool that preserves raw
  source provenance and either produces a complete canonical configuration or
  a structured unresolved-field report. It never supplies hidden values.
- Digest-bound expected-vector fixtures generated from, and independently
  checked against, the Python oracle.
- Benchmarks and bounded performance evidence for expensive radiation and
  coupled-solver paths.

## Excluded Scope

- Runtime selector changes, default activation, production consumer cutover,
  publication, deployment, or remote operations.
- New or amended constitutive science, runtime switches between scientific
  models, hidden defaults, parameter recommendation, calibration, validation,
  or transferability claims.
- Canopy snow; C4, crops, nonvascular vegetation, recruitment, succession,
  fire, and catastrophic disturbance.
- Soil decomposition, immobilization, mineralization, nitrification,
  denitrification, leaching, gaseous loss, or endogenous nutrient replenishment.
- Calm/nonneutral aerodynamic fallback. Unsupported forcing fails typed; no
  minimum-wind floor or silent neutral substitution is allowed.
- Agricultural `Kcb`/LAI PMET donation, scalar root stress, direct vegetation
  mutation of soil water, or profile averaging.

## Repository Orientation And Crate Graph

`openwepp-kernel-contract` owns dependency-neutral request, authorization,
finalized-use, transaction-ID, and receipt types. `openwepp-vegetation` depends
on that contract crate and owns vegetation calculations/state.
`openwepp-biogeochemistry` also depends on the contract crate and owns admitted
mineral-N and material receivers. Neither implementation crate depends on the
other or on hydrology. `openwepp-hillslope-orchestrator` depends on both and
coordinates candidates through traits; this prevents ownership cycles.

The expected dependency direction is:

    openwepp-kernel-contract
       ^                 ^
       |                 |
    vegetation     biogeochemistry
       ^                 ^
        \               /
         hillslope-orchestrator
                  ^
                  |
        default-off diagnostic harness

`artifacts/crate-and-dependency-graph.md` freezes exact Cargo features and
dependency edges before code edits. `artifacts/equation-module-map.md` maps
E01--E22 to modules and tests. No new edge may create a cycle or allow
vegetation to mutate another owner's storage.

## Required Interfaces

Exact field sets come from the canonical contracts. Before implementation,
freeze Rust signatures equivalent to these semantics:

    pub struct ResourceRequest<K, Q> { transaction_id, owner_id, key: K, amount: Q }
    pub struct MaximumAuthorization<K, Q> { transaction_id, owner_id, key: K, amount: Q }
    pub struct FinalizedUse<K, Q> { transaction_id, owner_id, key: K, amount: Q }

    pub trait WaterArbiter {
        fn authorize(&self, requests: &[WaterRequest])
            -> Result<Vec<WaterAuthorization>, WaterArbitrationError>;
    }

    pub trait NitrogenArbiter {
        fn authorize(&self, requests: &[NitrogenRequest])
            -> Result<Vec<NitrogenAuthorization>, NitrogenArbitrationError>;
    }

    pub fn execute_candidate(
        definition: &ModelDefinition,
        config: &VegetationConfiguration,
        beginning: &CoupledBeginningState,
        forcing: &SnowFreeForcing,
        water: &dyn WaterArbiter,
        nitrogen: &dyn NitrogenArbiter,
    ) -> Result<CoupledCandidate, CoupledExecutionError>;

    pub fn validate_and_commit(
        beginning: &mut CoupledOwnedState,
        candidate: CoupledCandidate,
    ) -> Result<CommitReceipt, CoupledCommitError>;

The implementer may refine names only after recording the reason in the
Decision Log. Semantics are fixed: request, maximum authorization, finalized
use, owner validation, then one atomic commit, with
`0 <= finalized use <= authorization <= request`.

## Configuration, State, And Migration

The schema milestone implements every canonical required field and the complete
initial vegetation/BGC state. Parsing rejects missing fields, unknown consumed
fields, duplicate aliases, unsupported sentinels, nonfinite values, incompatible
units, invalid topology, unsupported lifeforms/processes, and digest mismatch.
No `Default` implementation may create a scientifically executable
configuration or state.

The offline RHESSys adapter has a separate input schema and explicit mapping
table. Its output includes source path/hash, mapping version, raw-to-canonical
field provenance, unresolved required fields, and canonical configuration hash.
It must never make direct RHESSys spelling an alias in the canonical runtime
parser. A conversion succeeds only when the caller supplies every additional
required canonical value and complete initial state.

## Scheduler And Cadence

Before code edits, `artifacts/scheduler-and-cadence-decision.md` must bind the
execution interval, forcing sampling, phenology/GSI update order, persistent
temperature update, radiation traversal, interception carry, gas/energy/
hydraulic solve, C/N finalization, turnover, ledger reconstruction, and commit
order to contract sections. Rate-to-amount conversions name `dt` exactly once.
No module may infer its own timestep or commit independently.

## Milestones

### Milestone 0: Freeze implementation surfaces

Complete the required-reading map, implementation intent, exact production
write set, crate graph, E01--E22 mapping, operand-provider map, state ownership,
scheduler/cadence, model embedding, migration adapter, diagnostics, performance
budget, and milestone gate matrix. Run A0 admission and contract/schema checks.
No production edit is permitted until the pre-implementation gate is `PASS`.

### Milestone 1: Strict identity, configuration, and state

Add workspace members and dependency-neutral transaction DTOs. Implement model
identity loading, strict canonical configuration/initial-state parsing,
topology validation, typed exclusions, deterministic serialization, candidate
state wrappers, and the offline RHESSys migration adapter. Prove round-trip
byte identity, no defaults, exhaustive missing/unknown-field failures, digest
rejection, mixed-stratum identity, and structured unresolved migration output.

### Milestone 2: Radiation, topology, and interception (E01--E06)

Implement top-down direct/diffuse VIS/NIR propagation, sunlit/shaded scaling,
finite leaf/stem liquid stores, throughfall, stemflow, drainage, wet fraction,
carry, and wet/dry surface ownership. Reproduce oracle values and poison
wrong-sign, wrong-coefficient, big-leaf, omitted-stem, area-basis, rate/amount,
and non-donation aliases. This is an internal verified increment, not a usable
vegetation endpoint.

### Milestone 3: Coupled gas exchange, energy, and hydraulics (E07--E15)

Implement bounded FvCB, temperature responses, Medlyn stomata, boundary-layer
and aerodynamic resistance, leaf/canopy-air energy nodes, four-potential plant
hydraulics, layer requests, active authorization caps, `beta_hyd`, and the
admitted nested/simultaneous solver. Reproduce zero/saturated light, limitation
transitions, mixed strata, distinct roots, dry/frozen layers, fully supplied and
competing withdrawals, nonconvergence, and rollback vectors. Expose solver
diagnostics without fallback fluxes or hidden floors.

### Milestone 4: Persistent C/N and phenology (E16--E22)

Implement gross carbon gain, maintenance/growth respiration, storage,
allocation, retranslocation, potential/final N demand, finalized mineral-N use,
six-tissue C/N pools, GSI-timed onset/offset, leaf-C/SLA-owned LAI, turnover,
mortality, and exact litter/CWD proposals. Prove N-limited carbon remains in
NSC, donor/receiver C/N/dry-matter closure, deciduous/evergreen behavior, and no
immutable-N or independent terminal-GSI state.

### Milestone 5: Whole candidate transaction and atomic commit

Combine E01--E22 with typed water/N arbiters and BGC receiving owner. Validate
water, energy, carbon, nitrogen, and dry-material ledgers from independently
exposed operands before committing exactly once. Inject failures at every
phase and prove byte-identical rollback for all owners. Implement the zero-
transformation BGC branch and `BGC-E-040` when transformations are required.

### Milestone 6: Default-off diagnostic consumer and implementation closure

Exercise the complete state machine through a diagnostic harness that consumes
the same public crate interfaces a later real integration will use. Prove the
production runtime selector and legacy PMET/GSI-final-canopy paths are unchanged
and cannot select this model. Run benchmarks, A1/A3 authority vectors, exact
closure audits, Critical gates, dual reviews, and dual terminal verification.
Close only `science implementation`; the later land-surface-energy, hydrology
arbitration, real-consumer, soil-transformation, calibration, and cutover
packages remain explicit dependencies.

## Operand Providers And Output Acceptance

Before production edits, `artifacts/forcing-and-operand-provider-map.md` records
every forcing/state operand, unit, temporal basis, area basis, owner, source,
and missing/unsupported behavior. `artifacts/conservation-and-anti-tautology.md`
records output operands and rejected aliases.

Acceptance requires poison vectors separating all plausible area, interval,
sign, coefficient, aggregation, owner, and authorization/final-use aliases;
independent reconstruction of water, energy, C, N, and dry-material ledgers;
and exact rollback. Producer self-consistency and one-sided bounds are sanity
evidence only.

## Diagnostics And Performance

Required diagnostics include model/config/state identities, transaction ID,
stratum/layer identity, solver iteration count, normalized residuals, active
bounds, backtracking, pivot failures, authorization activity, gas/hydraulic
transpiration mismatch, ledger residuals, and typed failure. Diagnostics are not
mutable authority or fallback inputs.

Benchmark configuration parsing, radiation coefficient/integral construction,
one stratum-day solve, a vertically overlapping mixed stand, active water/N
competition, and rollback. `artifacts/performance-budget.md` freezes measured
baseline hardware/commands and acceptance budgets before optimization.
Digest-keyed precomputation is allowed only for configuration-invariant values
and must be bitwise or tolerance-equivalent under the admitted numerical
contract.

## Intended Write Set

- This package tree and bounded catalog/roadmap/backlog lifecycle updates.
- Root `Cargo.toml` and `Cargo.lock`.
- `crates/openwepp-kernel-contract/**` for dependency-neutral transaction DTOs.
- New `crates/openwepp-vegetation/**`.
- New `crates/openwepp-biogeochemistry/**`.
- Bounded `crates/openwepp-hillslope-orchestrator/**` coordination only.
- A versioned offline RHESSys adapter under the vegetation crate or a separately
  justified dependency-light migration crate recorded before edits.
- Contract-derived integration tests, oracle-generated expected-vector
  fixtures, diagnostic-harness tests, and benchmarks.
- Stable model registry bytes at one path selected in
  `artifacts/model-definition-embedding-strategy.md`.

Canonical science contracts are read-only for implementation. If execution
discovers missing or contradictory authority, stop before the affected code,
record the exact authority blocker, and open a separately authorized
contract-first amendment. No production runner selector, CLI default, output
publication, deployment, or unrelated crate is in the write set.

## Security, Rights, And Data Impact

Security impact is `bounded`: no secrets, credentials, network calls, unsafe
code, external messages, or deployment. Scientific source PDFs remain
gitignored and are not copied into fixtures. Migration fixtures must be small,
synthetic or affirmatively redistributable, and retain provenance/license
records. Input parsing is fail-closed; allocation/iteration limits are bounded;
`unsafe` is prohibited unless separately reviewed with a `// SAFETY:` invariant.

## Validation Strategy

This is a Critical kernel campaign. Each milestone closes its direct unit,
property, negative, serialization, oracle-vector, typed-error, rollback, and
affected integration gates before the next milestone. A0 remains admitted; A1
hard invariants and A3 constitutive vectors are non-deferrable where touched.

Run focused/edit-loop commands directly. At campaign closure require at least:

    bash tools/release/check_science_contract_admission.sh --base-ref <frozen-base> --worktree
    bash tools/release/check_authority_suite_antievasion.sh
    cargo nextest run --test auth11_required_suite_obligation_guards_contract
    cargo nextest run -p openwepp-vegetation --profile quick
    cargo nextest run -p openwepp-biogeochemistry --profile quick
    cargo nextest run -p openwepp-hillslope-orchestrator --profile quick
    cargo nextest run --test vegetation_boundary_authority_contract --profile quick
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo test --doc --workspace
    cargo deny check
    git diff --check

Select exact commands from `docs/standards/testing-and-gate-strategy.md` after
reconciling the terminal diff. Critical full-workspace and other heavy batch
runs must use the required comparator runner. Use absolute scratch outside the
checkout for any `TMPDIR` override. Retain failures and retries truthfully.

## Reviews And Verification

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation, subject to standing session-level user authorization, to:

- one Rust correctness reviewer for crate graph, ownership, errors, solver,
  transaction, and rollback, writing only `artifacts/review_agent_a.md`;
- one independent science/closure reviewer for E01--E22, units, area/time
  bases, poison vectors, C/N/material ownership, and claims, writing only
  `artifacts/review_agent_b.md`;
- one `comparator_suite_runner` for heavy authority, full-workspace, benchmark,
  and broad closure commands, writing only package gate-log artifacts; and
- two independent terminal verifiers, writing only their respective
  `verification_agent_*.md` artifacts.

Subagent requirement: REQUIRED during execution. The parent must not run heavy
batch/closure commands when the comparator runner is available. Every review
finding is dispositioned `accepted`, `rejected`, `deferred`, or `follow-up` with
rationale; accepted findings are fixed and invalidated gates rerun. Reviews and
verifiers check gate legitimacy, exact terminal bytes, no partial-authority
closure, and line-count governance, not artifact presence alone.

## Exit Criteria

- Every E01--E22 family is production Rust with the admitted equations,
  constants, algorithms, domains, guards, diagnostics, and error precedence.
- Strict configuration and complete initial state have no executable defaults;
  model bytes/digest and canonical section digests match authority.
- Distinct evergreen, deciduous, and overlapping strata retain identity; LAI
  derives only from coupled leaf C and SLA.
- Water/N request, authorization, finalized use, validation, and atomic commit
  pass, including denial/competition and byte-identical rollback.
- Independent water, energy, C, N, and dry-material reconstruction passes and
  all required poison vectors reject plausible wrong implementations.
- The offline RHESSys adapter either yields a complete canonical object or an
  exhaustive unresolved-field report without hidden filling.
- BGC receiving state works; transformations remain typed zero/failure and no
  unlimited nutrient source exists.
- The complete default-off diagnostic consumer runs while production selection
  and behavior remain byte/route unchanged.
- Exact-diff reconciliation, A0/A1/A3, focused, Critical, formatting, lint,
  deny, doctest, benchmark, and full-workspace requirements pass.
- Dual reviews have no undispositioned material findings; dual terminal
  verifiers pass final exact bytes.
- Calibration status remains `NOT_CALIBRATION_READY`; canopy snow, nonneutral/
  calm aerodynamics, soil transformations, real consumer integration, and
  cutover remain explicitly unclaimed.

## Progress

- [x] (2026-08-11) Scientific authority package released the exact coupled
  model and scaffolded this successor queue card.
- [x] (2026-08-11) Expanded the queue card into a complete autonomous ExecPlan,
  prompt lifecycle, and queued evidence scaffold; no production edit made.
- [x] (2026-08-11) Executed Milestone 0: froze implementation surfaces and
  passed model digest, oracle, and A0 pre-implementation gates.
- [x] (2026-08-12) Preserved the historical V1 HOLD, completed the separate
  contract-first topology authority package, and froze its released V2 bytes at
  commit `817b082d01d194cde61b1cf284bd85e40e44afc9`.
- [x] (2026-08-12) Reran Milestone 0 against the admitted V2 authority: exact
  digests/oracle, admission, unit checks, anti-evasion, AUTH11, A0 authority,
  formatting, Markdown, and diff hygiene all pass.
- [x] (2026-08-12) Execute Milestone 1: identity, strict
  configuration/state, and migration. V2 state now separates shared stratum
  pools from exact occupancy-local liquid/numerical lanes, binds complete
  state/configuration identity, enforces transaction lineage, and admits only
  the exact zero/single-tile V1 liquid conversions. The public candidate
  remains fail-closed before E04.
- [x] (2026-08-12) Implement Increment 2A's internal V2 tile-column engine:
  deterministic top-to-bottom occupancy ordering, exact conditional area,
  same-tile routing, one-time stand weighting, controlled fixed-cap plumbing,
  independent occupancy/column/stand closure, and failure isolation. This does
  not complete Milestone 2 or 3 and does not reactivate public execution.
- [x] (2026-08-12) Audit Increment 2B before constitutive integration. Two
  independent audits confirmed missing canonical leaf/stem radiation mixing,
  local-wind, hydraulic-path/state, beta-one residual, and exact-vector rules.
  Removed all inferred physics, retained only typed resource-boundary work, and
  recorded `artifacts/potential-pass-hold-legitimacy-audit.md`.
- [x] (2026-08-12) Preserve the V2 HOLD and Increment 2A evidence, complete the
  contract-first V3 potential-pass authority package, and freeze its released
  bytes at commit `94a4c99dc1228aa0399c01f4cc9590742960028f`.
- [ ] Migrate executable identity and occupancy state to V3's scalar common
  root node without normalizing ambiguous V2 layer warm starts.
- [ ] Execute Milestone 2: radiation, topology, and interception.
- [ ] Execute Milestone 3: gas exchange, energy, and hydraulics.
- [ ] Execute Milestone 4: persistent C/N and phenology.
- [ ] Execute Milestone 5: whole transaction and atomic commit.
- [ ] Execute Milestone 6: diagnostic consumer and Critical closure.
- [ ] Complete dual reviews, finding disposition, dual terminal verification,
  prompt archival, and truthful final disposition.

## Surprises & Discoveries

- Observation: the released successor initially contained only a README and a
  35-line queue card.
  Evidence: intake inspection found no prompt lifecycle, artifact scaffold,
  declared production write set, milestone gates, or review authorization.
- Observation: the authority model JSON bytes hash directly to the released
  identity; no canonicalization step is needed or permitted.
  Evidence: pre-implementation `sha256sum` returned
  `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`.
- Observation: the missing heterogeneous E04 topology authority was supplied
  by the released V2 successor without rewriting V1 or the historical HOLD.
  Evidence: Stage-A final disposition and both terminal verifiers pass at
  commit `817b082d01d194cde61b1cf284bd85e40e44afc9`.
- Observation: a closure poison that supplied an out-of-domain replicated
  store failed earlier at the occupancy-result domain guard, so it could not
  isolate the independent closure guard.
  Evidence: the first 2A crate run passed 47/48; replacing it with an in-domain
  full-store replication makes the intended independent closure guard fire.
- Observation: V2 closes occupancy liquid topology but does not close the
  leaf/stem optical reduction or several potential E11--E15 operand/state joins.
  Evidence: two independent audits found no canonical equations for these
  mappings; attempted historical/inferred formulations were removed before
  acceptance.
- Observation: V3 closes all five constitutive/schema gaps and the independent
  vector gap without changing the valid V2 tile-column topology.
  Evidence: the V3 authority package passed dual science review, 2,481/2,481
  full-workspace tests, and dual terminal verification at commit `94a4c99dc`.

## Decision Log

- Decision: use two implementation crates plus dependency-neutral transaction
  DTOs and orchestrator coordination.
  Rationale: vegetation and biogeochemistry own different mutable state and
  must not acquire circular dependencies or mutate hydrology directly.
  Date/Author: 2026-08-11 / Codex, implementing user direction.
- Decision: make the terminal consumer diagnostic and default-off, not a runtime
  selector or cutover.
  Rationale: production implementation is authorized; selector/cutover and the
  later real LSE/hydrology consumer remain explicitly unauthorized.
  Date/Author: 2026-08-11 / Codex, implementing user direction.
- Decision: keep the Python oracle independent and consume committed,
  digest-bound expected vectors in normal Rust tests.
  Rationale: Rust tests must not require a local `.venv`, while a separate
  authority gate must prove fixtures regenerate exactly from the oracle.
  Date/Author: 2026-08-11 / Codex, implementing user direction.
- Decision: place V2 routing in `column.rs` behind an occupancy-solver trait,
  and keep `execute_candidate()` unchanged.
  Rationale: topology, area conversion, release routing, and independent water
  reconstruction can be tested without inventing E11--E15 physics or exposing
  a partial public endpoint.
  Date/Author: 2026-08-12 / Codex, implementing user direction.
- Decision: stop Increment 2B before constructing radiation or physiological
  output and retain the public fail-closed branch.
  Rationale: every available composition requires at least one unadmitted
  constitutive mapping; typed resource validation alone is safe but is not
  potential-solve evidence.
  Date/Author: 2026-08-12 / Codex, applying contract-first fail-closed rules.
- Decision: resume this same implementation package against V3 rather than
  scaffold another implementation package.
  Rationale: V3 is the exact authority lift requested by the HOLD audit; the V1
  HOLD and V2 Increment 2A remain immutable implementation history.
  Date/Author: 2026-08-12 / Codex, implementing user direction.

## Outcomes & Retrospective

Planning outcome only: the implementation successor is now designed to be
autonomously executable, but execution has not started and no production Rust
exists. Update this section after every milestone with achieved behavior,
remaining blockers, gate evidence, and lessons.

Execution outcome: Milestone 0 passed. A compiling scaffold and focused vector
tests were produced, but independent review proved that the public candidate
does not execute E01--E22 and that several helpers use unauthorized proxy or
alternate numerical formulations. The package therefore remains `HOLD`; the
scaffold is not closure-eligible production science and no terminal claim is
made.

Continuation outcome (in progress): commit `c06420688` remains the immutable
failed-scaffold checkpoint. The package resumed in-place to correct every
accepted Review-B finding; no finding is deferred, rejected, or moved to a
follow-on package.

Continuation outcome (HOLD): remediation corrected substantial E01--E22
implementation defects, but fresh review exposed a canonical omission for
heterogeneous-tile E04 liquid-store routing. The exact authority boundary and
lift action are recorded in `artifacts/hold-legitimacy-audit.md`. Ambiguous
topology now fails closed; heavy gates, terminal verification, prompt archival,
and implementation-complete status remain prohibited.

Continuation outcome (active): the legitimate E04 authority HOLD is lifted by
the admitted `OPENWEPP_C3_WOODY_V2` contract package. Historical V1/HOLD records
remain immutable. This package is executing Stage-B intake and will retain its
fail-closed heterogeneous guard until the complete V2 path passes.

Increment 2A outcome: the internal column engine is active and tested with a
controlled constitutive seam. It derives conditional area, routes accepted
throughfall and both drainage terms to descendants, bypasses stemflow to the
same-tile ground, weights layer water exactly once, and reconstructs water
closure outside the solver. The exact potential and authorization-capped
E11--E15 occupancy solvers, public water path, owner candidates, and commit are
still missing; Milestones 2 and 3 remain incomplete.

Increment 2B outcome (HOLD): exact potential execution did not begin because
the released V2 authority remains incomplete at load-bearing radiation and
coupled-solver joins. No proxy solver or radiation handoff remains. Typed water
request/authorization validation is available as non-constitutive foundation;
the public transaction still emits no request or candidate.

V3 continuation outcome (active): commit `94a4c99dc` releases the exact mixed
leaf/stem radiation, canopy wind, common-root hydraulics, coupled uncapped
potential-pass, respiration, diagnostics, and independent vector authority.
This package has resumed against those bytes. The public transaction remains
fail-closed until the authorization-capped second pass passes the exact oracle.

Shared C/N authority outcome (scoped HOLD): a complete canonical audit found
that V3 does not define which leaf-carbon subpool owns LAI and does not define
the numerical semantics of the two persisted previous-offset flux fields. The
exact boundary and lift action are recorded in
`artifacts/cn-state-hold-legitimacy-audit.md`. Work that does not consume those
identities continues; the public transaction and the affected E20--E22
shared-state finalizer remain fail-closed.

## Idempotence And Recovery

All configuration migration and fixture generation must be deterministic and
write to temporary paths before atomic replacement. Candidate execution never
mutates owner state. A failed milestone retains its evidence, restores no user
files destructively, and resumes from the last passing milestone after fixing
the cause. Do not reset unrelated work, delete caches broadly, or reinterpret a
failed gate as later scope.

## Plan Revision Note

2026-08-11: expanded the authority-package successor stub into the complete
contract-backed implementation campaign requested after commit `dea6d358b`.
This revision freezes architecture, scope, milestones, claims, gates, and
handoff boundaries without authoring production Rust.
