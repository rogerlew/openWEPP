# Review Agent A — SPEC-INFILE-IRRIGATION-DEPLETION-001

Evidence: Static

## Findings (severity-ranked)

### IRDEP-A-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:240`
- Issue: Gap/conflict register rows do not include explicit provenance tags (`usersum2024`, `legacy-code`, `wepppy`, `wepppyo3`, `literature`).
- Why it matters: Provenance-tagged conflicts are required for source-authority arbitration and disposition auditability.
- Proposed disposition: amend

### IRDEP-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:43`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:154`
- Issue: The version matrix introduces a no-datver compatibility branch, but defaulting/error taxonomy does not define explicit strict-mode rejection behavior for this branch.
- Why it matters: Parser implementers need deterministic guard behavior for compatibility-off runs; otherwise strict-vs-compat behavior is underspecified.
- Proposed disposition: amend

### IRDEP-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:75`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:160`
- Issue: Continuation-period ordering requirements (date-order then OFE tie-break) are documented, but no explicit typed error is defined for continuation-order violations.
- Why it matters: Continuation ordering affects runtime period selection and is parser-contract significant; missing guard mapping invites non-deterministic ingestion behavior.
- Proposed disposition: amend

## Final recommendation
GO-WITH-AMENDMENTS
