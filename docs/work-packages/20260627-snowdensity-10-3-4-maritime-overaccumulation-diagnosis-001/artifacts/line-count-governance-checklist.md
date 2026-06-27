# Line-Count Governance Checklist

Evidence mode: Static.

## Counts

| File | Lines | Disposition |
|---|---:|---|
| `tools/snowfreeze_observed/maritime_overaccumulation_diagnosis.py` | 748 | Accepted for a package-local diagnostic runner/report generator. |
| `tests/integration/snowdensity10_3_4_maritime_overaccumulation_diagnosis.rs` | 96 | Acceptable focused guard test. |
| `package.md` | 114 | Acceptable package scope and closeout summary. |
| `artifacts/maritime_overaccumulation_diagnosis.md` | 45 | Acceptable summary artifact. |
| `artifacts/maritime_overaccumulation_diagnosis.json` | 1149 | Generated machine-readable evidence artifact. |

## Assessment

The only large handwritten file is the diagnostic tool. It is intentionally
package-local, has no production coupling, and combines snowbench orchestration,
observation pairing, mechanism classification, and report emission for this
single diagnostic gate. No shared helper extraction is required before closure.

The large JSON artifact is generated evidence and is retained to make the
diagnostic result reproducible without rerunning the tool.
