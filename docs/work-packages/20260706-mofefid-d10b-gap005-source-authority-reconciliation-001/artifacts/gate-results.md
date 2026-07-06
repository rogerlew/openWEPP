# Gate Results (D10B)

Status: executed
Evidence mode: Ran (all commands executed by Claude Code in-session,
2026-07-06; heavy suite run as a tracked background job)

| Gate | Command | Result |
|---|---|---|
| Whitespace | `git diff --check` | PASS (clean at scaffold and at closure) |
| Format | `cargo fmt --check` | PASS (after `cargo fmt` apply) |
| Lint | `cargo clippy --workspace --all-targets -- -D warnings` | PASS (0 errors) |
| Full test suite | `cargo nextest run --workspace --profile full` | **PASS — 1396 tests run: 1396 passed (7 slow), 2 skipped** (721.8 s) |
| Supply-chain | `cargo deny check` | PASS ("advisories ok, bans ok, licenses ok, sources ok") |
| D-val Case-4 baseline reproduction | `compare_dval.py --case 4 --ko 200 ...` (3 resolutions) | PASS — bit-exact D10 metric reproduction (`logs/s0-case4-*.log`) |
| Oracle self-evidence | `cargo test ... iwagaki_oracle` | PASS 4/4 |
| Contract-derived reconciliation tests | `cargo test ... d10b_reconciliation` | PASS 5/5 (recorded FAILING 5/5 pre-correction) |
| Focused module sweep | `cargo test ... ofe_routing` | PASS 61/61 |
| Seam-ledger sweep (pre/post) | `cascade_seam_ledger` example | `logs/s0-seam-ledger-decomposed.json` (pre) / `logs/s4-seam-ledger-final.json` (post: all terms 0.0) |
| Oracle + solver convergence dump | `iwagaki_oracle_dump` example | `logs/s3-oracle-metrics.json`, `logs/s4-oracle-solver-final.json` |

Anti-evasion guards: NOT RUN — not triggered (no external-authority suite
posture, cohort fixture, or required-case binding was touched; the D-val
Case-4 changes are within `ofe_routing` and its own tests). Justification
recorded here per the gate-selection standard.

Markdown lint: no repo-pinned markdownlint tool/config exists;
`git diff --check` clean stands as the docs whitespace gate.

Addendum (review A, MINOR-6): Contract/profile/BEI check —
`tools/check_sc_binding_exposure.py` on the amended SC-OFEROUTE-001:
**PASS-DEFERRED** (6 binding exposure rows, 5 science-review-follow-on) —
run read-only by Review Agent A (Ran, attributed).

## Post-dual-review closure gates (after the B-M1/M2/M3 fixes + rev 26)

| Gate | Result |
|---|---|
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS (0 errors) |
| Focused `ofe_routing` suite | PASS **64/64** (61 prior + 3 review regressions), re-run again after the verification-A comment sweep |
| `cargo nextest run --workspace --profile full` (re-run) | **PASS — 1399 tests run: 1399 passed (6 slow), 2 skipped** (736.0 s) |
| `git diff --check` | PASS |

Dual verification: Agent A PASS-WITH-NOTES (all notes closed in-package);
Agent B PASS-WITH-NOTES (notes closed; executional 64/64 confirmation).

## Review-response gates (Codex review fixes, 2026-07-06)

| Gate | Result |
|---|---|
| `git diff --check` | PASS |
| `cargo fmt --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS (0 errors) |
| Focused `ofe_routing` suite | PASS **67/67** (64 + 3 Codex-review regressions, k_o pin replaced 1-for-1) |
| `cargo deny check` | PASS |
| `cargo nextest run --workspace --profile full` | **PASS — 1402 tests run: 1402 passed (7 slow), 2 skipped** (738.5 s) |
| Solver ladder re-dump (exact source history) | `logs/rr-oracle-solver-final.json` — peaks 0.008288/0.008391/0.008484/0.008402; errors -0.3%..+2.1% vs extrapolated oracle; `t_peak` within 0.06 s (IMPROVED vs pre-fix) |
| Relabeled seam ledger | `logs/rr-seam-ledger-final.json` — identities zero; explicit `terminal_sampled_quadrature_m3` -0.0004% |

Markdown lint: no repo-pinned markdownlint tool exists; `git diff --check`
clean stands as the docs whitespace gate (as at first closure).
