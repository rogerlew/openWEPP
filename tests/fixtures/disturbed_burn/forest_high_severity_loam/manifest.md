# Disturbed-burn anchor — forest high severity fire / loam (ksatadj=1)

Real wepppy-produced burned-forest hillslope, the **`ksatadj = 1` anchor for
WS-2** (the `ksatadj` effective-conductivity re-port). Sibling to
`forest_high_severity_clay_loam/` (the WS-3 clay-loam anchor).

## Provenance
WEPP hillslope **313** from run `honeyed-marathoner`
(`/wc1/runs/ho/honeyed-marathoner/wepp/runs/p313.*`, build 2026-06-23). Copied
verbatim (trailing whitespace / EOF blank lines normalized for
`git diff --check`; token content unchanged): `p313.man`, `p313.sol`,
`p313.slp`, `p313.cli`, `pmetpara.txt`, `snow.txt`, `gwcoeff.txt`.
`p313.run.toml` is an openWEPP-native runfile added for the CLI.

## Why this fixture (WS-2 anchor)
`p313.sol` is a datver-**9002** disturbed soil carrying all three campaign
drivers:
- `ksatadj = 1` (the WS-2 target — effective-conductivity adjustment),
- `luse = forest high sev fire`, `stext = loam`,
- `keffflag = 1`, `lkeff = 0.1` (hydrophobicity floor),
- `ksflag = 0` (the legacy frost lever — inert in openWEPP, `ksflag→frost` is
  already decoupled; frost stays on).

This is the missing `ksatadj = 1` validation input: the prior `ksatadj` fix was
**byte-inert on H2637** because `ksatadj = 0` there. Run 313 actually exercises
the `ksatadj` branch. `p313.man` is the cropland-encoded forest masquerade
(`landuse = 1`, plant `Tah_6892`), 6 sim-years; `ksatadj` is **soil-side**, so
the `.man` encoding is irrelevant to the WS-2 conductivity path. Plant `Tah_6892`
has an explicit `pmetpara.txt` record (kcb 0.45), so PMET resolves without
fallback.

## Current run status (HEAD, pre-WS-2)
Parsing is clean end-to-end (9002 disturbed soil, 2023.3 slope, climate, PMET
hit), and the run reaches the direct production executor — but the **full run
does not yet complete** on this climate: it fails in winter hourly forcing with
`CLIM-RUNTIME-E-017: runtime context symbol radly=411 is out of domain (allowed
0 <= radly <= baseline sunmap horizontal daily potential rpoth/r3)`. That is an
**unrelated climate radiation-domain guard** (a real CLIGEN day exceeding the
computed horizontal potential), not a `ksatadj`/conductivity issue, and it is
independent of WS-2.

**WS-2 role, therefore:** this fixture anchors the `ksatadj` **parse + projection
+ conformance-vector operands** (the real `9002` policy row + soil layers), which
is WS-2's primary gate per `SC-SUBHYD-001#INV-SUBHYD-032`. A full end-to-end
run of this hillslope is additionally gated on the separate `radly` radiation
guard (fix the guard, or pair the soil with a radiation-clean climate).
