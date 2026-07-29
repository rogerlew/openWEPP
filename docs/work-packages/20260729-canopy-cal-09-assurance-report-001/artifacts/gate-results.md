# Gate Results

Status: `EXECUTED-HOLD`

Evidence class: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| Strict-result reproduction and byte comparison | `PASS` | Fresh seven-input execution matched retained JSON |
| Candidate figure build and repeatability | `PASS` | Eight figures rebuilt; frozen hashes unchanged |
| SVG XML validation | `PASS` | Eight of eight parsed |
| Report/result/catalog JSON Schema | `PASS` | Direct Draft 2020-12 validation |
| Descriptor foreign-key, path, and exact-use closure | `PASS` | 39 objects, 32 values, 12 references, one table, one figure |
| Figure source-manifest hashes | `PASS` | Nine of nine repository-relative sources matched |
| Package Markdown lint | `PASS` | 40 files, zero errors or warnings |
| Report Markdown lint | `PASS` | Two files, zero errors or warnings |
| American-English idempotence | `PASS` | No conversion delta |
| Existing admitted V2 `validate --all` | `PASS` | Both existing reports validate after removing the unadmitted canopy catalog row |
| Existing admitted V2 `plan --all` | `PASS` | Both existing report targets are current |
| Canopy-specific canonical V2 `validate` | `BLOCKED` | Unknown report ID; no typed new-report admission |
| Canopy-specific canonical V2 `plan` | `BLOCKED` | Cannot select an unadmitted report |
| Canonical V2 normalize/build/check | `NOT RUN` | Cannot legitimately follow failed admission/validation |
| Human scientific approval | `NOT RUN` | No authenticated approver assigned |
| Human reproduction/publication approval | `NOT RUN` | No authenticated approver assigned |
| Release transfer and publication | `NOT RUN` | Draft source is not admitted or approved |

The unresolved canonical gate prevents a complete assurance disposition. It
does not invalidate the passing source-level reconstruction and review
evidence.
