# Direct Hydrology Persisted Restart V1

Status: draft; executable authority review findings remain open

Authority identities: `OPENWEPP_DIRECT_HYDROLOGY_RESTART_V1` and
`OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1`.

## Scope and representation

This contract governs default-off snow-free V10/LSE-V2 shadow continuation.
It never serializes a Rust object or `DirectRunFrame` memory layout. The wire
representation is UTF-8 JSON with no insignificant whitespace, object members
in the schema order below, arrays in canonical owner/topology order, integers
as JSON integers, booleans as JSON booleans, and every binary64 value as the
lowercase string `0x` followed by exactly sixteen hexadecimal digits containing
the IEEE-754 bits. Thus `+0.0` and `-0.0` remain distinct.

The top-level member order is `schema`, `version`, `run_identity`, `topology`,
`configuration_identities`, `transaction_lineage`, `phase`,
`payload_sha256`. `payload_sha256` is the lowercase SHA-256
of the canonical prefix object with that member omitted. Parsers reject
unknown, missing, duplicate, or reordered members and noncanonical bytes.

## Canonical owners

`committed_owners` and `staged_candidate_owners` use the same named owner DTO
object in this order:

1. `gsi_configuration` and `gsi_state` under CP-GSI01;
2. `static_forcing_configuration` and `forcing_provider_cursor`;
3. `vegetation_v10_configuration` and `vegetation_v10_state`;
4. `lse_v2_configuration` and `lse_v2_state`;
5. `direct_hydrology`;
6. `surface_liquid_configuration`; the state exists exactly once inside
   `direct_hydrology`;
7. `soil_thermal_configuration` and `soil_thermal_state`;
8. `biogeochemistry_configuration` and `biogeochemistry_state`.

Each owner is an explicitly projected DTO, not an embedded Rust debug or raw
memory representation. Every owner carries its canonical identity/digest.
Topology contains ordered lanes, OFEs, tiles, and soil-layer maps. The direct
hydrology DTO persists lane water and soil continuation, ordered transfer
custody, downstream operands, and groundwater. It binds the phase-plan digest
but does not persist the plan. Publication, transient diagnostics, and the
transfer shadow projection are excluded and deterministically reconstructed.
`laned_active` and an active summary are unsupported and reject admission.

## Explicit checkpoint phase

The checkpoint contains exactly one tagged phase union and therefore never has
a third or duplicated owner set.

`BetweenDays` contains `committed_owners`, bounded `next_day_index`, and
`accepted_interval_count`. It is the only legal representation before interval
0 and after interval 48 has finalized.

`InProgressDay` contains, in order: `day_index`, `next_interval_index`,
`committed_beginning_owners`,
`staged_candidate_owners`, `accepted_gsi_daily_receipt`,
`staged_gsi_ending_state`, `beginning_provider_cursor`,
`ending_provider_cursor`, `validated_forcing_receipts`, and
`accepted_interval_count`. `next_interval_index` is a bounded `u8` in `1..47`.
The forcing receipt vector is ordered by `(ofe_id,tile_id)` and every
destination record has exactly 48 canonically ordered interval receipts even
when only a prefix has executed. Interval zero uses `BetweenDays`; interval 48
must finalize to `BetweenDays` before persistence. Beginning owners remain immutable while staged
owners advance. Receipt date, run, topology, destination, WB14, CO2,
reference-height, static-configuration, GSI-owner and receipt digests must join.
All lane GSI receipts must have exactly equal canonical bytes; heterogeneous
receipts are `heterogeneous_lane_gsi_receipt`, never averaged or selected.

## Restore algorithm and atomicity

Restoration parses into an isolated DTO, verifies canonical bytes and digest,
then independently validates schema/version, run identity, topology,
configuration identities, owner identities, transaction lineage, scheduler,
cursor, GSI receipt, receipt cardinality/order, and every owner. It reconstructs
the transient V9 and LSE-V1 projections from canonical V10/LSE-V2 owners and
requires exact serialized payload identity with the projections used by the
runtime. It reconstructs phase plans, transfer shadow projection, publication
scratch, and other deterministic caches. Only after the complete candidate
passes validation may the live owner be replaced by one non-fallible assignment.
No error path mutates the live owner.

## Typed failure taxonomy

Failures are stable categories: `schema`, `unsupported_version`,
`noncanonical_bytes`, `payload_digest`, `missing_field`, `extra_field`,
`reordered_field`, `duplicate_field`, `run_identity`, `topology_identity`,
`configuration_identity`, `owner_identity`, `transaction_lineage`,
`scheduler_position`, `provider_cursor`, `gsi_receipt`,
`heterogeneous_lane_gsi_receipt`, `forcing_receipt_cardinality`,
`forcing_receipt_order`, `forcing_receipt_digest`, `v10_v9_projection`,
`lse_v2_v1_projection`, `owner_validation`, and `unsupported_laned_active`.

## Primitive wire types

`HexF64` is exactly `0x` plus sixteen lowercase hexadecimal digits. `HexU128`
is exactly `0x` plus thirty-two lowercase hexadecimal digits. `Sha256` is
exactly sixty-four lowercase hexadecimal digits. Day indices and accepted
interval counts are bounded `u64`; destination counts are `u32`; interval
indices are bounded `u8`; lane, OFE, tile, run, and owner identities are named
nonempty strings. No `usize`, Rust enum representation, Debug output, or raw
layout is persisted.

Canonical admission is: strict typed parse with duplicate/unknown-field
rejection; complete semantic validation; canonical serialization; exact input
bytes equal canonical output bytes; then payload SHA-256 validation. This
rejects whitespace, alternate escaping, member reordering, alternate integer
spellings, and uppercase or malformed hexadecimal even when JSON Schema alone
would accept the semantic value.

The authoritative machine schemas, frozen boundary and in-progress vectors,
and poison matrix live in the restart-authority work package. Authority is not
released until their digests and every typed poison are executable evidence.
