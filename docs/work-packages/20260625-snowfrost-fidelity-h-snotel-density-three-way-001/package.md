# SNOWFROST-FIDELITY-H SNOTEL Density Three-Way Comparison

Status: executed-held at `HOLD-PYSNOBAL-CSS-WY2017-SNOBAL-CORE-FAILURE`.

Package type: external-observation acquisition + multi-model comparison /
characterization.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: acquire NRCS SNOTEL SWE + snow-depth (+ soil-temperature) for the
five `tests/fixtures/snotel_observed/` sites, normalize them into a reproducible
local corpus with **observed bulk density** (`SWE / depth`), and run each site
**three ways — openWEPP, pinned legacy WEPP, and PySnobal — against the SNOTEL
observations**, so the program can decide whether openWEPP's modeled snow-depth
over-prediction is driven by snow density (the `snow.txt` settling-density / SSD
parameter), by over-accumulation (SWE error), or by model structure. Evaluation
authority is the `SC-SNOWFREEZE-001` v74 signature rubric
(`INV-SNOWFREEZE-050` + `TOL-SNOWFREEZE-011`): H must emit a
per-model/per-site/per-cell profile, not a scalar accept/reject score.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/codex_exec_plans.md`, `docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`crates/AGENTS.md`, `tests/AGENTS.md`, `tools/snowfreeze_observed/README.md`,
`tests/fixtures/snotel_observed/README.md`, and SNOWFROST-FIDELITY-A through G1.

Subagent authorization: this package authorizes read-only data-provenance,
science-review, harness-review, and verification subagents for SNOTEL
acquisition review, density-correspondence legitimacy review, three-way
comparator anti-alias review, SSD-arm anti-tuning review, and final verification.
Subagents may not edit files; findings go to `artifacts/review-disposition.md`
and `artifacts/verification.md`.

## Purpose

SNOWFROST-FIDELITY-D/E proved openWEPP over-predicts physical snow depth at the
paired sites (`dominant-modeled-over-observed`, up to 1.6 m), and E ruled out
depth-vs-SWE aliasing and timing. F proved pinned legacy WEPP fails the same
way and that openWEPP SWE ≈ legacy SWE. That leaves an **unresolved fork**: is
the depth error **over-accumulation** (modeled SWE too high vs reality) or **low
density** (SWE about right, depth inflated)? The existing frost-depth pilot
sites have observed *depth* but no observed *SWE*, so the fork could not be
closed — which is why G0/G1 reached for PySnobal.

SNOTEL closes the fork directly: it reports paired **SWE (`WTEQ`)** and **physical
depth (`SNWD`)**, so observed **density = SWE / depth** is measurable, across five
distinct snow climates (Cascades/Sierra maritime, Wasatch intermountain, CO
continental, N.-Rockies St. Joe). Most sites also report **soil temperature
(`STO`)**, which gives PySnobal a real lower-boundary forcing in place of the
constant-ground-temp proxy that caused the G0/G1 `Tg=-0.5` crash.

This package is acquisition + comparison + characterization. It changes no
production snow/frost physics. SSD values are set as **site characterization
grounded in observed climatological density**, not tuned to minimize the depth
residual (see SSD Specification).

## Non-Goals

- Do not change production snow/frost physics, constants, or runtime control flow.
- Do not **tune SSD (or any parameter) to minimize the observed-depth residual.**
  SSD-appropriate values are derived from observed climatological density
  independently; sweeping SSD to fit the target is forbidden.
- Do not enable, port, approximate, or promote `Qwet`/`frzftp`.
- Do not classify openWEPP `OPENWEPP-DEFECTIVE` on legacy or PySnobal agreement
  alone; observed SNOTEL density/depth + `INV-SNOWFREEZE-048` and the new density
  correspondence are the authority (ADR-0017).
- Do not use `TOL-SNOWFREEZE-009` as a standalone snow-model acceptance band.
  Apply the v74 rubric profile: forcing-robust (`R`) cells carry verdict weight;
  forcing-limited (`L`) absolute magnitude cells are reported and discounted.
- Do not default-activate direct runtime or delete compatibility runtime.
- Do not store request-only/non-redistributable raw data in git; do not require
  network access for normal Rust/workspace tests.
- Do not silently install PySnobal/Cython/NumPy/pandas from the harness.

## Authority Envelope

In scope:

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-048` snow-depth correspondence;
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-049` observed-density correspondence
  (`SWE / depth`, pairing window = `SNWD` era, density cap `522 kg m^-3` per
  `INV-SNOWFREEZE-003`);
- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-050` snow/frost fidelity evaluation rubric
  and provisional `TOL-SNOWFREEZE-011` scoring levels;
- `tests/fixtures/snotel_observed/` fixtures and `tests/fixtures/snotel_observed/observations/`;
- NRCS AWDB REST API (`SNTL` network) acquisition + normalization;
- the existing runners: `openwepp-cli-hill`, the pinned legacy baseline
  (`/home/workdir/wepp-forest_260430_baseline`, per F's capture lineage), and the
  PySnobal bridge (`tools/snowfreeze_observed/pysnobal_compare.py`);
- a snow.txt **SSD arm** (as-built `250` vs climate-appropriate) for the two
  empirical models only.

Out of scope:

- production `crates/**` physics edits (a diagnostic SNOTEL network handler and
  density/rubric metric in `tools/` and the harness are allowed; physics is not);
- migration/fringe, SFCC/frozen-K, or heat-flow changes;
- watershed-scale validation;
- request-only datasets.

## Intended Write Set

- `docs/work-packages/20260625-snowfrost-fidelity-h-snotel-density-three-way-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  (observed-density correspondence + rubric/tolerance amendments; contract-first)
- `tests/fixtures/snotel_observed/observations/**` (normalized corpus + provenance)
- `tools/snowfreeze_observed/observed_harness.py` (SNTL network handler, inch/°F
  conversion, observed density)
- `tools/snowfreeze_observed/legacy_snow_compare.py`,
  `tools/snowfreeze_observed/pysnobal_compare.py` (SNOTEL site support, SSD arms,
  STO ground forcing)
- a three-way comparison/report tool under `tools/snowfreeze_observed/`
- the snotel fixtures' working snow.txt **copies under the SSD-arm run dirs**
  (not the committed fixtures — those stay as-built `ksflag=1`, SSD `250`)

## SSD Specification (snow settling density)

`snow.txt` field 3 ("snow settling density") seeds `densg` — the snowpack bulk
density that the daily settling factor evolves (cap `522 kg m^-3`,
`INV-SNOWFREEZE-003`). The wepp.cloud default is **`250 kg m^-3` at all five
sites**, which is low for maritime ripe snow and is a prime suspect for the
depth over-prediction (`depth = SWE / density` → low density inflates depth).

Run **two SSD arms** for the two empirical models (openWEPP and legacy WEPP;
PySnobal derives density from energy balance and ignores `snow.txt`):

1. **as-built arm** — SSD `250` (the committed fixture value). Establishes "what
   WEPP does as configured."
2. **climate-appropriate arm** — SSD set to the site's **observed climatological
   settled density** (computed in Phase 1 from SNOTEL peak-SWE-period `SWE/depth`),
   with the literature priors below as starting values. Establishes "what WEPP
   does with a physically-correct density input."

Provisional priors (refine to observed per site; do not fit to the depth residual):

| Fixture | Snow climate | as-built SSD | provisional appropriate SSD (kg m^-3) |
|---|---|---:|---:|
| `snotel_paradise_wa` | Cascades maritime | 250 | ~480 |
| `snotel_css_lab_ca` | Sierra maritime | 250 | ~480 |
| `snotel_mica_creek_st_joe_id` | N. Rockies intermountain | 250 | ~350 |
| `snotel_snowbird_ut` | Wasatch intermountain | 250 | ~350 |
| `snotel_niwot_co` | CO Front Range continental | 250 | ~300 |

**Anti-tuning rule:** the appropriate SSD is derived from observed density as
site characterization, recorded with its derivation, *before* the depth
comparison is run. It is invalid to choose SSD by minimizing the modeled-vs-
observed depth residual. The contrast between the two arms is the diagnostic: if
the over-prediction closes under the appropriate-SSD arm, the depth error is the
density input; if it persists, the error is structural (settling dynamics or
accumulation).

Phase-0 gate: **verify how WEPP's winter routine actually consumes the
settling-density parameter** (`snowd.for` `densg`/`densgt` settling factor;
wepppy `snow.txt` field mapping) before applying the appropriate-SSD arm, and
record it in `artifacts/ssd-semantics.md`. If field 3 is not the density seed
assumed here, stop and report.

## Phase Plan

### Phase 0: Scaffold, SSD semantics, density correspondence
- Required reading; record SSD parameter semantics (`artifacts/ssd-semantics.md`).
- Draft the observed-density correspondence into `SC-SNOWFREEZE-001` (contract-
  first), with a provisional density tolerance flagged for hydrology-reviewer
  ratification.
- Incorporate the `SC-SNOWFREEZE-001` v74 rubric amendment before rerun:
  `INV-SNOWFREEZE-050` is the evaluation authority; H reports profile cells
  using `R`/`L` tiers and KGE decomposition where applicable.
- Exit: SSD semantics recorded; density correspondence and rubric authority
  drafted; SSD arms and profile cells defined.

### Phase 1: SNOTEL acquisition and observed density
- Add a `SNTL` network handler to the observed harness (AWDB REST API; inch→m/mm,
  °F→°C). Pull `WTEQ, SNWD, PREC, TOBS, TMAX, TMIN, STO` for the five triplets.
- Normalize into `tests/fixtures/snotel_observed/observations/` with provenance
  locks (URL, access date, checksum, units, censoring, site mapping).
- Compute observed daily density = `SWE/depth` over the `SNWD` era only; derive
  each site's climatological settled density (peak-SWE-period) for the SSD arm.
- Exit: corpus + provenance present; observed density + per-site appropriate SSD
  recorded; raw downloads in cache, not git.

### Phase 2: Three-way model runs
- **openWEPP:** run each snotel fixture via `openwepp-cli-hill`; extract WAT
  `Snow-Water` (SWE) + `Snow-Depth`; both SSD arms.
- **legacy WEPP:** run the pinned baseline per F's capture lineage (WAT
  `Snow-Water` for SWE; daily-winter hour-24 rows for physical depth/density,
  `.run` daily-winter answer `No`→`Yes` in a temporary replay); both SSD arms.
- **PySnobal:** run the G0/G1 bridge; **use SNOTEL `STO` as the ground-temp
  forcing** where available (Paradise/CSS/Snowbird/Mica) to replace the constant-
  `Tg` proxy that caused the G0/G1 crash; Niwot (no `STO`) uses a documented
  fallback. Use water-year segmented diagnostic runs for the SNOTEL leg so the
  PySnobal comparison does not carry one synthetic snowpack state across
  multiple decades. PySnobal has no SSD arm.
- Exit: SWE + depth (+ density) series for all three models × five sites; the
  G0/G1 `Tg` crash is avoided or explicitly dispositioned with the `STO` forcing.

### Phase 3: Three-way rubric profile and density fork routing
- Compare modeled vs observed **SWE, depth, and density** for all three models ×
  five sites: signed/abs residuals, snow-control status, climate stratification.
- Emit a v74 rubric profile for every model/site: long-term peak SWE/depth
  magnitude (`L`), cold-season density (`R`), snow-cover duration (`R`), seasonal
  SWE/depth/density KGE cells, onset/peak/melt-out timing, depth-SWE slope, and
  explicit unavailable event/conservation cells.
- Decompose KGE cells into `r`, `beta`, and `gamma`; report ordinal
  fail/marginal/pass/strong per `TOL-SNOWFREEZE-011`. Observation-only failures
  remain `UNRESOLVED` unless independent authority exists; legacy/PySnobal are
  profile overlays, not targets.
- Resolve the fork per site: `OVER-ACCUMULATION` (SWE high vs SNOTEL), `LOW-
  DENSITY` (SWE ok, depth/density off, closes under appropriate-SSD), or
  `STRUCTURAL` (persists under appropriate-SSD) as diagnostic routing metadata,
  not the evaluation authority.
- Exit: `artifacts/three-way-comparison.md`, `artifacts/rubric-profile.md`, and
  `artifacts/density-verdict.md`; any production remediation routed to a follow-
  up package with a defect ID.

### Phase 4: Review, verification, closure
- Dual review (data-provenance + comparator/anti-tuning), verification, line-count
  governance, worker handoff. Update ROADMAP/catalog. Close complete or HOLD.

## Validation Commands (run from `/home/workdir/openWEPP`)

- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- SNOTEL acquisition + normalization command(s) (record in `artifacts/acquisition-log.md`)
- `.venv/bin/python tools/snowfreeze_observed/observed_harness.py compare --site <snotel_site> ...`
- legacy + PySnobal three-way run commands (record exact invocations + binary hashes)
- rubric profile report generated by `snotel_density_three_way.py compare`
- `.venv/bin/python -m py_compile tools/snowfreeze_observed/*.py`
- `cargo fmt --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
  `cargo test --workspace`; `cargo deny check`; `git diff --check`

## Acceptance Criteria

- SNOTEL SWE+depth (+STO) acquired for all five sites with provenance locks;
  observed density computed over the `SNWD` era; appropriate SSD derived from
  observed climatology (not residual-fit) and recorded with derivation.
- All three models run for all five sites; comparisons are metric-bearing for
  SWE, depth, and density; the PySnobal `Tg` crash is resolved or dispositioned.
- Each model/site has a v74 rubric profile with `R`/`L` cells, KGE decomposition
  where applicable, ordinal labels, and per-cell ADR-0017 disposition.
- Each site has a diagnostic fork route (`OVER-ACCUMULATION` / `LOW-DENSITY` /
  `STRUCTURAL`) with the two-SSD-arm contrast as evidence; no route relies on
  legacy/PySnobal agreement alone or on SSD tuned to the target.
- No production physics changed; density correspondence is contract-first and
  provisional; Rust/docs gates pass.

## HOLD Boundaries

Close as `HOLD` if: a SNOTEL source is unacquirable with no reproducible fetch
path; the SSD semantics differ from this package's assumption (Phase-0 gate
fails); the PySnobal `STO`-forced, water-year segmented runs still abort without
a dispositionable cause; the rubric profile cannot be emitted; or the three-way
comparison cannot be made non-tautologically (e.g., legacy depth capture cannot
be date-aligned).
