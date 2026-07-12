# HB-10 Watershed CLI Publication

Status: `ACTIVE`
Parent: `docs/work-packages/cqr-high-risk-b-execplan.md`

## Objective

Close the six fixed production rows in `openwepp-cli-watershed.rs` through
cover-first characterization and coherent mechanical extraction. Preserve CLI
arguments, runfile grammar/path resolution, manifest policy and error priority,
area/unit conversion, subprocess/routing order, typed frame handoff, output
inventory and publication values.

## Target And Start State

- Source SHA-256: `5e388b7781e2850411bd7c70e92695c06316c07fba99f89dcdfdc0fa4fc92f54`.
- Lines: `2,350`, WARN but below the 3,000-line blocker.
- Tier: six `E-PRODUCTION` rows.

| Fixed row | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `run` | 58 | 77.446% | 96.596 |
| `hillslope_area_m2_from_source_runfile` | 11 | 45.614% | 30.465 |
| `parse_watershed_runfile` | 66 | 69.369% | 191.186 |
| `validate_manifest_publication_metadata` | 18 | 53.846% | 49.854 |
| `validate_manifest_per_ofe_wb13_publication_policies` | 11 | 45.714% | 30.357 |
| `validate_manifest_mofe_hourly_carry_metadata` | 17 | 51.724% | 49.515 |

The fresh start CRAP audit also binds every eligible same-source production
function below 75%; the preimplementation record lists those floors. Literal
help rendering may be excluded only with explicit glue classification.

## Authority And Provenance

- `docs/contracts/openwepp-watershed-runfile-contract.md`: schema, required
  inputs/outputs, applicability selectors, paths and `CLIWAT-E-*` behavior.
- `SC-SYSTEM-001` `INV-SYSTEM-001..006` and `028..036`: pass inventory,
  routing dispatch, MOFE metadata, channel balance, no-event intake, typed
  `chan.inp` defaults and terminal event publication.
- `SC-WATBAL-001` `INV-WATBAL-042/097/098/099`: storage lineage, per-OFE
  publication, anti-clone and erosion-boundary manifest policy.
- Pinned baseline `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` remains comparator provenance;
  current typed runfile/manifest contracts control validation behavior.

## Bounded Write Set

- `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`: cover-first
  tests or coherent private extraction for the six rows and transitive floors.
- `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`: only
  missing real-CLI characterization and independent readback assertions.
- Package evidence and HB-10/High-B terminal records.

No grammar, selector, manifest policy, units, path, error text/code/order,
subprocess scheduling, topology/routing, science arithmetic, output schema or
publication authority change is authorized. A semantic defect transitions to
a committed DC package.

## A–H Obligations

| Family | Required evidence |
| --- | --- |
| A — nominal | Generated/reuse execution reaches typed network/publication frames and all required Parquet outputs. |
| B — boundaries | Zero/one/multiple contributors, jobs bounds, relative/absolute paths, single/multi-OFE, first/last OFE keys and 0/24 carry slots. |
| C — regimes | Metric/English source area, aggregate/per-OFE policy, active/inactive carry, event/no-event, serial/worker-pool and routing branches. |
| D — invalid domain | Missing/malformed runfile/input/manifest, unsupported units/policies/schema, duplicate IDs, invalid counts/keys/area/totals and stale pass inventory. |
| E — missing seam | Required selectors/files/manifest fields fail exactly; optional sidecars/default branches remain contract-bound. |
| F — non-finite | Area and carry totals reject NaN/infinity/negative values with existing priority; no coercion. |
| G — conservation | Independently reconstruct publication area, runoff volume/alias identities, terminal water/sediment and channel balance from consumer outputs. |
| H — fail closed | Preserve `CLIWAT-E-001..045`, path/detail provenance, routing/publication skip after upstream failure and no compatibility fallback. |

## Existing Real Consumer

`openwepp-runner/tests/watershed_cli_behavior_contract.rs` is the binding
consumer. Its executable vectors cover argument/jobs failures, relative paths,
serial/parallel pass supervision, typed frames, stale/malformed/no-event HBP,
baseline EBE and sediment publication, applicability, legacy sidecars,
multi-OFE manifest rejection/acceptance and per-OFE metadata. Source-string
assertions and private helpers are supporting evidence only; closure requires
the spawned CLI plus Parquet/manifest readback.

## Execution And Acceptance

1. Re-capture current runner JSON/LCOV/CRAP and enumerate all same-source
   production floors.
2. Map existing real-CLI vectors to A–H and add only missing failure/readback
   characterization before decomposition.
3. Re-measure; extract coherent argument, runfile resolution, manifest field
   family and execution/publication stages only as needed, without reordering.
4. Run full runner tests and the complete watershed CLI behavior contract.
5. Record exact metrics/hashes/counts, consumer lineage, line governance, two
   independent reviews and two verifications before disposition.

Minimum gates:

    cargo nextest run -p openwepp-runner
    cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract
    cargo fmt --check
    cargo clippy -p openwepp-runner --all-targets -- -D warnings
    git diff --check

Acceptance requires all six fixed rows at CRAP at most 30, zero eligible
same-source function below 75%, exact grammar/error/publication preservation,
real executable consumer PASS and the 2,000-line WARN/3,000-line blocker
disposition.

Subagent authorization: this package explicitly authorizes spawning/delegating
to review and verification subagents for read-only final metric, behavior,
consumer-lineage and gate review; expected outputs are package review and
verification artifacts; write access is read-only.
