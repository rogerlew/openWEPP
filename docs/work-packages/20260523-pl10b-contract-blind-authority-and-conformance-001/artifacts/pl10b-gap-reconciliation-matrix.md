# PL10b Gap Reconciliation Matrix

Status: `complete`
Evidence mode: `Static + Ran`

## Matrix

| Finding ID | Evidence | Classification | Disposition | Follow-on action |
|---|---|---|---|---|
| `PL10B-GAP-001` | `pl10b_contract_conformance_requires_annual_extension_projection_symbols` fails (`missing symbol jdherb`) | implementation defect | accepted | PL11 must project annual extension symbol families and active-slot aliases. |
| `PL10B-GAP-002` | `pl10b_contract_conformance_requires_perennial_cutday_indexed_projection` fails (`missing ...cutday_0001`) | implementation defect | accepted | PL11 must emit indexed `cutday` symbols for `1..ncut`. |
| `PL10B-GAP-003` | `pl10b_contract_conformance_requires_perennial_grazing_cycle_payload_projection` fails (`missing ...gday_0001`) | implementation defect | accepted | PL11 must emit indexed `gday/gend` + grazing payload symbols for `1..ncycle`. |
| `PL10B-GAP-004` | `pl10b_contract_conformance_rejects_invalid_grazing_window_domain` fails (projection returns `Ok`) | implementation defect | accepted | PL11 must add typed guard for invalid grazing windows (`gday >= gend`). |
| `PL10B-GAP-005` | `pl10b_contract_conformance_rejects_empty_perennial_grazing_cardinality` fails (projection returns `Ok`) | implementation defect | accepted | PL11 must add typed guard requiring non-empty grazing-cycle payload when `mgtopt=2`. |
| `PL10B-GAP-006` | Annual burn fraction naming continuity (`fbmag` parser field vs canonical `fbrnag` symbol family) | ambiguous authority requiring escalation | amended | Keep canonical alias map in `SC-PLANT-001`; PL11 must document explicit parser-field -> runtime alias mapping in implementation evidence. |

## Contract Defect Assessment

- No blocking contract defect found for PL10b scope.
- Contract amendment remains authoritative and internally consistent with the
  baseline and kernel-profile schema requirements.

## Reconciliation Outcome

- Implementation defects are explicitly transferred to PL11 scope with
  dependency-gate patches.
- PL10b closes as governance-complete with implementation conformance failures
  documented and classified.
