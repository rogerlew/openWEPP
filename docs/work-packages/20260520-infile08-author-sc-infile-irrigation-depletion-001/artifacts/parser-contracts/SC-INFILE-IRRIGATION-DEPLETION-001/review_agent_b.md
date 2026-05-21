# Review Agent B — SC-INFILE-IRRIGATION-DEPLETION-001

Evidence: Static

## Findings (severity-ranked)

### IRD-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-depletion-file.spec.md:165`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:143-150`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:160-170`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:176-185`
- Issue: Legacy-observed furrow behavior that disables irrigation for contour/non-cropland configurations is not codified in cross-file constraints, compatibility policy, or guard mapping.
- Why it matters: This branch can materially change irrigation activation semantics; omission risks behavioral divergence from documented legacy behavior and ambiguous strict/compat handling.
- Proposed disposition: amend

### IRD-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:93`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:98-106`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md:45-60`
- Issue: Field table includes derived `initialization_complete`, but the propagation map does not explicitly map this field to runtime state/ownership.
- Why it matters: Propagation completeness is required for externally relevant fields; missing mapping weakens executable contract closure and observability for initialization validation.
- Proposed disposition: amend

### IRD-B-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:120`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-DEPLETION-001.md:151-159`
- Issue: `D-IRD-003` defines derived continuation ordering-key observability metadata, but no explicit field-spec row or boundary export mapping is provided for that metadata.
- Why it matters: Runtime scheduler diagnostics and deterministic continuation validation depend on this derived artifact being explicitly represented at interfaces.
- Proposed disposition: amend

## Final recommendation
HOLD
