# Gate Evidence

Evidence class: `Ran`

| Gate | Result | Evidence |
| --- | --- | --- |
| Authority source checksums | `PASS` | `sha256sum -c SHA256SUMS` passed all 20 retained Harvard/Hubbard objects. |
| Corpus checksums | `PASS` | `sha256sum -c SHA256SUMS` passed README, records, source manifest, and metadata. |
| Package exact record-to-source joins | `PASS` | Six of six manifest paths exist and match their SHA-256. |
| CAL-04 deterministic extraction | `PASS` | Both tools compile; rebuild is byte-identical, 1,251 unique records, SHA-256 `890a0ff09ca707b097a15cb5de7964698a9b4d5af797ed6b81d5fccf7c141b61`. |
| CAL-04 roles and intervals | `PASS` | 932 Hubbard P3 calibration and 319 Harvard leaf-fall holdout records; roles/source objects disjoint, bounds ordered, no Harvard fall 1992, no observation beyond protected member year 2024. |
| CAL-05 deterministic matching | `PASS` | Byte-identical 28-plot rebuild, SHA-256 `13b62f088938841aa78d57b572e8e98174b34d72e28fb06600e4d9b5d4615b91`; keys, units, row counts, stock replicates, and `use.not=1` retained. |
| Corpus metadata join | `PASS` | Exact object identity join is 18/18. |
| License/terms and missing semantics | `PASS` | Original EML rights and missing codes reviewed independently; CC BY 4.0/CC0 dispositions match; no missing value becomes zero. |
| Scientific review | `PASS WITH HOLD` | Two independent reviews inspected original objects. All actionable findings were accepted and corrected; both require package `HOLD` because CAL-05 remains unresolved. |
| Terminal verification | `PASS` | Two independent verifiers reran exact-state source, extraction, join, lint, diff, authority, and write-set checks after corrections. |
| Documentation lint | `PASS` | The repository wrapper was mis-rooted and scanned zero files; direct canonical `markdown-doc lint --path` then validated 19 affected Markdown files with zero errors/warnings. |
| Diff hygiene/write set | `PASS` | An initial full check exposed source-native CRLF in downloaded CSVs. Directory-local `.gitattributes` now classifies retained CSV evidence as binary, preserving exact checksums; the full unqualified `git diff --cached --check` passes. Terminal names are within the declared write set; no production, fixture-input, parameter, contract, fitting, or physics change. |

An initial extractor determinism command incorrectly used an unsupported
`--output` option and returned argparse usage. Re-running with the documented
four positional arguments passed byte-identically; the invocation error did
not alter an artifact.

The documented `wctl doc-mv` helper likewise rejected the existing relative
openWEPP path and treated its absolute path as outside its configured project
root. The prompt was moved with an explicit patch after both helper attempts;
no reference rewrite was needed.
