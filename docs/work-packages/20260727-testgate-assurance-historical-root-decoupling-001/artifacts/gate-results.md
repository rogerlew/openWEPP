# Gate Results

Status: `PASS`

Evidence class: `Ran + Static`

Implementation target:
`576f43b85df7be3d2495395b1001a8f562a6ac7b`.

Focused results:

- `cargo nextest run -p openwepp-gate-planner`: `PASS`, 176/176 with
  14 skipped; primary run `e58fa7a8-b7ec-42dc-9f1e-fab12237c926`.
- `cargo nextest run --test testgate_assure_campaign_currency_contract`:
  `PASS`, 4/4; primary run `15118f8e-1abe-4135-b777-4f0b96e261fe`.
- Independent corrected review repeats: 176/176 and 4/4, both reviewers.
- `cargo clippy -p openwepp-gate-planner --all-targets -- -D warnings`:
  `PASS`.
- `cargo fmt --all -- --check`: `PASS`.
- `cargo deny check`: `PASS`.
- `bash tools/release/check_authority_suite_antievasion.sh`: `PASS`.
- `cargo nextest run --test
  auth11_required_suite_obligation_guards_contract`: `PASS`, 3/3; run
  `6e2882fe-4570-4b92-8c37-d4974b1e1025`.
- `git diff --check`: `PASS`.

The pre-correction full run remains diagnosis evidence: run
`9d17ef98-c121-4f18-b528-59d33b7afcce` at `2bf1a600` ran 2,299 tests,
with 2,278 passed, 21 failed, and 43 skipped. Every failure shared the now
corrected `GATE-ASSURANCE-ASSESSED-ROOT` coupling.

The comparator-owned exact closure-candidate run at
`3e78dedbd3b0f0a2c3e1e6d7d90bf625a240ddfd` passed:

- `cargo nextest run --workspace --profile full`: `PASS`;
- run `7e79049d-0871-4142-a9f7-86ac7ac714be`;
- 2,301/2,301 tests passed, 43 skipped, zero failures;
- retained log:
  `/tmp/openwepp-testgate-full-3e78dedb-20260727T163100Z/full-gate.log`;
- exit marker records `exit_code=0` and completion at
  `2026-07-27T16:47:03Z`.

At the same exact closure-candidate head, cargo-deny and warnings-denied
all-target Clippy for both `openwepp-gate-planner` and
`openwepp-assurance` passed. Coverage/CRAP is
`DEFERRED_TO_QUALITY_CI` per ADR-0041.
