# Gate Results

Status: `scaffold intake passed`

Evidence mode: `Ran`

Append every attempt; do not replace failed evidence with only a later pass.

## Scaffold and Intake

| Command | Result |
| --- | --- |
| `git rev-parse HEAD` | PASS: `4f5bb1c599a683b63be56ecd9e7296f8faf01ed0`. |
| `git status --short --branch` | PASS: clean `main...origin/main` before scaffold. |
| `tools/agents/find-agents --for ...` | PASS: instruction chains recorded in `required-reading-map.md`. |
| `sha256sum` over frozen V1 and all three V2 definition copies | PASS: V1 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`; every V2 copy `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`. |
| `cmp -s` across all three V2 definition copies | PASS: byte-identical. |
| `markdown-doc lint docs/...` | FAIL: CLI rejects positional paths; retained as invocation-shape evidence. |
| `markdown-doc lint --path docs/work-packages/20260812-c3-woody-potential-pass-authority-001` | PASS: 24 files, 0 errors, 0 warnings. |
| `markdown-doc lint --path docs/work-packages/README.md` | PASS: 1 file, 0 errors, 0 warnings. |
| `git diff --check` | PASS. |
