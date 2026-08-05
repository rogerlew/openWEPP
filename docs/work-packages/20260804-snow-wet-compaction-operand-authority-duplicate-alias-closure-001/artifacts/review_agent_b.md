# Independent Review B

Status: complete -- final re-review

Evidence mode: Static + Ran

## Findings

No unresolved closure-blocking findings.

1. **High -- resolved; historical HOLD lifted.** The initial exact-source
   `quick` attempt timed out in
   `assurance_v2_publication_contract::approval_conflicts_and_release_mismatch_fail_before_publication`
   and
   `assurance_v2_publication_contract::authority_bound_byte_negative_matrix_is_fail_closed`
   after approximately 619 and 618 seconds, respectively. Both processes were
   CPU-active on a heavily contended host. Review B correctly issued a HOLD at
   that point; the historical evidence remains in
   `target/21k-gates/9_nextest_workspace_profile_quick.log`. A lower-concurrency
   retry subsequently passed all 2,181 quick-profile tests. The frost profile,
   full workspace, doctests, `cargo deny`, full assurance validation, fixture
   check, anti-evasion, and AUTH11 gates also passed. The retry demonstrates
   that the original timeout was load-sensitive rather than a reproducible
   correctness failure, so the validation blocker is resolved.

No unresolved implementation-quality finding was found in the reviewed Rust,
offline replay, fixture, or materiality paths.

## Resolved Findings Confirmed by Review

- **High -- resolved:** the materiality runner now binds source, release binary,
  tool, predecessor receipts, and trace hashes; publishes through a pending
  result only after identity rechecks; and fails closed on operand, upstream
  mass, Stage-3 closure, density, and layer tolerances. It correctly reports
  density-mediated Stage-3 disposition as an observation rather than folding it
  into the upstream mass-invariance gate. Relevant paths are
  `tools/run_materiality.py` and
  `target/snow_wet_compaction_operand_closure/results/materiality.json`.
- **Medium -- resolved:** `Cargo.toml` registers the root integration target,
  allowing workspace/all-target Clippy to inspect it. The exact-source
  `cargo clippy --workspace --all-targets -- -D warnings` run passed.
- **Medium -- resolved:** the offline density replay in
  `crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs` requires the
  three named authority columns, rejects missing, negative, and non-finite
  values with `SnowbenchError`, and uses their exact sum in the real daily
  compaction consumer. Routed melt and snowpack state loss remain diagnostics.
  `snowbench_coe_melt.rs` emits the required internal gross-positive-melt
  column without changing the public v1 summary or runtime-trace schemas.
- **Low -- resolved:** package evidence now distinguishes upstream invariance
  from density-mediated Stage-3 disposition and records the current eight-test
  integration suite and current line counts.

## Independent Evidence

Static:

- Production constructs one private `wet_compaction_liquid_input_m` from
  `sum(max(hourly melt_raw_m, 0)) + rain_retained_m + rain_released_m`, validates
  it through the typed snow boundary guard, and hands only that scalar to the
  bulk and multilayer density paths. The retired reconstructed alias is absent
  from the consumer handoff.
- The inactive/onset, bulk, multilayer, wrong-formula separation, offline
  required-column, fixture custody, and real-consumer checks cover the principal
  regression surfaces. Production additions contain no `unwrap`, `expect`,
  broad boxed error, or undocumented `unsafe`; the reported `unwrap`/`expect`
  occurrences are test-only.
- The Snowbird development fixture is explicitly `DEVELOPMENT_ONLY`. Its Rust
  custody test fixes both SHA-256 identities, all 14,245 rows, every
  non-precipitation token, exact integer half-up scaling, 4,472 changed rows,
  and the 46,491.8/56,519.1 mm precipitation totals.
- Current line counts are 927 for the support helper, 2,579 for infiltration
  reconciliation, 2,723 for runoff reconciliation, 969 for offline density,
  1,209 for offline melt, and 428 for the new integration target. No reviewed
  module crosses 3,000 lines; the two reconciliation modules remain in the
  warning band.

Ran:

- Release CLI build passed. The receipt-bound materiality execution passed with
  maximum operand reconstruction `8.353e-17 m`, predecessor-alias
  reconstruction `2.776e-17 m`, upstream mass delta `2.443e-15 m`, Stage-3
  closure residual `3e-17 m`, density-process closure `2.274e-13 kg/m3`, layer
  SWE/depth residuals `4.441e-16/8.882e-16 m`, 24,046 driver-changed days, and
  22,392 density-changed days. The separately observational maximum Stage-3
  disposition delta was `0.0023629918187403603 m`.
- The package integration target passed 8/8; offline authority-column behavior
  passed 2/2; the production helper test passed 1/1; and the two focused density
  targets passed 3/3 each. Exact-source `cargo fmt --all -- --check` and
  workspace/all-target Clippy with warnings denied also passed. Logs are under
  `target/21k-gates/`.
- Terminal regression reconciliation passed: quick retry 2,181/2,181, frost
  358/358, and full workspace 2,270/2,270. Workspace doctests passed. Exact
  `cargo deny check` passed advisories, bans, licenses, and sources with only the
  repository's existing unused `MIT-0` allowance warning. Assurance
  `validate --all`, the Snowbird generator `--check`, the source-level
  anti-evasion guard, and AUTH11 3/3 all passed. The structured receipt contains
  18 successful commands with zero exit codes in
  `target/21k-gates/summary.json`.
- Before this review-only amendment, the current `git diff --binary HEAD` hash
  independently matched the terminal summary identity
  `bd07523f8e0f566c52a152ff4ef6d8dd2c2deadfae5ab760c88c7d6d4d4e4119`
  at scaffold HEAD `4a6948ddbcb652310f4ca063a6c57f9b206a3740`.

## Non-blocking Debt and Follow-ups

- `materiality_tool_fails_closed_on_contract_acceptance_thresholds` checks
  source markers. Add an executable negative test that injects each
  out-of-tolerance/non-finite metric and proves rejection, plus a check that
  acceptance executes before publication. The current tool logic and fresh
  successful execution are correct; this is regression-hardening debt.
- Source-text assertions remain useful anti-evasion tripwires but are brittle
  under refactoring. Keep the behavioral and materiality tests as the primary
  protection and avoid extending string matching as the main contract proof.
- Updating approximately 40 exact contract-version pins is intentional but
  creates mechanical review churn. A typed shared version assertion could
  reduce future drift without weakening exact adoption.
- `infiltration_reconciliation.rs` and `runoff_reconciliation.rs` are both over
  2,000 lines. Future work should prefer cohesive extraction before either
  reaches the 3,000-line hard threshold.
- Historical internal CoE boundary CSV files intentionally fail closed because
  the new named columns are required. If archival replay becomes a requirement,
  add an explicit versioned migration rather than a fallback interpretation.
- Reconcile the stale unused `MIT-0` allowance in `deny.toml` independently of
  this physics package.

## QA Disposition

**GO / PASS.** The historical HOLD was warranted by the first quick-profile
timeouts and is preserved above. It is now lifted because the low-concurrency
quick retry and every remaining required terminal gate passed. The
implementation, anti-alias separation, real-consumer proof, offline alignment,
fixture custody, materiality evidence, and validation matrix are acceptable.
Only the explicitly non-blocking debt above remains.
