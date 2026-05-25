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

## Hydrologic semantic guidance for follow-on package
- Shared forcing is aligned in the overlap window: `P` passes semantic tolerance
  with near-zero numeric drift across `365` common rows (no precipitation ingest
  mismatch signal).
- Divergence starts at the first shared key (`OFE=1`, `J=1`, `Y=1`):
  baseline row reports `RM=0.00`, `Snow-Water=4.40`, `Total-Soil=102.70`,
  `frozwt=1.22`, `SoilWaterTotal=103.92`; candidate row reports `RM=4.40`,
  `Snow-Water=250.00`, `Total-Soil=76.00`, `frozwt=0.00`,
  `SoilWaterTotal=76.00`.
- Candidate storage surfaces are invariant for all `1095` candidate rows in this
  bundle (`Total-Soil=76.00`, `frozwt=0.00`, `Snow-Water=250.00`,
  `SoilWaterTotal=76.00`), while baseline year-1 storage evolves
  (`Total-Soil` range `48.59..102.70`, `frozwt` range `0.00..27.43`,
  `Snow-Water` range `0.00..16.67`, `SoilWaterTotal` range `55.16..103.92`).
- Candidate provenance records
  `coupling_vectors.winter.ssd=250.0` and
  `coupling_vectors.hydout_equivalent.snow_water=250.0`, which is consistent
  with a static-parameter-to-state publication leak in the winter/hydout
  mapping.
- Legacy baseline logs in both lanes still emit one-year clamp warnings
  (`Number of years to simulate can't be larger than 1`; `1 years used`), so
  row-span closure requires explicit baseline-year policy handling in the next
  package in addition to day-1 physics closure.
- Potential contract/test amendments for next package:
- add a day-1 keyed semantic contract fixture asserting parity on
  `RM`, `Snow-Water`, `Total-Soil`, `frozwt`, `SoilWaterTotal`;
- add a non-invariant storage-state guard over the executed span for winter
  climate fixtures (reject constant `Snow-Water`/`frozwt`/soil-store surfaces
  when forcing and temperatures vary);
- add explicit mapping/closure checks that published `Snow-Water` derives from
  runtime SWE state, not `snow.txt` static density parameters;
- codify baseline replay-span policy (clamp-to-one-year vs multi-year rerun)
  before re-running hold-lift criteria.

## Final disposition
- Package `COMPLETED` with retained `HOLD`.
- Hold-lift is not approved pending closure of remaining hard replay parity
  blockers.
