# Review Agent A — SPEC-INFILE-LCWB-001

Evidence: Static

## Findings (severity-ranked)

### LCWB-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:32`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:141`
- Issue: The spec asserts intended runtime semantics (last-OFE vs all-OFE emission selection) from a historical snapshot while simultaneously stating the active source consumer path is unresolved.
- Why it matters: This creates executable ambiguity in the canonical contract surface; parser/propagation behavior should not be asserted as active until current authority is ratified or clearly marked as conditional.
- Proposed disposition: amend

### LCWB-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:138`
- Issue: Gap/conflict register omits explicit row-level provenance tags.
- Why it matters: Required provenance tagging is necessary for consistent source-arbitration and verifier closure.
- Proposed disposition: amend

### LCWB-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:111`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:112`
- Issue: Strict-vs-compat policy is explicit for open-failure handling, but non-empty payload policy is only implicit compatibility behavior with no strict canonical stance.
- Why it matters: Even sentinel-only files need explicit mode behavior to avoid parser-contract divergence in strict implementations.
- Proposed disposition: amend

## Final recommendation
HOLD
