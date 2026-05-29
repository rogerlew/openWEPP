# HILLSTAB08 WB16 Ealpha Gap Matrix

Status: complete  
Evidence mode: mixed (`Static` + `Ran`)

| Gap row | Status | Evidence | Notes |
|---|---|---|---|
| Producer-symbol projection ownership | closed | `[DIRECT][Static]` | Management/runtime projection now publishes required WB16 producer controls and seed aliases. |
| `frcfac` lineage runtime implementation | closed | `[DIRECT][Static] + [Ran]` | Runner producer computes OFE friction-equivalent lineage and publishes `ofe{n}_frcteq`. |
| `rdat(alpha)` lineage runtime implementation | closed | `[DIRECT][Static] + [Ran]` | Runner producer computes/publishes `ofe{n}_alpha` using canonical `alpha = sqrt(8*g*S/frcteq)` and `m=1.5`. |
| `eplane` multi-OFE equivalent-plane lineage implementation | closed | `[DIRECT][Static] + [Ran]` | Runner producer computes multi-OFE `ealpha` via baseline-authoritative equivalent-plane transformation. |
| Single-OFE and multi-OFE contract-derived parity vectors | closed | `[DIRECT][Static] + [Ran]` | Added/ran dedicated unit vectors for single-OFE and multi-OFE producer behavior, plus CLI provenance vector update. |
