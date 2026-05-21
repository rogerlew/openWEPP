# Review Agent B — SPEC-INFILE-PMETPARA-001

Evidence: Static

## Findings (severity-ranked)

### PMET-B1
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:172`
- Issue: Gap/conflict register rows do not include explicit provenance tags per conflict item.
- Why it matters: Procedure requires provenance-tagged conflict entries for authority arbitration and verification.
- Proposed disposition: amend

### PMET-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:75`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:175`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:113`
- Issue: Legacy string-width compatibility risk (`names` 8 chars, `actlnam` 20 chars) is identified, but Section 8 has no explicit typed outcomes/guard policy for truncation or overflow handling.
- Why it matters: Parser behavior on overlength identifiers affects crop-key matching determinism and can silently alter coefficient assignment.
- Proposed disposition: amend

### PMET-B3
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:177`
- Issue: `PMET-GAP-004` is a provenance-completeness note but currently represented as a HOLD blocker.
- Why it matters: Mixing provenance-only notes with correctness blockers can blur promotion criteria.
- Proposed disposition: amend

## Final recommendation
HOLD
