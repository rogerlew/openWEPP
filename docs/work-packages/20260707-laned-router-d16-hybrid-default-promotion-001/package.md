# LANED Router D16 Hybrid Default-Promotion Adjudication

Status: EXECUTED-HOLD-FIDELITY-TOLERANCE

## Objective

Adjudicate and, if supported by current-mesh evidence, promote the hybrid
implicit-explicit Lane-D stepper from experimental env opt-in to the default
stepper inside the already opt-in active Lane-D production path.

Promotion target:

- `OPENWEPP_LANED_ACTIVE=1` remains the active-owner selector.
- Inside that active path, the hybrid stepper becomes the default at the
  current 10-cell/OFE mesh.
- `OPENWEPP_LANED_ACTIVE_IMPLICIT=0` may remain as a diagnostic/plain-active
  opt-out for comparator and rollback evidence.
- No broader no-env Lane-D default activation is claimed in this package.

## Rationale

`GAP-OFEHYB-001` closed the Case-4 hybrid ladder blocker, and
`GAP-OFEHYB-002` closed the H2637 source-memory hybrid solve-cost blocker with
exact bare skin-only evaluation. The remaining decision named by the worker
handoffs is whether the current hybrid selector is mature enough to be the
default stepper for active Lane-D runs at the current mesh.

This package separates that solver-default decision from Tier-2 mesh policy and
from broader Lane-D production default activation. Mesh-resolution changes and
watershed/default-active routing surfaces remain follow-on policy work.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `docs/backlog/20260706-laned-router-numerics-performance-tiers.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/verification-h2637-timing.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/package.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/ratification-audit.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/timing-and-fidelity.md`
- `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-tier1-local-numerics-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-tier1-local-numerics-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-tier2-mesh-resolution-adjudication-001/package.md`

Implementation surfaces:

- `crates/openwepp-runner/src/hillslope/laned_active.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/`
- `tests/integration/laned_shadow_h2637.rs`

## Scope

Included:

- Contract-first adjudication of `SC-OFEROUTE-002#INV-OFEHYB-008` for the
  current 10-cell/OFE active Lane-D mesh.
- Fresh H2637 active plain-vs-hybrid/default timing, counters, closure, and
  output-delta evidence with exact release-binary provenance.
- Protected default/off byte identity proof if selector code changes.
- Production code change only if the current evidence supports promotion.
- Manifest/provenance semantics for the promoted default and diagnostic opt-out.
- Focused selector tests and Lane-D/ofe-routing gates.

Excluded:

- Tier-2 mesh-resolution changes.
- Generic non-bare implicit solve optimization.
- Broader no-env `OPENWEPP_LANED_ACTIVE` production default activation.
- Watershed HBP outlet re-pointing, active-mode erosion water-magnitude
  coupling, or inter-day routed-storage carry.
- Surrogate/provisional physics, compatibility wrappers, or silent fallback.

## Promotion Acceptance

Promotion is allowed only if every current-scope subgate passes:

- `GAP-OFEHYB-001` Case-4 full-hybrid ladder remains closed under
  `SC-OFEROUTE-001#INV-OFEROUTE-011` tolerances.
- `GAP-OFEHYB-002` H2637 solve-cost blocker remains closed on a current release
  binary.
- H2637 active hybrid output deltas are ratified as bounded numerical
  scheme-choice deltas under named tolerances in `SC-OFEROUTE-002`.
- The active day-closure hard-fails remain live under the promoted default.
- `OPENWEPP_LANED_ACTIVE=1` with no implicit env flag records
  `hybrid_implicit_stepping: true`.
- `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=0` remains a
  diagnostic plain-active comparator path and records
  `hybrid_implicit_stepping: false`.
- Subsystem-off/default behavior remains protected-output byte-identical.

If any promotion subgate cannot be proven, close as `EXECUTED-HOLD-*` with a
hold-legitimacy audit and no partial default flip.

Execution result: D16 held before implementation. The current-mesh hybrid is
faster and the Case-4 full-hybrid ladder passes, but H2637 active
plain-vs-hybrid publication deltas are material and no contract-authorized
default-promotion tolerance exists for accepting those deltas.

## Intended Write Set

- This package directory under
  `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/`.
- `docs/work-packages/README.md`.
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`.
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` only for
  parent pointer/status synchronization if promotion lands or is explicitly
  held.
- Selector/provenance code in `crates/openwepp-runner/src/hillslope/`.
- Local doc/comments in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`.
- Focused tests under existing Lane-D/ofe-routing test modules.

## Phase Plan

1. **D16-S0 Scaffold and reading.** Create package files, record authority
   map, and pin the promotion boundary.
2. **D16-S1 Baseline evidence.** Build the release runner binary and capture
   pre-change H2637 default/off, active plain, and explicit active hybrid
   evidence where needed.
3. **D16-S2 Promotion audit.** Evaluate `INV-OFEHYB-008` against the current
   Case-4, timing, counter, closure, and output-delta evidence.
4. **D16-S3 Contract amendment.** If promotion is justified, amend
   `SC-OFEROUTE-002` first, then synchronize `SC-OFEROUTE-001` pointer text.
   If not justified, write `hold-legitimacy-audit.md` and stop before code.
5. **D16-S4 Implementation.** If authorized by S3, make hybrid the active-path
   default, preserve explicit plain-active opt-out, and update manifest
   provenance.
6. **D16-S5 Verification.** Rebuild release runner and run H2637 default/off,
   active default-hybrid, explicit-hybrid, and opt-out plain-active proofs plus
   focused tests.
7. **D16-S6 Review and closure.** Complete dual review, disposition, dual
   verification, gates, line-count governance, final disposition, and handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to `rust_code_reviewer`, `rust_qa_reviewer`,
`verification_runner`, and `comparator_suite_runner` subagents for code review,
contract review, H2637 comparator/timing verification, gate verification, and
bounded codebase questions. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, and timing/comparator
artifacts. Write access is read-only unless a worker is explicitly assigned a
bounded implementation fix inside the intended write set.

Subagent requirement: REQUIRED for dual review and dual verification unless
the tool is unavailable; if unavailable, record the tool-policy block and run a
local equivalent gate with explicit evidence mode.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/promotion-readiness-audit.md`
- `artifacts/timing-and-fidelity.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- Contract/profile/BEI checks for touched `SC-*` contracts.
- Case-4 hybrid ladder evidence at current mesh.
- H2637 active endpoint/profile timing with exact binary provenance.
- H2637 active plain-vs-hybrid fidelity/delta audit with named tolerances.
- Protected-output byte identity with subsystem off.
- Active-mode closure evidence under promoted default.
- Active default/explicit/opt-out selector provenance proof.
- Focused Lane-D / `ofe_routing` tests.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- `.rs` line-count governance.

## Closure Outcomes

- `EXECUTED-COMPLETE-PROMOTED`: hybrid is active-path default at current mesh,
  all gates pass, broader no-env Lane-D activation remains explicitly separate.
- `EXECUTED-HOLD-*`: promotion blocker is proven and named; no code flip lands.
