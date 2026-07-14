# Kickoff: Scientific Assurance Dossier Lifecycle Foundation

Scope: local openWEPP repository engineering; flat-file reads/edits and local
Rust commands only. No network access, deployment, release, or mutation of
another repository is required or authorized.

Execution mode: `package-end-to-end`.

Phase plan: execute every phase in `package.md` sequentially through final
disposition. Do not stop at the governance contract, crate skeleton, candidate
page, or generated-output producer; prove the real generated usersum and
release-check consumer paths in the declared scope.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/20260714-assurance-dossier-lifecycle-foundation-001/package.md`
- `docs/work-packages/20260714-assurance-dossier-lifecycle-foundation-001/artifacts/required-reading-map.md`
- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/standards/scientific-assurance-dossier.md`
- `docs/standards/usersum-authoring-style-guide.md`
- `docs/decisions/0028-observed-data-admission-authority.md`
- `usersum/README.md`
- `usersum/snow-frost-modeling-and-validation.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `tools/release/README.md`
- `tools/release/check_hillslope_schedule_export.sh`
- `Cargo.toml`
- `.config/nextest.toml`

Conditional:

- `/home/workdir/wepppy/wepppy/weppcloud/routes/usersum/specification.md` and
  `/home/workdir/wepppy/wepppy/weppcloud/routes/usersum/vendors.yaml` before
  freezing the openWEPP export/handoff contract; read-only.
- `docs/standards/local-ci-gate-selection.md` before selecting or changing a
  nextest profile or expensive local gate.
- The exact ADR-0028 SNOTEL work-package artifacts and retained evidence paths
  discovered during the pilot inventory, before assigning any evidence status.
- `docs/specifications/science-contracts/AGENTS.md` and the relevant canonical
  contract only if execution discovers a proposed science-contract or kernel
  semantic change; that discovery is otherwise outside package scope and must
  stop implementation until the package is amended.

On-demand:

- The two completed 2026-07-14 V&V package dispositions when historical
  rationale or accepted-finding context is needed.
- Official nextest documentation only if local configuration and help cannot
  resolve a test-runner question.
- wepppy manifest, navigation, and contract-loader examples needed to verify
  the export fragment, read-only and without changing wepppy.

Required-reading budget: `222716` local bytes, threshold `OK` (at most 400000
bytes); map: `artifacts/required-reading-map.md`.

## Task

Execute the lifecycle/ownership contract, minimal deterministic builder,
SNOTEL snow-evidence public vertical slice, release drift/snapshot boundary, and
wepppy handoff exactly as bounded by `package.md`. Inventory evidence before
choosing a characterization. If favorable status is not supported, publish the
honest lower evidence status. Keep ordinary builds agent-free and keep
application fitness with the named decision owner.

Use nextest to verify the builder; do not model the evidence DAG in nextest.
Do not add a database, service, general workflow engine, persistent cache,
network fetch, arbitrary command execution, or cross-repository write.

Subagent requirement: REQUIRED for two independent reviewers/verifiers and for
the terminal heavy closure loop when available. This prompt explicitly
authorizes subagent spawning/delegation to Reviewer A, Reviewer B, and a
`comparator_suite_runner` or equivalent heavy-gate runner for the bounded scopes
in `package.md`. Outputs are compact package artifacts and durable command-log
paths. Reviewer write access is limited to each assigned artifact; the heavy
runner may write only package gate artifacts and tool-generated temporary
outputs.

Autonomy: execute end to end and update all required artifacts without asking
for more direction unless a declared hard blocker is proven. Missing empirical
evidence lowers the published evidence status; it does not by itself block the
lifecycle/build package.
