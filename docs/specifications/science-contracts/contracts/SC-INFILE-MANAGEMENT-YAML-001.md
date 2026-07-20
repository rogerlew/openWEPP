---
contract_id: SC-INFILE-MANAGEMENT-YAML-001
title: Canonical Management YAML Input Parser Contract
status: in_review
maturity: draft
owner: openWEPP
contract_version: 0.1.0
evidence_mode: Static + Ran
last_updated_utc: 2026-07-08T00:00:00Z
---

# SC-INFILE-MANAGEMENT-YAML-001 Canonical Management YAML Input Parser Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static + Ran`

## Evidence Anchors

- `[DIRECT][E-SPEC-MAN-YAML-01]` `docs/specifications/wepp-input-files/specs/management-yaml.spec.md`.
- `[DIRECT][E-LANUSE-AUTH-01]` `docs/contracts/openwepp-management-lanuse-authority-contract.md`.
- `[DIRECT][E-SC-MAN-01]` `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`.
- `[DIRECT][E-SC-OFEROUTE-01]` `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- `[DIRECT][E-RUST-SCHEMA-01]` `crates/openwepp-management-schema/src/lib.rs`.
- `[DIRECT][E-RUST-ADAPTER-01]` `crates/openwepp-input-contract/src/parsers/management.rs`.
- `[DIRECT][E-RAN-01]` `cargo test -p openwepp-management-schema`.
- `[DIRECT][E-RAN-02]` `cargo test --test infile_management_yaml_contract`.

## 1. Scope and Applicability

This contract governs surface `infile-management-yaml`, the canonical typed
openWEPP management YAML document for `ow-lanuse-1` and later native management
datvers.

Flat `.man` parsing remains governed by `SC-INFILE-MANAGEMENT-001`. Flat
`ow-lanuse-1` `.man` is a source/compatibility bridge and migration input; it
is not the preferred native producer output once YAML is available. YAML is the
management input document referenced by the runfile, not an optional sidecar.

## 2. Format Identity

Every accepted document MUST carry:

```yaml
format: openwepp-management-yaml
schema_version: 1
datver: ow-lanuse-1
```

Unknown `format`, unsupported `schema_version`, and unsupported `datver` fail
closed before runtime projection. Schema version `1` is the initial implemented
schema. Future `ow-lanuse-N` or schema versions require an explicit migrator or
contract amendment before consumption.

## 3. Extension Policy

Producer policy:

- openWEPP producers emit lowercase `.yaml` as the terminal extension.
- Migrated flat sources default to appending `.yaml` to the full source
  filename, for example `field.man` to `field.man.yaml`.
- The `.man` portion of `.man.yaml` is informal provenance, not a consumer
  requirement.
- Producers do not emit `.yml`, `.YML`, or `.YAML`.

Consumer policy:

- Consumers accept terminal `.yaml`, `.YAML`, `.yml`, and `.YML` for dispatch.
- Extension acceptance is not schema acceptance; the document still must pass
  identity, schema, datver, and operand validation.
- Extensionless YAML is not content-sniffed by this revision.

## 4. Typed Schema Obligations

The typed schema MUST contain:

- top-level identity (`format`, `schema_version`, `datver`);
- topology (`nofes`, `total_years`);
- management metadata and optional provenance;
- typed plant scenarios;
- typed operation, initial-condition, surface-effect, contour, drainage, and
  yearly scenarios where present;
- expanded management schedule slots with OFE, rotation, year, and yearly
  references;
- explicit native landuse variants rather than legacy-sentinel ambiguity.

The parser is strict by default: unknown fields, missing required fields,
invalid references, invalid counts, and unsupported schema/domain values fail
closed with typed errors.

## 5. Route Coefficient Authority

For native YAML, every native forest or native cropland plant scenario MUST
carry `routing_coefficients` when it is intended for Lane D/default-active
production. The implemented schema requires the object for all native YAML plant
scenarios so a canonical YAML document cannot silently depend on sidecars or
legacy-field inference.

The required object carries:

- `k_o`;
- `form_c_d`;
- `d_r_m`;
- `lambda`;
- `vegetation_c_d`;
- `authority.source`;
- `authority.version`;
- `authority.checksum`;
- `authority.disturbed_class`.

`k_o` is positive finite. Form `C_d`, `d_r_m`, and vegetation `C_d` are finite
and non-negative. `lambda` is finite and in `0..=1`. Authority fields are
non-empty. The schema does not infer these values from row width, ridge spacing,
random roughness, cover, residue, canopy, reports, or diagnostics.

## 6. Runtime Consumer Obligation

The real hillslope runtime intake MUST dispatch YAML management paths through
the shared typed schema and convert the validated YAML into the normalized
`ManagementParseOutput` consumed by the existing PL runtime projection. The PL
projection MUST receive the same route coefficient values as if a flat
`ow-lanuse-1` management bridge carried the marked route-coefficient extension.

Producer-only YAML emission, migration reports, optional sidecars, or retained
source `.man` files cannot close runtime eligibility.

## 7. Invariants

| Invariant ID | Statement | Guard | Failure posture | Evidence |
|---|---|---|---|---|
| `INV-MANAGEMENT-YAML-001` | Document identity is explicit and exact: `format=openwepp-management-yaml`, `schema_version=1`, `datver=ow-lanuse-1`. | schema parser | Typed validation failure. | `E-SPEC-MAN-YAML-01`, `E-RUST-SCHEMA-01`, `E-RAN-01` |
| `INV-MANAGEMENT-YAML-002` | Producers emit lowercase `.yaml`; consumers accept `.yaml/.YAML/.yml/.YML` only as dispatch extensions. | schema extension helpers | Producer validation failure or consumer dispatch rejection. | `E-RUST-SCHEMA-01`, `E-RAN-01` |
| `INV-MANAGEMENT-YAML-003` | Native YAML carries typed section registries and schedule references; malformed counts/references fail closed. | schema validator + adapter | Typed validation failure; no runtime projection. | `E-RUST-SCHEMA-01`, `E-RUST-ADAPTER-01` |
| `INV-MANAGEMENT-YAML-004` | Native route coefficients are explicit typed operands with provenance, finite non-negative values, and no sidecar/legacy-field inference. | schema validator | Missing or invalid route coefficient object fails closed. | `E-LANUSE-AUTH-01`, `E-SC-OFEROUTE-01`, `E-RAN-01` |
| `INV-MANAGEMENT-YAML-005` | Hillslope runtime intake reads YAML directly and projects route coefficients into the existing PL schedule surfaces. | integration test | Missing PL route symbols or wrong values fail the test. | `E-RUST-ADAPTER-01`, `E-RAN-02` |

## 8. Guard Mapping

| Guard | Runtime / Test Surface | Required Behavior |
|---|---|---|
| `G-MAN-YAML-001` | `openwepp-management-schema` identity validation | Reject unknown format/schema/datver. |
| `G-MAN-YAML-002` | extension helpers | Producer accepts only `.yaml`; consumer accepts `.yaml/.YAML/.yml/.YML`. |
| `G-MAN-YAML-003` | route coefficient validator | Reject missing, partial, negative, non-finite, or provenance-less route coefficients. |
| `G-MAN-YAML-004` | input-contract adapter | Convert YAML schedule indices and native landuse variants into `ManagementParseOutput` without reading a source `.man`. |
| `G-MAN-YAML-005` | hillslope runner intake | Dispatch YAML management paths through `parse_management_document_from_path`. |
| `G-MAN-YAML-006` | PL projection integration test | Prove `ofeN_route_*` and slotted route coefficient symbols originate from YAML. |

## 9. Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-08` | `0.1.0` | `Codex` | Initial canonical management YAML input contract: identity, extension policy, typed schema, route-coefficient authority, input-contract adapter, and runtime PL projection proof. |
