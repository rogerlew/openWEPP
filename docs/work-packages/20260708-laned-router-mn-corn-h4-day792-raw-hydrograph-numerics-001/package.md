# LANED Router mn_corn_h4 Day-792 Raw-Hydrograph Numerics

Status: `EXECUTED-HOLD-CFL-TIMESTEP-TRANSITION`

## Objective

Close the `mn_corn_h4` day-792 lane-1 raw outlet-hydrograph nonconvergence
blocker before any renewed Tier-2 target-`dx` promotion. Either implement a
contract-authorized active-router numerics correction, or hold with a
mechanism-level blocker and first actionable follow-on.

## Rationale

`20260708-laned-router-mn-corn-h4-routed-shape-attribution-001` classified the
day-792 miss as solver/day class. The miss is not a metric-only problem:
absolute hourly mass movement is not noise-scale, hourly CDF distance worsens,
and raw outlet-bin / sampled-hydrograph comparisons worsen on the fine rung
pair. This package isolates the numerical mechanism.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/package.md`
- `docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/day792-attribution.md`
- `docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/solver-class-hold-audit.md`
- `docs/work-packages/20260708-laned-router-mn-corn-h4-routed-shape-attribution-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/package.md`
- `docs/work-packages/20260708-laned-router-wa-active-router-positivity-preserving-solver-correction-001/artifacts/final-disposition.md`

## Scope

Included:

- Scaffold package-local execution, evidence, review, verification, and
  disposition artifacts.
- Add or reuse opt-in diagnostic trace surfaces for one selected
  active-router lane-day.
- Re-run `mn_corn_h4` day 792 lane 1 at `dx2p5`, `dx1p25`, and `dx0p625`
  with exact release-binary provenance.
- Capture per-step mass, CFL, stage limiter, final TVD scaling, source,
  storage, boundary flux, and spatial-region evidence sufficient to identify
  the first divergent interval and mechanism.
- Compare against the attribution package's other high shape rows enough to
  distinguish a day-specific artifact from a general mesh-policy correction.
- Implement a production correction only when the mechanism is in-envelope and
  authorized by `SC-OFEROUTE-001`.
- If a correction lands, rerun the attribution/adequacy evidence required to
  prove the day-792 blocker is closed.
- If no safe correction can land, hold with a mechanism-level blocker,
  evidence, and first actionable follow-on.

Excluded:

- Production target-`dx` promotion or active mesh default flip.
- Widening routed-shape thresholds or changing the one-third adequacy rule.
- Hybrid solver revival.
- Cost optimization.
- WEPPpy, management, climate, or disturbed-producer changes.
- Broad D16/Tier-2 ratification beyond proving whether this blocker is closed.

## Correction Authority Envelope

Observed defect:

- `mn_corn_h4`, `sim_day_index=792`, `lane_index=1`.
- `dx1p25` vs `dx0p625` hourly-shape L1:
  `0.020944940478490041`, threshold `0.0166667`.
- Raw outlet-bin L1 and sampled hydrograph L1 worsen on the fine pair.

In-scope correction classes:

- TVD-MacCormack stage-face limiter bug fix consistent with
  `SC-OFEROUTE-001` rev 41.
- Boundary/outlet bin attribution correction consistent with Algorithm item 5
  and item 6.
- Forcing/source sampling correction consistent with the D10B exact
  source-history rule.
- Pure diagnostic trace plumbing that is opt-in and has no default/off
  behavior change.

Out-of-scope correction classes:

- Surrogate smoothing or empirical damping not authorized by the contract.
- Mesh-policy default changes.
- Tolerance fitting or threshold widening.
- Any fallback wrapper that silently hides invalid solver state.

Hold boundaries:

- The trace identifies a mechanism requiring new contract authority.
- The trace is insufficient to distinguish mechanisms without a larger
  dedicated numerical harness.
- A correction would need broader mesh-policy or tolerance adjudication.

## Intended Write Set

Expected:

- `docs/work-packages/20260708-laned-router-mn-corn-h4-day792-raw-hydrograph-numerics-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Conditional diagnostic/correction write set:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`

Conditional if correction authority requires contract text:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/index.md` only if registry metadata
  changes.

## Phase Plan

1. **MND-A Scaffold and authority map.** Create package files, prompts,
   ignored raw run root, and catalog pointers.
2. **MND-B Diagnostic trace.** Add or reuse an opt-in day/lane step trace for
   per-step mass, CFL, limiter, source, storage, and boundary-flux evidence.
3. **MND-C Rerun ladder.** Re-run `mn_corn_h4` rungs `dx2p5`, `dx1p25`, and
   `dx0p625` with exact release-binary provenance and the diagnostic trace
   enabled for day 792 lane 1.
4. **MND-D Mechanism attribution.** Identify the first divergent outlet
   interval and spatial region; compare with other high shape rows.
5. **MND-E Correction or mechanism hold.** Implement only a
   contract-authorized correction; otherwise write a hold legitimacy audit.
6. **MND-F Re-evidence.** If a correction lands, rerun the blocker evidence
   and focused tests.
7. **MND-G Review, verification, and disposition.** Complete dual review,
   disposition, dual verification, line-count governance, gates, final
   disposition, and handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to review, verification, comparator/timing, explorer, and bounded worker
subagents for mechanism review, evidence verification, and gate review.
Expected outputs are package-local review, verification, comparator/timing,
mechanism-attribution, and implementation-readiness artifacts. Write access is
read-only for review/verification/comparator/explorer roles; worker write
access is bounded to package artifacts unless the executing parent explicitly
assigns a disjoint implementation write set.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/fixture-plan.md`
- `artifacts/step-trace-design.md`
- `artifacts/raw-hydrograph-numerics-summary.md`
- `artifacts/raw-hydrograph-numerics-summary.json`
- `artifacts/mechanism-attribution.md`
- `artifacts/mechanism-attribution.json`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review-agent-a.md`
- `artifacts/review-agent-b.md`
- `artifacts/disposition.md`
- `artifacts/verification-agent-a.md`
- `artifacts/verification-agent-b.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Exact release-binary provenance for reruns.
- `mn_corn_h4` `dx2p5`, `dx1p25`, and `dx0p625` active step-trace reruns.
- Mechanism-attribution replay from committed tooling.
- Focused Lane D / `ofe_routing` tests for Rust changes.
- Contract/profile/BEI checks if `SC-OFEROUTE-001` changes.
- Focused contract-derived tests if contract text lands.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, cohort fixtures,
  or external-authority suite posture are touched.

## Exit Criteria

`EXECUTED-COMPLETE` requires:

- The raw-hydrograph nonconvergence mechanism is identified.
- Any correction is contract-authorized, covered by tests, and rerun evidence
  shows the day-792 blocker closed.
- No production mesh-policy flip or tolerance widening lands.
- Reviews, disposition, verification, gates, final disposition, and handoff
  are complete.

`EXECUTED-HOLD-*` is required when:

- The mechanism requires new contract authority or a broader numerical design
  package.
- Required evidence cannot be produced in-envelope.
- A safe correction cannot be distinguished from tolerance fitting or surrogate
  damping.
- Reviews or verification leave a blocker open.
