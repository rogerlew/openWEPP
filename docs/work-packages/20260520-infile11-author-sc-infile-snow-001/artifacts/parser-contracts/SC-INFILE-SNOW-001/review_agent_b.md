# Review Agent B — SC-INFILE-SNOW-001

Evidence: Static

## Findings (severity-ranked)

### SNOW-B-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:118`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:146-154`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:165`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/snow.spec.md:116-117`
- Issue: Strict policy says trailing tokens/surplus records are rejected, but taxonomy defines `SNOW-E-002` only for `<3` records; Section 11 maps surplus/trailing policy through `SNOW-E-002`, creating an error-class mismatch for `>3`/trailing-token strict failures.
- Why it matters: Typed error precision is required for executable strict-vs-compat behavior; current mapping leaves strict surplus/trailing rejection under-specified.
- Proposed disposition: amend

### SNOW-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:130-133`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SNOW-001.md:160-165`
- Issue: Cross-file invariants (management initial-state coupling, pre-winter availability, and alias-lossless mapping) are declared but have no explicit guard linkage in Section 11.
- Why it matters: Procedure requires each invariant/rule to map to an enforcement path or governance hold; missing guard linkage weakens closure verification for cross-surface correctness.
- Proposed disposition: amend

## Final recommendation
HOLD
