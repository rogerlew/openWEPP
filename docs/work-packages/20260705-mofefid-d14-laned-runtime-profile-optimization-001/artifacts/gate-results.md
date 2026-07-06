# Gate Results

Status: **EXECUTED (D14-S5)**.

Subagent note: the package requires `comparator_suite_runner` for heavy
H2637/full-suite gates when available. That named role is Codex-session
tooling and is **not available in this Claude Code session**; per the package
fallback rule the heavy gates were delegated to a Claude general-purpose
subagent acting in the `comparator_suite_runner` role (read-only, compact
metrics + log paths), and endpoint timing/identity runs were executed
locally. Review roles (`rust_code_reviewer`, `rust_qa_reviewer`) were
likewise filled by independent Claude subagents; that substitution is
recorded here as the session-level tooling mapping.

## Package-specific gates (Ran, locally)

| Gate | Command | Result |
|---|---|---|
| Baseline H2637 default/off timing | `/usr/bin/time -v taskset -c 4 target/release/openwepp-cli-hill --run-dir run_off …` (3×, legacy + native variants) | PASS — 2.28–2.41 s wall; `baseline-timing.md` |
| Baseline H2637 Lane D shadow timing | same + `OPENWEPP_LANED_SHADOW=1` (3×) | PASS — 67.60/67.71/67.60 s wall; `baseline-timing.md` |
| Slot-level profiling evidence | `OPENWEPP_LANED_SHADOW=1 OPENWEPP_LANED_SHADOW_PROFILE=1 …` + `perf record -F 999` | PASS — `slot-timing-evidence.md` (H2637 itself profiled; no reduced fixture needed) |
| Before/after H2637 endpoint timing | same commands, optimized binary (3×) | PASS — 29.78/29.88/29.88 s wall; `baseline-timing.md` |
| Protected-output identity (default/off, shadow-off) | `cmp`/`sha256sum` vs pre-opt reference copies | PASS — byte-identical; `protected-output-evidence.md` |
| Routed-path closure/diagnostic parity before/after | canonical-JSON compare of the manifest `laned_shadow` block + trajectory counters | PASS — bit-identical; `protected-output-evidence.md` |
| Focused tests for changed profiler/timing/routing code | `cargo nextest run -p openwepp-hillslope-orchestrator -p openwepp-runner -E 'test(ofe_routing) or test(laned_shadow) or …'` | PASS — 64/64 |
| Markdown lint (touched docs) | `markdown-doc lint --path <touched docs> --no-ignore` | PASS — 21 files, 0 errors, 0 warnings |
| `git diff --check` | — | PASS (clean) |
| `cargo fmt --check` | — | PASS |

## Root closure gates (Ran by the delegated gate-runner subagent, 2026-07-06)

Runner report (verbatim results; sequential execution in the repo root):

| Gate | Result | Summary | Wall |
|---|---|---|---|
| `cargo fmt --check` | PASS | clean | 1.76 s |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | `Finished dev profile` | 12.04 s |
| `cargo nextest run --workspace --profile full` | PASS | `Summary [583.286s] 1387 tests run: 1387 passed (1 slow), 2 skipped` (run ID `be3352a6`; the one slow test is the pre-existing `snowdensity05e` snowbench) | 9 m 57.7 s |
| `cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture` | PASS | `PASS [226.860s] h2637_native_shadow_classifies_uniform_shape_after_d12` — the dev-profile off+on double run drops from `325.24 s` (D13 record) to `226.86 s` on the optimized code | 3 m 47.5 s |
| `cargo deny check` | PASS | advisories ok, bans ok, licenses ok, sources ok | 0.71 s |
| `git diff --check` | **FAIL → fixed → PASS** | the runner caught one trailing whitespace at `artifacts/optimization-disposition.md:43`; the space was removed and `git diff --check` rerun locally: clean | 0.03 s |

Logs: `nextest-full.log`, `h2637-ignored.log` under the session scratch
(`…/scratchpad/d14/`).

Post-fix re-checks (Ran, locally): `git diff --check` PASS;
`markdown-doc lint --path …/artifacts/optimization-disposition.md
--no-ignore` PASS.

## Not applicable

- Source-level anti-evasion guards — no required-case bindings, fixture
  governance, or authority-suite posture files were touched (Static:
  verified against the diff file list).
- Contract/BEI checks — no `SC-*` contract text was modified.
