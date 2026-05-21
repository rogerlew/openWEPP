# Review Agent B — SPEC-INFILE-GWCOEFF-001

Evidence: Static

## Findings (severity-ranked)

### GWCOEFF-B1
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:178`
- Issue: Gap/conflict register omits explicit per-row provenance tags.
- Why it matters: Provenance-tagged conflicts are required for source-authority arbitration and verification closure.
- Proposed disposition: amend

### GWCOEFF-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:44`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:116`
- Issue: Version-prefixed (`datver`-prefixed) variant is explicitly rejected in the applicability matrix, but no corresponding typed error branch is defined in Section 8.
- Why it matters: Rejection paths need executable typed outcomes for parser-contract determinism.
- Proposed disposition: amend

### GWCOEFF-B3
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:53`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:60`
- Issue: Grammar allows `trailing_text`, but strict-vs-compat policy for trailing token acceptance/rejection is not explicit.
- Why it matters: Parser implementations can diverge on comment/annotation tolerance without a mode-gated contract rule.
- Proposed disposition: amend

## Final recommendation
HOLD
