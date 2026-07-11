# Contract-Test Implementation Evidence

Status: `PASS (RED PHASE + REVIEW-CORRECTION RED PHASE)`

Evidence mode: `Static + Ran`

Recorded at UTC: `2026-07-11T05:00:48Z`

Contract-derived tests were added before production edits:

| Contract surface | Test vector | Anti-alias assertion | Pre-correction result |
|---|---|---|---|
| `SC-INFILE-CHANINP-001#G-CHN-003` | strict/compat three-record `nchnum=0`, plus strict extra record | requested 600/3600 s remains parsed, never 60 s default | expected failure: strict `CHN-E-002`; compat `DefaultedCompat` |
| `SC-ROUTE-001#INV-ROUTE-021` | negative grid-end storage plus W11C KW early pulse | typed `-E-003`; terminal volume + hydraulic storage = external input | expected failure: negative accepted; release KW storage negative |
| `SC-ROUTE-001#INV-ROUTE-022` | synthetic convex/non-convex MC recurrences and W11C MC route | reject `c3=-0.142857...`; admit `(0.2,0.6,0.2)` and preserve max bound | expected failure: unstable recurrence returned `q1=1.428571...` |
| `SC-SYSTEM-001#INV-SYSTEM-036` | two-channel serial CREAMS release-CLI vector | channel 2, 7200 m3, terminal mass rather than serial sum/rate | expected failure: public element ID was 1 |

Ran commands and observed evidence:

1. `cargo nextest run --test infile_chaninp_parser_contract wshedw11d --no-fail-fast`
   - `3 tests run: 1 passed, 2 failed` as expected.
   - Strict failure: `ChnE002 { expected: 4, found: 3 }`.
   - Compatibility failure: `DefaultedCompat != ParsedBranch`.
2. `cargo nextest run -p openwepp-watershed-orchestrator wshedw11d --no-fail-fast`
   - `2 tests run: 0 passed, 2 failed` as expected.
   - Synthetic recurrence exposed `[c1,c2,c3] =
     [0.428571...,0.714285...,-0.142857...]` and unguarded
     `q1=1.428571...`.
   - W11C MC whole-route vector published instead of rejecting.
3. `cargo nextest run -p openwepp-watershed-orchestrator
   wshedw11b_vector06_water_storage_has_zero_sediment_storage`
   - `1 failed` as expected; `water_storage_m3=-0.25` was accepted.
4. `cargo nextest run -p openwepp-runner --test
   mt3_hbp_hourly_consumer_contract
   wshedw11d_cli_accepts_three_record_zero_count_chaninp_without_defaulting`
   - `1 failed` as expected; compatibility parser returned `DefaultedCompat`.
5. `cargo nextest run -p openwepp-runner --test
   mt3_hbp_hourly_consumer_contract
   wshedw11d_creams_serial_publication_uses_terminal_extensive_outputs`
   - `1 failed` as expected; public element was 1 instead of terminal 2.
6. `cargo nextest run -p openwepp-runner --test
   mt3_hbp_hourly_consumer_contract wshedw11c_hourly_routing_sanity_matrix
   --no-capture`
   - `1 failed` as expected at the KW hydraulic nonnegative-storage assertion.

All failures directly exercise predeclared contract behavior. No compile-only,
skeleton, producer-only, or shadow-path evidence is used.

## Review-correction red vectors

Review A identified additional pinned-authority aliases after the initial
implementation. `SC-ROUTE-001` v56 and `SC-SYSTEM-001` v90 were amended before
the corresponding correction edits. The following new tests then failed on
the reviewed implementation as required:

| Vector | Pre-correction observed failure | Corrected anti-alias |
|---|---|---|
| Separate time-zero state / all `ntchr` updates | next-day first `q1` equaled the prior seed exactly (`0.1 m3/s`) despite different current forcing | first published terminal is the result of update `it=1`, not the seed |
| Pinned no-peak disposition | carried `7.5 m3` published as outlet volume with zero peak | `chvol=0`, `sfnl=7.5 m3` |
| Channel/impoundment/channel terminal | selector returned channels `{1,2}` | only downstream terminal channel `{2}`; upstream sediment excluded |

Ran:

- `cargo test -p openwepp-watershed-orchestrator
  wshedw11d_kinematic_wave_advances_first_interval_from_prior_q1 -- --nocapture`
  failed at `first interval must advance from the prior seed`.
- `cargo test -p openwepp-watershed-orchestrator
  wshedw11d_zero_peak_retains_available_carried_storage -- --nocapture`
  failed with observed outlet `7.5`, expected `0`.
- `cargo test -p openwepp-watershed-orchestrator
  wshedw11d_terminal_selector_follows_serial_impoundment_path -- --nocapture`
  failed with selector `{1,2}`, expected `{2}`.

The green suite additionally adds a 101-segment KW reconstruction using an
independent rectangular-Manning bisection and matched admitted static/dynamic
MC full-route vectors. Those vectors close the review's non-vacuity and
anti-tautology findings. A final-slot-only KW vector at both 3,600 and 600
seconds requires every earlier terminal to remain zero and terminal `ntchr` to
respond, completing the two-timestep grid-cardinality obligation. These green
vectors are not substituted for the red evidence above.
