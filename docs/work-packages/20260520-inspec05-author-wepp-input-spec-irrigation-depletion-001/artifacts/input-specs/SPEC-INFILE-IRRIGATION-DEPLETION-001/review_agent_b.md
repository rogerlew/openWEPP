# Review Agent B — SPEC-INFILE-IRRIGATION-DEPLETION-001

Evidence: Static

## Findings (severity-ranked)

### IRDEP-B1
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:240`
- Issue: Gap/conflict register rows do not include explicit provenance tags per item.
- Why it matters: The spec-authoring procedure requires provenance tagging for conflict resolution; missing row-level tags weakens authority arbitration and disposition auditability.
- Proposed disposition: amend

### IRDEP-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:43`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:117`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:154`
- Issue: Legacy compatibility branches (no-datver pre-93 path and pre-94.21 sprinkler-nozzle omission) are documented, but Section 8 does not define explicit typed strict-vs-compat outcomes for these branches.
- Why it matters: Parser-contract enforceability depends on explicit branch outcomes; without them, implementations can diverge on acceptance vs rejection behavior.
- Proposed disposition: amend

### IRDEP-B3
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:245`
- Issue: `IRDEP-GAP-004` is framed as a provenance triangulation gap but is currently modeled as a HOLD blocker.
- Why it matters: Non-correctness provenance gaps should be clearly separated from promotion-blocking correctness conflicts to avoid ambiguous gate behavior.
- Proposed disposition: amend

## Final recommendation
HOLD
