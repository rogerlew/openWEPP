# Codex Review Prompt: D16 Hybrid Default-Promotion Adjudication

Review package:
`docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/`

Subagent authorization: this package explicitly authorizes
spawning/delegation to review and verification subagents for read-only
contract/code review and comparator/gate verification. Expected outputs are
package-local review and verification artifacts. Write access is read-only
unless a separate implementation-fix assignment is provided.

## Review Target

Assess whether the package correctly adjudicates and implements promotion of
the hybrid implicit-explicit stepper to the default inside the active Lane-D
path at the current 10-cell/OFE mesh.

Do not treat this as broader no-env Lane-D production default activation.
`OPENWEPP_LANED_ACTIVE=1` remains the active-owner selector unless the package
explicitly and validly changes that authority, which is not in scope.

## Questions

1. Does `SC-OFEROUTE-002#INV-OFEHYB-008` have enough current evidence to close
   for active-path default promotion, including Case-4, H2637 timing/counters,
   closure, and output-delta tolerances?
2. Does any contract text overclaim beyond the implemented selector semantics?
3. Does the implementation make hybrid the default only inside active Lane-D
   runs while preserving subsystem-off/default behavior?
4. Is the explicit plain-active comparator/rollback surface
   (`OPENWEPP_LANED_ACTIVE_IMPLICIT=0`) unambiguous and correctly surfaced in
   the manifest?
5. Do active day-closure hard-fails and routed-hydrograph consumer surfaces
   remain live under the promoted default?
6. Are protected-output byte identity, H2637 timing, and output-delta evidence
   based on fresh release-binary provenance?
7. Are any required gates missing, deferred, or misclassified?

## Output Protocol

Write findings into
`docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/review-codex-<lane>.md`.

Use severity ordering: High, Medium, Low. Include file:line references when
possible. End with a verdict:

- `GO`
- `GO-WITH-AMENDMENTS`
- `NO-GO`

