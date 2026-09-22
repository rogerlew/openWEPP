# Provider-parent body QA — 04

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static inspection of frozen source and build manifest plus inspection of supplied compiler receipt. This reviewer did not execute Rust, tests, or physical work.

## Findings

No blocking QA finding for the body04 nonphysical build/list gate.

The held mutation control's no-op case is repaired in `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs:1221-1247`. For every override field, it parses the existing binary64 hex encoding and replaces it with `original ^ 1`; `mutate_payload` then asserts that the canonical payload bytes changed before exercising provider refusal. This is a compact, guaranteed semantic mutation that preserves the test's intent and avoids relying on an assumed nonzero rain rate.

The reviewed correction remains in controls only. It has no provider-production, source-adapter manifest, solver, support-advance, or physical-work call-path effect.

## Binding and gate assessment

- Detached source snapshot: 759 entries, SHA-256 `a717cc043b64603c19238642f5db0c1aefc09577ca2d0907dcede15eeaedfdd2`, preserved by `provider-parent-body-review-04-source.json`.
- Reviewed control source SHA-256: `fe8c6155704b800dabd22eb9ef0437f9b7bc41f0587b525677edfa5504df1422`.
- Build/list manifest: `provider-parent-body-build-support-04.json`, SHA-256 `abc7180d5f27dd57a6fac96ed90f08a89dc414c0208e5957cc173d5e0a32d389`; it binds the source aggregate, unchanged 619 pins and support links, detached source root, Nix environment, `CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target`, and `CARGO_BUILD_JOBS=2`.
- The selected operation remains a 180-second, nonphysical `cargo nextest list -p openwepp-hillslope-orchestrator --lib --message-format json` command under that environment.
- Supplied `provider-parent-body-compile-06.stderr` records successful lib-test compilation in 23.7 seconds. Its existing warnings mean this is compile evidence only, not fmt/Clippy evidence.

## Non-blocking follow-up

The source04 build/list receipt and the repaired selected control run are not yet present. fmt, Clippy with warnings denied, broader tests, deny, and final parent/physical acceptance remain missing evidence.

## QA disposition

**PASS — release the exact source-bound nonphysical body04 build/list gate.** Earlier clearances remain limited to their frozen source cuts.
