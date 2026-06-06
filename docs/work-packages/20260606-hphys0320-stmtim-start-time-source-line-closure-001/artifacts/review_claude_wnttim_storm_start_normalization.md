# Claude Code Review — HPHYS0320 `wnttim` Storm-Start Normalization

Reviewer: Claude Code
Date (UTC): 2026-06-06
Evidence mode: **Static** — read the working-tree diff, the pinned-baseline
Fortran source, the `SC-CLIMATE-001` amendment, and the HPHYS0319 paired ledger;
reasoned from them. No commands were run in this review. The `39/39` release-
binary result cited below is **Codex's run** per `worker-handoff.md`, not a run
I performed or verified.

Verdict: **APPROVE.** The change is a faithful, load-bearing port of the
baseline storm-start timing lineage, fails closed correctly, and reproduces the
HPHYS0319 baseline key value exactly by arithmetic. One open item remains, and it
is documentation (contract rationale), not code — left for Codex disposition.

This is the first genuine production forcing correction of the HPHYS0298->0320
arc, and the diagnostic record (0319 -> 0320) shows it is a **climate-forcing
timing defect, not snow physics** — the snow surface was only where the symptom
surfaced.

---

## What landed (working tree, uncommitted)

`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`

- New helper `simimpl28_stmtim_start_time(wnttim)` (`:830-841`): returns a typed
  `NonFiniteField` error for non-finite `wnttim`; otherwise normalizes
  `wnttim < 1.0 -> 1.0`.
- Call site (`:744`): applied inside `simimpl28_stmtim_hourly_partition` after the
  `rain`/`stmdur` finite checks and **before** the `rain_m <= 0.0001` early
  return.

Contract authority: `SC-CLIMATE-001` v22 `INV-CLIMATE-018` (hard-fail),
`REF-CLIMATE-WF-WNTTIM-MIN`, `OBL-CLIMATE-P-013`; gated cross-contract by
`SC-SNOWFREEZE-001#INV-SNOWFREEZE-046` and `SC-WATBAL-001#INV-WATBAL-094`.

---

## Findings

### F1 — Faithful and complete port of the start-time lineage (not partial) `[DIRECT][Static]`

The baseline storm-start lineage has **three** normalizations, not one. All three
are present in openWEPP and in the same order as the legacy
`winter.for` -> `stmtim.for` flow:

| Step | Legacy | openWEPP |
|---|---|---|
| Floor below hour 1 | `winter.for:232-234` `if (wnttim .lt. 1.0) wnttim = 1.0` | `06_simimpl28_hourly_forcing.rs:744` + `:837` |
| End-of-day overflow | `stmtim.for:60` `if ((wnttim+wntdur) .gt. 24.0) wnttim = 24.0 - wntdur` | `:780-782` |
| Full-day duration reset | `stmtim.for` (CAS) `if (wntdur .eq. 24.0) wnttim = 1.0` | `:783-785` |

The full-day reset uses an epsilon comparison (`(wntdur - 24.0).abs() <= EPS`)
rather than legacy exact `.eq.` on a float — a correct improvement, not a
divergence. HPHYS0320 only *added* the first step; the review confirms the other
two were already implemented and the new floor sits correctly upstream of both.

### F2 — The floor is load-bearing, and the result reproduces the baseline key exactly `[DIRECT][Static] + [INFERENCE][Static]`

The active-interval predicate (`:794`) is
`(hour >= wnttim) && (hour < (wnttim + wntdur))` over a 1-based hour index,
matching `stmtim.for:62`. For the HPHYS0319 key (2013 day 11 hour 11), the paired
baseline event is `rain = 0.00082 m`, `stmdur ~= 38040 s -> wntdur = 11`:

- Without the floor: `wnttim = 0` -> window `[0, 11)` -> **excludes hour 11**
  -> inactive -> `snow_branch = 0` -> `hrsnow = 0`. (openWEPP pre-fix.)
- With the floor: `wnttim = 1` -> window `[1, 12)` -> **includes hour 11**
  -> active; `hrtemp <= rst` selects the snow branch (`:806-815`):
  `hrsnow = rain_m / wntdur * 10 = 0.00082 / 11 * 10 = 0.00074545 m`.

That equals the HPHYS0319 fixed-baseline `hrsnow = 0.00074545 m` for H1/H7/H39
exactly. The fix does not merely flip a flag — it reproduces the baseline value
by the documented equation. (The `* 10` is the fresh-snow density depth factor;
see F6.)

### F3 — Placement relative to the dry-day early return is faithful `[DIRECT][Static]`

Legacy floors `wnttim` in `winter.for` **unconditionally per day**, before
`stmtim.for` is reached and independent of precipitation. openWEPP applies the
floor at `:744`, *before* the `rain_m <= 0.0001` early return at `:745`, so the
normalized `wnttim` is published (`wnttim_h`) even on dry/trace days — matching
the legacy unconditional semantics. Because flooring is idempotent, applying it
per-hour (openWEPP calls the partition function per hour) yields the same result
as legacy's once-per-day application. No defect.

### F4 — Fail-closed posture matches the contract `[DIRECT][Static]`

Non-finite `wnttim` returns `ClimateRuntimeInputError::NonFiniteField` rather than
clamping or defaulting (`:831-835`). This honors `OBL-CLIMATE-P-013`'s explicit
statement that this is *"a contract-cited baseline compatibility rule, not a
generic clamp,"* and the typed-failure posture for non-finite winter forcing
context. No silent fallback path was introduced.

### F5 — OPEN ITEM (documentation): contract states the *what*, not the *why* `[INFERENCE][Static]`

Under ADR-0017, baseline agreement is an investigation flag, not a target — so a
production rule that *replicates a baseline behavior* is only acceptable if the
baseline behavior is itself sound, and that soundness must be on the record, not
implied by the match.

My reasoned assessment is that the floor **is** sound: it encodes the 1-based
storm-hour window convention. Both the legacy random start (`winter.for`:
`wnttim = aint(randn)`, range `{0..23}`) and the breakpoint start (`stmstr`) can
yield `wnttim = 0`, i.e. a storm normalized to start "before hour 1." Because the
hourly loop is indexed `1..24`, a start of `0` must be pulled to `1` to be
visible to the active-interval test at all. That is a derived indexing
convention, not an arbitrary clamp.

The contract (`REF-CLIMATE-WF-WNTTIM-MIN`, `OBL-CLIMATE-P-013`) currently frames
the rule as "baseline-authoritative" / "baseline compatibility" and stops short
of stating this 1-based-window rationale. As written, a future reader could read
it as baseline mimicry rather than a defensible convention.

Recommendation (disposition for Codex): add one clause to
`REF-CLIMATE-WF-WNTTIM-MIN` giving the 1-based hour-window rationale, so the rule
reads as a derived convention with independent justification rather than
"baseline does this." This is the only open item; it is contract-text, not code,
and does not block the code change.

### F6 — Cross-consistency note with the HPHYS0298/0299 resolution `[DIRECT][Static]`

The snow branch emits `hrsnow_m = rain_m / wntdur * 10.0` (`:809`) — a snow
**depth** (per-hour water share x fresh-snow density factor). This is the same
surface whose **depth-vs-water-equivalent** confusion produced the false
`OPENWEPP-DEFECTIVE` verdict at HPHYS0298 (corrected at HPHYS0299). The value
here is correctly a depth and matches the baseline `hrsnow` depth, so this fix
does **not** reintroduce the 0298 artifact. Flagging for continuity: any
downstream consumer or future comparator pairing of `hrsnow_m` must treat it as
depth (water-equivalent x 10), per the 0299 resolution and
`project-comparator-surface-artifacts`.

---

## Scope / disposition boundary

Per the openWEPP review model, this artifact surfaces findings and evidence and
leaves architecture and disposition to Codex. F1-F4 and F6 are confirmations and
continuity notes with no requested change. **F5 is the single open item** and is
a contract-text clarification, not a code change. I did not run the test suite;
acceptance against the contract's named behavior, and the `39/39` release-binary
result, are Codex's attributed evidence.
