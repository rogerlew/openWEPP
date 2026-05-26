# Frost Energy Solver Work-Package Queue

Status: queued
Evidence mode: static
Date: 2026-05-26

## Static
- Queue is dependency-ordered to close the SIMIMPL30 frost-process `HOLD`.
- Baseline parity authority target remains:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Current openWEPP frost coupling remains reductive and does not yet represent
  the baseline frost routine chain (`winter`/`frostn`/`frsoil`/`frwatc`/
  `frzng`/`frznw`/`winthd`/`getfreezecond`) as required for process parity.
- Every code-authoring package below must enforce internal contract-first
  sequencing:
  1. contract amendments,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production code edits.

## Proposed Queue
| order | wp_id | objective | depends_on | exit signal |
|---|---|---|---|---|
| 1 | `20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001` | Close canonical `SC-SNOWFREEZE-001` frost process-authority gaps by mapping baseline frost routine chain (`winter` + `frostn` family + `getfreezecond`) to explicit openWEPP boundary/state aliases and invariants. | FROSTPLAN01, SIMIMPL30 | Canonical contracts ratify authoritative frost routine map, alias continuity, and invariant set with no unresolved authority ambiguity for migration scope. |
| 2 | `20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001` | Implement contract-derived tests for frost-hourly process families (`Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, `InfCap_frz`, `frost.hourly.*`) and record pre-implementation gate evidence before kernel edits. | SIMIMPL31 | New tests fail on current reductive path where expected and pre-implementation gate evidence is recorded before production edits. |
| 3 | `20260526-simimpl33-frost-energy-runtime-state-topology-and-seam-closure-001` | Implement required runtime state topology and typed seam wiring to support baseline frost process execution (fine-layer indices, layered conductivity terms, freeze/thaw bookkeeping state lineage). | SIMIMPL32 | Runtime state surfaces and typed errors required by canonical authority are available and validated without silent defaults. |
| 4 | `20260526-simimpl34-frost-energy-solver-kernel-migration-and-coupling-001` | Replace reductive `compute_active_frost_coupling` behavior with baseline-authoritative frost solver process migration and coupling into infiltration/runoff/water-balance paths. | SIMIMPL33 | Frost runtime behavior follows baseline-authoritative process shape with passing contract-derived tests and no proxy equations in production path. |
| 5 | `20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001` | Re-run winter-hourly parity lanes focused on frost closure and publish explicit GO/HOLD hold-lift disposition with residual ownership. | SIMIMPL34 | At least one admissible lane demonstrates non-zero overlap and closure evidence for frost process family, or HOLD is retained with explicit blockers. |

## Sequencing Constraints
1. SIMIMPL31 must complete before any frost runtime code edits.
2. SIMIMPL32 must complete before SIMIMPL33/SIMIMPL34 production edits.
3. SIMIMPL33 must land before full solver migration in SIMIMPL34.
4. SIMIMPL35 is final closure gate for frost process parity and hold-lift.

## Hold-Lift Target
- Replace current reductive frost branch posture with baseline-authoritative
  frost process behavior and evidence-backed parity disposition.

## Ran
- `sed -n '1,220p' docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/simimpl30_disposition.md`
- `sed -n '1,220p' docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/simimpl30-hold-lift-decision-report.md`
- `rg -n "frost\.hourly|GAP-SNOWFREEZE-002|SIMIMPL30|HOLD" docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/simimpl30_disposition.md`
