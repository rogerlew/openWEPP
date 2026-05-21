# Review Agent A — SPEC-INFILE-WEPPUI-001

Evidence: Static

## Findings (severity-ranked)

### WEPPUI-A-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:113`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:149`
- Issue: The spec identifies non-empty sentinel content as a strict/compat policy conflict, but Section 8 does not map this branch to explicit typed outcomes.
- Why it matters: Presence-only sentinel semantics need deterministic parser behavior when content exists (reject/warn/ignore by mode).
- Proposed disposition: amend

### WEPPUI-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:147`
- Issue: Gap/conflict register rows omit explicit provenance tags per row.
- Why it matters: Provenance-tagged conflicts are required by the authoring procedure for governance and verifier reproducibility.
- Proposed disposition: amend

### WEPPUI-A-003
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/wepp-ui.spec.md:152`
- Issue: `WEPPUI-GAP-004` (cross-repo ownership clarity) is tracked as `HOLD` rather than a non-blocking provenance note.
- Why it matters: Separating correctness blockers from governance/provenance notes reduces promotion ambiguity.
- Proposed disposition: amend

## Final recommendation
GO-WITH-AMENDMENTS
