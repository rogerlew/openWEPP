# ASSURE-05 Heavy Gate Runner

Status: PASS

Evidence class: Ran

Date: 2026-07-16 UTC

Frozen base and current `HEAD`:
`01ed70550a4e371e99afe35c4bdd4d9b667e812c`.

The delegated heavy runner did not edit source, report, or test files. Each
failed attempt stopped at its first nonzero gate. The terminal attempt ran the
complete sequence from the remediated frozen working tree.

## Attempt 1 — Initial Strict-Clippy Failure

| Gate | Exit | Time | Result | Evidence |
| --- | ---: | ---: | --- | --- |
| `cargo fmt --check` | 0 | 2.39 s | PASS | `/tmp/assure05-terminal-cargo-fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 101 | 13.99 s | FAIL | `/tmp/assure05-terminal-cargo-clippy.log` |
| `cargo nextest run --workspace --profile full` | — | — | NOT RUN | stopped after Clippy failure; no run ID or counts exist |
| `cargo deny check` | — | — | NOT RUN | stopped after Clippy failure |
| Fresh adjudicated CRAP gate | — | — | NOT RUN | stopped after Clippy failure; no current counts or maxima exist |
| Assurance validate, plan, seeded build, and check | — | — | NOT RUN | stopped after Clippy failure |
| Scoped `markdown-doc lint` | — | — | NOT RUN | stopped after Clippy failure |
| `git diff --check` | — | — | NOT RUN | stopped after Clippy failure |

## Blocking Failure

Clippy reported three `clippy::float_cmp` errors in
`tests/integration/assurance_v2_groundwater_report_contract.rs`:

- line 137: exact comparison of `values["duration_days"]` with `731.0`;
- line 138: exact comparison of `values["ofe_count"]` with `19.0`; and
- line 139: exact comparison of `seepage` with `0.0`.

`-D warnings` promoted each lint to an error. Cargo therefore could not compile
the `assurance_v2_groundwater_report_contract` test target. The exact command
exited 101 after 13.99 seconds with maximum resident set size 376,532 KiB.

The failure is not masked or waived. Terminal heavy closure remains open until
the test is corrected within the package-authorized test write set and the
complete gate sequence is rerun from a newly frozen tree.

## Attempt 2 — Full-Nextest Failure After Clippy Correction

The test comparisons were corrected by the primary executor using the test's
explicit tolerance helper. The heavy sequence restarted from formatting.

| Gate | Exit | Time | Result | Evidence |
| --- | ---: | ---: | --- | --- |
| `cargo fmt --check` | 0 | 2.31 s | PASS | `/tmp/assure05-terminal-rerun-cargo-fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | 1.36 s | PASS | `/tmp/assure05-terminal-rerun-cargo-clippy.log` |
| `cargo nextest run --workspace --profile full` | 100 | 579.74 s | FAIL | `/tmp/assure05-terminal-rerun-nextest-full.log` |
| `cargo deny check` | — | — | NOT RUN | stopped after full-Nextest failure |
| Fresh adjudicated CRAP gate | — | — | NOT RUN | stopped after full-Nextest failure; no current counts or maxima exist |
| Assurance validate, plan, seeded build, and check | — | — | NOT RUN | stopped after full-Nextest failure |
| Scoped `markdown-doc lint` | — | — | NOT RUN | stopped after full-Nextest failure |
| `git diff --check` | — | — | NOT RUN | stopped after full-Nextest failure |

Full Nextest run ID:
`12302dab-2d6d-49fb-bf78-6c641a386a02`.

Nextest reported 2,049 tests run: 2,027 passed, 22 failed, 3 skipped, and 4
slow. Its own test time was 578.519 seconds; the complete command took 579.74
seconds with maximum resident set size 210,492 KiB.

All 22 failures were in the
`openwepp::assurance_v2_publication_contract` binary. The common failure is
that retained publication fixtures still assume the converted report and
principal registry have the former `test_only` trust domain. Representative
failures were:

- `publication_api_has_separate_production_and_test_trust_domains` expected
  production publication to reject the report as `test_only`;
- three snapshot/release tests could not replace the absent literal
  `trust_domain: test_only`;
- authority/publication tests failed to open their derived fixture because the
  principal registry version, trust domain, or cardinality was invalid; and
- `draft_subject_root_is_stable_but_cannot_publish` invoked the test-fixture
  publication path for the now production-domain draft.

The complete log preserves every failing test name and assertion. The failing
test file is outside this runner's write authority, and the renewed task also
prohibited test edits. The failure is therefore reported without remediation
or waiver. Terminal closure remains failed.

## Attempt 3 — Terminal Pass

The primary executor remediated the publication-fixture compatibility within
the amended package write set. The heavy sequence restarted from formatting
and every required gate passed.

| Gate | Exit | Time | Result | Evidence |
| --- | ---: | ---: | --- | --- |
| `cargo fmt --check` | 0 | 2.28 s | PASS | `/tmp/assure05-terminal-attempt3-cargo-fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | 0.91 s | PASS | `/tmp/assure05-terminal-attempt3-cargo-clippy.log` |
| `cargo nextest run --workspace --profile full` | 0 | 577.27 s | PASS | `/tmp/assure05-terminal-attempt3-nextest-full.log` |
| `cargo deny check` | 0 | 1.04 s | PASS | `/tmp/assure05-terminal-attempt3-cargo-deny.log` |
| Fresh adjudicated CRAP gate | 0 | 2,165.74 s | PASS | `/tmp/assure05-terminal-attempt3-adjudicated-crap.log` |
| Assurance validate | 0 | 0.78 s | PASS | `/tmp/assure05-terminal-attempt3-assurance-validate.log` |
| Assurance JSON plan | 0 | 0.23 s | PASS | `/tmp/assure05-terminal-attempt3-assurance-plan.log` |
| Seeded build/check A | 0/0 | 0.59/0.41 s | PASS | `/tmp/assure05-terminal-attempt3-assurance-{build,check}-a.log` |
| Seeded build/check B | 0/0 | 0.55/0.44 s | PASS | `/tmp/assure05-terminal-attempt3-assurance-{build,check}-b.log` |
| Complete staging-tree comparison | 0 | 0.00 s | PASS | `/tmp/assure05-terminal-attempt3-stage-diff.log` |
| Scoped `markdown-doc lint` | 0 | 0.02 s | PASS | `/tmp/assure05-terminal-attempt3-markdown-doc.log` |
| `git diff --check` | 0 | 0.03 s | PASS | `/tmp/assure05-terminal-attempt3-git-diff-check.log` |

### Full Nextest

Run ID: `f7960089-7439-420e-aa3b-293c7fa5d773`.

Nextest ran 2,049 tests: 2,049 passed, 3 skipped, and 4 slow. Nextest test
time was 576.031 seconds; the complete command took 577.27 seconds. Maximum
resident set size was 209,460 KiB.

### Adjudicated CRAP

The exact command was:

```console
bash tools/release/run_adjudicated_crap_gate.sh --base-ref 01ed70550a4e371e99afe35c4bdd4d9b667e812c --output-dir docs/work-packages/20260716-assure05-first-production-v2-report-001/validation-evidence/adjudicated-crap
```

The fresh gate assessed 9,262 production entries. It reported 2 raw rows over
30, 2 currently adjudicated rows, and 0 actionable rows. No production Rust
file was touched relative to the frozen base, so touched-file maximum CRAP is
not applicable and touched-file actionable count is 0. The maximum raw
workspace CRAP was 90; both raw rows were exact current adjudications. The
source manifest contained 228 paths and remained
`5f0446b67c84ecc1606a8adc6527adf75734ab82bda0df7ee62265635f593fcd`
before, after, and at finalization.

The closure artifacts are under
`validation-evidence/adjudicated-crap/`. The report records workspace CRAP
SHA-256 `413bc7035a416db70298bb341f1330891b43a82d5b10bccd0aa479369740ff3e`
and LCOV SHA-256
`d116cab75263f0163e64a42b8506096569cc31fdda6ac3fe6f0fc2977816d399`.

### Assurance Confirmation

Validation selected one report at version `1.0.0`, lifecycle `DRAFT`, with
`fixture_only=false` and report source root
`84a8467ff818411a34c89bf825fc2e9280a7c37c50db9b38636fc831546f4d01`.
The deterministic JSON plan passed with every node current.

Two unrelated scratch roots were seeded with the required model narrative:

- `/tmp/assure05-terminal-attempt3-stage-a.JqlgE3`;
- `/tmp/assure05-terminal-attempt3-stage-b.PxcrNu`.

Both narrative seed files had SHA-256
`c603508fc832e0f949c0b9bf20d77d46ad895a5dedfd4d2a59582ee5f0ec8e70`.
Both named builds and checks passed with build-manifest SHA-256
`bb95a9c09fde56f141b250e89a9efd4d9328465062a096cf9feafaefcc6ce499`.
`diff -qr` over the complete roots produced no output.

### Documentation And Diff

The scoped Markdown command validated 25 files with 0 errors and 0 warnings.
`git diff --check` produced no output. Terminal heavy closure is PASS; this
technical result does not create human scientific, assurance-steward, or
release-owner approval.
