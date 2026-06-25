# Kickoff: SNOWFREEZE Frost-Depth Literature Annotation

Execution mode: package-end-to-end.

Autonomy: execute the complete docs-only package without user intervention
unless a hard blocker prevents reading the local sources or updating tracked
documentation.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `references/README.md`
- `docs/work-packages/20260625-snowfreeze-frost-depth-literature-annotation-001/package.md`

Conditional:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/backlog/20260612-frost-heave-frozen-fringe-impedance-formulation.md`
- `docs/architecture/coupled-frost-sub-solver-specification.md`

On-demand:

- Local PDFs named in the package source corpus.
- Official DOI/publisher pages when citation or license metadata needs
  confirmation.

Instructions:

1. Inventory the local reference files and confirm which files are tracked,
   ignored, or untracked.
2. Extract PDF text to `/tmp` for annotation only. Do not commit extracted text
   from copyrighted sources.
3. Update `references/annotated_bibliography.md` with concise annotations for
   each source and source-role limits.
4. Update the rights classification log for redistributable PDFs only.
5. Record source inventory, synthesis, reviews, disposition, verification, and
   handoff artifacts.
6. Run `git diff --check` and record results.

Protected boundaries:

- Do not change production code or tests.
- Do not commit ignored copyrighted PDFs.
- Do not promote `Qwet`, SFCC, impedance, or frozen hydraulic-conductivity
  formulas into runtime authority from this package.
