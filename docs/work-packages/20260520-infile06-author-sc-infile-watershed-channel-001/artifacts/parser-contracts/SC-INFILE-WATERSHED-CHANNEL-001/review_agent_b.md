# Review Agent B — SC-INFILE-WATERSHED-CHANNEL-001

Evidence: Static

## Findings (severity-ranked)

### CHN-B-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:96-97`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:101-113`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md:45-60`
- Issue: The field table defines externally relevant derived fields (`has_rating_curve`, `control_override_applied`), but the propagation map has no explicit rows mapping those fields to runtime state ownership/phase/mutability.
- Why it matters: Parser-contract requirements call for propagation rows for every externally relevant field; missing rows leave derived-state lifecycle and guard linkage under-specified.
- Proposed disposition: amend

### CHN-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:127`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:159-168`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:195`
- Issue: `D-CHN-003` derives a sidecar requirement flag (`ipeak > 2`), but the boundary export section does not explicitly export a typed `sidecar_required` surface for orchestrator/runtime consumers.
- Why it matters: Sidecar requirement signaling is operationally significant for parser-to-runner integration; leaving it implicit increases risk of inconsistent `chan.inp` gate behavior across implementations.
- Proposed disposition: amend

### CHN-B-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:155`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:165`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:208-215`
- Issue: The contract requires `tcr.txt` adjustments to remain a separate layer and non-mutating for canonical `.chn` payload, but does not define an explicit adjustment-state boundary surface or ownership row for that non-mutating overlay.
- Why it matters: Without explicit representation, there is a practical risk of silent in-place mutation of parsed `chntcr` in downstream integration code.
- Proposed disposition: amend

## Final recommendation
GO-WITH-AMENDMENTS
