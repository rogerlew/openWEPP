# Disposition

Evidence label: Static/Ran.

Status: `REVIEW-DISPOSITION-COMPLETE`

Implementation disposition:

- Behavior-preserving CQR landed for
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/direct.rs`.
- `direct_tests.rs` was added as a package-local test include to keep
  characterization local while resolving the 3000+ line-count blocker.
- No science formulas, thresholds, fail-closed guard classes, runtime-symbol
  names, serialization formats, or public output meanings were changed.

Metric disposition:

- Baseline deduplicated target functions above CRAP `30`: `7`.
- Final targeted CRAP functions above `30`: `0`.
- Final max target CRAP: `23.069544598035826`.
- Final targeted line coverage: `1782 / 1888 = 94.385593220339%`.
- Final targeted deduplicated source-span region coverage:
  `2123 / 2274 = 93.35971855760774%`.
- ADR-0021 science-tier thresholds pass, and every eligible production function
  with source-span regions is above the `75%` per-function region floor.

Finding disposition:

| Finding | Source | Disposition |
|---|---|---|
| Include `direct_tests.rs` in the completion commit. | Review Agent A | Accepted; `direct_tests.rs` is in the package write set and will be staged with the completion commit. |
| ADR-0021 obligation binding was too broad. | Review Agent B initial QA | Accepted/fixed; `obligation-to-test-map.md` now binds applicable SC obligation vectors to concrete tests and `coverage-closure.md` links it. |
| 2000-line WARN lacked follow-on split intent. | Review Agent B initial QA | Accepted/fixed; `line-count-governance.md` records production split intent for future direct-kernel include fragments. |
| Doc-lint evidence pointed at stale `wctl`/0-file output. | Review Agent B initial QA | Accepted/fixed; exact `markdown-doc lint --path ... --format plain` was rerun and captured in `final-current-3/doc_lint.log`. |
| Per-function floor wording used cargo-crap coverage, not llvm-cov region floor. | Review Agent B initial QA | Accepted/fixed; `coverage-closure.md` now records llvm-cov deduplicated per-function source-span regions. |
| `final-current-3` summary omits doc lint. | Review Agent B re-review | Accepted as non-blocking; `gate-results.md`, `doc_lint.log`, and `doc_lint.status` provide direct doc-lint evidence. |

Current status:

- Review findings are dispositioned.
- Pending: dual verification, final disposition, worker handoff, and completion
  commit.
