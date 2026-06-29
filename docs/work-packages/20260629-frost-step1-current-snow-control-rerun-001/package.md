# FROST STEP 1 Current-Snow Control Rerun

Status: complete — `EXECUTED-COMPLETE-DIAGNOSTIC-ROUTING`.

Package type: diagnostic-only work package.

Objective: re-run the five-site observed frost harness on the current no-env
default snow and re-assess the `INV-SNOWFREEZE-048` snow-depth control gate per
site using the `INV-SNOWFREEZE-050` forcing-robust tiering.

Primary gap: `GAP-SNOWFREEZE-002`.

## Rationale

The snow program has reached its current floor, and Paradigm 2 Stage 2 showed
that replacing the bulk snow-to-frost insulation handoff with a layer-resistance
profile does not change frost fidelity. The prior frost attribution blocker must
therefore be reclassified: magnitude-only snow-depth residuals are forcing
uncertainty, while forcing-robust snow timing/regime/slope defects still block
frost attribution.

## Scope

Included:

- Run `tools/snowfreeze_observed/observed_harness.py compare` for all five
  frost observation sites through `openwepp-cli-hill` on the current no-env
  direct-production default.
- Extract modeled `frdp`, modeled WAT `Snow-Depth`, snow-control residuals, and
  seasonal frost timing summaries from the harness comparison reports.
- Route each site as `PASS`, `FORCING-LIMITED`, `BLOCKED`, or
  `INCONCLUSIVE-NO-PAIRED-SNOW` for Step 2 readiness.
- Record per-site attribution scope: frost timing signatures versus frost
  magnitude.
- Update package-local `GAP-SNOWFREEZE-002` disposition and package catalog.

Excluded:

- No snow-model or frost-model changes.
- No contract ratification or physics amendment.
- No default, output-schema, fixture, or public selector change.
- No frost-magnitude attribution; that is Step 2 after this routing.

## Required Reading

- `docs/planning/snow-frost-fidelity-strategy.md` section 11.
- `docs/work-packages/20260624-snowfreeze-observed-frost-depth-harness-001/`.
- `docs/work-packages/20260625-snowfrost-fidelity-e-snow-depth-fidelity-adjudication-001/`.
- `docs/work-packages/20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/`.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-047`, `INV-SNOWFREEZE-048`, `INV-SNOWFREEZE-050`, and
  `GAP-SNOWFREEZE-002`.
- `tests/fixtures/snowfreeze_observed/`.

## Intended Write Set

- `docs/work-packages/20260629-frost-step1-current-snow-control-rerun-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`

## Execution Plan

1. Confirm the current default snow selectors from the contract/strategy record.
2. Build or reuse the current `openwepp-cli-hill` binary.
3. Run all five observed frost comparison fixtures.
4. Run the package-local forcing-robust routing analyzer.
5. Record routing, `GAP-SNOWFREEZE-002` disposition input, review,
   verification, and line-count governance artifacts.
6. Run scoped markdown lint/validate and any focused harness validations.

## Exit Criteria

- Five current-default comparison reports exist, or any failed run is recorded
  as a harness/runtime blocker with command evidence.
- Per-site routing table distinguishes paired snow-control sites from sites
  without paired snow depth.
- Paired snow-depth residuals are classified as magnitude-only forcing-limited
  versus forcing-robust blocker using current evidence; no forced verdicts.
- Step 2 frost-magnitude attribution eligibility is named per site.
- `GAP-SNOWFREEZE-002` remains open or is narrowed truthfully; this package does
  not close the gap.
- Markdown validation passes for touched documentation.

## Disposition

The current no-env default snow harness rerun completed for all five sites.
The legacy scalar snow-depth audit still reports three paired snow-control
failures and two sites with no paired snow-depth rows. Applying
`INV-SNOWFREEZE-050` forcing-robust tiering narrows the route:

- `site1_sleepers_south_field_vt`: `FORCING-LIMITED`; frost timing signatures
  are attributable, while frost magnitude carries snow-depth forcing
  uncertainty into Step 2.
- `site2_sleepers_w9_hardwood_vt`: `FORCING-LIMITED`; same attribution scope as
  site 1.
- `site4_ggd498_morris_mn`: `BLOCKED`; systematic snow-cover timing/regime
  mismatch remains, so frost attribution would alias snow error.
- `site3_scan_mandan_nd` and `site5_reynolds_creek_us_rls_id`:
  `INCONCLUSIVE-NO-PAIRED-SNOW`; snow control cannot be established from the
  current corpus.

`GAP-SNOWFREEZE-002` remains open and narrowed. Step 2 may proceed only for the
two Sleepers sites, with magnitude uncertainty carried explicitly; Morris needs
a snow-control blocker disposition before frost attribution.

## Security / Production Impact

Diagnostic-only. The package changes documentation and package-local analysis
artifacts only. It does not change runtime code, schemas, fixtures, defaults,
or external data acquisition behavior.
