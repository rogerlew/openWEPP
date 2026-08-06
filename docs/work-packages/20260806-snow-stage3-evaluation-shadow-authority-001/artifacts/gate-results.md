# Gate Results

Evidence class: `Ran`

Exact-head validation commit:
`e601f0f966c1531fb95bb81e304a23bd3044a1ab`.
TESTGATE was not run.

## Focused And Governance Gates

- focused evaluation-shadow authority: PASS, `4/4`;
- all 36 v127-pinned integration binaries: PASS, `151/151` (independent Rust
  reviewer);
- strict Binding Exposure: PASS, nine rows fully consolidated;
- assurance validation: PASS, three DRAFT reports and zero public;
- Markdown lint: PASS, zero findings;
- `cargo fmt --all -- --check`: PASS, `2.74 s`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  PASS, `3.10 s`;
- `cargo test --workspace --doc`: PASS, 20 crates and zero doctest cases,
  `6.49 s`;
- `cargo deny check`: PASS, `0.99 s`, with one non-failing unmatched `MIT-0`
  allowance warning;
- diff check: PASS.

## Scoped Diagnostic

`check_sc_unit_compliance.sh --path SC-SNOWFREEZE-001.md` reports one inherited
missing `snow.routed_melt_m` Symbol Alias Map entry. The same alias registration
and omission predate v127; this package adds no unit, alias, conversion, or
runtime surface and does not claim a clean unit-lint result.

## Critical Exact-Head Gates

The read-only heavy runner confirmed an empty worktree before and after every
gate at exact commit `e601f0f9`:

- `cargo nextest run --workspace --profile quick`: PASS, `2,193/2,193`, 57
  slow, 40 profile-skipped, `2,289.222 s` Nextest / `2,290.408 s` wrapper;
- `cargo nextest run --workspace --profile frost`: PASS, `360/360`, one slow,
  1,927 profile-skipped, `549.700 s` Nextest / `551.420 s` wrapper; and
- `cargo nextest run --workspace`: PASS, `2,282/2,282`, 58 slow, five skipped,
  `2,645.949 s` Nextest / `2,647.751 s` wrapper.

Exact logs and timing records are under
`target/local-ci-history/snow-stage3-shadow-authority-e601f0f9/`. The release
assurance binary built at the same commit has SHA-256
`44e1f77c22e33e325540716af093f742eccc5d59249a2fa849e1a9631301a483`;
`validate --all` passed all three DRAFT reports with public count zero.

The superseded clean candidate `c77296df` passed static gates but its focused
target failed `3/4` because the test helper searched only backticked table IDs.
No authority defect was hidden: `663fe049` corrected the helper, focused `4/4`
passes, and the heavy suite restarted from the new clean identity. The final
clean validation candidate includes the later review artifacts in `e601f0f9`;
all three heavy profiles passed there.
