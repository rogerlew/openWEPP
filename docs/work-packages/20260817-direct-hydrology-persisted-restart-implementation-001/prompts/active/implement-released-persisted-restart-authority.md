# Implement Released Persisted Restart Authority, Close the Forcing Adapter,
# Complete Child 4, and Close the Snow-Free Integration Campaign

Repository:

Use the existing openWEPP checkout. Expected location:

`/home/workdir/openWEPP`

If the checkout is mounted at `/workdir/openWEPP`, use that existing checkout
instead. Do not reclone.

Required starting commit:

`dea2d60d4d8a618b9a775f28e47a477c5ee85fe9`

Branch:

`main`

Required remote state:

`origin/main` must point to the same starting commit.

Execution mode:

`persisted restart implementation →
forcing-adapter terminal closure →
Child-4 terminal execution →
campaign terminal closure`

Execute the entire sequence autonomously through truthful terminal
dispositions. Do not stop after a local implementation milestone, after the
first successful restart test, or after closing only the restart package.

## Authorization Boundary

Authorized:

- local repository reads and edits;
- local commands and tests;
- production Rust implementing the released default-off restart capability;
- package, contract-derived test, benchmark, evidence, review, and verification
  updates;
- bounded refactoring required to expose sealed internal restart surfaces;
- intentional local commits at coherent boundaries;
- read-only independent reviewers and terminal verifiers;
- exact-current lifecycle reconciliation through campaign closure.

Not authorized:

- production selector activation;
- default changes;
- production hydrology replacement;
- production ET replacement;
- production output changes;
- WEPP runtime cutover;
- deployment;
- calibration;
- empirical-validation or transferability claims;
- canopy snow;
- terminal snow handoff;
- soil biogeochemical transformations;
- public publication;
- external messages;
- PR creation;
- remote branch creation;
- push.

Do not push unless separately directed by the user.

# Governing Released Authority

The implementation target is immutable:

```text
OPENWEPP_DIRECT_HYDROLOGY_RESTART_V1
OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1
````

Authority exact-evidence commit:

`684477022b1a801a405c0ddd23c6166673339e75`

Authority lifecycle-release commit:

`f95395597fe434dbc853c1b947b16fd434f013c1`

Do not edit, regenerate, reinterpret, or silently broaden the released
authority package.

Released authority package:

```text
docs/work-packages/
  20260817-direct-hydrology-persisted-restart-authority-001/
```

Released reference implementation:

```text
docs/work-packages/
  20260817-direct-hydrology-persisted-restart-authority-001/
  tools/restart-authority-reference/
```

Released manifest entries and required SHA-256 values:

```text
checkpoint-vector.json
ce129956f72c3b421c75524a5be841b2e6c3bc3851c864d6672ec7d22aec53f5

checkpoint-in-progress-vector.json
da3a7bcc6185269ae88dc93147b6e3ca53537676575a2a915e1168652717a2ad

checkpoint-cross-midnight-vector.json
b6a59297de0528acf5b1c00bd79ba0932a0c454eb1e17c9ba6df8e1096aa6838

checkpoint-multi-destination-vector.json
acbd4cb419db57fe21c46c36ded4d95fc5b0f12bdf42e999d882c4fef2a7ce5e

checkpoint-schema.json
c041ab5923d696e6f26ea8e646a7520fadc30df86191009f08b556b470d80ddd

generated-field-metadata.json
0cb8269e6eab705b3bd13329487428d7a59073004f011249b578f0dff94e5946

direct-run-frame-field-classification.md
01ddd6c1341fc57f8c65a12445d55e885f634101c1c391838c34873687fabd7b

poison-matrix.md
dbaea93ee55be51410c616d16e9c547d43dcbb613b64f03ca0f31041abf9ef71
```

Implementation must match these bytes and semantics. A difficult implementation
is not authority to change them.

# Existing Packages

Persisted restart implementation:

```text
docs/work-packages/
  20260817-direct-hydrology-persisted-restart-implementation-001/
```

Forcing adapter:

```text
docs/work-packages/
  20260817-snow-free-half-hour-forcing-adapter-implementation-001/
```

Child 4:

```text
docs/work-packages/
  20260814-vegetation-land-surface-real-consumer-shadow-001/
```

Campaign coordinator:

```text
docs/work-packages/
  20260814-snow-free-land-surface-real-hydrology-integration-001/
```

Do not create replacement packages.

# Phase 0 — Exact Intake

Before editing:

```bash
cd /home/workdir/openWEPP 2>/dev/null || cd /workdir/openWEPP

test "$(git rev-parse HEAD)" = \
  "dea2d60d4d8a618b9a775f28e47a477c5ee85fe9"

test "$(git rev-parse origin/main)" = \
  "dea2d60d4d8a618b9a775f28e47a477c5ee85fe9"

git status --short --branch
git log --oneline --decorate -12
git diff --check
```

Require:

```text
branch = main
worktree = clean
HEAD = required starting commit
origin/main = required starting commit
```

Do not pull, reset, rebase, merge, or replace the starting tree.

Run instruction discovery:

```bash
tools/agents/find-agents --for \
  Cargo.toml \
  crates/openwepp-hillslope-orchestrator \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime \
  crates/openwepp-hillslope-orchestrator/src/runtime_inputs/09_snow_free_half_hour_forcing.rs \
  crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs \
  crates/openwepp-vegetation/src/v10_state.rs \
  crates/openwepp-land-surface-energy/src/state.rs \
  crates/openwepp-land-surface-energy/src/v2_state.rs \
  crates/openwepp-biogeochemistry \
  crates/openwepp-plant-phenology \
  tests/integration \
  docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001 \
  docs/work-packages/20260817-direct-hydrology-persisted-restart-implementation-001 \
  docs/work-packages/20260817-snow-free-half-hour-forcing-adapter-implementation-001 \
  docs/work-packages/20260814-vegetation-land-surface-real-consumer-shadow-001 \
  docs/work-packages/20260814-snow-free-land-surface-real-hydrology-integration-001 \
  docs/ROADMAP.md \
  docs/work-packages/README.md
```

Read every discovered instruction file.

Read at minimum:

```text
root AGENTS.md
crates/AGENTS.md
tests/AGENTS.md
docs/work-packages/AGENTS.md
docs/codex_exec_plans.md

restart authority package.md
restart authority final-disposition.md
restart authority gate-results.md
restart authority artifact-manifest.json
restart authority complete schema
restart authority generated field metadata
restart authority field classification
restart authority poison matrix
all three authority reviews
both authority terminal verifiers
reference crate canonical.rs
reference crate checkpoint.rs
reference crate continuation_template.rs
reference crate evidence_fixture.rs
reference crate all owner-mapping modules

restart implementation package.md
forcing-adapter package.md and current evidence
Child-4 package.md and current HOLD evidence
campaign package.md and current HOLD evidence
ROADMAP.md
work-package catalog
```

Freeze:

```text
starting commit
authority evidence commit
authority lifecycle commit
authority manifest bytes
authority schema bytes
authority vector bytes
authority reference crate tree SHA
Rust toolchain
Cargo.lock
production selector/default exclusion baseline
production output/publication exclusion baseline
documentation-link baseline
current line counts
current focused and full-workspace baseline
```

Create implementation intake artifacts:

```text
docs/work-packages/
  20260817-direct-hydrology-persisted-restart-implementation-001/
  artifacts/

    start-commit-and-authority-binding.md
    required-reading-map.md
    released-authority-byte-inventory.md
    implementation-write-set.md
    module-and-dependency-design.md
    production-exclusion-baseline.md
    test-and-review-plan.md
    gate-results.md
    final-disposition.md
    worker-handoff.md
```

Copy this kickoff prompt byte-identically into:

```text
docs/work-packages/
  20260817-direct-hydrology-persisted-restart-implementation-001/
  prompts/active/
```

Record its SHA-256. Do not archive it until terminal implementation PASS.

# Phase 1 — Activate the Implementation Package

Before production Rust changes, make a governance-only increment.

Update the implementation package status to:

```text
executing /
released restart authority bound /
production implementation active
```

Record:

```text
authority evidence commit = 684477022b1a801a405c0ddd23c6166673339e75
authority release commit  = f95395597fe434dbc853c1b947b16fd434f013c1
implementation base       = dea2d60d4d8a618b9a775f28e47a477c5ee85fe9
```

Update the roadmap’s active campaign gate only as needed to say:

```text
persisted-restart implementation executing
```

Do not prematurely change forcing, Child-4, or campaign HOLD statuses.

Run:

```bash
python tools/docs/markdown_lint.py \
  docs/work-packages/20260817-direct-hydrology-persisted-restart-implementation-001 \
  docs/ROADMAP.md

git diff --check
```

Commit locally:

```text
Activate persisted restart implementation
```

Do not push.

# Phase 2 — Freeze Production Module Architecture

The released reference crate is authority evidence and must remain outside the
production dependency graph.

Do not make production runtime depend on files under `docs/work-packages/`.

Preferred implementation location:

```text
crates/openwepp-hillslope-orchestrator/src/persisted_restart_v1/
```

or an equivalently bounded orchestrator-owned module hierarchy.

Prefer the orchestrator because the checkpoint composes:

```text
DirectRunFrame
DirectV10RealConsumerShadow
GSI owner
forcing cursor and receipts
V10 state
LSE-V2 state
soil thermal
BGC
scheduler position
```

A new workspace crate is acceptable only if a written dependency audit proves
that it introduces no cycle and does not duplicate owner authority.

Suggested modules:

```text
persisted_restart_v1/
  mod.rs
  primitives.rs
  canonical.rs
  error.rs
  hydrology.rs
  surface_liquid.rs
  gsi.rs
  forcing.rs
  vegetation_v10.rs
  lse_v2.rs
  soil_thermal.rs
  biogeochemistry.rs
  checkpoint.rs
  admission.rs
  prepared_day.rs
  static_context.rs
  tests.rs
```

The exact split may differ.

Record in `module-and-dependency-design.md`:

```text
module ownership
public versus pub(crate) visibility
dependency direction
wire-authority parity method
static-context boundary
atomic installation boundary
failure precedence
test-only/evidence seams
line-count projections
```

No `.rs` file may reach 3000 lines.

At 2000 lines, split or record a specific WARN rationale.

# Phase 3 — Promote the Released Wire and Canonical Codec

Implement production-owned equivalents of the released authority types.

Do not redesign them.

Required primitive semantics include:

```text
HexF64
HexU128
Sha256Hex
fixed-width day, interval, lane, count and index wrappers
explicit wire enums
strict tagged phase union
canonical bare JSON with no trailing newline
duplicate-field rejection
unknown-field rejection
exact member order
exact lowercase hex
signed-zero preservation
strict canonical reserialization equality
```

Required public or sealed APIs conceptually include:

```rust
pub fn to_canonical_checkpoint_bytes(
    checkpoint: &DirectV10RealConsumerCheckpointV1,
) -> Result<Vec<u8>, RestartError>;

pub fn admit_checkpoint_v1(
    bytes: &[u8],
    context: &ExpectedRestartStaticContext<'_>,
) -> Result<IsolatedRestoredCheckpointV1, RestartAdmissionFailureV1>;
```

The strict admission order must follow released authority.

Freeze exact error precedence. At minimum distinguish:

```text
schema
unsupported_version
noncanonical_bytes
payload_digest
missing_field
extra_field
reordered_field
duplicate_field
run_identity
topology_identity
configuration_identity
owner_identity
transaction_lineage
scheduler_position
provider_cursor
gsi_receipt
heterogeneous_lane_gsi_receipt
forcing_receipt_cardinality
forcing_receipt_order
forcing_receipt_digest
v10_v9_projection
lse_v2_v1_projection
owner_validation
unsupported_laned_active
canonical_order
owner_omission
child4_retained_liquid
groundwater_posture
groundwater_total_area
erosion_publication
surface_liquid_configuration
```

Do not collapse typed authority failures into generic serialization errors.

Add a multi-poison precedence test.

## Authority parity

Add tests proving production code:

```text
parses every released vector
reserializes every released vector byte-identically
regenerates every released outer digest
accepts the complete released schema
matches all released fixed-width and signed-zero semantics
```

Do not update the released vectors to fit implementation.

# Phase 4 — Implement DirectHydrologyRestartV1

Promote the released exhaustive field projection.

Implement production equivalents conceptually like:

```rust
impl DirectHydrologyRestartV1 {
    pub fn project(
        frame: &DirectRunFrame,
        context: &DirectHydrologyProjectionContext<'_>,
    ) -> Result<Self, DirectRestartError>;

    pub fn restore_isolated(
        &self,
        context: &ExpectedDirectHydrologyRestartContext<'_>,
    ) -> Result<DirectRunFrame, DirectRestartError>;
}
```

## Persisted direct-hydrology state

Include every released continuation field for:

```text
DirectRunFrame
DirectLaneFrame
DirectWaterState
DirectTransferBuffers
DirectLaneTransferLedger
DirectRunTransferDownstreamOperands posture
DirectSubsurfaceLayerState
DirectEvapotranspirationStageState
DirectGrowthStateSurface
DirectWinterColumnState
DirectErosionDownstreamOperands
DirectErosionInflowIntake
DirectErosionRuntimeCarry
DirectGroundwaterRunState
DirectSurfaceLiquidOwnedState
```

Use exhaustive source destructuring without `..`.

A newly added runtime field must fail compilation until classified.

## Reconstructed state

Reconstruct exactly according to released authority:

```text
phase plan
day inputs
publication scratch
snow compatibility carry
frost compatibility carry
lane-transfer shadow projection
other explicitly released caches
```

Each reconstruction requires:

```text
persisted source operands
deterministic operation
exact comparison rule
typed mismatch error
```

## Unsupported state

Reject:

```text
laned_active
laned_active_summary
active winter state in the Child-4 snow-free checkpoint
retained snow liquid
unbound or mismatched phase plan
unreconstructable day inputs
```

Do not silently drop unsupported state.

## Surface-liquid restoration

Surface-liquid state may not restore without its exact configuration.

Restoration must:

```text
validate configuration identity
restore state
call existing configuration-bound owner validation
recompute canonical state digest
require exact digest equality
verify OFE/tile/surface/layer bindings
verify accepted transaction lineage
attach state only to the isolated candidate frame
```

Persist surface-liquid state exactly once inside direct hydrology.

## Exact candidate construction

Do not mutate a supplied `DirectRunFrame`.

Always construct a new isolated frame.

Required evidence:

```text
runtime → restart DTO → runtime
```

is bit-identical for every persisted field, and every reconstructed field
satisfies its released comparison rule.

# Phase 5 — Implement Typed GSI and Forcing Restart Owners

Use the actual repository owner types.

## GSI

Implement production checkpoint projection/restoration for:

```text
DirectGsiOwnerConfigurationV1
DirectGsiOwnerStateV1
DirectGsiDailyReceiptV1
DirectGsiDateV1
DirectGsiParametersV1
DirectGsiForcingV1
DirectGsiResultV1
```

Wire binary64 values through `HexF64`.

Restoration must invoke the existing CP-GSI01 replay:

```text
beginning state
+ exact parameters
+ exact daily forcing
→ expected result
→ expected ending state
```

Require exact match to persisted receipt and ending state.

Do not trust only the outer checkpoint digest.

## Provider static configuration and cursor

Persist and restore:

```text
run ID
CO2
reference height
GSI owner configuration digest
ordered destination configuration
WB14 digests
next day index
static configuration digest
pending precipitation carry
cursor digest
```

Restore cursor against the actual static configuration and scheduler day.

## Forcing day receipts

Persist the complete released shape:

```text
one day receipt per destination
destinations ordered by (ofe_id, tile_id)
exactly 48 intervals per destination
every atmospheric operand
every GSI binding
every WB14 binding
every parcel
every parcel support
every parcel temperature and enthalpy
every interval digest
every day digest
next-day carry
```

Restoration must construct the actual runtime receipt objects and invoke their
existing validation.

Do not persist only receipt digests.

# Phase 6 — Implement Typed Scientific Owners

## V10 vegetation

Persist the complete released V10 configuration/state identity and physical
payload.

After restoration:

```text
construct V10CoupledOwnedState
validate against expected VegetationConfiguration
recompute V10 state digest
project V10 → V9
verify exact non-identity payload preservation
```

Persist no independent V9 state.

## LSE-V2

Persist:

```text
model definition
configuration
owner
last transaction
ordered tile states
surface enthalpy
surface temperature warm starts
state digest
```

After restoration:

```text
validate LSE-V2
recompute digest
project V2 → V1
verify exact non-identity payload preservation
```

Persist no independent LSE-V1 state.

## Soil thermal

Persist:

```text
owner identity
configuration/state/snapshot identities
last transaction
ordered OFEs
ordered layers
layer identity
temperature
enthalpy
```

Invoke existing `SoilThermalSnapshot::validate()` after restoration.

## Biogeochemistry

Persist:

```text
ordered mineral layers
NH4
NO3
ordered material receivers
carbon
nitrogen
dry matter
last transaction
```

Invoke existing inventory and material-domain validation.

Do not collapse NH4 and NO3.

# Phase 7 — Implement the Complete Checkpoint Phase Union

Implement exactly the released two-phase architecture.

Conceptually:

```rust
pub enum DirectV10CheckpointPhaseV1 {
    BetweenDays {
        next_day_index: WireDayIndex,
        accepted_interval_count: AcceptedIntervalCount,
        committed: CompleteCommittedOwnerStateV1,
    },

    InProgressDay {
        day_index: WireDayIndex,
        next_interval_index: InProgressIntervalIndex,
        accepted_interval_count: AcceptedIntervalCount,

        committed_day_beginning:
            CompleteCommittedOwnerStateV1,

        staged_scientific:
            ScientificOwnerStateSetV1,

        accepted_gsi_daily_receipt:
            DirectGsiDailyReceiptRestartV1,

        staged_gsi_ending_state:
            DirectGsiOwnerStateRestartV1,

        ending_provider_cursor:
            SnowFreeHalfHourProviderCursorRestartV1,

        validated_forcing_day_receipts:
            Vec<SnowFreeHalfHourDayReceiptRestartV1>,
    },
}
```

The complete committed owner set includes:

```text
GSI configuration
GSI state
static forcing configuration
provider cursor
surface-liquid configuration
V10 vegetation
LSE-V2
direct hydrology, including surface-liquid state
soil thermal
BGC
```

The staged scientific owner set includes:

```text
V10 vegetation
LSE-V2
direct hydrology
soil thermal
BGC
```

Do not place a second direct-hydrology owner at checkpoint top level.

Do not assign interval transaction lineage to GSI or provider cursor.

## Between-days validation

Require:

```text
cursor next day == checkpoint next day
accepted interval count divisible by 48
scientific owner transaction lineage exact
GSI chronology exact
surface-liquid boundary posture exact
no staged candidate
```

## In-progress validation

Require:

```text
next interval in 1..=47
accepted interval count modulo 48 equals next interval
committed cursor points at current day
ending cursor points at next day
GSI beginning state equals committed GSI
GSI ending state equals staged ending state
GSI run/day/source joins forcing receipts
every destination appears exactly once
every destination has 48 intervals
every interval binds exact GSI receipt
carry union equals ending cursor carry
staged science transaction equals last accepted interval
committed day beginning remains immutable
```

# Phase 8 — Add Strict Admission and Atomic Installation

Checkpoint parsing and restoration must never mutate a live owner.

Required sequence:

```text
raw bytes
→ duplicate-aware parse
→ typed parse
→ canonical-byte equality
→ outer digest
→ schema/version
→ static identities
→ phase semantics
→ nested owner digests
→ owner-domain validation
→ cross-owner joins
→ isolated runtime construction
→ V10→V9 and LSE-V2→V1 verification
→ reconstructed cache checks
→ complete restored checkpoint candidate
```

Only then may a caller replace a live default-off shadow attachment.

Provide an atomic install conceptually like:

```rust
pub fn install_restored_checkpoint(
    target: &mut DirectV10RestartHost,
    restored: IsolatedRestoredCheckpointV1,
) -> Result<(), RestartInstallError> {
    // All fallible checks before this point.
    *target = restored.into_host();
    Ok(())
}
```

Prefer restoration by constructing a new `DirectV10RealConsumerShadow` or a
new in-progress transaction and replacing the old object once.

Do not sequentially assign:

```text
GSI
cursor
vegetation
LSE
hydrology
soil thermal
BGC
scheduler
```

## Failure injection

Inject failures after:

```text
raw parse
canonical comparison
outer digest
static identity
GSI restoration
cursor restoration
forcing restoration
V10 restoration
V10→V9 projection
LSE-V2 restoration
LSE-V2→V1 projection
hydrology restoration
surface-liquid validation
soil-thermal restoration
BGC restoration
phase validation
cache reconstruction
complete owner join
immediately before assignment
```

For each failure:

```text
capture actual live canonical owner bytes
attempt restore/install
require exact typed category
recapture actual live canonical owner bytes
require byte-identical equality
```

Do not use `Debug` strings or before/after hash-only evidence.

# Phase 9 — Refactor the Prepared Day Into a Resumable Transaction

The existing all-day prepared execution remains the physical source of truth.

Refactor sealed internals into a restartable default-off transaction without
changing accepted full-day behavior.

Conceptually:

```rust
pub(crate) struct DirectV10PreparedDayTransactionV1 {
    committed_day_beginning: CompleteCommittedOwnerState,
    staged_scientific: ScientificOwnerState,
    accepted_gsi_receipt: DirectGsiDailyReceiptV1,
    staged_gsi_ending_state: GsiState,
    ending_provider_cursor: SnowFreeHalfHourProviderCursor,
    validated_forcing_receipts: ValidatedSnowFreeHalfHourForcingReceipts,
    projected_day_input: DirectV10ShadowDayInput,
    next_interval_index: usize,
    accepted_interval_count: u64,
}
```

Required operations:

```rust
prepare(...)
advance_one_interval(...)
checkpoint(...)
restore(...)
finish(...)
abort(...)
```

## Prepare

Prepare must:

```text
clone the complete committed shadow
derive repository-owned GSI forcing
advance GSI exactly once on a clone
construct accepted GSI receipt
prepare forcing receipts and ending cursor
project exact V10/LSE day input
retain immutable day-beginning owners
initialize staged scientific owners
set next interval to zero
```

No owner commits.

## Advance one interval

Each interval must:

```text
execute the exact existing V10/LSE-V2 physical path
use current staged scientific owners
retain exact transaction identity
advance staged scientific state only
increment next interval only after acceptance
leave committed owners unchanged
```

Do not advance GSI or cursor per interval.

## Checkpoint

At interval zero, serialize `BetweenDays`.

At intervals 1 through 47, serialize `InProgressDay`.

At interval 48, the transaction must either:

```text
finish and serialize BetweenDays
```

or remain uncommitted in memory. Do not serialize interval 48 as
`InProgressDay`.

## Restore

Restore an in-progress transaction from canonical checkpoint bytes and static
context.

Drop all source runtime objects in tests before restoration.

## Finish

After all 48 intervals:

```text
validate staged scientific owners
validate GSI ending state
validate ending cursor
validate forcing receipts and carry
validate complete cross-owner lineage
construct one new committed owner set
perform one non-fallible replacement
```

## Abort

Abort must return the exact committed day-beginning owner set, including after
process-style restore at interval 24.

# Phase 10 — Restart Acceptance Matrix

Implement contract-derived integration tests, preferably under:

```text
tests/integration/
  direct_hydrology_persisted_restart_implementation_contract.rs

tests/integration/
  direct_v10_real_consumer_checkpoint_v1_contract/
```

Exact names may vary.

## Canonical byte tests

Require:

```text
repeated serialization produces identical bytes
production serialization matches all four authority vectors
every authority vector parses and reserializes byte-identically
signed zero preserved
U128 preserved
no trailing LF
no Debug representation
no Rust-layout dependency
```

## Fresh-object continuation

Require:

```text
between-day restore
interval-24 restore
day-boundary restore
multi-day restore
cross-midnight precipitation-carry restore
multi-destination restore
```

For interval 24:

```text
execute intervals 0..23
serialize
drop all source objects
restore fresh
execute intervals 24..47
finish
```

Compare against uninterrupted execution.

## Owner equivalence

Compare exact canonical bytes and state values for:

```text
GSI state
provider cursor
V10 vegetation
LSE-V2
direct hydrology
surface liquid
soil thermal
BGC
scheduler position
day receipt
```

## Required climate scenarios

At minimum:

```text
complete zero-radiation 48-interval day
complete realistic positive-radiation day
cross-midnight breakpoint storm
two-day continuation
GSI history progression
calendar-year rollover
open tile
covered tile
mixed open and covered destinations
multi-OFE
wet surface
dry surface
```

## Rollback

Require exact rollback for:

```text
failure before interval 1
failure at interval 15
failure at interval 24
failure at interval 47
failure after restore
failure immediately before finish
```

## Released poison matrix

Execute every released authority poison through the production admission API.

At minimum:

```text
schema
unsupported version
noncanonical bytes
payload digest
missing field
extra field
reordered field
duplicate field
run identity
topology identity
configuration identity
owner identity
transaction lineage
scheduler position
provider cursor
GSI receipt
heterogeneous GSI receipt
forcing cardinality
forcing order
forcing digest
V10/V9 projection
LSE-V2/V1 projection
owner validation
unsupported laned-active
canonical order
owner omission
retained snow liquid
groundwater posture
groundwater total area
erosion publication
surface-liquid configuration
signed-zero mutation
U128 truncation
cursor rewind
cursor skip
GSI history reorder
carry omission
destination omission
interval omission
interval duplication
committed/staged substitution
staged hydrology omission
semantic mutation with all outer digests recomputed
```

# Phase 11 — Restart Performance and Resource Qualification

Freeze benchmark selectors and budgets before optimization.

Measure:

```text
between-days serialization
between-days admission
between-days isolated restore
interval-24 serialization
interval-24 admission
interval-24 isolated restore
cross-midnight restore
multi-destination restore
one resumed interval
remaining 24 resumed intervals
finish
abort
checkpoint byte sizes
peak allocation
```

Record:

```text
hardware
OS
Rust toolchain
commit
selector
warm-up
sample count
median
maximum
budget
raw logs
```

Do not use zero-test or empty-owner fixtures.

If checkpoint size or restore cost is material, optimize representation or
allocation without changing canonical bytes.

Do not compress or change the released wire format unless the authority
explicitly admits it.

# Phase 12 — Restart Implementation Reviews and Gates

Before review, require:

```text
all released vector parity
all restart-equivalence cases
all poison cases
all failure injections
all focused checks
warnings-denied Clippy
formatting
diff hygiene
package documentation lint
line-count governance
```

Request fresh read-only reviews:

1. restart/hydrology continuation reviewer;
2. Rust/API/serialization reviewer.

Both must review one exact commit.

Disposition every finding:

```text
accepted
rejected with evidence
deferred outside package
nonblocking follow-up
```

Fix every accepted material finding and rerun invalidated tests.

Then run terminal verifier A and terminal verifier B against exact final bytes.

## Focused commands

```bash
cargo test \
  --manifest-path \
  docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/tools/restart-authority-reference/Cargo.toml

cargo clippy \
  --manifest-path \
  docs/work-packages/20260817-direct-hydrology-persisted-restart-authority-001/tools/restart-authority-reference/Cargo.toml \
  --all-targets -- -D warnings

cargo check \
  -p openwepp-plant-phenology \
  -p openwepp-hillslope-orchestrator \
  -p openwepp-land-surface-energy \
  -p openwepp-vegetation \
  -p openwepp-biogeochemistry \
  -p openwepp-runner

cargo nextest run \
  --test direct_hydrology_restart_authority_contract \
  --profile quick --no-fail-fast

cargo nextest run \
  --test direct_hydrology_persisted_restart_implementation_contract \
  --profile quick --no-fail-fast

cargo nextest run \
  --test direct_v10_real_consumer_checkpoint_v1_contract \
  --profile quick --no-fail-fast

cargo nextest run \
  --test snow_free_half_hour_forcing_adapter_contract \
  --profile quick --no-fail-fast

cargo nextest run \
  --test v10_nighttime_authority_contract \
  --profile quick --no-fail-fast

cargo clippy \
  -p openwepp-plant-phenology \
  -p openwepp-hillslope-orchestrator \
  -p openwepp-land-surface-energy \
  -p openwepp-vegetation \
  -p openwepp-biogeochemistry \
  -p openwepp-runner \
  --all-targets -- -D warnings

bash tools/release/check_authority_suite_antievasion.sh

cargo nextest run \
  --test auth11_required_suite_obligation_guards_contract \
  --profile quick --no-fail-fast

cargo fmt --all -- --check
git diff --check
```

Run Markdown lint for the implementation package and all downstream packages
changed by lifecycle reconciliation.

## Heavy gates

Install the already-required toolchain component or use the repository CI
environment:

```bash
rustup component add llvm-tools-preview
```

Use short external scratch:

```bash
mkdir -p /tmp/ow-restart-v1
```

Run:

```bash
cargo clippy --workspace --all-targets -- -D warnings

TMPDIR=/tmp/ow-restart-v1 \
  cargo nextest run --workspace --profile full

cargo test --doc --workspace
cargo deny check
cargo fmt --all -- --check
git diff --check
```

Record exact selected, passed, failed, and skipped counts.

Preserve failed runs and explain any infrastructure-only rerun.

# Phase 13 — Close Persisted Restart Implementation

Archive the implementation kickoff prompt byte-identically only after:

```text
restart review PASS
Rust review PASS
terminal verifier A PASS
terminal verifier B PASS
full exact-head gates PASS
```

Close as:

```text
COMPLETE /
DirectHydrologyRestartV1 and
DirectV10RealConsumerCheckpointV1 persisted restart implemented /
default-off only
```

Explicit limitations:

```text
production activation = NOT AUTHORIZED
production cutover = NOT AUTHORIZED
production selector = UNCHANGED
production outputs = UNCHANGED
deployment = NOT PERFORMED
```

Create or finalize:

```text
artifacts/authority-parity-ledger.md
artifacts/implementation-and-test-evidence.md
artifacts/restart-equivalence.md
artifacts/cross-midnight-carry-equivalence.md
artifacts/failure-injection-matrix.md
artifacts/production-no-mutation.md
artifacts/benchmark-results.md
artifacts/line-count-governance.md
artifacts/review_hydrology_restart.md
artifacts/review_rust.md
artifacts/review-finding-disposition.md
artifacts/verification_agent_a.md
artifacts/verification_agent_b.md
artifacts/terminal-diff-reconciliation.md
artifacts/gate-results.md
artifacts/final-disposition.md
artifacts/worker-handoff.md
```

Commit locally:

```text
Implement persisted V10 real-consumer restart
```

Do not push.

Do not stop. Continue immediately to forcing-adapter closure.

# Phase 14 — Close the Half-Hour Forcing Adapter

Resume:

```text
20260817-snow-free-half-hour-forcing-adapter-implementation-001
```

Update status to:

```text
executing /
physics PASS /
stateful GSI, cursor and persisted restart integration active
```

Required terminal evidence:

```text
repository-derived GSI forcing
one GSI owner advance per day
accepted GSI receipt held over all 48 intervals
static provider identity independent of daily GSI
provider cursor staged and atomically committed
provider cursor persisted and restored
complete destination × 48 receipt persistence
cross-midnight precipitation carry persistence
zero-radiation day PASS
realistic-radiation day PASS
restart-equivalent provider results
no raw closure-eligible forcing injection
no cursor-free consumer execution
```

Audit public visibility.

Closure-eligible external execution must require:

```text
prepared GSI receipt
staged GSI ending state
prepared provider cursor transition
validated forcing receipts
complete V10/LSE-V2 transaction
```

Make raw or legacy bypasses crate-private or test-only.

Run fresh:

1. forcing-science review;
2. Rust/ownership review;
3. terminal verifier A;
4. terminal verifier B.

Archive the forcing prompt only after PASS.

Close as:

```text
COMPLETE /
native 48-step V10/LSE-V2 repository forcing provider /
stateful CP-GSI01 owner /
atomically persisted provider cursor /
default-off only
```

Commit locally:

```text
Close native half-hour forcing provider
```

Do not push.

Do not stop. Resume Child 4.

# Phase 15 — Resume and Complete Child 4

Resume:

```text
20260814-vegetation-land-surface-real-consumer-shadow-001
```

Reconcile active documentation from historical V9 wording to the actual
terminal implementation:

```text
OPENWEPP_C3_WOODY_V10
OPENWEPP_SNOW_FREE_LSE_V2
native half-hour forcing
persisted restart V1
```

Preserve historical V9/V8 adaptation evidence in its historical artifacts.

Do not rewrite historical closed evidence as though V10 existed earlier.

Set active status:

```text
executing /
real provider and persisted restart prerequisites satisfied /
terminal scenarios and gates active
```

## Actual scheduler execution

Prove the actual lane-interleaved scheduler:

```text
captures actual repository day receipts
runs the default-off V10/LSE-V2 shadow
supports between-day and interval-24 restart
publishes nothing until shadow acceptance
atomically replaces only shadow owner state
never mutates production owners
```

No separate synthetic scheduler loop is terminal evidence.

## Required repository scenarios

At minimum:

```text
open bare mineral soil
evergreen covered forest litter
seasonal-deciduous forest litter
two overlapping canopy strata
mixed open and covered tiles
multi-OFE execution
wet canopy
dry canopy
wet litter
dry litter
daytime positive radiation
nighttime longwave cooling
zero-radiation day
realistic day/night cycle
full water supply
supported partial supply
typed unsupported nighttime partial root supply
root and ground competition
condensation into litter
condensation overflow
rain with infiltration and runoff
routed runon
cross-midnight rainfall carry
shared NH4/NO3 competition
restart at interval 24
restart at day boundary
multi-day restart
snow-present typed rejection
terminal-snow typed rejection
unsupported calm/nonneutral/frozen branches
```

Use actual repository fixtures where available.

Synthetic fixtures are acceptable for poisons only.

## Exact-one custody

Prove exact-one ownership for:

```text
canopy and ground shortwave
canopy-ground-atmospheric longwave
canopy and ground vapor
latent energy
ground heat
precipitation and runon enthalpy
canopy release
infiltration
runoff and routed runoff
V10 carbon
V10 nitrogen
BGC NH4/NO3
material proposals and receipts
soil thermal energy
surface liquid
```

## Production invariance

From the same production beginning state, require exact production invariance
with the shadow present versus absent for:

```text
DirectRunFrame
soil water
surface liquid
snow/frost
residue
runoff
infiltration
erosion inputs
WAT
published rows
scheduler counters
restart payload
model selectors
configuration defaults
```

The shadow may publish its own explicit diagnostic result object only.

## Publication buffering

Measure the current buffering implementation.

If memory grows without a bounded run-level limit, replace it with a bounded
staging owner, preferably scratch-file backed:

```text
stage
validate
replay only after complete shadow acceptance
abort without publication
```

Do not claim external-sink rollback after the sink has accepted replayed data.

State the exact publication atomicity boundary.

## Domain coverage audit

Report:

```text
total candidate intervals
supported intervals
rejected intervals
typed rejection count and fraction
site/run/day coverage
```

Include:

```text
snow
terminal snow
calm wind
nonneutral forcing
frozen branch
missing litter properties
missing surface-liquid state
missing parcel temperature/enthalpy
unsupported surface
solver nonconvergence
provider identity
restart identity
```

This is engineering coverage, not empirical validation.

## Legacy versus shadow comparison

Compare unchanged legacy production and V10 shadow structurally.

Report:

```text
legacy ET
V10 transpiration
wet-canopy evaporation/condensation
litter/surface evaporation/condensation
mineral-soil evaporation
runoff
infiltration
soil-water change
surface-liquid change
energy components
surface temperatures
```

Prove at least one case where denied canopy demand is not donated to ground
evaporation.

Agreement with legacy is not an acceptance criterion.

## Benchmarks

Run all seven corrected-litter benchmark surfaces.

Also benchmark:

```text
actual scheduler day
interval-24 checkpoint
interval-24 restore and continuation
multi-day shadow
publication staging
```

Freeze budgets before optimization and retain raw evidence.

## Child-4 reviews

Require fresh:

1. land-surface/vegetation science review;
2. hydrology/ownership review;
3. Rust/API review;
4. comparator suite;
5. terminal verifier A;
6. terminal verifier B.

Fix every accepted material finding.

Run exact-head full workspace and all package gates.

Close Child 4 as:

```text
COMPLETE /
V10 vegetation–LSE–real-hydrology actual-scheduler shadow complete /
native half-hour provider /
persisted restart /
default-off only
```

Retain:

```text
production activation = NOT AUTHORIZED
production selector = UNCHANGED
production outputs = UNCHANGED
calibration = NOT CLAIMED
empirical validation = NOT CLAIMED
transferability = NOT CLAIMED
```

Archive Child-4 prompt only after both terminal verifiers PASS.

Commit locally:

```text
Complete V10 real-consumer shadow
```

Do not push.

Do not stop. Close the campaign.

# Phase 16 — Campaign-Wide Terminal Closure

Resume:

```text
20260814-snow-free-land-surface-real-hydrology-integration-001
```

Reconcile the coordinator truthfully.

Historical HOLD statements such as:

```text
no authoritative half-hour provider exists
daily climate cannot be disaggregated
persisted hydrology restart is unavailable
```

must remain available as historical evidence but must no longer appear as
current blockers.

Do not delete historical HOLD artifacts.

Update campaign progress:

```text
[x] Child 1 complete
[x] Child 2 complete
[x] Child 3 complete
[x] Child 4 complete
[x] native half-hour forcing complete
[x] persisted restart complete
[x] campaign-wide reviews complete
[x] heavy gates complete
[x] dual terminal verification complete
```

## Campaign reconciliation

Verify:

```text
each requirement belongs to one child
no PASS was borrowed across child boundaries
all child final dispositions identify exact commits
all active prompts are archived
all review findings are dispositioned
all changed files belong to a declared write set
all production exclusions remain true
```

## Fresh campaign reviews

Run the campaign-required:

1. land-surface science reviewer;
2. hydrology/ownership reviewer;
3. Rust correctness reviewer;
4. comparator runner;
5. terminal verifier A;
6. terminal verifier B.

These may inspect child evidence but must issue exact-current campaign verdicts.

## Final exact-head gates

Install `llvm-tools-preview` if not already present.

Run:

```bash
cargo clippy --workspace --all-targets -- -D warnings

TMPDIR=/tmp/ow-snowfree-campaign \
  cargo nextest run --workspace --profile full

cargo test --doc --workspace
cargo deny check
cargo fmt --all -- --check
git diff --check
```

Run:

```text
authority suite
anti-evasion
AUTH11
all relevant science-contract tests
package Markdown lint
documentation-link scan
line-count governance
benchmark gates
production selector/default/publication source guards
```

Freeze exact counts and logs.

## Roadmap and catalog

After campaign terminal PASS:

1. Remove the completed snow-free campaign row from `docs/ROADMAP.md`.
2. Add its final outcome to `docs/work-packages/README.md`.
3. Mark restart implementation COMPLETE.
4. Mark forcing implementation COMPLETE.
5. Mark Child 4 COMPLETE.
6. Mark campaign COMPLETE.
7. Update stale backlog or handoff pointers.
8. Do not leave a completion summary in the forward-only roadmap.
9. Preserve future soil-transformation and canopy-snow work as separate
   prospective items.

Archive the campaign prompt only after both campaign terminal verifiers PASS.

Campaign terminal status:

```text
COMPLETE /
snow-free vegetation–land-surface–real-hydrology integration shadow complete /
default-off actual-scheduler real consumer only
```

Required retained limitations:

```text
runtime_activation = NOT_AUTHORIZED
production_cutover = NOT_AUTHORIZED
production_selector = UNCHANGED
production_defaults = UNCHANGED
production_outputs = UNCHANGED
calibration_evidence_status = NOT_CALIBRATION_READY
identifiability_status = NOT_ASSESSED
empirical_validation = NOT_CLAIMED
transferability = NOT_CLAIMED
canopy_snow = NOT_IMPLEMENTED
snow_terminal_handoff = NOT_IMPLEMENTED
soil_biogeochemical_transformations = NOT_IMPLEMENTED
multi_year_carbon_nitrogen_calibration =
    BLOCKED_ON_SOIL_TRANSFORMATIONS
```

Commit locally:

```text
Close snow-free land-surface integration campaign
```

Do not push.

# Suggested Local Commit Boundaries

Use intentional commits such as:

```text
1. Activate persisted restart implementation
2. Implement canonical DirectHydrologyRestartV1
3. Implement typed V10 checkpoint admission
4. Add resumable V10 prepared-day transaction
5. Prove persisted restart equivalence and rollback
6. Close persisted restart implementation
7. Close native half-hour forcing provider
8. Complete V10 real-consumer shadow
9. Close snow-free integration campaign
```

Before every commit:

```bash
git status --short
git diff --check
cargo fmt --all -- --check
```

Do not push.

# No Short-Cycling Rule

Do not stop or request user direction because:

```text
a production module is large
private fields need sealed pub(crate) accessors
the authority code must be promoted
a dependency needs refactoring
a test fails
Clippy fails
checkpoint bytes are large
continuation requires a transaction object
publication staging requires scratch files
a reviewer finds an in-scope defect
full workspace is expensive
llvm-tools-preview is absent
documentation needs reconciliation
```

Do not dispatch terminal review after merely:

```text
serializing DirectRunFrame
parsing one released vector
passing between-day restart
passing interval-24 restart
closing only the restart package
```

Continue through the package and campaign sequence.

# Legitimate Future HOLD

A HOLD is legitimate only for a newly demonstrated exact contradiction such
as:

* released authority bytes cannot represent an actual continuation-affecting
  runtime field;
* a released wire field cannot be restored without changing its scientific
  meaning;
* V10→V9 or LSE-V2→V1 reconstruction changes a non-identity payload;
* surface-liquid state cannot validate against its released configuration;
* a fresh-process interval-24 restore cannot reconstruct the remaining forcing
  or owner state;
* abort after restore cannot return exact committed day-beginning owners;
* the actual scheduler cannot preserve production output invariance;
* atomic installation cannot be implemented without partial live-owner
  mutation.

Before declaring HOLD:

```text
identify the exact released clause or field
identify exact source locations
show the failing real fixture
show all safe implementation routes attempted
show why each route violates released authority
name the first concrete authority-lift action
```

Do not alter the released authority in place.

Autonomy:

Execute persisted restart implementation, forcing-adapter closure, Child-4
terminal work, and campaign closure without asking for direction unless one
new exact released-authority, owner, continuation, or production-invariance
contradiction remains after all safe implementation routes are exhausted.

```

