# Review Agent A — SC-INFILE-SLOPE-001

Evidence: Static

## Findings (severity-ranked)

### SLP-A-001
- Severity: high
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:65`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:174`
- Issue: Contract lacks explicit boundary-export requirements for slope fields and derived profile state.
- Why it matters: Boundary mapping is a normative parser-contract requirement; omission leaves interface-level symbol/unit propagation ungoverned.
- Proposed disposition: amend

### SLP-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:39`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:133`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:140`
- Issue: Datver matrix includes an explicit compat candidate for older explicit datver forms, but compatibility policy does not explicitly codify accepted legacy range/version-gate behavior for that branch.
- Why it matters: Backward-compatibility policy requires explicit accepted legacy variants and version gates; missing codification can produce inconsistent handling of older explicit datver inputs.
- Proposed disposition: amend

### SLP-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:112`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md:114`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:132`
- Issue: Error taxonomy does not include a dedicated missing-file/open error class even though the paired spec defines explicit `InputFileMissing` behavior.
- Why it matters: Validation taxonomy should distinguish file-availability failures from record-shape failures; otherwise diagnostics and compat policy enforcement become ambiguous.
- Proposed disposition: amend

## Final recommendation
HOLD
