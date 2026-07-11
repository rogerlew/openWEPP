# Independent Review B

Status: `PASS-WITH-FINDINGS`

Evidence class: `Static + Ran`

Reviewer B independently read the governing repository, work-package,
science-contract, crate, and test instructions and audited the current
working-tree change and package evidence. This review did not read or rely on
`review_agent_a.md`.

## Verdict

`PASS-WITH-FINDINGS`. The named mechanism is supported by the captured
step/ledger operands, and the production correction agrees with
`SC-OFEROUTE-001` rev 51: the downstream predictor face is rejected if
non-finite, bounded at exact zero before the existing available-water cap,
and reused by both the state update and booked ledger. No guard or closure
tolerance is weakened, no hybrid path is revived, and the accepted endpoint
manifests prove effective daily and hourly execution. Two medium evidence
findings and two low documentation findings must be dispositioned before
final package closure.

## Findings

### `RB-M1` — Medium — the focused vector aliases the rejected outlet-cell discharge

The focused regression makes the outlet cell exactly dry and assigns
`q[n-1] = 0` at
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:2145`
and
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:2159`.
For this state, the contract-correct bounded predictor face is also exactly
zero. Consequently, a wrong implementation that substitutes the committed
outlet-cell discharge for the predictor boundary face produces the same
predictor face on this vector. The assertions at
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:2173`
prove completion, nonnegative publication, bin/ledger equality, and
independently reconstructed storage closure, but they do not distinguish
those two face formulas.

This conflicts with the claim in
`artifacts/operand-lineage.md:38` that the regression distinguishes and
rejects committed outlet-cell `q[n-1]` as a stage-boundary substitute. The
captured H2637 values listed at `artifacts/operand-lineage.md:51` do separate
the values, but the executional endpoint has no assertion pinning that
separation.

Required disposition: revise the cheap contract vector so the outlet cell has
a small, finite, locally consistent positive depth/discharge while the
penultimate discharge remains greater than twice the outlet discharge. Assert
that raw extrapolation is negative, committed outlet discharge is positive,
and the traced scheme-actual predictor face is exact zero, while retaining the
pre-fix failure and independent closure assertions. This will make the
expected value differ from both rejected candidates as required by the
conservation/publication anti-alias rule.

### `RB-M2` — Medium — the consumer-path artifact is not source/call-site complete

`artifacts/fidelity-and-byte-identity.md:9` gives a correct high-level chain
and cites manifest counters, but it does not name the producer source,
in-memory frame/handoff objects, runner handoff, downstream consumer call
site, and output surface required by the package consumer-path closure rule.
Static inspection supports the claim, but that proof is not yet recorded in a
current package artifact.

Required disposition: amend the artifact with the concrete chain, including
the runner selection and `DirectRunFrame.laned_active` population in
`crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`,
`DirectFrameExecutor::run_laned_active_publication_stream`,
`laned_active_route_lane`, `route_single_ofe_with_step_trace`, the
`RoutingResult.outlet_bin_outflow_m2` to `UpstreamHandoff` and
`DirectLanedActiveDayRouting` handoffs, the
`DirectErosionHydrographShapeAuthority::RoutedHydrograph` consumer in
`direct_runtime/erosion.rs`, and the streaming/output surfaces. Retain the
manifest negative proof: `skeleton_runs = 0`,
`compatibility_edge_invocations = 0`, publication capture equals one, and an
active summary is present.

### `RB-L1` — Low — recorder comments retain the superseded production narrative

The `BinRecorder::finish` documentation at
`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:869`
still says a negative predictor boundary attribution can occur normally and
is redistributed forward. Rev 51 instead says a valid production run never
books that raw negative face or relies on later-bin borrowing
(`SC-OFEROUTE-001.md:229` and `SC-OFEROUTE-001.md:251`). The defensive code and
`NegativeOutletBin` guard should remain, but the comment should describe this
as a retained historical/defensive invariant path rather than current valid
scheme behavior.

### `RB-L2` — Low — work-package catalog status is stale

`docs/work-packages/README.md:32` still labels this package `SCAFFOLDED
(pending dispatch)`, while `package.md` truthfully reports `IN EXECUTION` and
substantial execution evidence is present. Update the catalog during finding
disposition/finalization so the repository index does not advertise a stale
queue state.

## Verified Surfaces

- Static: the captured terminal identity is internally consistent: negative
  predictor faces enter both the old state update and outflow ledger, creating
  the positive storage excess and terminal bin deficit surfaced by the
  unchanged recorder guard. Zero local source and finite nonnegative upstream
  handoff keep ownership inside the declared routing envelope.
- Static: the rev-51 code computes the raw face, explicitly fails
  `NonFiniteState` before `max(0)`, passes the bounded face through the rev-41
  limiter and state update, and books `0.5 * (pred + corr) * dt` from the same
  stage faces. Corrector and other boundary faces derive from already
  nonnegative forcing/discharge surfaces.
- Static: the diff changes no closure threshold, dry-depth tolerance, bin
  deficit threshold, selector, seam, snow/winter kernel, daily/off path, or
  hybrid symbol. `NegativeOutletBin` remains a typed terminal defense.
- Ran: focused tests
  `source_quiet_dry_front_outlet_flux_stays_nonnegative_and_conservative` and
  `bin_recorder_retains_material_terminal_deficit_signal` passed `2/2` under
  nextest run `d616043d-918b-4a82-9f9d-8449c808677f`.
- Ran: `cargo fmt --check` and `git diff --check` passed.
- Ran: the binding-exposure check returned its successful
  `PASS-DEFERRED` posture (`10` rows, `9` preexisting follow-ons), and the
  touched-contract unit-compliance check passed with no findings.
- Ran/inspected: both accepted H2637 manifests carry release binary SHA-256
  `a822036fd327c2f54d877ab51dc6c2e9aae13accff2ad4a61c154cbd730a131d`,
  direct-production selection, requested/effective mode pairs `0/0` and
  `1/1`, one publication capture, zero skeleton/compatibility-edge calls, and
  identical active closure operands. The rejected first true-mode candidate
  correctly is not used as acceptance evidence.
- Ran/inspected: the disabled-path post-run manifest names the post-fix binary
  and the hardcoded config-B output paths; its five output checksums match the
  preserved pre-fix copies exactly. Its active summary is absent as required.
- Ran/inspected: the selected-cohort summary records three successful real
  active `dx=5 m` members with expected mesh policy and numerical-scale closure
  residuals. The package records the required full workspace, clippy, deny,
  Case-4, and 19-OFE gates as passing; this reviewer did not rerun the already
  recorded full workspace suite.
- Static: `kinematic_wave.rs` is `2548` lines. The warning-level decomposition
  rationale and owned follow-on split intent are adequate; the file is below
  the `3000`-line closure blocker.

## Closure Posture

No `HOLD` is claimed or justified: the mechanism, authority, correction, and
validation surfaces are in-envelope. Final closure remains unavailable until
the findings above are dispositioned and the workflow's pending dual
verification, final document lint, disposition, and worker-handoff artifacts
are completed.
