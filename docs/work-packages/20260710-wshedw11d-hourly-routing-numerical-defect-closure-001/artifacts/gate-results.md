# Gate Results

Status: `PASS`

Evidence mode: `Ran`

Gate operator: delegated `comparator_suite_runner`

Execution date: `2026-07-10 America/Los_Angeles`

## Accepted Snapshot

The accepted implementation/test/contract diff fingerprint is:

`c7e0d2ab4b688356fe269acc279f3aa4cd0e62a03b494b3e8f890b43d7debbf6`

It is the SHA-256 of `git diff -- crates tests` plus the three touched canonical
contracts (`SC-ROUTE-001`, `SC-SYSTEM-001`, and `SC-INFILE-CHANINP-001`). The
same fingerprint was observed immediately before and after the accepted full
workspace run. Package evidence/catalog documentation was intentionally
excluded from this execution fingerprint.

## Final Gate Ledger

| Gate | Exact command | Result | Final evidence |
|---|---|---|---|
| Formatting | `cargo fmt --check` | PASS | exit 0; wall 1.95 s |
| Workspace clippy | `cargo clippy --workspace --all-targets -- -D warnings` | PASS | exit 0; wall 3.41 s |
| Dependency policy | `cargo deny check` | PASS | exit 0; wall 0.80 s; advisories, bans, licenses, and sources all `ok` |
| Touched orchestrator crate | `cargo nextest run -p openwepp-watershed-orchestrator` | PASS | 113/113 passed, 0 skipped; nextest 0.218 s; wall 0.72 s |
| P102 fixture manifest | `(cd tests/fixtures/watershed/p102-sediment-active && sha256sum -c input-manifest.sha256)` | PASS | 18/18 committed inputs `OK` |
| Protected committed P102 consumer | `cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity --no-capture` | PASS | 1/1 passed, 28 filtered/skipped; nextest 29.900 s; wall 30.52 s |
| Authority anti-evasion source guard | `bash tools/release/check_authority_suite_antievasion.sh` | PASS | `PASS: authority suite anti-evasion checks passed`; wall 0.06 s |
| Required-suite obligation guard | `cargo nextest run --test auth11_required_suite_obligation_guards_contract` | PASS | 2/2 passed; nextest 0.008 s; wall 0.57 s |
| Exact release CLI build | `cargo build --release -p openwepp-runner --bin openwepp-cli-watershed` | PASS | final currency check exit 0, wall 0.20 s; preceding changed-source relink wall 68.39 s |
| Release W11D consumer/comparator suite | `OPENWEPP_W11C_WATERSHED_CLI=/home/workdir/openWEPP/target/release/openwepp-cli-watershed cargo nextest run -p openwepp-runner --test mt3_hbp_hourly_consumer_contract` | PASS | 7/7 passed, 0 skipped; nextest 1.230 s; wall 1.79 s |
| Full workspace | `cargo nextest run --workspace --profile full` | PASS | 1,693/1,693 passed, 3 configured skips, 4 slow; nextest 588.273 s; wall 590.78 s; exit 0 |

The accepted full run subsumes the touched top-level parser and typed watershed
runtime suites. Focused runs earlier in the same execution also passed:
`infile_chaninp_parser_contract` 20/20 and
`wshedw5_typed_watershed_runtime_contract` 18/18.

## Release-Binary Provenance

- Absolute binary:
  `/home/workdir/openWEPP/target/release/openwepp-cli-watershed`
- Size: `9,367,904 bytes`
- Mtime: `2026-07-10 22:55:04.977364784 -0700`
- SHA-256:
  `f82cc9fa539d26cdf9a6797d3e272bca22a7a19dc4b9988a3a95e7cd4c38d792`
- The exact target was relinked after the production correction and checked
  current again immediately before the final 7-test release suite.

## Compact W11D Release Metrics

The final seven-test suite invoked the explicit release binary through the real
watershed consumer.

- KW (`ipeak=3`) at 3,600 and 600 seconds: all zero/uniform/spike/spread cases
  published finite storage in `[0, 110.260168179987943] m3`; active
  peak/input ratio was at most `1.0`; terminal volume was at most
  `7200.000000000010004 m3` for `7,200 m3` external input (below the `1e-9 m3`
  roundoff tolerance).
- Largest printed absolute channel-balance residual was approximately
  `1.779e-12 m3`; largest printed absolute sediment residual was approximately
  `4.83e-13 kg`.
- The uniform KW vector carried authorized steady initial/final hydraulic
  storage `10.168594800427801 m3`. Its raw
  `input - outlet - final_storage` diagnostic is therefore
  `-10.1685948004 m3`, while the initial-storage-aware published ledger closes
  at approximately `1e-12 m3`.
- Independent orchestrator vectors reconstruct fresh `sinit`, spatially
  averaged KW `sfnl`, `chvol`, public inflow/storage, and both 3,600/600-second
  final-slot terminal states without reusing producer closure operands.
- CREAMS/event scalar (`ipeak=2`) selected topology-terminal element `2` and
  published `7,200 m3` runoff and `240 kg` sediment rather than serial
  internal-throughflow sums.
- A nonzero 60-second static/dynamic MC pair (`ipeak=4/5`) executed through the
  parser-to-release-CLI path with finite nonnegative output, passive peak bound,
  and distinct dynamic refresh behavior.
- Four MC zero-flow controls executed, while all 16 active W11C MC cases at
  3,600/600 seconds rejected before publication with typed identity
  `WKERNEL-WS10-CHANNEL-E-003`; no clamp, damping, or fallback was used.
- The three-record `nchnum=0` real CLI case retained `dtchr=600`, `ntchr=144`,
  no channel IDs, and output-disabled semantics without compatibility-default
  aliasing.
- The exact committed p102 sediment/publication fixture passed for both job
  counts after its wrapper-only selector changed from inadmissible static MC to
  KW; the refreshed 18-entry manifest and source anti-evasion guards passed.

## Superseded Diagnostic Runs

The execution retained, rather than hid, three useful red gates:

1. Initial clippy failed on two exact floating-point parser assertions; the
   assertions were converted to tolerance comparisons and all subsequent
   workspace clippy runs passed.
2. A pre-correction full profile ran 1,685 passed / 1 failed because the
   historical p102 wrapper selected an inadmissible 600-second MC recurrence.
   The protected fixture was reconciled without weakening the MC guard, its
   manifest and anti-evasion gates were refreshed, and the accepted full run is
   1,693/1,693.
3. The first admitted-MC release fixture retained channel controls that did not
   match its independently green unit geometry and failed typed E003. Correcting
   only that fixture anti-alias mismatch produced the final 7/7 release result.

A later 1,691/1,691 diagnostic full run was also superseded because additive
fixture and independent-reconstruction tests landed while it drained. It was
not accepted as final evidence. Only the unchanged-fingerprint 1,693-test run
above supports this PASS disposition.

## Disposition

All package-required format, lint, dependency, focused, release, comparator,
anti-evasion, protected-consumer, and full workspace gates pass on the accepted
snapshot. Gate disposition: `PASS`.
