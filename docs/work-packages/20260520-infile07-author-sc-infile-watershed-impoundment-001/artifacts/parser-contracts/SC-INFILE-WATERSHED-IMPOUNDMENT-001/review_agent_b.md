# Review Agent B — SC-INFILE-WATERSHED-IMPOUNDMENT-001

Evidence: Static

## Findings (severity-ranked)

### IMP-B-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:61-67`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:70-183`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:73`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:78`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:83`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:91`
- Issue: The contract states source-model payload preservation by section/order, but branch-local `strdes` comment records (present across enabled structure branches in the paired spec) are not represented in field tables or propagation surfaces.
- Why it matters: Omitting parse-consumed source records weakens source-model fidelity and makes reserialization/provenance behavior ambiguous.
- Proposed disposition: amend

### IMP-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:182`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:186-198`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md:45-60`
- Issue: The field table defines a derived externally relevant field (`structure_enabled_flags`), but the propagation map does not include an explicit row for that field.
- Why it matters: Required propagation completeness is not met for a branch-governing derived surface that affects downstream outlet routing logic.
- Proposed disposition: amend

### IMP-B-003
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:237`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:258-261`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:282`
- Issue: Compatibility policy for `jpond > npond` includes deterministic surplus-ignore behavior, but no explicit guard output payload/warning surface is defined to preserve auditability of truncated records.
- Why it matters: Compatibility behavior that drops records should be observable and machine-checkable to avoid silent data loss in integration workflows.
- Proposed disposition: amend

## Final recommendation
GO-WITH-AMENDMENTS
