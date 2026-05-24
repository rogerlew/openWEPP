# PL14S Comparator Run Provenance Manifest

Status: `completed`
Evidence mode: `Static + Ran`

## Static
- Provenance authority artifact:
  - `artifacts/pl14s_provenance_manifest.json`
- Comparator outputs captured for PL14S package scope:
  - `artifacts/h5_wat_semantic_comparator.json`
  - `artifacts/h5_wat_strict_comparator.json` (explicit skipped sentinel for parquet lane)
- Schema markers:
  - replay suite schema: `pl14s-legacy-suite-v1`
  - semantic report schema: `pl14s-semantic-wat-v1`

## Ran
- Candidate emission command:
```bash
target/debug/open_wepp_runner run-hillslope \
  --hillslope-binary target/debug/openwepp-cli-hill \
  --run-dir /tmp/pl14s_candidate_20260524T121758 \
  --run-file case.run \
  --output-dir /tmp/pl14s_candidate_20260524T121758/output \
  --policy strict \
  --manifest-path /tmp/pl14s_candidate_20260524T121758/output/openwepp_hillslope_run_manifest.json
```
- Replay/comparator suite command:
```bash
/tmp/pl14s_venv/bin/python tools/legacy_comparison_suite/run_pl14s_legacy_suite.py \
  --baseline-run-dir /workdir/wepp-forest_260430_baseline/tests/fixtures/delicate_game_pw0 \
  --baseline-binary /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill \
  --baseline-run-file p5.run \
  --candidate-wat /tmp/pl14s_candidate_20260524T121758/output/H5.wat.parquet \
  --output-root /tmp/pl14s_suite_20260524T121832
```
- Execution outcome summary:
  - baseline replay return code: `0`
  - semantic comparator return code: `0`
  - strict comparator: skipped (`required=false`, parquet candidate)
- Hash summary (from persisted provenance):
  - baseline `H5.wat.dat` sha256: `c383b31d42b311f9af9124db2fee1b1905a831b2e533ff63d9d667eafaf7ff83`
  - candidate `H5.wat.parquet` sha256: `5b71252fe6efd44cefa63f46ba7ee2585f719acae4e09f8e9c32690f20444e49`
  - semantic report sha256: `547787c4b382ac2079d8b0c1834e3f359ad76d807b143a7c50e33333a785cd27`
