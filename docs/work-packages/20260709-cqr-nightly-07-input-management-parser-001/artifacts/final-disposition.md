# Final Disposition

Evidence label: Static/Ran.

Status: `EXECUTED-COMPLETE-CQR-NIGHTLY`

Package:
`20260709-cqr-nightly-07-input-management-parser-001`

Target:
`crates/openwepp-input-contract/src/parsers/management.rs`

## Final Metrics

Ran:

- CRAP rows above `30`: `6` before, `0` after.
- Max CRAP after: `28.136080592592595`.
- Line coverage after: `1782/1984`, `89.81854838709677%`.
- Region coverage after: `2115/2446`, `86.46770237121831%`.
- Production line count after: `2960`.

## Final Gate Verdict

Ran:

- Focused tests: PASS.
- Targeted coverage/CRAP: PASS.
- `git diff --check`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo nextest run --workspace --profile full`: PASS, `1566` passed, `3`
  skipped in post-review delegated rerun.
- `cargo deny check`: PASS.
- `markdown-doc lint`: PASS.
- Dual review: PASS after accepted findings fixed.
- Dual verification: PASS.

## Closeout

Static/Ran:

- Package is complete.
- The closeout commit containing this artifact satisfies the package completion
  boundary before CQR Nightly target #8 starts.
