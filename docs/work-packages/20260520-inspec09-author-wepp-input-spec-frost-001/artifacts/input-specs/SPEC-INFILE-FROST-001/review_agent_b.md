# Review Agent B — SPEC-INFILE-FROST-001

Evidence: Static

## Findings (severity-ranked)

### FROST-B1
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:183`
- Issue: Gap/conflict register rows do not include explicit provenance tags (`usersum2024`, `legacy-code`, `wepppy`, `wepppyo3`, `literature`) per row.
- Why it matters: Conflict-resolution provenance tagging is a required structure rule and is needed for deterministic disposition/audit.
- Proposed disposition: amend

### FROST-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:87`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:185`
- Issue: Field dictionary assigns definitive land-cover semantics to `kfactor(1..3)` while `FROST-GAP-001` simultaneously states class-index mapping is unresolved.
- Why it matters: This creates contract ambiguity for executable parser/propagation behavior and can lead to inconsistent kernel mapping implementations.
- Proposed disposition: amend

### FROST-B3
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:133`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:134`
- Issue: Typed outcomes for malformed line-2 and out-of-range numeric values are deferred as unresolved policy without an explicit strict-vs-compat error taxonomy branch.
- Why it matters: Parser-contract enforcement paths become underspecified when present-file parse failures and clamping behavior are not mode-explicit.
- Proposed disposition: amend

## Final recommendation
HOLD
