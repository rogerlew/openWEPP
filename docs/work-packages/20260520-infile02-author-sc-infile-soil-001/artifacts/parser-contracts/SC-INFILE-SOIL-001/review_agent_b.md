# Review Agent B — SC-INFILE-SOIL-001

Evidence: Static

## Findings (severity-ranked)

### SOL-B1
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:71`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:93`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md:168`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/soil-file.spec.md:180`
- Issue: The field specification table omits required 9002+/9003+/9005 layer fields documented in the paired spec (`theta_r`, `theta_s`, `alpha`, `npar`, `ks`, and appended Rosetta `wp`/`fc` values).
- Why it matters: Parser-contract field coverage is incomplete for an active datver family; this violates required field-table completeness and leaves parse-to-simulation semantics undefined for those tokens.
- Proposed disposition: amend

### SOL-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:96`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:106`
- Issue: Propagation map lacks rows for the omitted 9002+ extended hydraulic/pedotransfer fields.
- Why it matters: Without propagation rows, ownership, phase, mutability, and guard linkage for these parsed values are unspecified.
- Proposed disposition: amend

### SOL-B3
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:150`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md:113`
- Issue: No explicit boundary-export requirements section is defined.
- Why it matters: Cross-process field/name/unit mapping is a normative parser-contract requirement and is currently undocumented for soil surfaces.
- Proposed disposition: amend

## Final recommendation
HOLD
