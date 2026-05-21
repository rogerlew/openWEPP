# Review Agent A — SPEC-INFILE-PMETPARA-001

Evidence: Static

## Findings (severity-ranked)

### PMET-A-001
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:172`
- Issue: Gap/conflict register rows do not include explicit provenance tags (`usersum2024`, `legacy-code`, `wepppy`, `wepppyo3`, `literature`).
- Why it matters: Provenance-tagged conflicts are required for consistent source-authority disposition.
- Proposed disposition: amend

### PMET-A-002
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:66`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:73`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:175`
- Issue: Deterministic crop-key matching policy is not explicit (case handling, trimming, and legacy `character*8` truncation semantics), despite these behaviors directly controlling fallback-to-first-row outcomes.
- Why it matters: Parser/runtime behavior can mis-map ET coefficients to crops if key normalization/truncation is not contractually fixed.
- Proposed disposition: amend

### PMET-A-003
- Severity: medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:39`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/pmetpara.spec.md:113`
- Issue: Version/datver-prefixed variant is marked `MUST reject`, but no corresponding typed error condition is declared in the defaulting/error table.
- Why it matters: Rejection branches should map to explicit typed error surfaces for executable parser contracts.
- Proposed disposition: amend

## Final recommendation
GO-WITH-AMENDMENTS
