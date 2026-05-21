# Review Agent A — SPEC-INFILE-FROST-001

Evidence: Static

## Findings (severity-ranked)

### FROST-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:40`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:131`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:133`
- Issue: The spec documents legacy-compat handling for missing/malformed line-2 records, but strict-mode deterministic behavior is still unresolved and not mapped to explicit typed outcomes for all line-2 failure shapes.
- Why it matters: This is parser-execution critical; without explicit strict behavior, implementations can diverge between fail-fast and silent defaulting on present-but-malformed files.
- Proposed disposition: amend

### FROST-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:183`
- Issue: Gap/conflict register rows omit explicit provenance tags (`usersum2024`, `legacy-code`, `wepppy`, `wepppyo3`, `literature`).
- Why it matters: Provenance-tagged conflicts are required for disposition governance and reproducible authority arbitration.
- Proposed disposition: amend

### FROST-A-003
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:188`
- Issue: `FROST-GAP-004` is a grammar/provenance completeness ambiguity but is currently represented as a promotion-blocking HOLD without explicit correctness-impact qualifier.
- Why it matters: Mixing provenance/clarity gaps with correctness blockers can blur promotion gate semantics.
- Proposed disposition: amend

## Final recommendation
HOLD
