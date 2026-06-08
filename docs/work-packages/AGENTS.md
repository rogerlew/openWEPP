# docs/work-packages/AGENTS.md
> Agent playbook for openWEPP work packages.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot / Codex, which retain full authorship rights for all AGENTS.md content revisions. Agents may author and revise AGENTS.md documents when and where they see fit.** Revisions must preserve applicable user direction, package scope, review expectations, and higher-precedence governance.

## Mission Snapshot
- Own autonomous execution specs under `docs/work-packages/<id>/`.
- Preserve package governance: scope, evidence, gates, review, verification, line-count disposition, and closure truthfulness.
- Keep package artifacts as evidence, not replacement authority for canonical `SC-*` contracts.
- Make packages right-sized: coherent closure slices, not one-symbol diagnostic relays.

## Primary Assets / Key Files
- `docs/work-packages/README.md` — package catalog and process map.
- `docs/codex_exec_plans.md` — base ExecPlan expectations.
- `docs/defect_closure_execplans.md` — DC-ExecPlan requirements.
- `docs/standards/mechanical-refactor-authoring-guide.md` — mechanical refactor closure loop and artifact expectations.
- `docs/standards/kernel-work-package-preparation.md` — required kernel package preparation procedure.
- `docs/standards/prompt-wording-guidance.md` — required prompt wording standard.
- Package-local `package.md`, `prompts/`, and `artifacts/` directories.

## Standard Workflow
1. Confirm the package is authorized by queue, decision, or user direction.
2. Read root `AGENTS.md`, this file, package-local `package.md`, and any nested `AGENTS.md` for files in the write set.
3. For kernel-affecting work, also read `docs/specifications/science-contracts/AGENTS.md` before edits.
4. Execute package phases end-to-end unless a declared hard blocker is reached.
5. Update artifacts truthfully as work proceeds; label `Static:` vs `Ran:` evidence.
6. Complete dual independent reviews, explicit finding disposition, dual verification, line-count governance, and final disposition before closure.

## Work-Package Authoring Requirements
- Use directory format `YYYYMMDD-<slug>-001` under `docs/work-packages/`.
- Add or update `docs/work-packages/README.md` so intent is discoverable.
- Scaffold `package.md`, `prompts/active/`, `prompts/archived/`, and `artifacts/` with queued placeholders.
- Encode status, objective, rationale, included/excluded scope, deliverables, dependencies, intended write set, phase plan, exit criteria, and security-impact gate.
- Require dual reviews with finding disposition: `accepted`, `rejected`, `deferred`, or `follow-up`.
- Require `.rs` line-count governance: 2000+ lines is `WARN`; 3000+ non-exempt files require refactor before closure.

## DC-ExecPlan Requirements
- Use a Defect-Closure ExecPlan when closing an observed invariant violation, fail-closed event on valid input, or conservation residual.
- Declare the Correction Authority Envelope: defect IDs, observed failures, in-scope contracts/source files, allowed edit classes, validation surfaces, acceptance criteria, and protected boundaries.
- If the root cause is in-envelope and expected behavior is supported by canonical `SC-*` authority, pinned-baseline provenance, or a contract-authorized physical invariant, land the contract-first correction in the same package.
- Close in `HOLD` only at a declared boundary: out-of-envelope mechanism, missing/contradictory authority, invalid upstream input with correct typed guard, unavailable evidence, or different process family/contract authority.
- The handoff's first actionable item must be `close defect <id>`, not a vague trace/inspect step.

## Mechanical Refactor Requirements
- Follow `docs/standards/mechanical-refactor-authoring-guide.md` for structural, behavior-preserving work.
- Required final closure loop for Rust implementation/mechanical refactor disposition is `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `cargo deny check`. Package prompts may define narrower focused iteration gates or additional gates, but may not waive final root closure gates unless a canonical decision or contract explicitly authorizes the exception.
- Package-required validation overrides generic ambient instructions to skip tests.
- Reconcile tests mechanically only; do not hide semantic changes inside refactor diffs.

## Validation Checklist
- Package-specific gates from `package.md`.
- Required Rust closure loop when implementation or mechanical refactor scope requires it.
- Doc-path integrity checks when moving documentation or required-reading lists.
- Source-level anti-evasion guards when touching external-authority suite posture, cohort fixtures, or required-case bindings.
- Dual review, review-disposition, dual verification, line-count governance, worker handoff, and disposition artifacts.

## Common Pitfalls
- Do not close a package while accepted review findings remain unfixed or undispositioned.
- Do not mark gates as run when they were reasoned about or partially executed.
- Do not use package artifacts to override canonical contract authority.
- Do not split a package solely to defer a known in-envelope correction.

## References
- Root guidance: `AGENTS.md`.
- Science contracts: `docs/specifications/science-contracts/AGENTS.md`.
- Prompt/procedure standards: `docs/standards/AGENTS.md`.
