# PL11 Symbol Projection Map

Status: `complete`
Evidence mode: `Static + Ran`

## Annual Extension Families

| Canonical control | Deterministic runtime symbol family |
|---|---|
| `jdherb` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_jdherb` |
| `jdburn` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_jdburn` |
| `jdslge` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_jdslge` |
| `jdcut` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_jdcut` |
| `jdmove` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_jdmove` |
| `fbrnag` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_fbrnag` |
| `fbrnog` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_fbrnog` |
| `frcut` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_frcut` |
| `frmove` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_frmove` |

Primary aliases for slot/crop `0001/0001` are also emitted as unprefixed symbols.

## Perennial Families

| Canonical control | Deterministic runtime symbol family |
|---|---|
| `mgtopt` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_mgtopt` |
| `ncut` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_ncut` |
| `cutday[k]` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_cutday_{event:04}` |
| `ncycle` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_ncycle` |
| `gday[k]` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_gday_{cycle:04}` |
| `gend[k]` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_gend_{cycle:04}` |
| `animal[k]` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_animal_{cycle:04}` |
| `bodywt[k]` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_bodywt_{cycle:04}` |
| `area[k]` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_area_{cycle:04}` |
| `digest[k]` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_digest_{cycle:04}` |

Ran:
- Presence and guard behavior for these families is asserted by integration conformance tests in `tests/integration/parser_runtime_seam_integration.rs`.
