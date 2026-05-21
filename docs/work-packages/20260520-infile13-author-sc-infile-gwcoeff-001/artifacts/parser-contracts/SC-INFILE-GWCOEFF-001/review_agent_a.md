Evidence: Static

## Findings (Severity-Ranked)

### GWC-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:153`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:154`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:51`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:59`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:141`
- Issue: Strict-mode policy rejects trailing tokens, but the paired spec’s canonical grammar and representative/fixture-backed evidence define numeric-leading lines with optional trailing text as a normal surface shape.
- Why it matters: This creates a contract/spec authority conflict and risks rejecting currently documented inputs under strict mode without a clearly ratified migration policy.
- Proposed disposition: amend

### GWC-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:124`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:171`
- Issue: Strict trailing-token rejection is routed through `GW-E-002` (record-count/arity mismatch), which is not a tokenization-specific error class.
- Why it matters: Typed error precision is required for executable strict-vs-compat behavior and deterministic verification/disposition.
- Proposed disposition: amend

Final recommendation: HOLD
