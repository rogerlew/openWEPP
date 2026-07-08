# Projection Rule Design

Status: `HELD`.
Evidence class: Static.

## Decision

No deterministic legacy-cropland projection rule is accepted in this package.

## Accepted Authority Classes

- Explicit native `routing_coefficients` in `ow-lanuse-1` native forest or
  native cropland records.
- Authorized explicit producers that emit the same five route fields with
  provenance under the management-lanuse authority contract.

## Rejected Authority Classes

- Mapping `rrc`, `rrough`, `rrinit`, row width, rill spacing, ridge spacing, or
  random roughness directly to `D_r_m` or `lambda`.
- Inverting `frcsol`, `frctrl`, `frcteq`, `inrfto`, `frlive`, or cover-derived
  aggregate friction into `k_o`, form `C_d`, `D_r_m`, `lambda`, or vegetation
  `C_d`.
- Using canopy cover, residue cover, rill/interrill cover, or erosion delivery
  ratios as roughness concentration or drag coefficients.
- Selecting class defaults without an explicit, ratified coefficient table.

## Default Eligibility Rule

The rev-46 all/none/mixed rule remains binding, with rev-48 authority wording:

- every scheduled lane has complete source-authorized static route coefficients:
  active Lane D may attach by default;
- no scheduled lane has coefficients: protected legacy/off path remains;
- partial/mixed coefficient authority: fail closed before streaming;
- attempted legacy-field synthesis: fail closed unless a later bridge contract
  ratifies all five operands.

## Future Bridge Requirements

A future bridge may reopen this only if it supplies:

- all five static Lane D operands;
- source lines or table provenance for every operand;
- units and finite domain bounds;
- fail-closed behavior for missing, mixed, or out-of-domain values;
- manifest/source labels distinguishing explicit, table, and projected origins;
- tests proving the real active consumer reads the bridge output;
- predeclared multi-case fidelity acceptance before any coefficient tuning.
