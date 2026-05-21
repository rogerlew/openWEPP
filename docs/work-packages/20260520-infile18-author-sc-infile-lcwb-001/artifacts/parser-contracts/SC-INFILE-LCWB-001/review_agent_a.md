Evidence: Static

## Findings (Severity-Ranked)

### LCWB-A-001 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:76`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:125`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:153`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:168`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:59`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:140`
- Issue: Strict payload policy is implemented as non-empty-byte rejection (`payload_nonempty = payload_bytes > 0`), which rejects whitespace-only sentinel bodies, while the paired spec currently defines strict acceptance as empty/whitespace-only.
- Why it matters: Contract/spec mismatch on strict payload semantics causes deterministic parser divergence and undermines authority ordering.
- Proposed disposition: amend

### LCWB-A-002 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:81`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:96`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:138`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:188`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:147`
- Issue: `ofe_row_selection_mode` is specified as a hard runtime-closure surface despite the paired spec explicitly marking active-source consumer closure for `lcwbflg` as unresolved (`LCWB-GAP-002`).
- Why it matters: This over-commits unresolved behavior as normative runtime contract and weakens evidence-tag rigor for executable semantics.
- Proposed disposition: amend

Final recommendation: HOLD
