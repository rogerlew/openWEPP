# Revision-61 memory attribution

Status: `PHASE 3 COMPLETE — CURRENT CONTROL VALID; HISTORICAL CAUSE UNRESOLVED`

Evidence mode: `Static + Ran`

## Measurement semantics

`rss_kib` is the first numeric field of `VmRSS:` in `/proc/self/status`, in
KiB, read by `release_probe_rss_kib()` while the release test is constructing
its JSON report. Output validation, the committed snapshot, telemetry, report
objects, allocator-retained pages, mappings, and thread stacks are still live;
fixture teardown occurs afterward. It is therefore a point-in-time resident
set sample, not peak RSS, `VmHWM`, heap bytes, allocator return, or proof of
cleanup. The exact source anchor is
`crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs:1070-1118`.

The admitted R1 control used fresh CPU-0 processes, exact binary
`9a91c82f...d8e573f`, `RUST_MIN_STACK=67108864`,
`CARGO_PROFILE_RELEASE_LTO=false`, one test thread, two warmups, and balanced
AB/BA twelve-block ordering. Arm B sampled `/proc/<pid>/status` and
`smaps_rollup` with a requested 20 ms sleep; observed timestamp spacing includes
read/parse overhead and is not treated as a 20 ms achieved cadence. Readiness
was verified by `/proc/<pid>/exe` and persisted as `readiness_status=ready`.

## Admitted current-baseline control

Raw custody: `raw/current_memory_results.jsonl` SHA-256
`08aa5e5c1282a22e0eceffb40c6fa24ad880bed8b8dcc52d9aef3ab9720b08d0`, samples
`0a7aa5cf742333de2b48c03b7ad5c8183b5e02621142ae3dd7dbe64cd3f7be18`, manifest
`7ef7ba6c68330485bcd5b946ec4242f886285dfe4594a611d8145b97d766b320`. There
are 26 result rows (2 warmups + 24 arms), 12 complete observed arms, 1,611
CSV rows, all exit 0, all probes present, every B arm ready/complete, and zero
field gaps. Offline checks reconcile every sample count, peak, and
`private_total_kib = Private_Clean + Private_Dirty`.

| Quantity | Arm A endpoint | Arm B endpoint / observer | Interpretation |
|---|---:|---:|---|
| Endpoint VmRSS median (KiB) | 70,850 | 70,556 | paired median B−A = −298 KiB; endpoint noise dominates |
| Endpoint VmRSS range (KiB) | 59,256–71,040 | 52,020–71,052 | 8/12 exceed 64 MiB in each arm |
| B peak VmRSS median (KiB) | — | 72,946 | external live-process peak |
| B sampled VmHWM median (KiB) | — | 75,664 | maximum successfully sampled `/proc/<pid>/status` high-water value; a lower bound on any later lifetime peak |
| B smaps Rss median (KiB) | — | 72,530 | mapping-accounted resident set |
| B private clean+dirty median (KiB) | — | 72,522 | private mapped pages only |
| Run-wall median (us) | 4,971,911 | 5,557,731.5 | observer adds median +577,644 us paired |

The B observer is a measurement perturbation: all 12 B arms completed and
preserved exact science/counters/closure. Every precise paired B−A wall-time
delta was positive (minimum +48,930 us; median +577,643.5 us). Endpoint RSS
does not increase consistently under observation (4/12 paired endpoint deltas
were positive; paired median −298 KiB); this control cannot be used as a
revision-61 treatment effect.

## Historical contrast

Archived endpoint samples, each three unpaired process runs, are:

- retained baseline: `70,696 / 54,624 / 59,364 KiB` (median 59,364);
- revision 61 feed-forward candidate: `69,768 / 59,504 / 70,484 KiB`
  (median 69,768).

The ranges overlap and both groups contain values above the 64 MiB ceiling.
Revision-61 wall times were approximately 3.9M us versus approximately 4.9M
us for the retained component baseline, but the endpoint samples were not
balanced or lifecycle-matched. No historical HWM, smaps, heap, allocator,
stack, mapping, post-drop, or cleanup measurements exist. The historical
candidate source was not recovered as an equivalent executable. The ordered
revision-61 session inventory contains 38 source patch attempts; 22 applied and
16 failed in an isolated exact-HEAD replay, and the resulting tree failed
`cargo check` (raw replay status/compiler transcript). A cut-aware detached
reconstruction was then manually reconciled until `cargo check --tests` and
the real one-OFE release probe passed, with 200 feed-forward calls, exact
protected closure, and `rss_kib=58,440 KiB`; however, the documented
changed-Rust manifest for that tree is `19a07121…23421` (the ordered 13-row
stream is `raw/v61_manual_candidate_changed_rust_hashes.txt`; an earlier
undocumented canonical computation recorded `2fc5e8ca…e6eb7f`). Neither is
comparable to the historical `650f6713…57d41` digest because the historical
Git-index/dirty-worktree state, per-file manifest, and candidate binary are not
custodied. Consequently the revision-61 versus baseline difference has **no
supported causal attribution**; the replay and manual-run records are evidence
about source recoverability/feasibility, not about memory mechanism behavior.
The supported conclusion is only that `VmRSS` endpoint variability and the
current observer perturbation are material, while the historical source of the
RSS difference remains an unresolved discriminator.

Any future candidate claim requires matched fresh-process treatment pairs,
endpoint VmRSS plus VmHWM/smaps attribution, explicit lifecycle timestamps,
binary/source custody, exact science/output parity, and three repeats per
treatment. It must not call endpoint RSS a peak or heap measure.
