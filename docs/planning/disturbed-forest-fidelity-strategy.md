# Disturbed-Forest Fidelity — Campaign Strategy

Status: **planning (scoping complete, pre-work-package)** — 2026-07-02.
Owner: Claude Code (strategy authoring). Evidence base: two `Explore` mapping
passes (wepppy disturbed producer; openWEPP `ksatadj`) + a direct read of the
wepppy validation artifact + operator scoping decisions. Evidence classes are
labelled inline (**Mapped:** subagent finding; **Ran:**/**Read:** direct;
**Decision:** operator).

## Purpose

Bring **disturbed** (burned / treated) forest, shrub, and grass landuse into
openWEPP as **first-class, contract-governed physics** — decoupled from the two
legacy workarounds it currently rides on (everything encoded as WEPP *cropland*;
frost disabled for non-ag to accommodate the forest conductivity model) — and
**validate it on the directional burn-ordering laws**, not on legacy magnitudes.

This campaign is the concrete instantiation of the "first-class `lanuse` mode /
management-file authority" foundation opened from
`docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
(Increment 1). That foundation also unblocks the deferred `SC-OFEROUTE-001`
routing activation gate and is the prerequisite for the canopy-phenology limbs.

## What the campaign delivers (scope)

1. A first-class openWEPP **forest/shrub/grass `lanuse` mode** with
   management-file authority, replacing the cropland-encoded workaround.
2. An **`openwepp-disturbed.json`** class→management map + openWEPP management
   files, and ingestion of the **authoritative** `(texture × class)` land-soil
   lookup as the single source of truth (fixing the drifted management scalars).
3. A **re-port of `ksatadj`** (the disturbed/burned-forest saturated-conductivity
   model) into the `direct_runtime` production lane under `INV-SUBHYD-032`,
   **frost-on**, augmented to produce sensible burned outputs on its own.
4. A **directional-ordering validation harness** (the openWEPP variant of the
   80-cell burn matrix) + a **burn-magnitude adjudication** of the physically
   impossible legacy peakflow.

Out of scope (later increments, same forest-landuse program): canopy phenology
leaf-off/leaf-on (the parent backlog), MOFE/multi-OFE disturbed routing, the
SH-validation limb.

## Grounding — the record this strategy rests on

### The producer (wepppy `nodb/mods/disturbed/`) — **Mapped**

Event-driven off the landuse/soil build. A **Soil Burn Severity raster** is
classified into 4 classes (unburned/low/moderate/high); combined with the
hillslope's **disturbed class** (forest / deciduous forest / mixed forest /
young forest / shrub / tall grass / short grass / treatments) it (A) remaps
landuse to burned management keys, (B) regenerates the per-hillslope soil, (C)
writes PMET params, and (D) pushes lookup plant/ini scalars onto the management,
overriding the `.man` template.

- **Authoritative lookup:** `wepppy/nodb/mods/disturbed/data/disturbed_land_soil_lookup.csv`
  (105 rows), keyed **`(stext, luse)`** = (texture ∈ {clay loam, loam, sand
  loam, silt loam} × disturbed class). 19 columns drive **soil**
  (`ki, kr, shcrit, avke, bd, ksflag, ksatadj, ksatfac, ksatrec, keffflag,
  lkeff`), **PMET** (`pmet_kcb, pmet_rawp`), and **plant** (`rdmax, xmxlai,
  decfct, dropfc`). Cover/roughness (`cancov, inrcov, rilcov, rrinit`) come from
  the `.man` template, **not** the lookup.
- **Class→management map:** `wepppy/wepp/management/data/disturbed.json` (NLCD
  code → `{DisturbedClass, ManagementFile, SoilFile}`). Management templates in
  `wepppy/wepp/management/data/UnDisturbed/`.
- **The two workarounds this campaign removes:**
  - Every disturbed vegetation `.man` is authored as WEPP **cropland**
    (`landuse=1`, perennial), with forest/shrub/grass physics encoded through
    cropland fields (`cancov`, `inrcov`, `rilcov`, `rrinit`, `rdmax`, `xmxlai`,
    `hmax`, `cuthgt`, `decfct`, `dropfc`, `extnct`); the `WeppWillSet` plant name
    marks them as runtime-overridden stubs.
  - Legacy disables frost for non-ag (`ksflag=0`) as a broad lever to
    accommodate `ksatadj` (see the decouple decision below).
- **Out-of-sync management files (the lookup wins):** the lookup's
  `rdmax`/`xmxlai` silently override the `.man`, so any `.man` that disagrees is
  dead text. Drifted: **Young_Forest** (rdmax 0.6→1, xmxlai 12→10),
  **Moderate_Severity_Fire** (xmxlai 3→4), **Shrub** (xmxlai 10→5),
  **Shrub_Low_Severity_Fire** (xmxlai 3→2), and the **grass-fire family** (flat
  lookup 0.4/5 vs severity-graded `.man`). The land-soil lookup is authoritative.

### `ksatadj` in openWEPP — **Mapped** (the headline finding)

`ksatadj` is **parsed and projected** (`DisturbedPolicy` V9002/9003/9005 flags
flow through `parsers/soil.rs` → `runtime_inputs/02_soil_slope.rs`) and
**contract-governed** (`SC-SUBHYD-001` `INV-SUBHYD-032`, hard-fail, source-intent
`avsat/(avpor·avcpm)` required; ADR-0024) — **but it is unimplemented in the
`direct_runtime` production lane and untested at runtime.** The kernel
(`02_ksat_adjustment.rs`) and the REFINTENT001 `sat_frac` fix were **deleted
2026-06-30** (commit `a381702b`) with the symbol-map lane; the direct lane never
got a port. Current WB14 conductivity uses base/frost conductivity only.

- **Reference (source-intent authority):** `wepp-forest/src/infpar.for:606-647`,
  gated `if(ksatadj==1)`: `keff = ks·sat_frac^(2λ+3)` for `solwpv≥9002`, burn
  floor `keff = max(keff, lkeff)` for 9003, overwrites the plane Green-Ampt `ks`.
  It is a **WB/infiltration-side** adjustment.
- **Erosion coupling is indirect only:** `ksatadj` → infiltration → rainfall-
  excess `ie`/runoff/`peakro`/`watdur` → EROD13/14/15/19. There is **no
  conductivity term inside the erosion kernels** and no back-coupling. So
  "hillslope erosion ksatadj" reduces to: get the WB re-port right, then confirm
  the `ie`/runoff delta propagates into erosion.
- So the ask is a **re-port + augment + validate**, not a validate.

### The validation semantics — **Read** (`wepppy/tests/disturbed/analysis_results.md`)

An 80-cell matrix (4 textures × 5 veg × 4 burn severities; McKenzie Bridge OR,
100 yr, ~1194 mm/yr; soil format 9002 with hydrophobicity). Acceptance is
**directional / ordering**, not exact value: **burned total > matched unburned**
for runoff, sediment, and peakflow; burned>unburned in the majority of matched-
day events; texture/veg ordering. This fits the campaign-wide "test a law, not a
number" re-anchoring (ADR-0011).

## Operator decisions shaping the strategy — **Decision**

1. **Management-file authority; first-class `lanuse` modes** (committed
   `5de38eb6`): opt-in physics authority lives in the management file, not the
   `.run`; cropland-encoded forest/range fixtures are **compatibility inputs,
   not authority**; `row`/`ridge`/`rrinit` → Papanicolaou roughness inference is
   **disallowed** without a ratified bridge contract.
2. **Decouple frost from `ksatadj`.** Do **not** keep the legacy `ksflag=0`
   frost-off lever for non-ag. Keep **frost on (`ksflag=1`)** and **augment
   `ksatadj`** to produce sensible outputs independently. Because there is no
   live `ksatadj` in openWEPP, this is a clean re-port with frost-on from the
   start — no entrenched coupling to unwind.
3. **The legacy 190,000× burn peakflow is not a target.** `analysis_results.md`
   shows forest-high **burned mean peakflow 1446.7 m³/s** from a ~201 m
   hillslope — physically impossible (river-scale flood). The **direction**
   (burn ↑ peak) is the law to reproduce; the **magnitude** is a legacy
   peak-runoff-model artifact and must be adjudicated, not matched.
4. The **land-soil lookup is authoritative** over the drifted management files.

## Architectural principles

- **Contract-first (ADR-0011) / legacy-as-flag (ADR-0017):** re-derive disturbed
  physics into contracts; legacy is reference and directional flag, not a parity
  oracle.
- **Single source of truth:** the `(texture × class)` lookup is authoritative;
  the management template must not silently override it (the source of the §
  out-of-sync drift). Promote cover/roughness into the lookup if the native
  `lanuse` mode replaces the cropland stubs, so there is one table.
- **Fail-closed:** active disturbed physics fails closed until the management/
  lookup supplies the required operands; no silent inference from cropland
  fields without a ratified bridge contract.
- **Frost/ksatadj decoupled:** frost on, `ksatadj` sensible on its own.
- **Validate a law, not a number:** directional burn ordering is the acceptance;
  legacy magnitudes (esp. the 190,000×) are explicitly not targets.

## Workstreams

### WS-1 — Landuse-authority foundation

The first-class forest/shrub/grass `lanuse` mode + management-file authority +
`openwepp-disturbed.json` + openWEPP management files + authoritative-lookup
ingestion. Two-table split preserved (lookup for `(texture×class)` scalars;
`disturbed.json` for class→management). Reconcile the drifted files on import
(the lookup wins). **Ratification mechanism DECIDED (2026-07-02, Option A):**
[ADR-0034](../decisions/0034-management-file-lanuse-input-authority.md) records
the architectural decision, and a **standalone interface contract**
[`docs/contracts/openwepp-management-lanuse-authority-contract.md`](../contracts/openwepp-management-lanuse-authority-contract.md)
(skeleton authored; `LANUSE-AUTH-1..6` rules binding) governs the input authority
— NOT folded into a physics contract, because input structure/provenance is an
interface concern that routing/soil/canopy all consume. WS-1 populates the
concrete `lanuse` operand schema (schema ID `openwepp-management-lanuse-v1`).
**Blocks** WS-2 validation coverage of disturbed soils and the routing activation
gate.

### WS-2 — `ksatadj` re-port (SUBHYD)

Re-implement `INV-SUBHYD-032` source-intent (`avsat/(avpor·avcpm)`,
`keff = ks·sat_frac^(2λ+3)`, 9003 `lkeff` floor, 9001 exponential recovery) in
the `direct_runtime` WB14 effective-conductivity formation, **frost-on**,
**augmented** for sensible burned outputs. Re-create the non-aliased conformance
vectors that died with the old kernel (must cover `solwpv≥9002` and a case where
`Σst/Σul` differs from `avsat/(avpor·avcpm)`). **First verify** whether openWEPP
currently couples `ksflag=0 → frost off` for non-ag (residual coupling to
remove) or the frost-arc activation already decoupled it.

### WS-3 — Directional validation + magnitude adjudication

The openWEPP 80-cell harness (4 textures × 5 veg × 4 burn) asserting the burn-
ordering laws (burned>unburned runoff/sediment/peak; texture/veg ordering), plus
the **burn-magnitude adjudication** — locate where the legacy peak-runoff calc
diverges to river-scale under hydrophobicity and establish the sensible openWEPP
magnitude envelope. This is the acceptance gate for WS-1 + WS-2 together.

**Committed anchor fixture:**
`tests/fixtures/disturbed_burn/forest_high_severity_clay_loam/` — the exact
legacy inputs (`p4.*`, forest × high-severity × clay loam from the wepppy
disturbed matrix) behind the `~1446.7 m³/s` forest-high mean. Its `p4.sol`
carries all three campaign drivers at once: `ksatadj=1` (WS-2), `ksflag=0` (the
frost lever to remove), and the `keffflag=1`/`lkeff=0.1` hydrophobicity floor.
Legacy peaks reach **380,150 m³/s** from a 201 m hillslope (unburned baseline
~0.008) — the physical-impossibility anchor the adjudication resolves. Preserved
as-built (`ksflag=0`) as the artifact-reproducing input; see its `manifest.md`.

### WS-4 — Canopy phenology (later)

Leaf-off (frost/daylength) and leaf-on (spring thermal-time), from the parent
backlog, on the WS-1 foundation under a growth–canopy contract. Deferred until
WS-1–WS-3 land.

## Validation strategy

- **Primary (law):** the WS-3 directional matrix — burned>unburned ordering and
  texture/veg ordering across runoff, sediment, peakflow.
- **Kernel conformance:** WS-2 non-aliased `ksatadj` vectors against the
  source-intent (`INV-SUBHYD-032`), not against legacy magnitudes.
- **Magnitude sanity (guard):** peak discharge must be physically plausible for
  a hillslope of the stated area — the anchor fixture's legacy 380,150 m³/s peak
  (forest-high mean 1446.7) fails this and is the adjudication target
  (`tests/fixtures/disturbed_burn/forest_high_severity_clay_loam/`).
- **Frost-on regression:** with frost kept on, the disturbed matrix still
  reproduces the burn-ordering laws (proving the decouple is sound).
- **No production-default flip** without its own no-regression + magnitude gate.

## Contract touchpoints

- `SC-SUBHYD-001` / `INV-SUBHYD-032` / `BR-SUBHYD-KSATADJ-*` — the `ksatadj`
  re-port target (currently an implicit HOLD/GAP; the branch guards allow a
  governance HOLD "until the source-intent operand lineage is implemented").
- `ADR-0024` (reference-implementation-intent authority) — the source-intent
  basis for `ksatadj`.
- A new **management-input / `lanuse`-authority contract** (or `SC-OFEROUTE-001`
  activation extension) — the WS-1 input-authority ratification.
- `SC-OFEROUTE-001` — routing operands are among the first-class `lanuse`
  parameters WS-1 defines (shared with the deferred routing activation gate).
- A growth–canopy contract — WS-4 phenology (later).
- `ADR-0011` / `ADR-0017` — contract-first, legacy-as-flag posture throughout.

## Sequencing & dependencies

```
WS-1 (foundation) ──┬──> WS-2 (ksatadj re-port, needs disturbed soils)
                    └──> WS-3 (validation, needs WS-1 + WS-2)
WS-2 ───────────────────> WS-3
WS-3 ───────────────────> WS-4 (phenology) + routing activation gate
```

WS-1 is the opening work-package (ready now — extends the existing management
parser/runtime). WS-2 can begin in parallel on the kernel side (the source-intent
re-port is independent of the `lanuse`-mode plumbing) but its *validation* needs
WS-1's disturbed soils. WS-3 gates the pair. WS-4 and routing activation follow.

## Open questions / risks

1. **openWEPP's current `ksflag → frost` coupling** — does it honor `ksflag=0 →
   frost off`, or did the frost-arc activation already decouple? Verify before
   WS-2 (determines whether there is residual coupling to remove). — WS-2 entry.
2. **`ksatadj` augmentation** — "augment as needed for sensible outputs" is
   open-ended; the augmentation must stay contract-anchored (source-intent +
   physical-plausibility), not tuned to the legacy magnitudes.
3. **The 190,000× mechanism** — where exactly the legacy peak calc blows up
   under hydrophobicity, and what the sensible openWEPP envelope is.
4. **Grass-fire lookup is flat/uncalibrated** (0.4/5 across severities) while the
   `.man` files are severity-graded; the authoritative table itself needs
   per-severity calibration before `burn_grass` is enabled. Surface, don't
   silently inherit.
5. **Cover/roughness single-source** — whether to promote `cancov`/`inrcov`/
   `rilcov`/`rrinit` into the lookup (removing the drift class) or keep them in
   the native `lanuse` record; either way, one source of truth.
6. **Migration policy** — require explicit conversion of cropland-encoded
   fixtures to the native `lanuse` mode, or a temporary compatibility adapter
   that emits a manifest warning and refuses ambiguous operands (fail-closed).

## References

- **Producer:** `wepppy/nodb/mods/disturbed/disturbed.py`,
  `data/disturbed_land_soil_lookup.csv` (authoritative), `.../data/disturbed.json`,
  `wepppy/wepp/management/data/UnDisturbed/`.
- **Validation:** `wepppy/tests/disturbed/analysis_results.md`;
  anchor fixture `tests/fixtures/disturbed_burn/forest_high_severity_clay_loam/`
  (the forest-high peakflow artifact, `p4.*` + `manifest.md`).
- **ksatadj:** `wepp-forest/src/infpar.for:606-647` (source intent);
  `SC-SUBHYD-001.md` (`INV-SUBHYD-032`); `docs/decisions/0024-…`;
  deleted kernel history at commit `a381702b`; the REFINTENT001 work-packages
  `20260618-refimpl-intent-authority-ksatadj-subhyd-001` /
  `20260618-refintent001-ksatadj-satfrac-defect-closure-001`.
- **Foundation / parent:** `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
  (Increment-1 foundation, canopy phenology WS-4), `SC-OFEROUTE-001` (routing
  operands + activation gate), `ADR-0011` / `ADR-0017` / `ADR-0024`.
