# Assurance Editorial Fast-Path Kickoff

Scope: local repository assurance engineering; flat-file reads and edits only;
no external connectivity.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `../package.md` sequentially through final
disposition.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, `crates/AGENTS.md`,
  `tests/AGENTS.md`, `docs/standards/AGENTS.md`, `../package.md`,
  `docs/decisions/0038-manuscript-first-scientific-assurance-publication.md`,
  `docs/governance/scientific-assurance-v2-source-build-contract.md`, and
  `assurance/v2/README.md`.
- Conditional: `docs/standards/scientific-model-evaluation-report.md` when
  evaluating report-facing behavior; `docs/standards/local-ci-gate-selection.md`
  when adding the focused profile.
- On-demand: prior ASSURE-04C/04D/05 package artifacts for exact transaction,
  review-lock, and CRAP command patterns.

Required-reading budget: 89,688 bytes for the initial governance set, `OK`;
map: `../artifacts/required-reading-map.md`.

Files: only the declared write set in `../package.md`.

Task: execute the package objective end-to-end. Preserve content identities and
review locks while making DRAFT American-English normalization transactional,
deterministic, and proportionately gated.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to two read-only independent reviewers, one heavy gate/CRAP
runner with package-artifact-only writes, and two read-only terminal verifiers.
Outputs are compact findings, metrics, exact commands, and log/artifact paths.
The parent must not run the full workspace/CRAP batch while the delegated heavy
runner is available.

Autonomy: execute through disposition without requesting more user direction
unless an authority or external dependency creates a genuine hard block.
