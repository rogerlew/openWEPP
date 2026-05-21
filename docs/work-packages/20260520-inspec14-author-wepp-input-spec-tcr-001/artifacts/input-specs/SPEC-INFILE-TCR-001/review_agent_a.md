# Review Agent A — SPEC-INFILE-TCR-001

Evidence: Static

## Findings (severity-ranked)

### TCR-A-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:176`
- Issue: Gap/conflict register rows do not include explicit provenance tags.
- Why it matters: Provenance-tagged conflicts are a required structure for source-authority disposition and verifier traceability.
- Proposed disposition: amend

### TCR-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:73`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:74`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:123`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:180`
- Issue: `taumin <= taumax` is declared only as a recommendation, but no explicit guard path or explicit governance disposition is defined for violations.
- Why it matters: Contract invariants should be enforceable (typed guard) or explicitly tracked as unresolved policy; recommendation-only language leaves execution behavior ambiguous.
- Proposed disposition: amend

### TCR-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:43`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:122`
- Issue: Open-failure handling specifies typed differentiation from `NotFound`, but compatibility-mode behavior is not explicitly declared.
- Why it matters: Strict-vs-compat bifurcation is incomplete for a core optional-sidecar branch, risking inconsistent implementations.
- Proposed disposition: amend

## Final recommendation
HOLD
