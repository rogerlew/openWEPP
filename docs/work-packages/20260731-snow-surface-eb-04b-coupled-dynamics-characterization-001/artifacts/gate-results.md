# Gate Results

Evidence: `Ran`

| Gate | Result |
| --- | --- |
| Analysis compile/import and anti-alias self-check | PASS |
| Full retained-input analysis | PASS — `24/24` classified |
| Independent thermal reconstruction | PASS — `22/22`, maximum residual `0.0 degC` |
| Independent geometry reconstruction | PASS — `2/2` |
| Prospective signature disposition | PASS — six of six dispositioned |
| Source operation-ordering audit | PASS |
| Exact EB-04/04A trace and report identities | PASS |
| Executable and executable-source-diff identity | PASS |
| Deterministic regeneration | PASS — 16 generated JSON, CSV/GZIP, SVG, and sidecar hashes identical across consecutive runs |
| Complete chronology retention | PASS — 83,232 data rows; exact 24-row frozen input manifest |
| Quantitative acceptance predicates and negative self-check | PASS — all eight required predicates true; each forced-false predicate rejects acceptance |
| Package Markdown lint | PASS — 37 terminal files, zero errors/warnings |
| Roadmap/catalog Markdown lint | PASS |
| Figure/sidecar inventory and SVG parse | PASS — `5/5` |
| Visual inspection | PASS — all five figures; geometry uses labeled points and rejected chronology slices are visually separate |
| Dual independent review | PASS / PASS after all findings were corrected and dispositioned |
| Dual independent terminal verification | PASS / PASS after two stale evidence/catalog counts were corrected and rechecked |
| `git diff --check` | PASS |
| Rust validation reuse | VALID — executable-source diff remains exactly EB-04A's fully validated identity |

No Rust, test, fixture, contract, dependency, or runtime-publication gate is
selected because EB-04B changes none of those surfaces. Under the canonical
testing strategy, documentation-only analysis runs affected documentation,
schema, regeneration, reference, and exact-identity checks rather than
unrelated Rust profiles.
