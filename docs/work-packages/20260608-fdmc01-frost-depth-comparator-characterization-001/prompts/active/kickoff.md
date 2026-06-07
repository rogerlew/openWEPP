# FDMC01 Kickoff — frost depth model comparator characterization

Execution mode: package-end-to-end (characterization)

Autonomy: execute the characterization end-to-end (locate legacy frost-depth surface,
run both engines, compare, write verdict) without asking for direction on intermediate
steps. Ask only if hard-blocked (legacy `frdp` not obtainable in the environment).

## This is characterization, not a fix

Size the frost **depth model** gap FQ-4 deferred: openWEPP computes frost depth with a
freeze-index proxy (`frdp = 0.20·clamp(−mean_temp/6)`, capped 0.20 m); legacy
`frostn.for` uses a layered energy-balance heat-flow model (`frdp ≤ 1.0 m`). Frost
activation and conservation are already closed (FQ-4) and the kfactor conductivity
magnitude is legacy-faithful — **neither is in scope.** Measure the depth/duration gap;
do **not** edit the frost kernel, constants, or `SC-SNOWFREEZE-001`.

## Substrate

`/wc1/runs/al/algebraic-radium/wepp/runs/` single-OFE prefixes — `ksflag=1`, frost
active in both openWEPP and legacy `wepp_260606_hill`. **NOT** `arboreal-dendrite`
(forest soil, `ksflag` off, MOFE → no standard frost; that is a rung-3 substrate).
17-OFE MOFE hillslope out of scope.

## Steps

1. Find where legacy `wepp_260606_hill` exposes frost depth (`frdp`)/frozen-soil state
   (frost/winter output, water-balance output, or debug). If not parseable, record a
   feasibility finding + minimal-acquisition proposal.
2. Run openWEPP (`frost.runtime_frdp_m`/`dfrost`, `ws_frz`/`frozwt`, frozen-days) and
   legacy on the cohort.
3. Compare per hillslope: max depth, frozen-days, onset/thaw timing, series shape; note
   where the openWEPP 0.20 m cap binds vs legacy depth.
4. Bounded materiality: the conductivity bite is near-total whenever frost exists, so
   depth/duration delta ≈ how long the bite runs → runoff shift. Characterize
   qualitatively (full magnitude is post-MOFE).
5. Verdict for backlog promotion: crude-but-close vs materially off → recommend (1)
   heat-flow parity DC or (2) contract amendment sanctioning the proxy. State the
   comparator-as-sizing-yardstick-not-acceptance-target distinction (ADR-0017).

## Hard constraints

- No production / contract / production-test change — characterization + analysis
  artifacts only.
- Snow magnitude (Stage-2), forest `ksatadj` (separate), MOFE/17-OFE (rung-3), ET,
  runoff, frost activation — all out of scope / closed; do not touch.

## Required reading

- `docs/work-packages/20260608-fdmc01-frost-depth-comparator-characterization-001/package.md`
- `docs/backlog/20260607-frost-depth-model-heat-flow-parity.md` (the item this sizes)
- `docs/ROADMAP.md` (Stage-2 queue item 2), ADR-0011/0017, `AGENTS.md`
- `SC-SNOWFREEZE-001.md` (`INV-SNOWFREEZE-006`/`-012`/`-013`, `GAP-SNOWFREEZE-002`)
- FQ-4 package + artifacts (proxy localization, frost activation evidence)
- Legacy `/workdir/wepp-forest_260430_baseline/src/frostn.for` (+ `frzng`/`frznw`/`frsoil`)
- Comparator `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
