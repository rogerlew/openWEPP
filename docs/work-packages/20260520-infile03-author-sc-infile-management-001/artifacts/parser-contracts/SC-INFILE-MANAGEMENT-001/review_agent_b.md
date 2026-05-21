# Review Agent B — SC-INFILE-MANAGEMENT-001

Evidence: Static

## Findings (severity-ranked)

### MAN-B1
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:72`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:183`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/plant-file.spec.md:59`
- Issue: The contract’s field table is materially incomplete for many section-local scenario branches, which is explicitly acknowledged by `MAN-GAP-001`.
- Why it matters: Required parser-contract completeness (“every externally relevant field”) is not met for a complex, high-coupling input surface; parser semantics remain underspecified for multiple scenario families.
- Proposed disposition: amend

### MAN-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:149`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md:113`
- Issue: The contract omits an explicit boundary-export requirements section.
- Why it matters: Management schedule fields commonly cross subsystem/process boundaries; missing name/unit/interface mapping increases integration drift risk.
- Proposed disposition: amend

### MAN-B3
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:127`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:171`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/plant-file.spec.md:467`
- Issue: Error taxonomy and guard map do not explicitly define date-domain validation/guards for numerous Julian-day schedule fields.
- Why it matters: Schedule validity depends on bounded/ordered day semantics; missing typed date guards can allow invalid management timelines into runtime dispatch.
- Proposed disposition: amend

## Final recommendation
HOLD
