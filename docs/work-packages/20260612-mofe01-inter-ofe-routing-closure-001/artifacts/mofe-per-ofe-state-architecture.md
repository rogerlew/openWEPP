# MOFE per-OFE State Architecture - Design Artifact (increment M-D)

Status: M-D complete; design-only; no production code edits

Evidence mode: Static

## Purpose

M-D defines the state architecture required before M-E can implement real
per-OFE daily water-balance state. It resolves the M-C/M-C2 blocker without
authorizing surrogate WAT row splitting from the current aggregate WB13 state.

The design target is an OFE-keyed daily state/flux collection that replaces the
current single aggregate writeback surface at the scheduler/writeback boundary
and gives publication code actual per-OFE state to consume. M-D intentionally
does not edit production Rust, tests, or science contracts.

## Current Boundary

| Seam | Static finding | Current-tree citation |
| --- | --- | --- |
| Scheduler writeback surface | One `HillslopeWritebackSurface` owns one aggregate `state_surface` map and one aggregate `flux_surface` map. | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:240` |
| Scheduler execution report | One `HillslopeKernelExecutionReport` returns one `writeback_surface`. | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:258` |
| Kernel lifecycle entry | `execute_with_kernel` receives one runtime writeback surface and returns one updated report. | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:501` |
| Kernel request assembly | Each phase request borrows the same aggregate state and flux maps. | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:700` |
| Kernel writeback application | Phase writeback updates the same aggregate maps. | `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:769` |
| Contract payload | `KernelWritebackPayload` is a pair of scalar update vectors, not an OFE-keyed collection. | `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs:1018` |
| Kernel request type | `HillslopeKernelRequest` exposes aggregate `state_surface` and `flux_surface` references only. | `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs:1384` |
| Runner daily loop | The runner executes one scheduler/kernel lifecycle per day and assigns one aggregate runtime surface. | `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs:1159` |
| Runner WB13 rows | The runner pushes one `wb13_row` per day from the aggregate execution result. | `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs:1186` |
| WAT publication | WAT rows are built only from the one-row-per-day `wb13_rows` vector. | `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs:1276` |
| WB13 construction | Current WB13 publication reads aggregate scalar surfaces and emits `UpStrmQ=0`, `QOFE=Q`, and `OFE=1`. | `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:980`; `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:988`; `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:995`; `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:1013` |
| WAT row projection | WAT rows consume the one `Wb13DailyWaterBalanceRow` surface per day. | `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:536` |
| Current publication policy | MOFE publication provenance is explicitly `single-row-canonicalized-hillslope-aggregate`. | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs:151` |
| Topology graph | Topology nodes are hillslope/channel/impoundment system nodes, not OFE-lane nodes. | `crates/openwepp-topology/src/lib.rs:19` |
| Runner topology setup | Hillslope runtime currently builds a one-hillslope graph. | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:1469` |
| Static OFE input symbols | Existing projection helpers already know how to address `ofeN_*` symbols for static input state. | `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs:930` |
| Current MOFE carry arrays | MOFE hourly carry arrays are enabled and seeded, but they are transfer/copy-forward arrays, not full per-OFE daily WB state. | `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:30`; `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:45`; `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:1122`; `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:1137` |
| Carry resolver | WB12/WB14 resolves runon from upstream hourly carry arrays and publishes scalar `UpStrmQ`/`SubRIn` only on the current aggregate surface. | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs:268` |
| Carry copy-forward | Current saturation/lateral arrays are copied into upstream arrays for the next lane inside the aggregate lifecycle. | `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs:1136` |
| Summary guard | Summary accumulation currently rejects `QOFE != Q`, matching the aggregate canonicalized policy rather than real per-OFE semantics. | `crates/openwepp-summary-accumulator/src/lib.rs:277` |

## Legacy Obligations

The pinned legacy baseline carries a per-plane state machine that openWEPP must
model as explicit state rather than publication synthesis:

| Legacy seam | Obligation | Baseline citation |
| --- | --- | --- |
| WATBAL row emission | WATBAL writes per-`iplane` rows with `runoff`, `runoffin`, `subrin`, `sbrunf`, and `QOFE` terms. | `/workdir/wepp-forest_260430_baseline/src/watbal.for:1073` |
| Runon and subsurface input | WATBAL computes `runoffin` and `subrin` from upstream terms before mutating the current plane balance. | `/workdir/wepp-forest_260430_baseline/src/watbal.for:343` |
| Plane loop | IRS loops over OFEs and carries upstream runoff continuation state. | `/workdir/wepp-forest_260430_baseline/src/irs.for:244` |
| Equivalent-plane continuation | IRS accumulates equivalent-plane values while upstream runoff continues. | `/workdir/wepp-forest_260430_baseline/src/irs.for:335` |
| Continuation classification | IRS uses cases 1-4 and calls `rochek` for runon/runoff continuation decisions. | `/workdir/wepp-forest_260430_baseline/src/irs.for:356`; `/workdir/wepp-forest_260430_baseline/src/irs.for:458` |
| WATBAL mutation isolation | IRS temporarily replaces `runoff(1..iplane)`, calls WATBAL, then copies values back because WATBAL may mutate runoff. | `/workdir/wepp-forest_260430_baseline/src/irs.for:541` |
| Saturation surplus handoff | IRS injects `surdra` into the event shape and may set downstream runon. | `/workdir/wepp-forest_260430_baseline/src/irs.for:566` |
| `rochek` classifier | `rochek` determines whether upstream runoff stops or continues after current-plane infiltration capacity. | `/workdir/wepp-forest_260430_baseline/src/rochek.for:1`; `/workdir/wepp-forest_260430_baseline/src/rochek.for:79`; `/workdir/wepp-forest_260430_baseline/src/rochek.for:102` |
| Erosion qin/qout | `xinflo` and `route` consume water-routing state later; that coupling is M-G, not M-D/M-E. | `/workdir/wepp-forest_260430_baseline/src/xinflo.for:130`; `/workdir/wepp-forest_260430_baseline/src/route.for:139` |

## Target Per-OFE State Shape

Introduce a first-class `PerOfeDailyWaterBalanceCollection` owned by the
hillslope daily lifecycle. The collection is keyed by one-based OFE index and
has one record per contributing OFE for every executed simulation day.

Target shape:

```text
PerOfeDailyWaterBalanceCollection
  simulation_day_index
  contributor_ofe_count
  records: Vec<PerOfeDailyWaterBalanceRecord> keyed OFE=1..N
  aggregate: DailyHillslopeAggregate
  transfer_identity: DailyTransferIdentity
  per_element_identity: Vec<DailyPerElementIdentity>
  publication_policy
```

Each `PerOfeDailyWaterBalanceRecord` owns:

| Field family | Contents |
| --- | --- |
| Identity | `ofe_index`, `year`, `julian_day`, contributor geometry, static area/effective-length/width, source slope segment ids. |
| Static input slice | OFE-local soil/slope/management inputs selected from existing `ofeN_*` symbols plus shared day/climate controls. |
| Persistent dynamic state | OFE-local WB11/WB18/WB19/WB13 state surfaces that persist across days: layer water, frozen water, snow water, profile stores, plant/cover state needed by water-balance consumers. |
| Day flux surface | OFE-local same-day fluxes: `P`, `RM`, `Irr`, `Ep`, `Es`, `Er`, `Dp`, `Tile`, `latqcc`, partition runoff, surface-saturation runoff, drainage/lateral terms, and closure residual diagnostics. |
| Upstream transfer input | `UpStrmQ`, `SubRIn`, 24-slot upstream saturation array, 24-slot upstream lateral array, area-scaling provenance, and source OFE id. |
| Current transfer output | `QOFE`, current surface runoff sent downstream, current lateral runoff sent downstream, `ui_SCrunf[24]`, `ui_LfCrf[24]`, and downstream recipient OFE id when one exists. |
| Publication row | The authoritative WB13/WAT row terms for this OFE/day, derived from the OFE-local dynamic state and day fluxes, not from aggregate row splitting. |

The existing scalar `HillslopeWritebackSurface` remains valid only as a
single-OFE compatibility representation or as the inner lane request surface
during staged migration. It is no longer the outer daily truth for multi-OFE
hillslopes after M-E.

### Lifecycle

1. At run initialization, derive `contributor_ofe_count` from the slope intake
   and construct one OFE lane state per contributor. Existing `ofeN_*` static
   symbols seed static slices; dynamic WB state is initialized per OFE, not
   reconstructed from publication rows.
2. At each simulation day, create an empty daily collection with `N` records.
3. Execute OFEs in upstream-to-downstream order.
4. For OFE 1, upstream transfer input is exactly zero and carries explicit
   provenance.
5. For OFE i > 1, upstream transfer input is the finalized transfer output from
   OFE i-1 after the declared area/effective-length conversion.
6. Run the kernel phase graph on the OFE-local lane state, using the upstream
   transfer input as same-day forcing.
7. Persist the OFE-local post-day dynamic state for the next day.
8. Store the OFE-local publication row and transfer-output arrays in the daily
   collection.
9. After OFE N, compute the aggregate hillslope identity from explicit OFE
   records and internal transfer cancellation. Aggregate outputs are derived
   summaries, not state authority.

Hourly transfer arrays reset per day/OFE as legacy `wathour` event/carry
workspace. Persistent WB storage does not reset at OFE boundaries or day
boundaries unless the underlying process contract requires it.

## Sequential Execution Model

M-E should implement per-OFE lane iteration over the existing phase scheduler,
not encode OFEs as `TopologyGraph` nodes.

Rationale:

- The current `TopologyGraph` is a watershed/system graph with node kinds
  hillslope, channel, and impoundment, not a hillslope-internal OFE graph
  (`crates/openwepp-topology/src/lib.rs:19`).
- The current runner constructs a one-hillslope topology graph at the daily
  scheduler seam (`crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs:1469`).
- The kernel/scheduler phase graph already expresses WB11/WB12/WB14/WB18/WB19
  process order. OFE routing is an iteration dimension around that graph, not a
  new watershed element type.
- Reusing `execute_with_kernel` as an inner lane executor preserves single-OFE
  behavior: N=1 executes exactly one lane, with zero upstream transfer and the
  same phase order.

Target daily pseudo-flow:

```text
for day in simulation_days:
    incoming = TransferInput::zero_for_first_ofe()
    collection = PerOfeDailyWaterBalanceCollection::new(day, ofe_count)

    for ofe in 1..=ofe_count:
        lane_surface = assemble_lane_surface(
            static_ofe_slice[ofe],
            persistent_dynamic_state[ofe],
            shared_day_forcing,
            incoming,
        )

        report = execute_with_kernel(lane_surface, context.with_ofe(ofe))
        record = extract_per_ofe_record(report.writeback_surface, incoming)
        collection.push(record)

        persistent_dynamic_state[ofe] = record.post_day_state
        incoming = record.transfer_output.as_downstream_input()

    collection.validate_identities()
    aggregate_surface = collection.aggregate_for_legacy_outer_consumers()
```

Legacy `irs`/`rochek` mapping:

- `irs` plane iteration maps to the outer `for ofe in 1..=ofe_count` loop.
- `rochek` case 3/4 continuation maps to a typed runoff-continuation classifier
  that decides whether downstream `UpStrmQ`/event shape carries current-plane
  runoff continuation or zero.
- Equivalent-plane accumulation maps to explicit transfer/output metadata and
  effective-length provenance rather than hidden aggregate row rewriting.
- WATBAL mutation isolation maps to lane-local state ownership: a phase may
  mutate the current lane state but cannot mutate finalized upstream records.
- Surface-saturation addback maps to current-lane `ui_SCrunf[24]` and outgoing
  `QOFE`; downstream `ui_SUrunf[24]` is derived by transfer, not by aggregate
  scalar substitution.

## Contract Surface

M-D does not amend contracts, but M-E0 must amend the following contract
surfaces before production implementation:

| Contract | Required amendment |
| --- | --- |
| `SC-RUNOFFPART-001` | Promote the existing MOFE hourly carry and M-B conservation addenda into a per-OFE lane execution invariant: case classifier, continuation state, transfer arrays, partition/runon identity, and no aggregate substitution. Existing `INV-RUNOFFPART-013`, `014`, and `028` are the current anchors. |
| `SC-WATBAL-001` | Replace the current MOFE04 aggregate WB13/WAT policy with a gated policy transition: per-OFE dynamic state row semantics, WB13 row cardinality, `QOFE` as current-OFE outlet/export term, `UpStrmQ`/`SubRIn` as current-OFE upstream inputs, and aggregate storage derivation only from explicit OFE records. Existing MOFE04 and HPHYS0255 addenda must remain historical/current-policy authority until M-E flips the implementation. |
| `SC-SYSTEM-001` | Add publication manifest semantics for `per_ofe_dynamic_state` policy, contributor count, per-OFE row cardinality, transfer-identity evidence, storage-lineage policy transition, and downstream fail-closed intake behavior. Existing `INV-SYSTEM-028` and `INV-SYSTEM-029` remain current-policy anchors. |

Measurable invariants for M-E:

1. Per-element identity:
   `local_liquid_i + UpStrmQ_i + SubRIn_i - outflows_i - delta_storage_i = residual_i`
   where `outflows_i` includes current-OFE outlet runoff (`QOFE_i`), lateral
   export (`latqcc_i` when it exits the OFE lane), ET, deep percolation, tile,
   and other contract-declared outputs. The final equation and units must be
   pinned in `SC-WATBAL-001` before implementation.
2. Transfer identity:
   for adjacent OFE pairs and each hour `h`,
   `sent_surface_i[h] == received_surface_{i+1}[h]` and
   `sent_lateral_i[h] == received_lateral_{i+1}[h]` after the declared area
   conversion. Daily `UpStrmQ_{i+1}` and `SubRIn_{i+1}` must equal the sums of
   the received arrays in the contract units.
3. Single-OFE zero-transfer identity:
   `UpStrmQ_1 == 0`, `SubRIn_1 == 0`, upstream arrays are all zero, and N=1
   outputs stay bit-identical or at-noise against the pre-M-E anchor.
4. Hillslope-total identity:
   internal transfer terms cancel. The whole-hillslope residual includes only
   external climate/irrigation inputs, ET, percolation, tile/drain exports,
   outlet runoff at OFE N, and net storage change.
5. Publication identity:
   WB13/WAT rows are keyed `(Y, J, OFE)` with exactly N rows per day for
   multi-OFE hillslopes. Duplicate keys, missing OFEs, aggregate-only rows, or
   `QOFE=Q` aliasing without OFE-local derivation are hard failures.

## Change Map

| Area | Required change | Migration note |
| --- | --- | --- |
| `openwepp-kernel-contract` | Add OFE scope metadata and a per-OFE writeback collection type. The inner scalar `KernelWritebackPayload` can remain the phase payload while the outer daily result becomes OFE-keyed. | Keep N=1 represented by one OFE record whose aggregate summary exactly matches the prior surface. |
| Scheduler | Add `execute_ofe_sequence_with_kernel` or equivalent wrapper that iterates the existing phase graph over OFE-local lane surfaces. | Do not make OFEs `TopologyGraph` nodes in M-E; keep the topology change as a later watershed/internal-routing unification if needed. |
| Scheduler writeback | Replace the outer `HillslopeWritebackSurface` daily authority with `PerOfeDailyWaterBalanceCollection` for MOFE contexts. | The scalar surface may remain as an inner lane adapter during staged migration. |
| Runtime intake | Split static OFE input symbols into per-OFE lane slices and initialize per-OFE dynamic state. | Existing `ofeN_*` helpers should be reused for static symbols, but dynamic state must not be reconstructed from static rows. |
| Hydrology carry helpers | Move upstream/current carry arrays into explicit per-OFE transfer input/output records. | Existing M-B array guards become per-lane validators. |
| Runoff reconciliation | Consume `TransferInput` as same-day forcing and produce `TransferOutput` from current-lane runoff/lateral/saturation arrays. | Preserve stale aggregate carry purge and malformed-array fail-closed behavior. |
| Lateral drainage | Publish current-lane `ui_LfCrf[24]` as transfer output and daily `latqcc` as OFE-local WB term. | Do not collapse lateral transfer into aggregate `SubRIn`. |
| Runner daily loop | Replace one scheduler lifecycle per day with one per-OFE sequence per day. | Single-OFE path executes one sequence lane and should retain byte-identical anchors. |
| WB13/WAT publication | Build WB13 rows from `collection.records`, not from a single aggregate surface. | The current `single-row-canonicalized-hillslope-aggregate` policy remains current until M-E contract/test gates flip it. |
| Summary accumulator | Relax or scope the current `QOFE=Q` guard only under the new per-OFE policy; retain it for legacy aggregate policy. | This must be guarded by explicit publication policy, not implicit row count. |
| HBP/pass/loss outputs | Audit whether they consume aggregate water-balance fields and whether per-OFE rows change their schema. | M-E should preserve existing HBP bytes for N=1; multi-OFE schema changes need explicit gates. |
| Erosion/sediment qin/qout | Leave to M-G. M-E should expose the water-routing transfer state M-G needs, but not implement sediment routing. | Prevents M-E from mixing water-state architecture with erosion coupling. |

## Red Tests And M-E Breakdown

M-E should be split into implementation sub-increments with no required gate
deferred across a red boundary.

| Sub-increment | Scope | Required red tests/gates |
| --- | --- | --- |
| M-E0 contract/test scaffold | Amend `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-SYSTEM-001`; add contract tests for policy transition and identities. | Contract tests fail on current aggregate architecture; gates classify failures as `FAIL` or `BLOCKED`, not "later scope". No production code. |
| M-E1 data model shadow state | Add per-OFE collection and lane record types; build static per-OFE slices; preserve scalar inner lane adapter. | Unit tests prove `N=1` collection round-trips to old aggregate surface; `N>1` static slices have exact OFE cardinality and no duplicate ids. No WAT publication flip. |
| M-E2 sequential OFE lane executor | Add per-day OFE iteration around the existing phase graph with explicit transfer input/output objects. | Two-OFE synthetic vector proves OFE 2 receives non-zero `UpStrmQ`/`SubRIn` only from OFE 1 transfer arrays; malformed transfer arrays hard-fail. |
| M-E3 dynamic state persistence | Persist WB storage/frost/snow/profile state per OFE across days. | Two-day multi-OFE vector proves OFE state does not bleed across lanes and does persist across days; H1/H6/H9/H11 smoke runs execute. |
| M-E4 internal WB13 record production | Populate authoritative per-OFE daily WB records without publishing public WAT schema changes yet if necessary. | Per-element and transfer identities close on targeted two-OFE and five-OFE fixtures; aggregate identity cancels internal transfers. |
| M-E5 publication policy flip | Build WB13/WAT rows from per-OFE records and update provenance/summary guards. | H1-H36 multi-OFE row cardinality is `days * nofe`; downstream row handoff terms match upstream sent terms to declared tolerance; single-OFE anchors stay bit-identical or at-noise. |
| M-E6 closure loop | Full workspace and package acceptance after implementation. | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo deny check`, package docs lint, dual review, dual verification. |

Sizing:

- M-E0/M-E1 are small to medium contract/data-model increments.
- M-E2/M-E3 are large implementation increments and should be split further if
  per-lane storage touches broad process state.
- M-E4/M-E5 are large publication/identity increments and must not start until
  M-E2/M-E3 gates are green.
- M-E6 is verification-only unless it discovers a defect.

## M-D Gate Classification

| M-D criterion | Result | Evidence |
| --- | --- | --- |
| No production code edits | PASS | M-D edits are confined to package documentation/evidence. |
| Artifact identifies current aggregate seams with file:line citations | PASS | See "Current Boundary". |
| Artifact defines target per-OFE state shape and lifecycle | PASS | See "Target Per-OFE State Shape". |
| Artifact selects sequential execution model | PASS | Per-OFE lane iteration over the existing phase graph is selected; topology-N-node encoding is rejected for M-E. |
| Artifact maps legacy `irs`/`rochek` continuation | PASS | See "Sequential Execution Model" and "Legacy Obligations". |
| Artifact defines contract surfaces and measurable invariants | PASS | See "Contract Surface". |
| Artifact maps implementation seams and single-OFE preservation | PASS | See "Change Map". |
| Artifact defines red tests and M-E sub-increments | PASS | See "Red Tests And M-E Breakdown". |
| Rust validation gates | NOT RUN | M-D made no production Rust, science-contract, dependency, or test edits. |

## M-E Dispatch Rule

Do not start M-E production code until M-E0 has amended the contracts and
installed failing contract tests for the per-OFE state semantics above. Do not
publish per-OFE WAT rows by splitting, apportioning, or relabeling the current
aggregate WB13 row.

## Claude review addendum (2026-06-13) — endorsed, with one scope flag

Evidence mode: Ran (current-tree citation spot-checks) + Static.

**Endorsed.** The architectural call is correct and the artifact is
implementation-grade. Citations spot-checked accurate (no drift):
`scheduler.rs:700` request assembly, `:769` `apply_kernel_writeback`,
`core_types.rs:1018` `KernelWritebackPayload`, `:1384`
`HillslopeKernelRequest`, the runner loop seam, and `topology:19`
`TopologyNodeKind` (Hillslope/Channel/Impoundment — confirming OFEs are
*not* topology nodes). Three design choices are especially right:

- **Lane iteration over the existing phase graph, not topology-N-nodes** —
  minimal surface area, and N=1 executes exactly one lane, which is what
  protects the single-OFE bit-identical anchor.
- **Per-OFE records derived from OFE-local state, never from aggregate row
  splitting** — the anti-surrogate-physics discipline carried into the shape.
- **E0–E6 with no gate deferred across a red boundary** — textbook
  non-deferral; E0 lands failing contract tests before any code.

**Scope flag for the operator (not a defect — a magnitude the dispatch
sequencing should respect):** M-E3 ("persist WB storage/frost/snow/profile
state per OFE across days") is where MOFE stops being "routing" and becomes
**making the entire hillslope state model per-OFE**. Every stateful process
must become per-element: WB11 soil water, WB18/WB19 profile stores, the
snow pack, and the **FDHP01 frost state machine** (the fine-sublayer ice
state, `frwatc` handoffs, seasonal energetics — all of D3) now carry one
instance per OFE. Implications:

1. M-E3 is the largest increment in the package and should almost certainly
   sub-split per stateful family (soil water / frost / snow / plant-cover),
   each behind its own conservation + single-OFE-anchor gate — the artifact
   already licenses this ("split further if per-lane storage touches broad
   process state"); recommend making it mandatory, not optional.
2. The single-OFE anchor for E1–E4 should be **bit-identical, full stop**
   (not "bit-identical or at-noise") — N=1 changes no physics, so any drift
   is a wiring leak (the FDHP01 increment-A lesson). Reserve "at-noise" only
   for E5 where publication arithmetic legitimately reshuffles floats.
3. Frost-per-OFE is the interaction most worth an explicit early check: the
   FDHP01 closure was proven on single-OFE; M-E3 must show the frost state
   machine re-instances per OFE without perturbing the single-OFE frost
   anchor. Worth a named frost-specific fixture in E3.

None of this blocks M-E0 (contract/test scaffold), which is purely additive
and the correct next dispatch.
