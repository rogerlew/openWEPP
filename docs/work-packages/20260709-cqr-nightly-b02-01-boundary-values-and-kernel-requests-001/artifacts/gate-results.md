# Gate Results

| Gate | Status | Evidence |
|---|---|---|
| focused nextest | PASS | `33` passed, `0` skipped |
| focused LCOV / CRAP | PASS | target CRAP rows above `30`: `0` |
| production line/region closure | PASS | `100%` / `100%`, excluding `#[cfg(test)]` lines |
| `cargo fmt --check` | PASS | Ran after final helper extraction |
| `git diff --check` | PASS | Ran after final helper extraction |
| package doc lint | PASS | `22` files, `0` errors, `0` warnings |
| workspace clippy | PASS | delegated final3 runner, exit `0` |
| full workspace nextest | PASS | delegated final3 runner: `1603/1603`, `3` skipped, `575.547s` |
| cargo deny | PASS | delegated final3 runner, exit `0` |
| dual review | PASS | two re-reviews accepted all findings |
| dual verification | PASS | Verification A/B final refreshes accepted |
