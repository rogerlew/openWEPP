# Review Agent B — SC-INFILE-LCWB-001

Evidence: Static

## Findings (severity-ranked)

### LCWB-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:81`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:96`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:138`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:188-190`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:32`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:147`
- Issue: The contract hard-codes `ofe_row_selection_mode` semantics (`last_ofe_only` vs `all_ofe`) as deterministic active behavior even though the paired spec marks active-source consumer closure unresolved (`LCWB-GAP-002`) and treats this as historical compatibility provenance.
- Why it matters: This over-commits unresolved behavior and weakens correctness-over-completion governance. Until consumer authority is closed, this mapping should remain explicitly provisional/HOLD-gated rather than normative runtime closure.
- Proposed disposition: amend

### LCWB-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:41`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:127`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md:170`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/lcwb.spec.md:40-44`
- Issue: For non-watershed context, the matrix describes a typed context-inapplicable outcome, but taxonomy/guards model only hard failure (`LCWB-E-002`) with no explicit non-error typed not-applicable branch semantics.
- Why it matters: Applicability behavior becomes ambiguous across implementations (error vs typed no-op outcome), which undermines executable parser-contract precision for branch closure and observability.
- Proposed disposition: amend

## Final recommendation
HOLD
