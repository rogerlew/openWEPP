# QA-M3 gate results

Status: **EXECUTED** (2026-07-07). Evidence mode: **Ran** for checks below.

| Gate | Command | Result |
|---|---|---|
| Diff hygiene | `git diff --check` | PASS |
| Markdown lint | `markdown-doc lint --path docs/work-packages/AGENTS.md --path crates/AGENTS.md --path tools/local_ci/README.md --path docs/work-packages/README.md --path docs/work-packages/20260707-laned-router-t3agg-qa-m3-build-provenance-guidance-001 --no-ignore` | PASS |
| Static closure proof | `rg -n "cargo build --release -p openwepp-runner --bins|mtime|sha256sum|stale" docs/work-packages/AGENTS.md crates/AGENTS.md tools/local_ci/README.md` | PASS |

Not run:

- Cargo/Rust gates: not in scope; this package changes docs/process guidance
  only.
- H2637 timing/comparator gates: not in scope; this package promotes guidance
  for future evidence runs and does not create new timing evidence.
- Anti-evasion guards: not in scope; no external-authority suite posture,
  cohort fixture, or required-case binding was touched.
