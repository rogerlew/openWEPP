# Review Agent B — SC-INFILE-WATERSHED-STRUCTURE-001

Evidence: Static

## Findings (severity-ranked)

### WST-B-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:46`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:52`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:114-127`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-structure-file.spec.md:71`
- Issue: The contract defines `structure_row{n_rows}` and states `n_rows` is resolved from cross-file topology, but it does not define an explicit guard/error for file-level row-count closure (`exactly 1 + n_rows` logical records) under strict mode.
- Why it matters: Parser acceptance can diverge on surplus/deficit row behavior if EOF/extra-row handling is not explicit, weakening deterministic parse behavior and cross-file closure reproducibility.
- Proposed disposition: amend

### WST-B-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:76`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:103`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:134`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:172`
- Issue: `nhill` is used as a required derivation/cross-file input (`element_id = nhill + record_index` and hillslope coverage closure), but `nhill` has no explicit parser-contract propagation/ownership row.
- Why it matters: Required external dependencies should have explicit ownership, phase, and guard mapping; otherwise implementations may disagree on authority source and validation timing.
- Proposed disposition: amend

## Final recommendation
GO-WITH-AMENDMENTS
