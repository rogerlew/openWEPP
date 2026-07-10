# Verification Agent A

Evidence label: Static/Ran.

Status: `PASS`

Verifier: `rust_code_reviewer` (`019f49ef-d866-71e2-b472-76ea5e56e28a`).

Verdict: PASS. No findings.

Evidence checked:

- Diff from scaffold commit `2e6d3a5a` is source-safe and test-only: the only
  Rust change is a `#[cfg(test)]` module in
  `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/01_ws22_ws23_ws26_detachment.rs`.
- No production formulas, guards, units, serialization, public output, or
  runtime behavior changed.
- Raw CRAP artifact recompute: `24` deduplicated target rows, `0` rows above
  `30`, max CRAP `16.153567674676058`.
- Raw coverage artifact recompute: lines `1331/1373`, regions `1348/1399`,
  functions `45/45`; target cargo-crap rows satisfy the `>=75%` floor.
- Target file line count: `1744`, below the `2000` WARN threshold.
- Final refreshed gate summary: clippy PASS, full nextest PASS
  (`1587` passed, `3` skipped), deny PASS.
- Full-workspace coverage/CRAP absence is accepted under the package's targeted
  equivalent rule for the known unrelated coverage-instrumented path blocker.

Commands run by verifier:

- `git diff --check 2e6d3a5a --`
- `cargo fmt --check`
- `cargo test -p openwepp-watershed-orchestrator --lib wshedimpl -- --nocapture`
  - PASS, `16` passed
- `jq`/`awk` recomputes against the recorded targeted CRAP/coverage artifacts
- Static review of `package.md`, the CQR nightly burndown, ADR-0021, source
  diff, final source/artifacts, and final gate logs
