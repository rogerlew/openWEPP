# Implementation and Test Evidence

Status: `EXECUTED-PASS-FOCUSED`

Evidence mode: `Static + Ran`

## Before/after contract evidence

The pre-implementation red phase is recorded in
`contract-test-implementation-evidence.md`. It exposed all four mechanisms:
three-record zero-count input defaulted, negative storage was accepted, a
non-convex MC recurrence executed, and serial CREAMS publication selected
channel 1. The same named tests pass after the production correction.

## Production implementation

Static:

- `chaninp.rs` parses records 1-3 before deciding record closure. Parsed
  `nchnum=0` closes with three nonempty records and an empty ID list; positive
  counts still require record 4, and strict extra/missing records retain
  `CHN-E-002`.
- `hourly.rs` keeps the time-zero boundary state separate and executes exactly
  `ntchr` updates for `it=1..ntchr`. Fresh initialization uses the pinned
  boundary steady state; later days seed from prior terminal `q1/qin/qlat`.
- Explicit `initial_storage_m3`/`final_storage_m3` are branch-specific. Fresh
  `sinit` and MC `sfnl` use inlet/outlet Manning-area means; KW `sfnl` uses the
  mean across every terminal spatial node. A zero-peak day retains all
  available storage. Daily `volint` is local interval volume plus dependency
  daily outlet volume, and `chvol = volint + sinit - sfnl`; the unrestricted
  interval residual is diagnostic only.
- Cross-day routing carries prior `sfnl -> sinit`. The four-term channel
  balance receives available volume `volint+sinit` and final hydraulic
  storage, equivalent to the pinned storage-change balance.
- Static and dynamic MC updates validate finite coefficient publication,
  `c1+c2+c3=1`, coefficient monotonicity, nonnegative sources/output, and the
  explicit-lateral-source passive maximum bound. Inadmissible W11C grids fail
  with `WKERNEL-WS10-CHANNEL-E-003`; coefficients and peaks are not repaired.
- A matched 60-second full production route admits both static and dynamic MC,
  proves nonnegative unit-sum coefficients, the passive peak bound, and
  coefficient/hydrograph divergence from dynamic refresh. Thus rejection is
  not vacuous.
- `network_frame.rs` selects terminal channels consistently on interval and
  event lanes, following serial paths through consumed impoundments. A channel
  feeding a topology-terminal impoundment remains the channel-oriented proxy;
  in `channel -> impoundment -> channel`, only the downstream channel
  publishes. Serial/internal channel water and sediment remain diagnostics.
- Direct event sediment is published as mass. The terminal rate is integrated
  over the active hourly sediment span superposed across its complete
  channel-dependency contributor ancestry; when hourly timing is absent, the
  exact direct-event duration is reconstructed from `dtchr`, direct
  contributors, and dependency durations. Impoundments explicitly terminate
  this sediment ancestry. This prevents rate-as-mass aliasing, downstream
  water-duration re-scaling, and pre-/post-impoundment double counting.

## Real consumer-path closure

| Stage | Current path |
|---|---|
| Producer source | HBP hourly `V_h/S_h`, parsed `chan.inp`, and upstream `RoutedChannelState::channel_outflow_m3` |
| In-memory state | `RoutedChannelIntervalWaterState::{initial_storage_m3,final_storage_m3}` and terminal `RoutedChannelState` |
| Runner handoff | `execute_watershed_dispatch_with_frame` records routed state in `WatershedNetworkFrame` |
| Consumer call | `publish_typed_routing_report -> build_typed_publication_frame -> terminal_sediment_yield_kg` |
| Output surface | real `openwepp-cli-watershed` writes `ebe_pw0.parquet` and `chanwb.parquet`; tests read both with the parquet reader |
| Negative old-path check | multi-segment independent reconstruction rejects boundary-mean and flux-residual storage; terminal tests reject serial channel and channel/impoundment/channel sums; zero-count CLI differs from the 60-second default candidate; parser no longer has a pre-parse four-line guard |

The release fixture numerically distinguishes the rejected aliases: 7,200 m3
terminal water vs the old 14,400 m3 serial sum, channel ID 2 vs old ID 1, and
240 kg terminal mass vs a raw rate or summed internal rates.

## Ran focused gates

| Command | Result |
|---|---|
| `cargo nextest run --test infile_chaninp_parser_contract --no-fail-fast` | PASS, 20/20 |
| `cargo nextest run -p openwepp-watershed-orchestrator` | PASS, 113/113 on final stabilized tree |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract` | PASS, 7/7, including parser-to-real-CLI admitted static/dynamic MC routes |
| `cargo nextest run --test wshedw5_typed_watershed_runtime_contract --no-fail-fast` | PASS, 18/18 |
| `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity` | PASS, 1/1 on the committed wrapper corrected to valid KW interval routing; protected HBP/publication/jobs-identity purpose retained without weakening MC rejection |
| 101-segment independent KW Manning reconstruction | PASS; all-node spatial mean matches within `1e-8 m3` and differs materially from boundary mean and flux residual |
| independent fresh daily operand reconstruction | PASS; separately reconstructs `sinit`, `sfnl`, `chvol`, public inflow, and storage from geometry/terminal discharge using the independent Manning inversion; rejects the flux residual |
| first-/last-terminal grid anti-alias | PASS; cross-day first terminal advances from its separate seed, and final-slot-only KW pulses reach terminal `ntchr` with every earlier terminal zero at both 3,600 and 600 seconds |
| admitted matched static/dynamic MC full route | PASS at `dtchr=60 s`; both publish 1,440 finite slots, convex coefficients and passive peaks; dynamic coefficient/hydrograph deltas are nonzero |
| W11C KW/CREAMS zero, uniform, early/late, spike/spread matrix | PASS, 15 routed cases with nonnegative hydraulic storage and terminal mass/identity closure |
| W11C static/variable MC 3600/600 release-path matrix | PASS: four zero controls; all 16 active cases typed-rejected before publication |
| focused three-crate `cargo clippy --all-targets -- -D warnings` | PASS |
| `git diff --check` | PASS |
| strict Binding Exposure checker, `SC-ROUTE-001` | PASS, 8/8 rows consolidated |
| strict Binding Exposure checker, `SC-SYSTEM-001` / `SC-INFILE-CHANINP-001` | unchanged baseline limitations: SYSTEM reports the same four deleted-historical row taxonomy issues at `HEAD`; CHANINP has no BEI at both `HEAD` and this tree. No W11D binding row fails. |

The package-authorized heavy worker owns final release-binary provenance,
workspace full, deny, and complete clippy results in `gate-results.md`.
