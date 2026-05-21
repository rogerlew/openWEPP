# Review Agent B — SC-INFILE-FROST-001

Evidence: Static

## Findings (severity-ranked)

### FROST-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:38`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:48-51`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:150-153`
- Issue: Section 2 grammar makes `line2` optional (`frost_file = line1 [line2]`), but strict-mode policy and applicability matrix require strict rejection when line2 is missing.
- Why it matters: This is a contract-internal executable inconsistency; parser implementations can legitimately diverge depending on whether they follow grammar vs policy text.
- Proposed disposition: amend

### FROST-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:122`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:133`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-FROST-001.md:167`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/frost.spec.md:133-135`
- Issue: Strict missing-line2 behavior is described as record-count/arity failure in the paired spec, but Section 11 routes the line2-optional branch through `G-FROST-005` with runtime-guard/warning outcomes (`FROST-E-005`/`FROST-W-002`) rather than an explicit strict syntax/record-count failure path.
- Why it matters: Guard taxonomy precision is required for deterministic strict-mode behavior; current mapping mixes compatibility provenance and strict syntax rejection semantics.
- Proposed disposition: amend

## Final recommendation
HOLD
