# Routine Interface Contract v1

Status: `draft-normative`

This contract defines the required interface and lifecycle metadata for
openWEPP routines.

## Scope

- Routine identity/version metadata
- Routine execution interface shape
- Lifecycle state behavior
- Replacement and selection semantics

This contract is engine-internal and does not replace external run-file or HBP
contracts.

## Identity and versioning

Each routine must declare:

- `routine_id` (stable identifier, never reused)
- `contract_id` (interface family identifier)
- `contract_version` (SemVer `MAJOR.MINOR.PATCH`)
- `impl_version` (implementation build/version identity)

Versioning rules:

1. `contract_version.major` changes for incompatible input/output or semantics.
2. `contract_version.minor` changes for additive-compatible behavior.
3. `contract_version.patch` changes for compatible fixes.

## Required descriptor fields

Each routine descriptor must include:

- `routine_id`
- `display_name`
- `lifecycle_state` (`experimental`, `active`, `deprecated`, `retired`)
- `contract_id`
- `contract_version`
- `impl_version`
- `input_schema_ref`
- `output_schema_ref`
- `units_manifest_ref`
- `capabilities` (list)
- `replaces` (optional routine reference)
- `replaced_by` (optional routine reference)

`units_manifest_ref` must resolve to explicit variable/unit definitions aligned
with module-level symbol glossaries.

## Lifecycle semantics

| State | Runnable | Default resolver candidate | Notes |
|---|---|---|---|
| `experimental` | yes | no | explicit opt-in required |
| `active` | yes | yes | default production state |
| `deprecated` | yes | no | warning required when selected |
| `retired` | no | no | metadata retained for audit/history |

## Execution interface

Routine implementations must provide four behaviors:

1. `describe()` -> routine descriptor
2. `validate_inputs(ctx, input)` -> typed validation result/error
3. `run(ctx, input)` -> typed output/error
4. `validate_output(ctx, output)` -> typed validation result/error

Execution requirements:

- No silent defaulting for missing required inputs.
- Domain/units violations return typed errors.
- Numerical edge cases are surfaced (no hidden clamping unless contract states
  clamping explicitly).

## Replacement semantics

Replacement metadata uses routine identity, not binary naming.

When `replaces` is set, the replacing routine must document:

- compatibility class (`compatible`, `breaking`)
- migration note reference
- comparator/parity evidence reference

Retiring a routine requires either:

- `replaced_by` reference, or
- explicit statement that no replacement exists.

## Selection semantics

Resolver inputs:

- required `contract_id`
- accepted `contract_version` range
- required capabilities
- optional explicit `routine_id`
- optional lifecycle opt-in flags (for `experimental`)

Resolver behavior:

1. If explicit `routine_id` is requested, use it only if lifecycle and contract
   constraints permit.
2. Otherwise, choose the highest-precedence compatible candidate:
   `active` > `deprecated` (unless policy override).
3. Exclude `retired` always.
4. Exclude `experimental` unless opt-in is explicit.
5. If no candidate satisfies constraints, fail with typed configuration error.

## Capability keys

Capabilities are string identifiers (for example):

- `domain:hillslope`
- `domain:channel`
- `domain:reservoir`
- `io:hbp-v1`

Capabilities are additive; removing a required capability is a breaking
contract change.

## Network amendment alignment

For watershed-node evolution (including reservoir modeling), routines selected
for a node must advertise capability compatibility with that `node_kind`.

Node adapters must reject routines lacking required node capabilities.

## Validation gates for lifecycle transitions

Before promoting `experimental` -> `active`, attach:

1. contract completeness evidence,
2. regression test coverage for changed behavior,
3. comparator/parity evidence where applicable,
4. migration notes when replacing an existing active routine.

## Failure posture

Contract mismatch is a hard error, including:

- missing descriptor fields,
- invalid lifecycle state value,
- unresolved schema/unit references,
- capability mismatch for requested domain or node kind,
- invalid replacement references.
