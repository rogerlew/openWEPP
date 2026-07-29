# Independent Scientific and Coefficient Verification A

Evidence class: `Static` + `Ran` documentation and ledger checks

## Verification scope

I independently verified every finding from `review-agent-a.md` against the
corrected narrative, coefficient ledger, claim map, active management schema,
native initial-seed projection, and governing plant/residue contracts. I also
rechecked the complete terminal coefficient contract. No empirical analysis or
model simulation was rerun.

## Commands and results

The following checks ran from the repository root:

```text
.venv/bin/python - <<'PY'
# csv.DictReader audit of every required coefficient-ledger column,
# allowed range classes, and the exact 21-row expected field set
PY
```

Result:

```text
rows=21 columns=21
missing_required_cells=[]
invalid_range_classes=[]
missing_expected_fields=[]
unexpected_fields=[]
```

```text
markdown-doc lint --path usersum/openwepp-canopy-phenology.md
```

Result: `1 files validated, 0 errors, 0 warnings`.

```text
markdown-doc lint --path \
  docs/work-packages/20260729-canopy-doc-01-canopy-phenology-usersum-001
```

Result: `20 files validated, 0 errors, 0 warnings`.

```text
rg -n \
  "FOREST_LANUSE_SENTINEL|legacy_residue_depth_conversion_factor|landuse == 1|0\\.6" \
  crates/openwepp-input-contract/src/parsers/management.rs \
  crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs
```

Result: the parser fixes native forest to `FOREST_LANUSE_SENTINEL=3`; the
initial-seed path passes `diam` into
`legacy_residue_depth_conversion_factor`; that function applies
diameter-specific categories only when `landuse == 1` and returns the fixed
non-cropland factor `0.6` otherwise.

```text
rg -n "Olson|Qualls|Lim, H\\., et al\\.|0\\.5 yr" \
  usersum/openwepp-canopy-phenology.md \
  docs/work-packages/20260729-canopy-doc-01-canopy-phenology-usersum-001/artifacts/source-and-claim-map.md
git diff --check
```

Result: Olson and Qualls citations and the scoped fallback claim are present;
the incomplete Lim bibliography form is absent; `git diff --check` passed.

## Finding closure

### A-01 — CLOSED

The public guide now inventories `cf` and `diam` with exact field names, units,
hard domains, range status, direction, and calibration cautions
([`usersum/openwepp-canopy-phenology.md`:203](../../../../usersum/openwepp-canopy-phenology.md)).
It separately identifies `initial_conditions[].inrcov`, `.rilcov`, and
`.sumsrm`, explains the initial cover/mass/depth lineage, and states that the
runtime ratio is derived rather than independently tuned.

Ledger rows 18-20 replace the prior synthetic-only treatment with exact `cf`,
`diam`, and derived-ratio records. The important direct-code nuance is correct:
native forest carries `landuse=3`, but the diameter-dependent conversion
branches are guarded by `landuse == 1`. The narrative and ledger therefore
correctly state that `diam` is validated and passed through yet is currently
inert for native-forest depth, which uses the fixed `0.6` non-cropland factor.

### A-02 — CLOSED

The coefficient ledger now has explicit `user_facing_label`,
`equation_location`, `effect_direction`, `scale`, and
`minimum_observations` columns in addition to units, hard domain, value status,
range class/authority, calibration target, identifiability, outputs, warnings,
and sources. Every required cell is populated in all 21 rows. The equation
locations bind the GSI indicators/window, foliar/LAI realization, cover,
height, surface/root decomposition, residue cover, external influx, and
derived depth lineages rather than citing only coarse files.

### A-03 — CLOSED

The corrected litter primer explains separate surface and root pools,
same-day source-before-decay ordering, temperature/moisture-modified
first-order exponential decay, mass-to-cover conversion, the distinct
mass-to-depth conversion, and their erosion/frost consequences
([`usersum/openwepp-canopy-phenology.md`:212](../../../../usersum/openwepp-canopy-phenology.md)).
The later calibration section retains the need for repeated material-separated
inputs and stock observations and explains source/rate equifinality.

### A-04 — CLOSED

The numeric fallback is now described as a narrow runtime rule, explicitly not
a calibrated value, typical range, or recommended setting
([`usersum/openwepp-canopy-phenology.md`:232](../../../../usersum/openwepp-canopy-phenology.md)).
Olson (1963) and Qualls (2016) appear in the public references, and the claim
map binds their first-order/forest-floor rate-class support separately from the
contract's exact `0.5 yr^-1` implementation constant.

### A-05 — CLOSED

The Lim et al. reference now supplies the complete 19-author list and the
authenticated DOI
([`usersum/openwepp-canopy-phenology.md`:353](../../../../usersum/openwepp-canopy-phenology.md)).
The bibliography no longer uses `et al.` as an author-list substitute.

## Terminal coefficient-contract verification

The terminal inventory contains:

- all six temperature, VPD, and photoperiod thresholds;
- summer foliar biomass, evergreen fraction, structural cover, and structural
  biomass;
- `xmxlai`, `bb`, `bbb`, and `hmax`;
- `oratea` and `orater`;
- the active residue-cover field `cf`, branch-inert native depth descriptor
  `diam`, and the exact derived mass-to-depth lineage; and
- authenticated needle and fine-woody external forcing boundaries.

Every row supplies units, hard domain, equation/process location, effect
direction, value status, an allowed range class, range authority, ecological
scope and scale, calibration target, minimum observation needs,
identifiability/equifinality treatment, coupled outputs, transfer or
compensation warning, and source binding. Numeric ranges remain limited to the
Hubbard Brook accepted timing ensemble and mature-LAI source interval. All
other unsupported general ranges remain `NOT_ESTABLISHED`; no schema example,
search domain, synthetic value, or single-site result is promoted.

Calibration remains sequential and observation-driven. It preserves timing
threshold covariance, litter-source/decomposition separation, no-refit
transfer evaluation, unsupported Harvard transfer, the tropical dry-forest
stop-loss, and the prohibition on tuning canopy parameters to compensate for
downstream residuals.

## Remaining findings

None.

## Disposition

`PASS`

All A findings are closed, the terminal coefficient contract is complete, and
the reviewed scientific, range-authority, decomposition, citation, calibration,
and assurance-handoff boundaries are internally consistent.
