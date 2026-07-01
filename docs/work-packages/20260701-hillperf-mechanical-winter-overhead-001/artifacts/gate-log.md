# Gate Log

Evidence class: Ran (every row is an executed command on this worktree).
Host: dual Xeon E5-2697 v2; concurrent watershed-package load present, so wall
times are single-rep **indicative**; identity results are load-insensitive.
Identity reference: the frozen 2026-07-01 baseline hashes in `package.md`
(reproduced by the clean worktree binary before any edit — entry gate).

| Gate | Tree state | Wall (indicative) | Max RSS | Identity vs frozen baseline |
|---|---|---:|---:|---|
| Entry (baseline reproduction) | clean branch @ base | 70.05 s | 82,660 KiB | CLEAN (all 5 hashes) |
| F7 | F7 only | 67.57 s | 74,984 KiB | CLEAN |
| F2 (+F6 partial, see note) | F7+F2+3 early F6 edits | **51.57 s** | 80,036 KiB | CLEAN |

**Gate-provenance note (process deviation, recorded honestly):** the third
gate binary was built while three early F6 edits (erosion enabled-check
hoist; ET + percolation trace-event construction moved behind their config
checks) were already in the tree, because the edits were made while the gate
pipeline was still running. The gate therefore covers F7+F2+those three F6
edits combined, not F2 in isolation. Since the result is CLEAN, every change
in the combination is identity-safe; isolation would only have mattered for
bisecting a failure. Standing rule adopted for the rest of the package: no
source edits while a gate pipeline is in flight.

**Open item at this row:** one workspace test failure surfaced by the F2
pipeline's `cargo nextest` step (fail-fast; 1157/1284 not run). Not in the
orchestrator lib tests (145/145 pass). A `--no-fail-fast` run is identifying
it; disposition (my-change vs pre-existing-on-main) to be recorded here
before the F2 commit.
