# Final Disposition

Evidence label: Static/Ran.

Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

One-line verdict: PASS. CQR nightly target #9 is complete with
characterization-only closure and no production behavior changes.

Target:
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`

Closure summary:

- Scaffold commit: `2e6d3a5a Scaffold CQR nightly watershed detachment package`
- Production code changed: no
- Test-only characterization added: yes, `#[cfg(test)]` module in target file
- Baseline rows above CRAP `30`: `4`
- Final rows above CRAP `30`: `0`
- Final max CRAP: `16.153567674676058`
- Final target coverage: lines `1331/1373`, regions `1348/1399`
- Target line count: `1744`, below WARN/blocker thresholds

Behavior identity:

- Existing formulas, coefficients, thresholds, units, finite guards, branch
  ordering, validation guards, and output/publication semantics were not edited.
- Exact characterization now pins WS22 table behavior, WS23 validation/error
  identity, WS23 case-4 closure outputs, WS23 leaf transport/potential/final
  helper outputs, WS23 low-shear iterative-loop outputs, WS26 expanding-width
  outputs, WS26 midlayer/terminal/cap behavior, WS26 low-width-shear
  class-fraction allocation, WS24 length guard, WS27 bracket terminal, WS30
  shape parsing/fallback behavior, and WS20/WS22 scalar helper outputs.

Gate summary:

- Focused detachment tests: PASS, `16` passed
- Targeted coverage/CRAP: PASS; target rows above CRAP `30` = `0`
- ADR-0021 science-tier coverage closure: PASS; target lines
  `96.94100509832484%`, target regions `96.35453895639743%`
- `git diff --check`: PASS
- Scoped `markdown-doc lint`: PASS, `22` files scanned, `0` errors,
  `0` warnings
- Final post-review `cargo clippy --workspace --all-targets -- -D warnings`:
  PASS
- Final post-review `cargo nextest run --workspace --profile full`: PASS,
  `1587` passed, `3` skipped
- Final post-review `cargo deny check`: PASS

Full-workspace coverage/CRAP disposition:

- Full-workspace `cargo llvm-cov --workspace --ignore-run-fail` did not produce
  an LCOV artifact before the optional attempt was terminated on the known
  unrelated coverage-instrumented workspace path.
- Package closure uses targeted watershed-orchestrator coverage plus
  target-module CRAP evidence, with artifact paths and hashes recorded in
  `coverage-after.md` and `crap-after.md`.

Review and verification:

- Dual review completed; all actionable findings were accepted and fixed or
  explicitly deferred as non-blocking debt.
- Dual verification completed and recorded in
  `verification_agent_a.md` and `verification_agent_b.md`.

Completion commit is required before starting CQR nightly target #10.
