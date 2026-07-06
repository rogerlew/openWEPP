# Timing Refresh

Status: **EXECUTED-HOLD**.

Evidence mode: Ran + Static.

## Purpose

Refresh D14 endpoint timing on the D10B-corrected Lane D path before any D15
activation claim.

## Prior D14 Budget

Static:

- Default/off H2637 2-year fixture: about `2.3 s` wall/user.
- Lane D shadow-on after D14 optimization: about `29.8-29.9 s` wall/user.
- Shadow overhead: about `+27.5 s` user CPU over default/off.
- Solver trajectory witness at D14: `10,334,879` steps,
  `1,134,300` hydrograph samples, `10,155,779` upstream interpolations.

## Current D10B-Corrected Run

Ran by package-authorized `comparator_suite_runner` subagent, read-only:

| Command | Result | Timing / evidence |
|---|---|---|
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | PASS | release CLI built successfully |
| default/off native H2637 timing: `taskset -c 4 /usr/bin/time -v target/release/openwepp-cli-hill --run-dir .../run_off_native --run-file p2637.run.toml --output-dir .../run_off_native/output` | PASS | user `2.58 s`, sys `0.01 s`, wall `0:02.60` |
| shadow-on native H2637 timing: `OPENWEPP_LANED_SHADOW=1 taskset -c 4 /usr/bin/time -v target/release/openwepp-cli-hill --run-dir .../run_on --run-file p2637.run.toml --output-dir .../run_on/output` | FAIL | exit `1` after user `20.05 s`, wall `0:20.06`; error `CLIHILL-E-011 ... HS-SIMPIPE-E-001 direct publication sink failed: laned shadow cascade: NegativeOutletBin` |
| shadow-on + profile: `OPENWEPP_LANED_SHADOW=1 OPENWEPP_LANED_SHADOW_PROFILE=1 taskset -c 4 /usr/bin/time -v target/release/openwepp-cli-hill --run-dir .../run_on_profile --run-file p2637.run.toml --output-dir .../run_on_profile/output` | FAIL | exit `1` after user `20.62 s`, wall `0:20.63`; same `NegativeOutletBin`; no `laned_shadow_profile` JSON emitted |

Package-local copied logs:

- `artifacts/logs/exit_codes.txt`
- `artifacts/logs/off.time.err`
- `artifacts/logs/shadow_on.time.err`
- `artifacts/logs/shadow_profile.time.err`

Provenance note: the timing subagent originally wrote its scratch run
directories under the prior D15 package path
`20260705-mofefid-d15-opt-in-production-activation-001/artifacts/phaseA-refresh/`.
Only the small timing/exit logs were copied into this rerun package; the
misplaced fixture/output scratch tree was removed. The command paths embedded
inside the copied `*.time.err` logs therefore reference that original scratch
location.

Ran locally for independent reproduction:

```sh
cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only --no-capture
```

Result: FAIL, `0` passed / `1` failed / `1` skipped, `110.804 s`. The ignored
H2637 evidence test fails with the same active shadow error:

```text
native-routed H2637 must run with Lane D shadow enabled:
RuntimeSurfaceFailure { surface: "r7c_direct_production_executor",
detail: "HS-SIMPIPE-E-001 direct publication sink failed:
laned shadow cascade: NegativeOutletBin" }
```

Ran locally with a temporary diagnostic-only error-context patch that was not
retained in the final worktree. The behavior was unchanged; the added context
identified the failing buffered day:

```text
laned shadow cascade day 88 window_s=86400 last_active_hour=24:
NegativeOutletBin
```

Static interpretation: the failure is not caused by the shadow's normal
`active source span + 6 h` clip shortening the event. Day 88 already uses the
full `86400 s` one-day window because the source remains active through
1-based hour `24` (`laned_shadow.rs` computes this window at lines 460-480).
The D10B terminal-bin guard now prevents publishing a negative outlet bin
before an endpoint timing or slot profile can be produced.

## Timing Decision

`BLOCKED`. The D15 rerun cannot proceed on timing grounds because the required
Lane D opt-in/shadow endpoint timing path does not complete.

The default/off path is acceptable for this rerun despite the small drift
(`2.58 s` user / `2.60 s` wall vs prior ~`2.3 s`, about `12-13%` slower).
No shadow-on endpoint, solver counters, step counts, or slot-profile evidence
exists for the D10B-corrected candidate.

First timing follow-on: close the H2637 day-boundary / terminal-bin handling
defect under `SC-OFEROUTE-001` before any activation timing claim. A production
activation package must not paper over this by silently extending, truncating,
or reshaping the routed hydrograph without authority for the 24-hour active
consumer and inter-day storage/hydrograph semantics.
