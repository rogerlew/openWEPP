# Disposition — `HOLD-PYSNOBAL-CSS-WY2017-SNOBAL-CORE-FAILURE`

Status: dispositioned. The hold is resolved as a **known upstream PySnobal/SNOBAL
numerical instability on thin/transitional snowpacks**, not an openWEPP defect and
not an export/forcing artifact. PySnobal remains diagnostic flag evidence only
(ADR-0017); this disposition does not change openWEPP physics, the density fork
routing, or the rubric verdicts.

Evidence mode:

- **Ran** — the crash and the bounded-forcing ranges are H run evidence
  (`pysnobal-hold.md`, `target/snowfrost_fidelity_h/pysnobal_snotel_summary.*`).
- **Static** — the root-cause and the "known instability" conclusion are from a
  2026-06-25 web/source review of the maintainer GitHub issues, the `libsnobal`
  C source, and the peer-reviewed literature (cited below), performed by a
  read-only research subagent and relayed here. Primary sources are linked; one
  string-level item is marked UNVERIFIED.

## 1. What H observed

H reran the three-way comparison with two improvements over G0/G1 — SNOTEL `STO`
observed soil temperature as the PySnobal ground-temperature forcing, and
per-water-year segmentation (so one synthetic snowpack state does not span
decades). **Four of the five SNOTEL sites produced finite, passing PySnobal SWE
and depth summaries.** CSS Lab aborted inside the PySnobal C core for water year
2017:

```text
[pysnobal/c_snobal/libsnobal/sati.c:17] ERROR: Input temperature (tk): -1811.981217 is less than zero
```

The CSS WY2017 exported forcing was finite and physically bounded — air temp
`-14.97..29.97 degC`, ground temp `0.0..20.56 degC`, precip `0..18.23 mm`, snow
fraction `0..1`, downwelling thermal `181.39..464.93 W m^-2` — and `pysnobal.py`
converts `T_a`/`T_g`/`T_pp` from degC to Kelvin before the C core. So the hugely
negative absolute temperature is produced *internally* by the solver, not passed
in. The earlier G0/G1 Morris `Tg=-0.5 degC` crash (`tk = -153.45`) is the same
failure class.

## 2. Finding — this is a documented SNOBAL instability

The symptom, regime, and "only some site-years" pattern match a closed maintainer
issue exactly:

- **`USDA-ARS-NWRC/pysnobal #3` "Early season crashes"** (CLOSED): *"PySnobal will
  crash with **tk<0** on some early season (November, December) timesteps that
  iSnobal makes it through ... an artifact of PySnobal holding the snowpack
  variables in memory rather than initializing them every timestep like iSnobal
  does."* This is our `tk<0` / `sati.c` abort, our thin/transitional regime, and
  a named PySnobal-vs-iSnobal state-persistence cause.

## 3. Mechanism (from `libsnobal` source)

The snow/active-layer temperature is back-solved from cold content with **no
clamp** (`pysnobal/c_snobal/libsnobal/new_tsno.c`):

```c
tdif = ccon / (spm * cp);   /* ccon = cold content J/m^2; spm = layer specific mass kg/m^2 */
tsno = tdif + FREEZE;       /* no min()/max()/sanity check on tsno */
```

As the active-layer specific mass `spm` becomes small (thin or near-exhausted
pack), a fixed cold content divided by near-zero mass sends `tsno` arbitrarily
negative; `-1811 K` and `-153 K` are division-driven blow-ups, far outside any
input range. That bad temperature propagates through `hle1()` (turbulent-flux
iteration, no in-loop guard) into `sati()` (saturation vapor pressure over ice),
which enforces `tk > 0` and aborts. The only defense is **upstream** — a
`1.0 kg m^-2` mass floor (`SMALL_THRESHOLD`, `_calc_layers.c`) that zeroes the pack
to bare ground; `MIN_SNOW_TEMP = -75 degC` is a no-snow sentinel, not a live
clamp on `new_tsno`'s output. A thin-but-above-floor active layer (~`1-10 kg m^-2`,
running at the 1-minute SMALL step) hit with a large cold-content step is exactly
the unguarded regime.

The adaptive timestep (`do_data_tstep` -> `_divide_tstep`, NORMAL/MEDIUM/SMALL =
60/15/1 min) is the *stabilizer*, not the diverging part: the recursion is bounded
at the 1-minute step. The abort means the forcing pushed the layer past what
1-minute substepping could stabilize.

## 4. Why it is not openWEPP and not a forcing artifact

- The negative Kelvin is generated inside `new_tsno`/`hle1`, not at the `sati`
  boundary; inputs were finite and within SMRF clip ranges.
- It reproduces only in the thin/transitional regime, consistent with the
  `cold_content/mass` singularity, not with any specific openWEPP operand.
- The peer-reviewed literature attributes the same shallow-snowpack
  pack-temperature instability to the Marks/SNOBAL model lineage: **Lute,
  Abatzoglou & Link (2022)**, *SnowClim v1.0*, GMD 15:5045-5071, §2.2.7, citing
  **Marks et al. (1999)**.
- The maintainers acknowledge thin-snow crashes operationally (`awsm #2`; AWSM
  `depth_thresh` strip-and-retry).

## 5. Disposition decision

1. **Record CSS WY2017 as a known upstream SNOBAL thin-snow instability**, with
   no openWEPP fidelity implication.
2. **PySnobal valid-regime rule:** PySnobal is least reliable precisely in the
   thin / early-season / transitional regime where it can diverge — which is also
   where the WEPP snow model is most uncertain and the comparison noisiest. Cells
   where PySnobal aborts or runs on a sub-threshold pack are marked
   **`PySnobal-unavailable`** in the v74 rubric profile (`INV-SNOWFREEZE-050`),
   never scored, and never read as an openWEPP failure. PySnobal contributes
   profile cells only in its stable, established-snowpack regime.
3. **Do not rabbit-hole into the snobal C core** for this program. PySnobal is an
   ADR-0017 flag/hypothesis, not a correctness authority, and is 4/5 usable; the
   instability is upstream and out of openWEPP scope.

## 6. Mitigations (only if broader PySnobal coverage is later wanted)

In maintainer-blessed priority order:

1. **Strip thin snow below a depth/SWE threshold and retry the timestep** — the
   maintainers' own approach (`awsm #2`; AWSM `restart_crash` + `depth_thresh =
   0.05 m`).
2. **Lute et al. (2022) clamp:** when `SWE < 15 mm x timestep_hours`, set
   `T_pack = min(T_air, 0 degC)` and recompute cold content.
3. **Clamp `new_tsno`'s output** (e.g. floor at `MIN_SNOW_TEMP`) — the direct fix
   at the blow-up site; the code structurally lacks one.
4. **Per-timestep snowpack state initialization** rather than carrying state in
   memory (the `pysnobal #3` named cause).

These belong in a separate, optional PySnobal-hardening package, not in openWEPP.

## 7. One follow-up check (cheap)

A recently merged sibling fix, **`iSnobal/pysnobal #10`**, corrected a bug where
snowpack **initial/restart state** temperatures were not converted degC->K (our
*forcing* conversion is confirmed, but the *state* initialization is separate).
Confirm the bridge initializes the snowpack state temperatures in Kelvin. Likely
not the CSS WY2017 cause (that is the thin-snow divide), but cheap to rule out.

## 8. References

- `USDA-ARS-NWRC/pysnobal #3` "Early season crashes" (CLOSED) — `tk<0` early-season
  crash, in-memory-state cause: https://github.com/USDA-ARS-NWRC/pysnobal/issues/3
- `pysnobal/c_snobal/libsnobal/new_tsno.c` — unclamped `cold_content/mass`
  surface-temperature solve: https://github.com/USDA-ARS-NWRC/pysnobal/blob/master/pysnobal/c_snobal/libsnobal/new_tsno.c
- `.../libsnobal/sati.c` (`tk>0` guard), `.../libsnobal/hle1.c` (turbulent-flux
  iteration), `.../libsnobal/_calc_layers.c` (`1 kg m^-2` mass floor),
  `.../h/snobal.h` (`MIN_SNOW_TEMP`, timestep thresholds).
- `.../libsnobal/_divide_tstep.c` — bounded NORMAL/MEDIUM/SMALL adaptive
  substepping: https://github.com/USDA-ARS-NWRC/pysnobal/blob/master/pysnobal/c_snobal/libsnobal/_divide_tstep.c
- `USDA-ARS-NWRC/awsm #2` "Early season crash prevention in iPySnobal" (OPEN):
  https://github.com/USDA-ARS-NWRC/awsm/issues/2
- AWSM `CoreConfig.ini` — `restart_crash` / `depth_thresh` ("can help with shallow
  snowpack that causes crashes"):
  https://github.com/USDA-ARS-NWRC/awsm/blob/master/awsm/framework/CoreConfig.ini
- `iSnobal/pysnobal #10` (CLOSED/merged) — initial-state degC->K conversion fix:
  https://github.com/iSnobal/pysnobal/pull/10
- `USDA-ARS-NWRC/pysnobal #10` (OPEN) — request for verbose C errors (esp. `hle1`):
  https://github.com/USDA-ARS-NWRC/pysnobal/issues/10
- Lute, A. C., Abatzoglou, J. T., & Link, T. E. (2022). SnowClim v1.0: a
  high-resolution snow modeling framework. *Geoscientific Model Development*,
  15, 5045-5071. https://doi.org/10.5194/gmd-15-5045-2022 (§2.2.7).
- Marks, D., Domingo, J., Susong, D., Link, T., & Garen, D. (1999). A
  spatially distributed energy balance snowmelt model (SNOBAL). *Hydrological
  Processes*, 13, 1935-1959. https://doi.org/10.1002/(SICI)1099-1085(199909)13:12/13<1935::AID-HYP868>3.0.CO;2-C

## UNVERIFIED

- Our exact verbose error string (`"[pysnobal/c_snobal/libsnobal/sati.c:17]
  ERROR: Input temperature (tk): -1811.981217 is less than zero"`) does not appear
  verbatim in public `master` (live string is terser, e.g. `"tk=%f, less than
  zero"`), so our build is a patched/forked variant. The file, the `tk<=0`
  semantics, and the failure regime all match, so it is the same guard with a more
  verbose message.
- Whether `pysnobal #3`'s in-memory-state fix was actually landed (the issue was
  closed without a documented fix in its body).
