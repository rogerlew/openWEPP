# WSHED-W11B Channel-Interval Sediment Implementation and Defect Closure

Status: `EXECUTED-COMPLETE`

Package ID: `20260710-wshedw11b-channel-interval-sediment-implementation-001`

Queue row: `WSHED-W11B`

Execution mode: `package-end-to-end`

Evidence mode: `Static + Ran` (label every artifact entry explicitly)

## Objective

Implement the `SC-ROUTE-001` v53 channel-interval sediment authority on the
real production watershed path and close defect `GAP-ROUTE-014` end-to-end.
After this package, a sediment-active HBP hourly input must route through at
least two dependency-ordered channels on the normalized `dtchr` water grid,
carry the authoritative geometry state, publish same-grid per-class egress for
the downstream channel, and independently close water and sediment balances.

This package is the bounded implementation successor to the held WSHED-W11
package and the completed WSHED-W11A authority package. It starts at the
contract-derived-test phase; it does not repeat the settled v51-v53 authority
acquisition.

## User-Visible Outcome

The production `openwepp-cli-watershed` path can consume paired HBP `V_h/S_h`
surfaces, activate the interval sediment lane under `INV-ROUTE-015`, route
water and per-class sediment through a two-channel network, and derive daily
public aggregates from the routed interval series. Equal daily totals with
different timing must produce a contract-consistent downstream response.

## Current Authority and Defects

- `SC-ROUTE-001` v53, `INV-ROUTE-015..020`, is canonical authority.
- The eleven W11A contract-derived vectors are current-scope implementation
  gates.
- `GAP-ROUTE-014` identifies two existing Rust terminal defects against pinned
  `dcap.for`: capped widening must derive geometry from capped erosion, and the
  post-contact/subcritical-boundary-shear terminal must re-enter incision with
  `t_exp = timpot` and decrement `depmid`.
- The existing WS20 flow partition uses event-peak fractions. The interval lane
  must instead use the v53 map: total `qlat(it)` only for the effective-length
  partition and derived `qlat_eff(it) := qe(it)/leff(it)` for the per-length
  solve slot.
- Existing M-T3 scalar summaries and the event-scalar sediment lane remain
  protected for non-activated configurations; they cannot carry the W11B claim.

## Correction Authority Envelope

### Defect IDs and observed violations

1. `GAP-ROUTE-014-A`: capped-widening Rust terminal returns uncapped width and
   unchanged depth after `dct` caps, contrary to `SC-ROUTE-001#INV-ROUTE-018`
   and pinned `dcap.for:238-261`.
2. `GAP-ROUTE-014-B`: post-contact subcritical-boundary-shear Rust terminal
   returns unchanged `depmid`, contrary to `INV-ROUTE-018` and pinned
   `dcap.for:210-215,173-190`.
3. `WSHED-W11B-DIRECT-001`: the production consumer lacks the complete active
   interval channel-sediment path required by `INV-ROUTE-015..020`; an eligible
   two-channel hourly network cannot yet produce and consume same-grid class
   egress end-to-end.

### In-scope authority and write set

- Canonical authority: `SC-ROUTE-001` v53; `SC-SYSTEM-001` may be amended only
  if the typed interval state/publication binding is not already explicit.
- Production owner: `openwepp-watershed-orchestrator` typed network frame and
  direct kernel routing modules.
- Real consumer: `openwepp-runner` watershed CLI test path and downstream
  channel dependency intake.
- Exact paths are listed under Intended Write Set. The envelope includes the
  likely correction surfaces and does not permit a wrapper-only closure.

### Allowed production edits

- Correct the two `GAP-ROUTE-014` terminal state transitions and their tests.
- Add typed interval water/sediment state and a cohesive `kernel/hourly.rs`
  owner or an equivalently focused module.
- Implement HBP-hour overlap projection, v53 hydraulic operand assembly,
  `t_exp/t_norm` routing, geometry carry, per-class interval continuity,
  downstream same-grid handoff, and daily aggregation.
- Narrow the M-T3 dependency guard only for complete `INV-ROUTE-015`
  activation; retain typed failures elsewhere.
- Add bounded output/publication fields only when necessary to prove the real
  consumer and independently reconstruct closure.

### Authorized evidence and tests

- Module-local contract-derived tests for all eleven W11A vectors.
- Focused orchestrator integration tests and one production CLI two-channel
  sediment-active fixture.
- Independent operand reconstruction, comparator-delta review, and exact
  binary provenance for release CLI evidence.

### Acceptance criteria

- Both `GAP-ROUTE-014` terminals match pinned baseline state and geometry-mass
  behavior, with the old locked-in expectations removed.
- All eleven W11A vectors pass, including vector 10(b)/(c) and vector 11's
  total-versus-per-length anti-alias fixture.
- A real two-channel CLI run proves the downstream channel consumes upstream
  interval class egress, not scalar/event compatibility state.
- Per-channel/per-interval/per-class and daily/network sediment closure pass
  independent reconstruction under `TOL-ROUTE-006..008`; water routing closure
  and storage posture match the named v53 operands.
- Protected non-activated/minor-0 behavior remains unchanged and malformed,
  mixed, impoundment, and unsupported states fail closed.

### Protected and branch-out boundaries

- No impoundment-hourly sediment routing; that requires distinct authority.
- No HBP schema change or per-hour enriched class payload; v53 binds the
  day-level class blend and `SC-SED-001#GAP-SED-008` limitation.
- No Lane D hillslope producer, mesh, groundwater, or route-coefficient edits.
- No wepppy orchestration changes.
- Missing or contradictory canonical physics is a hold-for-authority boundary;
  implementation size or diagnostic effort is not.

## Conversion Rule

If execution reproduces a root cause inside this envelope and expected behavior
is supported by `SC-ROUTE-001` v53 or pinned baseline provenance, it must proceed
through contract confirmation/amendment, contract-derived tests, the
pre-implementation gate, production correction, validation, dual review, and
disposition in this package. It may not close as `HOLD` while source reading,
implementation, or validation remains possible inside the envelope.

## Seven-Gate Bar

1. **Reproduction:** bind each defect to the existing Rust test/branch or a
   failing W11A vector.
2. **Mechanism:** identify the named state transition or missing direct-path
   handoff, not merely another symbol to inspect.
3. **Ownership:** keep the correction within the declared contract, kernel,
   frame, runner-test, and fixture paths.
4. **Authority:** cite `SC-ROUTE-001` v53 and pinned baseline SHA
   `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
5. **Safety:** preserve typed failures; do not clamp, default, canonicalize, or
   substitute surrogate physics.
6. **Testability:** make each correction fail before and pass after under a
   contract-derived anti-alias vector.
7. **Validation:** prove the real consumer, independent closure, magnitude, and
   protected-path behavior with current run evidence.

All seven gates already have named authority and test surfaces. The normal
execution outcome is correction, not hold.

## Included Scope

- W11A vectors 1-11 as executable tests.
- `GAP-ROUTE-014` correction.
- Typed interval grid, hydraulic operands, per-class sediment ingress/egress,
  geometry carry, closure ledgers, and downstream dependency handoff.
- Production CLI two-channel proof and same-total/different-shape sensitivity.
- Daily/public aggregate derivation from interval outputs.
- Protected fallback/fail-closed verification, reviews, verification, and full
  workspace closure gates.

## Excluded Scope

- New process authority, surrogate physics, impoundment routing, HBP schema
  expansion, enriched hourly composition, hillslope producer changes, wepppy,
  or unrelated refactors.
- Independent hourly event solves, event-peak fraction reuse on the active lane,
  raw-total `qlat` in the per-length slot, `qlat/lc` substitution, or scalar
  compatibility state carrying the direct-path claim.

## Dependencies and Required Reading

### Core

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- this `package.md`

### Conditional before kernel/test edits

- `docs/defect_closure_execplans.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/standards/local-ci-gate-selection.md`

### On-demand mechanism authority

- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- WSHED-W11A `artifacts/w11-handoff.md`, `codex_reconfirmation2.md`, and
  `codex-review-disposition.md`
- held WSHED-W11 intake, source map, and support matrix
- pinned baseline `src/{wshchr,chrqin,chnrt,dcap,detach,case12,case34}.for`
- `SC-SYSTEM-001`, `SC-SED-001`, ADR-0036, and unit governance only for the
  touched state/publication mechanism

Required-reading budget and instruction chains are recorded in
`artifacts/required-reading-map.md`.

## Intended Write Set

Package and queue:

- `docs/work-packages/20260710-wshedw11b-channel-interval-sediment-implementation-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- held WSHED-W11 `artifacts/worker-handoff.md` only for successor linkage

Canonical authority, only if execution proves a missing/contradictory binding:

- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md` only for lifecycle metadata

Production:

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` only to bind the
  already-required watershed `pw0.sol` `prtcmp -> crfrac` authority on the
  multi-class production path
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/kernel_core.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/types.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing.rs` for
  the canonical six-field geometry and segment-observation carriers
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` for
  typed boundary-symbol/value preservation
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/diagnostics.rs`
  only for the dynamic-MC zero-inlet/positive-lateral reference-flow floor
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs` for
  bounded integration and obsolete-guard removal only
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/mod.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`

Tests and bounded fixtures:

- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct_tests.rs`
- a focused module-local hourly-routing test file if needed for line governance
- `crates/openwepp-runner/tests/wshedw11b_channel_interval_sediment_contract.rs`
- `crates/openwepp-runner/tests/fixtures/wshedw11b/**`
- `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` only for
  protected-path regression coverage
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs` only to
  retain the protected P102 routed-yield non-alias assertion
- `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` only to bind
  explicit three-class `crfrac` authority in legacy typed-frame fixtures

## Required Deliverables

- all queued artifacts in `artifacts/README.md`;
- contract confirmation/implementation evidence and all eleven tests;
- pre-implementation contract gate before production edits;
- typed interval state and direct production consumer;
- operand lineage, anti-alias fixtures, consumer-path proof, independent
  conservation reconstruction, comparator disposition, and release-binary
  provenance;
- dual review, complete finding disposition, dual verification, line-count
  governance, gate results, final disposition, and defect-shaped handoff.

## Phase Plan

### Phase A — Intake and contract confirmation

1. Record workspace state and applicable instructions without touching unrelated
   changes.
2. Revalidate v53 authority, W11A handoff, held W11 evidence, current Rust
   branches, and the three defects in this envelope.
3. Complete operand lineage and the branch/topology support matrix.
4. Amend canonical authority first only if a real contradiction is found;
   otherwise record explicit no-amendment confirmation.

### Phase B — Contract-derived tests

1. Implement failing W11A vectors 1-11 before production edits.
2. Ensure vector 10(b)/(c) locks pinned terminal state and independently
   reconstructs geometry mass.
3. Ensure vector 11 separates event-peak, raw-total, `qlat/lc`, and authoritative
   `qlat_eff = qe/leff` profiles.
4. Add two-channel dependency and protected/fail-closed tests.
5. Complete `pre-implementation-contract-gate.md`; no production edit may
   precede this gate.

### Phase C — Production correction and direct path

1. Correct both `GAP-ROUTE-014` terminals and reconcile obsolete tests.
2. Add typed interval state and the cohesive hourly-routing owner.
3. Implement exact overlap projection, v53 hydraulic map, `t_exp/t_norm`,
   geometry carry, per-class closure, same-grid egress, and downstream intake.
4. Move daily/public consumers to aggregates derived from interval outputs and
   narrow the M-T3 guard only for fully authorized activation.

### Phase D — Consumer and conservation proof

1. Build the exact release runner binaries before CLI evidence.
2. Run the real two-channel sediment-active fixture through the production CLI.
3. Independently reconstruct water and per-class sediment closure using
   produced operands, not the producer formula.
4. Prove same-total/different-shape sensitivity and negative use of old scalar
   paths.
5. Prove protected non-activated/minor-0 identity and all required hard failures.

### Phase E — Review, verification, and disposition

1. Run focused gates and the required full closure loop.
2. Run dual independent scientific/Rust reviews; disposition every finding.
3. Fix accepted findings and complete dual verification.
4. Record line counts, binary provenance, comparator confidence tiers, gate
   legitimacy, security posture, final disposition, and handoff.

## Conservation and Output Acceptance

Before production edits, `operand-lineage.md` must record every interval water
and sediment field's units, grid, normalization, volume/area basis, class basis,
authority, and diagnostic/authoritative status. Tests must give distinct values
for raw-total `qlat`, `qlat/lc`, `qe/leff`, event-peak partition, daily mass,
active span, and compatibility-state candidates.

Acceptance requires independent reconstruction from produced output/state,
two-sided magnitude/ratio checks where physically bounded, protected output
anchors, and metadata/schema alignment. Exact self-consistency and one-sided
bounds are supporting evidence only.

## Validation Gates

Focused iteration:

- `cargo nextest run -p openwepp-watershed-orchestrator`
- `cargo nextest run --workspace --profile erosion`
- focused W11B runner integration test
- `git diff --check`

Release consumer evidence:

- `cargo build --release -p openwepp-runner --bins`
- record binary path, mtime/size or hash, fixture, command, and actual output
  directories before accepting evidence

Final closure:

1. `cargo fmt --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo nextest run --workspace --profile full`
4. `cargo deny check`
5. scoped Markdown lint
6. touched contract/consumer/conservation/anti-alias gates
7. legacy comparator delta review by confidence tier

The heavy closure, comparator, and release-CLI runs must be delegated to the
authorized comparator-suite-runner subagent when available.

## HOLD Legitimacy

`HOLD` is exceptional and valid only at a declared boundary: missing or
contradictory authority, proven out-of-envelope mechanism, invalid upstream
input with correct typed guard, unavailable required evidence, or an excluded
process family. A hold must update `hold-legitimacy-audit.md` with the boundary,
proof, considered in-envelope correction route, and why it cannot close now.
Effort, code size, uncertainty, partially passing tests, or a working wrapper
are not legitimate hold reasons.

The handoff's first actionable item must be `close defect <id>`; it may not be a
next inspection step.

## Review and Verification Requirements

Both reviewers must check science/contract fidelity, the conversion rule,
envelope adequacy, hold legitimacy, direct consumer closure, anti-tautology,
typed failures, protected boundaries, gate legitimacy, and `.rs` line-count
governance. Findings require `accepted`, `rejected`, `deferred`, or `follow-up`
disposition with rationale. No finding may remain undispositioned.

Verification A rechecks accepted-finding closure and focused/full gates.
Verification B independently reconstructs the consumer/conservation evidence,
negative old-path proof, comparator posture, and final package eligibility.

## Exit Criteria

`EXECUTED-COMPLETE` requires every acceptance criterion, all eleven vectors,
real two-channel consumer proof, independent water/sediment closure, protected
path gates, dual review/disposition/verification, line-count governance, and the
full closure loop to pass with current direct evidence.

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` requires continued execution or
a legitimate `EXECUTED-HOLD-*`; it cannot be deferred while marking this package
complete.

## Security Impact

Expected impact: none beyond existing local HBP/runfile parsing and bounded test
fixtures. Preserve path validation, length/CRC checks, allocation bounds,
serialization safeguards, typed numeric errors, and fail-closed behavior.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation to
source-lineage reviewers, scientific/Rust reviewers, verification agents, and a
comparator-suite-runner for bounded read-only source review, package-local
review/verification artifacts, and heavy closure/comparator/CLI runs. Expected
outputs are compact findings, metrics, commands, and log paths. Write access is
read-only except for explicitly assigned package review/verification artifacts.

## Progress

- [x] (2026-07-10) Scaffold package, artifacts, queue entry, and kickoff prompt.
- [x] (2026-07-10) Executed Phase A intake; confirmed v53 without amendment,
  mapped baseline/current mechanisms, and completed operand lineage.
- [x] (2026-07-10) Encoded all eleven Phase B vectors before production;
  focused nextest failed RED on the planned missing interval owner; recorded
  the pre-implementation gate as PASS.
- [x] (2026-07-10) Executed Phase C production correction and direct path;
  first dual review identified and the implementation corrected distinct
  water operands, local baseflow, gross detachment, six-field geometry carry,
  tillage authority, partial-state guards, finite domains, and outlet metadata.
- [x] (2026-07-10) Executed Phase D real release consumer and independent
  water/sediment conservation proof.
- [x] (2026-07-10) Executed Phase E dual review/verification, current-tree full
  closure gates, comparator disposition, and final package disposition.

## Surprises and Discoveries

- The held W11 support matrix's `ipeak > 5` fail-closed row is stale. Canonical
  v53 and pinned `wshchr.for` authorize `ipeak >= 4` as Muskingum-Cunge; only
  `ipeak = 5` selects dynamic coefficient refresh.
- The current scalar publication assigns `sediment_yield_kg` from `qsed_kg_s`.
  The interval lane must derive the daily mass publication from interval egress
  sums while leaving the non-activated compatibility behavior protected.
- Initial review proved the first water implementation passed whole-reach
  lateral flow twice to the inherited wave helpers. It also proved that a net
  continuity residual cannot be relabeled as gross detachment/deposition and
  that `wida/widb` cannot stand in for canonical `wera/werb`. All three were
  corrected before release evidence or package disposition.

## Decision Log

- 2026-07-10: Scoped WSHED-W11B as the executable implementation/DC successor
  to held WSHED-W11 and completed WSHED-W11A. Included `GAP-ROUTE-014` terminal
  correction in the same envelope as interval-lane adoption because both share
  the same contract, Rust owner, and validation surface.
- 2026-07-10: Expanded the production write set to the three adjacent kernel
  owner files listed above after review localized necessary fixes there. This
  remains the same W11B mechanism and objective: `routing.rs` owns internal
  state types, `helpers.rs` owns typed guards, and `diagnostics.rs` owns the
  inherited dynamic-MC helper. No new subsystem or science family was added.
- 2026-07-10: Expanded the bounded runner write set after verification proved
  the multi-class production frame discarded its already-required `pw0.sol`.
  Pinned `convrt.for:84-88` maps channel-indexed `prtcmp` fractions to
  `crfrac`; the CLI now uses the existing Rust `prtcmp` port for that same
  authority. No HBP schema, runfile schema, or transported-sediment fallback
  was added.
- 2026-07-10: Replaced the active-lane water gain surrogate with pinned
  segmented `wshchr` KW/MC equations, including `mofapp=1`, branch-specific
  `qref`, signed interior MC state, and outlet-only epsilon normalization.

## Outcomes and Retrospective

Executed complete. The production CLI now routes paired HBP hourly water and
sediment through serial channels on the pinned segmented wave grid, hands
same-index class egress downstream, and publishes terminal daily aggregates.
Both `GAP-ROUTE-014` terminals are corrected. Verification additionally caught
and closed the old water gain surrogate, MC reference-flow/epsilon/gate drift,
the discarded watershed-soil `crfrac` authority, and a zero-source class mass
creation edge. Release spike/spread runs independently close `7,200 m3` water
with nonnegative storage and distinct peaks; sediment closes at 240 kg. Full
workspace, erosion, deny, clippy, release, comparator, and documentation gates
pass on the final tree.
