Evidence: Static

## Findings (Severity-Ranked)

### IMP-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:61`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:70`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:73`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-impoundment-file.spec.md:91`
- Issue: Contract states source model preserves branch-local payload and section order, but field table omits branch comment records (`strdes`) that are part of multiple conditional sections in the paired spec/legacy grammar.
- Why it matters: Missing source-model fields break file-faithful representation and undermine parser completeness/round-trip expectations for optional-branch structure.
- Proposed disposition: amend

### IMP-A-002 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:70`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:186`
- Issue: The contract enumerates a large set of externally relevant symbols in the field table, but propagation is collapsed into section-level grouped rows rather than per-field propagation rows.
- Why it matters: This violates per-field propagation requirements and prevents symbol-level closure checks for ownership, mutability, and guard linkage.
- Proposed disposition: amend

### IMP-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:238`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:265`
- Issue: Cross-file ordering constraint (impoundment ordering consistent with structural indexing) is declared, but no explicit guard row maps this ordering invariant to a dedicated enforcement path.
- Why it matters: Ordering mismatches can silently corrupt topology joins; without explicit guard mapping, this invariant is not verifiable at parser-contract level.
- Proposed disposition: amend

### IMP-A-004 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:237`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:219`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md:251`
- Issue: Compatibility mode allows `jpond > npond` with warning/truncation behavior, but taxonomy/guard outcomes only model mismatch as errors and do not define explicit compatibility warning-class outcomes.
- Why it matters: Compatibility behavior is not fully precise or machine-checkable, which weakens deterministic implementation and auditability.
- Proposed disposition: amend

Final recommendation: HOLD
