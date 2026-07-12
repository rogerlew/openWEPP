# HB-09 Watershed Output Writers

Status: `ACTIVE`
Parent: `docs/work-packages/cqr-high-risk-b-execplan.md`

## Objective

Close the two fixed HB-09 production rows in the watershed Parquet writer by
covering schema/field/error obligations first, then mechanically decomposing
only coherent output iteration and Float64 field families. Preserve every
schema, column order/type/nullability, alias, unit conversion, Option/null
behavior, file order, error code/path and emitted value.

## Target And Start Metrics

- Source: `crates/openwepp-watershed-output/src/writers.rs`.
- SHA-256: `f29e399ed6297c2543f421c766a7f3635e6c4e134e53bf94d59d699df2e9c353`.
- Lines: `2,706`, WARN but below the 3,000-line blocker.
- Tier/class: production / two `E-PRODUCTION` rows.

| Fixed row | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `write_output_record_parquet_outputs` | 29 | 84.483% | 32.142 |
| `float64_value` | 69 | 100.000% | 69.000 |

Coverage alone cannot close `float64_value` because CRAP is bounded below by
CC. A fresh same-source JSON/LCOV/CRAP run must enumerate every eligible
production function below the 75% region floor; no stale cross-crate artifact
may substitute for that audit.

## Authority And Consumer

- `SC-SYSTEM-001` watershed publication closure, especially closed
  `GAP-SYSTEM-005/006`: the baseline-authoritative CLI comparator and all
  required row-model-backed Parquet outputs.
- `SC-ROUTE-001` routed water/sediment units, terminal outlet ownership,
  `INV-ROUTE-021/022`, and WS10 publication aliases.
- `docs/contracts/openwepp-watershed-runfile-contract.md`: required output
  paths for `ebe_pw0`, channel/water-balance/soil/loss families and
  `totalwatsed3`.
- Schema authority is the fourteen `watershed_*_schema` builders in this
  crate; source record authority is `WatershedOutputRecord` implemented by
  typed publication frames and interchange seeds.
- Pinned baseline comparison authority remains
  `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`, including the registered
  delicate-game `ebe_pw0` fixture.

The real consumer is `openwepp-runner/tests/watershed_cli_behavior_contract`.
Its baseline-authoritative and sediment-active CLI vectors execute watershed
publication, read `ebe_pw0.parquet` and `totalwatsed3.parquet`, and assert
dispatch/branch/publication, water, sediment and alias identities. Writer unit
tests are necessary but cannot alone close the downstream claim.

## Bounded Write Set

- `writers.rs`: cover-first tests and behavior-preserving private helper
  extraction for the two fixed rows and transitive floor closure.
- Existing watershed-output test module/fixtures in the same file.
- Runner watershed CLI consumer only if a missing emitted-field assertion is
  demonstrated.
- Package evidence and HB-09/High-B terminal records.

No schema field/order/type/nullability/metadata change, output path change,
new alias, numeric normalization, science formula, terminal-selection rule,
runner orchestration or fallback is authorized. Any semantic/schema defect
requires a committed DC package.

## A–H Obligations

| Family | Required evidence |
| --- | --- |
| A — nominal | Both record implementations write all fourteen required Parquet outputs; representative values read back exactly. |
| B — boundaries | Empty/one/multi-row batches, five sediment classes, zero values, optional null/present fields, every first/last schema field. |
| C — regimes | Depth versus volume, kg versus tonnes, water/storage/channel/loss families, typed publication versus interchange seed. |
| D — invalid domain | Unsupported Arrow type, schema/batch mismatch, invalid parent/path and writer initialization/write/finalization failures. |
| E — missing seam | Missing optional operands remain null; required output paths/schema fields cannot be silently omitted. |
| F — non-finite | Preserve current Arrow/Parquet handling or exact typed rejection for NaN/infinity; no normalization to zero/null. |
| G — conservation | Depth×area/1000 volume, detachment−deposition yield, pollutant sums, ET sums and channel balance independently reconstruct. |
| H — fail closed | Preserve `OWSOUT-E-001..005`, exact path/detail provenance and no partial success masking or field fallback. |

## Existing Focused Tests

The writer tests already build typed and interchange batches, assert schema
metadata, integer/string columns, representative Float64 aliases, optional
nulls, depth/volume and sediment values, and read emitted Parquet. Cover-first
work must inventory every `float64_value` match family and every output writer
call/error edge, then add only missing A–H vectors.

Named real-consumer candidates in `watershed_cli_behavior_contract` include
the baseline `ebe_pw0` comparison, sediment-active P102/jobs identity, manifest
area `Q == runvol`/Runoff reconstruction, and output inventory/readback. Select
the narrowest vector that reads both representative writer surfaces after the
final change.

## Execution And Acceptance

1. Capture fresh same-source JSON/LCOV/CRAP and eligible region floors.
2. Add missing A–H characterization before decomposition.
3. Re-measure; extract a declarative output-spec iteration and coherent
   Float64 families only as needed, retaining exact match precedence and
   arithmetic grouping.
4. Run full watershed-output tests and the named watershed CLI consumer.
5. Record hashes/metrics/counts, schemas/lineage, review/verification and line
   governance before terminal disposition.

Minimum gates:

    cargo nextest run -p openwepp-watershed-output
    cargo nextest run -p openwepp-runner --test watershed_cli_behavior_contract
    cargo fmt --check
    cargo clippy -p openwepp-watershed-output -p openwepp-runner --all-targets -- -D warnings
    git diff --check

Acceptance requires both fixed rows at CRAP at most 30, zero eligible function
below 75%, exact schema/value/error preservation, a real CLI Parquet consumer
PASS, and two final reviews/verifications under the High-B plan.
