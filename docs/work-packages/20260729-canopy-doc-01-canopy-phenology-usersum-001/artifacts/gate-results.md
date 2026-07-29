# Validation Results

Status: `complete`

Evidence mode: `Ran`

| Gate | Result | Terminal evidence |
| --- | --- | --- |
| usersum Markdown | `PASS` | `markdown-doc lint --path usersum`: 12 files, 0 errors/warnings |
| package Markdown | `PASS` | `markdown-doc lint --path docs/work-packages/20260729-canopy-doc-01-canopy-phenology-usersum-001`: 20 files, 0 errors/warnings |
| roadmap Markdown | `PASS` | `markdown-doc lint --path docs/planning/canopy-phenology-assurance-roadmap.md`: 1 file, 0 errors/warnings |
| work-package catalog Markdown | `PASS` | `markdown-doc lint --path docs/work-packages/README.md`: 1 file, 0 errors/warnings |
| changed Markdown links | `PASS` | Repository-relative target checker parsed changed Markdown; no missing target |
| usersum boundary | `PASS` | Negative check found no link from the narrative into `docs/`, `crates/`, an absolute path, or a parent path |
| spelling preview | `PASS` | `diff -u usersum/openwepp-canopy-phenology.md <(uk2us usersum/openwepp-canopy-phenology.md)` proposed no change |
| coefficient inventory | `PASS` | Python CSV audit: 21 rows × 21 columns; exact expected fields; every mandatory cell populated; range classes allowed |
| version/revision and audience | `PASS` | Version 1.0 and 2026-07-29 match the final revision row; audience line present |
| dual review | `PASS` | Reviews A and B completed; nine findings dispositioned and corrected |
| dual verification | `PASS` | Verifications A and B close every finding and report none remaining |
| Rust/source impact | `PASS / NOT APPLICABLE` | No `.rs`, schema, contract, prior evidence, assurance publication, or release surface changed |
| diff hygiene | `PASS` | `git diff --check` |

The initial spelling invocation used an unsupported `--check` option. It was a
command-shape error, not a content failure; the documented `diff -u` preview
then ran successfully and is the retained gate.
