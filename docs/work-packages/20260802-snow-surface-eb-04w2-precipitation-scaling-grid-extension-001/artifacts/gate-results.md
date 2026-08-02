# Gate Results

Status: `PASS / TERMINAL`

Evidence mode: **Ran + Static**.

| Gate | Result |
|---|---|
| prospective freeze/tool identity | `PASS`; freeze `c8dee2c...84b5`, tool `760a410...0b55` unchanged |
| transformer self-check and Python compilation | `PASS` |
| real-fixture transformation preflight | `PASS`; 20 cells, max residual `5.684e-14 mm`, zero protected mismatches |
| extension execution | `PASS`; 20/20 unique cells, all return code zero |
| retained anchors | `PASS`; 24 cells and 144 output identities |
| combined analysis | `PASS`; 44 unique cells and four lanes |
| conservation/diagnostic closure | `PASS`; max `4.441e-15 m` versus `1e-12 m` |
| frozen selection reconstruction | `PASS`; producer plus two independent reviewers and two terminal verifiers agree |
| SVG XML parse | `PASS`; four figures |
| visual figure inspection | `PASS`; no clipping, overlap, or obstructed legend observed |
| package Markdown | `PASS`; 28 files, zero errors/warnings |
| roadmap/catalog Markdown | `PASS`; three files, zero errors/warnings |
| trailing whitespace and diff check | `PASS` |
| Rust/workspace suites | `NOT_APPLICABLE`; no production, contract, manifest, fixture, observation, or test changes |
| Python lint | `NOT_RUN`; `ruff` unavailable, nonblocking after compilation, self-test, preflight, and real execution |
| dual independent review | `PASS`; no findings |
| review disposition | `PASS`; closure admitted |
| execution prompt lifecycle | `PASS`; archived, with no active execution prompt |
| dual terminal verification | `PASS`; all ten acceptance criteria independently verified |

All selected gates pass on the terminal tree. Rust/workspace suites remain
not applicable because the terminal diff contains no production or test code.
