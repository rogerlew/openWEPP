# Owned File Manifest

Status: `EXECUTED-CURRENT`

Evidence mode: `Static + Ran` workspace status/diff inventory.

Intake found the W11B scaffold untracked; `docs/ROADMAP.md`, the package catalog,
and the W11 handoff already contained user-directed W11B linkage. Numerous
unrelated untracked top-level `artifacts/` paths were present and remain
untouched.

## Rust production and tests

| File | Package action |
|---|---|
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | bind required `pw0.sol -> prtcmp -> crfrac` authority for multi-class channels |
| `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` | retain protected P102 routed-yield non-alias proof |
| `crates/openwepp-watershed-orchestrator/src/lib.rs` | export typed interval state |
| `.../src/lib_mod/mod.rs` | module export wiring |
| `.../src/lib_mod/kernel/kernel_core.rs` | include hourly owner |
| `.../src/lib_mod/kernel/constants.rs` | pinned dynamic-MC reference-discharge floor |
| `.../src/lib_mod/kernel/routing.rs` | routing type/function wiring |
| `.../src/lib_mod/kernel/types.rs` | interval operands, ledgers, WS20 result fields |
| `.../src/lib_mod/kernel/helpers.rs` | retain typed boundary symbol/value in guards |
| `.../src/lib_mod/kernel/diagnostics.rs` | dynamic-MC zero-inlet/positive-lateral reference floor |
| `.../src/lib_mod/kernel/direct.rs` | activate interval lane; preserve event lane |
| `.../src/lib_mod/kernel/hourly.rs` | new cohesive interval water/sediment owner |
| `.../src/lib_mod/network_frame.rs` | public typed states and terminal publication |
| `.../src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs` | correct both dcap terminals; explicit interval clock |
| `.../src/lib_mod/kernel/routing/02_ws20_segment_routing.rs` | explicit operands/clocks and detachment/deposition ledgers |
| `.../src/lib_mod/kernel/direct_tests.rs` | reconcile obsolete dcap expectations and constructive geometry proof |
| `.../src/lib_mod/kernel/hourly_tests.rs` | new eleven-vector/direct-consumer/branch tests |
| `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` | protected one-channel plus real two-channel CLI proof |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | explicit fixture `crfrac`, interval timing, superposition, and fail-closed proof |

All Rust files were clean at package intake. The intended optional standalone
`wshedw11b_channel_interval_sediment_contract.rs` and static fixture directory
were not needed: the package expressly allowed the existing protected-path
runner test, whose generator now builds the two-channel fixture dynamically and
executes the actual CLI binary.

## Package/governance files

| File/group | Pre-existing state | Package action |
|---|---|---|
| `docs/work-packages/20260710-wshedw11b-channel-interval-sediment-implementation-001/package.md` | untracked scaffold | execution progress, decisions, outcome |
| same package `artifacts/*.md` | untracked queued scaffold | replace placeholders with direct evidence, reviews, verification, gates, disposition, handoff |
| same package `prompts/README.md` | untracked scaffold | no execution change |
| `docs/work-packages/20260710-wshedw11-channel-network-hourly-water-sediment-routing-001/artifacts/worker-handoff.md` | modified W11B linkage | final successor disposition only |
| `docs/work-packages/README.md` | modified W11B queue entry | final package disposition only |
| `docs/ROADMAP.md` | modified W11B queue linkage | final package disposition only |

No science contract, lifecycle index, parser implementation, HBP format, Cargo
dependency, or unrelated artifact file is in the W11B write set.
