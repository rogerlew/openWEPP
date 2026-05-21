# 20260521-arch02-simulation-subsystem-kernel-architecture-discovery

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Run an architecture investigation to define the openWEPP simulation structure,
subsystem ownership model, and model-kernel placement strategy before Wave 4
ratification.

## Why This Package Exists
Wave 1-3 parser implementation established input-surface coverage and parser
contracts. Before ratifying Wave 4 execution, openWEPP needs an explicit,
evidence-backed simulation architecture baseline that triangulates:
- static ownership and requirement patterns in `wepp-forest`
- subsystem reference-linking patterns in `/workdir/rancor`
- Rust simulation architecture exemplars and best-practice patterns suitable for
  this domain

This package produces that baseline and converts it into actionable
architecture requirements for openWEPP.

## Scope
### Included
- Extract simulation subsystem boundaries, ownership patterns, and mutation
  surfaces from `wepp-forest` static analysis.
- Assess `/workdir/rancor` architecture patterns for subsystem linkage,
  simulation composition, and applicability constraints in Rust.
- Research Rust simulation model architecture exemplars and summarize practical
  patterns for openWEPP (state ownership, scheduler/orchestrator separation,
  kernel interfaces, and cross-subsystem referencing).
- Build a comparative matrix across the three evidence tracks.
- Produce a recommended openWEPP simulation/subsystem/kernel architecture
  structure with explicit ownership and dependency direction.
- Identify decision points and follow-on implementation package queue impacts.

### Explicitly Out of Scope
- Direct kernel implementation in `src/` or `crates/`.
- Ratification checklist closeout itself.
- Comparator acceptance decisions for any specific kernel.

## Deliverables
1. `artifacts/wepp-forest-subsystem-ownership-patterns.md`
2. `artifacts/wepp-forest-kernel-requirements-extract.md`
3. `artifacts/rancor-simulation-architecture-pattern-assessment.md`
4. `artifacts/rust-simulation-architecture-exemplar-survey.md`
5. `artifacts/cross-system-architecture-comparison-matrix.md`
6. `artifacts/openwepp-simulation-architecture-requirements.md`
7. `artifacts/openwepp-subsystem-and-kernel-ownership-proposal.md`
8. `docs/architecture/simulation-subsystem-kernel-architecture.md`
9. `artifacts/follow-on-architecture-implementation-wp-queue.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/arch02_disposition.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/home/workdir/openWEPP/docs/work-packages/20260520-arch01-subsystem-map-and-contract-spine/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/`
- `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/`
- `/home/workdir/openWEPP/references/50201000/`
- `/home/workdir/wepp-forest/` (static analysis only)
- `/workdir/rancor/` (architecture pattern reference)

## Phase Plan
### Phase 0 - Framing and Source Inventory
- Freeze investigation questions and architecture decision surface.
- Capture source inventory and evidence classes (`Static` vs `Ran`).

### Phase 1 - Legacy Pattern Extraction
- Extract subsystem decomposition and ownership patterns from `wepp-forest`.
- Catalog mutation boundaries and implicit orchestration responsibilities.
- Derive requirement candidates for typed ownership in openWEPP.

### Phase 2 - Cross-Architecture Investigation
- Assess `/workdir/rancor` simulation architecture for transferable patterns and
  incompatibilities in Rust.
- Survey Rust exemplars and best-practice patterns relevant to openWEPP.

### Phase 3 - Synthesis and Recommendation
- Produce comparative matrix with `[DIRECT]` and `[INFERENCE]` evidence tags.
- Publish openWEPP architecture requirements and ownership proposal.
- Publish canonical architecture draft in `docs/architecture/`.
- Produce follow-on work-package queue for implementation sequencing.

### Phase 4 - Review and Verification
- Run dual-agent review/disposition/verification gates.
- Resolve high-severity findings or leave explicit `HOLD` with risk rationale.

## Exit Criteria
- All deliverables exist and are internally consistent.
- Subsystem boundaries and ownership model are explicit enough to drive Wave 4
  planning.
- Rust applicability constraints are explicit for any adopted external pattern.
- No unresolved high-severity findings without explicit `HOLD` disposition.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: architecture/documentation package only.
