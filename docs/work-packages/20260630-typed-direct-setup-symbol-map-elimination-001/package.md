# Typed Direct Setup Path And Symbol-Map Carrier Elimination

Status: EXECUTED-HOLD-STAGE0-PREMISE-CORRECTED

Package id: `20260630-typed-direct-setup-symbol-map-elimination-001`

## Objective

Make the typed `DirectRunFrame` the sole production direct runtime
representation end to end, eliminating the setup-time symbol-map
`HillslopeWritebackSurface` carrier from the production direct path and reducing
RSS toward the PERFARCH03 array-native working-set floor.

## Required Reading

- `docs/architecture/array-native-runtime-specification.md`, especially the
  single-authority and memory/layout rules.
- `docs/decisions/0025-array-native-hillslope-day-frame.md`.
- `docs/decisions/0030-r7-terminal-contract-and-compatibility-runtime-deletion.md`.
- `docs/work-packages/20260630-compatibility-runtime-deletion-001/`.
- `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md`.
- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/`.

## Scope

Stage 0 profiles and attributes the current production direct RSS/time. Later
stages were gated on Stage 0:

1. Typed setup/seeding: production direct frame construction must consume typed
   input carriers rather than a merged symbol-map runtime surface.
2. Symbol-map carrier deletion: delete now-orphaned production scheduler/day-frame
   symbol-map paths, retaining only explicit `--compatibility-runtime` replay.
3. Working-set trim: remove unnecessary whole-run retention and drive RSS toward
   the PERFARCH03 floor.

## Non-Scope

- No physics, snow, frost, hydrology, erosion, or output-schema change.
- No default-policy change.
- No removal of explicit `--compatibility-runtime` unless separately directed.
- No watershed or stream-temperature work.

## Gates

Per stage:

- H2637 direct fixture, a multi-OFE fixture, and a Wave-2 fixture remain
  byte-identical before vs after the stage for HBP/WAT/PASS/loss/plot.
- RSS and elapsed time are measured and recorded.
- No production symbol-map invocations remain in the staged scope.

Final gates:

- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo nextest run --workspace --profile full`.
- `cargo deny check`.
- `bash tools/release/check_authority_suite_antievasion.sh`.
- `cargo test --test auth11_required_suite_obligation_guards_contract`.
- Markdown lint on touched docs.

## Disposition

Stage 0 executed and corrected the premise. The production direct path still has
a setup-time symbol-map carrier, so typed setup and carrier deletion remain
necessary. However, H2637 RSS does not move when optional WAT/PASS/plot outputs
are removed, and a tiny two-day fixture uses only `19584 KiB`; the evidence
points to whole-run retained publication/ledger state as the dominant RSS driver,
not a fixed setup-only symbol-map allocation.

The package therefore stops before Stage 1. Proceeding directly into typed setup
would be high-risk and would not satisfy the package's per-stage RSS gate. The
next actionable package should first stream/drop retained direct publication
artifacts and output projection rows, then resume typed setup and symbol-map
carrier deletion once the RSS gate has a plausible implementation path.
