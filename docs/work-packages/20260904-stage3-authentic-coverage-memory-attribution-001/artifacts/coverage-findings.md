# Authentic coverage findings

Status: `PHASE 2 REVIEWED — V31 RUNTIME DISCRIMINATOR CAPTURED; V61 OPEN`

Evidence mode: `Static + Ran`

## Direct answer to Q1

The authentic runner topology is **N=2 occupancies and S=6 soil/thermal
nodes**, not five. The real one-OFE release fixture invokes the production
runner, seeds a `CompleteOwner` from the persisted endpoint fixture, and the
live frame rebinding asserts six layers (`stage3_runner_qualification.rs:152`
and `snow_stage3_v11_production_seed.rs:1102-1123`). The preserved seed
`raw/authentic_owner_seed.json` has two vegetation occupancies, one hydrology
lane with six layers, one LSE OFE with six interface layers, and both
`beta_hyd` values at the exact upper bound 1.0; sun/shade/wet temperatures are
the exact 273.15 K lower bound while the dry-stem 273.15 K value is interior to
its 200–350 K domain (hex-float custody is retained in the raw JSON).

The revision-31 release assertion is an existential post-run check for one
completed sweep with `58/14/16/28`. For N=2/S=6 the unknown vector has
`10*N + 3 + S = 29` coordinates and 58 signed probes; the required partition
is 14 identity-anchor, 16 component-replay, and 28 complete probes. The
archived candidate log contains only the panic at
`component_replay_audit.rs:131`, before JSON emission: no sweep row, stencil,
eligibility, lifecycle, completion/error, or loss counter was serialized.

The strongest source-level limiting predicate is the recovered candidate's
`covered_finite_difference_stencil`/`covered_trial_is_valid` chain
(`/tmp/v31_recover/...solver_covered_solve.rs:863-900`,
`...solver_covered_evaluation.rs:1686-1725`). `Centered` requires both signed
trials to satisfy the closed beta and temperature domains. The authenticated
seed begins on those closed boundaries, so an inward one-sided stencil is
admissible while a centered stencil is not unless the runtime state moves off
the boundary before the first sweep. The candidate's reviewed release
assertion additionally required the completed target accounting. The exact
historical binary was executed directly: its focused real-runner
`component_replay_audit::real_one_ofe_replay_and_forced_complete_outputs_are_bit_identical`
test passed, while the authentic one-OFE release profile reproduced the
`component_replay_audit.rs:131` assertion panic before JSON emission. The
release failure was then instrumented only in a debugger at the audit-return
boundary, before `aggregate_component_replay_audit`. The raw capture
(`raw/historical_v31_gdb_take_return.log`) scans all 2,000 authentic N=2/S=6
sweeps: 200 rows (iteration 0 and 1 for the first 100 maps) are
`48/14/10/24`, and 1,800 rows are `54/14/16/24`
(600 iteration-0/1 rows for the other 300 maps plus 1,200 iteration-2/3/4
rows across all 400 maps). These are logical/identity/component/complete
counts, with zero rows at `58/14/16/28`. Every captured sweep is
`completed=true` and `failed=false`; the shortfall is therefore not collector
loss or lifecycle failure. The first captured stencil has ten inward one-sided
coordinates, and the iteration-2 stencil has four; each omits one signed probe
while preserving the canonical closed-domain guard. Those two vectors are
representative captures; the aggregate distribution is decoded from every
returned row.
This is the direct runtime discriminator for the release failure: the first
false target condition is the logical probe count, driven by boundary-aware
one-sided stencil selection, with the complete count short by the corresponding
four probes. The focused positive fixture remains separate. A lifecycle capture
also shows the authentic audit is taken once before the forced-complete oracle's
distinct second audit. No assertion bypass or binary mutation was used; the
debugger decoding is against the package-custodied nonproduction type layout.

The recorded automatic v31 recovery routes were attempted but did not establish
equivalence: the six-file diff conflicted when overlaid on exact HEAD, and the
a28 overlay failed compilation on missing post-a28 APIs. A later session-patch
replay was also attempted for the historical revision-61 source inventory; 22 of
38 source patch applications passed and 16 failed, and the resulting isolated
tree failed `cargo check` with missing request fields/API symbols. These are
reconstruction failures, not runtime predicate traces; no recovered candidate
was copied into or executed from the retained checkout.

## Direct answer to Q2

The retained authentic path is producer-to-consumer real execution:
terminal/provider -> strict V8 projection -> native V3 adoption -> canonical
covered solver -> transaction publication. The current release control
exercises one lane, two vegetation strata, six soil layers, one day, 48 total
parent supports (44 snow-free, 4 covered), 20 direct trials, 32 split-child
trials, 4 accepted microsteps, and 56 accepted publication supports. Across
all 12 admitted memory-control blocks, output/counter identity is exact and
`finalization_phase_wall_us.{candidate,identity_replay,install,sealed_source}`
is a separate carrier/finalization telemetry family, not a component-replay
counter; its zero values do not prove a replay count. Retained-source static
search finds no production component-replay declaration or consumer hook, so
replay is not applicable to the retained implementation and its runtime count
is not published. The opportunity is structurally present in the covered
endpoint, while the rejected v31 implementation is absent from the consumer.

Separately, the historical v31 binary's focused real-runner test observed at
least one completed component-replay probe and verified bit-identical
forced-complete output. This diagnostic fixture is positive coverage evidence,
but it does not establish the release workload's missing `58/14/16/28`
aggregate or expose its first failing event.

The two-OFE bootstrap fixture independently proves the lawful topology
`[(lane 1, 100 m², 6), (lane 2, 200 m², 6)]`, but it is not a replay-positive
production run and is not promoted as one. Eligible coordinates, historical
candidate declarations, and static graph nodes are not counted as executed
replay.

## Evidence matrix and limits

| Arm | Topology/stencil | Lifecycle/result | Replay observed | Reconstruction |
|---|---|---|---|---|
| Current retained one-OFE release (12 A/B blocks) | N=2/S=6; current production stencil not instrumented | exit 0; complete runner telemetry; exact closure | not applicable; implementation structurally absent and no replay counter is published | JSONL/CSV and raw logs reconcile independently |
| Historical v31 candidate release profile | N=2/S=6; boundary-aware one-sided stencils at closed beta/temperature bounds | direct binary exit 101 after audit-return capture; 2,000 rows decoded, all completed/not failed | 200 rows `48/14/10/24`, 1,800 rows `54/14/16/24`; zero target rows | runtime discriminator captured in GDB; no collector/lifecycle loss |
| Historical v31 focused real-runner diagnostic | lawful one-OFE focused fixture; component replay plus forced-complete oracle | direct binary exit 0; parity assertion passes | at least one completed replay probe | raw focused-run log; counts not serialized |
| Recovered v31 on exact HEAD | candidate diff conflicts in four tracked files | not built | none | equivalence route rejected; raw apply log |
| Recovered v31 on a28 base | missing post-a28 runner/orchestrator APIs at compile | check exit 101 | none | not equivalent to historical candidate; raw compiler log |

No producer-only or shadow-only evidence closes the replay claim. A future
successor would need a bounded test-only event audit at the real solver
consumer, exact signed-stencil/eligibility records, complete/failed lifecycle
records, independent offline joins, and three fresh release repeats before
any mechanism can be reconsidered.
