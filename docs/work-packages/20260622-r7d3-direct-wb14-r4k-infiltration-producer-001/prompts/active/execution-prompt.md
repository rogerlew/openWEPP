# Execution Prompt

Execute this R7D3 package end to end. Close
`HOLD-R7D2-DIRECT-WB14-R4K-INFILTRATION-PRODUCER-AUTHORITY-ABSENT` by
implementing baseline-authoritative direct WB14/R4K infiltration/depression
producer authority and wiring its outputs into R4A, WB18, ET, and direct
publication.

Read before edits:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- this package's `package.md`

Execution rules:

- Keep iterating through in-envelope blockers. Do not stop after diagnostics,
  one partial correction, or a merely improved H2637 run.
- Do not use compatibility WB13 rows, compatibility public-output builders, or
  compatibility `wb12_infiltration`/`wb12_depression_storage_delta` as direct
  authority.
- Preserve the direct runtime phase-span shape: inputs, direct compute, state
  mutation, downstream operands, and shadow projection.
- Close complete only when package acceptance gates pass. Otherwise close in a
  named hold with exact residual fields and the first follow-up code action.
