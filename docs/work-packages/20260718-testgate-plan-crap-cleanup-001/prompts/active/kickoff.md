# TESTGATE-PLAN-CRAP-01 Kickoff

Execution mode: package-end-to-end.

Scope: local repository engineering in `/home/workdir/openWEPP`, limited to
flat-file reads/edits in the declared write set. No external systems or network
actions are required.

Execute every phase in `package.md` sequentially through disposition. Preserve
all planner/verifier semantics and add only essential adversarial coverage.
Cover under-tested branches before behavior-preserving decomposition. Do not add
exceptions, weaken guards, change policy, or repeat successful broad gates.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`,
  `docs/work-packages/20260718-testgate-plan-crap-cleanup-001/package.md`,
  `docs/decisions/0021-module-coverage-closure-thresholds.md`, and
  `docs/standards/code-quality-refactor-authoring-guide.md`.
- Conditional: `tests/AGENTS.md` when adding integration tests; no science
  contract reading is required because kernel behavior is out of scope.
- On-demand: `docs/standards/mechanical-refactor-authoring-guide.md`,
  `docs/standards/module-test-enhancement-authoring-guide.md`, and predecessor
  package evidence.

The required-reading map and byte budget are recorded in
`artifacts/required-reading-map.md`.

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two read-only reviewer roles, two read-only terminal
verifier roles, and one terminal heavy-gate runner. Expected outputs are compact
findings/verdicts and exact gate results; only the heavy runner may write
generated `target/` evidence.

Run focused crate tests during development. After source freeze and dual review,
run the conservative terminal sequence exactly once. Keep package artifacts and
status truthful under the Gate Evidence Non-Deferral Rule.
