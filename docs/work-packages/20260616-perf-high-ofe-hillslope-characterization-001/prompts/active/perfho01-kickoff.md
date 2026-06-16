# PERFHO01 Kickoff — High-OFE Hillslope Performance Characterization

Execution mode: package-end-to-end (characterization)

Autonomy: execute the characterization end-to-end (fast repro → profile →
OFE-count scaling curve → verdict) without asking for direction on intermediate
steps. Ask only if a profiler cannot run in this environment.

## This is characterization, not a fix

Profile and attribute openWEPP's ~80–110× single-hillslope wall-clock gap vs
legacy on the H2637 19-OFE substrate, and emit a verdict + recommendation. **No
production code edits. No `SC-*` contract edits.** If optimization is warranted,
name it as a follow-on — do not implement it here.

## The gap (measured, FARPOINT01)

`openwepp-cli-hill` (release) runs H2637 (19 OFEs × 34 sim-yr, daily) in **~1000 s**;
legacy `wepp_260606_hill` runs the same in **~10 s**. Legacy proves ~10 s is
achievable, so the gap is an openWEPP implementation cost to attribute — not
inherent to the physics or the 235,961-row output.

## Substrate

- Fixture (working): `/tmp/openwepp_farpoint01_h2637/{with_ui,without_ui}/runs/`
  (legacy inputs + `h2637.run` TOML). If absent, re-stage per FARPOINT01
  `artifacts/fixture-and-baseline-evidence.md` (inputs versioned under
  `/workdir/wepp-forest/docs/work-packages/20260503-wb05a-h2637-ofe19-hourly-qcap-resolution/artifacts/replays/`).
- Invocation: `openwepp-cli-hill --run-dir <runs> --run-file h2637.run
  --output-dir <out> --policy compat --legacy-sidecar-discovery`.
- Low-OFE anchor for the scaling curve: the MOFE01 arboreal-dendrite 1–5-OFE
  runs (`/wc1/runs/ar/arboreal-dendrite/wepp/runs/p*.{slp,sol,man,cli}` if
  present; else any available 1–5-OFE hillslope).

## Steps

1. Build a fast, faithful repro for iteration (reduced sim-years, and/or the
   low-OFE runs). Note the fidelity caveat of any reduction.
2. Profile a representative run (profiler of your choice). Attribute wall-clock to
   named functions/modules with a % breakdown. Treat the package's "candidate hot
   paths" as **leads to test, not conclusions** — especially the per-OFE-per-day
   `String` building in `per_ofe_internal_wb13.rs` (eager `storage_reconciliation_detail`,
   `describe`, `format_wb12_storage_terms`, hillslope-total `detail_parts.join`),
   which may be doing failure-path-only work on every row.
3. Measure the OFE-count scaling curve (wall-clock per sim-day vs OFE count:
   1–5 OFEs → 19 OFEs) to extract linear-vs-superlinear scaling.
4. Verdict (`artifacts/perfho01-verdict.md`): cost attribution + scaling exponent
   + recommendation — either "acceptable as-is" or a **named** optimization
   follow-on (the specific hot path, expected gain, and the bit-identity /
   determinism bound any fix must hold).

## Hard constraints

- No production / contract / production-test change — characterization +
  analysis artifacts only.
- Any *recommended* optimization must preserve bit-identical outputs and the
  conservation-gate behavior (`docs/numerics/`); say so in the verdict. No
  floating-point-perturbing reordering.
- Truthfulness: label evidence `Static:` vs `Ran:`; the attribution must be
  profiler-backed (`Ran:`), not asserted.

## Required reading

- `docs/work-packages/20260616-perf-high-ofe-hillslope-characterization-001/package.md`
- `AGENTS.md`, `docs/codex_exec_plans.md`
- `docs/numerics/README.md` (determinism policy)
- `docs/decisions/0004-subprocess-hillslope-orchestration.md`
- FARPOINT01 `artifacts/fixture-and-baseline-evidence.md` (substrate + timings)
- `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`
  and the `cli-hill` run path (the candidate hot paths)
