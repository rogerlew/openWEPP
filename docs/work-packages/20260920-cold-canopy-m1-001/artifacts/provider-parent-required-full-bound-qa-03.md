# Required full-validation recorder QA — 03

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static recorder diff plus supplied Ran non-launching boundary-control receipt. This reviewer did not execute a recorder command or full-workspace regression.

## Findings

No blocking finding in the repaired recorder correction.

The only change from preserved failed cut01 (`provider-parent-recorder-full-exception-cut01.py`, SHA-256 `7c2219447b3a042e24d589f1382f235601d29a881ce59fce564c00f719ac8414`) replaces the second unconditional physical-over-180 check with the same `validate_declared_bound(...)` call used at initial preflight. It recomputes the remaining time immediately before that call, so the full declared 900-second bound still must fit before the 1800-second reserve.

The allowlist remains exact: the flag requires physical classification, timeout 900, and the established Nix/environment `cargo nextest run --workspace --profile full --no-fail-fast` argv. The support record must independently bind argv, timeout, physical policy, and `required_full_validation`. Ordinary physical commands above 180 seconds, altered argv/environment, flag misuse, nonphysical full validation, and reserve overrun remain rejected. Source/support/binary custody, unique receipt reservation, and no-retry behavior are unchanged.

## Guard evidence

`provider-parent-full-bound-controls-02.json` reports 15 passing non-launching cases. The added main-path case constructs a current source/support-bound manifest, runs both real guard positions and custody preflight, and intercepts `subprocess.Popen` before a process can start. It records two guard calls and `processes_launched: 0`. The helper cases retain the exact allowlist, altered-token/environment, physical/nonphysical, wrong-bound, and reserve-bound negatives.

## Non-blocking follow-up

No full-workspace command ran. A later run still requires its own current frozen source, support/binary manifest, reviewer clearance, and recorder reserve preflight.

## QA disposition

**PASS — release the recorder correction and its non-launching guard controls.** This permits review of the exact established 900-second Critical full-validation command only; it does not clear or launch that command.
