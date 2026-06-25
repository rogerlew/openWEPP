# SNOWFREEZE Frost-Depth Literature Annotation

Status: complete

Package type: research / annotation work package.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: annotate the newly available frost-depth physics literature in
`references/`, classify the new vendorable PDFs, and produce a source map that
future frost-depth fidelity packages can use before proposing production
physics changes.

This package follows `docs/codex_exec_plans.md`,
`docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`, and
`references/README.md`.

## Purpose

R7H closed opt-in because compatibility frost output is not a valid acceptance
target for frost-depth magnitude. The next frost work needs observation-first
physics decisions, not another compatibility-parity grind. This package makes
the local literature corpus discoverable and separates implementable authority
from candidate research directions.

The package is docs-only. It does not amend frost physics, production runtime
behavior, observation thresholds, direct activation, or compatibility rollback
semantics.

## In Scope

- Inventory the local frost-depth papers currently present in
  `references/copyrighted/` and `references/vendorable/`.
- Update `references/annotated_bibliography.md` with concise entries for the
  frost-depth research corpus.
- Update `references/rights_classification_first_pass_2026-05-11.md` for the
  two new CC-BY vendorable PDFs.
- Record a candidate-physics map for future `GAP-SNOWFREEZE-002` packages.
- Register this package in `docs/work-packages/README.md`.

## Out of Scope

- Do not change Rust, tests, fixtures, runtime outputs, or physics constants.
- Do not default-activate direct frost runtime.
- Do not use these papers to bypass `SC-SNOWFREEZE-001` contract-first
  sequencing.
- Do not commit ignored `references/copyrighted/**` PDFs.
- Do not promote `Qwet` or any frozen hydraulic-conductivity model without a
  follow-on contract work package and observation-backed acceptance gates.

## Intended Write Set

- `docs/work-packages/20260625-snowfreeze-frost-depth-literature-annotation-001/**`
- `docs/work-packages/README.md`
- `references/annotated_bibliography.md`
- `references/rights_classification_first_pass_2026-05-11.md`
- `references/vendorable/Amico2011.pdf`
- `references/vendorable/Devoie2022.pdf`

## Source Corpus

The package annotates these local sources:

- Dun et al. (2010), WEPP frost-simulation subroutine improvements.
- Watanabe and Flury (2008), capillary-bundle frozen hydraulic conductivity.
- Kurylyk and Watanabe (2013), review of freezing/thawing math.
- Dall'Amico et al. (2011), energy-conserving freezing variably saturated soil.
- Kurylyk et al. (2014), analytical thaw benchmarks.
- Azmatch et al. (2012), SFCC-derived frozen hydraulic conductivity.
- Ming et al. (2020), saturated frozen hydraulic conductivity from SFCC.
- Amankwah et al. (2021), salt-exclusion SFCC behavior.
- Cheng et al. (2023), impedance factor and ice segregation.
- Devoie et al. (2022), measured SFCC repository.

## Phase Plan

### Phase 0: Inventory and Rights

Identify untracked reference files, confirm ignored copyrighted-cache behavior,
extract metadata from local PDFs, and classify redistributable sources.

Exit criteria:

- `artifacts/source-inventory.md` lists the source corpus and rights posture.
- The rights log names the two CC-BY vendorable PDFs.

### Phase 1: Annotation

Read the local PDFs at abstract/conclusion/detail level sufficient to classify
their relevance without copying copyrighted text. Update the annotated
bibliography with source role, local path, quality, and kernel mapping.

Exit criteria:

- `references/annotated_bibliography.md` includes entries for the source
  corpus.
- Dun et al. (2010) no longer says full text is pending.
- Watanabe/Qwet and Dun/Qwet authority conflicts are recorded as candidate
  research, not production authority.

### Phase 2: Synthesis and Closure

Record how the sources should sequence future frost-depth physics work, review
the docs diff, and run docs-safe validation.

Exit criteria:

- `artifacts/literature-synthesis.md` identifies the recommended physics ladder.
- Review, disposition, verification, and worker handoff artifacts are present.
- `git diff --check` passes.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `git status --short references docs/work-packages`
- `pdfinfo references/vendorable/Amico2011.pdf`
- `pdfinfo references/vendorable/Devoie2022.pdf`
- `pdftotext <each local source PDF> /tmp/<source>.txt`
- `git check-ignore -v references/copyrighted/<source>.pdf`
- `git diff --check`

## Closure Disposition

Complete. The frost-depth literature corpus is indexed, the new vendorable PDFs
are rights-classified, and future physics packages have a staged authority map:
validate heat-flow/snow first, then evaluate SFCC/frozen-K candidates, and only
then consider `Qwet` with impedance or capillary-bundle limits.
