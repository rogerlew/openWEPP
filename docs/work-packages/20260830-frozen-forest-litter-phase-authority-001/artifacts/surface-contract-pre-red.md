# Surface-owner V2 contract-first pre-red

Evidence mode: `Ran`

Status: `EXPECTED RED — PRODUCTION NOT YET AUTHORIZED FOR THIS WORKER`

## Scope

The canonical `SC-SURFACELIQUID-001` contract is amended to version 14 before
surface-owner production edits. The contract-derived integration test proves
the new authority text and an independent liquid/ice plus fusion-energy vector
before consulting production source. This worker did not edit production.

Execution snapshot: `6fa804082273c1c4340614ffc208a74a8b48e408` plus the
uncommitted version-14 contract/test diff.

## Command and result

Ran:

```text
nix develop -c cargo nextest run \
  --test surface_liquid_hydrology_custody_authority_contract \
  --no-fail-fast
```

Result: expected exit `100`; Nextest run
`0b6870a9-0fcb-4b1d-b566-33fd7e985940`; 13 tests attempted, 12 passed, 1
failed, 0 skipped.

The sole failure was:

```text
version_14_binds_frozen_litter_surface_owner_v2_before_production
unchanged production is missing frozen-litter V2 obligation
pub enum SurfaceLiquidOwnerEnvelopeV2
```

This is the intended production-absence boundary. Before that assertion the
test passed all exact version/invariant/source-hash/schema/chronology/refusal
bindings and independently reconstructed:

- separate liquid- and ice-vapor mass custody;
- equal opposing liquid/ice phase transfer;
- total water conservation; and
- exact `333700 J kg^-1 * 0.125 kg m^-2 = 41712.5 J m^-2` fusion energy.

The other 12 tests in the integration target passed, so the retained red is
not a malformed contract, malformed vector, compilation failure, or regression
in the frozen version-13 obligations.

Ran after the terminal contract wording pass: focused Nextest run
`e9580888-679e-46d2-a817-a7dfdfcfa11d` reproduced the same sole production-
absence failure in `0.013 s` test time.

## Diff and line-count evidence

Ran: `git diff --check -- <owned contract> <owned test>`: `PASS`.

Owned-path diff at handoff: contract `+216/-25`; integration test `+98/-8`.
Line counts at retention:

- `SC-SURFACELIQUID-001.md`: 1266 lines;
- `surface_liquid_hydrology_custody_authority_contract.rs`: 607 lines.

The contract growth is authority material. The integration-test growth is one
bounded contract/pre-red test plus one small recursive Rust-source reader; no
production extraction or behavior edit occurred in this slice.
