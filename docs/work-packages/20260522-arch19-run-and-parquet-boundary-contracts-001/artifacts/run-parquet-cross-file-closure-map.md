# Run-Parquet Cross-File Closure Map

Static: closure map assembled from openWEPP docs/contracts and `wepppyo3`
interchange sources.
Ran: none.
Status: `complete-with-open-items`.

## Boundary-to-Surface Closure Map

| boundary clause | producer surface | consumer surface | current closure state | follow-on ownership |
|---|---|---|---|---|
| `RUN-B-001` engine selector explicitness | `docs/contracts/openwepp-runner-contract.md` | runner launch boundary | `closed (governance)` | runner implementation owners |
| `RUN-B-002` run context determinism | parser options (`TcRunContext`, `LcwbRunContext`) | parser applicability + orchestrator adapters | `partial` (context exists, no top-level `.run` parser) | input-contract + orchestrator owners |
| `RUN-B-003` strict/compat sidecar policy | `openwepp-legacy-bridge/src/sidecar.rs` | compatibility adapter diagnostics and bindings | `closed` | legacy-bridge owners |
| `RUN-B-004` primary-surface declaration completeness | `input-surface-registry.md` + parser contracts | parser pipeline closure checks | `open` (`.run` registry entry absent) | parser governance owners |
| `RUN-B-006` alias continuity | `SC-INFILE-*` parser contracts | runtime/parquet boundary symbol mapping | `partial` | contract authors + runtime owners |
| `PRQ-R-003` version metadata governance | `schema.rs::schema_with_version` and family schema builders | parquet consumers and comparators | `closed (imported surfaces)` | output boundary owners |
| `PRQ-R-004` compression rule | `lib.rs::ensure_snappy`, `parquet.rs` | parquet writer boundary | `closed` | interchange owners |
| `PRQ-R-006` HBP branch/warning preservation | `SC-INFILE-HBP-001` + `hill_hbp.rs` | pass/parquet conversion surfaces | `partial` (inventory mapped; openWEPP-local validation gate missing) | ARCH19 follow-on validation owner |
| `PRQ-R-007` confidence-tier comparator governance | ADR-0011 + ARCH14/18 artifacts | acceptance/disposition workflows | `closed (governance)` | package authors/reviewers |

## Dependency Closure Links

| upstream artifact | consumed by | closure statement |
|---|---|---|
| ARCH17 runtime-input classification | run boundary authority | established representative parser->runtime seam pattern used as template for `.run` follow-on closure |
| ARCH18 parquet handoff | parquet boundary authority | HBP schema/profile + warning carry-forward constraints adopted into `PRQ-R-*` rules |
| ADR-0005 | parquet boundary authority | schema inheritance posture preserved; openWEPP remains contract authority owner |
| parser-contract requirements | run/parquet boundary map | cross-file constraints and boundary-export duties tied to future `.run` contract implementation |

## Open Closure Items (Must Remain HOLD)

| item_id | open item | gate impact |
|---|---|---|
| `XMAP-HOLD-001` | No canonical `.run` spec + `SC-INFILE-RUN-*` contract yet exists. | blocks promotion of run boundary to `GO` |
| `XMAP-HOLD-002` | No openWEPP-local automated parquet boundary conformance test gate. | blocks promotion of parquet boundary to `GO` |
| `XMAP-HOLD-003` | No typed openWEPP run model currently routes dataset-family parquet outputs by explicit run contract fields. | blocks end-to-end boundary closure claim |
