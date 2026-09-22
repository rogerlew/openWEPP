# Provider-parent controls run QA — 01

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static manifest/source inspection; inspection of the existing Ran build/list receipt. This reviewer did not execute Rust or the selected test command.

## Findings

No blocking QA finding for the proposed selected, nonphysical control run.

## Exact run binding

- Support manifest: `provider-parent-controls-run-support-01.json`, SHA-256 `96b819a70d105b43b46bdc18a1500c3fd8836ce3e56d3b85ae507f603c55c0c8`.
- Detached source snapshot: 759 entries, aggregate SHA-256 `5cfa36252496195644931d63d359bb253bf2ff1783fb1f5af41fe2b01bbabc25`.
- Selected executable: `/tmp/openwepp-cold-canopy-m1-target/debug/deps/openwepp_hillslope_orchestrator-0ad665558c5e4d7c`, SHA-256 `9db3559ca5070f4422a8c729bef6e73974fd991d498009bc6e7fdb650b968fde` (rehash agrees with the manifest).
- The manifest pins the preceding detached build/list source receipt, result receipt, and stdout. The inspected build/list receipt binds the same detached source root, `CARGO_TARGET_DIR`, `CARGO_BUILD_JOBS=2`, Nix development environment, and package target. Its recorded elapsed time is `63.821386883` seconds and it lists seven `m1_provider_` tests.
- The selected command is bounded to 180 seconds and is exactly:

  ```text
  nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(m1_provider_)' --no-fail-fast --success-output immediate --failure-output immediate
  ```

The manifest records the detached source root as its working directory and marks the activity nonphysical. Its support links remain the declared existing links; the selected executable and prior build/list receipts are separately pinned, so this is a source-and-binary-bound test invocation rather than an ambient rebuild selection.

## Selected-test scope

The selector resolves to the seven provider controls in `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_fixed_sequence_provider_controls.rs`:

1. real source encoding and parent-slice oracle;
2. both-cycle expansion across frozen records, parent days, and phases;
3. contract-mutation refusals before owner staging;
4. missing, foreign, reordered, and stale GSI-receipt refusals;
5. independent canonical ASCII/escape vectors;
6. refusal preservation of real-caller owners, clock, staging, and publication; and
7. actual-caller refusal for wrong-cycle and out-of-sequence parents.

Static call inspection confines these controls to retained-byte/source-adapter validation, payload and receipt admission, owner-byte initialization/snapshot checks, and coupled-clock construction/admission. The selected control module has no invocation of support advancement, execution, or a constitutive/physical solver. The run therefore exercises the intended admission controls only; it is not evidence of physical model execution, parent staging/commit completion, or final package acceptance.

## Non-blocking follow-up

The standard broader workflow gates (`cargo fmt`, Clippy with warnings denied, full relevant tests, and deny) remain unrepresented by this narrow selected control run. They require their separately scoped evidence at package disposition.

## QA disposition

**PASS — release the exact manifest-bound, 180-second nonphysical selected-control run.** A PASS result from that run will establish only these seven controls; it does not replace the outstanding broader acceptance evidence.
