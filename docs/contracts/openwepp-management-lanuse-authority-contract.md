# openWEPP Management-File `lanuse` Authority Contract

Status: `draft-normative` (**skeleton** — ADR-0034 ratifies the authority model
and `LANUSE-AUTH-1..6`; the concrete `lanuse` operand schema is designed in the
disturbed-forest campaign WS-1 foundation work-package.)

Governing decision: [ADR-0034](../decisions/0034-management-file-lanuse-input-authority.md).
Campaign: [disturbed-forest-fidelity-strategy](../planning/disturbed-forest-fidelity-strategy.md).

This is an **interface contract** (input structure + provenance), not a science
contract. It governs *where* landuse-physics operands come from and that they are
present, typed, and authorized. The *physics* of what each operand does stays in
its domain science contract (`SC-*`), which references this contract for operand
provenance.

## Scope

- The **management-file `lanuse` block** as the authoritative source of opt-in,
  first-class landuse-physics operands (forest / shrub / grass / …).
- The **authority relationship** between the `.run` file, the management
  file/`lanuse` record, the authoritative `(texture × class)` land-soil
  parameterization lookup, and the consuming science contracts.
- The **provenance and fail-closed rules** (typed presence, no legacy-field
  inference, compatibility-input quarantine, reproducibility).

Out of scope:
- The physics of any operand (owned by the consuming `SC-*` contract).
- The legacy WEPP `.man` / soil / climate file formats (pinned compatibility
  formats; see the interface-contract registry).
- Default activation of any `lanuse`-mode physics (a separate later gate).

## Authority model

1. **Management file is authoritative** for opt-in landuse-physics operands. The
   `.run` file **points to** the management sidecar and is reproducibility
   metadata only; it carries no physics selectors and is not the sole record
   that a run used an enhanced path.
2. **`lanuse` selects a physical mode**, not a cropland masquerade. Each mode's
   landuse record carries its operands as first-class, typed parameters.
3. **The `(texture × class)` land-soil lookup is the single source of truth** for
   the parameters it owns; a management template must not silently override it.
   (In wepppy the authoritative table is
   `nodb/mods/disturbed/data/disturbed_land_soil_lookup.csv`; openWEPP ingests an
   equivalent authoritative table. Where a template and the lookup disagree, the
   lookup wins — see the campaign strategy's out-of-sync reconciliation.)
4. **Consuming science contracts reference this contract** for operand provenance
   and retain authority over the operand physics.

## Normative rules

- **`LANUSE-AUTH-1` — First-class modes.** `lanuse` resolves to a physical
  landuse mode. Forest/shrub/grass physics MUST NOT be authored as a cropland
  record for authority purposes.
- **`LANUSE-AUTH-2` — Typed presence / fail-closed.** For each active physics
  domain, the required operands MUST be present and typed in the `lanuse` record
  (or its authoritative lookup). Missing/untyped required operands are a
  **hard-fail**, not a silent default.
- **`LANUSE-AUTH-3` — No legacy-field inference.** New-physics operands MUST NOT
  be inferred from legacy cropland fields (row width, ridge spacing, `rrinit`,
  etc.) unless a separate **ratified bridge contract** defines the mapping.
- **`LANUSE-AUTH-4` — Compatibility-input quarantine.** Cropland-encoded
  forest/range fixtures are **compatibility inputs**. They may be read by an
  explicit adapter that emits a manifest warning and **refuses ambiguous**
  new-physics operands; they are never the authority for new landuse physics.
- **`LANUSE-AUTH-5` — Reproducibility from sidecars.** An activated
  `lanuse`-mode run MUST be reproducible from its sidecars alone: the
  management/`lanuse` record declares the physical mode and every operand the run
  used.
- **`LANUSE-AUTH-6` — Single source of truth.** No operand may have two
  disagreeing authorities. Where a management template and the authoritative
  lookup both carry a parameter, the lookup is authoritative and the template
  value MUST be reconciled or dropped.

## Operand surface (skeleton — populated at WS-1)

The `lanuse` record carries, per active physics domain, the operands the
consuming contract requires. Enumerated concretely in WS-1; the domains and their
consuming contracts:

| Domain | Consuming contract | Operand set (indicative) |
|---|---|---|
| Soil erodibility / conductivity / hydrophobicity | `SC-SUBHYD-001` / erosion | `ki, kr, shcrit, avke, bd, ksflag, ksatadj, ksatfac, ksatrec, keffflag, lkeff` |
| PMET | (PMET surface) | `pmet_kcb, pmet_rawp` |
| Plant / cover / roughness | growth–canopy contract | `rdmax, xmxlai, decfct, dropfc`, and (promoted from the `.man` template) `cancov, inrcov, rilcov, rrinit` |
| Overland-flow routing | `SC-OFEROUTE-001` | `k_o, C_d, D_r, lambda, LAI, h_c`, vegetation `C_d` |
| Canopy phenology (future) | growth–canopy contract | leaf-off frost/photoperiod controls, leaf-on thermal-time/chilling controls, evergreen floor, litter-pool params |

This table is indicative, not a ratified schema. `bd`, cover/roughness
promotion, routing/phenology sets, concrete key names, legacy-input migration
manifests, and single-OFE vs. MOFE/multi-OFE cardinality are WS-1 /
later-increment design items; the **rules above are already binding** on
whatever schema WS-1 lands. **WS-1 design (in progress):**
`docs/work-packages/20260702-dff-ws1-native-forest-lanuse-mode-001/artifacts/lanuse-v1-schema.md`
— the openWEPP-native-datver carve, rangeland-**shaped** (structural-reference-only) forest parameter set
(Tier-A shared growth symbols + Tier-B plant-community params), and the
growth-surface projection. If a legacy-field mapping is later authorized, its
mechanics belong in a separate ratified bridge contract, not in this authority
skeleton.

## Schema ID and versioning

- Schema ID: `openwepp-management-lanuse-v1` (assigned when the WS-1 schema
  lands; breaking changes require `v2`, additive keys stay in `v1`).
- The class→management binding map (`openwepp-disturbed.json`, the openWEPP
  analogue of wepppy's `disturbed.json`) is versioned alongside the `lanuse`
  schema.

## Relationship to other contracts

- **`SC-OFEROUTE-001`** — the Papanicolaou routing operands this contract carries
  satisfy `SC-OFEROUTE-001`'s activation-input requirement; that contract's
  `row`/`ridge`/`rrinit`-inference prohibition is the same `LANUSE-AUTH-3` rule.
- **`SC-SUBHYD-001` / `INV-SUBHYD-032`** — the `ksatadj`/soil operands this
  contract carries feed the `ksatadj` re-port; the *physics* stays in SUBHYD.
- **Growth–canopy contract (future)** — canopy phenology + litter operands.
- **Bridge contract (future, if authorized)** — the only sanctioned path for any
  legacy-field → new-physics operand mapping (`LANUSE-AUTH-3`).

## Revision history

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-02` | `0` (skeleton) | `Claude Code` | Initial skeleton under ADR-0034: authority model + `LANUSE-AUTH-1..6` normative rules + operand-surface stub. Concrete `lanuse` operand schema deferred to the disturbed-forest campaign WS-1 foundation work-package. |
