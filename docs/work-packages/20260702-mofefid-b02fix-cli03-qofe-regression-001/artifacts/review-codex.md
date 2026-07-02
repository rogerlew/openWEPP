# Codex Review - MOFEFID-B02FIX CLI03 QOFE Regression

Date: 2026-07-02
Reviewer: Codex
Branch/worktree: `worktree-mofefid-b02fix-cli03` / `/tmp/openwepp-b02fix-review`
Reviewed range: `d30e985b..becc1118`, plus Codex test-strengthening commit.

## Outcome

Accepted for merge.

The branch correctly identifies the red `cli03_mf_multiofe_publication_emits_public_per_ofe_wat_rows`
test as a B02/QOFE convention migration miss, not a D4 routing regression. The
test-only fix aligns the stale assertions with `SC-RUNOFFPART-001#INV-RUNOFFPART-032`:
published `QOFE == Q` on all WB13 rows, while runon handoff evidence continues
to use the retained local-length runoff basis reconstructed from `Q`.

I made one review-time strengthening before merge: the new `assert_b02_qofe_equals_q_all_rows`
helper was initially called only on `wat_rows[0..3]` despite the package and
contract saying "all rows." It now runs over all six emitted WAT rows.

## Evidence Classes

Static:
- Reviewed `docs/work-packages/20260702-mofefid-b02fix-cli03-qofe-regression-001/package.md`.
- Reviewed `tests/integration/cli03_runner_contract_derived_tests.rs`.
- Reviewed `SC-RUNOFFPART-001#INV-RUNOFFPART-032`.
- Grep-reviewed sibling `QOFE` assertions across `tests`, `crates/openwepp-runner/src`, and active science contracts.

Ran:
- `cargo fmt --check` -> pass.
- `cargo nextest run --test cli03_runner_contract_derived_tests` -> 22/22 passed.
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
- `cargo nextest run --workspace --profile full` -> 1223/1223 passed, 1 skipped.
- `cargo deny check` -> pass.

## Findings

| Candidate | Verdict | Evidence | Disposition |
|---|---|---|---|
| B02-fix direction: stale CLI03 test asserted the pre-B02 `QOFE != Q` local-length publication convention. | Accepted | Static: `INV-RUNOFFPART-032` requires published `QOFE == Q` on all WB13 rows and volume recovery from `Q(outlet) * A_total` or `H.pass.runvol`, not `QOFE * per_OFE_area`. The migrated test now asserts `QOFE == Q` and reconstructs handoff from `Q * local_length_ratio * area_ratio`. Ran: CLI03 suite 22/22 and full workspace 1223/1223 passed. | Closed. |
| Test-scope mismatch: helper name/package said "all rows" but initial branch called it only on first-day `wat_rows[0..3]`. | Accepted, fixed by Codex | Static: original call covered only three of six emitted rows. | Updated call to pass `&wat_rows`; all six rows are checked. |
| Sibling old-convention assertions. | Rejected | Static grep found historical contract revision rows and current reconciled contract text, but no remaining active test helper asserting `QOFE != Q` as the canonical publication rule. | No further changes. |

## Merge Decision

Merge-ready. This is test-only relative to production code, and the full
workspace profile is green after the all-row assertion strengthening.
