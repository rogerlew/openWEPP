# Provider-parent body QA — 03

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static inspection of the frozen body03 source, source/build manifests, and supplied compiler receipt. This reviewer did not execute Rust, tests, or physical work.

## Findings

No blocking QA finding for the body03 build/list gate.

The reported owner-oracle defect is mechanically corrected in `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs:722-744`: the independent constructor now reads the numeric fixture fields with `as_f64()` before constructing the M1 reservoirs. This matches the fixture's JSON-number encoding and lets the both-cycle, seven-owner comparison reach its intended byte comparison. The correction remains test-only and introduces no provider-body, manifest, source-adapter, solver, support-advance, or physical-execution path.

## Binding and gate assessment

- Frozen detached source: 759 entries, SHA-256 `67b684300cb32e8a317559d6282bd2e6bd3f91139df667e12c96611b1936c971`, preserved in `provider-parent-body-review-03-source.json`.
- Proposed build/list manifest: `provider-parent-body-build-support-03.json`, SHA-256 `29301de10e3ccbeec7317a73235262f03007ccbfa434675a1f606a3281fbbf4f`; it binds that source aggregate, the unchanged 619 support pins, existing links, detached source root, Nix development environment, `CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target`, and `CARGO_BUILD_JOBS=2`.
- The unchanged command is `cargo nextest list -p openwepp-hillslope-orchestrator --lib --message-format json` under that environment, bounded to 180 seconds and declared nonphysical.
- Supplied `provider-parent-body-compile-05.stderr` records a successful lib-test compile in 23.6 seconds. It also reports the existing crate warning set, so it is compile evidence only, not fmt/Clippy evidence.

## Non-blocking follow-up

No body03 build/list execution receipt exists yet. It must be recorded before treating the listed binary as built. The selected controls have also not run. `cargo fmt`, Clippy with warnings denied, broader tests, deny, and final parent/physical acceptance remain missing evidence.

## QA disposition

**PASS — release the exact source-bound, nonphysical 180-second build/list gate.** This rebind supersedes body02 only for the numeric-fixture control repair and does not retroactively clear the prior unexecuted source02 gate.
