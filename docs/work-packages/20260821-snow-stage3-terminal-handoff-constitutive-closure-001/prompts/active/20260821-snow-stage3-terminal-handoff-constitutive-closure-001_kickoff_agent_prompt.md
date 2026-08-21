The prompt below is bound to current `origin/main` at `85d88fb903b302a33d43304a5001911f13f7d8d5`. The corrected package now explicitly holds on the real Stage 3 event solve, shared V11 carrier, typed owner execution, coupled chronology, complete restart, and positive physical scenarios.  

The repository already contains the real ingredients the successor must connect rather than approximate:

- a persistent Stage 3 state and `evaluate_stage3_persistent_day_with_terminal_event()` with event-local mass, liquid, vapor, energy, and bracket diagnostics;  
- an actual V11 segmented transaction and typed seven-owner envelopes; 
- the real V11/V10/LSE/hydrology/BGC/soil-thermal consumer stack; 
- a runner-owned Stage 3 persistent-state surface which currently advances during day-input construction rather than inside an atomic owner transaction.   

```text
# Scaffold and Execute the Actual Stage-3/V11 Terminal-Handoff
# Constitutive Closure

Repository:

`/home/workdir/openWEPP`

Use `/workdir/openWEPP` only when that is the existing checkout. Do not
reclone.

Required starting commit:

`85d88fb903b302a33d43304a5001911f13f7d8d5`

Branch:

`main`

Required remote state:

`origin/main` must resolve to the same commit before edits.

New package:

`docs/work-packages/
  20260821-snow-stage3-terminal-handoff-constitutive-closure-001/`

Execution mode:

`package scaffold →
exact owner/cadence/assurance intake →
contract-bound constitutive integration →
complete restart and scenario qualification →
exact-head terminal closure`

Execute this package end-to-end through one truthful disposition.

Do not start Child 3.

## Authorization Boundary

Authorized:

- local repository reads and edits;
- creation and execution of the new work package;
- bounded default-off Rust implementation in the orchestrator, runner,
  persisted-restart, vegetation-consumer, Stage 3, and test surfaces;
- bounded refactoring required to expose existing typed owner APIs;
- a narrow prospective science-contract amendment only when the exact current
  contracts do not bind a required cadence, owner source, topology mapping, or
  restart boundary;
- typed assurance-identity maintenance using the repository's admitted
  assurance tooling;
- updating stale contract-version tests after exact semantic review;
- local commits at coherent package boundaries;
- independent reviews, comparator execution, and terminal verification.

Not authorized:

- production selector activation;
- default changes;
- CoE retirement;
- production snow-owner cutover;
- production output changes;
- deployment;
- calibration;
- empirical-efficacy or transferability claims;
- canopy-intercepted snow;
- Richards implementation;
- PR creation;
- remote branch creation;
- push.

Do not push unless separately directed by the user.

## Historical Boundaries

Preserve these packages byte-for-byte as historical evidence:

```text
20260819-snow-stage3-terminal-meltout-lse-handoff-implementation-001
    EXECUTED HOLD

20260821-snow-stage3-shared-carrier-terminal-handoff-implementation-001
    EXECUTED HOLD

20260821-snow-stage3-terminal-handoff-live-owner-closure-001
    EXECUTED HOLD
```

Do not rewrite any historical package as though the constitutive closure had
already existed.

The new package consumes their accepted contracts, evidence, findings, and
bounded infrastructure but owns all new implementation and terminal claims.

## Current Accepted Infrastructure

Retain unless a direct correctness defect is proven:

- the ordinary scheduler's internal attachment hook;
- the test-only status of the old caller-built handoff APIs;
- the persistent surface-liquid/WB14 continuation APIs;
- replay and late-failure plumbing;
- immutable publication-batch construction;
- the V9 module split and line-count work;
- the complete Child 2A coupled-time authority;
- the complete Child 2B V11 segmented-support implementation;
- the complete Child 2C carrier/event-boundary authority;
- DirectV10 Restart V1;
- coupled-time Restart V2;
- V11 Restart V3;
- the released persisted-restart authority and implementation;
- the root-zone hydraulic owner;
- the native half-hour forcing/GSI provider.

## Current Rejected Closure Claims

The successor must replace all of these surfaces:

```text
configured event day/lane/tick as physical event authority;

configured canopy and snow temperatures, humidity, conductances, and
longwave weights as live carrier authority;

hard-coded air density or heat capacity where a sealed atmospheric owner
already supplies or derives them;

wind speed used directly as a heat or vapor conductance;

zero-filled snow mass, vapor, liquid, and energy ledgers;

one-participant "shared" carrier receipts;

TerminalStateRates used instead of the actual Stage 3 terminal solver;

Debug-formatted owner payloads;

synthetic hash chains standing in for typed V11/LSE/hydrology/BGC/
soil-thermal candidates;

execution after a complete ordinary day using that production day frame as
the shadow's beginning physical state;

restart of only configuration, receipt-chain, and consumed-marker wrappers;

one-lane, one-surface, calendar-only plumbing fixtures as physical acceptance.
```

No variant of those mechanisms may remain on the closure-eligible path.

# Phase 0 — Exact Intake and Package Scaffold

Before any edit:

```bash
cd /home/workdir/openWEPP 2>/dev/null || cd /workdir/openWEPP

test "$(git rev-parse HEAD)" = \
  "85d88fb903b302a33d43304a5001911f13f7d8d5"

test "$(git rev-parse origin/main)" = \
  "85d88fb903b302a33d43304a5001911f13f7d8d5"

test "$(git branch --show-current)" = "main"

git status --short --branch
git diff --check
```

Require a clean synchronized tree.

Do not pull, reset, rebase, merge over, amend, or replace the starting commit.

Run instruction discovery:

```bash
tools/agents/find-agents --for \
  Cargo.toml \
  crates/openwepp-hillslope-orchestrator \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver \
  crates/openwepp-hillslope-orchestrator/src/v11_vegetation_consumer.rs \
  crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs \
  crates/openwepp-vegetation/src/v11.rs \
  crates/openwepp-persisted-restart-v1 \
  crates/openwepp-runner/src/hillslope \
  assurance/v2 \
  docs/specifications/assurance-amendment-and-identity-workflow.md \
  docs/specifications/science-contracts \
  tests/integration \
  docs/work-packages/20260807-snow-terminal-enthalpy-event-numerics-001 \
  docs/work-packages/20260817-direct-hydrology-persisted-restart-implementation-001 \
  docs/work-packages/20260820-c3-woody-v11-segmented-support-001 \
  docs/work-packages/20260821-snow-stage3-shared-carrier-authority-closure-001 \
  docs/work-packages/20260821-snow-stage3-terminal-handoff-live-owner-closure-001 \
  docs/work-packages/20260819-snow-stage3-production-cutover-campaign-001 \
  docs/ROADMAP.md \
  docs/work-packages/README.md
```

Read all discovered instructions.

## Required scaffold

Create:

```text
docs/work-packages/
  20260821-snow-stage3-terminal-handoff-constitutive-closure-001/
    package.md

    prompts/
      README.md
      active/
        README.md
        20260821-snow-stage3-terminal-handoff-constitutive-closure-001_kickoff_agent_prompt.md
      archived/
        README.md

    artifacts/
      README.md
      exact-intake.md
      required-reading-map.md
      owned-file-manifest.md
      historical-package-protection.md
      current-source-and-owner-map.md
      stage3-cadence-and-terminal-api-audit.md
      v11-consumer-and-owner-map.md
      carrier-operand-source-map.md
      terminal-liquid-lineage.md
      surface-receiver-topology.md
      coupled-parent-chronology.md
      assurance-drift-census.md
      stale-contract-assertion-census.md
      contract-impact-and-preimplementation-gate.md
      contract-implementation-evidence.md
      contract-test-implementation-evidence.md
      implementation-and-test-evidence.md
      canonical-owner-state-ledger.md
      restart-schema-and-equivalence.md
      event-and-replay-matrix.md
      scenario-matrix.md
      failure-injection-matrix.md
      publication-boundary.md
      production-noninterference.md
      comparator-results.md
      performance-and-resource-disposition.md
      line-count-governance.md
      security-impact.md
      calibration-readiness-matrix.md
      exact-diff-reconciliation.md
      gate-results.md
      review_snow_vegetation_science.md
      review_hydrology_ownership.md
      review_rust_correctness.md
      review_rust_qa.md
      review-finding-disposition.md
      verification_agent_a.md
      verification_agent_b.md
      final-disposition.md
      worker-handoff.md
```

When any canonical contract changes, also create the standard contract-cycle
tree for each changed contract:

```text
artifacts/science-contracts/<CONTRACT-ID>/
  contract_ref.md
  review_agent_a.md
  review_agent_b.md
  disposition.md
  verification_agent_a.md
  verification_agent_b.md
```

Copy this complete directive byte-identically into `prompts/active/` and record
its SHA-256.

Measure and record the required-reading byte count before implementation.
Classify it as:

```text
OK
WARN
REQUIRES-JUSTIFICATION
```

according to repository prompt governance.

## Package initial lifecycle

Set the new package to:

```text
EXECUTING /
historical HOLDs preserved /
constitutive closure active /
Child 3 blocked
```

Update:

```text
docs/ROADMAP.md
docs/work-packages/README.md
20260819-snow-stage3-production-cutover-campaign-001/package.md
```

only enough to identify the new package as the active Child-1 successor.

Do not mark Child 1 complete.

Commit the scaffold and exact intake locally.

Suggested message:

```text
Scaffold Stage-3 terminal constitutive closure
```

Do not push.

# Phase 1 — Freeze the Actual Existing Owner Surfaces

Before implementation, produce an exhaustive source inventory.

## 1.1 Existing Stage 3 state and solver

Trace and record:

```text
DirectSnowStage3PersistentState
DirectSnowStage3PersistentDayResult
DirectSnowTerminalEventRequest
DirectSnowTerminalEventResult
Wb11HydrologyKernel::initialize_stage3_persistent_state*
Wb11HydrologyKernel::evaluate_stage3_persistent_day_with_terminal_event
Wb11HydrologyKernel::serialize_stage3_persistent_state
Wb11HydrologyKernel::restore_stage3_persistent_state*
solve_terminal_enthalpy_event
```

For every field in the persistent state and terminal result, classify:

```text
persistent owner state
static configuration identity
forcing input
accepted physical output
diagnostic only
restart-required
parent-ledger operand
terminal-liquid operand
prohibited compatibility alias
```

Do not treat `DirectDayFrame.snow_coupling` summary fields as the primary Stage
3 owner when the persistent Stage 3 result contains the authoritative operand.

## 1.2 Current runner custody

Audit:

```text
DirectProductionDayInputBuilder::snow_stage3_persistent_state
DirectProductionDayInputBuilder::build
DirectProductionSnowFrostAuthority::snow_liquid_partition
```

Freeze the current defect:

```text
the runner-owned RefCell candidate is advanced during day-input construction
before the scheduler's owner transaction can accept or roll it back.
```

Select one final custody rule:

```text
The persistent default-off Stage-3/V11 attachment owns Stage 3 state.
The day-input builder may seed the initial state and produce immutable forcing
receipts, but it may not advance persistent Stage 3 state.
```

The old builder-side persistent path may remain only:

```text
behind historical evaluation-only mode;
or
as a migration/diagnostic projection from the attachment;
```

and must not evolve the same Stage 3 state in parallel.

## 1.3 Existing V11 and complete real-consumer owner

Trace:

```text
VegetationConfigurationV11
V11CoupledOwnedState
V11ParentTransaction
V11OwnerEnvelope
V11CompleteOwnerCandidate
V11SharedResourceOwnerTransition
execute_v11_segment
DirectV11ImportedStack
DirectV11RealConsumerStack
DirectV10RealConsumerShadow
```

Record which object owns or contains:

```text
V11 physical state
LSE-V2 state
direct-hydrology frame
surface-liquid state/configuration
soil-thermal snapshot
BGC state
GSI state
provider cursor
root-zone hydraulic configuration
forcing configuration
accepted interval count
next day index
```

The successor should reuse `DirectV10RealConsumerShadow` as the complete
snow-free physical owner where possible rather than inventing another parallel
V11/LSE/hydrology state.

## 1.4 Current contracts

Read and bind exact current versions of:

```text
SC-COUPLEDTIME-001
SC-LANDSURFACEENERGY-001
SC-SNOWENERGY-001
SC-SNOWFREEZE-001
SC-SURFACELIQUID-001
SC-VEGETATION-001
SC-VEGETATIONTRANSACTION-001
SC-ROOTZONEHYDRAULICS-001
SC-SNOWFREEFORCING-001
```

Do not assume version numbers from historical tests.

# Phase 2 — Reconcile Assurance and Stale Contract Assertions

This phase is mandatory before terminal package closure. It may begin before
Rust implementation, but it must be rerun after any admitted contract change.

## 2.1 Assurance identity census

Compare actual current bytes against:

```text
assurance/v2/identity.lock.json
```

At minimum inspect the bindings for:

```text
SC-SNOWENERGY-001.md
SC-SNOWFREEZE-001.md
all other contracts changed by Child 2A–2C
```

Do not manually edit generated hashes.

Read:

```text
docs/specifications/assurance-amendment-and-identity-workflow.md
assurance/v2/README.md
```

Use the admitted typed assurance planner/amendment commands.

The agent must:

1. inspect the current generation;
2. identify the exact source paths and affected DRAFT report;
3. ask the assurance tool to plan the admitted operation;
4. execute only the typed operation selected by that plan;
5. retain the generated transaction receipt;
6. verify the old-generation-to-new-generation chain;
7. run the operation-specific gates.

Do not use a generic “sync,” “refresh,” “bless,” or hand-authored lock edit.

When the tool determines that `adopt-report-source` is the admitted operation,
use that exact operation and path. When it selects another typed operation,
record why.

A refusal by the assurance tool due to report lifecycle or source
classification is a legitimate assurance-authority blocker and must be
documented exactly.

## 2.2 Stale assertion census

Search all tests and tools for literal old contract-version assertions,
including the current snow wind/source custody guards.

For each assertion classify it as:

```text
current canonical contract guard
historical-artifact guard
superseded guard
invalid broad string assertion
```

Then:

- update current guards to the released contract version and current invariant;
- redirect historical guards to immutable historical artifacts;
- remove only guards made redundant by a stronger current exact contract test;
- never perform a repository-wide blind version-number replacement.

Rerun the exact affected assurance and frost profiles.

Record all changes in:

```text
assurance-drift-census.md
stale-contract-assertion-census.md
```

# Phase 3 — Preimplementation Contract and Cadence Gate

The intended implementation should consume existing released authority.

Before production Rust changes, adjudicate these four exact questions.

## 3.1 Stage 3 parent cadence

The existing persistent Stage 3 public evaluator is day-oriented and internally
operates over hourly forcing. The V11/LSE parent cadence is 1,800 seconds.

Determine whether current authority already admits an extracted Stage 3
support evaluator for one coupled-time parent interval.

The acceptable implementation is:

```text
extract/reuse the existing Stage 3 transition and terminal-event equations
over one admitted TimeSupport without changing equations, constants,
operation order, or error criteria.
```

Forbidden approaches:

```text
run the whole-day solver and proportionally scale its result;
duplicate a daily forcing row;
halve an hourly result;
interpolate a completed daily state;
reuse CoE melt timing;
fit a new event time;
construct TerminalStateRates from a completed day summary.
```

When current contracts do not explicitly admit the 1,800-second Stage 3
consumer boundary, prospectively amend only the exact cadence/consumer
boundary before Rust implementation.

Do not alter Stage 3 constitutive equations.

## 3.2 Snow-covered V11 executor

Determine the exact existing authority for V11 physiology under a snow-covered
ground surface.

The pre-event segment must evolve V11 state. It may not use the snow-free
ground/litter LSE branch.

Define or confirm a typed executor such as:

```rust
DirectV11SnowCoveredRealConsumerStack
```

that imports the unchanged V10/V11 vegetation constitutive cycle while
replacing only the lower-surface boundary with the released shared
V11-canopy/Stage-3-snow carrier.

It must still stage:

```text
vegetation state
root-water debits
BGC NH4/NO3 debits
soil-thermal state where active
canopy liquid
carbon and nitrogen state
material receipts
```

over the accepted pre-event support.

When the released contracts do not bind a required input or state transition,
amend the exact boundary contract first.

Do not run the existing snow-free `DirectV11RealConsumerStack` unchanged over
a snow-covered ground surface.

## 3.3 Terminal receiver topology

Determine how one lane/OFE Stage 3 snow owner maps terminal liquid to the
configured snow-free surfaces.

The mapping must be explicit and digest-bound.

It must support the actual declared topology, including:

```text
open bare-mineral tile
covered forest-litter tile
multiple tiles in one OFE
multiple OFEs/lanes
```

Do not keep:

```text
surface_liquid_configuration.records.len() == 1
```

as the closure domain unless the canonical contract explicitly limits Child 1
to that topology.

When one Stage 3 column supplies multiple surface tiles, use the exact admitted
area/tile split once. Do not infer a split from arbitrary record order.

## 3.4 Complete restart authority

Determine whether the existing additive restart authorities can compose the
complete state without changing any released wire.

Preferred posture:

```text
new additive outer restart envelope
containing existing immutable nested restart payloads
```

Protected nested bytes:

```text
DirectV10 Restart V1
coupled-time Restart V2
V11 Restart V3
released direct-hydrology restart bytes
```

Do not revise existing released wire in place.

## Contract gate

When no canonical amendments are needed, record a reviewed
`NO_CONTRACT_CHANGE_REQUIRED` disposition with exact source references.

When amendments are required:

1. edit contracts prospectively;
2. implement contract-derived tests and vectors;
3. complete two independent authority reviews;
4. disposition all findings;
5. obtain two independent contract verifications;
6. promote contracts only after PASS;
7. rerun assurance identity maintenance;
8. commit the exact authority checkpoint;
9. only then edit production Rust.

# Phase 4 — Define the Complete Persistent Shadow Attachment

Replace the scaffold wrapper with one complete typed attachment.

A suitable architecture is:

```rust
pub struct DirectSnowStage3V11ShadowAttachment {
    pub static_context:
        DirectSnowStage3V11StaticContext,

    pub committed:
        DirectSnowStage3V11CommittedState,

    pub restart_posture:
        DirectSnowStage3V11RestartPosture,
}
```

## Static context

Conceptually include:

```text
run and topology identity
Stage 3 model/configuration identity
V11 configuration
LSE configuration
surface-liquid configuration
root-zone hydraulic configuration
soil-layer/OFE maps
BGC configuration
GSI owner configuration
half-hour forcing static configuration
sealed Stage 3 exposure provider
terminal-event tolerance policy
support-admissibility policies
WB14 configuration
receiver topology
```

No static context field may carry:

```text
event day
event lane
event elapsed time
live canopy temperature
live snow temperature
live humidity
live conductance
live snow mass
live liquid
live energy ledger
ending owner payload
```

## Committed state

Conceptually include:

```rust
pub struct DirectSnowStage3V11CommittedState {
    pub stage3_by_lane:
        Vec<DirectSnowStage3PersistentState>,

    pub real_consumer:
        DirectV10RealConsumerShadow,

    pub v11_configuration:
        VegetationConfigurationV11,

    pub v11_parent_state:
        V11ParentTransactionState,

    pub coupled_clock:
        CoupledClockStateV1,

    pub accepted_event_ordinal:
        u64,

    pub receipt_chain:
        Vec<DirectSnowStage3V11ParentReceipt>,
}
```

Use exact current typed structures where available.

Do not serialize owner state with `Debug`.

## Canonical owner bytes

Every owner must use its actual canonical serialization or typed restart
projection:

```text
Stage 3:
    serialize_stage3_persistent_state

V11:
    V11CoupledOwnedState canonical bytes

LSE:
    canonical V2 state/restart projection

surface liquid:
    configuration-bound canonical state bytes

direct hydrology:
    released DirectHydrologyRestartV1 projection

BGC:
    canonical mineral/material state

soil thermal:
    canonical snapshot state

GSI/provider:
    released typed owner/cursor DTOs

coupled time:
    released Restart V2
```

Any owner without canonical bytes must receive a bounded canonical
projection before it can enter the complete owner set.

No `format!("{:?}")`, display string, or synthetic predecessor hash may carry
owner custody.

# Phase 5 — Transfer Stage 3 State Custody Out of Day-Input Construction

Refactor the runner so the day-input builder no longer advances persistent
Stage 3 state.

The builder may produce an immutable prepared forcing object such as:

```rust
PreparedStage3V11DayV1 {
    day_index,
    ordered_lane_forcing,
    half_hour_provider_receipts,
    accepted_gsi_receipt,
    provider_ending_cursor,
    forcing_sha256,
}
```

The exact type may differ.

Required behavior:

```text
runner setup:
    create the default-off attachment from typed static context and explicit
    initial Stage 3/V11 owner state;

day preparation:
    derive forcing and receipts only;

shadow transaction:
    clone committed attachment;
    execute 48 coupled parent intervals;
    validate complete day;
    install one complete ending attachment;

failure:
    discard candidate;
    retain all committed attachment and runner state exactly.
```

The existing runner `RefCell<Vec<Option<DirectSnowStage3PersistentState>>>`
must be:

```text
removed from the selected constitutive path;
or
converted to an immutable/migration seed;
or
retained only under a separately named historical evaluation-only mode.
```

Add a source guard proving that selected constitutive execution cannot advance
both the builder-side Stage 3 state and the attachment Stage 3 state.

# Phase 6 — Execute the Actual Half-Hour Coupled Parent

For every sealed 1,800-second parent interval:

```text
begin from committed complete shadow owners
derive exact atmospheric and precipitation receipts
derive exact current Stage 3 state
derive exact current V11/LSE/hydrology/BGC/soil-thermal state
start one V11 parent transaction
start one coupled-time parent transaction
```

Then select one of three physical branches.

## Branch A — No snow at parent start

Execute exactly one full snow-free V11/LSE/direct-hydrology segment.

Do not run Stage 3 carrier physics.

## Branch B — Snow persists through parent end

Execute exactly one snow-covered V11/Stage-3 shared-carrier chronology over the
complete parent support.

No snow-free ground LSE, terminal liquid transfer, or WB14 terminal receiver
call occurs.

## Branch C — Solid snow exhausts inside parent

Execute:

```text
snow-covered segment
→ terminal event
→ zero-duration custody transition
→ snow-free remainder
```

The V11 parent transaction spans both physical segments and finalizes once.

# Phase 7 — Implement the Actual Snow-Covered Shared Carrier

For every accepted pre-event slab derive inputs from actual owners.

## Reference atmosphere

Use the exact sealed half-hour forcing receipt:

```text
air temperature
specific humidity
pressure
wind/exposure identity
downward longwave
shortwave components
precipitation parcels
support
```

Do not invent atmospheric constants when they are derivable from the accepted
forcing and admitted physical constants.

Any invariant physical constant must come from the canonical contract or
model definition and be included in model identity.

## V11 canopy surface

Derive from the current staged V11/LSE state and configuration:

```text
leaf, stem and wet-canopy temperatures
leaf/stem emissive areas
canopy sensible conductance
canopy vapor conductance
canopy liquid
active occupancy topology
physiology and root-hydraulic state
```

No configured receipt may supply live canopy temperature, humidity, or
conductance.

## Stage 3 snow surface

Derive from current persistent Stage 3 state and snow configuration:

```text
surface temperature
surface humidity
snow roughness/emissivity/albedo
ice mass
retained liquid
cold content
layer structure
```

No configured receipt may supply live snow temperature, humidity, or
conductance.

## Shared carrier execution

Use the released Child 2C equations and receipts.

The active participant set must include the actual coupled participants, not
only `stage3-snow`.

At minimum audit:

```text
V11 canopy
Stage 3 snow
shared carrier
any support-limiting soil/plant owner
```

The shared-air solution must produce independently reconstructed:

```text
reference sensible flux
canopy sensible flux
snow sensible flux
reference vapor flux
canopy vapor flux
snow vapor flux
canopy–snow reciprocal longwave
snow net longwave
```

Require exact participant/support receipt joins.

## Pre-event V11 execution

Execute the actual V11 constitutive segment from the parent beginning state.

Its ending state becomes the beginning state of any later segment.

Do not freeze vegetation.

Do not calculate the post-event segment from the parent beginning state.

# Phase 8 — Execute the Actual Stage 3 Terminal Solver

Use actual Stage 3 state and actual accepted carrier fluxes.

Do not construct `TerminalStateRates`.

Implement or extract a typed support evaluator using the existing Stage 3
transition and `solve_terminal_enthalpy_event` implementation.

Required output must include the real:

```text
DirectSnowTerminalEventResult
```

with:

```text
event occurrence
evaluated support
unevaluated support
terminal liquid
melt
refreeze
sublimation
deposition
external liquid
cold-content change
complete energy
unallocated energy
mass residuals
energy residual
event bracket
LTE evidence
accepted/rejected trial counts
```

## Coupled-time event projection

Convert the terminal solver's proposed event time to the coupled-time tick
using the released quantization rule.

Construct the actual candidate set from:

```text
terminal solver bracket lower tick
terminal solver bracket upper tick
proposed tick
parent start/end
support-admissible neighboring ticks required by Child 2C
```

Do not use one configured candidate.

Evaluate each candidate by rerunning the actual Stage 3 terminal transition to
that support. Do not linearly extrapolate snow/liquid/energy state.

Apply released deterministic event-boundary coalescing.

When no candidate passes:

```text
ERR-CT-021
→ reject the complete parent attempt
→ preserve every committed owner
→ retry only according to admitted controller/event policy
```

No dropped remainder and no state freezing.

# Phase 9 — Execute the Zero-Duration Terminal Transition

At the accepted event tick:

```text
close Stage 3 solid-snow state
close snow mass, liquid, vapor and energy ledgers
advance the event ordinal once
advance no modeled time
integrate no rate
mark terminal transfer unconsumed in candidate state
```

Construct exactly one typed terminal parcel from the actual event result.

Use the canonical terminal-liquid identity:

```text
terminal liquid =
    accepted retained liquid
    + accepted liquid precipitation on snow support
    + accepted melt
    - accepted refreeze
    - any separately admitted snow-side liquid export
```

When `DirectSnowTerminalEventResult.terminal_liquid_kg_m2` already represents
that complete identity, use it directly and independently reconstruct it from
its component operands.

Do not add the same rain or melt again from `DirectDayFrame`.

Required parcel identity:

```text
source owner
source Stage 3 beginning and ending state digests
parent transaction
event receipt
event ordinal
source lane/OFE
destination OFE/tile/surface
mass basis
temperature = 273.15 K where the contract requires it
specific sensible enthalpy = 0 relative to T_ref
remaining half-open support
consumed marker
parcel digest
```

The terminal parcel may be consumed once.

Restart after the event must retain the consumed/unconsumed posture exactly.

# Phase 10 — Execute the Actual Snow-Free Remainder

When the accepted event leaves zero support:

```text
skip all snow-free physical execution
```

When nonzero:

```text
remainder >= maximum minimum support of all active post-event owners
```

Then:

1. credit the terminal parcel to the actual persistent surface-liquid owner;
2. partition post-event atmospheric precipitation by exact support;
3. execute the actual partial WB14 continuation under INV-011;
4. retain/infiltrate/overflow/route terminal and post-event liquid exactly once;
5. construct the actual post-event LSE forcing without snow operands;
6. execute `DirectV11RealConsumerStack` over the accepted remainder;
7. begin the post-event V11 segment from the staged pre-event V11 ending;
8. stage actual V11, LSE-V2, direct hydrology, surface liquid, BGC, and
   soil-thermal endings;
9. validate all cross-owner receipts and ledgers.

Do not treat terminal liquid as same-snapshot beginning-store supply for
evaporation.

Do not reuse:

```text
snow albedo
snow temperature
snow roughness
snow sensible flux
snow latent flux
snow longwave
terminal unallocated snow energy
```

after the event.

# Phase 11 — Complete Parent Finalization and Installation

Construct one parent candidate:

```rust
pub struct DirectSnowStage3V11ParentCandidate {
    pub ending_state:
        DirectSnowStage3V11CommittedState,

    pub parent_receipt:
        DirectSnowStage3V11ParentReceipt,

    pub publication_reduction:
        DirectSnowStage3V11PublicationReduction,
}
```

The parent receipt must bind:

```text
parent support
all accepted slab receipts
pre-event V11 receipt
Stage 3 beginning/ending state
shared-carrier receipts
terminal event proposal and accepted event
terminal parcel
post-event V11 receipt when present
all seven typed owner endings
GSI/provider cursor
coupled-time ending cursor
complete water, energy, carbon, nitrogen and material ledgers
```

Required chronology:

```text
all fallible computation
→ all canonical owner validation
→ all cross-owner joins
→ all ledger closure
→ complete restart projection
→ publication reduction construction
→ one non-fallible attachment replacement
```

No live owner may be assigned sequentially.

The production `DirectRunFrame` remains byte-identical with and without the
default-off shadow.

# Phase 12 — Ordinary Runner Integration

The runner must construct the attachment from actual typed run ownership.

It may do so only under an explicit existing or newly typed default-off
configuration posture.

Do not change the default.

No ordinary closure API may accept:

```text
event tick
event day/lane
live canopy receipt
live snow receipt
carrier conductance
carrier ledger
terminal rates
ending owner payloads
caller-built handoff request
caller-built owner executor
```

The runner may provide only:

```text
typed static configuration
initial owner states
sealed forcing/GSI capabilities
explicit default-off selection
```

Prefer running the isolated shadow's 48 coupled parent intervals from a
whole-day prepared forcing capability.

The physical shadow may be invoked from a day-level scheduler hook, but it must
advance its own complete owner state through 48 ordered half-hour parents. It
must not use the completed legacy production day frame as its beginning
physical owner state.

The existing immutable publication-batch API should remain.

External publication occurs only after the complete model transaction returns
a prepared immutable batch.

# Phase 13 — Complete Additive Restart

Define an additive outer restart identity, for example:

```text
OPENWEPP_STAGE3_V11_COUPLED_RESTART_V1
```

Do not modify existing released nested wire bytes.

Persist:

```text
complete Stage 3 persistent state by lane
V11 Restart V3 state
complete DirectV10 real-consumer owner state
LSE-V2 state
surface-liquid state and partial WB14 continuation
direct-hydrology state
BGC state
soil-thermal state
GSI state
provider cursor
coupled-time Restart V2 state
active parent/segment/slab position
accepted event ordinal
proposed and accepted event receipts
terminal-parcel consumed marker
accepted owner receipts
publication reduction/outbox state
parent transaction identity
```

Do not persist:

```text
rejected nonlinear attempts
unaccepted event candidates as owner state
Debug payloads
duplicated V10/V9 or LSE-V1 compatibility state
```

Required restore points:

```text
between days
before parent interval
after accepted snow-covered slab
immediately before terminal event
immediately after terminal event
after terminal parcel construction
after terminal parcel credit
inside snow-free remainder
immediately before parent finalization
after parent commit
```

For every restore:

```text
drop all original runtime objects
restore fresh
continue
compare with uninterrupted execution
```

Require exact no-replay of:

```text
V11 segment
Stage 3 event
terminal parcel
WB14 continuation
GSI daily advance
provider cursor advance
parent transaction increment
publication batch
```

# Phase 14 — Positive Physical Scenario Matrix

Plumbing-only fixtures are insufficient.

Use actual Stage 3 and V11 constitutive execution.

Required positive cases:

```text
1. no snow at parent start → full snow-free V11 parent;
2. snow persists through parent end;
3. exact terminal event at parent start;
4. exact terminal event at parent end;
5. interior event with admissible pre/post support;
6. event coalesced toward parent start;
7. event coalesced toward parent end;
8. no admissible candidate → ERR-CT-021 retry;
9. sublimation without meltout;
10. deposition;
11. rain during snow-covered support;
12. refreeze before terminal melt;
13. retained liquid plus newly generated melt;
14. terminal liquid retained at litter;
15. terminal liquid infiltrated;
16. ponding and overflow;
17. routed runon/runoff;
18. multiple tiles in one OFE;
19. multiple OFEs/lanes;
20. different canopy states across covered tiles;
21. cross-midnight terminal event;
22. snow disappearance followed by later snowfall and reappearance;
23. restart before event;
24. restart at event;
25. restart after event;
26. consecutive segmented parent intervals.
```

At least two cases must use existing repository-backed climate/soil/management
fixtures rather than entirely synthetic data:

```text
one open-snow case
one covered-forest case
```

Reuse existing admitted Stage 3 and V11 fixture custody.

Do not introduce new calibrated parameter values.

# Phase 15 — Poison and Failure Matrix

Inject failure after each of these boundaries:

```text
static-context validation
forcing/GSI receipt validation
Stage 3 beginning-state validation
pre-event V11 execution
shared-carrier solve
Stage 3 terminal trial
event candidate selection
terminal ledger validation
terminal parcel construction
surface-liquid credit
partial WB14 continuation
post-event LSE support admission
post-event V11 execution
BGC receipt construction
soil-thermal ending construction
complete owner join
restart projection
parent receipt construction
publication reduction construction
immediately before installation
```

For every failure:

```text
capture canonical production bytes
capture canonical complete shadow bytes
capture coupled clock
capture provider/GSI cursor
capture publication state

attempt operation

require exact typed failure category

recapture all bytes

require exact equality
```

Also poison:

```text
caller-configured event time
caller-configured carrier temperature
caller-configured conductance
wind-as-conductance
zeroed ledger substitution
Debug owner payload
wrong Stage 3 lane
wrong V11 parent
wrong participant set
wrong support receipt
wrong event ordinal
wrong terminal mass
rain counted twice
melt counted twice
refreeze omitted
wrong parcel destination
wrong tile basis
wrong enthalpy
WB14 reset
WB14 replay
post-event snow operand
post-event segment from parent beginning state
duplicate event replay
duplicate parcel replay
reordered owner set
missing BGC owner
missing soil-thermal owner
restart cursor rewind
restart cursor skip
publication before complete acceptance
```

# Phase 16 — Independent Conservation and Compatibility Evidence

Independently reconstruct from accepted runtime operands:

## Snow support

```text
beginning ice
+ snowfall
+ deposition
+ refreeze
- sublimation
- melt
= ending ice
```

## Snow liquid

```text
beginning retained liquid
+ liquid precipitation
+ melt
- refreeze
- terminal parcel
- admitted snow-side export
= ending retained liquid
```

## Snow energy

```text
shortwave
+ longwave
+ sensible
+ latent
+ advected
= cold-content change
+ melt latent energy
- refreeze latent energy
+ terminal unallocated energy
```

## Surface/soil water

```text
terminal parcel
+ post-event precipitation/runon
=
surface storage change
+ infiltration
+ runoff/overflow
+ evaporation
```

## Complete parent

Reconstruct:

```text
water
energy
carbon
NH4
NO3
dry material
modeled support
```

from consumer-visible operands.

Do not accept a producer's own residual as independent evidence.

## Production noninterference

For the same run beginning state:

```text
default attachment absent
versus
default-off attachment present
```

require exact production equality for:

```text
production DirectRunFrame
CoE snow/melt state
published production rows
WAT/HBP/PASS bytes
scheduler counters
selectors
defaults
```

The shadow may differ only in its own explicit attachment and diagnostic batch.

# Phase 17 — Assurance and Exact-Head Closure

After final code and any approved contract changes:

1. rerun the typed assurance identity workflow;
2. verify the complete identity generation chain;
3. ensure no hand-edited generated lock;
4. rerun all stale-contract assertion tests;
5. rerun the frost profile;
6. run the exact-head workspace gates.

Child 1 cannot close as COMPLETE while the assurance identity lock or current
contract assertions remain stale.

# Phase 18 — Reviews

Require fresh read-only reviews on one exact commit.

## Review A — snow/vegetation/LSE science

Must explicitly verify:

```text
actual Stage 3 solver invocation
actual V11 pre-event evolution
actual shared-air carrier
no configured live physics
event coalescing over actual terminal candidates
no snow flux after event
no snow-free flux before event
terminal-liquid and energy lineage
```

## Review B — hydrology/ownership/restart

Must explicitly verify:

```text
Stage 3 state custody moved out of precommit day-input mutation
surface-liquid receiver topology
partial WB14 continuation
multi-lane/tile ownership
complete canonical owner state
restart before/at/after event
no replay
atomic installation
```

## Review C — Rust correctness/API

Must explicitly verify:

```text
ordinary API cannot accept physical DTOs
no Debug owner serialization
no duplicate owner state
no partial live mutation
failure precedence
bounded visibility
existing wire protection
```

## Review D — Rust QA

Must independently rerun and audit:

```text
focused test selection
affected-crate selection
line counts
Clippy
formatting
cargo-deny
source guards
assurance gates
```

Then run:

```text
comparator_suite_runner
terminal verifier A
terminal verifier B
```

All accepted material findings must be fixed and all invalidated evidence
rerun.

# Phase 19 — Validation Commands

Discover exact current target names before execution.

At minimum run:

```bash
cargo check \
  -p openwepp-hillslope-orchestrator \
  -p openwepp-vegetation \
  -p openwepp-land-surface-energy \
  -p openwepp-biogeochemistry \
  -p openwepp-persisted-restart-v1 \
  -p openwepp-runner

cargo nextest run \
  --test snow_stage3_shared_carrier_authority_contract \
  --no-fail-fast

cargo nextest run \
  --test snow_stage3_shared_carrier_terminal_handoff_implementation \
  --no-fail-fast

cargo nextest run \
  --test snow_stage3_terminal_handoff_constitutive_closure_contract \
  --no-fail-fast

cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract \
  --no-fail-fast

cargo nextest run \
  --test c3_vegetation_implementation_contract \
  --no-fail-fast

cargo nextest run \
  --test vegetation_real_hydrology_shadow_contract \
  --no-fail-fast

cargo nextest run \
  --test land_surface_energy_real_hydrology_shadow_contract \
  --no-fail-fast

cargo nextest run \
  --test surface_liquid_hydrology_custody_authority_contract \
  --no-fail-fast

cargo nextest run \
  --test snow_free_half_hour_forcing_adapter_contract \
  --no-fail-fast

cargo nextest run \
  --test direct_hydrology_restart_authority_contract \
  --no-fail-fast

cargo nextest run \
  --test root_zone_hydraulic_authority_contract \
  --no-fail-fast

cargo clippy \
  -p openwepp-hillslope-orchestrator \
  -p openwepp-vegetation \
  -p openwepp-land-surface-energy \
  -p openwepp-biogeochemistry \
  -p openwepp-persisted-restart-v1 \
  -p openwepp-runner \
  --all-targets -- -D warnings

bash tools/release/check_authority_suite_antievasion.sh

cargo nextest run \
  --test auth11_required_suite_obligation_guards_contract \
  --profile quick --no-fail-fast

cargo fmt --all -- --check
cargo deny check
git diff --check
```

Use the Nix environment when required:

```bash
nix develop --command cargo ...
```

Do not claim a command ran unless its actual output is retained.

## Heavy and exact-head gates

Use `comparator_suite_runner` for broad runs.

Run:

```bash
cargo clippy --workspace --all-targets -- -D warnings

TMPDIR=/tmp/ow-stage3-constitutive \
  cargo nextest run --workspace --profile full

cargo test --doc --workspace
cargo deny check
cargo fmt --all -- --check
git diff --check
```

Also run the current frost/assurance profiles and exact generated-identity
verification discovered during intake.

Record exact:

```text
selected
passed
failed
skipped
runtime
commit
toolchain
environment
```

# Phase 20 — Line-Count Governance

Record every changed Rust file before and after.

Rules:

```text
< 2000 lines:
    preferred

>= 2000 lines:
    WARN with decomposition rationale

>= 3000 lines:
    closure blocker unless explicitly exempted by repository policy
```

Do not add this implementation back into a near-ceiling file.

Prefer bounded modules such as:

```text
direct_runtime/snow_stage3_v11/
  mod.rs
  static_context.rs
  committed_state.rs
  forcing.rs
  snow_covered.rs
  terminal_event.rs
  terminal_liquid.rs
  snow_free_remainder.rs
  parent_candidate.rs
  restart.rs
  publication.rs
  tests.rs
```

Exact naming may differ.

# Phase 21 — Terminal Disposition

Close only when:

```text
actual Stage 3 persistent state is the snow owner;
actual V11 state evolves over snow-covered support;
actual shared carrier is executed;
actual terminal enthalpy event is localized;
actual event-boundary coalescing is used;
terminal liquid is transferred once;
actual V11/LSE/hydrology remainder is executed;
all seven typed owner states are canonical;
one complete attachment is installed;
complete restart passes;
repository-backed physical scenarios pass;
assurance identity and stale assertions are clean;
full exact-head workspace gates pass;
all four reviews pass;
both terminal verifiers pass.
```

Terminal status:

```text
COMPLETE /
actual Stage-3/V11 terminal-handoff constitutive path complete /
default-off ordinary-runner shadow only /
Child 3 authorized but not started
```

Retain:

```text
production Stage 3 activation = NOT AUTHORIZED
CoE retirement = NOT AUTHORIZED
production selector = UNCHANGED
production defaults = UNCHANGED
production outputs = UNCHANGED
seasonal efficacy = NOT CLAIMED
calibration = NOT CLAIMED
transferability = NOT CLAIMED
canopy-intercepted snow = NOT IMPLEMENTED
```

Archive the kickoff prompt byte-identically only after both terminal verifiers
PASS.

Update:

```text
ROADMAP:
    Child 1 complete; Child 3 next

campaign coordinator:
    resumed Child 1 complete

work-package catalog:
    record package outcome and exact terminal commit
```

Commit locally.

Suggested message:

```text
Complete Stage-3 V11 terminal constitutive handoff
```

Do not push.

# Suggested Local Commit Boundaries

```text
1. Scaffold Stage-3 terminal constitutive closure
2. Reconcile snow assurance identities and stale guards
3. Freeze Stage-3/V11 cadence and owner authority
4. Move persistent Stage-3 custody into the attachment
5. Implement actual snow-covered V11 shared carrier
6. Integrate actual terminal event and liquid transition
7. Execute typed snow-free remainder and atomic parent commit
8. Add complete restart and physical scenarios
9. Close Stage-3 terminal constitutive handoff
```

Before every commit:

```bash
git status --short
git diff --check
cargo fmt --all -- --check
```

# No Short-Cycling

Do not stop after:

```text
moving the persistent Stage 3 state;
calling the terminal solver once;
removing Debug serialization;
running one V11 segment;
passing one interior event;
adding restart fields;
fixing the assurance lock;
passing package-focused tests.
```

Continue through the complete constitutive path, full restart, positive
scenarios, assurance reconciliation, exact-head gates, reviews, and dual
terminal verification.

# Legitimate Future HOLD

A HOLD is legitimate only for a newly demonstrated exact contradiction such
as:

```text
the current Stage 3 authority does not admit any 1,800-second support
projection without changing its constitutive meaning;

the shared carrier lacks an authoritative live V11 or Stage 3 operand after
complete source tracing;

the terminal liquid cannot be mapped to the declared multi-tile receiver
topology without a new ownership rule;

the existing V11 resource transaction cannot represent snow-covered owner
transitions without changing an immutable model definition;

the released restart authorities cannot be composed additively without
changing protected nested wire bytes;

the typed assurance workflow refuses current-source adoption because the
report lifecycle forbids it.
```

Before declaring HOLD:

```text
identify the exact contract clause or missing field;
identify the exact source and recipient;
show the failing real fixture;
show every safe implementation route attempted;
show why each route violates existing authority;
name the first narrow contract-first lift action.
```

The following are not HOLD reasons:

```text
the current day builder must be refactored;
the Stage 3 evaluator must be extracted to a parent-support API;
the attachment must own more state;
the V11 snow-covered executor is substantial;
tests fail;
Clippy fails;
the assurance tool requires typed receipts;
the full workspace is expensive;
line-count refactoring is required;
reviewers find in-scope implementation defects.
```

Autonomy:

Execute the package end-to-end without asking for direction unless one exact
authority, ownership, restart-wire, or assurance-lifecycle contradiction
remains after all safe in-scope routes are exhausted.
```

The key architectural instruction is that the existing day-level scheduler hook may remain a convenient **invocation point**, but it cannot use the completed production `DirectDayFrame` as physical shadow state. The attachment must advance its own Stage 3 and V11 owners through the ordered 48 half-hour parents using sealed forcing and actual canonical owner states.
