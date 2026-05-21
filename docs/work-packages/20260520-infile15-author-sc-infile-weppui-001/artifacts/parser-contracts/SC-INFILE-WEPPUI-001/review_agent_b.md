# Review Agent B — SC-INFILE-WEPPUI-001

Evidence: Static

## Findings (severity-ranked)

### WUI-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:39`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:69`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:111`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:143-149`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:160`
- Issue: The contract requires strict-mode typed failure for non-ENOENT open errors (`WUI-E-000`), but `ui_run` defaulting still encodes open-fail => `0` in the core field derivation, effectively collapsing strict IO faults into daily mode semantics.
- Why it matters: This is a correctness-over-completion violation: strict-mode operational faults must not silently degrade mode selection. Current derivation text makes strict/compat behavior non-deterministic at implementation boundaries.
- Proposed disposition: amend

### WUI-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:67-75`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:128`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WEPPUI-001.md:132-137`
- Issue: Section 8 requires requested/effective mode divergence observability, but the field and boundary tables do not define explicit requested/effective mode surfaces (only `ui_run` and sentinel metadata).
- Why it matters: Without explicit divergence fields, closure of mode-selection invariants and branch-drift diagnostics is not executable, limiting verification of `watbal` vs `watbal_hourly` selection guarantees.
- Proposed disposition: amend

## Final recommendation
HOLD
