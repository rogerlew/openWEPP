# SNOW-HOURLY-ERA5-CLOUD-PROXY-SANITY

Status: `complete / sanity association passes / no promotion`

Date: `2026-08-03`

Plan class: `External-data diagnostic / proxy sanity check`

## Purpose

Compare ERA5 total cloud cover with the retained SIMIMPL daily cloud proxy at
Mica, Paradise, Snowbird, and Niwot. Test whether cloud-proxy differences are
consistent with the already verified horizontal shortwave differences without
changing forcing, calibration, or snow physics.

## Authority And Scope

- ERA5 `tcc` is the only acquired independent cloud-cover field. ERA5-Land has
  no independent land-generated cloud state and is excluded from cloud scoring.
- The retained proxy is reconstructed exactly from the checksum-bound
  SIMIMPL diagnostic longwave and hourly temperature export using its published
  emissivity operator. It is a proxy, not an observation.
- The predecessor radiation package and its result receipt are read-only input
  authority.
- Write set: this package tree plus `docs/ROADMAP.md`,
  `docs/planning/snow-surface-energy-balance-roadmap.md`, and
  `docs/work-packages/README.md`.
- Production, tests, fixtures, observations, precipitation, multipliers,
  selectors, and defaults are read-only.

## Frozen Operators

1. Reconstruct SIMIMPL cloud exactly as
   `(LW / (sigma * T_kelvin^4) - 0.72) / 0.28`; require `[0,1]`, daily
   constancy, and exact source hashes.
2. Treat ERA5 `tcc` as instantaneous at `valid_time`; convert timestamps to
   fixed local standard time without the radiation interval-start shift.
3. Require complete 24-hour local days. Compare the SIMIMPL proxy with:
   - primary ERA5 24-hour arithmetic-mean cloud, independent of realized
     shortwave; and
   - an explicitly outcome-dependent sensitivity using same-record realized
     `ssrd` weights. This sensitivity is not independent sanity evidence.
4. Report correlation, ERA-minus-proxy signed error, MAE, mean values, and
   clear/mixed/overcast category agreement for all complete days and wet
   November-March days selected from unchanged retained precipitation.
5. Relate daily cloud residual to the horizontal shortwave-energy residual.
   The primary association uses 24-hour mean cloud. The realized-`ssrd`-
   weighted association is mathematically coupled and reported only as a
   sensitivity, never causal or independent evidence.
6. Keep sites separate. No pooled score may conceal a site failure.

## Acceptance And Claims

- Emit a checksum-bound machine result and readable scientific interpretation.
- Distinguish proxy sanity from cloud observation validation.
- Do not infer ERA5-Land cloud, provider admission, snow improvement, or a
  cloud correction.
- Run direct syntax/JSON/Markdown/diff/protected-path checks, dual independent
  review with finding disposition, and dual exact-current verification.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only science/data-governance reviewers and two read-only
terminal verifiers. Expected outputs are package-local review/verification
evidence; write access is read-only.

## Progress

- [x] (2026-08-03) User authorized separate cloud-proxy sanity analysis.
- [x] (2026-08-03) Frozen scope, operators, authority limits, and write set.
- [x] (2026-08-03) Executed four-site comparison and published bound results and
  interpretation.
- [x] (2026-08-03) Remediated initial review findings and completed two fresh
  exact-current reviews with `PASS`.
- [x] (2026-08-03) Completed dual terminal verification after correcting one
  lifecycle-only HOLD; closed with no remaining finding and no promotion.
