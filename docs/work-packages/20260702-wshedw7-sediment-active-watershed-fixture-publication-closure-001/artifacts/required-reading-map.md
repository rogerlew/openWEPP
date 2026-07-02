# Required Reading Map

Status: `queued`

Evidence mode: not-run

## Reading Budget

- local_required_bytes_total: `380992`
- threshold_outcome: `OK`
- measurement_method: `wc -c <core required-reading files>`
- measured_at_utc: `2026-07-02T17:07:59Z`

Thresholds use the canonical limits in
`docs/standards/kernel-work-package-preparation.md`.

## Map

| Path | Tier | Why required | Trigger / applicability | Read timing | Owner / maintainer | Last verified UTC | Notes |
|------|------|--------------|-------------------------|-------------|--------------------|-------------------|-------|
| `AGENTS.md` | Core | Root governance for all package work | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `crates/AGENTS.md` | Core | Rust crate governance for production/test edits | Always before crate edits | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `tests/AGENTS.md` | Core | Test and fixture governance | Always before test/fixture edits | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/codex_exec_plans.md` | Core | Execution-plan contract for package autonomy | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/work-packages/AGENTS.md` | Core | Work-package, gate, review, and conservation/publication rules | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/work-packages/README.md` | Core | Package index and process conventions | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/standards/prompt-wording-guidance.md` | Core | Prompt wording and subagent authorization requirements | Always for package execution prompt | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/standards/kernel-work-package-preparation.md` | Core | Kernel/publication package preparation requirements | Always for scaffolded package | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/specifications/science-contracts/AGENTS.md` | Core | Science-contract and no-surrogate-physics governance | Always for conservation/publication scope | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/architecture/watershed-runtime-architecture-specification.md` | Core | Target watershed architecture and benchmark policy | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/decisions/0032-watershed-runtime-ratification.md` | Core | Ratified public entrypoint, jobs default, and benchmark mode | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/package.md` | Core | W6 predecessor scope and closure basis | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/review-disposition.md` | Core | W6 residual sediment coverage risk | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/scaling-matrix-evidence.md` | Core | Current strict-fixture scaling baseline and null channel-balance context | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/publication-operand-lineage.md` | Core | Existing publication operand lineage and unavailable operand treatment | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `tests/fixtures/watershed/carnivorous-adobo/README.md` | Core | Existing committed development fixture provenance | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `tests/fixtures/watershed/onshore-xenophobia/README.md` | Core | Existing committed large fixture provenance | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/package.md` | Core | Package-local authority, write set, and gates | Always | Pre-edit | Agent | `2026-07-02T17:07:59Z` | |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract-authoring procedure authority | Contract or kernel authority edits | Pre-edit when triggered | Agent | `2026-07-02T17:07:59Z` | |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel-profile compliance authority | Contract or kernel authority edits | Pre-edit when triggered | Agent | `2026-07-02T17:07:59Z` | |
| `docs/specifications/science-contracts/index.md` | Conditional | Contract registry for changed authority | Contract edits | Pre-edit when triggered | Agent | `2026-07-02T17:07:59Z` | |
| `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` | Conditional | Pinned baseline provenance | Legacy migration/parity scope | Pre-edit when triggered | Agent | `2026-07-02T17:07:59Z` | |
| `docs/specifications/science-contracts/contracts/SC-SED-001.md` | On-demand | Sediment contract authority | Sediment semantics, guards, or physics touched | Phase-local | Agent | `2026-07-02T17:07:59Z` | |
| `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` | On-demand | Routing contract authority | Routing semantics, guards, or physics touched | Phase-local | Agent | `2026-07-02T17:07:59Z` | |
| `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` | On-demand | Public watershed consumer path | CLI/public path touched | Phase-local | Agent | `2026-07-02T17:07:59Z` | |
| `crates/openwepp-watershed-output/src/**` | On-demand | Typed publication output writer | Publication writer touched | Phase-local | Agent | `2026-07-02T17:07:59Z` | |
| `tests/fixtures/watershed/**` | On-demand | Committed watershed fixture corpus | Fixture inventory/adoption | Phase-local | Agent | `2026-07-02T17:07:59Z` | |

## Change Log

| UTC | Agent | Change |
|-----|-------|--------|
| `2026-07-02T17:07:59Z` | Codex | Initialized required-reading map from canonical template for WSHED-W7. |
