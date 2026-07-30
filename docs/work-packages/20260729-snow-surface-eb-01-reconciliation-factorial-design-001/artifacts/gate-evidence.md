# Gate Evidence

Status: `terminal PASS`.

Evidence class: Ran + Static.

| Gate | Result | Evidence |
| --- | --- | --- |
| Deterministic generation | `PASS` | `.venv/bin/python .../tools/generate.py`, then package validator `--check` |
| CSV parse/shape | `PASS` | 14 CSVs parsed; rectangular header/data shape |
| SVG accessibility | `PASS` | Three XML-parsed SVGs; each has one title, one description, and `role="img"` |
| Figure sidecars | `PASS` | Three exact stem pairs |
| Package-local Markdown links | `PASS` | Package validator |
| Package Markdown lint | `PASS` | `markdown-doc lint --path <package>`: zero errors/warnings |
| Package Markdown schema | `PASS` | `markdown-doc validate --path <package>`: zero errors |
| Roadmap/catalog Markdown | `PASS` | Scoped lint and validate on campaign roadmap, `docs/ROADMAP.md`, and catalog |
| American spelling preview | `PASS` after correction | `uk2us` preview identified and corrected two British unit spellings |
| Whitespace | `PASS` | `git diff --check` |
| Rust/workspace gates | `NOT APPLICABLE` | Terminal write set is documentation and package-local analysis only |

The figures were additionally rasterized with `rsvg-convert` and inspected
visually. Labels, legends, marks, and categorical encoding are legible; no
prose is embedded in the plots.

After review correction, the exact terminal command set above was rerun. Both
reviewers independently reran the package validator and diff check and issued
`PASS`.
