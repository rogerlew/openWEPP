# Review Agent B — SC-INFILE-IRRIGATION-FIXEDDATE-001

Evidence: Static

## Findings (severity-ranked)

### FDIR-B-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:158`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:173`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:204-215`
- Issue: The contract defines strict-mode contour/non-cropland furrow rejection (`FDIR-E-009`) and compat disable-warning behavior (`FDIR-W-005`), but Section 11 has no guard that maps the strict branch to `FDIR-E-009`.
- Why it matters: Parser-contract procedure requires every invariant/rule to have an explicit guard path. Without strict-path guard linkage, implementations can diverge on whether this is enforced as hard failure vs compatibility-only behavior.
- Proposed disposition: amend

### FDIR-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:43`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:201`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:35-38`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:227-231`
- Issue: The paired spec keeps fixed-date datver-floor enforcement as unresolved/HOLD (commented-out `verchk` path), but this contract hard-codes strict/compat datver acceptance windows as settled policy without carrying that unresolved authority conflict into the contract HOLD register.
- Why it matters: Correctness-over-completion posture requires unresolved authority conflicts to remain explicit in contract governance state; otherwise parser behavior may be promoted as settled before source conflict disposition is complete.
- Proposed disposition: amend

## Final recommendation
HOLD
