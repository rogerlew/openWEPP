# Review Agent B — SC-INFILE-CHANINP-001

Evidence: Static

## Findings (severity-ranked)

### CHAN-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:38-39`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:143`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:191`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:46-47`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:150-151`
- Issue: The matrix/spec require distinct strict handling for missing required surface vs non-ENOENT open failure, but taxonomy collapses required-branch strict IO handling into a single `CHN-E-000` path.
- Why it matters: Losing typed branch distinction weakens strict-vs-compat precision and blocks deterministic closure verification for missing vs operational I/O fault conditions.
- Proposed disposition: amend

### CHAN-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:90`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:148`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:198`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:200`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:157`
- Issue: Field semantics state compat retention-with-warning for unknown `ichnum` IDs, but guard/taxonomy paths for `G-CHN-008` are error-only (`CHN-E-005`) with no explicit unknown-ID compatibility warning mapping.
- Why it matters: Compatibility behavior is under-specified and non-executable, creating cross-implementation drift for topology-ID mismatch handling.
- Proposed disposition: amend

## Final recommendation
HOLD
