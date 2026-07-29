# Independent Verification B

Status: `complete`

Evidence class: `Static + Ran`

Disposition: `PASS`

## Verification scope

Independently verified every Review B finding against the corrected usersum
narrative, coefficient-authority ledger, active native-forest schema fields,
and runtime residue-depth derivation. Also rechecked the terminal coefficient
contract, reader terminology, bibliography, self-contained links,
calibration/downstream-compensation guidance, DOC/ASSURE separation, and
working-tree write-set discipline.

## Checks run

| Check | Result |
| --- | --- |
| Parsed `coefficient-authority-ledger.csv` with Python's CSV reader and required all 18 contract-bearing metadata columns to be nonempty for every row | PASS: 21 rows, 21 columns, no missing required value |
| Inspected `PlantScenario::NativeForest`, `InitialConditionScenario::NativeForest`, `legacy_initial_residue_depth_m`, `legacy_residue_depth_conversion_factor`, and `direct_production_typed_residue_cover_authority` | PASS: exact `cf`, `diam`, `inrcov`, `rilcov`, and `sumsrm` lineage and derived runtime ratio agree with the public guide |
| `rg -n '\]\(' usersum/openwepp-canopy-phenology.md` plus local target existence check | PASS: the sole narrative link targets an existing file within `usersum` |
| Negative `rg` checks for repository-only relative links and internal assurance/work-package vocabulary | PASS: no matches |
| Crossref DOI metadata queries for Donnelly et al. (2022), Lim et al. (2024), and Qualls (2016) | PASS: titles and complete author lists agree with the bibliography |
| `markdown-doc lint --path usersum` | PASS: 12 files validated, 0 errors, 0 warnings |
| `git diff --check` | PASS |
| `git diff --name-only` and `git status --short` inspection | PASS: all current changes are within the declared package write set; no production, schema, contract, prior-evidence, or assurance-publication surface changed |

## Review B finding closure

### B-01 — Closed

The narrative now states that every scalar in the three tables is a required
input and that no default native-forest vector ships
(`usersum/openwepp-canopy-phenology.md:139-143`). The new field-group map
binds every coefficient to an equation/process location and minimum
observation need (`:153-165`), while the tables retain field-specific units,
hard domains, range status, and effect direction (`:167-210`). Calibration
targets, equifinality, transfer limits, and the downstream-compensation guard
remain explicit in the calibration section (`:255-296`).

The ledger header now has separate fields for user-facing label, equation
location, effect direction, value status, scale, minimum observations,
identifiability, coupled outputs, warnings, and source paths
(`artifacts/coefficient-authority-ledger.csv:1`). All 21 rows populate every
required metadata field. No accepted ensemble, source interval, schema
example, or fallback is promoted to a general default or typical range.

### B-02 — Closed

The public guide explicitly says there is no standalone native-forest YAML
depth coefficient and names the exact input lineage:
`initial_conditions[].inrcov`, `.rilcov`, `.sumsrm`, and the referenced
plant's `cf` and `diam`
(`usersum/openwepp-canopy-phenology.md:221-230`). It separately names the
derived runtime `residue_depth_conversion_m_per_kg_m2` and prohibits
independent tuning.

Static code inspection confirms the description: initial cover is inverted
with `cf`; the native `landuse=3` path uses the fixed non-cropland depth
factor, so `diam` does not alter current native-forest depth; the runtime ratio
is initial depth divided by initial `sumsrm` when that mass is positive. The
ledger now records `cf`, `diam`, and the derived ratio as separate exact
lineage rows (`artifacts/coefficient-authority-ledger.csv:18-20`) rather than
using the former placeholder.

### B-03 — Closed

The narrative expands “overland-flow element (OFE)” at first use and identifies
`not_represented` as a YAML status token
(`usersum/openwepp-canopy-phenology.md:239-247`).

### B-04 — Closed

The Lim et al. bibliography entry now carries the complete published 19-author
list (`usersum/openwepp-canopy-phenology.md:353-358`). Crossref metadata
matches the listed authors and title. The corrected Donnelly and added Qualls
entries were also checked against DOI metadata.

## Terminal contract and boundary

The corrected guide covers the six timing thresholds, foliar and persistent
structure, LAI/cover/height realization, above- and below-ground
decomposition, immediate residue cover/depth controls, and authenticated
needle/fine-woody forcing. It preserves the required observation-driven order:
composition and structure, full-leaf canopy state, joint timing, independent
litter/decomposition, then transfer evaluation without refit.

The document remains a model-science narrative rather than an assurance
report. Hubbard and mature-LAI values are scoped configuration evidence;
Harvard and tropical-dry-forest outcomes remain qualitative limitations.
There are no assurance result tables, reproduction procedures, internal
verdict machinery, or publication claims. The closing section states what a
native-forest run may and may not support.

## Remaining findings

None.

## Closure judgment

All Review B findings are corrected, the terminal usersum/coefficient contract
passes, and the DOC/ASSURE boundary remains legitimate. Verification B is
`PASS`.
