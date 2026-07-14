# Kickoff: Scientist-Facing V&V Strategy Inversion

Scope: local repository documentation work; flat-file reads and edits only; no
external connectivity or external-system actions are required.

Execution mode: `package-end-to-end`.

Phase plan: execute every phase in `package.md` sequentially through final
disposition.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/package.md`
- `docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/artifacts/required-reading-map.md`
- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/decisions/0028-observed-data-admission-authority.md`
- `docs/specifications/correctness-authority-model.md`

Conditional:

- None. Kernel, science-contract, executable, dataset-admission, and external
  publication changes are outside this package.

On-demand:

- Repository bibliography entries `R-114` through `R-124` only when checking
  retained research-basis wording.
- Existing integrated-validation artifacts only when checking current-state
  characterization.

Required-reading budget: `108880` local bytes, `OK` (at most 400000 bytes). Map:
`artifacts/required-reading-map.md`.

## Task

Rewrite the V&V strategy around transparent scientific assurance dossiers,
author the companion dossier standard, update indexes, validate the changed
documentation, and close the package only after two independent reviews and
finding disposition.

Preserve bounded-claim discipline, verification/validation separation,
uncertainty, calibration/evaluation independence, comparator-as-flag posture,
negative evidence, and truthful release language. Do not claim new empirical
support or select a new dataset. Do not make planned tooling a prerequisite for
publishing current evidence and gaps.

Subagent requirement: REQUIRED for two independent documentation/scientific
reviewers. This prompt explicitly authorizes subagent spawning/delegation to
Reviewer A and Reviewer B for independent review and verification. Outputs are
compact package artifacts; write access is bounded to each reviewer's assigned
artifact file. No heavy runner is required.

Autonomy: execute the package end-to-end without requesting more user direction
unless a hard blocker is reached.
