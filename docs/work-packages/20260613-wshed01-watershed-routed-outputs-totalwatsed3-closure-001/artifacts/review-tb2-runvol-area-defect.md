> **RESOLVED 2026-06-14 (T-B2-REDO2, Claude-verified).** Fix landed: `:737`
> `qofe · outlet.area` → `Σ runvol = 27.691 Mm³` (coeff 0.554), independent of
> the WAT-`Q` column (18.895 Mm³), closure ex-day-1 `−0.41 mm/2191 d` with
> nonzero-at-noise daily residuals `[−0.248, +0.005] mm`. The arc
> over-scale → under-scale → correct is closed; this artifact is the forensic
> record. See the plan's T-B2-REDO2 ACCEPTED note.

# Review finding — T-B2 `runvol` over-scaled by wrong reference area (runoff > precip)

**Reviewer:** Claude Code · **Date:** 2026-06-14 · **Evidence class:** Ran (duckdb
audits on the native T-B2 totalwatsed3 output) + Static (read the producer +
M-I identity).

**Status:** T-B2 ships a `runvol` that fails a hard physical bound. Hold T-C;
disposition the area term first. **Anchors are unaffected** (output-surface
only) — this is not a physics regression, it is a publication-area mistake in
the new PASS surface.

## The defect (hard, not a tolerance question)

The native totalwatsed3 produced by T-B2
(`/tmp/openwepp_wshed01_tb2/totalwatsed3.parquet`, 2192 days, 36-hillslope
arboreal-dendrite cohort) reports **annual runoff volume 2.0–3.1× annual
precipitation volume**:

| year | precip (Mm³) | runvol (Mm³) | runvol/P | Q-col/P |
|---|---|---|---|---|
| 1 | 7.179 | 15.735 | **2.19** | 0.32 |
| 2 | 8.319 | 17.285 | **2.08** | 0.31 |
| 3 | 8.263 | 25.601 | **3.10** | 0.47 |
| 4 | 9.307 | 23.272 | **2.50** | 0.37 |
| 5 | 9.162 | 26.420 | **2.88** | 0.43 |
| 6 | 7.781 | 18.445 | **2.37** | 0.35 |

Runoff cannot exceed precipitation. 69% of precip-days have `runvol > P`. The
`P − (Runoff + Lateral + ET + Perc + Interception) − ΔStorage` closure is
**max |daily| = 171.9 mm, cumulative = −32,855 mm** (driven entirely by
runoff). The `Q`/P ratios (0.31–0.47) are a plausible runoff coefficient —
they point at the correct magnitude.

## Root cause — `qofe` depth × **whole-hillslope** area

`build_hillslope_pass_row_from_outlet_delivery`
(`crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs:728`):

```rust
row.runvol_m3 = outlet.physical_surface_outflow_mm * publication_area_m2 / 1_000.0;
row.sbrunv_m3 = outlet.row.wb13_row.latqcc       * outlet.area_m2       / 1_000.0;  // :729
```

- `physical_surface_outflow_mm` = `current_transfer_output.qofe * 1000`
  (`per_ofe_internal_wb13.rs:189`). Legacy `watbal.for:1099`:
  `QOFE = runoff·efflen/slplen` — a depth **normalized by the outlet OFE's
  slope length**, i.e. referenced to the **outlet OFE footprint**.
- `publication_area_m2` = `Σ_all_OFE (fwidth·slplen)` = the **whole hillslope
  area** (`02_output_and_climate_helpers.rs:733-761`,
  `derive_mofe04_publication_area_from_slope`).

Multiplying a depth referenced to the outlet OFE by the **whole** hillslope
area over-scales the exported volume by `totlen/slplen(outlet)`. For the 1–5 OFE
cohort that area-weighted blend is ~2.5× — matching the observed
`runvol/P ≈ 2.0–3.1`. Single-OFE hillslopes (outlet OFE *is* the hillslope) are
correct; the inflation is entirely the multi-OFE members.

The line right below it (`:729`, `sbrunv`) already uses the right reference,
`outlet.area_m2`. `runvol` is the only term using `publication_area_m2`.

## The code's own dual identifies the correction

`QOFE` (slplen-normalized) and `Q` (`watbal.for:1094`, totlen-normalized) are
duals of the same exported volume:

```
exported_volume = QOFE_outlet · A_outlet_OFE  ≡  Q_outlet · A_hillslope
                = runoff · efflen · width   (both)
```

Two in-tree formulations already compute this correctly — the fix is to match
either, **disposition is Codex's**:

- **M-I identity** (`per_ofe_internal_wb13.rs:576-581`): the outlet's
  `physical_q_mm` (= `physical_surface_outflow_mm` = `qofe·1000`) is weighted by
  `record.area_m2` (**outlet OFE area**) — and that identity closes at
  `3.31e-13`. So `runvol = qofe · outlet.area_m2` (i.e. swap
  `publication_area_m2 → outlet.area_m2` on `:728`, matching `:729`).
- **Non-MOFE path** (`02_output_and_climate_helpers.rs:713`):
  `runvol_m3 = wb13_row.q · area_m2` — uses the **totlen-normalized published
  `q`** with the hillslope area. The MOFE path could mirror this:
  `runvol = outlet.row.wb13_row.q · publication_area_m2`.

Both are arithmetically identical; pick one for consistency with the
surrounding code.

## Why T-B2's own verification didn't catch it (the lesson)

T-B2's reported check (plan `:283-285`) was *"PASS `runvol` vs outlet WAT
`QOFE * hillslope area / 1000`"* → `1.46e-11`. That compares `runvol` against
**the same wrong formula that produced it** — a self-consistency restatement,
not an independent bound. The `1.46e-11` and the `1.79e-07` PASS-vs-totalwatsed3
sum are both **internal-consistency** identities; neither bounds runoff against
precipitation. This is the recurring hollow-green pattern (M-E4 tautology,
M-F clone): a green identity that re-derives an operand from itself. The
precip-bounded closure — which the T-arc exists to establish — exposes it on
the first run. **The scope I wrote seeded this:** plan `:250` said *"runvol =
outlet routed surface runoff × hillslope area"*, which is the dimensional
mismatch (`qofe` is not referenced to the hillslope area). That line is
corrected.

## Residual caveat — do not assume the area fix alone closes T-C

Substituting the `Q` column for `runvol` in the closure drops it from
−32,855 mm to **+2,950 mm cumulative / 31.1 mm max-daily** — directionally
confirming the area term dominates, but a residual remains (and +2,950 mm is
the same magnitude as the earlier W-D residual). I have **not** proven the area
correction alone yields noise-floor closure. T-C must re-audit after the fix
and attribute any remaining residual independently (candidate: the totalwatsed3
`Q`-column aggregation, or a snow/RM-vs-P timing term) rather than declaring
closure on the area fix.

## Recommended disposition

1. Correct the MOFE `runvol` reference area in
   `build_hillslope_pass_row_from_outlet_delivery` (`:728`) per one of the
   in-tree duals above; single-OFE anchors must stay byte-identical (their
   area is unchanged), MOFE PASS `runvol` changes by design.
2. Replace the self-consistency PASS check with an **independent bound**: a red
   test asserting per-hillslope `Σ runvol ≤ Σ precip` (annual), and that for a
   multi-OFE fixture `runvol = qofe·A_outlet ≠ qofe·A_hillslope`.
3. Re-run the native totalwatsed3 closure (T-C) and attribute the remaining
   residual; do not close T-C on the area fix alone.

---

# Follow-up review — T-B2-REDO over-corrected (crossed pairing, now UNDER-scaled)

**Reviewer:** Claude Code · **Date:** 2026-06-14 · **Evidence class:** Ran
(duckdb on `/tmp/openwepp_wshed01_tb2_redo_qarea/`, reconstructing the M-I
export from per-hillslope WAT outlet rows; reproduced Codex's exact
`6948.564523` residual) + Static (read the REDO diff + the geometry at
`02_output_and_climate_helpers.rs:1126-1162`).

**Status:** T-B2-REDO (`377b6e80`) is **still defective** — it swapped one
mis-pairing for its mirror image. Runoff is now **~4× too small**. The
`Σ runvol ≤ Σ precip` bound passes (it is one-sided) and the new tests pass
(they encode the wrong formula), so the Rust loop went green on a worse result.
Add **T-B2-REDO2**.

## What REDO did, and why it is also wrong

REDO changed `:728` to (`:737`):

```rust
row.runvol_m3 = outlet.row.wb13_row.q * outlet.row.wb13_row.area / 1_000.0;
```

The two surfaces here are a **crossed pairing**:
- `wb13_row.q` = `routed_runoff · 1000 · efflen/cumulative_length` — the
  **totlen-normalized** depth (`:1142`).
- `wb13_row.area` = the per-OFE `lane_area` passed as `publication_area_m2`
  (`per_ofe_internal_wb13.rs:160` passes `*lane_area_m2`; `:1162` `area =
  publication_area_m2`) — the **outlet OFE area**, *not* the hillslope area.

`Q` (totlen) must pair with the **hillslope** area; `QOFE` (slplen) must pair
with the **outlet OFE** area. REDO paired `Q` (totlen) with the **outlet** area,
under-scaling by `totlen/slplen`. By removing the `publication_area_m2`
parameter from the function, REDO left only the outlet area in scope, then
reached for `q` instead of `qofe`.

## Empirical proof (WAT outlet rows, independent of the producer)

Reconstructing the outlet export from the per-hillslope `H*.wat.parquet` outlet
rows (`ofe_id = max`), independent of the PASS producer:

| quantity | value |
|---|---|
| `Σ QOFE_outlet · A_outlet` (correct) | **27.691 Mm³** |
| `Σ Q_outlet · A_outlet` (REDO) | 6.851 Mm³ |
| ratio (= OFE-count blend, totlen/slplen) | **4.04** |
| precip | 50.011 Mm³ |
| correct runoff coefficient | **0.554** |

For H1 (5 OFEs) the outlet row has `QOFE/Q = 3.165/0.633 = 5.0 = totlen/slplen`
exactly. Substituting the correct `QOFE_outlet · A_outlet` into the totalwatsed3
closure:

- **REDO runvol:** cumulative residual `+6948.56 mm` (reproduces Codex's number
  exactly — confirms the reconstruction).
- **Correct runvol:** cumulative `+30.54 mm`, **entirely day 1** (`+30.95 mm`,
  the storage-baseline `prepend` init); **excluding day 1: −0.41 mm over 2191
  days, only 1 day > 1 mm.**

So the correct runvol **closes T-C** — and the `+2,950 mm` "residual caveat"
above was an **artifact of the wrong (WAT-`Q`) runvol**, not a second defect.

## The fix (empirically verified) — `q` → `qofe`

`02_output_and_climate_helpers.rs:737`: use the **slplen-normalized** depth with
the outlet area —

```rust
row.runvol_m3 = outlet.row.wb13_row.qofe * outlet.row.wb13_row.area / 1_000.0;
```

`WAT.QOFE · WAT.Area` (= `wb13_row.qofe · wb13_row.area`) is what closes at
−0.41 mm above. Equivalent to the M-I export (`physical_surface_outflow_mm ·
outlet.area_m2`, which closed the per-hillslope identity at 3.31e-13) — Codex
should pick whichever reads cleanest; both are slplen-depth × outlet-area.
Disposition (which of the two QOFE surfaces) is the implementer's.

## The lesson — my recommended bound was one-sided

`Σ runvol ≤ Σ precip` catches **over**-scaling (T-B2) but is **blind to
under**-scaling (REDO: 0.137 < 1 passes). Tests that assert the producer's own
formula also pass either way. The only thing that caught the mirror error was
reconstructing the export from **independent operands** (WAT `QOFE × A_outlet`)
and running the **actual closure**. The acceptance gate must be **two-sided and
closure-anchored**: `runvol` must equal `QOFE_outlet · A_outlet` (≡
`Q_outlet · A_hillslope`) *and* the totalwatsed3 closure must drop to its floor
(ex-day-1 sub-mm) — not a one-sided inequality, and not a test that re-states
the implementation. This is the conservation-is-acceptance thesis catching an
error that every proxy gate missed.
