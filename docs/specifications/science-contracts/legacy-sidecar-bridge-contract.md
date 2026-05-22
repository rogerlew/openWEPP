# Legacy Sidecar Bridge Contract

Status: Draft (ARCH08)
Evidence: Static
Ran evidence: none

## Purpose

Specify typed strict/compat adapter contracts for:
- legacy sidecar boundary normalization
- HBP edge compatibility validation

Implementation path:
- `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/sidecar.rs`
- `/home/workdir/openWEPP/crates/openwepp-legacy-bridge/src/hbp.rs`

## Contract Interfaces

### Sidecar interface

Request:
- `SidecarAdapterRequest`
  - `policy: CompatibilityPolicy`
  - `contracts: Vec<SidecarContract>`
  - `discovered: Vec<SidecarDiscovery>`

Response:
- `SidecarAdapterResponse`
  - `bindings: Vec<SidecarBinding>`
  - `warnings: Vec<SidecarWarning>`

Error:
- `SidecarAdapterError`

### HBP interface

Request:
- `HbpAdapterRequest`
  - `policy: CompatibilityPolicy`
  - `contract: HbpHeaderContract`
  - `shard_bytes: &[u8]`

Response:
- `HbpAdapterResponse`
  - `observed_magic: [u8; 4]`
  - `payload_len: usize`
  - `magic_source: HbpMagicSource`
  - `warnings: Vec<HbpWarning>`

Error:
- `HbpAdapterError`

## Deterministic Warning IDs

| warning class | message id | mode | semantics |
| --- | --- | --- | --- |
| sidecar legacy alias accepted | `LSB-W-001` | compat | sidecar resolved via configured alias |
| sidecar unknown ignored | `LSB-W-002` | compat | discovered sidecar not in contract set |
| HBP legacy magic accepted | `HBP-W-001` | compat | shard magic matched configured legacy alias |

## Deterministic Sidecar Error IDs

| error class | code | semantics |
| --- | --- | --- |
| invalid sidecar id | `LSB-E-001` | sidecar id failed identifier rules |
| invalid file name | `LSB-E-002` | empty/invalid contract or discovery filename |
| duplicate contract id | `LSB-E-003` | duplicate `SidecarContract.id` |
| duplicate canonical file name | `LSB-E-004` | canonical name collision across contracts |
| duplicate/invalid alias | `LSB-E-005` | duplicate alias or alias equal to canonical |
| duplicate discovery file name | `LSB-E-006` | duplicate discovered filename after normalization |
| missing required sidecar | `LSB-E-007` | required contract sidecar unresolved |
| strict alias disallowed | `LSB-E-008` | alias-only match rejected in strict mode |
| strict unknown disallowed | `LSB-E-009` | unknown discovery rejected in strict mode |

## Deterministic HBP Error IDs

| error class | code | semantics |
| --- | --- | --- |
| invalid minimum-bytes contract | `HBP-E-001` | `minimum_bytes < 4` |
| duplicate legacy alias magic | `HBP-E-002` | duplicate magic alias in contract |
| canonical listed as alias | `HBP-E-003` | canonical magic duplicated in alias list |
| shard too short | `HBP-E-004` | shard shorter than contract minimum |
| unknown magic | `HBP-E-005` | magic matched neither canonical nor configured aliases |
| strict legacy magic disallowed | `HBP-E-006` | legacy magic alias in strict mode |

## Invariants

- `INV-LSB-CONTRACT-001`: Sidecar IDs and canonical file names are unique per
  request.
- `INV-LSB-CONTRACT-002`: Required sidecars must resolve to exactly one
  canonical or compat-allowed alias discovery.
- `INV-LSB-CONTRACT-003`: Strict mode emits no compatibility warnings because
  compat-only branches are rejected as errors.
- `INV-LSB-CONTRACT-004`: HBP contract must define at least a 4-byte minimum
  to permit magic extraction.
- `INV-LSB-CONTRACT-005`: HBP magic resolution is deterministic with precedence:
  canonical, then configured alias, then unknown failure.

## No-Fallback Policy

- No default sidecar synthesis on missing required inputs.
- No implicit acceptance of unknown sidecar files in strict mode.
- No implicit acceptance of unknown or short HBP shards in any mode.
- Compatibility acceptance always emits typed warnings with stable IDs.

## ARCH08 Test Linkage

Tests co-located in crate modules cover:
- strict/compat sidecar and HBP branches
- typed failure mapping
- warning ID stability for compat paths
