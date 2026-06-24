# SNOWFREEZE Observed Frost-Depth Harness

Status: complete — observation harness/corpus landed; frost-defect attribution
remains `UNRESOLVED` pending modeled snow-depth diagnostics.

Package type: Defect-Closure ExecPlan / external-observation harness and
fidelity characterization.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: acquire the frost-depth observation datasets bound by
`SC-SNOWFREEZE-001` `INV-SNOWFREEZE-047`, normalize them into a reproducible
local validation corpus, and build an openWEPP harness that compares produced
simulation frost-depth outputs against those observations using the contract
measurement-correspondence rules.

This package follows `docs/codex_exec_plans.md`,
`docs/defect_closure_execplans.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`, and
`tests/AGENTS.md`.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only data-provenance, harness-design,
science-contract, and verification subagents for dataset acquisition review,
measurement-correspondence review, harness anti-tautology review, and final
gate verification. Expected outputs are compact findings summarized into
`artifacts/review-disposition.md` and `artifacts/verification.md`; subagents
may not edit files.

## Purpose

R7H closed opt-in because direct-vs-compatibility frost bit-parity is not a
valid frost acceptance target. The governing question is now physical
frost-depth fidelity against historic observations. This package creates the
observation-backed comparison substrate before any frost-model remediation.

The package must make comparison failures actionable without turning
observations into a blunt oracle. Under ADR-0017 and
`INV-SNOWFREEZE-047`, legacy/compatibility output is only a flag. Observation
agreement is the acceptance target, but `OPENWEPP-DEFECTIVE` still requires
like-for-like measurement correspondence, snow-insulation control, censoring
handling, and independent correctness authority.

## Non-Goals

- Do not resume R7H direct-vs-compatibility frost bit-parity.
- Do not default-activate direct runtime.
- Do not delete compatibility runtime, rollback, or shadow paths.
- Do not tune frost physics, coefficients, units, or storage plumbing in this
  package unless the package is amended first with contract authority and
  defect-specific acceptance gates.
- Do not store request-only or non-redistributable raw datasets in git.
- Do not make network access required for normal workspace tests.

## Correction Authority Envelope

Gap:

- `GAP-SNOWFREEZE-002`: frost-depth magnitude fidelity remains open after
  FDHP01 and R7H. Compatibility frost is conservation-closed but not validated
  to observed frost-depth magnitude.

Observed failure class:

- No current reproducible openWEPP harness compares site hillslope model output
  to historic frost-depth observations with the `INV-SNOWFREEZE-047`
  measurement rules.

In-scope mechanisms:

- source acquisition for the five pilot observation authorities named in
  `tests/fixtures/snowfreeze_observed/`;
- normalized observation schema, unit conversion, censoring, and provenance
  locks;
- deterministic local cache/download tooling;
- simulation harness for the five site hillslope fixtures;
- extraction of modeled `frdp` from WAT output and required snow-depth
  diagnostic/control surface;
- comparison metrics for seasonal maximum, observation-date series, onset,
  thaw, frozen duration, and snow-insulation control;
- ADR-0017 verdict report generation;
- focused integration/contract tests for schema, provenance, correspondence,
  and local harness execution.

Out-of-scope mechanisms:

- frost heat-flow formula changes;
- storage/frozen-water rebalance deletion or replacement;
- direct runtime default activation;
- compatibility parity tuning;
- watershed-scale validation.

## Dataset Acquisition Scope

Acquire or script acquisition for the pilot sources below. Every source must
produce a provenance record containing source URL/DOI, access date, license or
terms, raw file checksum when a file is downloaded, parser version, normalized
row count, units, method, site mapping, and any censoring or quality flags.

| Site | Source | Required observation product | Authority role |
| --- | --- | --- | --- |
| Sleepers South Field | USGS ScienceBase / DOI `10.5066/P96753GI` | Frost-tube depth plus paired snow depth | magnitude |
| Sleepers W9 Hardwood | USGS ScienceBase / DOI `10.5066/P96753GI` | Frost-tube depth plus paired snow depth | magnitude/regime |
| SCAN Mandan #1 | USDA NRCS AWDB REST API, station `2020:ND:SCAN` | Soil-temperature profile, snow depth if available | timing/duration and upper-bound |
| GGD498 Morris | NSIDC GGD498 v1 / DOI `10.7265/1mcs-q536` | Frost-tube depth | magnitude, limited overlap |
| Reynolds Creek US-Rls | USDA-ARS / HydroShare or Ag Data Commons soil temperature | Soil-temperature profile | timing/duration and upper-bound |

Full per-site coordinates, periods, methods/cadence, exact access endpoints,
license, censoring, and fixture mapping are catalogued in
`artifacts/dataset-inventory.md`.

Public source references checked during scaffolding:

- USGS Sleepers River frost data catalog:
  `https://data.usgs.gov/datacatalog/data/USGS%3A5e6bce83e4b01d5092632650`
- NRCS SCAN overview:
  `https://www.nrcs.usda.gov/resources/data-and-reports/soil-climate-analysis-network`
- NRCS SCAN soil temperature via AWDB REST API (station `2020:ND:SCAN`, element `STO`):
  `https://wcc.sc.egov.usda.gov/awdbRestApi/services/v1/data?stationTriplets=2020:ND:SCAN&elements=STO:*&duration=DAILY`
- NSIDC GGD498 v1:
  `https://nsidc.org/data/ggd498/versions/1`
- Reynolds Creek data overview:
  `https://www.ars.usda.gov/pacific-west-area/boise-id/northwest-watershed-research-center/docs/reynolds-creek-experimental-watershed-data/`
- Reynolds Creek soil-temperature data (USDA-ARS Box / Ag Data Commons,
  public-domain archive license with citation requested;
  the HydroShare copy is access-gated, do not depend on it):
  `https://ars-usda.app.box.com/s/4jwgmxyxb8vacosvp1t5sdlibdc5qxoe`

If a source requires account credentials, manual approval, or a request-only
workflow, record the blocker in `artifacts/dataset-inventory.md`, provide a
stub manifest row, and keep the normal harness green for the redistributable
subset.

## Expected Normalized Corpus

Preferred checked-in corpus layout:

- `tests/fixtures/snowfreeze_observed/observations/manifest.json`
- `tests/fixtures/snowfreeze_observed/observations/sites/*.csv`
- `tests/fixtures/snowfreeze_observed/observations/provenance/*.json`

Required normalized columns:

- `site_id`
- `source_id`
- `date`
- `water_year`
- `method`
- `authority_role`
- `observed_frost_depth_m`
- `observed_isotherm_depth_m`
- `observed_snow_depth_m`
- `censoring`
- `quality_flag`
- `source_record_id`

Rules:

- All normalized depths are meters.
- Frost-tube depth maps to `observed_frost_depth_m`.
- Soil-temperature profiles derive `observed_isotherm_depth_m` by explicit
  interpolation and never become direct magnitude targets.
- Snow depth, not SWE, controls the insulation gate. If only SWE is available,
  mark snow-control status `UNRESOLVED` unless an independently authoritative
  density/depth conversion is added to `SC-SNOWFREEZE-001`.
- Raw downloads may live in a local cache under `target/` or an operator-owned
  cache path, not in git unless redistributability is proven.

## Harness Requirements

The harness must:

1. Run each site fixture through `openwepp-cli-hill` from the current tree.
2. Extract modeled `frdp` from WAT output, converting WAT millimeters to meters.
3. Extract or add a modeled snow-depth diagnostic sufficient for
   `TOL-SNOWFREEZE-009`; `Snow-Water` alone is not enough.
4. Align modeled daily rows to observation dates after burn-in.
5. Apply method-specific comparison:
   - frost tube: direct magnitude comparison to `frdp`;
   - soil-temperature isotherm: onset/thaw/duration and magnitude upper-bound
     only;
   - penetrometer/mechanical resistance: secondary, non-authoritative unless
     the contract is amended.
6. Honor censoring:
   - exclude left-censored onset observations from onset timing error;
   - exclude right-censored sensor-depth caps from magnitude error.
7. Compute metrics:
   - seasonal maximum residual;
   - observation-date depth series residuals;
   - onset date residual;
   - thaw date residual;
   - frozen-duration residual;
   - snow-depth residual and snow-control verdict;
   - per-site ADR-0017 verdict.
8. Emit machine-readable and human-readable reports under a run output
   directory, with source hashes and binary/runtime metadata.

The report must distinguish:

- `PASS`
- `HARNESS-SURFACE-MISMATCH`
- `OPENWEPP-DEFECTIVE`
- `UNRESOLVED`
- `SOURCE-BLOCKED`

## Intended Write Set

- `docs/work-packages/20260624-snowfreeze-observed-frost-depth-harness-001/**`
- `docs/work-packages/README.md`
- `tests/fixtures/snowfreeze_observed/**`
- `tools/snowfreeze_observed/**`
- `tests/integration/snowfreeze_observed_frost_depth_contract.rs`
- `crates/openwepp-runner/tests/**` only if the harness is implemented as a
  runner CLI contract test.
- `crates/openwepp-runner/src/**` only for a snow-depth diagnostic required by
  `INV-SNOWFREEZE-047`; this must be a diagnostic/publication surface, not a
  frost physics change.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md` only
  for contract-first clarification of observation schema, diagnostics, or
  tolerances discovered during harness implementation.

## Phase Plan

### Phase 0: Authority and Data-Governance Lock

- Read required instructions and authority docs.
- Record source access status in `artifacts/dataset-inventory.md`.
- Decide which raw/normalized files may be checked in.
- Define the normalized schema and provenance lock.
- If licensing prevents check-in, define cache and fetch workflow that keeps
  normal tests local-only.

Exit criteria:

- `artifacts/dataset-inventory.md` lists every pilot site with access status,
  license/terms, acquisition route, raw storage policy, and current blocker.
- `artifacts/observation-schema.md` defines columns, units, censoring, and
  method mapping.

### Phase 1: Acquisition and Normalization

- Implement reproducible acquisition for redistributable sources.
- Normalize source records into the schema.
- Add checksum/provenance manifests.
- Document any source-specific parsing assumptions.

Exit criteria:

- Each acquired source has normalized rows and provenance locks.
- Request-only or blocked sources have explicit `SOURCE-BLOCKED` records.
- Normalization tests prove units, date parsing, site mapping, and method role.

### Phase 2: Simulation Harness

- Build the openWEPP simulation harness over
  `tests/fixtures/snowfreeze_observed/`.
- Add or expose required modeled snow-depth diagnostic if absent.
- Extract modeled `frdp` and snow depth into a comparable daily series.
- Ensure network is not required for local harness tests.

Exit criteria:

- Harness runs at least one redistributable site end-to-end from fixture input
  to comparison report.
- Harness supports all five pilot site definitions, even when a site is
  `SOURCE-BLOCKED`.
- Focused tests enforce no use of legacy/compatibility frost output as the
  observation target.

### Phase 3: Observation Comparison Report

- Run the harness for every available pilot site.
- Produce site metrics and ADR-0017 verdicts.
- Separate snow-confounded cases from frost-model defects.
- Record initial tolerance calibration notes for hydrology-reviewer
  ratification.

Exit criteria:

- `artifacts/validation-report.md` records current openWEPP result summaries by
  site and method.
- `artifacts/model-defect-ledger.md` records only verdicts allowed by
  `INV-SNOWFREEZE-047`.
- Any proposed frost physics remediation is moved to a follow-up package with a
  concrete defect ID and authority path.

### Phase 4: Review, Verification, and Closure

- Run focused tests and required gates.
- Complete dual review and dual verification.
- Update line-count governance for touched Rust files.
- Close `COMPLETE-SNOWFREEZE-OBSERVED-HARNESS` or hold at a named blocker.

Exit criteria:

- Every package gate is `PASS`, `BLOCKED`, or `NOT RUN` with current evidence.
- Accepted review findings are fixed or dispositioned.
- Worker handoff names the next actionable frost-depth remediation package if
  defects are found.

## Required Gates

Focused gates:

- `cargo test --test snowfreeze_observed_frost_depth_contract`
- Harness command over at least one public redistributable site.
- Source/provenance schema validation command.

Rust/doc gates:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo deny check`
- `git diff --check`

External-authority/anti-evasion gates:

- `bash tools/release/check_authority_suite_antievasion.sh`
- `cargo test --test auth11_required_suite_obligation_guards_contract`

Package-specific evidence gates:

- Dataset inventory complete.
- Normalized corpus provenance locked.
- Network-free local test path available.
- Snow-depth insulation control implemented or explicitly `UNRESOLVED`.
- Observation method correspondence enforced.
- Censoring enforced.
- ADR-0017 verdict taxonomy enforced.

## Acceptance Criteria

The package may close complete only when:

- all redistributable pilot observations are acquired or reproducibly fetched;
- blocked/request-only sources are explicitly classified and do not break local
  tests;
- normalized observation data and provenance locks are present;
- the harness compares current openWEPP outputs to observations without using
  legacy/compatibility frost output as target;
- model-vs-observation reports separate frost-tube magnitude, soil-temperature
  timing/upper-bound, snow-confounded, censored, blocked, and unresolved cases;
- no production frost physics changes are hidden in the harness package;
- closure artifacts record current commands and results.

## Closure Disposition

Closed 2026-06-24 as `COMPLETE-SNOWFREEZE-OBSERVED-HARNESS`.

Delivered:

- public redistributable sources acquired or reproducibly fetched for USGS
  Sleepers River, NRCS SCAN Mandan, NSIDC GGD498 Morris, and USDA-ARS
  Reynolds Creek station 127;
- Dun-2010 request-only sources classified explicitly as `SOURCE-BLOCKED`;
- normalized observation corpus and provenance locks under
  `tests/fixtures/snowfreeze_observed/observations/`;
- offline validation and contract tests;
- a `compare` harness command that runs openWEPP direct-production executor by
  default, reads WAT `frdp`, aligns dates, and emits JSON/Markdown reports;
- method-specific metrics that keep frost-tube residuals, soil-temperature
  upper-bound checks, censoring exclusions, and seasonal timing summaries
  separate;
- direct end-to-end comparison attempts for all five acquired sites.

Known limitation carried forward:

- modeled snow depth is not yet exposed as a contract-approved diagnostic.
  Therefore site residuals are reported as `UNRESOLVED`, not
  `OPENWEPP-DEFECTIVE`, until `INV-SNOWFREEZE-047` snow-control criteria can be
  applied.
- direct-runtime storage-reconciliation guard failures currently block
  comparison reports for `site3_scan_mandan_nd` and `site4_ggd498_morris_mn`.
  They are recorded as `HARNESS-SURFACE-MISMATCH`, not observation-model
  defects.

Legitimate hold states:

- `HOLD-SNOWFREEZE-OBS-DATA-ACCESS`: a required public source cannot be
  acquired or legally stored and no reproducible fetch/cache path is available.
- `HOLD-SNOWFREEZE-OBS-SNOW-DEPTH-DIAGNOSTIC`: openWEPP cannot expose modeled
  snow depth needed for the contract snow-insulation gate without a
  contract/API decision.
- `HOLD-SNOWFREEZE-OBS-HARNESS-SURFACE`: WAT/publication rows cannot be aligned
  to observation dates without a missing output/runtime metadata contract.
- `HOLD-SNOWFREEZE-OBS-CONTRACT-RATIFICATION`: provisional tolerances or
  measurement correspondence are insufficient for verdict assignment and need
  hydrology-reviewer ratification.

Invalid terminal reasons:

- datasets are manually downloaded but not provenance-locked;
- harness requires live network during `cargo test`;
- only seasonal maximum is compared;
- soil-temperature isotherm is treated as direct frost-depth magnitude;
- snow-depth confound is ignored;
- compatibility frost output is used as the acceptance target;
- a model defect is claimed without ADR-0017 criteria.

## Security and Licensing

- Do not commit credentials, cookies, tokens, or authenticated download URLs.
- Do not commit raw data whose license or terms disallow redistribution.
- Preserve source attribution and license text in provenance records.
- Use deterministic checksums for downloaded raw files and normalized outputs.
- Network access belongs in explicit fetch commands, not normal tests.

## Review Requirements

Two independent reviews are required before closure:

1. Data-provenance and licensing review.
2. Harness/contract review covering measurement correspondence,
   snow-insulation control, censoring, ADR-0017 verdicts, and anti-tautology.

Findings must be dispositioned as `accepted`, `rejected`, `deferred`, or
`follow-up` in `artifacts/review-disposition.md`.
