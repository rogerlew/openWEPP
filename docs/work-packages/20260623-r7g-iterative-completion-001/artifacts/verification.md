# Verification

Status: focused gates pass; package held on frost architecture boundary.

Evidence class: Ran.

Rust gates:

| Command | Result |
| --- | --- |
| `cargo fmt --check` | pass |
| `cargo test -p openwepp-runner r7g_ --lib` | pass, latest `5 passed` |
| `cargo test -p openwepp-hillslope-orchestrator r7g_ --lib` | pass, latest `6 passed` |
| `cargo test -p openwepp-hillslope-orchestrator direct_runtime --lib` | pass, `81 passed` |
| `git diff --check` | pass, latest HOLD edit check clean |
| `wctl doc-lint --path docs/work-packages` | pass, prior package-tree run: `964 files validated`, `0 errors`, `0 warnings` |
| `wctl doc-lint --path docs/work-packages/20260623-r7g-iterative-completion-001` | ran after HOLD edit, but validated `0` files because the package directory is currently untracked |

Release/H2637 endpoint loop:

| Command label | Result |
| --- | --- |
| `r7g_cont_direct_default6` | exit `0`, `13.21 s`, `871664 KiB` |
| `r7g_cont_explicit_direct1` | exit `0`, `13.53 s`, `870900 KiB` |
| `r7g_cont_default_disabled1` | exit `0`, `643.02 s`, `228604 KiB` |
| `r7g_cont_rollback1` | exit `0`, `636.68 s`, `229708 KiB` |
| `r7g_cont_direct_default7` | exit `0`, `13.16 s`, `871668 KiB` |
| `r7g_cont_direct_default_frost5` | exit `0`, `163.88 s`, `947704 KiB` |
| `r7g_cont_direct_default_frost6` | exit `0`, `122.43 s`, `948084 KiB` |
| `r7g_cont_direct_default_frost7` | exit `0`, `94.08 s`, `941936 KiB` |
| `r7g_cont_direct_default_frost8` | exit `0`, `106.64 s`, `939632 KiB`; rejected regression attempt |
| `r7g_cont_direct_default_frost9` | exit `0`, `95.88 s`, `941560 KiB` |
| `r7g_cont_direct_default_frost10` | exit `0`, `87.11 s`, `942324 KiB` |
| `r7g_cont_direct_default_frost11` | exit `0`, `89.88 s`, `941936 KiB`; retained active-frost source state |
| `r7g_cont_direct_default_frost12` | exit `0`, `101.16 s`, `941936 KiB`; no parity improvement, reverted |
| `r7g_cont_direct_default_frost28` | exit `0`, `107.96 s`, `941936 KiB`; day-5 frost trace |
| `r7g_cont_compat_frost28_trace` | failed later at `HKERNEL-WB12-STORAGE-E-003`, but produced day-5 trace rows needed for reduction |
| `r7g_cont_direct_default_frost29` | exit `0`, `188.57 s`, `941936 KiB`; fine carry preservation, performance red |
| `r7g_cont_direct_default_frost30` | exit `0`, `195.27 s`, `942324 KiB`; no-freeze coarse projection safeguard included, no-material consumer safeguard not yet measured |

Manifest checks:

- Direct default and explicit direct:
  `compatibility_edge_invocations=0`.
- Active-frost direct default retained run:
  `compatibility_edge_invocations=0`, `day_frame_commits=235961`,
  `direct_compute_operations=5225374`,
  `direct_state_mutations=5448916`.
- Direct default and explicit direct:
  `scheduler_kernel_executed=false`.
- Default-disabled and rollback compatibility output checksum maps compare
  equal.
- Direct default and explicit direct output checksum maps compare equal.
- Compatibility and direct output checksum maps do not compare equal.

Parity drilldown:

- WAT/PASS schemas and row counts match.
- loss JSON and plot output compare equal.
- HBP/WAT/PASS differ. Active frost is partially projected but still
  under-projects `frozwt`/`frdp`; snowpack/runoff residuals are also material.
- Additional day-5 traces:
  `/tmp/r7g-cont-h2637/traces/frost28-direct-day5.jsonl`,
  `/tmp/r7g-cont-h2637/traces/frost28-compat-day5.jsonl`, and
  `/tmp/r7g-cont-h2637/traces/frost30-direct-day5.jsonl`.
- Latest source state after `direct-default-frost30` has focused tests passing
  but no full H2637 rerun after the final R4A no-material consumer safeguard.

Not run:

- Full workspace `cargo test --workspace`, clippy, and `cargo deny check` were
  not run in this continuation.
- Full H2637 was not rerun after the final no-material consumer safeguard
  because the package is intentionally closed as
  `HOLD-R7G-FROST-STATEFUL-SUBSOLVER-REQUIRED`.
