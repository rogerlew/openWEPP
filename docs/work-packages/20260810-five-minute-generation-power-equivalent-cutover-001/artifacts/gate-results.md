# Gate Results

Status: `IN PROGRESS — focused corrections pass; exact-clean terminal campaign pending`

Evidence mode: `Static + Ran`

The intended result remains the optional `DIAGNOSTIC_ONLY` WAT5 water product
and the explicit erosion `NO_ADOPTION` branch. The production hourly-mean
erosion path remains selected.

## Reopened correction gates

| Requirement | Result | Evidence |
|---|---|---|
| Exact-worktree A0 | PASS | 43 contracts, 13 science surfaces, fingerprint `134c65ccfe96425cbbfbc822cf6c493a2993e952167fd2f85c24b24ff996c7a4` |
| Transactional failure integrity | PASS | existing-WAT5 sentinel preservation; day-2 no-partial-set; forced-close sibling preservation; forced-link and manifest rollback; successful commit |
| Storage-aware schema v2 | PASS | 27 columns; explicit depression retention; renamed raw post-depression generation; positive-storage Parquet reconstruction |
| Focused implementation | PASS | 762/762 affected-package tests and 14/14 named integration tests |
| Authority anti-evasion | PASS | shell guard, 3/3 required-suite contract, and 7/7 direct-policy contract |
| Affected Clippy / package Python | PASS | all targets/features with `-D warnings`; 5/5 Python tests |
| Enabled 45-year workload | PASS as diagnostic measurement | 4.11 s, 68,248 KiB peak RSS, 394,488 rows, 3,526,518 compressed bytes |
| Replacement full workspace / doctests | PENDING | must run at the exact clean implementation commit |
| Fresh science, Rust, and terminal reviews | PENDING | must bind directly to the replacement implementation SHA |

## Superseded 2026-08-10 closure receipts

The table below records the prior implementation closure. It does not close
the reopened package or support terminal acceptance of the replacement.

| Requirement | Result | Evidence |
|---|---|---|
| Predecessor terminal closure | PASS | exact detached source `a8a96498ee909c4305fbc0a4db562b72e45efd2b`; full 2,346/2,346, run `64cd5e97-d253-4da1-a3cf-3c4e16f83d22`; fresh dual terminal PASS |
| Feasibility/adoption decision | PASS | 1,008 prospective records; every fixed exponent rejected; `NO_ADOPTION`; no Topanga outcomes opened |
| WAT5 exact-worktree contract admission | PASS | `A0_ADMITTED contracts=43 science_surfaces=17 base=c9f28a7d... head=WORKTREE authority_sha256=6f95845b...`; scoped unit and Binding Exposure checks PASS |
| Authority anti-evasion shell guard | PASS | `bash tools/release/check_authority_suite_antievasion.sh` |
| Required-suite anti-evasion contract | PASS, 3/3 | nextest run `a424fd46-72b0-4d92-ae07-c30f7fde7bad` |
| Final WAT5/direct-authority contracts | PASS, 12/12 twice | nextest runs `c71d7b95-824e-4b27-837f-37ce77163822` and independent `3a63cdad-31e5-41b9-8cf4-623204765075`; includes complete-input fingerprint regression, 17 atomic WAT5 bindings, four conservative Plant+WAT5 shared-path bindings, and local approved-contract ownership |
| Focused WAT5 implementation gates | PASS | `implementation-test-evidence.md`; 18/18 orchestrator, 23/23 output, 13/13 named integration, 1/1 conversion, 21/21 unit registry, 5/5 Python |
| Real WAT5 consumer and independent closure | PASS | p61 26-column/24-row Parquet; raw residual `7.105427357601002e-15 mm`; independently reconstructed hourly residuals exactly zero |
| Source completeness | PASS | p102 rejects positive hourly-only supply with `WAT5-E-001`; no target or temporary WAT5 file |
| Protected output noninterference | PASS | diagnostics off/on WAT, PASS, HBP, and loss SHA-256 identities are byte-exact |
| Science Review A / B | PASS / PASS | `review_agent_a.md`, `review_agent_b.md`; all findings closed |
| Rust correctness / QA review | PASS / PASS | `rust_code_review.md`, `rust_qa_review.md`; all findings closed |
| Pre-repair full workspace nextest | PASS, 2,379/2,379; 33 skipped, superseded for terminal closure | profile `full`, run `eb241b16-8d4f-496a-8be4-b0648e893915`, `8,557.61 s`, `terminal-full-nextest.log`; predates Critical admission-policy repair |
| Pre-repair workspace doctests | PASS, 0 failed, superseded for terminal closure | `cargo test --doc --workspace`, `4.36 s`, `terminal-doctests.log` |
| Post-repair full workspace nextest | PASS, 2,380/2,380; 33 skipped | run `b920db77-070f-4686-a7bf-2e2727094374`, `8,753.02 s`, `terminal-full-nextest-post-a0.log` |
| Post-repair workspace doctests | PASS, 0 failed | `7.95 s`, `terminal-doctests-post-a0.log` |
| Quick-profile inclusion | PASS | final inventory: 2,331 matched quick tests, 2,380 matched full tests, zero quick-only tests, 49 full-only tests |
| Formatting / affected Clippy / deny | PASS | `cargo fmt --all -- --check`; affected all-target/all-feature Clippy with `-D warnings`; `cargo deny check` |
| Dual terminal implementation verification | PASS / PASS | `verification_agent_a.md`, `verification_agent_b.md`; both audit final hash `6f95845b...`, 2,380/2,380, doctests, and 87/87 manifest |

The earlier quick attempt `7fd4a181-...` is explicitly non-admitted: it was
interrupted after 54 passes when independent reviews opened implementation
defects. It is not used as closure evidence. The successful full inventory
contains every test selected by `quick`, so no second long-running quick
campaign is required.

The earlier base=head admission receipt reporting `science_surfaces=0` is also
non-admitted because it could not observe dirty/untracked implementation
paths. Terminal A0 uses the new explicit `--worktree` mode, which includes
untracked paths, binds all 17 changed science-crate paths, validates every
atomic authority on shared files, and fingerprints the complete authority
input surface. The unchanged draft `SC-WATBAL-001` is not promoted; diagnostic
authority and `TOL-WAT5-001` are consolidated in approved, active
`SC-OUTPUT-WAT5-001`.

The first post-repair attempt, run `cfd0d2d4-7e91-4feb-b8f1-c4054b061f6d`,
was interrupted as soon as Review B found an omitted fingerprint input: 43
passed, two received `SIGINT`, and 2,334 did not run. It is non-admitted. Run
`b920db77-070f-4686-a7bf-2e2727094374` started only after that final executable
repair and is the sole candidate terminal Critical receipt.
