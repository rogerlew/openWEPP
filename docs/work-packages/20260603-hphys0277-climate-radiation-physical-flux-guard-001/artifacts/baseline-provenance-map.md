# Baseline Provenance Map

Status: completed
Evidence mode: static

Static: baseline and contract provenance for the HPHYS0277 high hourly radiation
guard.

Ran: not-run; this artifact records source-level provenance only.

## Baseline Authority

- Baseline worktree: `/workdir/wepp-forest_260430_baseline`.
- Baseline commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Canonical contract: `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`.
- Primary source files inspected:
  - `/workdir/wepp-forest_260430_baseline/src/radcur.for`
  - `/workdir/wepp-forest_260430_baseline/src/hr_tmp.for`
  - `/workdir/wepp-forest_260430_baseline/src/winter.for`
  - `/workdir/wepp-forest_260430_baseline/src/sunmap.for`

## Bound Derivation

Static: `radcur.for` computes potential solar radiation from the solar constant
`0.082 MJ m^-2 min^-1` and Earth-sun distance factor
`rdsun = 1 + 0.033*cos(2*pi*sdate/365)`.

Static: HPHYS0277 uses the physically conservative one-hour normal-incidence
extraterrestrial bound:

`E0h_max(sdate) = ((12*60)/pi) * 0.082 * rdsun * 2*sin(pi/24)`

Static: Runtime hourly `winter.hourly.rad_mj_m2_####` / `hradmj` publication
must satisfy `0 <= hradmj <= E0h_max(sdate)` plus explicit roundoff tolerance.
The guard is intentionally conservative because it allows the maximum possible
one-hour extraterrestrial energy at normal incidence rather than a slope,
latitude, cloud, canopy, albedo, or daylight-window reduced value.

## Non-Authority Exclusions

- No fixed heuristic cutoff is used as final authority.
- No clipping, capping, renormalization, or substitution is allowed.
- No downstream snowmelt, ET, runoff, or storage compensation is introduced.
