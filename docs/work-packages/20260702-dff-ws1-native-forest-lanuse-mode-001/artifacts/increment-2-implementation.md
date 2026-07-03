# DFF-WS1 Increment-2 — Native Forest `lanuse` Mode (parser/runtime build)

Status: **implemented on branch `dff-ws1-inc2-native-forest-lanuse`, gated, pushed
for Codex review** (2026-07-02). Author: Claude Code (WP run end-to-end under
explicit operator direction). Evidence classes labelled inline
(`Ran:` executional / `Static:` read-and-reasoned).

Governing authority: [ADR-0034](../../../decisions/0034-management-file-lanuse-input-authority.md),
[`openwepp-management-lanuse-authority-contract.md`](../../../contracts/openwepp-management-lanuse-authority-contract.md)
(`LANUSE-AUTH-1..6`), [`SC-INFILE-MANAGEMENT-001`](../../../specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md).
Design: [`lanuse-v1-schema.md`](lanuse-v1-schema.md) + [`review-codex.md`](review-codex.md).

## What was built (against the 5 reviewed Increment-2 items)

### 1. Parser — `crates/openwepp-input-contract/src/parsers/management.rs`
- **Native datver.** `OW_LANUSE_1_DATVER = "ow-lanuse-1"` added to
  `ALLOWED_DATVERS`; `DatverFamily::OwLanuse1` arm. Legacy option domains
  (operation pcode, resmgt, mgtopt) map to the `2016.3+` family via
  `legacy_option_family()`.
- **Forest sentinel.** `FOREST_LANUSE_SENTINEL = 3` (legacy forest code) selects
  the forest carve **only** under `ow-lanuse-1`; under legacy datvers `landuse=3`
  stays rejected (`MAN-E-004`, `LANUSE-AUTH-4` quarantine). Rangeland
  (`landuse=2`) stays rejected under every datver.
- **Scenario variants.** `PlantScenarioData::Forest(PlantForestData)`,
  `InitialScenarioData::Forest(InitialForestData)`,
  `YearlyScenarioData::Forest(YearlyForestData)`. `PlantForestData` groups
  Tier-A `growth` (the 19 kernel symbols) + `cf`/`diam` residue-seed operands +
  Tier-A `decomposition` (`oratea`/`orater`) + Tier-B `community`
  (`tempmn/gtemp/plive/wood` + grass/shrub/tree strata, rangeland-**shaped**
  structural reference, WS-4).
- **All-section policy.** `plant`/`initial`/`yearly` gain supported forest
  branches; `operation`/`surface`/`contour`/`drain` fail closed with the new
  typed `ForestSectionNotApplicable` (`MAN-E-004`) — forest defines no
  tillage/contour/drain, so no empty scenario is invented. Forest yearly slot
  enforces `tilseq=conset=drset=0`, `imngmt=2`, `mgtopt=3`.
- **Blank-slot handling.** `normalize_lines` `#landuse` keep-blank logic is
  landuse-agnostic and works unchanged for forest scenarios (test:
  `forest_scenario_blank_description_slots_preserved`).
- **Fail-closed presence.** Every Tier-A operand is an explicit fixed-arity
  numeric field; a missing/non-numeric value fails closed at parse (test:
  `forest_plant_block_missing_operand_fails_closed`).

### 2. Projection — `runtime_inputs/01_management.rs` + `05_projection_helpers.rs`
- **Confirmed the symbol-surface projection is still the production path**
  (`Static:` traced `build_hillslope_pl_runtime_surfaces_from_management` →
  `direct_production_typed_lane_seed_authority` → `DirectProductionGrowthCropAuthority`
  reads by string key → `DirectGrowthInputs` → daily kernel). The "seed-authority
  cutover" replaced the older monolithic writeback surface, not the PL surfaces.
- Forest arms at all three cropland-gated sites: yearly projection
  (`project_yearly_forest_slot`), growth-equation projection
  (`growth_equation_parameter_values_forest` → same 19 symbols), and initial
  canopy seeding (`build_initial_seed_projection_forest`). The daily kernel
  (`direct_runtime/growth.rs`) is **unchanged**.
- Behaviour-preserving refactor: `InitialSeedProjection` gained `imngmt/rtyp/iresd`
  so the seed-insert helpers are variant-agnostic; all 196 pre-existing
  input-contract + orchestrator tests still pass (`Ran:`).
- **`Ran:` end-to-end projection proof** (`forest_lanuse_projects_full_growth_symbol_surface`):
  a parsed forest `.man` projects the full 19-symbol growth surface + initial
  seeds with the correct operand values — it *runs*, and fails closed on any
  missing symbol.

### 3. Tier-A physics authority (High)
Resolved as **option (b): explicit, required, fail-closed values** — no unnamed
cropland/rangeland defaults. Lookup-owned operands (`xmxlai/rdmax/decfct/dropfc`)
carry the authoritative `forest high sev fire`/`clay loam` values
(xmxlai=2, rdmax=0.3, decfct=1, dropfc=1) reconciled by class; the remaining
Tier-A operands are the explicit forest authority in the `.man`.

### 4. Reconciliation manifest (Medium) — `runtime_inputs/08_forest_lanuse_reconciliation.rs`
`reconcile_forest_lanuse_authority(management, soil)` fails closed unless every
forest `.man` class is backed by a `.sol` `DisturbedPolicy` whose `luse` matches
(underscore/space-normalized). **Wired into the production seed path**
(`direct_production_typed_lane_seed_authority`, where both surfaces are present),
so it is a real consumer, not a shadow guard. Cropland-only managements are a
no-op (existing cropland-encoded disturbed fixtures do not regress). 4 unit tests.

### 5. SC amendment — `SC-INFILE-MANAGEMENT-001` v0.3.0
Registered the `ow-lanuse-1` datver (§1.2) + a new §1.4 native-forest branch;
un-swept the native forest branch from the generic `MAN-E-004` reject and added
`ForestSectionNotApplicable` / legacy-datver forest-sentinel to its trigger;
registered `MAN-GAP-004` (Tier-B WS-4 consumption) and `MAN-GAP-005`
(lookup/`disturbed.json` ingestion follow-on).

## Scope boundaries (explicit, for Codex disposition)
- **Growth physics is WS-4.** Increment-2 delivers the input surface + the
  shared-symbol projection so forest runs; the plant-community growth model that
  consumes Tier-B (`tempmn` cold-decline, trees/shrubs/grasses) is downstream
  (`MAN-GAP-004`).
- **Reconciliation is the `.man`↔`.sol` leg only.** The additional legs against
  the authoritative `(texture × class)` lookup table and `openwepp-disturbed.json`
  are **not** ingested in this increment (`MAN-GAP-005`). The forest `.man`
  carries the lookup-owned operands explicitly so they are reconcilable once the
  table is ingested; `stext`/texture stays soil-authoritative (`LANUSE-AUTH-3`).
  No orphan `openwepp-disturbed.json`/CSV assets were committed (would be
  unconsumed shadow data).
- **Fixture:** `tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man`
  is the native forest management example (forest-high-severity-fire, single OFE).
- **Compiler-forced touch of `openwepp-runner`** beyond the two named crates:
  adding the `Forest` enum variants made `active_management_crop_name` (PMET path)
  a refutable destructure; it now resolves `itype` for both variants. This is the
  reconciliation wiring site too.

## Open items for Codex review
- Section policy for forest in op/surface/contour/drain: chosen **fail-closed
  rejection** (`ForestSectionNotApplicable`) over accepting an empty no-op
  scenario. Confirm this reading of "no-op / forest-supported" is the intended one.
- `imngmt=2` / `mgtopt=3` are **required** for forest yearly slots (established
  perennial idle). Stand-replacing/thinning management is deferred to WS-4 —
  confirm the hard requirement vs. a warned default.
- Reconciliation requires a `DisturbedPolicy` for **any** forest `.man` (incl.
  unburned `forest`/`young forest`, which still carry a 9002 policy row in the
  wepppy-produced `.sol`). Confirm this is desired vs. allowing forest managements
  paired with non-disturbed soils.

## Codex review response (round 1 — all 4 findings addressed)

All findings verified against source and fixed on the same branch.

- **High — PMET compatibility fallback for native forest** (confirmed): a native
  forest management whose plant name misses the PMET sidecar could inherit the
  first-row fallback coefficients (compatibility mode always on). **Fix:**
  `project_typed_pmetpara_runtime` now forces **strict** PMET lookup when the
  active scenario is native forest (`active_management_is_native_forest`), so a
  miss fails closed (`LANUSE-AUTH-2`); cropland keeps its compatibility fallback.
  Tests: `native_forest_pmet_miss_fails_closed_no_first_row_fallback` +
  `cropland_pmet_miss_keeps_compatibility_first_row_fallback`.
- **Medium — `ow-lanuse-1` legacy option domain only on pcode** (confirmed):
  applied `legacy_option_family()` consistently so annual `resmgt`, perennial
  `mgtopt`, and permanent-contour parsing all treat `ow-lanuse-1` as `2016.3+`
  (matching the contract), not just operation `pcode`.
- **Medium — reconciliation not lane-scoped** (confirmed, latent for MOFE):
  `reconcile_forest_lanuse_authority` now derives forest classes from the
  schedule's active references (`active_forest_classes`: yearly `itype` + initial
  `iresd`), not the whole registry — so a lane's single soil policy no longer
  false-fails against unreferenced registry classes. Test:
  `reconciliation_is_scoped_to_scheduled_forest_class` (two registry classes, one
  scheduled).
- **Medium — docs overstate lookup reconciliation** (confirmed): softened
  `SC-INFILE-MANAGEMENT-001` §1.4 and the package objective to say the
  lookup-owned operands are authored explicitly in the `.man` (of record,
  fail-closed) and MUST equal the lookup row, with automated `.man`↔lookup
  ingestion as the `MAN-GAP-005` follow-on. The implemented reconciliation is the
  `.man`↔`.sol` leg.

Still recommended (Codex): one focused full-CLI forest `.man` + `.sol` run,
including a PMET-sidecar case, as end-to-end verification. The PMET no-fallback
behaviour now has unit coverage; the full-CLI run remains a follow-on.

## Codex review response (round 2 — both findings addressed)

- **Medium — PMET guard only checked the first scheduled slot:** the forest PMET
  discipline keyed off `active_yearly_scenario` (first slot / first yearly ref),
  so a mixed cropland-first/forest-later schedule could dodge it. **Fix:**
  replaced with `management_schedules_native_forest`, which scans **every**
  scheduled yearly ref — any forest scenario present ⇒ forest PMET discipline
  (fail closed on a lookup fallback). Test:
  `cropland_first_forest_later_schedule_applies_forest_pmet_discipline`
  (year-1 cropland `Corn` active, year-2 forest; sidecar miss ⇒ fail closed).
- **Low — stale package wording:** rewrote package.md's "Increment 2 — SCOPED …
  next" section to "DELIVERED", and corrected the reconciliation description to
  the implemented `.man`↔`.sol` leg with lookup/`openwepp-disturbed.json`
  ingestion deferred to `MAN-GAP-005`.

## Codex review response (round 3 — both findings addressed)

- **Medium — mixed schedule could apply a non-forest PMET hit to forest years:**
  the round-2 fix only rejected the *fallback*; a schedule with year-1 cropland
  (explicit PMET **hit**) + year-2 forest would still apply the single cropland
  `kcb/rawp` to the forest year. Since PMET is a single per-hillslope authority
  (schedule-aware/per-record PMET is a WS-4 change), the fix **rejects mixed
  cropland/forest schedules** outright at the PMET projection (fail closed,
  before the sidecar early-return). Added `management_schedules_cropland`; the
  mixed-schedule test now asserts the mixed rejection
  (`mixed_cropland_forest_schedule_is_rejected_by_single_pmet_surface`).
- **Medium — `git diff --check` failed** on the new fixture files (trailing
  whitespace / EOF blank lines inherited from the copied `cancov_forest`
  sources: `gwcoeff.txt`, `p2.cli`, `p2.slp`, `p2.sol`). Normalized trailing
  whitespace + collapsed EOF blank lines; token content unchanged and the CLI run
  was re-verified (exit 0). `git diff --check main..HEAD` is now clean.

## End-to-end CLI verification (`Ran:` — Codex's requested full run)

Built a native-forest run dir and ran it through the production hillslope CLI
(`openwepp-cli-hill`), fixture
`tests/fixtures/dff_ws1_native_forest/hjandrews_conifer_forest/` (derived from
`cancov_forest/hjandrews_conifer_or`: `.sol`/`.slp`/`.cli`/`pmetpara.txt`/etc.
copied verbatim; `p2.man` converted from the cropland masquerade to a native
`ow-lanuse-1` forest `.man`, `forest_class=forest`, plant name kept `Tah_4899`).
Committed test: `tests/integration/dff_ws1_native_forest_cli.rs`.

Result: **the native forest hillslope runs end-to-end** (exit 0, 45-yr run,
~31s). Manifest evidence: `scheduler_status_message_id:
R7C-DIRECT-PRODUCTION-EXECUTOR` (ran on the direct path where reconciliation +
PMET authority live — so the `.man` forest_class↔`.sol` `DisturbedPolicy`
reconciliation **passed**), explicit `Tah_4899` PMET record resolved (no fallback
warning), and `output/H2.hbp` + `H2.loss.json` + `H2.wat.parquet` produced.

**This run caught a real bug in the round-1 PMET fix** (exactly why the
end-to-end matters): forcing *strict* PMET mode for forest also changed the
query **normalization** to a different mode than the sidecar records were parsed
with (compat), producing a spurious miss on a record that exists. Corrected: run
the lookup in the configured mode (correct normalization) and, for native
forest, reject the result **only if it took the compatibility first-row
fallback**. A genuine forest PMET hit is unaffected; a genuine miss still fails
closed. Unit tests updated accordingly (`native_forest_pmet_miss_fails_closed_...`
now asserts the typed refusal, `cropland_..._keeps_...fallback` unchanged).

## Line-count governance (WARN, documented per crates/AGENTS.md)
The forest additions pushed two files across the 2000-line `WARN` threshold
(neither reaches the 3000-line `BLOCK`):
- `crates/openwepp-input-contract/src/parsers/management.rs` — 1705 → 2157.
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs` — 1686 → 2137.

**Rationale:** the forest parse (`parse_plant_forest` / `parse_initial_forest` /
`parse_yearly_forest`) and the forest projection (`project_yearly_forest_slot` /
`build_initial_seed_projection_forest` and the forest growth helpers) are
cohesive extensions that sit next to their cropland counterparts and share the
private section/symbol helpers, so inline keeps the diff reviewable and avoids a
premature module boundary.
**Follow-on split intent:** extract the forest parse block into a
`parsers/management/forest.rs` submodule and the forest projection into a
`runtime_inputs/09_forest_projection.rs` include once the WS-4 growth-physics
limb lands (which will add more forest-specific surface). Tracked as a
decomposition follow-on for the campaign.

## Gates (`Ran:`, from the worktree)
See the branch push message / the final report for the recorded gate results
(`cargo fmt --check`, `clippy --workspace --all-targets -D warnings`, per-crate +
full-workspace `nextest --profile full`, SC-unit lint, authority anti-evasion,
`cargo deny`).
