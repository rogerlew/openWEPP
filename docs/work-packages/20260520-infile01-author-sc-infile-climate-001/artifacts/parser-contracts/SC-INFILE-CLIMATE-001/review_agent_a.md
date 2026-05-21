# Review Agent A — SC-INFILE-CLIMATE-001

Evidence: Static

## Findings (severity-ranked)

### CLI-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:72`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:199`
- Issue: Contract omits a boundary-export section that specifies which parsed/derived fields cross process boundaries and how names/units map across those interfaces.
- Why it matters: `parser-contract-requirements.md` requires explicit boundary export requirements; without this, parser-to-interface propagation is underspecified and integration behavior can drift.
- Proposed disposition: amend

### CLI-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:55`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:74`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:102`
- Issue: Source grammar includes optional `generator_cmd` metadata token, but field-spec and propagation tables do not define whether/how it is preserved, normalized, or dropped.
- Why it matters: The two-layer model contract requires explicit source-vs-simulation handling for parsed fields; missing treatment of source tokens weakens parse fidelity and reproducibility.
- Proposed disposition: amend

### CLI-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:97`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:178`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:197`
- Issue: Breakpoint cardinality is explicitly carried as unresolved (`CLI-GAP-003`), but no provisional strict/compat guard rule is defined for over-limit breakpoint counts.
- Why it matters: Parser guard coverage is incomplete for a core branching surface; implementations can diverge on acceptance/rejection for high-cardinality breakpoint days.
- Proposed disposition: amend

## Final recommendation
HOLD
