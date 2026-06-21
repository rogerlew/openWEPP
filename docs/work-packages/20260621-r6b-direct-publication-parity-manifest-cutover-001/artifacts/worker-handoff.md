# Worker Handoff

Status: executed-hold.
Evidence mode: Static + Ran.

## Current Hold

`HOLD-R6B-DIRECT-PUBLICATION-TYPED-OPERAND-BRIDGE-ABSENT`

## First Actionable Item

Implement the production typed operand bridge that populates
`DirectRunFrame`/`DirectRunPublicationFrame` before
`DirectFrameExecutor::run_publication_capture`.

Follow-on package
`docs/work-packages/20260621-r6c-direct-publication-typed-operand-bridge-001/`
executed-held at
`HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`; it proved the bridge must
be added to the production climate lifecycle before publication-artifact
building, not patched at the output helper boundary.

The bridge must:

- start from parsed inputs and accepted direct run/lane/day operands;
- reject missing required publication operands with typed fail-closed errors;
- keep compatibility WB13 rows, runtime surfaces, writeback payloads, and stale
  logical state out of accepted direct-publication authority;
- populate HBP, WAT, PASS, loss, and manifest operands from direct authority;
- preserve public writes as fail-closed until parity, anti-alias,
  reconstruction, manifest, no-compatibility, and benchmark gates pass.

## Do Not Skip

- Populate direct frame operands before accepting output-family parity.
- Add anti-alias fixtures before accepting each family.
- Add independent reconstruction before accepting conservation-sensitive
  outputs.
- Cut manifest provenance/checksum reads to typed direct projection.
- Run cutover gates and benchmarks before completion.

## Evidence To Reuse

- Source map and current failure are recorded in `data-path-proof.md`.
- Field lineage and blocked producers are recorded in `operand-lineage.md`.
- The fail-closed marker is
  `R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT`.
