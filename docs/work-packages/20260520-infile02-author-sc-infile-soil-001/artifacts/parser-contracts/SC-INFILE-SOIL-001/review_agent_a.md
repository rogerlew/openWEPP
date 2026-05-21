# Review Agent A — SC-INFILE-SOIL-001

Evidence: Static

## Findings (severity-ranked)

### SOL-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:87`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:88`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:89`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md:168`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md:178`
- Issue: Field specification table collapses large datver-dependent layer payloads into tuples and does not provide per-field rows for several externally relevant 9002+/9005 fields (for example `theta_r`, `theta_s`, `alpha`, `npar`, `ks`, Rosetta `wp`/`fc`, etc.).
- Why it matters: Parser-contract requirements call for explicit per-field symbol/alias/unit/type/cardinality/default semantics for externally relevant fields; tuple compression leaves data-model and guard obligations underdefined.
- Proposed disposition: amend

### SOL-A-002
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:69`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:194`
- Issue: Contract has no boundary-export requirement section for soil fields crossing process boundaries.
- Why it matters: Missing boundary mapping violates parser-contract completeness and leaves interface name/unit propagation ambiguous across CLI/interchange/runtime boundaries.
- Proposed disposition: amend

### SOL-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:99`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:145`
- Issue: Cross-file consistency rule on `ntemp` includes both OFE and channel-scoped semantics, but propagation mapping does not disambiguate which topology source is authoritative per run mode.
- Why it matters: Source-vs-simulation model separation and cross-file closure depend on explicit mode-scoped topology ownership; ambiguity can create inconsistent validation behavior.
- Proposed disposition: amend

## Final recommendation
HOLD
