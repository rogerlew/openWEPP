# Execute ASSURE-03

Scope: local openWEPP repository assurance migration; flat-file reads, edits,
builds, and tests only; no external connectivity or external-system writes.

Execution mode: package-end-to-end.

Phase plan: execute every phase in `package.md` sequentially through final
disposition unless a proven hard blocker prevents a current-scope gate.

Required reading:

- Core: root `AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/work-packages/AGENTS.md`, this package, the ASSURE-02 handoff,
  `docs/planning/scientific-assurance-v2-migration-plan.md`, and
  `docs/planning/scientific-assurance-v2-implementation-roadmap.md`.
- Conditional: `crates/AGENTS.md` for compiler edits, `tests/AGENTS.md` for
  integration tests, `docs/standards/AGENTS.md` and
  `docs/standards/prompt-wording-guidance.md` for standards/prompts.
- On-demand: accepted v2 authority, current v1 source/code, public narratives,
  release scripts/workflow, and historical package records named in
  `artifacts/required-reading-map.md`.

Required-reading budget: 88,903 local bytes for Core, `OK`; map:
`artifacts/required-reading-map.md`.

Files: only the declared write set in `package.md`.

Task: record acceptance, preserve exact history, close `ASSURE03-REL-001`,
remove the active/public v1 candidate, establish deterministic zero reports,
repair reader paths without diminishing snow/frost science, and close every
package gate.

Constraints: no v2 compiler or report implementation; no scientific
reassessment; no WEPPcloud vendoring; fail closed on nonempty legacy catalogs,
transition markers, unsafe paths, snapshot conflicts, drift, and release-mode
ambiguity. Prove real CLI, release-script, workflow, public-catalog, and export
consumers rather than producer-only structure.

Subagent requirement: REQUIRED. Delegate heavy full-workspace, explicit release,
and CRAP closure runs to the heavy-gate runner. This prompt explicitly
authorizes subagent spawning/delegation to the heavy-gate runner for compact
metrics/log paths with bounded package-artifact write access, and to two
independent read-only reviewers plus two read-only verifiers for the scopes and
artifacts in Phase 5. Do not represent coding-agent review as external peer
review.

Autonomy: execute end-to-end and update all evidence without requesting next
steps unless hard-blocked. A phase is complete only with direct current evidence;
do not defer a required gate to ASSURE-04 and call ASSURE-03 complete.

Outputs: final source, generated zero-report surfaces, migration/recovery
evidence, gate results, reviews, disposition, verifications, and handoff.
