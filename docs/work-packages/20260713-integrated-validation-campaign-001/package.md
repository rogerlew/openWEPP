# Integrated Production Validation Campaign

Status: `QUEUED-AUTHORING-COMPLETE`

Package ID: `20260713-integrated-validation-campaign-001`

Roadmap ID: `INTVAL-20260713`

Execution mode: `package-end-to-end`

Evidence mode: label every claim as **Static** or **Ran**.

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
Maintain `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes
& Retrospective` throughout execution.

## Purpose / Big Picture

The pre-integration CQR campaign ended with `GO-INTEGRATED-VALIDATION`. This
package converts that authorization into one fixed-source, end-to-end
validation campaign across current production hillslope, multi-OFE,
snow/frost, watershed-routing, publication, fail-closed, and release paths.

After completion, maintainers can point to one committed assessment showing
which production scenarios ran, which real consumers read their outputs,
whether water and sediment close under independent reconstruction, whether
serial and parallel publication agree, and whether invalid inputs fail closed.
The result is exactly `PASS-INTEGRATED-VALIDATION` or
`HOLD-INTEGRATED-VALIDATION`.

## Progress

- [x] (2026-07-13 UTC) Author the autonomous campaign package, kickoff prompt,
  required-reading map, artifact scaffold, roadmap entry, and catalog entry.
- [ ] Freeze a clean source commit and publish the executable scenario matrix.
- [ ] Execute hillslope hydrology, erosion/MOFE, and snow/frost lanes.
- [ ] Execute watershed routing/publication and fail-closed lanes.
- [ ] Execute release, required-authority, and full-workspace gates.
- [ ] Publish independent reconstruction, comparator-delta, and regression
  assessment artifacts.
- [ ] Complete dual review, finding disposition, and dual verification.
- [ ] Commit the exact PASS/HOLD disposition and update roadmap/catalog state.

## Surprises & Discoveries

- The roadmap contained the CQR authorization but no separately governed
  integrated-validation ExecPlan. This package fills that boundary; authoring
  it does not claim validation execution.

## Decision Log

- Decision: treat validation as a fixed-source evidence campaign, not an
  implementation package.
  Rationale: changing production code between lanes destroys same-source
  comparison. A semantic defect enters a separate defect-closure package.
  Date/Author: 2026-07-13 / Codex.
- Decision: use production consumers and independent output reconstruction as
  closure authority; comparator agreement remains a diagnostic flag.
  Rationale: producer-only tests and legacy similarity cannot establish
  conservation, publication, or consumer-path correctness.
  Date/Author: 2026-07-13 / Codex.
- Decision: run lanes once at the frozen source, then the expensive release
  and full closure gates once at the end.
  Rationale: this preserves coherent identity without repeating the untenable
  per-module heavy-gate cadence retired during CQR.
  Date/Author: 2026-07-13 / Codex.

## Outcomes & Retrospective

Authoring is complete; execution has not started. At terminal disposition,
record scenario counts, exact source and fixture hashes, conservation and
publication results, comparator flags, gate results, defect transitions, and
the exact PASS/HOLD recommendation.

## Context And Orientation

openWEPP is the Rust simulation engine. `openwepp-runner` owns production
hillslope and watershed commands. `openwepp-hillslope-orchestrator` owns the
array-native hillslope runtime, and `openwepp-watershed-orchestrator` owns typed
network routing. HBP, WAT, and Parquet surfaces count as validated only when the
real downstream runner or watershed consumer reads them.

The campaign entry point is commit `f699a217`, which closes the 45-module CQR
campaign with 65 of 67 raw identities removed, two exact observability
dispositions, zero actionable identity, and passing workspace gates. Execution
must begin there or at a documented descendant whose diff is classified before
any lane runs. All lanes use one frozen source commit.

Canonical science contracts govern correctness. The pinned baseline at
`/workdir/wepp-forest_260430_baseline`, commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, may expose deltas, but ADR-0017
forbids treating agreement as the correctness target.

## Scope And Write Set

Included scope is validation evidence and test-only characterization needed to
make an existing required scenario observable. The campaign covers native
H2637 hillslope hydrology/groundwater; p61 single-OFE and p102 multi-OFE
erosion; production snow/frost; sediment-active p102 watershed publication;
two-channel hourly water, particle-class sediment, and baseflow routing;
fail-closed public inputs; required authority suites; and release gates.

Excluded scope is new physics, calibration, threshold relaxation, fixture
result editing, schema redesign, production refactoring, wepppy, and
opportunistic defect correction. Test-only additions may not duplicate the
producer or replace the real consumer.

Normal execution may write only this package tree, `docs/work-packages/README.md`,
`docs/ROADMAP.md`, and narrowly scoped tests/fixture metadata when the scenario
matrix proves existing evidence cannot observe a required production path.
Before test edits, update `artifacts/intended-write-set.md` and run
`tools/agents/find-agents --for <path>`. Production Rust, contracts, schemas,
and fixture values are protected; a need to edit them triggers defect closure.

## Dependencies And Required Reading

Core readings are root `AGENTS.md`, `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`, this package, and the required-reading map.
Before lanes, read `docs/standards/local-ci-gate-selection.md`, the CQR final
assessment, `tools/release/README.md`, and ADR-0017.

Read `crates/AGENTS.md` and `tests/AGENTS.md` before touching those paths. Read
science-contract governance and the relevant contract if a failure requires
kernel/authority analysis. Read fixture provenance and prior packages only for
the active lane. The maintained tiers and byte budget are in
`artifacts/required-reading-map.md`.

## Evidence Model And Invariants

Every lane records frozen commit, command, exit code, elapsed time, maximum RSS
when practical, fixture/output hashes, selected test count, and log path. For
water, sediment, energy, or mass claims, record operand lineage with units,
normalization, area/volume basis, producer, consumer, and rejected aliases.
Independently reconstruct totals from produced outputs; do not restate the
producer helper.

Campaign invariants are:

1. All lanes use one frozen production commit.
2. Real command/runner consumers read every surface carrying a closure claim.
3. Serial/parallel comparisons cover schemas, row order, values, null posture,
   and metadata.
4. Conservation checks are two-sided and independently reconstructed.
5. Invalid authority fails closed without partial publication.
6. Comparator deltas are classified only after like-for-like units, timing,
   area basis, and lineage stage are proven.
7. A failure is not waived by rerunning until green; reproduction identity and
   ownership are recorded.

## Phase Plan

### Phase 0: Freeze Source And Manifest

Require a clean worktree; record HEAD and ancestry from `f699a217`; archive
status, tool versions, host summary, fixture checksums, and package inventory.
Populate `artifacts/scenario-matrix.md` with an exact command, fixture,
producer, consumer, expected evidence, and log path for every scenario. Use
only `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN`.

Run the anti-evasion intake gates:

    bash tools/release/check_authority_suite_antievasion.sh
    cargo nextest run --test auth11_required_suite_obligation_guards_contract

### Phase 1: Hillslope Hydrology And Groundwater

Run H2637 production closure and authority failures:

    cargo nextest run --test laned_shadow_h2637 h2637_native_active_owner_routes_and_closes
    cargo nextest run --test laned_shadow_h2637 h2637_active_fails_closed_without_routing_coefficients
    cargo nextest run --test laned_shadow_h2637 h2637_default_mixed_routing_coefficients_fails_closed
    cargo nextest run --test laned_shadow_h2637 h2637_default_malformed_routing_coefficients_fails_closed

Archive real outputs. Reconstruct precipitation, runoff, infiltration, storage
delta, lateral transfer, recharge, generated baseflow, and deep seepage. Prove
baseflow is external exactly once and does not reenter active surface routing.

### Phase 2: Erosion And Multi-OFE Continuity

Run:

    cargo nextest run --test erosion_single_ofe_p61_sediment
    cargo nextest run --test erosion_multi_ofe_p102_chain
    cargo nextest run --workspace --profile erosion

Require authoritative nonzero sediment. Independently reconstruct per-particle
and total continuity at OFE boundaries and public HBP/WAT surfaces. Prove p102
downstream consumption is routed state, not a diagnostic/scalar alias.

### Phase 3: Snow And Frost

Run:

    cargo nextest run --workspace --profile frost

The matrix must bind a real production path for snow accumulation/melt carry,
phase partition, frozen storage, thaw/release, and invalid-state rejection.
Diagnostic snowbench comparisons remain supporting evidence unless production
consumes the same path. Reconstruct SWE, liquid storage, and water transfers.

### Phase 4: Watershed Routing And Publication

Run:

    cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity
    cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract
    cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract
    cargo nextest run -p openwepp-watershed-orchestrator hourly_tests

The p102 case compares `--jobs 1` and parallel outputs. Two-channel evidence
must prove dependency order, same-grid consumption, shape sensitivity,
baseflow counted once, per-channel/class/network closure, and negative proof
that compatibility state does not carry the active-hourly claim.

### Phase 5: Fail-Closed And Release Lanes

Bind malformed/missing HBP, WAT, manifest, groundwater, hourly, channel, and
authority scenarios by running:

    cargo nextest run -p openwepp-runner
    cargo nextest run -p openwepp-watershed-orchestrator
    bash tools/release/run_release_candidate_gates.sh

Do not pass `--skip-stability` or `--skip-authority-required`. If required local
cohort inputs are unavailable, record exact missing paths/arguments and HOLD;
do not weaken the lane. Periodic/manual suites run only when required by a
current claim.

### Phase 6: Assessment, Review, And Verification

Publish `artifacts/integrated-validation-assessment.md` with the scenario
table, hashes, closure results, consumer proofs, failures, comparator flags,
regressions, and defect transitions. Run final gates once:

    cargo fmt --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo nextest run --workspace --profile full
    cargo deny check
    markdown-doc lint --path docs/work-packages/20260713-integrated-validation-campaign-001 --path docs/work-packages/README.md --path docs/ROADMAP.md
    git diff --check

Obtain two independent reviews, disposition every finding as `accepted`,
`rejected`, `deferred`, or `follow-up`, fix accepted findings, and obtain two
independent verifications. A current-scope deferred/follow-up finding forces
HOLD.

## Defect Transition And Recovery

When a scenario fails, rerun only enough to prove deterministic identity and
exclude harness/environment corruption. Diagnose to a named authority and
write-set boundary. A semantic defect or missing authority creates a separate
`YYYYMMDD-dc-intval-<slug>-001` package under
`docs/defect_closure_execplans.md`; its first action is `close defect <ID>`.
Do not edit production/contracts here.

Close this package `HOLD-INTEGRATED-VALIDATION` with the defect ID, reproducer,
affected scenarios, authority boundary, and restart condition. After closure,
restart the entire campaign from the new frozen source; never combine pre-fix
and post-fix lane results.

Commands are otherwise idempotent. Store outputs below a named scratch root
and archive only compact logs, manifests, hashes, and reconstruction evidence.
Do not commit binaries, secrets, or unbounded run directories.

## Exit Criteria

`PASS-INTEGRATED-VALIDATION` requires every matrix row PASS; all named domain,
publication, fail-closed, required-authority, stability, release, and final
gates PASS; independent closure and real-consumer proof for every conservation
claim; required serial/parallel identity; classified comparator deltas; no
unresolved defect, contract gap, regression, dirty overlap, or accepted-but-
unfixed finding; dual review and verification PASS; committed assessment; and
a clean worktree.

Otherwise the exact result is `HOLD-INTEGRATED-VALIDATION`, with a finite
defect/authority queue and named full-campaign restart condition. There is no
partial PASS.

## Subagent Authorization

Subagent requirement: **REQUIRED**. This package explicitly authorizes and
requires subagent spawning/delegation to `comparator_suite_runner` for release,
stability, authority, domain-profile, full-workspace, comparator, and
serial/parallel runs. Outputs are compact metrics, exits, timings, hashes, and
log paths; writes are limited to named package artifacts and scratch outputs.

This package explicitly authorizes two independent review agents, two
independent verification agents, and a read-only fixture/operand inventory
agent. Review/verifier writes are limited to named artifacts; inventory writes
are limited to `scenario-matrix.md` or `operand-lineage.md`. The parent must not
run heavy batches when the runner is available; unavailability requires
command-level evidence before local substitution.

## Security Impact Gate

Do not add network access, credentials, fallback dependencies, unsafe shell
interpolation, arbitrary output paths, or weaker validation. A security-
sensitive behavior change is outside scope and requires a separate package.

## Revision Note

2026-07-13: initial package authored after CQR terminal
`GO-INTEGRATED-VALIDATION`, with fixed-source validation, explicit defect
transition, real-consumer/independent-closure evidence, delegated heavy runs,
and exact terminal outcomes.
