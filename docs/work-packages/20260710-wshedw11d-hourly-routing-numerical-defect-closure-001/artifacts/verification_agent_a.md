# Verification Agent A

Status: `EXECUTED-PASS-RECOMMENDATION`

Evidence mode: `Static + Ran`

Verified at UTC: `2026-07-11T06:10:28Z`

Role: same-agent independent reverification of Review A's science, numerical,
conservation, topology-publication, and parser-contract findings. This artifact
recommends a result; it does not set final package disposition.

Stabilized implementation/test/contract fingerprint:
`c7e0d2ab4b688356fe269acc279f3aa4cd0e62a03b494b3e8f890b43d7debbf6`
(`git diff -- crates tests` plus the three touched canonical contracts).

## Recommendation

`PASS` for Verification A's assigned scope. All five High findings and the one
Medium finding from `review_agent_a.md` are closed on the corrected tree. No
new High, Medium, or Low finding remains.

| Original finding | Result |
|---|---|
| `A-H1` KW terminal storage operand | closed |
| `A-H2` `ntchr` recurrence count and terminal state | closed |
| `A-H3` tautological water acceptance | closed |
| `A-H4` absent admitted production MC vector | closed |
| `A-H5` serial impoundment publication double count | closed |
| `A-M1` `chan.inp` contract inconsistency | closed |

## Review-disposition reverification

### A-H1 — KW terminal spatial storage

Static:

- `SC-ROUTE-001` v56, reviewed 2026-07-11, now defines branch-specific
  terminal storage in the symbol table and `INV-ROUTE-021`: KW averages the
  Manning area at every terminal `nseg + 1` spatial node, while MC retains the
  inlet/outlet boundary mean (`SC-ROUTE-001.md:123,158`). This matches pinned
  `wshchr.for:450-469` versus `574-612`.
- `ws11_route_baseline_wave_series` retains `previous_spatial` through the
  terminal update and sends it to `ws11_kinematic_terminal_storage_m3`
  (`hourly.rs:362-637,772-812`). The helper inverts Manning discharge at every
  node, averages the resulting areas, and multiplies by reach length. MC alone
  calls the boundary-mean helper at `hourly.rs:814-849`.
- `hourly_tests.rs:515-585` forces the pinned 101-segment cap, independently
  reconstructs all 102 rectangular Manning areas with the test-only bisection
  at lines 18-46, and differs from both the MC boundary mean and unrestricted
  flux residual by more than `1 m3`, with an additional ratio anti-alias.
- `INV-ROUTE-021` cites both the specialized storage anchor and the pinned
  whole-file `REF-ROUTE-WSHCHR-WAVE` anchor. Thus the KW `450-469` source site
  remains in the explicit authority chain even though the specialized row's
  compact line-range label emphasizes the initialization/MC/closure sites.

Ran:

- The final W11D orchestrator filter passed the independent 101-segment vector
  as part of 10/10 tests.

Conclusion: the production operand is the pinned KW spatial-area average, not
the former MC boundary alias. `A-H1` is closed.

### A-H2 — separate time-zero state and exactly `ntchr` updates

Static:

- `INV-ROUTE-021` now distinguishes the time-zero boundary from the
  `it=1..ntchr` terminals and requires one published routed terminal for every
  projected interval (`SC-ROUTE-001.md:158`; vector 11 at lines 461-463).
- Production builds a separate fresh/prior boundary state, leaves the public
  terminal vector empty, and executes `for interval in 0..qin_m3_s.len()`
  (`hourly.rs:397-485`). Every iteration appends one terminal; final storage
  consumes the post-loop spatial state or the last MC boundary
  (`hourly.rs:480-637`).
- The cross-day vector at `hourly_tests.rs:1047-1103` proves both sides of the
  boundary: day one's hour-23 pulse reaches `.last()`, and day two's changed
  first forcing produces a first terminal greater than the carried seed while
  preserving carried-storage closure.
- The additive direct vector at `hourly_tests.rs:646-695` isolates only
  `qlat[ntchr-1]` on both 3,600-second/24-slot and 600-second/144-slot grids. It
  requires all earlier `q1` terminals to remain zero, the last terminal to be
  positive, the vector length to equal `ntchr`, and final storage to respond.
  This fails the old seed/index alias and makes the multi-timestep last-slot
  obligation non-vacuous.

Ran:

- The final 10/10 W11D filter passed the first/last-terminal vectors.
- The W11C real-CLI sanity matrix passed 1/1. Its late-pulse observations were
  finite and positive at both timesteps (3,600-second peak/storage approximately
  `0.9924402323 m3/s` / `65.47395263 m3`; 600-second approximately
  `1.9999938169 m3/s` / `110.26016818 m3`). The direct vector above supplies
  the blocking terminal-index proof rather than relying on these broad sanity
  observations.

Conclusion: the route advances exactly `ntchr` intervals to a genuine terminal
state, including the final slot at both required timesteps. `A-H2` is closed.

### A-H3 — independent `sinit` / `sfnl` / `chvol` acceptance

Static:

- The 101-segment vector independently reconstructs multi-node `sfnl` and
  rejects the two plausible wrong storage candidates; it does not call the
  production depth solver or storage closer.
- The additional whole-channel vector at `hourly_tests.rs:589-643` separately
  reconstructs fresh `sinit`, terminal `sfnl`, `chvol`, public inflow, and
  public storage from fixture geometry, terminal discharge, external input,
  and the independent rectangular-Manning bisection. It also proves that the
  unrestricted interval-flux residual differs by more than `1 m3`.
- Production closes the extensive daily outlet as
  `volint + sinit - sfnl`, with typed rejection of material negative volume and
  only the contract-authorized roundoff-to-zero disposition
  (`hourly.rs:890-925`). A zero routed peak retains all available carried water
  as final storage before this close (`hourly.rs:84-121`).
- The zero-peak carried-storage vector at `hourly_tests.rs:1105-1140` asserts
  `7.5 m3` initial storage, `7.5 m3` final storage, and zero outlet rather than
  allowing stored water to become an unrepresented source.

Ran:

- The final 10/10 W11D filter passed both independent reconstructions, the
  flux/boundary anti-aliases, and the dry carried-storage disposition.

Conclusion: acceptance no longer derives expected storage solely by inverting
the producer's balance. `A-H3` is closed.

### A-H4 — non-vacuous admitted static and dynamic MC routes

Static:

- `INV-ROUTE-022` retains the finite, unit-sum, monotone-coefficient and passive
  maximum-principle requirements, typed `WKERNEL-WS10-CHANNEL-E-003` rejection,
  and prohibition on clamp, damping, peak clipping, or branch fallback.
- The production segment update checks coefficient sum and sign before use and
  checks the explicit-lateral-source passive bound before publication
  (`hourly.rs:974-1087`).
- `hourly_tests.rs:764-820` executes complete matched 60-second static
  (`ipeak=4`) and dynamically refreshed (`ipeak=5`) routes. Each publishes
  1,440 finite nonnegative terminals, convex coefficients, a passive peak, and
  a closed water ledger; the dynamic coefficients and hydrograph must both
  differ from static by more than `1e-9`.
- The real CLI vector at
  `mt3_hbp_hourly_consumer_contract.rs:299-321,693-750` reproduces the admitted
  geometry and executes both branches. The separate 3,600/600-second W11C
  matrix continues to require typed rejection for all active inadmissible
  grids (`mt3_hbp_hourly_consumer_contract.rs:270-296`).

Ran:

- The final W11D orchestrator filter passed the full-route static/dynamic
  vector, and the final runner W11D filter passed 4/4 including both admitted
  and rejected real-CLI paths.

Conclusion: the guard cannot pass by rejecting every active MC route, and the
dynamic branch is not a static alias. `A-H4` is closed.

### A-H5 — terminal publication through an intervening impoundment

Static:

- `SC-SYSTEM-001` v90, reviewed 2026-07-11, now states the supported rule
  precisely: dependency traversal crosses intervening impoundments for terminal
  water selection, while an impoundment is an explicit sediment-authority
  boundary (`INV-SYSTEM-036`, line 154; vector 11 at lines 538-541).
- `collect_dispatch_ids_from_steps` first identifies consumed impoundments and
  marks their upstream channel dependencies internal only when routing
  continues beyond that impoundment (`network_frame.rs:1025-1071`). A channel
  feeding a topology-terminal impoundment remains the channel-oriented proxy.
- `network_frame.rs:1403-1455` proves `channel 1 -> impoundment 9 -> channel 2`
  yields terminal set `{2}`, not `{1,2}`, and publishes only the downstream
  120 kg sediment mass rather than carrying/recounting the unsupported 240 kg
  pre-impoundment ancestry. The adjacent vector preserves independent terminal
  outlets and the terminal-impoundment proxy behavior.

Ran:

- Both selector vectors passed in the final 10/10 W11D filter; the explicit
  serial-impoundment selector also passed alone 1/1.

Conclusion: serial channel water is not double-counted and sediment claims now
match the implemented authority boundary. `A-H5` is closed.

### A-M1 — `chan.inp` matrix and lifecycle metadata

Static:

- `SC-INFILE-CHANINP-001` v0.1.4 now separates positive-count four-record case
  B1 from zero-count three-record case B2 and records the 2026-07-11 lifecycle
  update (`SC-INFILE-CHANINP-001.md:7-9,32-42`).
- The unchanged parser reads the three fixed records first, derives a
  three-versus-four record count from `nchnum`, produces an empty ID list for
  zero count, and retains strict trailing-record rejection
  (`chaninp.rs:455-546`).
- The parser contract tests distinguish strict and compatibility zero-count
  parses from defaulting and reject an extra nonempty fourth record
  (`infile_chaninp_parser_contract.rs:64-116`). The real CLI anti-alias at
  `mt3_hbp_hourly_consumer_contract.rs:89-166` requires the zero-count
  600-second result to match the positive-count 600-second control and differ
  from the 60-second default candidate.

Ran:

- The final focused parser filter passed 3/3 and the final runner W11D filter
  passed 4/4, including the real-consumer timestep anti-alias.

Conclusion: contract, parser behavior, and metadata now agree. `A-M1` is
closed.

## Protected p102 correction

Static:

- The stabilized test at `watershed_cli_behavior_contract.rs:329-409` again
  consumes the committed p102 fixture directly; no test-only staging rewrite
  remains.
- The committed wrapper changes only `pw0.chn`'s historical inadmissible
  `ipeak=4` selector to valid KW `ipeak=3`. The fixture README records the exact
  reason and unchanged hillslope/HBP substrate, and the manifest binds wrapper
  SHA-256
  `e6e9cacbb2ef769897aabbebe05ab7a9132474d652df273403aab8fb6b7397ed`.
- This does not bypass or relax the MC guard: admitted and typed-rejected MC
  production behavior remains independently exercised by the W11D suites.

Ran:

- `sha256sum -c input-manifest.sha256` passed all 18 committed fixture inputs.
- The isolated p102 workflow passed 1/1 in 30.480 seconds on the actual
  committed fixture, including `--jobs 1`/`--jobs 4` identity and downstream
  HBP/Parquet publication assertions.
- The required source-level authority anti-evasion script passed, and the
  AUTH11 required-suite obligation guard passed 2/2.

## Executed verification ledger

| Command | Result |
|---|---|
| `cargo nextest run -p openwepp-watershed-orchestrator wshedw11d --no-fail-fast` | PASS, final 10/10 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract wshedw11d --no-fail-fast` | PASS, final 4/4 |
| `cargo nextest run --test infile_chaninp_parser_contract wshedw11d --no-fail-fast` | PASS, final 3/3 |
| `cargo nextest run -p openwepp-watershed-orchestrator wshedw11d_terminal_selector_follows_serial_impoundment_path --no-fail-fast` | PASS, 1/1 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract wshedw11c_hourly_routing_sanity_matrix --no-fail-fast --no-capture` | PASS, 1/1 |
| `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity --no-fail-fast` | PASS, 1/1 |
| `sha256sum -c input-manifest.sha256` from the p102 fixture directory | PASS, 18/18 |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract --no-fail-fast` | PASS, 2/2 |
| `git diff --check` | PASS |

## Verification boundary and conclusion

This was a proportionate focused reverification of the accepted Review A
findings, not a substitute for the package's final workspace closure loop. At
inspection time, `gate-results.md` still recorded the earlier pre-stabilization
full-profile/release failures; the owning agent must replace that historical
disposition with one green full-profile result and reconcile the final gate
ledger on the stabilized source before marking the package complete.

Within Verification A's scope, canonical authority, production code, independent
operand reconstruction, real consumers, and focused executions agree. The KW
storage operand is baseline-authoritative; all `ntchr` terminals route; water
acceptance is non-tautological; both admitted and rejected MC paths exist;
terminal publication crosses impoundments without serial double counting; and
the three-record parser contract is internally consistent. Recommendation:
`PASS`.
