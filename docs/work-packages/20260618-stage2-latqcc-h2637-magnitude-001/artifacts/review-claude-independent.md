# STAGE2-LATQCC-H2637-MAGNITUDE — Independent Review (Claude Code)

Verdict: **`CONTRACT-GAP` is earned, well-evidenced, and appropriately humble — a model Stage-2
magnitude adjudication.** The equation check is real arithmetic, the operand checks interrogate
the prime leads (not hand-waves), the legacy comparison carries the dimensional-consistency proof
I required, and — the load-bearing honesty — it **refuses to overclaim `CORRECT`** and does **not**
close the FARPOINT01 flag. No defect; no fix; clean handoff.

Evidence mode: **Static** (all seven artifacts + the residual/operand tables).

## Why the verdict is trustworthy

The risk with `CONTRACT-GAP` is that it's the comfortable answer (no defect, no commitment, no
work). It isn't here:

- **Equation correctness is real recomputation, not assertion.** The Darcy lateral flux was
  recomputed per OFE-day **per substep** (`Ke = (86400/substeps)·Σk_depth/Σsat_depth`;
  `q_potential = fcdep·anisotropy·Ke·sin(atan(slope))/slplen`; `q_target = min(potential, pool,
  capacity)`) and emitted `latqcc` matched at **machine precision** (max residual 4.16e-17 m;
  WAT `latqcc − q·1000` = 0.0 mm). The peak row (OFE7, day 5507, 71.6 mm) verified term-by-term.
  `INV-SUBHYD-003/012` not violated. ✓
- **Operand checks hit the prime leads.** Conductivity: lateral `ssh` **==** percolation `ssc`
  in every traced row (no lateral override/inflation), anisotropy 1.0 — so no lateral-path
  conductivity defect. Drainable thickness: layer-bounded, the high-`latqcc` rows are
  **potential-limited** (not a storage-cap failure). `drfc`: equals the contract formula
  `fc+(1-coca)·dg`, withdrawals sum to `q` at 1e-17, and — good discipline — the **withdrawn
  "FC 2× too low" suspicion was re-checked fresh and not reproduced**, exactly as scoped. ✓
- **Legacy stays a flag, with the dimensional proof.** The comparison is volume-to-volume after
  area scaling (`latqcc_m³ = latqcc_mm/1000·OFE_area`), explicitly "not a depth to a volume" —
  the comparator-surface-artifact hazard I flagged is closed. Verdict `UNRESOLVED` flag, "legacy
  does not prove openWEPP wrong." ✓
- **The honesty that matters:** it distinguishes *no defect found* (equation-correct + operands
  bound-valid) from *CORRECT* (absolute magnitude affirmatively validated) and declines the
  latter, leaving the FARPOINT01 71% flag **open as an absolute-magnitude authority gap** rather
  than declaring victory. That is the right call. ✓

## Sharpening for the follow-on (not a gap in this package)

The gap is correctly identified, but its **root** is worth naming for the recommended
absolute-magnitude `SC-SUBHYD-001` suite. The traced lateral conductivity runs up to ~`9.2e-5
m/s` (~7.9 m/day) — high, and (for forest) sourced through the **provisional `ksatadj`
sat-fraction conductivity model** (not standard WEPP, no physical "why" on record). This package
correctly showed the lateral path doesn't *add* inflation (`ssh==ssc`) and the equation/operands
are internally coherent — but the absolute magnitude ultimately rides on whether that
forest conductivity value is physically right, which existing authority can't say. So the
follow-on suite should **specifically pin the forest `ksatadj` equivalent-conductivity** (the
actual magnitude driver) and the `solwpv=9002` 24-substep lineage — not just a generic lateral
benchmark. The handoff gestures at "soil-file conductivity lineage"; I'd make the `ksatadj` model
the explicit target.

## The meta-point for the operator

This is the **second consecutive package** (MAGPARITY01 → STAGE2-LATQCC) to drive the H2637
runoff/lateral question down to "correct-by-construction, absolute magnitude unvalidatable from
existing authority." The structural, conservation, transfer, area, export, **equation, and
operand** questions are now **all closed**. What remains is a pure **external-authority gap** —
closing it needs *new* authority (a site/class lateral-flow benchmark, field data, or a validated
forest-conductivity model), which is a different kind of work (sourcing/authoring authority) than
internal adjudication. No further internal tracing will move it.

## Disposition

Land the record (docs-only — no production code touched; diagnostic build was in `/tmp`). The
FARPOINT01 flag stays correctly open as an absolute-magnitude gap. The operator's real choice now
is **author the absolute-magnitude `SC-SUBHYD-001` suite** (pinning the `ksatadj` forest
conductivity) **vs. accept-and-document the gap** and move to other science — both are legitimate;
no defect compels the former.
