# PL14S Comparator JSON Artifact Index

Status: `completed`
Evidence mode: `Static + Ran`

## Static
- Canonical persisted JSON artifacts for PL14S:
  - `artifacts/h5_wat_semantic_comparator.json`
  - `artifacts/h5_wat_strict_comparator.json`
  - `artifacts/pl14s_provenance_manifest.json`

## Ran
- Artifact hashes:
  - `h5_wat_semantic_comparator.json`
    - sha256: `547787c4b382ac2079d8b0c1834e3f359ad76d807b143a7c50e33333a785cd27`
  - `h5_wat_strict_comparator.json`
    - sha256: `7ea87b31953946558ddf39ff9009d913a61c358a8addbe432f038fe888915c37`
    - content posture: strict comparator sentinel (`status=skipped`, parquet lane)
  - `pl14s_provenance_manifest.json`
    - sha256: `76fb2500518932e6264b2118fbf5c28d636cb184a804a5021d54d1cad2f2a1a4`
- Strict comparator lane status:
  - `skipped=true`, `required=false`, reason recorded in provenance.
