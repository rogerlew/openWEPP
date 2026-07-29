# ASSURE-MAINT02 Kickoff

Scope: local repository assurance-tooling implementation; flat-file reads and
edits plus deterministic local commands only. Do not use network or external
systems.

Execution mode: package-end-to-end.

Task: execute `ASSURE-MAINT02` through terminal disposition. Implement typed
new-report admission and secure retained-SVG assembly, admit only the CAL-09
draft, and render/check the complete three-report V2 catalog into two unrelated
temporary roots and `target/assurance-preview/`.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `docs/codex_exec_plans.md`,
  `docs/standards/testing-and-gate-strategy.md`,
  `docs/governance/scientific-assurance-v2-source-build-contract.md`,
  `docs/specifications/assurance-amendment-and-identity-workflow.md`,
  `assurance/v2/README.md`,
  `docs/work-packages/20260715-assure04c-deterministic-manuscript-assembly-001/artifacts/assembly-contract.md`,
  and
  `docs/work-packages/20260729-canopy-cal-09-assurance-report-001/artifacts/worker-handoff.md`.
- Conditional: the exact V2 schemas, Rust modules, integration tests, and
  CAL-09 source files before editing or reviewing them.
- On demand: ASSURE-04A through ASSURE-04D package evidence, the CAL-09 package
  evidence, and publication/release contracts only when a specific
  implementation or test question requires them.

Required-reading budget: `171524` local bytes for the Core set, `OK`; exact map
is in `artifacts/required-reading-map.md`.

Files: only the intended write set in `package.md`. Treat all other paths as
read-only. Prospectively amend and review the package before expanding the
tracked write set.

Constraints: preserve report science, existing approved/draft lifecycle state,
generated-identity authority, and tracked public bytes. Do not hand-edit
digests or locks. Do not create human approval, publish, transfer a release,
change kernel/runtime physics, use network access, or embed active/external SVG
content.

Acceptance: the typed admission transaction passes dry-run, apply,
idempotence, stale-input, rollback, confinement, and generated-receipt tests.
Exactly three reports validate, plan, build, and check through the real
`--all` path. Two unrelated output roots are byte-identical. The stable preview
checks successfully. CAL-09 renders six main and two supplement SVG figures
inline with captions and alternatives. Existing report rendering and protected
tracked surfaces remain unchanged.

Subagent requirement: REQUIRED for two independent read-only implementation
reviews and two fresh independent read-only terminal verifications. This
prompt and package explicitly authorize delegation to those roles, with compact
findings/PASS evidence written to the named artifacts. REQUIRED: use
`comparator_suite_runner` for campaign-strength full-workspace or comparator
gates. Do not run those heavy gates on the parent model unless the runner is
unavailable and the package records the fallback.

Autonomy: execute every phase without requesting routine direction. Stop only
at an authority boundary, irreconcilable protected-path overlap, missing
external fact that materially changes the contract, or repeated hard blocker.

Outputs: completed implementation, admitted CAL-09 draft, full-catalog preview,
transaction and deterministic-build evidence, gate ledger, two reviews,
finding disposition, two terminal verifications, exact-diff reconciliation,
and truthful final disposition.
