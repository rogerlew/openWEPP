Evidence: Static

## Findings (Severity-Ranked)

### IRD-A-001 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:33`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:160`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:42`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:45`
- Issue: Datver policy remains non-executable: contract uses “policy-gated”/compat wording but does not define explicit numeric strict allowlist/range and compat acceptance set.
- Why it matters: Parser implementations cannot deterministically enforce unsupported/compat behavior without a concrete datver acceptance matrix.
- Proposed disposition: amend

### IRD-A-002 — High
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:71`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:98`
- Issue: Externally relevant period fields are individually specified, but propagation is grouped into one generic period-payload row instead of explicit per-field propagation rows.
- Why it matters: This fails per-field propagation traceability and makes guard/consumer ownership opaque for scheduler-critical symbols.
- Proposed disposition: amend

### IRD-A-003 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:167`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:129`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:172`
- Issue: Compatibility behaviors (no-datver acceptance, nozzle default injection, `depsrg` remap/clamp) require explicit warning observability, but warning-class outcomes are not represented in taxonomy/guard failure mapping.
- Why it matters: Strict-vs-compat execution will diverge across implementations without typed, contract-level compatibility outcomes.
- Proposed disposition: amend

### IRD-A-004 — Medium
- File: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:125`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:114`
- Issue: Spec-level sentinel behavior for `irbeg==0` schedule-state transitions is not captured as an explicit derived rule/guard in the parser contract.
- Why it matters: This branch affects continuation-stream semantics and must be contract-visible to avoid hidden runtime-only behavior.
- Proposed disposition: amend

Final recommendation: HOLD
