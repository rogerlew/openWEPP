# Worker Handoff

Status: completed/HOLD
Evidence mode: static

Static:

Recommended HPHYS0271 scope:

- Diagnose the H1 sim-day 36 spurious melt event as the first high-value snowpack seam. Claude Code review bisected H1 and found days 1-35 track baseline, then candidate releases `~27-28 mm` redistributed melt from `~54 mm` raw melt while baseline has `RM=0` and continues accumulating snow.
- Add term-level `melt.for` trace evidence for H1 sim-day 36: `amelt`, `bmelt`, `cmelt`, `dmelt`, raw `hrmelt`, bounded/redistributed melt, and warm-branch entry conditions.
- Add hourly forcing trace evidence for H1 sim-day 36: hourly air temperature in the units consumed by `melt.for`, adjusted/surface temperature if used, radiation, dewpoint/vapor-pressure inputs, wind, canopy cover, albedo/ground-cover drivers, rainfall/snowfall, and the `winter.for`/`snowd.for` branch flags.
- Use baseline `winter.for`, `snowd.for`, `melt.for`, `stmtim.for`, `hr_tmp.for`, and `radcur.for` to prove whether the spurious melt is caused by a mis-scaled melt term, wrong hourly temperature/radiation/dewpoint/wind forcing, or warm-branch activation mismatch.
- Preserve corrected `/workdir/wepp-forest@03fee4558456535138592630b5dedc4d81ce8d06` negative-melt authority; do not reproduce the pinned baseline bug.
- Do not spend further effort on negative-melt redistribution for this cohort until a mixed thaw/refreeze day is proven material; H1 day 36 is a strongly positive raw-melt defect.
- Do not reopen WB17 `Ep`, aggregate storage, or WB13 publication compensation until melt-trigger/forcing lineage is proven closed or assigned elsewhere by trace evidence.

Key evidence from HPHYS0270:

- H1 day 99 candidate starts with `2.275 mm` SWE vs baseline `144.34 mm` Snow-Water and ends with `2.769 mm` SWE.
- H7 day 99 candidate starts with `2.275 mm` SWE vs baseline `159.59 mm` Snow-Water and ends with `2.769 mm` SWE.
- H39 day 115 candidate starts with `3.296 mm` SWE vs baseline `141.23 mm` Snow-Water and ends with `0.962 mm` SWE.
- Same-day HPHYS0270 closure errors are `0.0`, so target-day arithmetic is not the first defect.
- Claude Code reviewer bisection found H1 sim-days 1-35 are not the defect; sim-day 36 is the first discrete melt-release divergence.

Ran:

- Not applicable.
