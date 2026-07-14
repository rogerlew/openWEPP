# Independent Review B

Evidence class: **Static + Ran**

Initial recommendation: **HOLD**

Review scope: terminal working-tree changes for the adjudicated CRAP closure
gate and the growth-state CQR follow-on, with emphasis on anti-evasion,
fail-closed behavior, evidence provenance, CI/release integration, and package
truthfulness. This review was completed independently and did not consult
Reviewer A.

## Findings

### B-01 — High, blocking: registry evidence is path-present, not evidence-bound

`tools/release/check_adjudicated_crap.py:238-306` accepts any syntactically
40-hex `adjudicated_at_commit` and any two distinct existing review paths. It
does not prove that the commit exists, that the registered file hash is the
file hash at that commit, or that the referenced review artifacts are the
original artifacts that accepted this file, symbol, classification, CC, and
hash. The focused fixture makes the gap concrete:
`tests/python/test_adjudicated_crap_gate.py:51-54` creates three one-heading
files, line 78 supplies a non-resolving synthetic commit, and
`tests/python/test_adjudicated_crap_gate.py:106-116` still expects and receives
`PASS`.

This does not satisfy ADR-0021's symbol-level evidence requirements or
`ACRAP-003`, `ACRAP-005`, and `ACRAP-012` in
`docs/work-packages/20260713-adjudicated-crap-closure-gate-001/package.md:115-124`.
An accidental new registry row can suppress actionable debt by copying the
current source hash and pointing at unrelated existing Markdown files.

Required disposition: verify that the adjudication commit resolves and that
the registered source file has the registered hash at that commit. Bind each
adjudication and both independent review artifacts by immutable SHA-256 plus
explicit accepted-disposition metadata, and add negative tests for a missing
commit, historical source mismatch, changed evidence content, and unrelated
evidence.

### B-02 — High, blocking: retained evidence can masquerade as current evidence

`tools/release/run_adjudicated_crap_gate.sh:79-127` knows whether it collected
fresh LCOV or received `--crap-json`, but passes no acquisition mode or source
provenance to the checker. `tools/release/check_adjudicated_crap.py:568-614`
then stamps the current repository HEAD and dirty-worktree state onto either
kind of report. The actual retained-artifact reproduction at
`/tmp/openwepp-acrap-focused/adjudicated-crap-report.json` is a `PASS` report
with current repository metadata and the historical CRAP hash, but contains no
field identifying it as retained or ineligible for current-source closure.

That conflicts with the binding rule at
`docs/work-packages/AGENTS.md:179-184` that old evidence cannot close current
implementation work. It also weakens the agent/human audit purpose because the
machine artifact is not self-describing.

Required disposition: record `acquisition_mode` (`fresh` or `retained`), the
LCOV hash and pinned tool versions for fresh acquisition, and retained-source
provenance for imported evidence. A retained assessment must explicitly report
that it is not current-closure evidence; it should not be possible to combine
`--base-ref` current-worktree stamping with a retained artifact without that
non-closure status being machine-visible. Add a focused test asserting the
distinction.

### B-03 — Medium, blocking: failed CI gates do not publish the reports

In `.github/workflows/release-gates.yml:98-138`, `release_dir` is written to
`GITHUB_OUTPUT` only after the release command returns successfully, and the
upload step has no `if: always()` condition. Because the release script is
fail-fast, a CRAP failure leaves the most important human and machine failure
reports on the runner and skips artifact upload. This contradicts
`ACRAP-006` at
`docs/work-packages/20260713-adjudicated-crap-closure-gate-001/package.md:118`.

Required disposition: publish the known release directory output before
starting the command and run the artifact upload under `if: always()`. Preserve
the failing report/log set even when a prior workspace gate prevents CRAP
collection.

### B-04 — Medium, blocking: the hosted CI job does not provision Nextest

`.github/workflows/release-gates.yml:91-97` installs `cargo-deny`,
`cargo-llvm-cov`, and `cargo-crap`, but not `cargo-nextest`.
`tools/release/run_release_candidate_gates.sh:487-492` unconditionally invokes
`cargo nextest`. A clean `ubuntu-latest` job therefore depends on an undeclared
runner-image tool instead of a reproducible provisioned dependency.

Required disposition: install a supported, pinned Cargo Nextest release (or a
pinned installation action) before invoking the release script, and record the
version with the other gate tools.

### B-05 — Medium, blocking: deleted production files disappear from the touched census

`tools/release/check_adjudicated_crap.py:340-380` uses
`--diff-filter=ACMR`, excluding `D`. A deleted `crates/*/src/*.rs` path is still
a production file touched since the frozen base, but the report omits it. The
current touched-path test covers modified and untracked files only
(`tests/python/test_adjudicated_crap_gate.py:148-181`). The workspace-wide debt
check remains conservative, but the audit claim and `ACRAP-004` are not met.

Required disposition: include deleted paths, and preferably record Git status
so deletion/rename provenance is unambiguous. Add deletion and rename coverage
to the focused tests.

### B-06 — Medium, blocking: reused output directories can retain stale PASS evidence

`tools/release/run_adjudicated_crap_gate.sh:77-106` creates but does not clean
or transactionally replace its output directory. Version mismatch, coverage
failure, cargo-crap failure, or checker input failure exits before replacing
all prior outputs. The default output path is reusable. In addition,
`tools/release/run_adjudicated_crap_gate.sh:127-133` writes `sha256sums.txt`
only after a passing checker, and the manifest omits `workspace.lcov`, tool
versions, and logs. A failed rerun can therefore leave an old PASS report and
old hash manifest beside partial new evidence.

This conflicts with the recovery claim at
`docs/work-packages/20260713-adjudicated-crap-closure-gate-001/package.md:171-175`
and materially reduces artifact auditability.

Required disposition: clear the known generated filenames before acquisition
or build in a temporary run directory and atomically publish one completed run.
Emit an explicit failure envelope/manifest on every exit path and bind all
metric-lineage artifacts, including LCOV, CRAP JSON, versions, and logs.

### B-07 — Low, nonblocking: coverage-failure attribution should name the proven cause

The terminal heavy artifact records the failed instrumented target but only
states that full Nextest is ordinary-test authority
(`docs/work-packages/20260713-cqr-growth-state-crap-regression-001/artifacts/heavy-run.md:73-76`).
The target itself explicitly documents that stock threaded `cargo test` races
its process-global environment mutations and is supported only under Nextest
process isolation (`tests/integration/laned_shadow_h2637.rs:1-8`). The observed
failures at `/tmp/openwepp-acrap-terminal-20260713/llvm-cov.log:1177-1219`
match that exact failure mode.

Recommended disposition: add that direct attribution and source citation to
the heavy evidence rather than leaving the failure merely superseded.

### B-08 — Low, nonblocking: growth characterization should bind exact test IDs

The helper extraction is an exact move and the terminal report closes the
caller at CC 27/CRAP 27.015625 and the helper at CC 5/CRAP 5. The package's
characterization artifact nevertheless refers generically to “annual and
perennial equation-state tests”
(`docs/work-packages/20260713-cqr-growth-state-crap-regression-001/artifacts/characterization.md:8-16`).
The exact consumer tests are
`r5d_annual_growth_phase_computes_mutates_downstream_shadow_and_r4n_context`
and `r5d_perennial_growth_phase_supports_grazing_after_annual_phase_identity`
in `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_r5d.rs:12-122`.
Also, the package retrospective still says implementation is pending after the
implementation is recorded complete
(`docs/work-packages/20260713-cqr-growth-state-crap-regression-001/package.md:103-132`).

Recommended disposition: name the exact tests and correct the stale
retrospective before final closure.

## Positive Results

- Static diff review confirms the root mass/root depth block was moved without
  changing comparisons, arithmetic grouping, branch order, tuple order, or
  caller-side validation.
- The fresh terminal report records `raw=2`, `adjudicated=2`,
  `actionable=0`, with the touched growth file present. The target caller is
  CC 27 / CRAP 27.015625; the extracted helper is CC 5 / CRAP 5 with 100%
  reported coverage.
- Both historical formatter source hashes match their files at commit
  `fa50c0becf6ea63fd9697b4cfe9add66ae036207` and match the current files.
- The terminal heavy artifact records passing formatting, all-target Clippy,
  1,960 full-profile Nextest tests, and `cargo deny` on an unchanged growth
  source hash.

## Reviewer-Executed Checks

| Check | Result |
| --- | --- |
| `.venv/bin/python -m unittest -v tests.python.test_adjudicated_crap_gate` | PASS, 8/8 |
| `bash -n tools/release/run_adjudicated_crap_gate.sh tools/release/run_release_candidate_gates.sh` | PASS |
| `sha256sum -c /tmp/openwepp-acrap-terminal-20260713/sha256sums.txt` | PASS, all four listed artifacts |
| `git diff --check` | PASS |

## Initial Disposition

The growth decomposition is acceptable at a high level, but the integrated
gate is not yet closure-ready. Findings B-01 through B-06 are blocking. Keep
both packages `ACTIVE`/`HOLD` until accepted fixes are applied, affected focused
and terminal gates are rerun, every finding is dispositioned, and Reviewer B
performs independent post-disposition verification.
