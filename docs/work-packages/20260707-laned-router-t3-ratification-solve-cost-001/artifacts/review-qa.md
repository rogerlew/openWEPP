# QA Review

Evidence class:
- Static: reviewed the current diff, package artifacts, closure-gate logs, parent-package handoff edits, Tier-1/Tier-2 scaffold packages, and relevant AGENTS guidance.
- Ran: `git diff --check` (exit 0, no output), file/artifact discovery, `wc -l` on touched `.rs` files, artifact size checks, and process checks for active cargo/nextest gates. I did not rerun `cargo fmt`, `cargo clippy`, full `nextest`, or `cargo deny`.

## Findings

### HIGH - Required clippy gate fails while the package is marked executed-held

The package requires `cargo clippy --workspace --all-targets -- -D warnings` before disposition (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/package.md:167`) and the repo workflow makes the Rust closure loop required for implementation packages (`docs/work-packages/AGENTS.md:164`). The captured clippy log is a hard failure:

- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/cargo-clippy-full.log:2`
- `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/cargo-clippy-full.log:17`

The failure is `clippy::too_many_lines` on `run_iwagaki_cells_hybrid` at `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs:273`; the function currently spans through `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs:393`. That is a maintainability and required-gate blocker. The Case-4 ratification hold is legitimate, but it does not justify retaining new production/test support code that fails the required quality gate.

### HIGH - Gate results are stale and not disposition-grade

`gate-results.md` still says `Status: IN PROGRESS` (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/gate-results.md:3`) while the package and final disposition are already `EXECUTED-HOLD-CASE4-HYBRID-LADDER` (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/package.md:3`, `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/final-disposition.md:3`). The table also uses `PARTIAL` for a required gate (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/gate-results.md:9`), but package governance allows only `PASS`, `FAIL`, `BLOCKED`, or `NOT RUN` (`docs/work-packages/AGENTS.md:50`).

The table is also stale relative to the log artifacts: it lists clippy as `NOT RUN` (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/gate-results.md:17`) even though the captured clippy log fails, and the full-nextest log only shows startup/slow-test lines with no completion summary (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/closure-gates/cargo-nextest-full.log:1`). `cargo deny check`, markdown/doc lint, contract/profile checks, protected-output audit, and line-count governance remain `NOT RUN` or undocumented (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/gate-results.md:7`, `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/gate-results.md:8`, `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/gate-results.md:19`, `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/gate-results.md:20`). A held package with code changes still needs a truthful gate table for the code that will remain in tree.

### MEDIUM - Phase-F review, verification, and line-count artifacts are incomplete

The package plan requires dual review, dual verification, line-count governance, gate results, disposition, final disposition, and worker handoff (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/package.md:131`). It also authorizes expected outputs under `artifacts/review-*.md` and `artifacts/verification-*.md` (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/package.md:142`). At review time, the artifact set contains timing verification but no dual verification, no code-review artifact, no review-disposition artifact, and no line-count governance artifact; `gate-results.md` still marks line-count governance `NOT RUN` (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/gate-results.md:20`).

My local line-count check found the touched `.rs` files below the 2000-line WARN threshold, with `kinematic_wave.rs` highest at 1859 lines, but that evidence is not recorded in the package. This is straightforward to fix, but closure/disposition is premature until the package carries its own evidence.

### MEDIUM - The Case-4 hold is valid, but the retained gate is not currently reproducible from the recorded command

The hold-legitimacy audit records `cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick` as the failing ratification run (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/hold-legitimacy-audit.md:12`). The retained Case-4 hybrid test is now marked ignored in source (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs:95`), and the package records that the same broad `ofe_routing` run passes after the failing vector was quarantined (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/implementation.md:37`, `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/implementation.md:38`).

The failure numbers are credible and the hold posture is legitimate, but future reviewers need a current command/log that explicitly runs the ignored ratification vector, for example the test name with nextest `--run-ignored only` or `--ignored`. Without that, the documented gate command now exercises the green focused suite, not the held gate.

### MEDIUM - Catalog state is stale after final hold disposition

The work-package catalog still lists `20260707-laned-router-t3-ratification-solve-cost-001/` as `ACTIVE` (`docs/work-packages/README.md:15`, `docs/work-packages/README.md:16`) even though the package and final disposition say `EXECUTED-HOLD-CASE4-HYBRID-LADDER` (`docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/package.md:3`, `docs/work-packages/20260707-laned-router-t3-ratification-solve-cost-001/artifacts/final-disposition.md:3`). The queued Tier-1/Tier-2 catalog entries are present and coherent (`docs/work-packages/README.md:22`, `docs/work-packages/README.md:28`), but the parent package status should not remain active once the final disposition is held.

## Non-Blocking Debt / Follow-Ups

- The Tier-1 and Tier-2 scaffold packages are executable enough for queued work: each has `package.md`, active prompt, archived prompt placeholder, artifact directory, gates, and explicit subagent authorization. Before execution, add required-reading maps and placeholder artifact files for better ergonomics.
- The H2637 scratch artifacts are modest in size (~464 KiB total) and include run logs/hashes. If this pattern repeats, prefer committing a small reproducible patch/run script plus hashes over copied fixture trees.
- `git diff --check` passed in my local review run, but package evidence should record exit codes or command wrappers for empty logs such as `git-diff-check.log` and `cargo-fmt-check.log`.

## Verdict

NO-GO. The Case-4 hybrid ladder hold is legitimate and selector promotion is correctly blocked, but the package cannot be accepted with a failing required clippy gate, stale/incomplete gate results, and missing Phase-F governance artifacts.
