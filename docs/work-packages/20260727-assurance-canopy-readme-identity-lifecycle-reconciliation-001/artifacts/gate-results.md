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
- Review-found serializer repair used the corrected exact release binary,
  SHA-256
  `d70b0cc372664e8c3c357e562c7cee9efa02ac4c95327da8497fa5f260733357`.
  Its read-only check and apply removed only the two schema-invalid empty
  `review.findings`/`review.approvals` keys, advanced generation
  `b0a85461...` to
  `94df966626df18d8231227f83dacb9c617198553c0676d7ba21eacb931fc4160`,
  and wrote receipt
  `assurance/v2/transactions/2b73f43f14e2cf0d2a2957b70bd83660c4af4dc64c0a12d7f0597b5e7e577570.json`
  (SHA-256
  `bd60d22a2d260fd5e4955504478981931ea73d02dca86672b2105a51b6c157e3`).
  A repeat check is `changed:false`.
- Generation-chain verification:
  `target/release/openwepp-assurance verify-generation --base-ref
  15763d7f6d5d4125333d9b7583424c714f5f5ea4` — `PASS`, 20 transitions.

## Focused and consumer gates

- `target/release/openwepp-assurance validate --all`: `PASS`, two DRAFT
  reports, zero public reports.
- `inspect` on the snow/frozen-soil report: `PASS`; no active events and the
  prior review-entry event is in invalidated custody.
- `plan --all --format json`: `PASS`; both reports and all nodes current.
- Disposable `build --all` then `check --all`: `PASS`, two reports.
- `cargo nextest run --test assurance_v2_amendment_contract`: `PASS`, 16/16
  with two fixture generators skipped; independent QA run
  `b8c7ac7e-df36-4e69-9721-fdcc77b56a35`.
- `cargo nextest run -p openwepp-assurance`: `PASS`, 25/25.
- Post-review assurance crate run: `PASS`, 27/27, run
  `d5bb2ea3-1897-41cb-8237-7dcd1ea84955`, including the
  adoption-specific selected-source race.
- Production source contract suite: `PASS`, 12/12, independent QA run
  `b032ed1a-e416-4d4a-9e3c-2801e7283988`.
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

The first unfiltered full profile at `df40f94f` stopped red after detecting two
production-source contract failures: a stale `IN_REVIEW` assertion and the
schema-invalid empty review keys written by the initial reset. The run was
interrupted after diagnosis and is not closure evidence. Commits `d6c2837b`
and `3b24155f`, plus corrective generation `94df9666...`, close those findings.
The selected-source race, full `assurance/` namespace rejection, exact defective
reset envelope, schema acceptance, and no-op behavior now have focused tests.

## Pending

- Comparator run `9d17ef98-c121-4f18-b528-59d33b7afcce` at `2bf1a600`
  completed 2,299 tests: 2,278 passed, 21 failed, and 43 skipped. All 21
  failures share `GATE-ASSURANCE-ASSESSED-ROOT`: TESTGATE policy incorrectly
  requires immutable registry historical roots to equal mutable current DRAFT
  review-lock roots. This is owned by
  `20260727-testgate-assurance-historical-root-decoupling-001`; the full profile
  must rerun after correction.
- The same comparator execution passed `cargo deny check` and warnings-denied
  all-target Clippy for `openwepp-assurance`. Logs:
  `/tmp/openwepp-gate-rerun-2bf1f-20260727-083312/`.
- Dual terminal verification.

Coverage/CRAP disposition: `DEFERRED_TO_QUALITY_CI` per ADR-0041.
