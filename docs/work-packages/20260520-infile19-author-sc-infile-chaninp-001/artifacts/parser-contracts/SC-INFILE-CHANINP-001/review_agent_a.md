Evidence: Static

## Findings (Severity-Ranked)

### CHAN-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:89`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:90`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:162`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:197`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:198`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:125`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:134`
- Issue: Guards and constraints require topology closure (`nchan`, valid channel/structure ID set), but these dependency surfaces are not explicitly modeled in the field table or propagation map.
- Why it matters: Cross-file guard enforcement is not fully executable without explicit dependency surfaces; this is a parser-contract completeness defect on required topology coupling.
- Proposed disposition: amend

### CHAN-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:90`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:148`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:198`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md:155`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/chaninp.spec.md:157`
- Issue: The contract states compat behavior can retain unknown `ichnum` IDs with warning, but guard/error wiring for `G-CHN-008` is strict-error-only (`CHN-E-005`) and no dedicated warning path is defined for unknown-ID retention.
- Why it matters: Strict-vs-compat behavior is not executable as written; parser outcomes for invalid IDs can drift across implementations.
- Proposed disposition: amend

Final recommendation: HOLD
