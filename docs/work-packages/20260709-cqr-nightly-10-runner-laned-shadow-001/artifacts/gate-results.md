# Gate Results

Evidence label: Static/Ran.

Status: `EXECUTED`

Focused/local gates run so far:

| Gate | Status | Evidence |
|---|---|---|
| scaffold commit before implementation | PASS | `8b4c79c5` |
| `cargo fmt --check` | PASS | exit `0` |
| `cargo test -p openwepp-runner laned_shadow --lib -- --nocapture` | PASS | `15` passed, `83` filtered |
| `OPENWEPP_LANED_SHADOW_PROFILE=1 cargo test -p openwepp-runner diagnostic_profile_helpers_cover_opt_in_surfaces_without_public_outputs --lib -- --nocapture` | PASS | `1` passed, `97` filtered; review env-profile reliability check |
| `cargo nextest run -p openwepp-runner laned_shadow` | PASS | `15` passed, `133` skipped |
| `cargo clippy -p openwepp-runner --lib --tests -- -D warnings` | PASS | exit `0` |
| `git diff --check` | PASS | exit `0` |
| targeted LCOV | PASS | `/tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted.lcov`, `e09a39365ce1413bb9bfdcbbf70bc4a7a3a02536c34e126e51fba50d2bf4ecd7` |
| targeted JSON coverage | PASS | `/tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted-llvmcov.json`, `0e9dcedd6889b63c49578543e38b9cb0e78ed769b9748a2ee3536f6e8a99f31b` |
| targeted CRAP | PASS | `/tmp/openwepp-cqr-nightly-10-runner-laned-shadow-targeted-crap.json`, rows above `30`: `0` |
| `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260709-cqr-nightly-10-runner-laned-shadow-001 --format json` | PASS | `22` files scanned, `0` errors, `0` warnings |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | `final_heavy_clippy.log`, `314` bytes, `027fc132f5824c2ccb0d88755c8a6592ada6039b1e6f9f4ca420e2debebb3986`; summary `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in 5.29s` |
| `cargo nextest run --workspace --profile full` | PASS | `final_heavy_nextest_full.log`, `1213` bytes, `c915030f606fcf33ef1c818eae522744f6686c23665f2e26180cd1c495b708ef`; summary `1594 tests run: 1594 passed (4 slow), 3 skipped` |
| `cargo deny check` | PASS | `final_heavy_deny_check.log`, `48` bytes, `f1a0fca39d4280363937aabd77783990ea6480bd9ca257816de3b68fc8efa845`; summary `advisories ok, bans ok, licenses ok, sources ok` |
| dual review | PASS | Review Agent A `PASS`; Review Agent B `PASS` |
| dual verification | PASS | Verification Agent A `PASS`; Verification Agent B accepted code/gate evidence and its lifecycle-artifact hold was resolved by final disposition updates |

Full-workspace coverage/CRAP disposition:

- Full-workspace coverage for this nightly batch remains documented as blocked
  by the unrelated coverage-instrumented `laned_shadow_h2637` path before LCOV
  emission. This package uses the same Phase D targeted target-module
  coverage/CRAP equivalent as packages #8 and #9.
