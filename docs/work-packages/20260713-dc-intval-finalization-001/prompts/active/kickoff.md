# Finalize Integrated Validation End To End

Execution mode: `package-end-to-end`.

Close `INTVAL-FINAL-001` as one iterative DC campaign. Begin with
`INTVAL-AUTH-BIND-001`, inventory and restore all five missing required targets
as one batch, and continue through every newly exposed release, authority,
stability, or integrated-validation defect that is correctable inside
`package.md`. Do not create piecemeal successor packages for intermediate gate
failures. Finish only at verified `PASS-INTEGRATED-VALIDATION` or a proven
external/authority HOLD boundary.

Subagent requirement: **REQUIRED**. This prompt explicitly authorizes subagent
spawning/delegation for the inventory, bounded implementation, heavy runner,
two reviewers, and two verifiers described in `package.md`.
