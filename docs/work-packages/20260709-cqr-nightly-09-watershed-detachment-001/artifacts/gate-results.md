# Gate Results

Evidence label: Ran.

Status: `EXECUTED`

Focused/local gates run so far:

- `cargo test -p openwepp-watershed-orchestrator --lib wshedimpl -- --nocapture`
  - PASS, `16` passed, `0` failed, `66` filtered.
- `cargo llvm-cov -p openwepp-watershed-orchestrator --lib --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted.lcov`
  - PASS, `82` passed.
- `cargo llvm-cov -p openwepp-watershed-orchestrator --lib --ignore-run-fail --json --output-path /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted-llvmcov.json`
  - PASS, `82` passed.
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-09-watershed-detachment-targeted-crap.json`
  - PASS, exit `0`; targeted-LCOV unmatched-file warnings expected.
- `cargo fmt --check`
  - PASS.
- `cargo clippy -p openwepp-watershed-orchestrator --lib --tests -- -D warnings`
  - PASS after accepted review fixes.
- `git diff --check`
  - PASS.
- `markdown-doc lint --path docs/work-packages/README.md --path docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001 --format json`
  - PASS after final artifact updates, `22` files scanned, `0` errors,
    `0` warnings.

Initial delegated heavy gates before final review-strengthening fix:

- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS, exit `0`.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/cargo_clippy.log`
  - sha256:
    `eeb95daac2ac7c5ae3513290864161185c92086af75cbc7a9a34f899b9ccab6f`
- `cargo nextest run --workspace --profile full`
  - PASS, exit `0`.
  - Summary: `1579` tests run, `1579` passed, `3` skipped, `4` slow.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/cargo_nextest.log`
  - sha256:
    `85cb193720b269ca18a326fec880d7235c98e04d6e94cd4fa4c0f916b2eabf24`
- `cargo deny check`
  - PASS, exit `0`.
  - Summary: `advisories ok, bans ok, licenses ok, sources ok`.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/cargo_deny.log`
  - sha256:
    `f1a0fca39d4280363937aabd77783990ea6480bd9ca257816de3b68fc8efa845`

Full-workspace coverage/CRAP attempt:

- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path /tmp/openwepp-cqr-nightly-09-full.lcov`
  - BLOCKED/INTERRUPTED; no `/tmp/openwepp-cqr-nightly-09-full.lcov`
    artifact was produced and no status file was written before the long-running
    optional coverage attempt was terminated.
  - Partial log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/cargo_llvm_cov.log`
  - sha256:
    `b8fab3cfcb70d2d693328fd91f7dc36d5c7c3d24b3c79ae8798f5a7119e32a48`
  - Block evidence: this is the same full-workspace coverage path that is
    unstable for the unrelated coverage-instrumented `laned_shadow_h2637`
    integration test; targeted target-module coverage/CRAP evidence is recorded
    in `coverage-after.md` and `crap-after.md`.
- `cargo crap --workspace --lcov /tmp/openwepp-cqr-nightly-09-full.lcov --min 0 --format json --output /tmp/openwepp-cqr-nightly-09-full-crap.json`
  - NOT RUN because the required full LCOV artifact was absent.

Previous post-review delegated gates before final ADR-0021 coverage
strengthening:

- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS, exit `0`, `0` warnings/errors.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/final_cargo_clippy.log`
  - sha256:
    `81bca696d770d638b7ddee1d0c5ad6c3906dddc99672522e8e6088a574006805`
- `cargo nextest run --workspace --profile full`
  - PASS, exit `0`.
  - Summary: `1579` tests run, `1579` passed, `3` skipped, `4` slow.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/final_cargo_nextest_full.log`
  - sha256:
    `cd21ca473b6453f54578d139e4658109ec3025a0064da98830abe020d145a1fd`
- `cargo deny check`
  - PASS, exit `0`.
  - Summary: `advisories ok, bans ok, licenses ok, sources ok`.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/final_cargo_deny.log`
  - sha256:
    `f1a0fca39d4280363937aabd77783990ea6480bd9ca257816de3b68fc8efa845`

Final refreshed delegated gates after final ADR-0021 coverage strengthening:

- `cargo clippy --workspace --all-targets -- -D warnings`
  - PASS, exit `0`.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/final_refresh_clippy.log`
  - sha256:
    `bb15bf68c8d83022cc73ac95496f4623d5a7ee59d993e226e52fbc6efa568ce7`
- `cargo nextest run --workspace --profile full`
  - PASS, exit `0`.
  - Summary: `1587` tests run, `1587` passed, `3` skipped, `4` slow.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/final_refresh_nextest_full.log`
  - sha256:
    `7589118edfe2a777a4bdaa6dc9ea87d7853155f090776b1385a85ce35af4535d`
- `cargo deny check`
  - PASS, exit `0`.
  - Summary: `advisories ok, bans ok, licenses ok, sources ok`.
  - Log:
    `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/final_refresh_deny_check.log`
  - sha256:
    `2c3ed61fcd52a58c1ec7ff56dc313f7e5cb99800986b5de66e7659dc62ab4efd`
- Refresh command log:
  `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/final_refresh_command-log.txt`
  - sha256:
    `8332aad0d6d02e2e21b5eab14a447a750a69c8d914d98bec1bd9c6ec8c384417`
- Refresh summary:
  `docs/work-packages/20260709-cqr-nightly-09-watershed-detachment-001/artifacts/final_refresh_summary.txt`
  - sha256:
    `bc0e199d7ea8d1697b8a5ffba4855b9721761b163c4f9c4619dde162e3f5355a`

Pending final local closure checks: none.
