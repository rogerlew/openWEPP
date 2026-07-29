# CANOPY-DOC-01 Archived Kickoff

Scope: local repository scientific-documentation task; flat-file reads and
edits only; no external systems or network actions are required.

Execution mode: package-end-to-end.

Phase plan: execute every phase in `package.md` sequentially through
disposition.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`, this package,
  `docs/codex_exec_plans.md`,
  `docs/standards/testing-and-gate-strategy.md`,
  `docs/standards/usersum-authoring-style-guide.md`,
  `docs/planning/canopy-phenology-assurance-roadmap.md`,
  `docs/decisions/0034-management-file-lanuse-input-authority.md`,
  `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`,
  `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`,
  `usersum/README.md`, and
  `usersum/snow-frost-modeling-and-validation.md`.
- Conditional:
  `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md`
  and `crates/openwepp-management-schema/src/lib.rs` when inventorying exact
  user fields; downstream contracts when PLANT/RESIDUE do not support a causal
  statement.
- On demand: claim-bearing source ledgers, final dispositions, calibration
  ensembles, identifiability analyses, figure sidecars, and primary literature
  from completed canopy packages.

Required-reading budget: `370735` local bytes, `OK`; map:
`artifacts/required-reading-map.md`.

Files: `usersum/openwepp-canopy-phenology.md`, `usersum/README.md`, optional
conceptual assets under `usersum/`, the canopy roadmap, work-package catalog,
and this package tree.

Task: author the broad native-forest canopy-phenology model narrative and
coefficient guide. Build the source/claim map and coefficient ledger before
prose. Explain units, meaning, hard domains, evidence-supported or
not-established ranges, calibration sequencing, identifiability, transfer,
and downstream compensation risks for every in-scope user coefficient.

Constraints: do not alter production science, contracts, schemas, defaults,
or runtime behavior. Do not present examples, search domains, single-site
fits, or accepted ensembles as universal typical ranges. Keep detailed
quantitative evaluation and reproduction material in `CANOPY-ASSURE-01`.
Keep `usersum` self-contained and use published sources for reader-facing
citations.

Subagent requirement: REQUIRED for two independent terminal
scientific/editorial reviews and verifications. This prompt explicitly
authorizes subagent spawning/delegation to those roles for the narrative,
coefficient authority, calibration guidance, citations, style, validation,
and closure review. Outputs are the four review/verification artifacts named
in `package.md`; write access is limited to those artifacts. No heavy batch,
comparator, or full-workspace subagent is required.

Autonomy: execute all package phases and update required artifacts without
requesting additional user direction unless hard-blocked.

Outputs: complete the usersum narrative, catalog entry, package evidence,
reviews, verifications, roadmap/catalog disposition, and prompt archival.
