# R6F Artifacts

Status: executed-held.

These artifacts are part of the execution contract for R6F. They are not
optional notes. A worker must keep them current while iterating through direct
publication blockers.

## Artifact Completion Rules

- Do not mark an artifact complete with generic "mismatch", "blocked", or
  "needs follow-up" language.
- Every blocker artifact must name the output family, row or metadata key,
  field or byte span, direct operand, producer or consumer, authority, attempted
  correction, and validation result.
- Every proposed `HOLD` must first satisfy
  [no-premature-stop-audit.md](no-premature-stop-audit.md).
- If one blocker is fixed and another appears, append the new blocker to the
  ledger and continue the same package.

## Required Artifact Set

| Artifact | Purpose | Closure state |
|---|---|---|
| `required-reading-map.md` | Records required context and why each item matters. | All required rows read before code edits. |
| `owned-file-manifest.md` | Tracks every touched file and why it is in scope. | Every edit has an in-scope rationale. |
| `r6f-current-failure-reproduction.md` | Reproduces the inherited R6E failure. | Marker, counters, and no-output state recorded. |
| `r6f-blocker-ledger.md` | Iterative blocker ledger. | No open in-envelope blocker remains at closure. |
| `r6f-hbp-byte-diff.md` | HBP byte/field reduction. | HBP identity proven or exact out-of-envelope boundary proven. |
| `r6f-output-family-burndown.md` | HBP/WAT/PASS/loss/manifest state tracker. | All output families closed or legitimate hold recorded. |
| `r6f-operand-lineage.md` | Maps publication fields to typed direct operands. | Every accepted field has direct authority. |
| `r6f-no-compatibility-proof.md` | Proves cutover has no compatibility authority. | Scans/counters/tests support the conclusion. |
| `r6f-anti-alias-fixtures.md` | Documents fixtures that reject self-consistency aliases. | Fixture evidence covers accepted output families. |
| `r6f-independent-reconstruction.md` | Reconstructs operands independently of writer code. | Reconstruction agrees with direct publication. |
| `r6f-manifest-cutover-evidence.md` | Proves manifest provenance/checksum cutover. | Manifest reads direct projection only. |
| `pre-implementation-contract-gate.md` | Records contract authority before science-affecting edits. | Each formula/unit correction has authority. |
| `gate-results.md` | Records focused and final commands. | Required gates are run or an honest hold explains why. |
| `line-count-governance.md` | Tracks touched Rust file size. | WARN/violation files have disposition. |
| `review_agent_a.md` / `review_agent_b.md` | Independent reviews. | Findings dispositioned. |
| `verification_agent_a.md` / `verification_agent_b.md` | Independent verification. | Closure evidence checked. |
| `disposition.md` | Final package verdict. | `COMPLETE-R6-DIRECT-PUBLICATION-CUTOVER` or legitimate `HOLD-R6F-*`. |
| `worker-handoff.md` | Exact next action if anything remains. | Starts with `close defect ...` for holds. |
