# Review Agent A — SC-INFILE-MANAGEMENT-001

Evidence: Static

## Findings (severity-ranked)

### MAN-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:72`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:183`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/plant-file.spec.md:98`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/plant-file.spec.md:240`
- Issue: Field specification table captures only a limited subset of management symbols while the paired spec defines substantially broader section-local parameter surfaces; contract itself acknowledges incomplete canonical coverage (`MAN-GAP-001`).
- Why it matters: Parser-contract requirements mandate complete externally relevant field coverage; incomplete field table prevents full data-model, propagation, and guard closure.
- Proposed disposition: amend

### MAN-A-002
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:94`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:191`
- Issue: Contract omits required boundary-export mapping for parsed/derived management fields crossing process boundaries.
- Why it matters: Without explicit boundary export requirements, multi-process interoperability (CLI/payload/interchange) is underspecified and prone to alias/unit drift.
- Proposed disposition: amend

### MAN-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:143`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md:147`
- Issue: Cross-file consistency constraints are stated without explicit evidence tags.
- Why it matters: Evidence-tag rigor is required for auditability and deterministic review/disposition; untagged normative constraints weaken provenance traceability.
- Proposed disposition: amend

## Final recommendation
HOLD
