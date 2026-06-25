# Kickoff — SNOWFROST-FIDELITY-H (SNOTEL density three-way)

Execute `package.md` in this directory. Primary gap `GAP-SNOWFREEZE-002`.

## What and why
Acquire NRCS SNOTEL SWE + snow depth (+ soil temp) for the five
`tests/fixtures/snotel_observed/` sites, compute **observed density = SWE/depth**,
and run each site **three ways — openWEPP, pinned legacy WEPP, PySnobal — vs the
SNOTEL observations**. The goal is to resolve the open fork from A–G1: is
openWEPP's snow-depth over-prediction **over-accumulation** (SWE too high) or
**low density** (SWE ~ok, depth inflated)? SNOTEL's paired SWE+depth measures
density directly; the existing pilot sites couldn't.

## Hard guards (read before touching anything)
- **No production snow/frost physics change.** This is acquisition + comparison +
  characterization. Diagnostic harness/tooling and a contract-first density
  correspondence only.
- **Do NOT tune SSD to match the depth target.** Run two SSD arms (as-built
  `250`; climate-appropriate). The appropriate SSD is **derived from observed
  climatological density** (peak-SWE-period `SWE/depth`) as site characterization
  *before* the depth comparison — never chosen by minimizing the residual. See
  the SSD table + anti-tuning rule in `package.md`.
- **Phase-0 gate:** verify how WEPP's `snowd.for` consumes the `snow.txt`
  settling-density field before applying the appropriate-SSD arm. If it isn't the
  `densg` seed assumed in `package.md`, stop and report.
- **ADR-0017:** legacy WEPP and PySnobal are flags/hypotheses, not authority.
  Observed SNOTEL density/depth + `INV-SNOWFREEZE-048` + the new density
  correspondence are the authority. No `OPENWEPP-DEFECTIVE` on legacy/PySnobal
  agreement alone.

## Reuse what exists
- AWDB REST API is already used for the SCAN site — extend `observed_harness.py`
  for the `SNTL` network (inch→m/mm, °F→°C). Triplets + endpoints are in each
  fixture `manifest.md` and `tests/fixtures/snotel_observed/README.md`.
- Legacy capture lineage: reuse `legacy_snow_compare.py` / F's method (WAT
  `Snow-Water` SWE; daily-winter hour-24 depth via `.run` No→Yes replay).
- PySnobal: reuse `pysnobal_compare.py` (G0/G1). **Improvement:** feed SNOTEL
  `STO` (observed soil temp) as the ground-temp forcing where available
  (Paradise/CSS/Snowbird/Mica) — this should fix the G0/G1 constant-`Tg`
  `sati.c` crash and the lane sensitivity. Niwot has no `STO`; document a fallback.

## Done means
Three-way × five-site comparison of SWE, depth, and density vs observed; a
per-site fork verdict (`OVER-ACCUMULATION` / `LOW-DENSITY` / `STRUCTURAL`) with
the two-SSD-arm contrast as evidence; observed-density correspondence drafted
contract-first (provisional tolerance, hydrology-reviewer-pending); all gates
green; any physics remediation routed to a named follow-up package. Otherwise
close `HOLD-<specific>` with a worker handoff.
