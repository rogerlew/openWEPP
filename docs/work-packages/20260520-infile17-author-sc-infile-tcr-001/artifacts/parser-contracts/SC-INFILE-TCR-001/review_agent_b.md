# Review Agent B — SC-INFILE-TCR-001

Evidence: Static

## Findings (severity-ranked)

### TCR-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:135`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:180`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md:168-172`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/tcr.spec.md:124`
- Issue: The contract enforces `taumin<=taumax` as a hard semantic error (`TCR-E-004`) without a compatibility warning path, but the paired spec explicitly defines compatibility behavior as warning + legacy-flow preservation for this relational violation.
- Why it matters: This is a strict-vs-compat policy mismatch between canonical spec and contract. Without alignment, implementations can diverge on whether compat runs fail or continue, breaking parser-contract authority ordering and reproducibility.
- Proposed disposition: amend

## Final recommendation
HOLD
