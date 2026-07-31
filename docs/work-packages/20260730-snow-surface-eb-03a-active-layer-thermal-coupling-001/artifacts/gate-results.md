# Gate Results

Status: `complete / pass`

Evidence mode: `Ran`

## Direct Package Gates

- `cargo fmt --all -- --check`: PASS.
- `cargo nextest run --test snow_surface_eb03_contract --test
  snow_surface_eb03_runtime --test
  paradigm2_stage3_liquid_routing_meltwater_temperature`: PASS, 20/20.
- Terminal combined EB-03A plus affected Stage 0/3 guards: PASS, 24/24.
- `cargo nextest run -p openwepp-meteorology`: PASS, 22/22.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- strict Binding Exposure: PASS for both touched contracts.
- SC unit compliance: PASS for both touched contracts.
- raw unit-conversion guard on the new meteorology implementation/error
  surface: PASS.
- real direct-production B absent/empty/B/L/S/LS consumer: PASS.
- assurance source adoption: PASS; the touched snow/freeze source was adopted.
- tracked human-review rendering: PASS, 92/92 files current.
- assurance `validate --all`: PASS, three DRAFT and zero public reports.
- `markdown-doc lint --no-ignore`: PASS for 27 package files, the campaign
  roadmap, and both touched contracts.
- `git diff --check`: PASS.

## Domain And Workspace Profiles

- Exact frost profile after the Stage 0 source-boundary allowlist correction:
  PASS, 324/324, one slow, 1,863 skipped, `541.391 s`.
- The first frost inventory run was 323/324; its only failure was the stale
  allowlist. The corrected test passed alone before the complete passing
  rerun.
- Quick profile: NOT PASS as a full workspace gate. It completed 181 tests
  before `cqr_quality_evidence_self_test_passes` failed because its generated
  “valid exact-head fixture was not CURRENT”; 1,915 tests were cancelled by
  fail-fast. The same CQR self-test reproduced alone in `122.630 s`.
- ADR-0043 Critical full profile, run
  `52f3e791-935b-4f8e-9cb5-4e82bdf3e8c3`: NOT PASS. Exact argv was
  `cargo nextest run --workspace --profile full` at base HEAD
  `53ff5854f2b870c742dc74998e3393e8512dbc59` with the inherited EB-03 plus
  EB-03A dirty tree. Two assurance publication tests timed out at the
  configured `720 s`:
  `authority_lifecycle_and_bound_byte_negative_matrix_is_fail_closed` and
  `bootstrap_narrative_empty_directory_and_symlink_drift_fail_closed`. After
  the non-pass was definitive, the continuing inventory run was interrupted
  to avoid hours of additional publication-fixture work. Terminal
  summary: 195/2,146 tests started, 191 passed, two timed out, two interrupted,
  29 skipped, and 1,951 not run; elapsed `1,598.956 s`. No separate durable
  stdout log was configured; this artifact preserves the terminal capture.

The quick failure occurs in the CQR evidence-handoff fixture rather than a snow
test. The full timeouts occur in assurance publication tests, and this package
amends assurance inputs; dependency independence was not established. Neither
result is relabeled as unrelated or waived. Because quick and full are current
package gates, both non-passing results force terminal `HOLD` and prevent
EB-04 admission.

## Hold-Lift Evidence

SNOW-SURFACE-EB-03B preserves the historical non-pass evidence above and
corrects both blockers under separate authority. Terminal exact-tree results:

- quick `ad35d09e-b7cd-4698-823c-27d7ee375230`: PASS 2109/2109;
- frost `f8669779-a747-447f-abb3-8364d9ab3e12`: PASS 324/324; and
- Critical full `bd84eb5d-358d-45ac-961f-ee248e02a55e`: PASS 2158/2158.

The EB-03A terminal validation hold is lifted; the package now passes.
