# HB-06 Runner WB13 Consumer Adoption

Status: `ACTIVE`
Parent: `docs/work-packages/cqr-high-risk-b-execplan.md`
Hold record: `docs/work-packages/cqr-pre-integration-campaign-evidence/hb/modules/HB-06.md`

## Objective

Make the production runner's common WAT projection consume the accumulator's
typed WB13 row validation, then prove the executable-to-Parquet downstream path
without changing public values, schema, key order, buffering, or error posture.

## Authority And Finding

`SC-SYSTEM-001` INV-031 and the WB13 water-balance authority require canonical
`QOFE == Q` and the existing storage/profile relationships. The accumulator
enforces these relationships, but production currently maps
`DirectPublicationDayRow` directly into `HillslopeWatRow`. Public accumulator
integration tests therefore do not prove production adoption.

Two independent read-only scope reviews accepted the common runner WAT
projection as the smallest correct seam. Both prohibit text render/parse and
schema/writer changes. The selected typed-constructor approach follows review
A because it avoids per-row `BTreeMap<String, f64>` construction and lets the
validated row supply canonical fields directly. Review B's stronger requirement
to return bit-identical output and exercise the executable Parquet consumer is
binding.

## Bounded Write Set

- `crates/openwepp-summary-accumulator/src/lib.rs`: add a typed WB13 input/value
  seam and make `from_surface` delegate to it without changing symbol-load or
  validation priority.
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`: use
  that seam in `build_hillslope_wat_row_from_direct_publication` and populate
  the 25 canonical WAT fields from the validated row.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
  and its focused tests: reject a deliberately corrupted publication frame
  whose independently stored `soil_water_total_m` differs from
  `total_soil_m`. Normal projection already assigns the canonical alias
  identity in `projection.rs`; this is a boundary guard, not new physics.
- Focused tests in the nearest existing accumulator and runner test modules.
- This package's evidence and the HB-06 campaign record.

No edits are authorized to hillslope-output schemas/writers,
`DirectPublicationStreamingSink` ordering/buffering, manifests, or unrelated
runner paths.

## Invariants And Acceptance

1. Preserve checked OFE/year/julian conversion and existing runner error
   precedence.
2. Preserve all canonical scalar values bit-for-bit; do not render or parse the
   text WB13 representation.
3. Preserve runner-only additive WAT fields and Option/nullability behavior.
4. Keep the runner's publication-boundary QOFE guard; accumulator admission is
   defense in depth and real-consumer adoption.
5. Test constructor/from-surface parity, pairwise-distinct operand mapping,
   invalid storage/profile relationships, and unchanged runner projection.
6. Run the accumulator public-output contract and the SIMIMPL04 executable WAT
   publication contract. The latter must read the emitted Parquet output.
7. Recapture HB-06 focused coverage/CRAP, prove zero eligible production
   function below 75%, obtain two independent final reviews, and update HB-06
   to `MODULE-PASS` only if the executable consumer proof passes.

## Focused Commands

    cargo nextest run -p openwepp-summary-accumulator
    cargo nextest run -p openwepp-runner --lib
    cargo nextest run -p openwepp-runner --test simimpl04_wb13_publication_contract
    cargo nextest run --test wb13_daily_water_balance_output_surface_contract
    cargo fmt --check
    cargo clippy -p openwepp-summary-accumulator -p openwepp-runner --all-targets -- -D warnings

Record exact commands, counts, source hashes, metrics, lineage, reviewer
findings, and any deviations under `artifacts/` before terminal disposition.

## Discovery Amendment

Strict runner adoption exposed that the direct publication constructor accepted
manually corrupted frames with distinct storage aliases; an existing unit test
even required `21 mm != 19 mm`. This contradicts `SC-WATBAL-001`, which defines
both fields as the same unfrozen `watcon` publication within `1e-6 mm`.
Canonical runtime projection already performs
`soil_water_total_m = total_soil_m`, so the bounded correction is an admission
guard at `direct_publication_storage_operands` plus replacement of obsolete
acceptance fixtures. A validation policy that permits distinct aliases is
explicitly prohibited.
