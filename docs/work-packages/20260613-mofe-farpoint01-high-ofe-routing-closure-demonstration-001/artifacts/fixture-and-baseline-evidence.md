# FARPOINT01 — Fixture + Legacy Baseline Evidence (Increment F-A)

Status: COMPLETE 2026-06-16
Evidence mode: **Ran** (legacy baseline + openWEPP run) + **Static** (provenance, arithmetic)

This artifact records the >10-OFE substrate selected for FARPOINT01, the clean
`wepp_260606` legacy baseline, and the openWEPP run that surfaced **Finding 1**
(the frost-overflow double-count, closed in increment F-B — see
`dc-execplan-frost-overflow-double-count.md`).

## 1. Substrate — H2637 (19 OFEs)

H2637 is the canonical documented >10-OFE legacy case (the WB-05A OFE19 hourly
q-cap investigation). It is the durable choice because **its WEPP inputs are
versioned in-repo** (wepp-forest), not on the ephemeral `/wc1` run store.

- Source inputs (two variants, differing only by the empty `wepp_ui.txt`):
  - `/workdir/wepp-forest/docs/work-packages/20260503-wb05a-h2637-ofe19-hourly-qcap-resolution/artifacts/replays/with_wepp_ui/runs/`
  - `.../without_wepp_ui/runs/`
- Inputs: `p2637.{slp,sol,man,cli}` + sidecars `pmetpara.txt`, `snow.txt`,
  `gwcoeff.txt`, `chntyp.txt`, `chan.inp`, (`wepp_ui.txt` in the with-ui variant).
- Geometry: slope header line 2 = `19` OFEs; management `19 # number of ofes`;
  soil SoilMultipleOfe (19). All three agree → openWEPP OFE-parity check passes.
- Simulation span: 34 years (1987–2020).
- Rejected alternative: the carved-letter MOFE cohort (H182/H237/H240/H248/H121,
  12–14 OFEs) lives only under `/wc1/runs/ca/carved-letter/` with no documented
  p-number→input mapping; not reproducible from the repo.

### Working fixture (staged)
`/tmp/openwepp_farpoint01_h2637/{with_ui,without_ui}/` each contain:
- `runs/` — the legacy inputs + an authored openWEPP TOML runfile `h2637.run`
  (`openwepp-hillslope-runfile-v1`; the two variants differ only in `wepp_ui`).
- `output/` — legacy `wepp_260606_hill` outputs.
- `owepp_output/` — openWEPP outputs.

## 2. Clean legacy baseline — `wepp_260606_hill` (Ran)

Binary: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
(2026-06-06; wepp_260430 + negmelt fix; not stripped). Invocation:
`( cd runs && wepp_260606_hill < p2637.run )`, outputs to `../output/`.

| variant | exit | structure | non-finite |
|---|---|---|---|
| without_ui | 0 ("WEPP COMPLETED … SUCCESSFULLY") | 19 OFE × 1987–2020, 235,961 rows | 0 |
| with_ui    | 0 | same | 0 |

Documented far-point signatures confirmed in the baseline:
- **QOFE/Q = exactly the OFE ordinal** (1.000, 2.000, … 19.000) — clean integers
  because all 19 OFEs share length 26.11 m, so `totlen/slplen` at OFE *k* = *k*.
  This is a reporting *normalization* (the WSHED01 QOFE/Q duality),
  **conservation-neutral**, not a defect.
- **OFE19 q-cap contrast**: terminal-OFE ΣQ jumps 53,317 → 122,656 mm
  (without-ui → with-ui), the WB-05A hourly q-cap behavior. (Legacy comparator is
  a *flag*, ADR-0017; not a match target.)

## 3. openWEPP run — Finding 1 (Ran)

CLI: `target/release/openwepp-cli-hill --run-dir … --run-file h2637.run
--output-dir …/owepp_output --policy compat --legacy-sidecar-discovery`
(built at HEAD `f50b5426`).

- openWEPP **ingests 19 OFEs with no parser rejection** — the "no hard OFE cap"
  property holds in practice.
- Both variants **fail-closed identically** at the per-element WB13 conservation
  gate (no partial output written — correct behavior):

```
CLIHILL-E-011 … HS-SIMPIPE-E-001 per-element storage identity residual
8.231171454338721 mm exceeds tolerance 1e-11; ofe=5
[sim_day_index=3324, calendar_year=1996, julian_day=37]
```

The byte-identical residual across `with_ui`/`without_ui` proves the break is
deterministic and **orthogonal to the `wepp_ui` hourly/q-cap path** — it is a
frost↔routing accounting seam, not the OFE19 q-cap defect.

This is an **in-scope, defect-shaped finding** (package acceptance: "if openWEPP
also breaks at high OFE count, that is a defect-shaped finding, not a package
failure"). It is closed in increment F-B.

Performance note (provisional): openWEPP took 264 s to reach the day-3324 abort
(~27 % of the sim) vs the legacy Fortran's 9–12 s for the whole run. Flagged for
later attribution; not diagnosed here.
