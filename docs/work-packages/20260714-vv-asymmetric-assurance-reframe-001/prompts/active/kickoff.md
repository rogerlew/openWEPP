# Kickoff: Asymmetric Scientific Assurance Reframe

Scope: local repository documentation work; flat-file reads and edits only; no
external connectivity or external-system action is required.

Execution mode: `package-end-to-end`.

Phase plan: execute every phase in `package.md` sequentially through final
disposition.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260714-vv-asymmetric-assurance-reframe-001/package.md`
- `docs/work-packages/20260714-vv-asymmetric-assurance-reframe-001/artifacts/required-reading-map.md`
- `docs/governance/openwepp-verification-validation-strategy.md`
- `docs/standards/scientific-assurance-dossier.md`
- `docs/work-packages/20260714-vv-strategy-scientist-facing-inversion-001/artifacts/disposition.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/decisions/0028-observed-data-admission-authority.md`
- `docs/specifications/correctness-authority-model.md`

Conditional:

- `references/annotated_bibliography.md`, entries `R-114` through `R-125`, when
  editing the research basis.

On-demand:

- NRC, EPA, Oreskes, Nearing, and Wang primary-source landing pages only when a
  source characterization cannot be resolved from canonical bibliography
  metadata.

Required-reading budget: `114476` local bytes, `OK` (at most 400000 bytes).
Map: `artifacts/required-reading-map.md`.

## Task

Separate binary verification acceptance, nonterminal empirical corroboration,
and decision-owner application fitness throughout the strategy and dossier
standard. Replace terminal scientific release qualification with verified
software plus an as-of corroboration snapshot. Preserve the content-bound
manifest, uncertainty, calibration separation, negative evidence, review, and
known-invalid exclusions. Update research provenance and navigation, validate
the docs, and close only after dual independent review and verification.

Subagent requirement: REQUIRED for two independent documentation/scientific
reviewers. This prompt explicitly authorizes subagent spawning/delegation to
Reviewer A and Reviewer B for the bounded scopes in `package.md`. Outputs are
compact package artifacts with write access limited to each assigned artifact.
No heavy runner is required.

Autonomy: execute the package end-to-end without requesting more user direction
unless a hard blocker is reached.
