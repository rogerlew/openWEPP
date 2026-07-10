# Final Disposition

Evidence label: Static/Ran.

Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

One-line verdict: PASS. CQR nightly target #10 is complete with
characterization-only closure and no production behavior changes.

Target:
`crates/openwepp-runner/src/hillslope/laned_shadow.rs`

Closure summary:

- Scaffold commit: `8b4c79c5 Scaffold CQR nightly runner laned shadow package`
- Production code changed: no
- Test-only characterization added: yes, `#[cfg(test)]` module in target file
- Baseline rows above CRAP `30`: `3`
- Final rows above CRAP `30`: `0`
- Final max CRAP: `14.016830348056178`
- Final target coverage: lines `684/699`, regions `842/877`
- Final production coverage split: lines `321/330`, regions `406/437`
- Target line count: `1003`, below WARN/blocker thresholds

Behavior identity:

- No production selector, routing formula, coefficient source, finite guard,
  profile accounting path, manifest/public output path, or serialization code
  changed.
- New tests pin dynamic operand validation guards, lane/area/runoff
  fail-closed checks, zero-source day commits, positive uniform source routing
  with and without routed-melt classification, missing buffered operand
  fail-closed paths, and diagnostic profile helper behavior.

Gate summary:

- Focused runner tests: PASS, `15` passed
- Focused nextest: PASS, `15` passed
- Targeted coverage/CRAP: PASS; target rows above CRAP `30` = `0`
- ADR-0021 science-tier production coverage closure: PASS; production lines
  `97.27272727272728%`, production regions `92.90617848970251%`
- `cargo fmt --check`: PASS
- `git diff --check`: PASS
- Scoped `markdown-doc lint`: PASS, `22` files scanned, `0` errors,
  `0` warnings
- Final `cargo clippy --workspace --all-targets -- -D warnings`: PASS
- Final `cargo nextest run --workspace --profile full`: PASS, `1594` passed,
  `3` skipped
- Final `cargo deny check`: PASS

Full-workspace coverage/CRAP disposition:

- Full-workspace coverage for this nightly batch remains documented as blocked
  by the unrelated coverage-instrumented `laned_shadow_h2637` path before LCOV
  emission.
- Package closure uses targeted openwepp-runner coverage plus target-module
  CRAP evidence, with artifact paths and hashes recorded in
  `coverage-after.md` and `crap-after.md`.

Review and verification:

- Dual review completed; all actionable findings were accepted and fixed or
  resolved by final gate evidence.
- Dual verification completed. Verification Agent A passed. Verification Agent
  B accepted the source, coverage, CRAP, gate, review, and line-count evidence;
  its lifecycle-artifact hold is resolved by these final closure updates and
  the completion commit step.

Completion commit is required before declaring the CQR nightly 10-module batch
complete.
