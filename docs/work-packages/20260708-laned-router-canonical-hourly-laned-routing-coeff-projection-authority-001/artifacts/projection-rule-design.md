# Projection Rule Design

Status: `HELD`.
Evidence class: Static.

## Decision

No deterministic legacy-cropland projection rule is accepted in this package.

## Accepted Authority Classes

- Explicit native `routing_coefficients` in `ow-lanuse-1` native forest or
  native cropland records.
- Authorized explicit producers that emit the same five route fields with
  provenance under the management-lanuse authority contract. Post-closure
  consensus narrows the preferred producer boundary: WEPPpy should materialize
  those fields into native `ow-lanuse-1` management files rather than rely on
  additional runfile sidecars.

## Rejected Authority Classes

- Mapping `rrc`, `rrough`, `rrinit`, row width, rill spacing, ridge spacing, or
  random roughness directly to `D_r_m` or `lambda`.
- Inverting `frcsol`, `frctrl`, `frcteq`, `inrfto`, `frlive`, or cover-derived
  aggregate friction into `k_o`, form `C_d`, `D_r_m`, `lambda`, or vegetation
  `C_d`.
- Using canopy cover, residue cover, rill/interrill cover, or erosion delivery
  ratios as roughness concentration or drag coefficients.
- Selecting class defaults without an explicit, ratified coefficient table.
- Treating legacy `lanuse=1` cropland by itself as coefficient-complete. Legacy
  datvers are compatibility inputs unless migrated to native `ow-lanuse-1`.
- Adding a separate sidecar whose absence silently changes the physics for the
  same legacy management file.

## Default Eligibility Rule

The rev-46 all/none/mixed rule remains binding, with rev-48 authority wording:

- every scheduled lane has complete source-authorized static route coefficients:
  active Lane D may attach by default;
- no scheduled lane has coefficients: protected legacy/off path remains;
- partial/mixed coefficient authority: fail closed before streaming;
- attempted legacy-field synthesis: fail closed unless a later bridge contract
  ratifies all five operands.

## Preferred Follow-On Route

Do not reopen this as another sidecar bridge by default. The preferred follow-on
is an `ow-lanuse-1` canonicalization/migration package that:

- declares `ow-lanuse-1` the production-authoritative datver for new physics;
- requires WEPPpy Disturbed/native producers to embed the five route
  coefficients directly in the `.man`;
- keeps legacy datvers on legacy/off single/MOFE driver paths unless explicitly
  converted;
- proves legacy datver runs cannot silently activate Lane D by missing sidecar
  accident;
- preserves all/none/mixed fail-closed behavior for native coefficient
  authority.

If a future bridge is still proposed, it must avoid optional sidecar semantics
and supply all five operands with provenance, bounds, consumer tests, manifest
labels, and predeclared multi-case fidelity acceptance.
