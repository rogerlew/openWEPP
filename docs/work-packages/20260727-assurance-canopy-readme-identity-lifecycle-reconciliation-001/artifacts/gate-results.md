# Gate Results

Status: `IN PROGRESS / FULL WORKSPACE PENDING`

Evidence class: `Ran + Static`

## Transaction provenance

- Implementation commit: `0d2ed10b`.
- Exact release binary:
  `target/release/openwepp-assurance`, SHA-256
  `d52956f5d33b0af5ffb205a9dec2d37a77a6b96fe7d6b6ff8443371300ffc5f3`.
- Source-adoption `--check`: read-only `PASS`; old generation
  `1b3c92574e68c675010548f339ca9ebf52f0ff039a6f9d1595a9223c8478d3f5`,
  candidate generation
  `bc2679767c6dc8197af6a3dd3072602cdae195718e89a81a6cb7f77f0106d71a`.
- Source-adoption `--apply`: `PASS` with the same candidate and receipt
  `assurance/v2/transactions/c9defca3a2e84eac7067da884589503a457e4b399c8ea6f78de73df7b75e18d8.json`.
  The receipt SHA-256 is
  `dca719d485501781500afcf40b4692b5185b7324f48768be2da8f4d51bbb74a8`.
- Documentation `rebind-implementation --check/--apply`: `PASS`; final
  generation
  `b0a85461c9ea2523db992d8098ad2b1aaea1eda76b5e49b24f32b22dfea802c8`,
  receipt
  `assurance/v2/transactions/c72814010c9d92b3d38297b33d5e99769076ba5a3a323928aee891e842e371b2.json`,
  receipt SHA-256
  `1b691e6ec134f103408c44fcc48a87883ada8be0cd81544810e6cb3b66d8813e`.
- Repeat source-adoption and implementation-rebind checks: `changed:false`;
  no recovery directory remains.
- Generation-chain verification:
  `target/release/openwepp-assurance verify-generation --base-ref
  15763d7f6d5d4125333d9b7583424c714f5f5ea4` — `PASS`, 19 transitions.

## Focused and consumer gates

- `target/release/openwepp-assurance validate --all`: `PASS`, two DRAFT
  reports, zero public reports.
- `inspect` on the snow/frozen-soil report: `PASS`; no active events and the
  prior review-entry event is in invalidated custody.
- `plan --all --format json`: `PASS`; both reports and all nodes current.
- Disposable `build --all` then `check --all`: `PASS`, two reports.
- `cargo test -p openwepp --test assurance_v2_amendment_contract
  -- --test-threads=1`: `PASS`, 15 passed and two fixture generators ignored.
- `cargo nextest run -p openwepp-assurance`: `PASS`, 25/25.
- `cargo clippy -p openwepp-assurance --all-targets -- -D warnings`:
  `PASS` after accepting and correcting two style findings.
- `cargo fmt --all -- --check`: `PASS`.
- `bash tools/release/check_authority_suite_antievasion.sh`: `PASS`.
- `cargo nextest run --test
  auth11_required_suite_obligation_guards_contract`: `PASS`, 3/3.
- `git diff --check`: `PASS`.

The first post-adoption amendment integration run correctly exposed two tests
that had relied on production snow report lifecycle being `IN_REVIEW`.
Commit `02f05234` made those fixture preconditions explicit; the terminal
15/15 run passes. The first warnings-denied Clippy run exposed an obfuscated
conditional and an oversized CLI unit test; commit `749847fd` corrected both.

## Pending

- Comparator-run exact-head unfiltered
  `cargo nextest run --workspace --profile full`.
- Comparator-run `cargo deny check` and any terminal broad Clippy selection.
- Dual independent review and dual terminal verification.

Coverage/CRAP disposition: `DEFERRED_TO_QUALITY_CI` per ADR-0041.
