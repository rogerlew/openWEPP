# Multilayer Surface-EB Sublimation + Longwave

## Status

- `state`: **breadcrumb (deferred — sequenced after frost; not being planned now).**
- `date`: 2026-06-29 (created, Claude Code)
- `relates`:
  [ADR-0029](../decisions/0029-commit-paradigm-2-multilayer-snow.md) (Paradigm 2
  multilayer), [ADR-0028](../decisions/0028-observed-data-admission-authority.md)
  (admission), Paradigm 2 **Stage 0** (`openwepp-meteorology` surface energy
  balance) and **Stage 3** (per-layer thermal/liquid solver) — the infrastructure
  this would build on; **SNOWDENSITY-10.3.16** (the Stage A bulk-sublimation
  non-promotion this supersedes); strategy §10.2 item 6 / paradigm2 spec §1.1
  (forcing-robust tiering); the
  [canopy snow interception/sublimation backlog](20260627-canopy-snow-interception-sublimation.md)
  (the *canopy* side of sublimation — distinct from this *surface-pack* side; together
  they are the full sublimation picture).

## The idea

Revisit sublimation now that the multilayer infrastructure exists. Sublimation and
longwave are **coupled terms in the same surface energy balance** — longwave (with
shortwave + sensible) sets the surface temperature that *governs* both sublimation
and melt, and the turbulent latent flux *is* the sublimation mass loss. So the
candidate is one coherent thing: a **per-layer surface-EB ablation term — the
surface latent flux → SWE mass loss from the surface layer, with longwave already in
the Stage 0 balance.** It builds on Stage 3's per-layer thermal solve + Stage 0's
surface energy balance (an incremental addition, not a new build), and is physically
grounded — unlike the crude bulk sink that failed at 10.3.16 Stage A.

## Why it is different from the 10.3.16 Stage A non-promotion

- **Physically grounded** (surface-EB latent flux on the surface layer), not an
  ad-hoc bulk sink — addresses the standing "our sublimation implementation might
  just not be good" hypothesis.
- **It removes mass (SWE) — a real aggregate change**, so it is NOT subject to the
  decomposition-robustness that sank Stage 1 (density) and Stage 2 (frost insulation).
- **The SNOTEL corpus has dry/windy continental sites (Niwot CO, Snowbird UT)** where
  sublimation is physically significant — a regime to validate against, unlike the
  humid New England sites where Stage A could show nothing.

## The caveat / realistic path

Sublimation primarily moves the **SWE/depth magnitude**, which at high-relief SNOTEL
sites is **forcing-limited** (undercatch/lapse) and therefore report-only under the
forcing-robust rubric. So a *correct* sublimation may still not improve the rubric's
verdict-bearing cells (density, depth-SWE slope, timing, ordering) — likely
**physically-real-but-not-rubric-promotable**, for a forcing reason. The
bidirectional guardrail (mass removal → under-persistence) still applies, mitigated
by per-layer + VPD/wind physical keying (sublimate where it is active, not
uniformly). **Realistic posture: an opt-in physical-fidelity capability (like the
water-temperature arm)** — gated on physical correctness + conservation + snow
no-regress + the dry-site signature where it *can* register — **not** a default
promotion contingent on a rubric win it cannot achieve for forcing reasons.

## Why it is worth it — the streamflow / ET-tuning case (stronger than fidelity)

Sublimation is a recognized, important snow process (10–40% of snowfall in some
environments) the bulk model lacks entirely. But the **strongest** argument is a
water-balance one (operator, 2026-06-29): **users currently calibrate by inflating
ET to reduce over-predicted runoff.** A sublimating snowpack removes water
(SWE → vapor) that would otherwise become melt → runoff — i.e. it does
**physically** what the ET tuning fakes. So physical sublimation could **improve
streamflow estimates and reduce the need for the ET calibration fudge** — physics
replacing a tuning knob.

This **shifts the validation target**: the payoff registers in **gauged streamflow /
the snow-season runoff bias** — the actual, *observed* calibration target — rather
than only the forcing-limited snow SWE magnitude. That substantially weakens the
"magnitude is forcing-limited so the rubric can't reward it" caveat above and moves
sublimation from a fidelity nicety toward a **defect-shaped streamflow improvement**.

**Cautions (so it does not become the next fudge):** parameterize sublimation from
**physics** (VPD/wind via the surface energy balance), **never fit it to
streamflow** — replacing the ET knob with a sublimation knob is the same fudge in
new clothes. And **diagnose first** that the over-predicted runoff is actually
*snowmelt-driven* (sublimation-addressable) rather than an ET-kernel or
infiltration/runoff-partition issue (cf. the H2637 runoff/lateral-magnitude arc); it
is snow-catchment-specific.

## Sequence

Sequenced after frost. The streamflow/ET-tuning relevance above could justify
**revisiting this sequence** (it is more defect-shaped than a fidelity nicety); left
after frost unless reprioritized. Breadcrumb only — not planned further here.
