# R7D6 Kickoff Prompt

Execute `docs/work-packages/20260623-r7d6-direct-erod13-erod14-typed-producer-001/package.md`
autonomously.

Required reading:

- Root `AGENTS.md`.
- `docs/work-packages/AGENTS.md`.
- `docs/specifications/science-contracts/AGENTS.md`.
- R7D5 artifacts, especially `producer-authority-map.md` and
  `worker-handoff.md`.
- `SC-SYSTEM-001` HBP sediment payload authority and EROD14/EROD15 addenda.
- Any canonical `SC-SED-*` contract relevant to EROD13/EROD14 producer
  ownership.

Execution rules:

- Start by authoring the operand-lineage table in
  `artifacts/operand-lineage.md`; do not edit production code before that
  table exists.
- Preserve the R7D5 fail-closed guard until a typed direct producer populates
  `DirectPublicationErosionOperands`.
- Do not use compatibility scheduler results, WB13 rows, HBP bytes, public
  output builders, or runtime sediment aliases as direct production authority.
- Iterate in-package while H2637 remains in-envelope: identify the residual,
  implement the next direct producer correction, rerun focused tests/H2637, and
  update artifacts. Do not stop after the first blocker unless it is a named
  out-of-envelope authority hold.
- Commit and push only after the package reaches complete or executed-held
  disposition and artifacts are current.
