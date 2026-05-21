# Review Agent A — SPEC-INFILE-GWCOEFF-001

Evidence: Static

## Findings (severity-ranked)

### GWCOEFF-A-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:178`
- Issue: Gap/conflict register rows do not include explicit provenance tags per row.
- Why it matters: Conflict provenance is a required governance structure for source-authority disposition.
- Proposed disposition: amend

### GWCOEFF-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:44`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:116`
- Issue: Version-prefixed (`datver`) variant is marked reject in the applicability matrix, but Section 8 does not define an explicit typed rejection outcome for this branch.
- Why it matters: Rejection branches should map to concrete typed errors for deterministic parser implementation.
- Proposed disposition: amend

### GWCOEFF-A-003
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:184`
- Issue: `GWCOEFF-GAP-005` is a provenance-ownership completeness note but is currently modeled as a HOLD blocker.
- Why it matters: Provenance-only notes should be clearly separated from correctness-impact blockers to keep promotion criteria crisp.
- Proposed disposition: amend

## Final recommendation
GO-WITH-AMENDMENTS
