# MOFEFID-D11 Gap-007 Dynamic Friction Closure

Status: **EXECUTED-COMPLETE** (scaffolded and executed 2026-07-06). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract focus: `SC-OFEROUTE-001#GAP-OFEROUTE-007`.

## Objective

Resolve the remaining D11 hold for Lane D friction operands by ratifying and
wiring the dynamic/source-timed operands:

- skin rainfall intensity `I`,
- post-growth vegetation `LAI`,
- canopy height `h_c`.

The package must close `GAP-OFEROUTE-007` for the opt-in Lane D shadow
consumer, or record a legitimate hold with a named authority boundary. It must
not activate Lane D by default and must not claim D10 Case-4 shock acceptance.

## Rationale

D11 rev 20 closed the static native-management routing coefficient source for
`k_o`, form `C_d`, `D_r`, `lambda`, and vegetation `C_d`. The remaining hold is
not a need for new landuse constants; it is a runtime timing/source problem:
`I` is a day/hour rainfall driver, `LAI` is dynamic plant state, and `h_c` is a
plant canopy-height surface. Closing the hold requires contract-first binding
and proof that the real Lane D shadow consumes those live operands instead of
zero placeholders.

## Scope

### Included

- Amend `SC-OFEROUTE-001` to ratify the Lane D shadow source/timing for `I`,
  `LAI`, and `h_c`, if evidence supports it.
- Add a package-local source-authority map and pre-implementation contract gate.
- Wire the opt-in Lane D shadow to the live direct day-frame surfaces:
  - WB14 hourly rainfall depth converted by `/3600 s` for `I`,
  - post-growth direct runtime `leaf_area_index` for `LAI`,
  - typed management `canhgt` for `h_c`, required when `LAI > 0`.
- Preserve the current native routing-coefficient fail-closed requirement.
- Add focused tests proving the real consumer uses non-zero `I`/`LAI`/`h_c`
  when source-authorized and fails closed on invalid/missing vegetation height.
- Update D11/D12 handoff language and work-package index status.

### Excluded

- No production/default Lane D activation.
- No `OPENWEPP_LANED_SHADOW` default enablement or runtime policy promotion.
- No D10 shock-numerics or Iwagaki Case-4 acceptance.
- No D12 melt-limb implementation beyond using already produced runtime
  rainfall profiles.
- No D13 ADR-0036 erosion hourly-shape implementation.
- No D14 runtime profiling/optimization.
- No D15 default-promotion policy.
- No surrogate, provisional, proxy, tuned, or heuristic process physics.

## Dependencies

- `SC-OFEROUTE-001` rev 20.
- D11 package:
  `docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/`.
- Follow-on static coefficient commit: `f72e7749`.
- Current Lane D shadow:
  `crates/openwepp-runner/src/hillslope/laned_shadow.rs`.
- Direct runtime publication stream:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`.
- Direct day input/runtime authority:
  `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/`.

## Intended Write Set

Primary:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260706-mofefid-d11-gap007-dynamic-friction-closure-001/`
- `docs/work-packages/README.md`
- `crates/openwepp-runner/src/hillslope/laned_shadow.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`

Conditional:

- Focused runner/orchestrator tests under `crates/` or `tests/integration/`.
- `docs/specifications/science-contracts/index.md` only if lifecycle/status text
  requires an update.

Protected:

- Production/default runtime selectors.
- Public parquet/HBP/pass/watershed schemas.
- D10/D12/D13/D14/D15 package scopes.
- Raw copyrighted source additions.

## Phase Plan

1. **S0 - Intake and source map.** Record required reading, inspect D11 rev 20
   boundary, and classify `I`, `LAI`, and `h_c` sources and rejected aliases.
2. **S1 - Contract-first amendment.** Amend `SC-OFEROUTE-001` with exact
   source/timing/guard rules before code edits.
3. **S2 - Contract-derived tests.** Add focused tests for non-zero dynamic
   operand consumption and vegetation-height fail-closed behavior.
4. **S3 - Consumer implementation.** Wire the real Lane D shadow consumer to
   direct day-frame dynamic operands without changing public output schemas.
5. **S4 - Evidence and gates.** Run focused tests, Rust gates, doc gates, and
   line-count checks; record Static/Ran evidence.
6. **S5 - Review, verification, and disposition.** Complete review,
   verification, finding disposition, worker handoff, and final package status.

## Exit Criteria

- `GAP-OFEROUTE-007` is closed for the opt-in Lane D shadow consumer, or a
  legitimate remaining hold names a source-authority boundary.
- The shadow no longer uses an all-lane `I = 0` or all-lane
  `LAI = h_c = 0` placeholder when source-authorized live operands are present.
- `I` uses a named `/3600 s` conversion from the existing WB14 hourly rainfall
  profile, with non-finite/negative values hard-failed.
- `LAI` uses post-growth direct runtime plant state, with non-finite/negative
  values hard-failed.
- `h_c` uses parsed typed-management `canhgt`; missing or non-positive `h_c`
  hard-fails when `LAI > 0`, while physically absent canopy (`LAI = 0`) may
  route with zero vegetation resistance.
- Consumer-path proof names producer, frame state, runner handoff, shadow
  collector, and negative proof that placeholders are gone.
- No production/default activation, D10 Case-4 acceptance, D12, D13, D14, or
  D15 work occurs.
- Accepted review findings are fixed and verified before completion.

## Required Gates

- `git diff --check`
- Markdown lint for touched docs/package/index.
- Contract/profile/BEI checklist for changed `SC-OFEROUTE-001` surfaces.
- Focused tests for Lane D dynamic friction operands and fail-closed guards.
- Focused H2637/Lane D shadow gate when executable and still in scope.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

If a heavy gate is not run, record `NOT RUN` or `BLOCKED` with the reason.

## Conservation / Output Acceptance

This package changes diagnostic routing timing/friction inputs only and does
not add or change public output fields. It must still record friction operand
lineage, rejected aliases, and consumer-path proof. The daily protected output
schema and default runtime policy are protected.

## HOLD Legitimacy

A hold is valid only for a remaining authority boundary that cannot be closed
inside this write set. The hold audit must name the operand, evidence,
considered correction route, and why closure is not possible now.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `comparator_suite_runner` subagents for read-only review, verification,
source audit, fixture inspection, and heavy gate execution. Expected outputs
are compact findings, metrics, log paths, and package-local artifact text.
Write access is read-only unless the operator assigns a bounded write set.

Subagent requirement: REQUIRED for independent review/verification and full
workspace heavy gates when session-level tool policy permits spawning. If
session-level explicit user authorization is absent or a spawn is unavailable,
record the block and run equivalent local gates only where package governance
allows local substitution.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/source-authority-map.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/test-implementation-evidence.md`
- `artifacts/consumer-path-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/worker-handoff.md`
- `artifacts/final-disposition.md`
