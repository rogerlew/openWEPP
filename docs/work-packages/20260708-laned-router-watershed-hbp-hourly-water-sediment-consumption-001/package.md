# Lane D Watershed HBP Hourly Water/Sediment Consumption

Status: `QUEUED`
Package ID:
`20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001`
Owner: Codex
Scaffold date: `2026-07-08`
Evidence mode: `Static scaffold; no implementation executed`
Execution order: after
`20260708-laned-router-tier1-local-numerics-001` completes and records its
final disposition, and after WSHED-W7 has resumed and closed the
sediment-active watershed fixture/publication path on current main.

## Objective

Close the watershed-facing consumer path for active Lane D HBP minor-1 hourly
surfaces. When Lane D active routing owns the hillslope surface-water path, the
watershed path must consume the active routed outlet hourly runoff volume and
hourly sediment mass, not only validate or inventory those arrays.

The package must prove, or implement and then prove, this end-to-end path:

1. The hillslope HBP producer serializes the active routed outlet hourly runoff
   volume and matching hourly exported sediment mass as the HBP minor-1 pair.
2. The watershed supervisor parses those arrays from the current HBP event and
   builds the typed watershed contribution from them.
3. The watershed/channel routing consumer uses the hourly water limb and hourly
   sediment timing in the production dispatch path when every eligible
   contributor carries the pair.
4. Changing the hourly distribution while holding daily runoff/sediment totals
   fixed changes the watershed/channel result on a named consumer surface.
5. Active-routed contributors without required hourly authority fail closed, and
   legacy/off contributors use only a contract-authorized fallback. Mixed
   active hourly authority must not silently collapse to a daily scalar path.

This is a consumer-path closure package. Producer-only, parser-only,
inventory-only, counter-only, or shadow-only evidence cannot close it.

## Rationale

Lane D active routing is now the conditional hillslope default for
coefficient-complete runs, and the active production mesh default is `dx5`.
That closes the hillslope active-owner path, but the next watershed-facing claim
is broader than "HBP outlet re-pointing". It must prove that watershed routing
reads the active routed hourly water and sediment timing through the real typed
frame and channel kernel, with no daily-lump compatibility path carrying the
closure claim.

The current code already has relevant surfaces:

- HBP minor-1 fields:
  `event.hourly_runoff_volume_m3[24]` and
  `event.hourly_sediment_mass_kg[24]`.
- Hillslope writer assembly in
  `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`.
- Watershed HBP intake validation in
  `crates/openwepp-runner/src/watershed_supervisor.rs`.
- Typed frame fields in
  `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`.
- Hourly limb scaffolding in
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`.

Those surfaces are not enough by themselves. This package must record the real
producer -> HBP -> supervisor -> `WatershedNetworkFrame` -> channel/watershed
consumer path and prove hourly distribution sensitivity in production routing.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/package.md`
- this package's `package.md`

Conditional:

- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/final-disposition.md`
  and `artifacts/worker-handoff.md` after Tier 1 completes.
- WSHED-W7 resume final disposition and worker handoff after that package
  closes. The canonical ordering source is `docs/ROADMAP.md`
  `## Watershed Runtime Performance Queue`.
- `docs/specifications/science-contract-authoring-procedure.md` and
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before any `SC-*` contract edit.
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
  before HBP schema, parser, writer, or run-level intake policy edits.
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` before
  watershed/channel routing, hourly-limb, or sediment time-base edits.
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` before
  active Lane D ownership, HBP producer authority, or active/fallback policy
  edits.
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` before
  sediment timing, sediment mass, erosion-shape, or exported-mass authority
  edits.
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` only
  if runoff-partition source-shape or upstream hourly runoff authority is
  changed.

Implementation-local:

- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits.
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/types.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `crates/openwepp-watershed-output/src/writers.rs` if watershed publication
  surfaces are changed.
- `tests/integration/laned_shadow_h2637.rs` and watershed integration tests
  only if active fixture generation or watershed CLI proof touches them.

## Scope

### Included

- Package-local scaffold, artifacts, prompts, and catalog/roadmap updates.
- Required-reading map with a tiered reading budget.
- Contract-first authority for active-routed HBP hourly producer/consumer
  behavior if current `SC-*` contracts do not already bind it.
- Source map and operand-lineage table for water and sediment hourly operands:
  units, source, area/volume basis, normalization, authoritative consumer, and
  rejected aliases.
- Production or fixture-backed proof that active Lane D HBP emits the 24-slot
  hourly runoff/sediment pair from the active routed path.
- Watershed intake proof that the pair populates `HillslopeContribution`.
- Watershed/channel consumer proof that the hourly water limb and hourly
  sediment timing are used by the production dispatch path.
- Distribution-sensitivity evidence: same daily runoff/sediment totals,
  different hourly distributions, different watershed/channel result on a
  named output or diagnostic consumer surface.
- Fail-closed policy and tests for active-routed contributors missing required
  hourly surfaces or carrying malformed/mixed hourly authority.
- Protected legacy/off behavior proof for runs without active routed
  coefficient authority.
- Review, verification, finding disposition, line-count governance, final
  disposition, and worker handoff.

### Excluded

- Tier 1 local numerics implementation or optimization adjudication.
- Mesh-policy changes, active default eligibility changes, or `dx5` policy
  revisions.
- Active-mode erosion water-magnitude coupling. That is a separate upstream
  hillslope sediment-process question.
- New sediment process physics, Wave-1/Wave-2 sediment production, or surrogate
  sediment fills. If nonzero sediment authority is absent, hold with evidence
  and route to the sediment-production package.
- wepppy orchestration, disturbed-management coefficient generation, or
  external fixture source edits.
- Silent fallback wrappers, compatibility bridges, skeleton paths, or shadow
  paths carrying the closure claim.
- Relaxing HBP, water, sediment, or route closure tolerances without a
  contract-first amendment.

## Dependencies

- Tier 1 local numerics must complete first so this package consumes the active
  router's post-Tier-1 production behavior and contract revision.
- WSHED-W7 resume must complete next so this package builds on a proven
  sediment-active watershed fixture/publication path. `WSHED-W7DC01` is
  historical/superseded unless a fresh producer-side zero-sediment regression
  reappears.
- Conditional Lane D default activation:
  `20260708-laned-router-conditional-default-activation-001`.
- Active `dx5` production mesh-policy ratification:
  `20260708-laned-router-dx5-production-mesh-policy-ratification-001`.
- Current HBP minor-1 authority in `SC-INFILE-HBP-001`.
- Current route/channel hourly-limb authority in `SC-ROUTE-001`.
- Current sediment exported-mass and hourly-shape authority in `SC-SED-001`.
- Existing watershed typed-frame path through `WatershedNetworkFrame`.

If Tier 1 exits in `EXECUTED-HOLD-*`, this package must start by deciding
whether the hold changes the active router behavior or contract basis. If it
does, this package must remain queued or be amended before execution.
If WSHED-W7 resume exits in `EXECUTED-HOLD-*`, this package must decide whether
the hold invalidates watershed sediment-active publication as a prerequisite. If
it does, this package remains queued or must be amended before execution.

## Intended Write Set

Package and catalog:

- `docs/work-packages/20260708-laned-router-watershed-hbp-hourly-water-sediment-consumption-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Contract authority, only after the package's contract-first phase:

- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md` only
  if runoff-partition source-shape authority is actually touched.

Primary implementation, if current production code does not already satisfy the
contract:

- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-runner/src/watershed_supervisor.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`
- `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/types.rs`
- `crates/openwepp-watershed-output/src/writers.rs` only if publication
  surfaces must expose the consumer proof.

Focused tests and fixtures:

- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- watershed supervisor/integration tests.
- Lane D/HBP integration tests only where needed to prove active HBP producer
  provenance.

Protected:

- No Tier 1 source edits in this package.
- No wepppy edits.
- No cohort fixture posture or required-case binding edits unless explicitly
  justified and followed by anti-evasion gates.

## Phase Plan

### Phase A - Intake, Ordering Check, and Source Map

1. Record `git status --short --branch` and identify unrelated dirty files.
2. Confirm Tier 1 has completed; if not complete, stop before implementation
   and keep this package queued.
3. Confirm WSHED-W7 resume has completed; if not complete, stop before
   implementation and keep this package queued.
4. Update `artifacts/required-reading-map.md` with current byte counts.
5. Produce `artifacts/source-map.md` naming the producer, HBP parser,
   supervisor intake, typed frame, route kernel, and publication/output
   consumers.
6. Produce `artifacts/operand-lineage.md` for hourly water and sediment
   operands and reject daily scalar aliases explicitly.

### Phase B - Contract-First Authority

1. Determine whether existing `SC-INFILE-HBP-001`, `SC-ROUTE-001`,
   `SC-OFEROUTE-001`, and `SC-SED-001` already authorize the required
   end-to-end consumer claim.
2. Amend contracts before tests or production code if any authority is missing:
   active routed HBP producer, run-level hourly-pair eligibility, watershed
   hourly-limb consumption, sediment time-base consumption, and active
   missing/mixed hourly-surface fail-closed policy.
3. Record `artifacts/contract-disposition.md` with amended or no-amendment
   rationale and BEI/profile obligations.

### Phase C - Contract-Derived Tests and Fixtures

1. Add tests that prove active-routed HBP serializes a valid 24-slot pair on
   the active path, not the old DC01 daily-lump path.
2. Add or update watershed intake tests proving malformed, missing, or mixed
   active hourly authority fails closed under the contract.
3. Add route-kernel tests for hourly water and hourly sediment distribution
   sensitivity with equal daily totals.
4. Add an end-to-end fixture or harness proving the production watershed path
   consumes the hourly pair through `WatershedNetworkFrame`.
5. Record pre-implementation failures or current-code sufficiency in
   `artifacts/pre-implementation-contract-gate.md`.

### Phase D - Implementation

1. Move any stale daily-scalar consumer to the hourly pair where the contract
   requires it.
2. Keep active-routed missing/malformed hourly authority fail-closed with typed
   errors.
3. Preserve legacy/off fallback behavior only where the contract explicitly
   allows it.
4. Avoid compatibility wrappers, synthetic hourly fills, surrogate sediment
   physics, or publication-only proof.

### Phase E - End-to-End Evidence

1. Build exact release binaries required for hillslope and watershed evidence
   and record path, mtime, size, and hash.
2. Run an active Lane D hillslope producer fixture and parse the HBP minor-1
   event fields.
3. Run watershed dispatch from that HBP and prove the hourly arrays populate
   the typed contribution and are consumed by the channel/watershed kernel.
4. Run the equal-total/different-hourly-distribution sensitivity proof and
   record the changed consumer surface.
5. Run protected legacy/off identity or no-change gates for no-coefficient
   fallback.
6. Record water/sediment closure checks for hourly sums and watershed totals.

### Phase F - Review, Verification, and Closure

1. Complete line-count governance and owned-file manifest.
2. Complete dual review and disposition accepted findings before final
   verification.
3. Complete dual verification, including one verifier focused on real consumer
   proof rather than producer/parser presence.
4. Run final gates and record `artifacts/gate-results.md`.
5. Write `artifacts/final-disposition.md` and `artifacts/worker-handoff.md`.

## Required Gates

Always record:

- `git status --short --branch`
- `git diff --check`
- Markdown/doc lint for touched package, contract, and catalog docs.
- Contract/profile/BEI checks required by touched `SC-*` contracts.
- Focused HBP parser/writer tests for the hourly pair.
- Focused watershed supervisor intake tests.
- Focused `openwepp-watershed-orchestrator` hourly water/sediment consumer
  tests.
- Active Lane D hillslope HBP run with complete coefficients and hourly pair
  evidence.
- Production watershed dispatch proof that the hourly pair reaches
  `HillslopeContribution`.
- Consumer-path proof that channel/watershed routing uses hourly water and
  sediment timing.
- Distribution-sensitivity proof with equal daily totals and different hourly
  distributions.
- Missing/malformed/mixed active hourly authority fail-closed proof.
- Protected legacy/off fallback proof for no-coefficient runs.
- Exact release binary provenance for every CLI binary used in evidence.
- `.rs` line-count disposition.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

If any required-case binding, cohort fixture, or external-authority suite
posture is touched, also run:

- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Subagent Authorization

This package explicitly authorizes spawning/delegating to review,
verification, comparator, watershed fixture, and release-gate subagents.
Expected outputs are package-local artifacts, compact metrics, and log paths.
Write access is bounded to package-local artifacts unless a subagent is
explicitly assigned implementation fixes within the declared write set.

Heavy comparator, watershed fixture, and final full-closure gates must be
delegated to a `comparator_suite_runner` subagent when available. If no such
subagent/tool is available in the execution environment, the parent executor
may run the gates directly and must record the unavailability and commands in
`artifacts/gate-results.md`.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/source-map.md`
- `artifacts/operand-lineage.md`
- `artifacts/contract-disposition.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/implementation.md`
- `artifacts/hbp-producer-evidence.md`
- `artifacts/watershed-consumer-proof.md`
- `artifacts/hourly-distribution-sensitivity.md`
- `artifacts/protected-fallback-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/hold-legitimacy-audit.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Exit Criteria

Close as `EXECUTED-COMPLETE-WATERSHED-HBP-HOURLY-CONSUMPTION` only if all of
the following are true:

- Tier 1 has completed or this package is explicitly amended to account for its
  hold/disposition.
- WSHED-W7 resume has completed or this package is explicitly amended to account
  for its hold/disposition.
- Required contract authority is current before production code.
- Active Lane D HBP producer evidence shows the 24-slot water/sediment pair is
  sourced from active routed outlet behavior.
- Watershed supervisor and `WatershedNetworkFrame` carry the pair.
- The production watershed/channel consumer uses hourly water and sediment
  timing; producer/parser/inventory-only evidence is not used for closure.
- Equal-total/different-hourly distribution evidence changes a named
  watershed/channel consumer surface.
- Active missing/malformed/mixed hourly authority fails closed.
- Legacy/off fallback behavior remains protected.
- Required gates, review, verification, and finding disposition are complete.

Legitimate hold outcomes include:

- `EXECUTED-HOLD-TIER1-DEPENDENCY`: Tier 1 has not completed or its disposition
  changes this package's authority basis.
- `EXECUTED-HOLD-WSHED-W7-DEPENDENCY`: WSHED-W7 resume has not completed or its
  disposition leaves sediment-active watershed publication unavailable.
- `EXECUTED-HOLD-CONTRACT-AUTHORITY`: current contracts do not authorize the
  consumer behavior and cannot be safely amended in this package.
- `EXECUTED-HOLD-HBP-PRODUCER`: active HBP hourly water/sediment producer
  evidence cannot be produced without a separate hillslope package.
- `EXECUTED-HOLD-WATERSHED-CONSUMER`: the real watershed/channel consumer
  cannot be moved in-envelope.
- `EXECUTED-HOLD-SEDIMENT-PRODUCTION`: no real nonzero sediment HBP fixture is
  available and creating one would require sediment process-physics work.
- `EXECUTED-HOLD-FIDELITY-OR-CLOSURE`: consumer movement violates water,
  sediment, or route closure gates after in-envelope fixes.
