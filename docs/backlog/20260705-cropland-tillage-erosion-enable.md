# Cropland/Tillage Erosion Enable (the ag-scaffolding erosion arm)

## Status
- `state`: backlog (concept) — the collection point for every LABELED
  cropland/tillage extension the erosion port deliberately deferred
- `maturity`: concept; each piece is individually source-anchored
- `default_path`: not eligible until a cropland/agricultural validation
  driver exists (per the project identity: forest hot path prioritized,
  ag scaffolded — see README "Scientific orientation")
- `evidence_mode`: Static (the deferral labels in SC-SED-001 /
  SC-RESIDUE-001 and the Wave-1 enable scope)

## What this is

The Wave-1 erosion engine enables on the NO-TILLAGE scope
(`wave1_operand_seed.enabled = !management_has_active_tillage`;
`multi_ofe_wave1_chained` keys on the same predicate). Every
cropland-specific operand the port encountered was deferred behind that
gate with an explicit label rather than silently approximated. This
entry collects them so the cropland enable is ONE coherent future unit
instead of scattered flags:

1. **Tilled-seed operand sourcing** (the enable itself): the Wave-1
   operand seed hardcodes non-cropland/non-tilled operands
   (consolidation `daydis` resets at tillage, tillage-driven roughness
   decay, the `bdtill` density line). Sourcing these is the gate-lift.
2. **Cropland `fidel` (interrill-detached composition)**: non-cropland
   `fidel = frac` is exact on the current scope (`param.for:452-458`);
   cropland derives `fidel` from the per-class interrill delivery
   `drinti` (`param.for:412-450`) — the labeled extension point in
   `Wave1EnrichmentInputs`.
3. **`strcov` (standing-mat cover term)** — the E.5-era ledger item:
   `covcal.for:160-176` adds `strcov = rmagt/srmhav·basmat` to both
   ground covers. Requires a standing-residue pool (`rmagt`), harvest
   mass (`srmhav`), and basal fraction (`basmat`) — none modeled today
   (`SC-RESIDUE-001#INV-RESIDUE-020` labels the absence; additive-only,
   so the current omission is conservative). Standing stubble after
   harvest is where legacy leans on this — a cropland-tier term.
4. **Cut-action pool topology**: the runtime's Cut moves surface→root
   with the ground-pool addition mapped from the surface transfer
   (`INV-RESIDUE-020` labeled mapping); legacy cuts STANDING mat into
   the flat/ground pools (`decomp.for:685-693`). A standing pool (item
   3) makes the source-true topology representable.
5. **Random-roughness daily decay**: the seed carries static `rrinit`
   (the 1b-A "recorded follow-up"); cropland fidelity needs the
   rainfall-driven decay + tillage reset.

## Promotion criteria

- A cropland/agricultural validation driver (fixture set + authority)
  enters scope, OR the DFF program needs tilled treatments (e.g.
  post-fire salvage/mechanical disturbance modeled as tillage).
- Then: promote as ONE work package — the enable (1) is the gate, items
  2–5 are its operand-fidelity companions; entry recon = the tillage
  subfactor chain (`soil.for` tillage branches) + `resup.for` standing
  dynamics.

## Provenance

- E.1–E.5 + the ground-cover closure WP (2026-07-04/05): each deferral
  labeled at its site (SC-SED-001 rev 44/46/51, SC-RESIDUE-001 rev 13).
- The irrigation entry
  ([20260617-irrigation-management-gated-activation.md](20260617-irrigation-management-gated-activation.md))
  is the sibling ag-activation pattern.
