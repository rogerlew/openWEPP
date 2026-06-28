# Sturm Thresholds Source Verification

Evidence mode: `Static`

This artifact supersedes the first-pass extraction for authority. The first-pass
file is retained only as a navigation aid; every threshold below was rechecked
against the source PDFs available in this repository.

## Primary Authority: Sturm 1995

Source: `references/copyrighted/sturm1995.pdf` (R-59, scanned, no text layer).

Verified by visual reading of rendered page images:

| Source location | Verified item | Package use |
|---|---|---|
| PDF page 13, printed p. 1273, Figs. 8-9 | Original dichotomous key and cube structure: temperature branch first; very-high-temperature branch is ephemeral; low-temperature/low-precipitation splits taiga vs tundra by wind; high-temperature/low-precipitation splits alpine vs prairie by wind; high-temperature/high-precipitation is maritime; low-temperature/high-precipitation enters rare deep-snow branches. | Runtime decision-tree structure. Rare deep branches fail closed because they are not one of the six standard Sturm 1995 class labels. |
| PDF page 14, printed p. 1274, Table 7 and CDM definition | CDM uses monthly mean air temperature and `Tc=10 degC`; high/low temperature separation is bracketed by the Alaskan examples and later fixed for mapping. | Run normals compute CDM from monthly means with `Tc=10 degC`. |
| PDF page 15, printed p. 1275 | Precipitation separation is bracketed by `1-3 mm day^-1`; wind separation is bracketed by `0.5-2 m s^-1`. | Precipitation uses the mapping value from p. 1276. Actual-wind runtime classification uses `<=0.5 m s^-1` as clearly low, `>=2.0 m s^-1` as clearly high, and fails closed inside the unresolved interval. |
| PDF page 16, printed p. 1276 | Mapping values selected: `125 degC-month` CDM, `2 mm day^-1` precipitation, wind represented by vegetation stature, and ephemeral threshold adjusted to `30 degC-month`. | Binding 1995 thresholds for this opt-in candidate: CDM `30/125`, SPR `2`, and fail-closed actual-wind bracket. |

Runtime threshold interpretation:

- `CDM < 30 degC-month`: `ephemeral`.
- `30 <= CDM < 125 degC-month`: high-temperature seasonal branch.
- `CDM >= 125 degC-month`: low-temperature seasonal branch.
- `SPR >= 2 mm day^-1`: high precipitation.
- Wind-dependent branches classify low wind only at `<=0.5 m s^-1` and high
  wind only at `>=2.0 m s^-1`; `0.5 < wind < 2.0 m s^-1` is unresolved by the
  1995 source and must fail closed.

The run normals are computed from the run's own climate series: monthly mean
air temperature, monthly mean precipitation rate, and mean wind over months
where monthly mean air temperature is below `Tc`. No site identity, geographic
lookup, observations, NSIDC raster class, or fixture residual enters assignment.

## Cross-Check: Sturm And Liston 2021

Source: `references/copyrighted/hydr-JHM-D-21-0070.1.pdf` (R-61,
born-digital), Fig. 2 and section 2a.

Cross-check findings:

- The decision tree follows Sturm 1995, but the updated product changes the
  ephemeral/seasonal CDM threshold from `30` to `61 degC-month`.
- The high/low precipitation threshold changes from `2` to `4 mm day^-1`.
- The high/low temperature CDM threshold remains `125 degC-month`.
- Class names differ: Sturm 1995 `Taiga` is `Boreal Forest` in 2021, and Sturm
  1995 `Alpine` is `Montane Forest` in 2021.

Package rule: this candidate pairs Sturm 1995 class names with Sturm 2010
density parameters. The 2021 values and names are recorded only as a
cross-check; they are not silently inherited into the 1995-named runtime
candidate.

## Density Parameter Authority

Source: `references/copyrighted/sturm2010_swe_climate_classes.pdf` (R-58),
Eq. 6 and Table 4.

Sturm 2010 supplies density parameters for alpine, maritime, prairie, tundra,
and taiga. Ephemeral measurements were excluded, so no Sturm 2010 ephemeral row
exists. The runtime candidate therefore retains the existing process-first
fresh-snow/Anderson compaction behavior for ephemeral and documents that
fallback; it does not fabricate Sturm parameters.

## No-Fitting Statement

No threshold, class mapping, density parameter, wind decision, rare-branch
mapping, smoothing rule, or fallback constant in this package was fitted to the
SNOTEL/cancov fixtures. The observed-data rubric validates the opt-in candidate;
it does not calibrate it.
