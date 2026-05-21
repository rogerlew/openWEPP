# Review Agent B — SPEC-INFILE-TCR-001

Evidence: Static

## Findings (severity-ranked)

### TCR-B1
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:176`
- Issue: The gap/conflict register lacks explicit provenance tags per row.
- Why it matters: Provenance-tagged conflict rows are required for reproducible governance, disposition, and verification.
- Proposed disposition: amend

### TCR-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:73`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:74`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:123`
- Issue: The relational invariant between `taumin` and `taumax` is only documented as a recommendation, and no typed guard/error mapping is defined.
- Why it matters: Unenforced curve-parameter ordering allows physically inconsistent or unintended parameterization while still passing parser acceptance.
- Proposed disposition: amend

### TCR-B3
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:53`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:61`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:117`
- Issue: Grammar allows trailing tokens and text states they should be ignored, but no explicit strict-vs-compat typed policy is provided for trailing-token handling.
- Why it matters: Without mode-gated handling, parser behavior can diverge between permissive and strict implementations.
- Proposed disposition: amend

## Final recommendation
HOLD
