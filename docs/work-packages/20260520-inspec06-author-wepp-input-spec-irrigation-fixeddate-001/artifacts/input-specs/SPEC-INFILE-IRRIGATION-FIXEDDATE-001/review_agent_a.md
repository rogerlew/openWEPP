# Review Agent A — SPEC-INFILE-IRRIGATION-FIXEDDATE-001

Evidence: Static

## Findings (severity-ranked)

### FDIRR-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:80`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:193`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:134`
- Issue: Furrow line-5 arity conflict (`qspply tstart tend tdepl` in usersum vs 3-field legacy read path) is recorded as a HOLD gap, but parser acceptance/rejection behavior is not normatively specified with strict/compat typed outcomes.
- Why it matters: This is executable-parser critical; without deterministic arity policy, conformance tests and contract implementation can diverge on valid/invalid row shapes.
- Proposed disposition: amend

### FDIRR-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:41`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:125`
- Issue: No-datver compatibility branch is documented, but strict-mode behavior for omitted datver is not explicitly mapped to a typed error outcome.
- Why it matters: Strict-vs-compat policy must be directly enforceable in parser contracts.
- Proposed disposition: amend

### FDIRR-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:191`
- Issue: Gap/conflict register lacks explicit provenance tags per row.
- Why it matters: Conflict resolution requires explicit provenance typing for governance and verifier traceability.
- Proposed disposition: amend

## Final recommendation
HOLD
