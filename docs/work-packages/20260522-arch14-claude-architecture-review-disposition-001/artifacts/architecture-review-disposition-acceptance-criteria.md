# ARCH14 Acceptance Criteria

Static: governance/disposition criteria only.
Ran: none.

## Per-Finding Criteria

1. Every `CRF-*` row includes severity, decision, rationale, closure evidence, owner, and mapped follow-on package.
2. `CRF-001` and `CRF-002` are not `reject` and explicitly require:
   - migration to typed kernel state surfaces,
   - kernel-seam wiring of `openwepp-unit-boundary` types.
3. Every high-severity finding (`CRF-001`, `CRF-002`, `CRF-003`, `CRF-005`, `CRF-006`) has:
   - explicit closure evidence,
   - named owner,
   - package assignment,
   - `HOLD` status until closure evidence is produced.
4. Any `amend` disposition records what claim was corrected while preserving core risk remediation.
5. Any `defer` disposition includes risk acceptance and explicit dependency placement.

## Package-Level Criteria

1. Required artifact bundle exists and is non-placeholder:
   - `claude-review-findings-register.md`
   - `disposition-register.md`
   - `remediation-work-package-queue.md`
   - `architecture-review-disposition-acceptance-criteria.md`
   - `worker-handoff.md`
   - `owned-file-manifest.md`
   - `gate-results.md`
   - `arch14_disposition.md`
   - `review_agent_a.md`
   - `review_agent_b.md`
   - `verification_agent_a.md`
   - `verification_agent_b.md`
2. Dual reviews are independent and severity-ranked.
3. Dual verification artifacts confirm cross-file consistency and decision-policy compliance.
4. Final disposition states `GO`, `GO-WITH-AMENDMENTS`, or `HOLD` with rationale.
5. Correctness-over-completion policy is enforced: unresolved high-severity findings keep final ARCH14 status at `HOLD`.
