# Review Agent B — SC-INFILE-GWCOEFF-001

Evidence: Static

## Findings (severity-ranked)

### GWC-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:37`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:78-79`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:107-109`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:120-130`
- Issue: `lr_bf` derivation is modeled as presence-based (`1` when sidecar present) while the contract also requires strict rejection of malformed present files. This leaves an ambiguous state model for present-but-invalid input (enable branch vs hard error).
- Why it matters: Parser-contract correctness requires deterministic guardable branch semantics. Ambiguous enable-state derivation can propagate incorrect routing mode into runtime or observability surfaces during error paths.
- Proposed disposition: amend

### GWC-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:124`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:150-156`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md:171`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/gwcoeff.spec.md:121`
- Issue: Strict trailing-token rejection is routed to `GW-E-002` (record-count mismatch), conflating token-policy failure with arity failure and diverging from the paired spec's explicit trailing-token typed expectation.
- Why it matters: Error taxonomy precision is required for executable parser behavior and downstream guard verification. Collapsing distinct syntax failures into record-count errors weakens diagnostics and compatibility testing.
- Proposed disposition: amend

## Final recommendation
HOLD
