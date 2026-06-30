# Direct Publication RSS Reduction

Status: EXECUTED-HOLD-PARTIAL-RSS-REDUCTION

Package id: `20260630-direct-publication-rss-reduction-001`

## Objective

Flatten direct endpoint run-length-scaling RSS by removing whole-run retained
publication allocations that are not required by requested outputs, while
preserving direct-production output identity.

## Required Reading

- `docs/work-packages/20260630-typed-direct-setup-symbol-map-elimination-001/`.
- `docs/architecture/array-native-runtime-specification.md`, especially
  memory/layout section 4.11.
- `docs/decisions/0025-array-native-hillslope-day-frame.md`.
- `docs/decisions/0030-r7-terminal-contract-and-compatibility-runtime-deletion.md`.
- `docs/work-packages/20260618-perfarch03-full-array-native-floor-prototype-001/`.
- Direct publication code in `crates/openwepp-runner/src/hillslope/` and
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/`.
- Hillslope parquet writers in `crates/openwepp-hillslope-output/src/`.

## Scope

Stage A profiles the current endpoint RSS, including allocator/RSS evidence and
explicit retained-row accounting. Stage B eliminates justified run-length
retention in identity-preserving increments:

1. Move, rather than clone, retained `DirectPublicationExecution`.
2. Build direct WAT/PASS projection vectors only when those outputs are
   requested.
3. Remove setup allocations that Stage A proves are run-length-scaling even
   though they are not publication output.
4. Evaluate deeper streaming/chunking only if it can satisfy byte identity or
   is held with a named byte-identity blocker.

## Non-Scope

- No typed setup or symbol-map carrier deletion.
- No physics, output-schema, default-policy, snow, frost, hydrology, or erosion
  behavior change.
- No parquet layout change unless byte identity is proven.

## Gates

Per implementation step:

- H2637 direct endpoint RSS measured.
- At least one shorter direct run RSS measured.
- HBP/WAT/PASS/loss/plot byte identity is preserved for outputs in scope.
- No runtime selection or compatibility invocation regression.

Final closure gates:

- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo nextest run --workspace --profile full`.
- `cargo deny check`.
- `bash tools/release/check_authority_suite_antievasion.sh`.
- `cargo test --test auth11_required_suite_obligation_guards_contract`.
- Scoped Markdown lint/validate.

## Disposition

Held after a large identity-preserving partial reduction.

The premise from the prior package was corrected one level further. H2637 RSS
was not dominated by setup-time symbol-map seeding. It was dominated first by a
typed direct setup allocation: every lane preallocated a
`Vec<DirectDayConstructorInputs>` for every day/OFE, even though the production
direct executor constructs day inputs dynamically. That allocation alone is
about `909 MiB` for H2637 (`235961` rows x `4040 B`).

The package removes that allocation on the production direct path, moves rather
than clones the retained direct publication execution, and skips WAT/PASS
projection row construction when those outputs are not requested. H2637 full
output RSS drops from `1159672 KiB` to `316212 KiB`; H2637 HBP/loss-only drops
from `1159296 KiB` to `184644 KiB`. H2637 HBP/WAT/PASS/loss/plot outputs are
byte-identical to the baseline full-output run, and HBP/loss are byte-identical
for the minimized run.

The package does not close the requested run-length-flat gate. The remaining
H2637 minimized run is still `184644 KiB` versus `19584 KiB` for `cli01`,
because `DirectRunPublicationFrame.rows` still retains all
`DirectPublicationDayRow` values for the whole run. Full-output H2637 is higher
again because WAT/PASS projection vectors and parquet/Arrow buffers still scale
with row count. The next package should replace whole-run retained direct
publication rows with a streaming publication sink while preserving byte
identity, or explicitly amend the identity gate if parquet row-group chunking is
accepted to change file bytes.
