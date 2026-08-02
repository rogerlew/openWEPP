# Gate Results

Status: `PASS / TERMINAL`

Evidence mode: **Ran + Static**.

| Gate | Command / evidence | Result |
|---|---|---|
| prospective freeze identity | `sha256sum experiment-freeze.json` | `PASS`; `910e115...892` unchanged |
| frozen tool identity | `sha256sum tools/run_precipitation_scaling.py` | `PASS`; `5926c5e...346` unchanged |
| transformer self-check | package tool `--self-check` | `PASS` |
| Python compilation | `.venv/bin/python -m py_compile` with target-local cache | `PASS` |
| forcing transformation | 32 cell provenance records | `PASS`; max residual `2.842e-14 mm`, zero protected mismatches |
| execution inventory | receipt JSON query | `PASS`; `32 / 32`, all return code zero |
| baseline replay | operator reconstruction plus raw hashes | `PASS`; zero operator residual and eight byte-identical outputs |
| diagnostic conservation | results JSON query | `PASS`; maximum `3.331e-15 m` versus `1e-12 m` limit |
| result schema/inventory | `jq -e` queries | `PASS`; four lanes, 32 runs, no independent validation |
| figure parse | `xmllint --noout` on three SVGs | `PASS` |
| figure visual inspection | rasterized three SVGs and inspected | `PASS`; no clipping, overlap, or unreadable legend observed |
| package Markdown | `markdown-doc lint --path <package> --no-ignore` | `PASS`; 27 files, zero errors/warnings on terminal tree |
| roadmap/catalog Markdown | three scoped `markdown-doc lint` runs | `PASS`; zero errors/warnings |
| whitespace | `git diff --check` | `PASS` |
| Rust/workspace suites | validation selection | `NOT_APPLICABLE`; exact diff has no Rust, contract, manifest, fixture, observation, or test changes |
| Python lint | `ruff` discovery | `NOT_RUN`; command unavailable, nonblocking because compilation, self-test, and real execution pass |
| dual independent review | `review-agent-a.md`; `review-agent-b.md`; disposition | `PASS`; four claim/presentation findings corrected, no rerun required |
| dual terminal verification | `verification-agent-a.md`; `verification-agent-b.md` | `PASS`; exact corrected tree independently verified |
| prompt lifecycle | `prompts/active/`; `prompts/archived/execute.md` | `PASS`; no active execution prompt remains |

All selected current-scope gates pass. The unavailable optional Python linter
does not invalidate the compiled, self-tested, and fully executed package-local
tool.
