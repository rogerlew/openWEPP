# Review Agent A — SPEC-INFILE-WATERSHED-IMPOUNDMENT-IMP-001

Evidence: Static

## Findings (severity-ranked)

### IMP-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:6`
- Issue: Spec status is `draft` even though Section 10 defines unresolved HOLD conditions tied to open conflicts.
- Why it matters: Promotion gating depends on explicit HOLD state when unresolved conflicts remain; current status understates readiness and risks premature downstream contract authoring.
- Proposed disposition: amend

### IMP-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:13`
- Issue: Many non-trivial `[DIRECT]` claims are not explicitly bound to evidence anchors at claim site (EA IDs/path+line linkage is only listed later in a separate section).
- Why it matters: Procedure requires evidence anchors per non-trivial claim; weak claim-to-source linkage increases interpretation drift in review/disposition.
- Proposed disposition: amend

### IMP-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:296`
- Issue: Gap/conflict rows do not include explicit provenance tags (`usersum2024`, `legacy-code`, `wepppy`, `wepppyo3`, `literature`) per row.
- Why it matters: Missing provenance tags weakens conflict-resolution auditability and violates required conflict-tagging structure.
- Proposed disposition: amend

## Final recommendation
HOLD
