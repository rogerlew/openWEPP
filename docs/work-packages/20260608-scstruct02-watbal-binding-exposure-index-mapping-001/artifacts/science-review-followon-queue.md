# SCSTRUCT02 Science Review Follow-On Queue

Evidence mode: Static
Status: routed

## Follow-on defect

Close `SCSTRUCT02-WATBAL-BEI-SCIENCE-REVIEW`: semantically adjudicate every `SC-WATBAL-001` Binding Exposure Index row marked with review gate `science-review-follow-on`.

## Required outcome

For each routed row, choose exactly one:

1. Map the binding residue to precise existing `INV-WATBAL-*` / `OBL-WATBAL-*` IDs.
2. Promote a genuinely unpromoted binding obligation through the full flagged-addition review gate.
3. Mark genuinely non-binding historical/superseded narrative and move it to the sidecar.
4. Keep a narrower science HOLD with explicit authority gap and owner.

## Current routed count

- Science-review routed rows updated in this pass: 69
