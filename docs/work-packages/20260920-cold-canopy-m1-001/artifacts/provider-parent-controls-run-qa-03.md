# Provider-parent controls run QA — 03

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static manifest/source inspection and inspection of the supplied Ran build/list receipt. This reviewer did not execute the selected Rust controls.

## Findings

No blocking QA finding for the exact selected nonphysical control run.

The selector resolves to eight private `m1_provider_` controls, including `m1_provider_actual_seven_owner_bytes_match_independent_canonical_projections`. Static inspection places the added test solely on retained-fixture decoding, actual caller initialization, and byte-map comparison across both cycles. Across all eight selected controls, there is no support advancement, execution, or constitutive solver invocation.

## Exact binding

- Run support manifest: `provider-parent-controls-run-support-03.json`, SHA-256 `aac77d99a34902173a2ac84cf1854596bc6e1fb6dd15c9ffac9800a7d3824092`.
- Detached source snapshot: 759 entries, SHA-256 `67b684300cb32e8a317559d6282bd2e6bd3f91139df667e12c96611b1936c971`.
- Selected binary: `/tmp/openwepp-cold-canopy-m1-target/debug/deps/openwepp_hillslope_orchestrator-0ad665558c5e4d7c`, SHA-256 `a9d8f98ee77fd750d54a1251889e585f640f79d719a810454f502c9269154fb6`; direct rehash agrees with the manifest.
- The manifest has 623 pins, including the binary and the body03 source, build receipt, and list stdout. The inspected body03 receipt records a successful detached-cwd `nextest list` build/list in `47.07535186` seconds using the same Nix environment, target directory, jobs setting, and source aggregate.
- The manifest binds a 180-second, nonphysical command:

  ```text
  nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(m1_provider_)' --no-fail-fast --success-output immediate --failure-output immediate
  ```

## Non-blocking follow-up

The selected command has not yet run. Its result will cover only these eight admission/initialization controls. `cargo fmt`, Clippy with warnings denied, broader tests, deny, physical execution, and final parent acceptance remain separate missing evidence.

## QA disposition

**PASS — release the exact manifest-bound 180-second nonphysical eight-control run.**
