# R6I Review And Disposition

Evidence class: Static plus Ran.

## Review 1 - Correctness

Scope:

- Direct lane carry mutation order.
- PMET seed-surface source authority.
- R6G/R6H marker behavior.
- Cutover fail-closed behavior.

Findings:

- No correctness defects found in the R6I implementation.
- Confirmed the corrected production path is direct-seed and direct-frame
  authority, not WB13/runtime compatibility wrapping.
- Confirmed current-fixture HBP and WAT parity tests pass and R6G/R6H markers
  are absent.

Disposition: accepted, no code change required.

## Review 2 - QA / Maintainability

Scope:

- Test coverage.
- Disabled-path behavior.
- Line-count governance.
- Remaining blockers.

Findings:

- Focused test coverage now includes the direct-runtime fine-layer ULP
  projection, day-2 PMET seed bit parity, HBP/WAT identity, and CLI
  fail-closed behavior.
- Frost-disabled or absent-frost direct publication inputs produce `None` for
  the carry projection, so the new projection is not added to disabled
  contexts.
- Touched files remain below the 3000-line hard refactor threshold.
- Remaining direct publication cutover blocker is manifest writer wiring, not
  R6I PMET/WAT scope.

Disposition: accepted. Manifest writer cutover is follow-up scope and recorded
in `worker-handoff.md`.

## Finding Disposition Table

| Finding | Disposition | Action |
| --- | --- | --- |
| Direct commit skipped frost fine-layer carry projection | accepted | Fixed in `DirectLaneFrame::commit_day` via typed projection. |
| R6H hold tests expected stale `Es` mismatch | accepted | Updated to assert WAT identity and manifest fail-closed boundary. |
| Manifest projection writer still absent | follow-up | Out of R6I scope; handoff created for next package. |

## Review Verdict

`COMPLETE-R6I-DIRECT-PMET-LAYER-ULP-PARITY` is supported by current evidence.
