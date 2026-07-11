# Review Agent A — Science, Numerics, and Conservation

Status: `EXECUTED-HOLD-RECOMMENDATION`

Evidence mode: `Static + Ran`

Reviewer role: independent WSHED-W11D science/numerics reviewer. This review
does not disposition its own findings.

## Recommendation

`HOLD` pending correction and re-review. The contract-first order was recorded,
the parser correction is sound, and the MC rejection guard is directionally
sound, but the claimed KW `sfnl` is not the pinned storage operand, the water
grid does not advance all `ntchr` intervals to the day-end boundary, and the
acceptance tests reconstruct the producer's closure rather than the
baseline-authoritative storage. A mandatory admissible production MC vector is
also absent. These are current-scope correctness and gate-legitimacy failures,
not follow-up suggestions.

## Authority and scope checked

Static:

- Applicable root, work-package, science-contract, crate, and test
  `AGENTS.md` instructions.
- Package `package.md`, active prompt, required-reading map, contract/test and
  implementation evidence, profile checklist, and current gate artifacts.
- Current diffs for `SC-ROUTE-001`, `SC-SYSTEM-001`,
  `SC-INFILE-CHANINP-001`, `hourly.rs`, `network_frame.rs`, `chaninp.rs`, and
  their tests.
- Pinned baseline SHA
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, especially
  `wshchr.for:95-121,125-193,197-331,388-469,473-615,618-655,686-721` and
  `wshdrv.for:614-674`.
- Local HEC-HMS Muskingum-Cunge technical reference and NEH 630 Chapter 17
  stability/accuracy passages.

## Findings

### A-H1 — KW final storage uses the MC boundary-mean formula, not pinned KW spatial storage

Severity: `High`

Static evidence:

- Pinned `wshchr.for:450-469` computes KW terminal storage area as
  `asum / (nseg + 1)`, where `asum` contains the Manning cross-sectional area
  at every terminal spatial node `is=0..nseg`.
- Pinned `wshchr.for:574-612` separately computes MC terminal area as
  `0.5 * (ain + aout)`. The two branches coincide only when `nseg=1` or the
  spatial area profile happens to be linear in exactly the required way.
- `SC-ROUTE-001.md:69,123,158` omits `wshchr.for:450-469` from the storage
  anchor and declares the inlet/outlet boundary mean for every wave route.
- `hourly.rs:706-740` implements only the inlet/outlet boundary mean and calls
  it for all wave branches at `hourly.rs:321-325`. The terminal spatial
  discharge vector is discarded at `hourly.rs:558-560`, so an authoritative
  KW spatial-area average cannot be reconstructed afterward.
- The W11C channel happens to route with one segment, so the current fixture
  aliases the correct KW spatial average to the rejected boundary-only
  candidate. This violates the anti-alias requirement in
  `docs/work-packages/AGENTS.md:93-111`.

Impact:

`sfnl`, carried next-day `sinit`, `chvol`, duration, and `chanwb` are wrong for
valid multi-segment KW routes even though their algebraic balance is zero. This
is a baseline-authority mismatch in the exact process family W11D claims to
close.

Required correction:

Amend `INV-ROUTE-021` and its algorithm/test vectors to distinguish fresh
boundary-mean initialization, KW terminal spatial-area averaging, and MC
terminal boundary averaging. Retain or return the KW terminal spatial state,
compute `sfnl` before it is discarded, and add an `nseg > 1` anti-alias vector
whose spatial mean differs numerically from the boundary mean.

### A-H2 — The nominal `ntchr` interval grid performs only `ntchr-1` routing updates and does not expose the pinned day-end state

Severity: `High`

Static evidence:

- The contract defines interval `i` as
  `[(i-1)*dtchr, i*dtchr)` and requires `it=1..ntchr`
  (`SC-ROUTE-001.md:673-678,698-712`).
- `ws11_project_hourly_totals` creates exactly `ntchr` interval values indexed
  `0..ntchr-1` (`hourly.rs:1523-1553`).
- The router stores the initial condition as `q1_m3_s[0]`
  (`hourly.rs:377,428-443`) and advances only
  `for interval in 1..qin_m3_s.len()` (`hourly.rs:445-560`). Thus an
  `ntchr`-slot interval forcing grid executes `ntchr-1` state transitions; its
  first slot is simultaneously treated as initial state and interval-1
  hydraulic output.
- Final storage is then computed from index `ntchr-1`
  (`hourly.rs:321-325`). In the pinned nodal route, `qin`, `qlat`, and `q1` are
  indexed `0..ntchr`, the recurrence advances `it=1..ntchr`, and final storage
  consumes `qin(ntchr)`/`q1(ntchr)` (`wshchr.for:257-331,397-449,511-615`).
- The cross-day test calls `.last()` the terminal outlet and carries it
  (`hourly_tests.rs:793-826`), but that assertion only locks the current index
  convention; it does not prove the state is at 24:00.

Ran corroboration:

`cargo nextest run -p openwepp-runner --test
mt3_hbp_hourly_consumer_contract wshedw11c_hourly_routing_sanity_matrix
--no-capture` passed, but its late-spike KW observations changed from
`peak=0.992440232277080 m3/s, storage=65.473952629946794 m3` at 3,600 s to
`peak=1.999993816881156 m3/s, storage=110.260168179987943 m3` at 600 s. The
test prints this material terminal-state divergence and still passes.

Impact:

The field named `final_storage_m3` is not proven to represent the day-end
boundary required by `sfnl`; the late forcing is sampled/averaged against a
missing terminal node. The resulting extensive `chvol` can close exactly while
being computed from the wrong state.

Required correction:

Make the nodal and interval roles explicit. Execute all `ntchr` interval
updates and retain the `ntchr` terminal node (or define and prove an equivalent
finite-volume mapping that produces a true 24:00 state). Reconcile the
same-grid sediment consumer deliberately rather than relabeling the initial
condition as interval 1, and add first-slot/last-slot anti-alias tests at more
than one timestep.

### A-H3 — Water acceptance is tautological and does not independently reconstruct `sinit`/`sfnl`

Severity: `High`

Static evidence:

- Contract vector 8 requires fresh KW early/late vectors to independently
  reconstruct `sinit`, `sfnl`, and `chvol`
  (`SC-ROUTE-001.md:440-443`). The package conservation rule forbids exact
  producer self-consistency as closure evidence
  (`docs/work-packages/AGENTS.md:93-111`).
- The release assertion derives "authorized initial storage" as
  `published_outlet + published_storage - external_input`
  (`mt3_hbp_hourly_consumer_contract.rs:445-472`). That is the inverse of the
  production formula and necessarily returns the producer's `sinit` when the
  same fields are used.
- The two-channel unit test sums the production
  `initial_storage_m3`/`final_storage_m3` fields and compares them with the
  production `channel_outflow_m3` (`hourly_tests.rs:928-957`).
- No test calls or independently reproduces the Manning depth/area operands,
  distinguishes KW spatial storage from MC boundary storage, or checks a
  multi-segment terminal profile. A source search found no test reference to
  `ws11_hydraulic_reach_storage_m3` or `ws11_close_daily_outlet_volume`.

Impact:

The tests would pass if `sfnl` were any finite nonnegative value, because
`chvol` is constructed from that same value. They therefore cannot close
W11D-ROUTE-STORAGE-001 or detect A-H1/A-H2.

Required correction:

Build expected storage from independent fixture geometry, terminal
discharges/spatial states, and an independently implemented Manning inversion;
then compare produced `sinit`, `sfnl`, and `chvol` separately. Include a
two-sided magnitude/ratio audit and explicit rejected candidates:
`sum(qin+qlat-q1)*dt`, boundary mean on multi-segment KW, last interval start,
and serial internal-throughflow sums.

### A-H4 — The mandatory admissible production MC vector is absent; all active hourly MC routes may reject vacuously

Severity: `High`

Static evidence:

- `SC-ROUTE-001.md:447-449` requires an admissible finer-grid MC vector that
  proves coefficient sum, monotonicity, passive maximum principle, and
  static/dynamic branch identity without repair.
- The whole-route test at `hourly_tests.rs:427-480` expects every active
  `ipeak=4,5,6` case to fail. The release matrix likewise expects all 16 active
  W11C MC cases to reject.
- The only admitted vector calls the shared segment helper directly with
  hand-supplied celerity/top-width/reference flow
  (`hourly_tests.rs:508-548`). It sets an `ipeak=4` context and never exercises
  `ws11_route_baseline_wave_series`, dynamic `ipeak=5` refresh, publication,
  storage, or static-vs-dynamic identity.

Ran evidence:

- `cargo nextest run -p openwepp-watershed-orchestrator wshedw11d
  --no-fail-fast`: 3/3 passed.
- `cargo nextest run -p openwepp-runner --test
  mt3_hbp_hourly_consumer_contract
  wshedw11d_release_cli_rejects_inadmissible_mc_grids --no-fail-fast`: 1/1
  passed.

Those passes prove the rejection path, not that a valid configured MC route can
execute.

Impact:

An implementation that rejects every nonzero MC configuration satisfies the
current suite. That does not close the package's static/dynamic MC production
claim or the contract's test obligation.

Required correction:

Add at least one active parser-to-production `ipeak=4` route and one active
`ipeak=5` route on an admissible finer grid (for example, investigate the
allowed 60 s grid), assert nonnegative coefficients/output and the passive
bound, and prove the dynamic coefficients/output differ from static for a
matched forcing where the refresh is active.

### A-H5 — Terminal-channel selection double-counts a channel/impoundment/channel path

Severity: `High`

Static evidence:

- `collect_dispatch_ids_from_steps` marks a channel internal only when it is a
  dependency of another **channel** (`network_frame.rs:1025-1056`). A channel
  consumed by an impoundment is deliberately retained.
- In a supported `channel -> impoundment -> downstream channel` topology (also
  the first WS11 system vector at `SC-SYSTEM-001.md:510-512`), both the channel
  above the impoundment and the channel below it therefore remain in
  `outlet_channel_ids`; `build_typed_publication_frame` sums both runoff
  volumes (`network_frame.rs:657-665`). They are serially related, not
  independent terminal yields.
- Contributor ancestry follows channel dependencies only
  (`network_frame.rs:803-832`), so a downstream channel after an impoundment
  also lacks the "complete topology contributor ancestry" asserted by
  `INV-SYSTEM-036`.
- The new selector test covers `channel 3 -> terminal impoundment`, but not an
  impoundment with a downstream channel (`network_frame.rs:1340-1384`).

Impact:

Event water can again be double-counted on a valid serial network, and terminal
sediment duration can omit upstream ancestry. This conflicts with the
topology-terminal/non-double-counting statement in `INV-SYSTEM-036`; the
contract's "terminal channel" caveat does not establish that serial channels
separated by an impoundment are independent extensive outlets.

Required correction:

Separate channel-oriented per-element diagnostics from watershed extensive
yield. For watershed event output, follow dependents through both channel and
impoundment nodes to actual network outlets (or use authoritative impoundment
outlet state); do not sum serial channel termini. Add the explicit
channel/impoundment/channel anti-alias vector and either carry complete
sediment ancestry through the impoundment or keep the unsupported sediment
claim fail-closed/out of scope.

### A-M1 — The amended `chan.inp` contract remains internally inconsistent

Severity: `Medium`

Static evidence:

- The applicability matrix still defines the accepted `ipeak>2` case as a
  "full 4-line canonical payload" (`SC-INFILE-CHANINP-001.md:32-40`).
- The amended normative grammar immediately below allows the canonical
  three-record `nchnum=0` form (`SC-INFILE-CHANINP-001.md:47-69`).
- Contract metadata remains dated `2026-07-09` despite the 2026-07-11 revision
  (`SC-INFILE-CHANINP-001.md:7-9`). `SC-ROUTE-001` and `SC-SYSTEM-001` likewise
  retain pre-amendment `last_reviewed` values in their front matter.

Impact:

Parser code and focused tests implement the detailed grammar correctly, but
the canonical source-model applicability table contradicts that behavior and
the lifecycle metadata does not identify the review date.

Required correction:

Split case B into positive-count four-record and zero-count three-record forms
(or describe both in one row), and refresh amendment metadata consistently.

## Confirmed strengths / no finding

Static:

- The recorded sequence contracts -> red contract tests -> pre-implementation
  gate -> production edits follows the required ordering. The scientific
  authority defect in A-H1 means the authority gate was not substantively
  valid, but the temporal sequence itself is documented.
- `ws11_close_daily_outlet_volume` uses the declared available-volume equation
  and rejects material negative outlet volume. Its exact-zero conversion is
  bounded by `TOL-ROUTE-009`; `ws11_grid_end_disposition` similarly rejects
  values below the absolute roundoff band. No new material negative-storage or
  peak clip was observed.
- Cross-day plumbing carries the previous same-channel `final_storage_m3` into
  the next dispatch and includes it in available-volume publication. This is
  structurally correct once the terminal storage operand itself is corrected.
- The MC segment guard checks finite coefficients, coefficient sum,
  coefficient nonnegativity, nonnegative sources/output, and the source-aware
  passive bound before publication. The W11C inadmissible grids preserve the
  typed `WKERNEL-WS10-CHANNEL-E-003` failure identity.
- The parser implementation correctly parses three fixed records first,
  requires record 4 only for positive normalized `nchnum`, and rejects an extra
  strict nonempty record for zero count.

## Commands run

Ran:

| Command | Result |
|---|---|
| `git diff --check` | PASS |
| `cargo nextest run -p openwepp-watershed-orchestrator wshedw11d --no-fail-fast` | PASS, 3/3 |
| Four focused W11B storage/carry/network tests | PASS, 4/4 |
| `cargo nextest run --test infile_chaninp_parser_contract wshedw11d --no-fail-fast` | PASS, 3/3 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract wshedw11d_creams_serial_publication --no-fail-fast` | PASS, 1/1 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract wshedw11d_release_cli_rejects_inadmissible_mc_grids --no-fail-fast` | PASS, 1/1 |
| `cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract wshedw11c_hourly_routing_sanity_matrix --no-capture` | PASS, 1/1; material KW terminal timestep deltas printed above |

The focused green runs establish that the implemented guards and asserted
self-consistency behave as coded. They do not override the pinned-source,
anti-tautology, missing-vector, or topology findings.

## Gate-legitimacy conclusion

Static + Ran:

The package may not be marked complete while A-H1..A-H5 remain open. In
particular, full workspace/release/comparator green status cannot substitute
for a baseline-authoritative KW storage operand, a true day-end grid state, an
independent conservation reconstruction, or a non-vacuous admissible MC route.
Under the package Gate Evidence Non-Deferral Rule, the current recommendation
is `HOLD` until corrections are implemented, findings are dispositioned by the
owning agent, and independent verification reruns the corrected vectors.
