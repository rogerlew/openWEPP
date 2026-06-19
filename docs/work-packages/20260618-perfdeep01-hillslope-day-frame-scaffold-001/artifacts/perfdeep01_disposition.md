# PERFDEEP01 Disposition

Evidence class: Static + Ran.

Status: complete 2026-06-18.

Verdict: GO - Stage 0 is complete; PERFDEEP02 hydrology-island migration may proceed.

## Decision

PERFDEEP01 delivered the Stage-0 array-native scaffold without changing
production authority. `HillslopeDayFrame` now has a typed schema, seed/flush
paths, and a shadow roundtrip identity harness. The required Stage-0 migration
ledgers are complete (publication operand lineage, guard-tier catalogue,
contract transition map), and H2637 endpoint behavior remained flat with output
parity preserved.

No hydrology phase was migrated and no logical production surface was retired;
that remains Stage-1 scope by design.

## Acceptance Table

| Criterion | Status | Evidence |
| --- | --- | --- |
| Frame schema + seed/flush + shadow harness implemented | PASS | `crates/openwepp-hillslope-orchestrator/src/day_frame.rs` + focused tests |
| Roundtrip identity fixture (`to_bits`) | PASS | `cargo test -p openwepp-hillslope-orchestrator perfdeep01_frame_ -- --nocapture` (`3 passed`) |
| Publication operand-lineage ledger complete | PASS | `artifacts/perfdeep01-publication-operand-ledger.md` |
| Guard-tier catalogue complete | PASS | `artifacts/perfdeep01-guard-tier-catalogue.md` + bounded-site inventory |
| Contract transition compatibility map complete | PASS | `artifacts/perfdeep01-contract-transition-map.md` |
| Endpoint flatness vs PERFMIG01 baseline | PASS | `669.06 s / 227916 KB` vs `669.97 s / 228144 KB` (delta `-0.91 s`, `-228 KB`) |
| H2637 output parity | PASS | byte-identical `.hbp`, `.wat.parquet`, `.loss.json`, `.plot.parquet`; `pass.parquet` Arrow-equivalent |
| Determinism rerun evidence | PASS | rerun `h2637_determinism_run2 671.10 228844`; run1 snapshot vs run2 is byte-identical for `.hbp`, `.wat.parquet`, `.loss.json`, `.plot.parquet`; `pass.parquet` Arrow-equivalent by schema/row multiset equality |
| `cargo fmt --check` | PASS | Ran |
| `cargo check --workspace` | PASS | Ran |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran |
| `cargo test --workspace` | PASS | Ran (`EXIT:0`) |
| `cargo deny check` | PASS | Ran (`advisories ok, bans ok, licenses ok, sources ok`) |
| Scoped markdown lint | PASS | `markdown-doc lint --path docs/work-packages/20260618-perfdeep01-hillslope-day-frame-scaffold-001 --format plain` (`9 files, 0 errors, 0 warnings`) |
| Diff hygiene | PASS | `git diff --check` produced no findings |

## Follow-on

Stage-0 acceptance is **substantially** met; PERFDEEP02 is authorized **with two carried conditions**
(Claude review, `artifacts/review-claude-independent.md`):

1. **First Stage-1 gate (deferred Stage-0 item):** the real-surface, every-symbol seed/flush round-trip
   (incl. frost/snow/irrigation/MOFE families). The Stage-0 round-trip ran on a synthetic ~20-symbol
   warm-rain fixture (`perfdeep01_h2637_like_warm_rain_surface`), not the package-required *real* H2637
   surface — so family-specific paths are not yet proven on the full symbol set.
2. **Representation:** the frame is the ratified **dense-slot baseline** (`Vec<Option<BoundaryValue>>` by
   `SymbolId` — the PERFARCH03-validated 146× design; spec §4.1 amended), not the "typed schema" wording in
   §Decision above. Typed-field promotion is a deferred open fork, not a Stage-1 requirement.