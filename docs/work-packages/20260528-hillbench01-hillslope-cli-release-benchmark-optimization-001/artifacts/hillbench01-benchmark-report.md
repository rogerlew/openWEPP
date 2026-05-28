# HILLBENCH01 Benchmark Report

Status: complete  
Evidence mode: ran  
Date: 2026-05-28

## Scope
- single-OFE release benchmark lane (`p111`)
- multi-OFE release benchmark lane (`p324`)
- baseline compare against `wepp_260430_hill`

## Benchmark Harness
- Script:
  - `artifacts/hillbench01_release_benchmark.py`
- OpenWEPP command shape:
  - `target/release/openwepp-cli-hill --run-dir <lane>/runs --run-file <lane>_openwepp.run --output-dir <lane>/output --policy compat`
- Baseline command shape:
  - `(cd <lane>/runs && /workdir/wepp-forest_260430_baseline/release/wepp_260430_hill < <legacy-runfile>)`
- Runs:
  - warmups: `2`
  - measured repetitions: `12`
- Raw benchmark captures:
  - `artifacts/hillbench01-pre-optimization-benchmark.json`
  - `artifacts/hillbench01-post-optimization-benchmark.json`

## Pre-Optimization Medians
- `single_p111`
  - openWEPP: `0.567147 s`
  - baseline: `0.105816 s`
  - openWEPP/baseline ratio: `5.3597`
- `multi_p324`
  - openWEPP: `0.605059 s`
  - baseline: `0.169932 s`
  - openWEPP/baseline ratio: `3.5606`

## Post-Optimization Medians
- `single_p111`
  - openWEPP: `0.485230 s`
  - baseline: `0.106464 s`
  - openWEPP/baseline ratio: `4.5577`
- `multi_p324`
  - openWEPP: `0.540308 s`
  - baseline: `0.171589 s`
  - openWEPP/baseline ratio: `3.1488`

## Delta Summary
- openWEPP median improvement:
  - `single_p111`: `-14.44%`
  - `multi_p324`: `-10.70%`
- baseline medians stayed within run variance bands across pre/post runs.
