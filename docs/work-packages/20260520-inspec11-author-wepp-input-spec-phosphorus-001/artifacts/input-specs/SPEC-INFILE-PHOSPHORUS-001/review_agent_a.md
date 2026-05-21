# Review Agent A — SPEC-INFILE-PHOSPHORUS-001

Evidence: Static

## Findings (severity-ranked)

### PHOS-A-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:183`
- Issue: Gap/conflict register rows omit explicit provenance tags per conflict item.
- Why it matters: Provenance-tagged conflicts are required for disposition traceability and authority arbitration.
- Proposed disposition: amend

### PHOS-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:125`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:186`
- Issue: Header-string policy conflict (legacy ignores header; wepppy requires exact literal) is acknowledged, but strict-vs-compat typed outcomes are not explicitly defined.
- Why it matters: Parser determinism depends on whether header mismatches are rejected, warned, or ignored in each mode.
- Proposed disposition: amend

### PHOS-A-003
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/phosphorus.spec.md:188`
- Issue: `PHOS-GAP-004` is a provenance-ownership tracking note but is represented as a HOLD blocker.
- Why it matters: Promotion logic is clearer when provenance-only concerns are separated from correctness blockers.
- Proposed disposition: amend

## Final recommendation
GO-WITH-AMENDMENTS
