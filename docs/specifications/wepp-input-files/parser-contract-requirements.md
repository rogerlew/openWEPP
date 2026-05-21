# Parser Contract Data Model and Propagation Requirements

Status: Active
Last updated: 2026-05-21
Scope: `SC-INFILE-*` parser contracts

## Purpose

Define mandatory requirements for parser contracts so input-file handling covers:

1. syntax and version parsing,
2. typed data-model definition,
3. explicit propagation of parsed data through simulation state surfaces.

This prevents hidden mutation coupling and implicit state transfer patterns from
legacy common-block workflows.

## Normative Requirements

Each `SC-INFILE-*` contract must define all items below.

### 1) Two-Layer Model Contract

Contracts must separate:

1. `Source Model` (file-faithful parse representation)
2. `Simulation Model` (normalized typed representation consumed by kernels/orchestrators)

A contract must explicitly state which fields are preserved exactly from source,
and which are normalized or derived.

### 2) Field Specification Table (Required)

Every externally relevant field must include:

- canonical WEPP/wepp-forest symbol name,
- openWEPP alias/boundary name (if different),
- units,
- type,
- cardinality,
- required/optional status,
- datver applicability,
- defaulting/derivation rule.

### 3) Propagation Map Table (Required)

Every externally relevant field must include a propagation row from parse to
runtime usage.

Required columns:

1. source symbol,
2. parser model field,
3. runtime state field,
4. owning module,
5. simulation phase (`init`, `daily`, `event`, `annual`, `watershed`),
6. mutability (`immutable`, `mutable`),
7. downstream consumers,
8. guard/invariant IDs.

### 4) State Ownership and Mutation Rules

Contracts must define:

- which state is immutable after parse,
- which state is mutable during simulation,
- which module owns each mutable surface,
- forbidden mutation paths (for example cross-module ad-hoc mutation).

### 5) Derived Values and Closure Hooks

For derived values (for example layer aggregates, breakpoint expansions,
scenario indexes), contracts must specify:

- formula or algorithm source,
- when derivation occurs,
- closure/invariant checks tied to the derivation,
- tolerance or exactness expectations.

### 6) Validation and Error Taxonomy

Contracts must separate and type:

- syntax parse errors,
- semantic validation errors,
- cross-file consistency errors,
- runtime guard failures discovered post-parse.

No silent fallback/default masking for invalid required inputs.

### 7) Validation Surface Assignment (Required)

Contracts must classify each guard to one enforcement surface:

- parser-local surface (file-local parse function and model checks),
- downstream cross-validation surface (cross-file/run-context checks),
- runtime simulation guard surface (post-parse execution checks).

For each guard, contracts must specify:

- owning module/surface,
- failure taxonomy code(s),
- closure path used by verification gates.

Parser-local packages are not required to broaden parser function signatures
solely to satisfy run-context/cross-file guards when those guards are assigned
to a downstream cross-validation surface.

### 8) Cross-File Consistency Constraints

Contracts must define consistency checks across primary and sidecar surfaces
when fields are coupled (counts, IDs, structure expectations, and phase
compatibility flags).

At minimum this includes:

- `.run`, `.cli`, `.sol`, `.man`, `.slp`,
- watershed input files (for example `.str`, `.chn`, `.imp`),
- sidecars (for example irrigation sidecars, `pmetpara.txt`, `snow.txt`,
  `frost.txt`).

### 9) Backward Compatibility Requirements

Contracts must define backward compatibility policy for legacy text sidecars:

- accepted legacy variants,
- version gates,
- round-trip expectations,
- explicitly unsupported forms.

### 10) Boundary Export Requirements

Contracts must specify which parsed/derived fields cross process boundaries
(HBP, parquet, CLI args, and other interface surfaces) and how names/units map
across those boundaries.

### 11) Surface Registry Completeness (Program-Level Required)

The parser-contract program must maintain a complete input-surface registry
covering hillslope, watershed, and sidecar surfaces.

Canonical registry location:

- `docs/specifications/wepp-input-files/input-surface-registry.md`

Each registry entry must have one disposition:

- governed by an active `SC-INFILE-*` contract,
- explicitly deferred with rationale and risk note, or
- explicitly unsupported with typed error behavior and user-facing guidance.

## Minimum `SC-INFILE-*` Section Set

Each parser contract must include, at minimum:

1. Scope and version applicability.
2. Source grammar/record structure.
3. Field specification table.
4. Propagation map table.
5. State ownership and mutability rules.
6. Derived-value rules with closure hooks.
7. Validation/error taxonomy.
8. Cross-file consistency constraints.
9. Compatibility policy.
10. Guard map and invariant linkage.
11. Validation-surface assignment and ownership mapping.

## Relation to Other Governance

This document complements, and does not replace:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/README.md`
- `docs/architecture/README.md`

Parser contracts remain subject to dual-agent review, disposition, and
verification gates.
