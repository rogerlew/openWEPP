# `openwepp-management-lanuse-v1` — Forest `lanuse` Mode Schema (WS-1 design)

Design of the openWEPP-native forest `lanuse` mode under
[ADR-0034](../../../decisions/0034-management-file-lanuse-input-authority.md) +
[`openwepp-management-lanuse-authority-contract.md`](../../../contracts/openwepp-management-lanuse-authority-contract.md).
Increment-2 (Rust parser/runtime) implements this; the plant-community growth
physics that consumes the community params is WS-4.

## 1. Carve mechanism — native datver + `Forest` variant

Mirror the soil-parser precedent (`SoilDatver` native codes `7778/9002/9003/9005`
unlocking the extra `DisturbedPolicy` row):

- **Native management datver.** Add an openWEPP-native datver to
  `ALLOWED_DATVERS` (`management.rs:9`) + a new `DatverFamily` arm. Proposed:
  `ow-lanuse-1` (string, unambiguously openWEPP-native so no legacy `.man`
  collides). Increment-2 fixes the literal.
- **New scenario variants.** `PlantScenarioData::Forest(PlantForestData)` and
  `InitialScenarioData::Forest(InitialForestData)`, alongside the existing
  single-variant `Cropland(...)`. Reached **only** under the native datver + a
  native forest `lanuse` sentinel.
- **Legacy stays rejected.** Legacy `iplant=2` (rangeland) / `iplant=3` (empty
  forest) keep their `MAN-E-004` rejects; this is a *new* native mode, not a
  revival of the half-implemented legacy path (`LANUSE-AUTH-4` quarantine).
- **All sections, not just Plant/Initial (Codex review, Medium).** The parser
  rejects non-cropland landuse in **every** section (plant/operation/initial/
  surface/contour/drain/yearly, e.g. `management.rs:704`, `:1057`), so the carve
  must define an explicit forest **section policy** for each: forest-supported
  `Plant`/`Initial` variants + parsed forest **yearly** schedule, and explicit
  **no-op / forest-supported** branches for operation/surface/contour/drain
  (forest has no tillage/contour/drain), with blank-description-slot handling
  (`normalize_lines` `#landuse`) and per-section tests. Increment-2 scope.

## 2. `PlantForestData` — the forest parameter block (rangeland grammar as structural reference)

The rangeland (`iplant=2`) grammar is a **structural reference only** (what
fields a plant-community input needs) — **not** validated authority: rangeland
was abandoned in legacy WEPP and openWEPP intentionally rejects it. Field names/
shapes are borrowed as a starting structure; the parameter **values and physics
are openWEPP-native** (contract-first, ADR-0011), with operand values from the
authoritative lookup.

Two tiers. **Tier A** = the shared growth symbols the daily kernel already
consumes (must be present so the mode runs today). **Tier B** = plant-community
fields whose *structure* mirrors the rangeland grammar (cold-decline +
grass/shrub/tree), reserved for WS-4; parsed and stored now, but their values and
the model that reads them are **re-derived under a growth–canopy contract**, not
inherited from the abandoned rangeland model.

### Tier A — shared growth symbols (required; feed the existing seam)

Source column: **lookup** = the authoritative `(texture × class)` table (values
of record); **forest authority †** = to be resolved per the Tier-A
physics-authority requirement below (named authority / explicit values /
placeholder-no-fidelity) — **not** unnamed cropland or rangeland defaults.

| Forest field | Source | Growth symbol | Notes |
|---|---|---|---|
| `bb`, `bbb` | forest authority † | `bb`, `bbb` | canopy-cover/height coeffs (`cancov = 1-exp(-bb·biomass)`) |
| `xmxlai` | **lookup** (`xmxlai`) | `xmxlai` | max LAI (canopy density proxy) |
| `rdmax` | **lookup** (`rdmax`) | `rdmax` | max rooting depth |
| `gddmax`, `dlai` | forest authority † | `gddmax`, `dlai` | heat-unit growth (may be phenology-driven in WS-4) |
| `hmax` | forest authority † (or **lookup** if the table gains it) | `hmax` | max canopy height |
| `extnct`, `flivmx`, `hi`, `rsr`, `rtmmax`, `spriod`, `btemp`, `otemp`, `pltol` | forest authority † | same | remaining growth-eqn inputs |
| `decfct`, `dropfc` | **lookup** (`decfct`, `dropfc`) | `decfct`, `dropfc` | residue decomp / leaf-drop (deciduous 0.2, mixed 0.55) |
| `oratea`, `orater` | forest authority † | `oratea`, `orater` | decomposition-surface |

† resolved per the Tier-A physics-authority requirement below — never unnamed
cropland/rangeland defaults.

**Tier-A physics-authority requirement (Codex review, High).** These symbols
drive canopy, LAI, roots, ET, canopy interception, and runoff — so Increment 2
MUST NOT run forest mode on unnamed "forest defaults" (that would rebrand the
cropland masquerade with no fidelity gain). One of: (a) name a forest parameter
**authority** and source the values from it; (b) require **explicit** values in
the forest `lanuse` record (fail-closed if absent, `LANUSE-AUTH-2`); or (c) ship
a clearly-labeled **default-off / placeholder mode** that emits a manifest
warning and makes **no fidelity claim**. Rangeland numeric values MUST NOT be
used as defaults unless separately authorized (they are structure-only).

### Tier B — plant-community fields (rangeland grammar as structure; WS-4 derives + consumes)

Field *structure* borrowed from the `iplant=2` grammar (a reference, not proven
authority — see §2 preamble), **carved** to the forest-relevant subset; values
and physics re-derived under a growth–canopy contract at WS-4: `tempmn`
(senescence-onset temperature — the cold-decline driver),
`gtemp` (growth-onset temp), `plive` (max standing live biomass), `wood`
(woody-biomass fraction), and the structural components — grass
(`gcoeff/gdiam/ghgt/gpop`), shrub (`scoeff/sdiam/shgt/spop`), tree
(`tcoeff/tdiam/thgt/tpop`). Stored on `PlantForestData` now; the plant-community
canopy/decline model that reads them is WS-4. (Explicitly deferred: `aca/aleaf/
ar/bugs/cf1/cf2/cn/cold/ffp/pitol/pscday/rgcmin/root10/rootf/scday2` — carry-or-
drop decided at WS-4 when the growth model is designed.)

### `InitialForestData` — cover/roughness (promoted to first-class)

`cancov`, `inrcov`, `rilcov`, `rrinit` — today read from the cropland
`IniLoopCropland` template; promoted here to first-class forest fields
(`LANUSE-AUTH-6` single-source-of-truth). `rspace`/`rtyp` (ridge) are **not**
carried — forest has no ridge geometry, and `LANUSE-AUTH-3` forbids inferring
routing roughness from them.

## 3. Projection onto the growth-surface seam

**Scope corrected after Codex review (High).** The *daily kernel*
(`direct_runtime/growth.rs`) is symbol-oriented and unchanged. But the
*projection path is cropland-gated* and must gain forest arms — it is **not**
whole-seam landuse-agnostic:
- yearly projection rejects `landuse != 1` then destructures cropland
  (`01_management.rs:635`);
- growth projection accepts only `PlantCroplandData`
  (`05_projection_helpers.rs:158`);
- initial canopy seeding pulls cropland plant data (`01_management.rs:259`).

Increment 2 adds `growth_equation_parameter_values_forest(slot, PlantForestData)
-> [(&str,f64);19]` (same 19 symbols → kernel unchanged), a forest
`InitialForestData` seed projection (`cancov_seed`, `bbb_seed`, `hmax_seed`, …),
and a forest yearly-schedule arm. Tier-B params project to new forest-only
symbols (`tempmn`, tree/shrub/grass structure) no current kernel reads —
reserved for WS-4.

## 4. Soil/ksatadj linkage

The disturbed **soil** operands (`ki/kr/shcrit/avke/ksflag/ksatadj/ksatfac/
ksatrec/keffflag/lkeff`) live in the `.sol` `DisturbedPolicy` row (already
parsed), not in the `.man`. The forest `lanuse` mode does **not** duplicate them;
WS-2 re-ports `ksatadj` in SUBHYD. The management-side authority (this schema)
and the soil-side authority (`DisturbedPolicy`) share the `(texture × class)`
lookup as the single upstream source (`LANUSE-AUTH-6`). Today
`DisturbedPolicy.luse` is opaque free-text (`"forest use"`) decoupled from the
`.man` landuse. **Requirement (Codex review, Medium):** Increment 2 MUST add a
reconciliation manifest tying the `.man` forest class, the lookup row,
`openwepp-disturbed.json`, and the `.sol` `DisturbedPolicy` (class + texture +
burn) together and **fail closed on any mismatch** — not a deferred "may."

## 5. `SC-INFILE-MANAGEMENT-001` amendment (stub)

- **§7 MAN-E-004:** un-sweep forest from the generic "invalid enum domain"
  clause — the native forest `lanuse` mode is *supported* under the native
  datver; only *legacy* `iplant=2/3` remain rejected.
- **Field spec:** add the forest (`ow-lanuse-1`) parameter branch (Tier A + Tier
  B) alongside the documented cropland/rangeland branches.
- **§13 gap register:** register the WS-4 dependency (plant-community growth
  physics not yet consuming Tier-B params).

## 6. Authoritative-lookup ingestion + drift reconciliation

- Ingest `disturbed_land_soil_lookup.csv` (or an openWEPP equivalent) as the
  authoritative `(texture × class)` table; the `.man`/lanuse template must not
  override it (`LANUSE-AUTH-6`).
- **Drifted files to reconcile (lookup wins):** Young_Forest (rdmax 0.6→1,
  xmxlai 12→10), Moderate_Severity_Fire (xmxlai 3→4), Shrub (xmxlai 10→5),
  Shrub_Low (xmxlai 3→2), and the grass-fire family (flat lookup 0.4/5 vs graded
  `.man`) — the last also flags an **authority-table data-quality gap**
  (grass-fire lookup is uncalibrated across severities).

## 7. `openwepp-disturbed.json`

The class→management binding (openWEPP analogue of wepppy's `disturbed.json`)
points burn/disturbed classes to the new native forest management files. Authored
in Increment 2 alongside the management files, versioned with the `lanuse` schema.

## Open design items for Increment 2 / WS-4

- The literal native datver token and the forest `lanuse` sentinel.
- **Tier-A physics authority (High — a requirement, not just an open item):**
  resolve per §2 Tier-A — named forest authority, explicit values, or labeled
  placeholder-no-fidelity mode. Not implementable on unnamed defaults.
- Per-section forest policy (operation/surface/contour/drain/yearly no-op or
  supported branches) + blank-slot tests.
- Reconciliation manifest (`.man` class ↔ lookup ↔ `openwepp-disturbed.json` ↔
  `.sol` `DisturbedPolicy`), fail-closed on mismatch.
- Tier-B carry-or-drop final list (WS-4, when the growth model is designed).
- MOFE/multi-OFE cardinality (deferred per ADR-0034 scope).
