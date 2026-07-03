# Disturbed-burn anchor — forest high severity fire / loam (ksatadj=1)

Real wepppy-produced burned-forest hillslope, the **`ksatadj = 1` anchor for
WS-2** (the `ksatadj` effective-conductivity re-port). Sibling to
`forest_high_severity_clay_loam/` (the WS-3 clay-loam anchor).

## Provenance
WEPP hillslope **313** from run `honeyed-marathoner`
(`/wc1/runs/ho/honeyed-marathoner/wepp/runs/p313.*`, build 2026-06-23). Copied
with trailing whitespace / EOF blank lines normalized for `git diff --check`
(token content unchanged): `p313.man`, `p313.sol`, `p313.slp`, `pmetpara.txt`,
`snow.txt`, `gwcoeff.txt`. `p313.run.toml` is an openWEPP-native runfile added
for the CLI.

`p313.cli` was additionally **`rad`-clamped** with `tools/clamp_cli_radly.py`
(112 days capped to `floor(r3)`, the openWEPP sunmap horizontal daily potential;
this old CLIGEN output predates the generator-side clamp). Only out-of-domain
days changed; all other columns/rows are byte-identical.

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
**Runs end-to-end** through the production hillslope CLI (exit 0), emitting
`H313.hbp` / `H313.loss.json` / `H313.wat.parquet`. This is the **pre-WS-2
baseline** — `ksatadj` is not yet implemented, so the conductivity is base +
frost only (the `ksatadj = 1` policy is parsed and projected but has no consumer
at HEAD). WS-2 will change this hillslope's conductivity for the `ksatadj = 1`
soil and re-anchor against it.

Before the `rad` clamp above, the full run failed in winter hourly forcing with
`CLIM-RUNTIME-E-017: radly=411 out of domain` (day-of-year 40, `r3=403.3`) — an
old-CLIGEN over-generation on a low-sun day, unrelated to `ksatadj`. The clamp
resolves it faithfully (caps to the same potential the runtime guard uses).
