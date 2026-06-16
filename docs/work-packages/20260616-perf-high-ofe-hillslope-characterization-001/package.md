# PERFHO01 — High-OFE Hillslope Performance Characterization

Status: queued (FARPOINT01 follow-on; operator-directed 2026-06-16, scaffolded by
Claude Code for Codex execution)

Package type: **Characterization** (validation/characterization shape — **not** a
Defect-Closure ExecPlan; this package lands no production or contract change).

## Objective

Attribute openWEPP's large single-hillslope wall-clock cost at high OFE count and
emit a verdict + recommendation. On the FARPOINT01 H2637 substrate (19 OFEs,
34 sim-years, daily) `openwepp-cli-hill` (release) takes **~1000 s** per run while
the legacy `wepp_260606_hill` Fortran does the same run in **~9–12 s** — a
**~80–110×** gap. Legacy proves ~10 s is achievable for this problem, so the gap
is an openWEPP **implementation** cost, not inherent to the physics or output.

Deliverable is a **characterization verdict** (where the time goes, how it scales
with OFE count, and whether/where optimization is warranted) — **not** a fix. No
code, no contract amendment.

This matters architecturally: openWEPP is subprocess-per-hillslope (ADR-0004); a
~17 min/hillslope cost sets the floor for watershed throughput even under
parallel orchestration.

## Why this substrate

H2637 (19 OFEs) is the FARPOINT01 fixture with in-repo provenance and a measured
~80–110× gap; the arboreal-dendrite 1–5-OFE ladder (MOFE01) gives a low-OFE
anchor for the scaling curve. Both run the same per-OFE WB13 publication path.

## What "characterization" means here (boundaries)

- **No production edit.** Profile and attribute; do not optimize in this package.
- **No contract amendment.** No `SC-*` change.
- **Determinism is sacrosanct for any *future* optimization** (out of this
  package's scope but state it in the recommendation): per `docs/numerics/`,
  any optimization follow-on must preserve bit-identical outputs and the
  conservation-gate behavior — no reordering that perturbs floating-point sums.
- **Legacy timing is a reference for the gap, not a target.** This is a wall-clock
  reference (legacy achieves ~10 s); it is not a physics comparator claim.

## Evidence (from FARPOINT01, Ran)

- `openwepp-cli-hill` (release, HEAD `41469058`), H2637 19-OFE × 34-yr:
  without_ui `1016 s`, with_ui `1035 s`, two further runs `944 s` / `936 s`
  (median ~`1000 s`).
- Legacy `wepp_260606_hill` same run: `9 s` (without_ui) / `12 s` (with_ui).
- The pre-fix run reached `sim_day 3324` (~27 % of the 34-yr sim) in `264 s`,
  consistent with a ~1000 s full run (roughly linear in sim-days).
- openWEPP outputs: `H2637.wat.parquet` = 235,961 rows × 34 cols (17.4 MB);
  235,961 = 19 OFE × ~12,419 days.
- Substrate fixture (working): `/tmp/openwepp_farpoint01_h2637/{with_ui,without_ui}/runs/`
  (legacy inputs + authored `h2637.run` TOML). Durable provenance + how-to-stage:
  FARPOINT01 `artifacts/fixture-and-baseline-evidence.md`.

## Candidate hot paths — LEADS to profile, NOT conclusions

Attribute by profiling; do **not** assume these are the cost. They are starting
points (with file pointers) for a profiler-driven attribution:

- **Per-OFE-per-day string building** in
  `crates/openwepp-runner/src/hillslope/scheduler_trace/per_ofe_internal_wb13.rs`:
  `storage_reconciliation_detail: String` is built and stored per record;
  `PerElementWaterBalanceTerms::describe` and the hillslope-total
  `detail`/`detail_parts.join("; ")` build large strings; `format_wb12_storage_terms`
  builds a ~22-field string per OFE per day. At ~236 k rows this is ~236 k+
  string allocations — a prime suspect for eager work that is only needed on the
  rare failure path.
- **Daily identity scans** (`scan_internal_identity_terms`,
  `hillslope_total_identity_residual_mm`) run every day for every OFE.
- **Parquet row accumulation** (235,961 rows × 34 cols held then written).
- **Per-OFE lane execution / scheduler** — sequential 19 lanes/day; determine
  whether per-day cost is **linear** in OFE count or **superlinear**.

## Tasks

1. Establish a fast, faithful repro for iteration (e.g. reduced sim-years and/or
   the arboreal-dendrite low-OFE runs) — note any fidelity caveat of the reduced
   repro.
2. Profile a representative openWEPP `cli-hill` run (profiler of Codex's choice —
   `perf`/`cargo flamegraph`/`callgrind`/targeted instrumentation). Attribute
   wall-clock to named functions/modules with a % breakdown.
3. Measure the **OFE-count scaling curve**: wall-clock vs OFE count across the
   arboreal-dendrite ladder (1–5 OFEs) and H2637 (19 OFEs), normalized per
   sim-day, to extract the scaling exponent (linear vs superlinear).
4. Emit the verdict + recommendation.

## Acceptance criteria

- A profiler-backed attribution: ≥ the dominant share of wall-clock assigned to
  named hot paths, with evidence (flamegraph/perf report/instrumented timings).
- The OFE-count scaling exponent, with the low-OFE-to-19-OFE measurements.
- A verdict classifying the cost: (a) acceptable for the subprocess-per-hillslope
  architecture as-is, or (b) a **named optimization follow-on** specifying the hot
  path, the expected gain, and the **bit-identity / determinism constraint** any
  fix must satisfy. No "make it faster" hand-wave — name the path and the bound.

## Deliverables

- `artifacts/perf-profile-evidence.md` — profiler output + % breakdown.
- `artifacts/perf-scaling-curve.md` — wall-clock vs OFE count.
- `artifacts/perfho01-verdict.md` — the verdict + recommendation (+ any
  defect-shaped or refactor follow-on, per ADR-0018 if it implicates a specific
  envelope).

## Dependencies

- FARPOINT01 (`20260613-mofe-farpoint01-high-ofe-routing-closure-demonstration-001/`)
  — substrate provenance, timings, the per-OFE WB13 path.
- `docs/numerics/README.md` — determinism policy (constrains any future fix).
- `AGENTS.md`, `docs/codex_exec_plans.md`, ADR-0004 (subprocess-per-hillslope).
- `openwepp-cli-hill` release binary; the H2637 fixture.

## Autonomy

Execute the characterization end-to-end (repro → profile → scaling curve →
verdict) without asking for direction on intermediate steps. This package **must
not** land a production or contract change — if optimization is warranted, that is
a follow-on package's scope. Ask only if a profiler cannot be run in the
environment.
