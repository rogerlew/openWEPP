# Gate Evidence

Status: `terminal PASS`.

Evidence class: Ran + Static.

| Gate | Result | Evidence |
| --- | --- | --- |
| Deterministic generation | `PASS` | `.venv/bin/python .../tools/generate.py` |
| CSV parse/shape | `PASS` | Four ledgers parsed with rectangular nonblank rows; amended dimensions are source authority `10 x 6`, equation `13 x 7`, acquisition `4 x 5`, and operand readiness `15 x 6`, including headers |
| SVG accessibility | `PASS` | Two SVGs parse; each has one title, one description, and `role="img"` |
| Figure sidecars | `PASS` | Two exact same-stem pairs |
| Visual inspection | `PASS` | Both SVGs rasterized with `rsvg-convert`; axes, legend, labels, and status text are legible |
| Package Markdown | `PASS` | `markdown-doc lint` and `validate`: 23 files, zero errors or warnings |
| Roadmap and catalog Markdown | `PASS` | Scoped `markdown-doc lint` and `validate`: zero errors or warnings |
| American spelling preview | `PASS` | `uk2us` preview; source prose normalized without changing quoted titles |
| Whitespace | `PASS` | `git diff --check` |
| Dual review | `PASS` | Both corrected-tree reviewers pass; all findings dispositioned |
| Rust/workspace gates | `NOT APPLICABLE` | Terminal write set contains documentation and package-local analysis only |

The exact terminal command set was rerun after the original corrections and
again after the accepted derived-sky-view amendment corrections.
