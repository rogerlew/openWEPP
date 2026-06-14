# Watershed Routing Scope

Status: W-A executed

Evidence mode: Ran + Static

## Scope Decision

WSHED01 remains in scope. W-A found the first hard blocker at the
impoundment parser, not in MOFE hillslope inputs. The next increments should
proceed as:

1. W-B: contract-first no-impoundment parser fix.
2. W-C: watershed dispatch/output publication from real routed operands.
3. W-D: wepppy totalwatsed3 audit on the routed output.

## Legacy Authority Map

No-impoundment state:

- `wshinp.for:228-253` sets `npond=0` and increments only on `elmt=3`.
- `wshini.for:321-345` reads/checks impoundment-file `jpond` only when
  `npond.gt.0`.
- `impint.for:523-525` loops over `1..npond`, so zero impoundments skip the
  payload loop.
- `wshdrv.for:1228-1296` guards impoundment output routines with
  `npond.gt.0`.

Routing authority:

- `wshdrv.for:891-906` walks watershed elements and routes impoundment elements
  with `wshiqi`/`wshimp`.
- `wshrun.for:1-7` states the watershed runoff-routing routine covers channels
  and impoundments.
- `wshini.for:214-299` builds channel contributing areas and explicitly
  ignores/masks impoundment relationships when deriving channel-area arrays.
- `chnrt.for:217-243` distinguishes lateral-inflow and no-lateral-inflow
  channel runoff cases for sediment/channel routing.

## openWEPP Seam Map

Input/parse sequence:

- `openwepp-cli-watershed.rs:223-254` parses channel, slope, then
  impoundment. The current blocker is here.
- `openwepp-cli-watershed.rs:258-295` parses `chan.inp` and builds a runtime
  surface after the impoundment parse.
- `openwepp-cli-watershed.rs:327-343` parses HBP pass shards after
  channel/impoundment runtime seeding.
- `openwepp-cli-watershed.rs:476-497` runs `Ws10ChannelImpoundmentKernel`,
  builds a row seed, and writes interchange parquet.

Routing execution:

- `openwepp-watershed-orchestrator/src/lib_mod/types.rs:90-113` defines the
  mutable state/flux writeback surface and execution report.
- `dispatch.rs:155-266` schedules topology nodes, invokes the watershed kernel
  per step, evaluates writeback, and applies state/flux updates.
- `kernel_core.rs:16-35` dispatches node execution to channel or impoundment
  handlers.
- `validation.rs:382-393` publishes channel `Qpo`, `Durrof`, and `Roff`.
- `validation.rs:884-910` publishes impoundment `Qo`, `Durout`, `Hnext`, and
  outflow volume.

Output publication:

- `openwepp-watershed-output/src/contracts.rs:5-19` defines the required
  watershed output config, including `totalwatsed3`.
- `contracts.rs:77-96` enumerates the 14 required output paths.
- `writers.rs:145-158` writes the required output set and includes
  `totalwatsed3`.
- `writers.rs:1516-1614` currently builds a single-row record batch and
  defaults unmapped float fields to `0.0`.
- `openwepp-cli-watershed.rs:1908-1975` currently builds the row seed mainly
  from routed channel/impoundment runoff and sediment fields.

W-C must therefore do more than prove the writer emits files. It must bind real
date-indexed watershed water-balance operands into the totalwatsed3 schema and
reject placeholder/default-zero publication.

## totalwatsed3 Input Contract

openWEPP watershed schema:

- `writers.rs:814-829`: date columns plus `runvol`.
- `writers.rs:892-927`: `Area`, `P`, `RM`, `Q`, `Dp`, and `latqcc`.
- `writers.rs:964-1019`: storage fields including `Total-Soil Water`,
  `SoilWaterTotal`, profile stores, `InterceptionStorage`, `frozwt`.
- `writers.rs:1020-1097`: `Snow-Water`, `Precipitation`, `Rain+Melt`,
  `Percolation`, `Lateral Flow`, `Runoff`, `Transpiration`, `Evaporation`,
  and `ET`.
- `writers.rs:1098-1120`: `Baseflow`, `Aquifer losses`,
  `Reservoir Volume`, and `Streamflow`.

wepppy producer/audit expectations:

- `/home/workdir/wepppy/wepppy/wepp/interchange/totalwatsed3.py:72-135`
  defines the same parquet schema and water-balance columns.
- `totalwatsed3.py:955-1006` converts volume columns to depths and derives
  `Precipitation`, `Rain+Melt`, `Percolation`, `Lateral Flow`, `Runoff`, `ET`,
  baseflow, and `Streamflow`.
- `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py:76-87`
  requires the core volume/storage columns and loads optional reported depth
  columns when present.
- `totalwatsed3_daily_closure_audit.py:90-153` computes the daily closure
  identity from independent volume-derived and reported operands.
- `totalwatsed3_daily_closure_audit.py:188-226` records storage/profiling
  diagnostics, including `SoilWaterTotal` vs legacy storage.

## Conservation Identity

Primary daily acceptance identity for routed watershed output:

```text
residual_mm =
  Precipitation
  - (Runoff + Lateral Flow + ET + Percolation + Interception)
  - delta_storage
```

Storage must be independently measured. The current wepppy audit computes:

- legacy storage: `Total-Soil Water + frozwt + Snow-Water`;
- enriched storage when present: `SoilWaterTotal + Snow-Water`.

Acceptance must use independent operands:

- reported depth columns cannot be the only operands if they were copied from
  the same calculation under test;
- reconstructed depth columns from volumes must be compared against reported
  depths;
- exact `0.0` residuals are suspect unless explained by an empty or synthetic
  fixture. The real arboreal-dendrite routed run should close at
  nonzero-at-noise, consistent with MOFE01.

## Red Tests for Next Increments

W-B red tests:

- `jpond=0` with `expected_structural_count=Some(0)` accepts an empty
  `WatershedImpoundmentFile` in strict and compatibility modes.
- `jpond=0` with `expected_structural_count=Some(>0)` fails with a typed count
  mismatch.
- Negative/non-numeric `jpond` still fails typed.
- Existing active-impoundment fixtures still parse and seed runtime symbols.
- Arboreal-dendrite CLI proceeds past `CLIWAT-E-010`.

W-C red tests:

- Watershed CLI emits all 14 configured parquet outputs after W-B.
- `totalwatsed3.parquet` has more than one daily row for real runs; the
  one-row/default seed path is not accepted for arboreal-dendrite closure.
- Water-balance fields are non-placeholder: `Area`, `P`, `RM`, storage,
  `runvol`, `Dp`, `latqcc`, `Ep`, `Es`, and `Er` must be sourced from routed
  HBP/WAT inputs or a documented zero-producing process, not writer defaults.
- Reported depth columns agree with volume-derived depths at the established
  tolerance.

W-D red tests:

- Run `/home/workdir/wepppy/tools/totalwatsed3_daily_closure_audit.py` on the
  routed `totalwatsed3.parquet`.
- Whole-run and daily closure residuals are at the established floor.
- Residuals are nonzero-at-noise for the real routed run; exact-zero closure is
  not accepted as proof.

## Sizing

W-B is a small parser/test correction. W-C is the main implementation slice and
may split if the first post-W-B CLI run exposes a separate routing-vs-publication
defect. W-D is cross-repo validation only unless the wepppy consumer rejects a
valid openWEPP schema; any wepppy production edit remains out of scope without a
new explicit package scope.
