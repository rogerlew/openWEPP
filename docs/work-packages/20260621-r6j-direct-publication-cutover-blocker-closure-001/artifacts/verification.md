# R6J Verification

Evidence class: Static plus Ran.

## Gate Table

| Gate | Status | Evidence |
| --- | --- | --- |
| Reproduce inherited R6I fail-closed marker | PASS | Pre-correction `cargo test -p openwepp-runner r6i_cutover_candidate_clears_pmet_layer_ulp_then_fails_manifest_cutover -- --nocapture` passed as an expected fail-closed marker. |
| Manifest direct projection writer cutover | PASS | `r6j_cutover_candidate_writes_direct_outputs_and_manifest` and CLI evidence show `/execution_provenance/publication_source = direct-publication-frame`, `/wb13_publication/source = direct-publication-frame`, replay candidate count `0`, and direct row counts. |
| HBP parity, current fixture | PASS | `cargo test -p openwepp-runner r6j_cutover_parity_evidence_covers_hbp_wat_pass_and_loss -- --nocapture` compares direct HBP bytes and parsed latest-event payload to compatibility. |
| WAT parity, current fixture | PASS | Same focused test requires empty reduced WAT mismatch fields and full row equality; disk Parquet evidence is read back and compared. |
| PASS parity, current fixture | PASS | Same focused test requires empty PASS mismatch fields and full row equality; disk Parquet evidence is read back and compared. |
| Loss parity, current fixture | PASS | Same focused test compares direct loss JSON to compatibility loss JSON. |
| Manifest parity/checksum reconstruction, current fixture | PASS | Runner test recomputes every output checksum from disk and compares to manifest `output_checksums`; explicit CLI run `/tmp/r6j_evidence_oLbost` independently produced matching direct provenance. |
| Public direct output writes, current fixture | PASS | `cargo test -p openwepp-runner --test r6_direct_publication_cutover_cli_contract -- --nocapture` and explicit CLI run wrote HBP, loss, PASS parquet, WAT parquet, plot, and manifest. |
| Independent reconstruction, current fixture | PASS | Direct HBP/WAT/PASS/loss/manifest surfaces are reconstructed from `DirectRunPublicationFrame`; compatibility builders are used only by focused tests. |
| No-compatibility proof | PASS | Static production writer scan plus runtime counters: cutover writer consumes `DirectPublicationArtifacts`; manifest counters show `skeleton_runs = 0` and `compatibility_edge_invocations = 0`. |
| Default-disabled isolation | PASS | `cargo test -p openwepp-runner r2a_default_fixture_run_constructs_no_direct_runtime_skeleton -- --nocapture` passed; H2637 default-disabled release reps passed below. |
| H2637 default-disabled timing/RSS | PASS | Release reps: `635.10 s / 229228 KiB`, `631.16 s / 229236 KiB`, `631.32 s / 228788 KiB`; median `631.32 s <= 676.67 s`. |
| H2637 opt-in direct cutover endpoint/RSS | PASS | Fresh release direct cutover completed in `637.53 s / 349400 KiB`; HBP/WAT/PASS/loss/plot are byte-identical to fresh same-binary default output; manifest source and counters are direct-only. |
| `cargo fmt --check` | PASS | Ran with `cargo check`/clippy chain after final code edits. |
| `cargo check -p openwepp-runner -p openwepp-hillslope-orchestrator` | PASS | Ran after final code edits. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after splitting the expanded R6J test and fixing needless borrow. |
| `cargo test --workspace` | PASS | Full workspace suite passed after final code edits. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `git diff --check` | PASS | Ran after final documentation edits. |
| Markdown doc lint | PASS | `markdown-doc lint --path docs/work-packages/20260621-r6j-direct-publication-cutover-blocker-closure-001 --no-ignore`, `--path docs/work-packages/README.md --no-ignore`, and `--path docs/ROADMAP.md --no-ignore` each reported 0 errors and 0 warnings. |

## Explicit CLI Evidence

Current-fixture direct cutover:

```text
temp_dir=/tmp/r6j_evidence_oLbost
stderr_lines=0
H5.hbp 1654
H5.loss.json 339
H5.pass.parquet 7075
H5.plot.parquet 199
H5.wat.parquet 14503
openwepp_hillslope_run_manifest.json 7454
publication_source=direct-publication-frame
wb13_source=direct-publication-frame
row_count=2
publication_area_m2=1800.0
compatibility_edge_invocations=0
```

H2637 default-disabled release gate:

```text
release_build  59.82  1130080
target/release/openwepp-cli-hill sha256=543358575e4adfca11472e1d917083a0843216930da9947fada6093e983f5dd5
r6j_h2637_default_rep1  635.10  229228
r6j_h2637_default_rep2  631.16  229236
r6j_h2637_default_rep3  631.32  228788
r6j_h2637_default_after6 640.41 227396
```

H2637 opt-in direct cutover:

```text
pre_correction_r6j_h2637_direct_cutover  1399.94  442844
row_count=235961
publication_area_m2=206522.26699999993
compatibility_edge_invocations=0
r6j_h2637_direct_cutover_after6 637.53 349400
H2637.hbp MATCH
H2637.wat.parquet MATCH
H2637.pass.parquet MATCH
H2637.loss.json MATCH
H2637.plot.parquet MATCH
wat_left=235961 wat_right=235961 wat_left_minus_right=0 wat_right_minus_left=0
pass_left=12419 pass_right=12419 pass_left_minus_right=0 pass_right_minus_left=0
direct_manifest_publication_source=direct-publication-frame
direct_manifest_compatibility_edge_invocations=0
```

## Verification Notes

- H2637 opt-in cutover is functionally live, direct-manifested, and
  parity-clean against fresh same-binary default output. WAT remains
  per-OFE-day (`235961` rows), PASS remains outlet-day (`12419` rows), and both
  match default output exactly.
- Protected default-disabled output comparison against the retained
  `/tmp/perfdeep07/default/rep1/h2637_same` baseline: HBP and WAT hashes match;
  PASS row equivalence via DuckDB passes with `12419` rows and zero
  bidirectional differences. Loss/plot differ in `precipitation_mm` against
  that older retained baseline, matching current default output behavior and
  not an R6J direct-cutover path change.
