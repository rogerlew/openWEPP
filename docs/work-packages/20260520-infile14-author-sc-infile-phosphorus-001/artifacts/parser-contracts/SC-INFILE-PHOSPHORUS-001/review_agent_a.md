Evidence: Static

## Findings (Severity-Ranked)

### PHOS-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:85`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:100`
- Issue: Externally relevant fanout symbols (`tmpsrp`, `tmpslfp`, `tmpbfp`, `tmpscp`) are grouped into a single field and a single propagation row instead of symbol-level rows.
- Why it matters: Parser-contract requirements call for per-field propagation coverage; grouped rows obscure ownership/guard linkage and weaken executable traceability.
- Proposed disposition: amend

### PHOS-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:134`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:183`
- Issue: `G-PHOS-007` maps strict trailing-token rejection to `PHOS-E-002` (record-count mismatch), conflating token-policy and record-closure failures.
- Why it matters: Error taxonomy precision is required for deterministic enforcement and verification.
- Proposed disposition: amend

### PHOS-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:39`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:52`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md:74`
- Issue: Header literal policy is normative (strict mismatch error) but the field table has no explicit source/simulation field for header text itself; only a derived match flag is modeled.
- Why it matters: A policy-gating surface should have explicit model representation to make parser behavior auditable and reproducible.
- Proposed disposition: amend

Final recommendation: HOLD
