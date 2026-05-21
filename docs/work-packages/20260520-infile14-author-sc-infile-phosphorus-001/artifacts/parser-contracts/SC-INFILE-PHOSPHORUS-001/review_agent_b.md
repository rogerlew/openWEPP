# Review Agent B — SC-INFILE-PHOSPHORUS-001

Evidence: Static

## Findings (severity-ranked)

### PHOS-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:91-94`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:179`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:136`
- Issue: Propagation rows for `srp/slfp/bfp/scp` do not include domain guard `G-PHOS-003` even though negative-concentration rejection (`PHOS-E-004`) is part of the required invariant set.
- Why it matters: Guard-linkage completeness is a hard parser-contract requirement. Missing field-to-guard linkage creates implementation ambiguity on where non-negative enforcement is guaranteed.
- Proposed disposition: amend

### PHOS-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:85`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:100`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:157-160`
- Issue: The contract groups `tmpsrp/tmpslfp/tmpbfp/tmpscp` into a single mixed-units row in both field and propagation tables, which hides per-field mapping and unit fidelity.
- Why it matters: Review focus requires no hidden grouped omissions for externally relevant fields. Grouping these four canonical symbols obscures per-field propagation correctness (especially `mg/L` vs `mg/kg`) and weakens boundary mapping auditability.
- Proposed disposition: amend

## Final recommendation
HOLD
