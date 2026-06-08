# 20260608-refactor018-openwepp-input-contract-parsers-hbp-mechanical-modularization-001

## Status
- state: queued
- date: 2026-06-08
- timezone: UTC

## Objective
Mechanically modularize
`crates/openwepp-input-contract/src/parsers/hbp.rs` into cohesive modules
while preserving the public API surface, internal behavior, and all
downstream consumer expectations.

## Why This Package Exists
`crates/openwepp-input-contract/src/parsers/hbp.rs` is a single mixed-concern
production file at 2095 lines, exceeding the `.rs` 2000+ warning threshold for
required refactor. The file contains at least seven distinguishable concern
layers: public types, error types, private internal types, a `Cursor` reader
abstraction, utility helpers, a large binary layout parser (~843 lines), payload
validation (~384 lines), path resolution helpers, and public entry-point
functions. This package reduces review and maintenance risk by splitting these
concerns into dedicated module files without intended semantic drift.

## Scope
### Included
- Conversion of `crates/openwepp-input-contract/src/parsers/hbp.rs` into
  `crates/openwepp-input-contract/src/parsers/hbp/mod.rs` (thin facade/re-export
  wiring entrypoint).
- Mechanical movement of cohesive concern groups into dedicated module files
  under `crates/openwepp-input-contract/src/parsers/hbp/`.
- No change to `crates/openwepp-input-contract/src/parsers/mod.rs` — the
  `pub mod hbp;` declaration resolves to `hbp/mod.rs` transparently after the
  file move; the seam is stable.
- Validation and evidence updates demonstrating no intended behavior changes.

### Explicitly Out of Scope
- New parsing logic, format support, or error-handling behavior changes.
- Threshold/guard loosening or canonicalize-and-proceed handling.
- Public API changes (signatures, visibility, re-export paths) unless explicitly
  declared and approved.
- Changes to any downstream consumer crate.

## Candidate Concern Split
The following is a starting-point inventory for Phase A seam freeze; the worker
should verify line ranges and rename modules as judgment dictates:

| Module file (candidate) | Content group | Approx. lines |
|-------------------------|---------------|---------------|
| `types.rs` | All `pub` Hbp* structs and non-error enums | ~39–275 |
| `error.rs` | `HbpFormatErrorCode`, `HbpParseError` + Display/Error impls | ~176–276 |
| `internal_types.rs` | Private `YearEntry`, `DirectoryEntry`, `EntryPayload`, `PayloadBlockEntry`, `Layout` | ~288–352 |
| `cursor.rs` | `struct Cursor<'a>` + all cursor reader methods | ~353–471 |
| `helpers.rs` | Utility free functions: `format_violation`, `map_cursor_err`, `crc32c`, `expected_state_schema`, `expected_dims`, `decode_zlib_block`, `u64_to_usize`, `scaled_i64_to_f64`, `key_in_year_table`, `validate_year_table` | ~473–662 |
| `layout_parser.rs` | `fn parse_layout` (the largest single concern, ~843 lines) | ~663–1505 |
| `payload_validator.rs` | `struct PayloadValidationResult`, `fn validate_payload` | ~1506–1889 |
| `path.rs` | `has_forbidden_pass_suffix`, `resolve_path` | ~1890–1928 |
| `mod.rs` (facade) | `parse_hbp_from_bytes_internal` + 4 `pub fn` entry points, module declarations, `pub use` re-exports | ~1929–2095 + wiring |

The worker may merge or further split modules as long as each `.rs` file stays
within line-count governance and the public API surface is preserved.

## Deliverables
1. Mechanical modularization implementation with preserved API and behavior:
   - `crates/openwepp-input-contract/src/parsers/hbp/mod.rs`
   - `crates/openwepp-input-contract/src/parsers/hbp/*.rs`
2. Work-package artifacts:
   - `artifacts/required-reading-map.md`
   - `artifacts/refactor018-modularization-plan-report.md`
   - `artifacts/refactor018-public-api-surface-parity-report.md`
   - `artifacts/refactor018-contract-implementation-evidence.md`
   - `artifacts/refactor018-contract-test-implementation-evidence.md`
   - `artifacts/refactor018-preimplementation-contract-gate.md`
   - `artifacts/refactor018-implementation-and-test-evidence.md`
   - `artifacts/refactor018-kernel-profile-compliance-checklist.md`
   - `artifacts/refactor018-line-count-governance-checklist.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/refactor018_disposition.md`
   - `artifacts/worker-handoff.md`
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through final
disposition without user intervention unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:` sections.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/standards/mechanical-refactor-authoring-guide.md`
- `/workdir/openWEPP/docs/prompt_templates/mechanical-refactor-kickoff-template.md`
- `/workdir/openWEPP/docs/prompt_templates/required-reading-map-template.md`
- `/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/workdir/openWEPP/crates/openwepp-input-contract/Cargo.toml`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/mod.rs`
- `/workdir/openWEPP/crates/openwepp-input-contract/src/parsers/hbp.rs`

## Intended Write Set
- `docs/work-packages/20260608-refactor018-openwepp-input-contract-parsers-hbp-mechanical-modularization-001/**`
- `docs/work-packages/README.md`
- `crates/openwepp-input-contract/src/parsers/hbp.rs`  (deleted — replaced by hbp/mod.rs)
- `crates/openwepp-input-contract/src/parsers/hbp/mod.rs`
- `crates/openwepp-input-contract/src/parsers/hbp/*.rs`

## Phase Plan
### Phase A - Intake, Sizing, and Surface Freeze
- Capture pre-refactor public API inventory (all `pub` symbols) and line-count
  baseline.
- Verify `parsers/mod.rs` seam: confirm `pub mod hbp;` resolves transparently
  to `hbp/mod.rs` after the file move (no change required).
- Freeze concern-split boundaries and module naming.

### Phase B - Mechanical Extraction
- Move `hbp.rs` → `hbp/mod.rs` as a first step; confirm compilation baseline.
- Extract concern groups into dedicated module files under `hbp/`.
- Wire `mod.rs` with module declarations and `pub use` re-exports to preserve
  all public symbols at their original import paths.
- Preserve signatures, visibility, and any `#[allow]`/`#[cfg]` attributes
  present in the source.

### Phase C - Validation and Evidence
- Run required validation gates and record truthful outputs.
- Capture post-refactor public API inventory and diff against pre-refactor
  baseline for the parity report.
- Complete dual review and dual verification artifacts.

### Phase D - Disposition
- Publish final disposition, parity result, and residual-risk ownership.

## Contract-First Sequencing Requirement
Contract-first sequence remains mandatory for kernel-adjacent package posture:
1. canonical contract amendments,
2. contract-derived tests,
3. pre-implementation contract gate,
4. production edits.

For this package, no canonical contract amendments are expected because this
is mechanical decomposition only with no intended behavior changes. Artifacts
must explicitly record this determination before production edits.

## Exit Criteria
- `hbp.rs` → `hbp/mod.rs` conversion is complete; `hbp.rs` is deleted.
- All public API symbols remain accessible at their original import paths.
- Every new `.rs` file under `hbp/` is within `.rs` line-count governance.
- Required gates pass:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test -p openwepp-input-contract`
  4. `cargo test --workspace`
  5. `cargo deny check`
- Gate commands above are mandatory execution requirements; omission is only
  permitted when a hard blocker is recorded with command-level evidence.
- Required artifacts are complete with truthful `Static`/`Ran` evidence.
- Review findings are fully dispositioned and line-count governance is
  documented.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal Rust module organization refactor on a parser crate;
  no new external interface, no behavior change, no new trust boundary.
