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
