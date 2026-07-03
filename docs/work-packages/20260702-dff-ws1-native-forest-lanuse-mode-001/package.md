# DFF-WS1 — Native Forest `lanuse` Mode Foundation

Status: **INCREMENT 1 (schema design) — Codex-reviewed, revisions incorporated;
INCREMENT 2 (parser/runtime code) SCOPED** (2026-07-02). See
`artifacts/review-codex.md` — 5 findings dispositioned (2 High: seam claim
corrected + Tier-A physics authority requirement; 2 Medium: all-section carve
policy + soil/mgmt reconciliation; 1 Low: rangeland-shaped wording).
Campaign: [disturbed-forest-fidelity](../../planning/disturbed-forest-fidelity-strategy.md)
WS-1. Governing authority:
[ADR-0034](../../decisions/0034-management-file-lanuse-input-authority.md) +
[`openwepp-management-lanuse-authority-contract.md`](../../contracts/openwepp-management-lanuse-authority-contract.md).
Management-file authority: `plant-file.spec.md` /
`SC-INFILE-MANAGEMENT-001`. Owner: Claude Code.

## Objective

Give openWEPP a **first-class, openWEPP-native forest `lanuse` mode** so that
forest / shrub / grass landuse no longer has to masquerade as WEPP cropland
(`landuse=1`). Populate the `openwepp-management-lanuse-v1` schema under the
ADR-0034 authority contract, and ingest the authoritative `(texture × class)`
land-soil parameterization as the single source of truth.

## The blocker, and the resolved design (operator, 2026-07-02)

**Blocker (grounded):** the legacy WEPP `.man` **forest block (`iplant=3`) is
empty** — "plants for Forestland not yet supported" across every section
(plant/operation/initial/surface/yearly; `plant-file.spec.md:190-194,…`). That
is *why* the disturbed producer encodes forest as cropland. The **rangeland
block (`iplant=2`)** has a populated *grammar* (grass/shrub/**tree** field
surface + `tempmn` cold-onset), but **rangeland was abandoned in legacy WEPP —
the model was never finished or validated**, and **openWEPP intentionally
rejected it from the start** (`MAN-E-004` is a deliberate scope decision, not an
incomplete port). So the rangeland parameters are **not proven authority**.

**Resolved design:** **start the schema *structure* from the rangeland grammar**
— use its grass/shrub/tree/cold-onset field surface as a **structural reference**
(legacy-as-flag, ADR-0017), **not** as validated physics — **and carve
openWEPP's own native `lanuse` mode** with contract-first-derived parameters and
physics (ADR-0011). Do **not** revive or inherit the abandoned legacy rangeland
model; the operand *values* come from the authoritative lookup and openWEPP's own
derivation, and the growth physics is authored under a contract (WS-4).

## Carve mechanism (openWEPP-native, from the soil precedent)

The soil parser already implements a native-extension **carve pattern** the
management parser lacks — openWEPP-native datver codes
(`SoilDatver::V7778/V9002/…`, `soil.rs:101`) that unlock **extra typed rows**
(`DisturbedPolicy`). It transfers as a *gated native-extension pattern*, not an
exact parser shape (the management parser's section structure + all-section
landuse gates differ). Mirror the pattern on the management side:

1. **New openWEPP-native management datver** added to `ALLOWED_DATVERS`
   (`management.rs:9`, currently the four legacy versions only) + a new
   `DatverFamily` arm.
2. **New `PlantScenarioData::Forest(PlantForestData)` variant** (and the
   parallel `InitialScenarioData::Forest`) — the `*ScenarioData` enums are
   already single-variant `Cropland(...)` (`management.rs:41-44`), purpose-built
   for this extension.
3. The variant is reached **only** under the native datver + a native forest
   `lanuse` sentinel; the legacy `iplant=2/3` rejects (`MAN-E-004`) stay for
   legacy inputs (compatibility quarantine, `LANUSE-AUTH-4`).

## Integration seam (kernel symbol-compatible — NOT a whole-seam free win)

**Corrected after Codex review (High).** The *daily growth kernel*
(`direct_runtime/growth.rs`) is **symbol-oriented** — it consumes a
boundary-symbol surface, so it needs **no** landuse-specific change. But the
*runtime projection path that builds that surface is cropland-specific* and
must be extended: yearly projection rejects `landuse != 1` then destructures
cropland data (`runtime_inputs/01_management.rs:635`), growth projection only
accepts `PlantCroplandData` (`05_projection_helpers.rs:158`), and initial
canopy seeding pulls cropland plant data (`01_management.rs:259`). So the win is
narrower: **emit the same growth symbols and the kernel is unchanged**, but a
forest *projection* (Plant + Initial + yearly schedule) must be added. Increment
1 maps the rangeland-shaped forest parameters onto those symbols (see
`artifacts/lanuse-v1-schema.md`); Increment 2 builds the forest projection path.

## Increment 1 — DELIVERED (design)

`artifacts/lanuse-v1-schema.md` — the concrete `openwepp-management-lanuse-v1`
schema: the native datver + `Forest` variant carve, the rangeland-**shaped** (structural-
reference-only) forest parameter set carved to openWEPP's needs, and the projection onto the growth-
surface boundary symbols. Plus:
- the `SC-INFILE-MANAGEMENT-001` amendment stub (un-sweep forest from the generic
  `MAN-E-004` reject; register the native forest branch);
- the authoritative-lookup ingestion plan (the `(texture × class)` operand values;
  `LANUSE-AUTH-6` single-source-of-truth) and the drifted-file reconciliation list.

## Increment 2 — SCOPED (parser/runtime code, next) — expanded after Codex review

The Rust build (scope widened per the review — the projection path and *all*
sections, not just Plant/Initial):
- **Parser (`parsers/management.rs`):** native datver in `ALLOWED_DATVERS` +
  `DatverFamily` arm; forest sentinel semantics; `PlantScenarioData::Forest` /
  `InitialScenarioData::Forest` + parse. **Section policy for the other
  sections** — operation / surface / contour / drain / yearly currently reject
  non-cropland (e.g. `management.rs:704`, `:1057`); define explicit **no-op /
  forest-supported** branches for each, plus blank-description-slot handling
  (`normalize_lines` `#landuse` markers) and tests.
- **Projection (`runtime_inputs/01_management.rs` + `05_projection_helpers.rs`):**
  add forest arms to the currently cropland-gated **yearly projection**
  (`:635`), **growth projection** (`05_projection_helpers.rs:158`), and
  **initial canopy seeding** (`:259`) — emit the growth symbols the kernel reads.
- **Tier-A physics authority (High):** do **not** run on unnamed "forest
  defaults." Either name a forest parameter authority, require explicit values,
  or ship a clearly-labeled **default-off / placeholder mode** with manifest
  warnings and **no fidelity claim**. Active growth symbols drive canopy / LAI /
  roots / ET / interception / runoff, so silent defaults would just rebrand the
  cropland masquerade.
- **Soil/management reconciliation (Medium):** a manifest requirement tying the
  `.man` class, the lookup row, `openwepp-disturbed.json`, and the `.sol`
  `DisturbedPolicy` together, **failing closed on mismatch** (not a deferred
  "may").
- Fail-closed per `LANUSE-AUTH-2`; parser-contract + projection tests; no
  daily-kernel change. Large; natural Codex handoff or a dedicated worktree.

## Later (not WS-1)

The plant-community **growth physics** (trees/shrubs/grasses + the `tempmn`
cold-driven decline) is WS-4 / the canopy-phenology increment. Increment 2
establishes the input surface; the growth model that fully consumes the
plant-community params beyond the shared growth symbols is downstream.

## Gates (Increment 1: docs)

- `git diff --check`; markdown-doc lint/validate.
- Increment 2 adds: fmt / clippy `-D warnings` / orchestrator + input-contract
  suites / SC lint / authority anti-evasion.

## Provenance

- Rangeland grammar as **structural reference** (abandoned legacy, not proven
  authority; ADR-0017): `plant-file.spec.md:155-190` (`iplant=2` block).
- Carve precedent: `soil.rs:101-171` (`SoilDatver` native datvers +
  `DisturbedPolicy`).
- Runtime seam: `runtime_inputs/05_projection_helpers.rs:211-355`,
  `direct_runtime/growth.rs`.
- Authoritative operands: `wepppy/nodb/mods/disturbed/data/disturbed_land_soil_lookup.csv`.
- Authority: `ADR-0034`, `openwepp-management-lanuse-authority-contract.md`
  (`LANUSE-AUTH-1..6`), `SC-INFILE-MANAGEMENT-001`.
