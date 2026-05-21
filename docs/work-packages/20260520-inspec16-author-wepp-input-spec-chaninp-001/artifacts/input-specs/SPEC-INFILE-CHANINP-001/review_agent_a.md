# Review Agent A — SPEC-INFILE-CHANINP-001

Evidence: Static

## Findings (severity-ranked)

### CHANINP-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:135`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:142`
- Issue: The spec introduces strict-mode `MissingFile` error behavior for `ipeak > 2` without an explicit compatibility-mode branch, despite documenting legacy missing/open-failure defaulting.
- Why it matters: This is a core executable parser contract fork; without an explicit strict-vs-compat policy pair, implementations can diverge and break backward-compat expectations.
- Proposed disposition: amend

### CHANINP-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:42`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:193`
- Issue: Version/datver applicability matrix is underspecified for malformed/truncated/open-error cases and is not aligned to the typed-error branches in Section 8.
- Why it matters: Incomplete branch mapping weakens parser executability and makes error-policy behavior harder to verify.
- Proposed disposition: amend

### CHANINP-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:193`
- Issue: Gap/conflict register omits explicit provenance tags and uses non-scoped generic IDs (`G1..G4`) rather than spec-scoped gap IDs.
- Why it matters: Provenance tags are required for conflict governance; generic IDs reduce traceability and consistency with the specification corpus.
- Proposed disposition: amend

## Final recommendation
HOLD
