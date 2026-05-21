# Review Agent B — SPEC-INFILE-IRRIGATION-FIXEDDATE-001

Evidence: Static

## Findings (severity-ranked)

### FDIRR-B1
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:80`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:125`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:193`
- Issue: Furrow line-5 arity is unresolved (`usersum` includes `tdepl`, legacy read path consumes only `qspply,tstart,tend`), but typed behavior currently treats arity mismatch generically as an error without a normative strict/compat split.
- Why it matters: This directly affects executable parser behavior and compatibility with legacy datasets; missing explicit policy can cause false rejects or silent divergence.
- Proposed disposition: amend

### FDIRR-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:41`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:108`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:125`
- Issue: No-version compatibility branch is defined in matrix/branch sections, but Section 8 does not specify a typed outcome for strict-mode rejection vs compatibility-mode acceptance.
- Why it matters: Contract implementation needs deterministic branch behavior for malformed/legacy headers.
- Proposed disposition: amend

### FDIRR-B3
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:191`
- Issue: Gap/conflict register omits explicit per-row provenance tags.
- Why it matters: Conflict-resolution provenance tagging is a required structure rule and supports reproducible disposition decisions.
- Proposed disposition: amend

## Final recommendation
HOLD
