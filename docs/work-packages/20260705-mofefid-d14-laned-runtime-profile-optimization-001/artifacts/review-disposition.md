# Review Disposition

Status: **EXECUTED (D14-S5)**.

Dual independent reviews were performed by Claude subagents in the
package-authorized roles (`rust_code_reviewer` = Review A, adversarial
bit-identity/code review of the working-tree diff; `rust_qa_reviewer` =
Review B, governance/truthfulness/scope QA). The named Codex-side roles are
unavailable in this session; the substitution is recorded in
`gate-results.md`. Review B additionally re-executed the bit-identity
evidence independently (fresh fixture copies; reproduced all six SHA256s,
the full-precision manifest block, the counters, and consistent endpoint
timing).

Review A headline: "I could not refute the bit-identity claim — the
restructure survives an adversarial operation-by-operation walk"; explicit
clean passes on f64-value/failure-ordering differences, OPT-3 delegation
bit-identity, instrumentation safety, other-caller surfaces, and Rust
quality. Review B headline: engineering substance "real, well-evidenced, and
survived independent executional re-verification"; explicit clean passes on
scope/protected surfaces, bit-identity, anti-tautology, instrumentation
acceptance, and non-goals.

## Findings and dispositions

| # | Source | Severity | Finding | Decision | Action taken |
|---|---|---|---|---|---|
| A1 | Review A | medium | The four profiling tests toggle the process-global enable flag and deterministically fail under plain `cargo test` (3/3 repro); the justifying comment cited a nonexistent repo convention. | accepted | Added crate-internal `profile::test_flag_guard()` mutex; all four flag-toggling tests hold it; comments corrected to state the real mechanism. Ran: the reviewer's exact repro now passes 3/3; full `--lib` libtest run 294/294; focused nextest 59/59. |
| A2 | Review A | low | Error-surface reordering (intensity validated at loop top instead of inside `step`) — analyzed identical `Result` for every reachable input; forcing closures called fewer times (all in-repo closures pure). | accepted (record-only) | Recorded here; no code change required. The analysis matches the executor's pre-implementation bit-identity argument in `optimization-disposition.md`. |
| A3 | Review A | low | Bit-identity grid test omitted `Re = NaN`. | accepted | Added `f64::NAN` to the Reynolds grid; test passes. |
| A4 | Review A | low | Profiling lifecycle notes: global flag + thread-local accumulator is single-thread-correct today (runner is single-threaded; watershed parallelism is subprocess-based); `OPENWEPP_LANED_SHADOW_PROFILE=1` without the shadow env is silently inert. | accepted (documentation) | Interpretation notes added to `slot-timing-evidence.md` §5. |
| A5 | Review A | info | `alpha_evaluations` counts dry-cell early-return calls. | accepted (documentation) | Counter-semantics note added to `slot-timing-evidence.md` §5. |
| B1 | Review B | blocker (at snapshot time) | `gate-results.md` was a PENDING placeholder with required gates unrecorded. | accepted | Review B read the artifacts mid-flight, before the delegated gate runner reported. `gate-results.md` now records all package gates plus the runner's verbatim root-closure results (1387/1387 full nextest, H2637 ignored PASS 226.86 s, deny PASS, fmt/clippy PASS). |
| B2 | Review B | high (at snapshot time) | `protected-output-evidence.md` cited gate evidence that did not yet exist. | accepted | The citation now resolves: the H2637 ignored-run result is recorded in `gate-results.md`. |
| B3 | Review B | high (at snapshot time) | D15 runtime-budget handoff missing; `hold-legitimacy-audit.md` asserted completion prematurely. | accepted | `worker-handoff.md` authored with the explicit budget, risks, and refresh rule; closure artifacts written in dependency order thereafter. |
| B4 | Review B | medium | `git diff --check` failed on one trailing whitespace in `optimization-disposition.md:43` (also caught by the delegated gate runner). | accepted | Whitespace removed; `git diff --check` re-run clean. |
| B5 | Review B | medium | OPT-3 artifact sentence "exactly one implementation" was false (regime dispatch exists in two textual places; `shen_li` retains its inline form). | accepted | Artifact corrected to describe the real structure; the bit-identity grid test is named as the binding between the two dispatches. |
| B6 | Review B | low-medium | New `ofe_routing/profile.rs` + module registration were outside the enumerated conditional write set (authorized in spirit by the Included scope). | accepted | Write-set deviation note added to `package.md`. |
| B7 | Review B | low-medium | Kickoff-required `comparator_suite_runner` delegation/attribution unrecorded at snapshot time. | accepted | `gate-results.md` opens with the role-mapping and delegation record. |
| B8 | Review B | low | Post-optimization timing table omitted the Sys column. | accepted | Final-code confirmation series re-run on an idle machine with wall/user/sys recorded. |
| B9 | Review B | low | Line-count "Before" column off by one vs `git show HEAD` blob counts. | accepted | Column corrected to the git-blob values (1002/445/329/505/1260). |
| B10 | Review B | low (note) | Global-flag tests rely on process isolation. | accepted | Same surface as A1; resolved by the A1 mutex fix (stronger than the note asked). |
| B-nit | Review B | note | "Step-for-step identical" phrasing overclaims — counters prove count-equality; the conjunction with bit-identical diagnostics and hashes carries the claim. | accepted | Wording adjusted in `baseline-timing.md`, `docs/work-packages/README.md`, and the strategy §6.1 row to the counter-witness framing. |

No finding was rejected or deferred. All accepted findings are fixed or
recorded as specified; independent verification of the fixes is in
`verification-disposition.md`.
