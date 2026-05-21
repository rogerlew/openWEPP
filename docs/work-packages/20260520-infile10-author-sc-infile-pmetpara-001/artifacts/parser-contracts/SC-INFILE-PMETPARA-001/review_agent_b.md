# Review Agent B — SC-INFILE-PMETPARA-001

Evidence: Static

## Findings (severity-ranked)

### PMET-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:37`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:60-68`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:74-82`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:134-136`
- Issue: The contract says missing-sidecar branch must produce explicit provenance (`iflget=1` branch), but there is no field-spec/propagation/boundary surface for sidecar-presence or default-branch provenance state.
- Why it matters: This is an externally relevant branch outcome and should be contract-visible per parser-contract requirements; without a modeled state surface, implementations cannot consistently expose/consume the optional-surface mode transition.
- Proposed disposition: amend

### PMET-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:115`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:147`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PMETPARA-001.md:155-162`
- Issue: `PMET-W-001` (missing-sidecar compatibility warning) exists in taxonomy/policy but is not linked from any guard failure behavior in Section 11.
- Why it matters: Invariant-to-guard linkage is a hard requirement; missing linkage makes compatibility observability non-executable and undermines verification of strict-vs-compat behavior.
- Proposed disposition: amend

## Final recommendation
HOLD
