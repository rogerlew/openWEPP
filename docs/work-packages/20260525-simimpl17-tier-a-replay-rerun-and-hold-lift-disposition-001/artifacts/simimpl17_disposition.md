# simimpl17_disposition

Status: package-complete-with-hold
Evidence mode: static+ran
Date: 2026-05-25
Decision: HOLD

## Static
- Phase A complete: intake/dependency confirmation (`SIMIMPL14/15/16` `GO`).
- Phase B complete: canonical contract authority ratified; no new amendments
  required.
- Phase C complete: contract-derived tests verified via targeted gate runs.
- Phase D complete: Tier-A reruns executed and evidence persisted.
- Phase E complete: closure criteria evaluated and final disposition recorded.

## Ran
- Candidate + replay evidence bundle generated:
- `artifacts/replay-run-20260525T075424Z/`
- Shared-input rerun evidence shows:
- candidate manifest `climate_day_count=1095`, `executed_day_count=1095`,
  `wb13 row_count=1095`.
- shared-input hash manifest recorded at
  `replay-run-20260525T075424Z/shared_fixture/input_file_sha256.txt`.
- legacy baseline lane logs clamp simulation years to `1`, yielding dat strict
  line-count mismatch (`393` baseline vs `1095` candidate).
- Criteria outcomes:
- fail: `CRIT-001`, `CRIT-002`, `CRIT-003`, `CRIT-004`
- pass: `CRIT-005`, `CRIT-006`, `CRIT-007`
- partial: `CRIT-008`
- Required repository gates all passed (`fmt`, `clippy`, `test`, `deny`).

## Final disposition
- Package `COMPLETED` with retained `HOLD`.
- Hold-lift is not approved pending closure of remaining hard replay parity
  blockers.
