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

## F5 and exit gates (final state)

| Gate | Tree state | Wall (indicative) | Max RSS | Identity |
|---|---|---:|---:|---|
| F5 | F7+F2+F6p+F5 (pre-fmt) | **45.85 s** (load ~3) | 81,120 KiB | CLEAN |
| Exit identity | final commit `2398ed44` (post-fmt rebuild) | — | — | CLEAN |

Exit evidence (Ran, 2026-07-01):

- **Full workspace suite: 1284/1284 passed**, 1 skipped (worktree `.venv`
  restored per finding-dispositions.md).
- `cargo fmt --check`, `cargo clippy --workspace --all-targets`
  (0 warnings), `cargo deny check` all clean.
- **3-rep timing, loaded window** (load average 26.2 — watershed packages
  active): 50.19 / 50.99 / 50.34 s; legacy re-anchor in the same window
  10.52 s → **ratio 4.80×** with both sides under identical load,
  consistent with the 45.85 s quiet single-rep (4.75× vs the 9.65 s quiet
  legacy anchor). A quiet-window 3-rep remains the recommended
  post-merge confirmation, but both load conditions independently place the
  endpoint **under the 5× bar**.
- Cumulative: package entry 70.05 s → exit ~45.9–50.5 s depending on load;
  ~30% wall-time reduction, all five protected outputs byte-identical
  throughout.

## Exit re-profile (loaded window; perf-wp1exit.data)

Top self-time: frost hourly machinery now dominates cleanly —
`compute_active_frost_hourly_state` 11.7%, `derived_frost_depths` 7.3%
(+ `round` 5.1%, its leaf), `compute_direct_winter_frost_partition` 6.5%,
thaw feedback 2.7%. **The F2 target is gone from the profile**: no
`format_inner`/`write_str`/`record_constructed_boundary_symbol` above 1.2%;
allocator self-time fell ~16% → ~5%. Remaining structure confirms the WP-2
premise (two frost solves per winter day) and promotes F4's
`derived_frost_depths`+`round` (12.4% combined) to the top remaining
mechanical-adjacent item — contract-gated, staying in WP-2's contingency
stage. `build_simimpl28_hourly_winter_forcing_typed` no longer appears above
the 1.2% cut: the F3-narrowed variant is now formally **dropped** (not just
deferred) as below churn threshold; likewise the span-report projection-drop
refactor stays deferred (memmove 5.0% total, only part of it that pattern).
