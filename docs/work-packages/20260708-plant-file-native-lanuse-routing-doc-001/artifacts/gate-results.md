# Gate Results

Status: executed.
Evidence mode: Static + Ran.

| Gate | Status | Evidence |
| --- | --- | --- |
| Worktree status recorded | PASS | `command-evidence.md` records dirty worktree and unrelated pre-existing changes. |
| Static source cross-check | PASS | `rg` check found matching authority in parser contract, lanuse authority contract, parser code, tests, and fixtures. |
| `git diff --check` | PASS | Exit code `0`. |
| Markdown lint: target spec | PASS | `markdown-doc lint --path docs/specifications/wepp-input-files/specs/plant-file.spec.md --format json`: `0` errors, `0` warnings. |
| Markdown lint: package directory | PASS | `markdown-doc lint --path docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001 --format json`: `19` files, `0` errors, `0` warnings. |
| Markdown lint: work-package catalog | PASS | `markdown-doc lint --path docs/work-packages/README.md --format json`: `0` errors, `0` warnings. |
| Dual review | PASS | `review-agent-a.md`, `review-agent-b.md`; no findings. |
| Dual verification | PASS | `verification-agent-a.md`, `verification-agent-b.md`; no findings. |
| Finding disposition | PASS | No findings remained undispositioned. |
| `.rs` line-count governance | PASS | Not triggered; no Rust files edited by this package. |
| Anti-evasion guards | PASS | Not triggered; no required-case binding, cohort fixture, external-authority suite posture, or `SC-*` file edited by this package. |
