# hillstab05-slope-remediation-report

Status: complete  
Evidence mode: Ran

## Residual Decomposition (Input Set)
Residual slope-family counts carried into HILLSTAB05 from prior rerun analysis:
- slope token parse (`line 7, column 3`): `33`
- endpoint constraint branch: `24`
- cross-OFE boundary mismatch branch: `11`
- `HS-RUNTIME-E-023` derived-average-slope runtime failure: `46`

Representative cases:
- token parse: `WB05C-CO-H0111` (`p111.slp`)
- endpoint: watchlist `p182`
- cross-OFE mismatch: `WB05C-CO-H0516` (`p516.slp`)
- runtime avg slope: `WB05C-OR-H0198` (`p198.slp`)

## Root Cause
1. Compatibility slope parsing did not fully match baseline legacy closure
   semantics for near-terminal endpoint tolerance and cross-OFE continuity.
2. Compatibility parser-to-runtime handoff treated non-positive derived `avgslp`
   as hard failure instead of applying baseline-authoritative floor behavior.

## Remediation Applied
- Contract authority updates in `SC-INFILE-SLOPE-001.md` (`v0.1.4`) declared:
  - compatibility endpoint tolerance `1e-3`,
  - strict-only cross-OFE mismatch hard-fail branch,
  - compatibility `avgslp` floor `0.000001`.
- Parser/runtime/runner updates implemented these branches directly and added
  explicit runtime floor-applied flags.

## Post-Remediation Evidence
From `/tmp/hillstab05/**/logs/stderr.log` and rerun JSON:
- slope token parse (`line 7, column 3`): `0`
- endpoint constraint: `0`
- cross-OFE boundary mismatch: `0`
- `HS-RUNTIME-E-023`: `0`

Representative closure checks:
- `WB05C-CO-H0111`: pass (`true`)
- `WB05C-CO-H0516`: pass (`true`)
- watchlist `p182`: pass (`true`)
- `WB05C-OR-H0198`: no longer slope-runtime failure; now fails downstream as
  `HKERNEL-WB16-PEAK-E-003`.

## Conclusion
- Targeted slope residual families are closed.
- Residual hold-lift blockers are now primarily downstream runtime families
  outside this package scope.
