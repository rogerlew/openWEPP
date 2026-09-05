# Gate results

Status: `TERMINAL HOLD — BOUNDED CORRECTNESS RETAINED; RELEASE/PERFORMANCE QUALIFICATION FAILED`

Evidence mode: `Static + Ran`

Terminal base commit: `a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`.
The sorted existing changed/untracked Rust-source manifest has SHA-256
`2813f6e8faabb9408bac5e59b9271626ff5bcdc7fe49ab6dda810d3a1c3eee0d`.
It is reproducible with:

`git ls-files -m -o --exclude-standard -- '*.rs' | sort | while IFS= read -r file; do if test -f "$file"; then sha256sum "$file"; fi; done | sha256sum`

The current release runner test binary has SHA-256
`5d70ce0966222480e20f12a468e47fc32fdc136e77e90a87334270a3289e6564`.

## Exact-source bounded increment gates

| Gate | Result | Evidence |
| --- | --- | --- |
| `cargo nextest run -p openwepp-land-surface-energy --no-fail-fast` | PASS | `140/140`; includes forced-exhaustive exact-beta, inactive, exact-zero-PAR, call-order/error, and complete hydraulic fallback vectors. |
| LSE `cargo clippy --all-targets -- -D warnings` | PASS | exit 0 on terminal Rust source. |
| `cargo check -p openwepp-runner --all-targets` | PASS | exit 0. |
| `cargo fmt --all -- --check` | PASS | exit 0. |
| `git diff --check` | PASS | exit 0 after final artifact reconciliation. |
| `bash tools/release/check_authority_suite_antievasion.sh` | PASS | source anti-evasion guard passed. |
| `cargo nextest run --test auth11_required_suite_obligation_guards_contract --no-fail-fast` | PASS | `3/3`. |
| exact CPU-0 one-OFE release diagnostic | PASS science/counts; DIAGNOSTIC ONLY | Post-revert single warm repetition: `4,920,992 us`, RSS `60,708 KiB`, potential `352,898 us`, physical evidence `1,018,388 us`; exact source/outlet/storage/clamp and 48/56/20/32/4 counts unchanged. This mixed-regime point is retention evidence, not a pure snow-free or complete 10-OFE budget surface and not the required 5-warmup/30-batch qualification protocol. |

The release command was:

`timeout 1800 taskset -c 0 env RUST_MIN_STACK=67108864 CARGO_PROFILE_RELEASE_LTO=false nix develop -c cargo test --release -p openwepp-runner --lib hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile -- --ignored --exact --nocapture --test-threads=1`

## Required failing/deferred gates

| Gate | Result | Disposition |
| --- | --- | --- |
| isolated 1/10/19-OFE release matrix | FAIL (historical tested source) | Science/scaling passed; wall/RSS budgets failed at `5.333934174/12.4533514695/22.8885138375 s/day`. Its log binds tree digest `c5e9…` and binary `edb119…`, not current `f065…`/`5d70…`; it is retained prior-source failure evidence only. Log: `terminal-heavy-gates/release_qualification_matrix_1_10_19_ofe_rerun3.log`. |
| orchestrator all-target warnings-denied Clippy | FAIL | Broad warning inventory remains; no blanket allow/suppression was added. |
| full-workspace nextest rerun | FAIL | Historical exact run reached all `4,145` tests but one runner trace-writer test failed on a missing output path and the long Stage-3 test was terminated; retained log/meta under `terminal-heavy-gates/cargo_nextest_workspace_no_fail_fast_rerun.*`. Not rerun after terminal test-only C-018 addition because the package is nonqualifying. |
| 10-OFE year/century and 5,000-hillslope qualification | NOT RUN | The prior-source 10-OFE matrix validly fails the applicable complete-day CPU/wall limits by orders of magnitude; projections remain explicitly nonqualifying. |

Validators are not relabeled as workflow evidence. The exact release workload,
matrix, and authentic consumer runs remain the workflow evidence. These
results support only the bounded retained increments. Exact-workspace
correctness, release, and performance qualification remain unestablished;
the package is dispositioned `HOLD`.

## Final revision-31 candidate and reversion gates

| Gate | Result | Evidence |
| --- | --- | --- |
| corrected authority review and preimplementation verification | PASS | Dual corrected review and dual verification accepted ordered authority manifest `767bc190...b1583`. |
| candidate implementation review | PASS FOR MEASUREMENT | Both reviewers approved exact 16-path candidate manifest `edc3f0b9...be71`; all accepted implementation findings were closed for that candidate. |
| focused component/full LSE/authentic runner parity | PASS | `14/14`, `154/154`, and release-profile replay-versus-forced parity `1/1`; graph, custody, audit, full-solve, backtracking, owner, and output-byte evidence passed. |
| exact three-run candidate release gate | FAIL ON RUN 1 | Source `039a3125...1abc`; binary `f9386eec...eeaf`; exit `101` before JSON because no authentic completed `N=2,S=6` `58/14/16/28` sweep was present. Runs 2/3 were not run under the conjunctive protocol. |
| mandatory full candidate revert | PASS | All v31 implementation/test-support/runner files and seams removed; no semantic residue. |
| post-revert structural/full LSE/check gates | PASS WITH EXPECTED RED | Exactly seven absent declarations produce the named expected red; LSE `140/140`; LSE/orchestrator/runner all-target checks, formatting, diff hygiene, and residue checks pass. |

The current manifest differs from the pre-v31 frozen baseline only because the
authority test received 23 authorized post-baseline contract/parser patches.
An in-memory reversal reconstructed authority-test SHA
`912bb3deae3708f681a82417a631ebf6dcb7079e84ab64542ebbba00e8772096`
and the exact frozen aggregate `78d756be...bbbe`; no authority patch was
incorrectly reverted. The current release binary named near the top is
historical retained-source diagnostic evidence. The failed v31 candidate
binary is not current production.

The final ordered eight-file component authority/evidence manifest described in
`science-contracts/component-temperature-dependency-replay/contract_ref.md`
hashes to
`193b9854f6d9a0c5013352d8d313669e01ce0494a8021e5b1d90b77c151d0f35`.
This terminal evidence digest supersedes earlier preimplementation authority
digests only for document custody; it does not retroactively change the exact
candidate implementation manifest reviewed before release measurement.
