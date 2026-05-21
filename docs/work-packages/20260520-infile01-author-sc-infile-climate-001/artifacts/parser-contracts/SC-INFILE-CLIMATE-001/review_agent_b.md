# Review Agent B — SC-INFILE-CLIMATE-001

Evidence: Static

## Findings (severity-ranked)

### CLI-B1
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:148`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:155`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/parser-contract-requirements.md:113`
- Issue: The contract has no explicit boundary-export section defining which parsed/derived climate fields cross process boundaries (CLI args, parquet, HBP/interchange) and how names/units map.
- Why it matters: Boundary export mapping is a normative parser-contract requirement and is needed to prevent silent name/unit drift at interface surfaces.
- Proposed disposition: amend

### CLI-B2
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:102`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:110`
- Issue: Propagation-map phase entries are uniformly `init`, even for fields whose declared consumers are event/daily kernels (`runoff partition`, `watbal`, `snowfreeze`, `evap`).
- Why it matters: Phase granularity is required for propagation integrity; collapsing all rows to `init` weakens mutability/ownership tracing across simulation phases.
- Proposed disposition: amend

### CLI-B3
- Severity: low
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:22`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md:23`
- Issue: Legacy evidence anchors reference whole files without line-local anchors.
- Why it matters: Evidence traceability is weaker for verifier follow-up when anchors are not line-addressable.
- Proposed disposition: amend

## Final recommendation
HOLD
