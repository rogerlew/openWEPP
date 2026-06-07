# Kernel Profile Compliance Checklist

Evidence mode: `Static:` and `Ran:`.

| Requirement | Status | Evidence |
|---|---|---|
| Canonical contract authority updated before closure claim | pass | `SC-SOIL-001` v23, `INV-SOIL-017`. |
| No heuristic/proxy process physics | pass | Change is overlap mapping for existing corrected lineage, not new physics. |
| No silent defaults or guard loosening | pass | Nonmonotone parser-layer regression still fails closed. |
| Protected boundaries respected | pass | No frost, ET, runoff, snow, or hydrology-kernel files edited. |
| Hydrology seed-grid authority preserved | pass | `INV-SOIL-015` remains governing WB11/WB18/WB19 seed aliases. |
| Validation truthfulness labeled | pass | Artifacts distinguish `Static:` and `Ran:` evidence. |
| Downstream violations not normalized | pass | `p11` remains fail-closed on `HKERNEL-WB11-PERC-E-003` and is handed off. |
