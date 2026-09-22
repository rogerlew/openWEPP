# Provider-parent original60 physical-run QA — 01

**Reviewer:** `/root/provider_qa` (secondary QA)  
**Evidence:** Static source/manifest inspection and inspection of the supplied body03 build/list receipt. This reviewer did not execute the physical command.

## Findings

No blocking QA finding for the exact original60 physical test.

The selected test is the exact-name selector for `m1_original_sixty_second_solve_has_original_geometry_active_cold_shade_and_real_closure` in `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/m1_coupled_expected_red.rs:830`. Its direct path parses the pinned original input, constructs `M1CoupledColumnInput`, calls `solve_m1_coupled_column`, checks the active cold-shade state and reservoir operands, then independently reconstructs mass and enthalpy closure. The reconstruction uses the solved operands and explicit closure equations. It does not call the fixed-sequence provider, native receiver, caller support advance, parent staging, or parent commit.

The currently held provider mutation control is a separate no-op-mutation test defect. It does not invalidate this narrow existing original60 solver test or change its selector. It continues to block any claim that provider controls or parent acceptance have passed.

## Exact binding

- Physical-run manifest: `provider-parent-original60-run-support-01.json`, SHA-256 `47a47303539703641f4aeabba6552e2d193fddf681a99af0db3aca399e5118eb`.
- Detached source snapshot: 759 entries, SHA-256 `67b684300cb32e8a317559d6282bd2e6bd3f91139df667e12c96611b1936c971`.
- Selected binary: `/tmp/openwepp-cold-canopy-m1-target/debug/deps/openwepp_hillslope_orchestrator-0ad665558c5e4d7c`, SHA-256 `a9d8f98ee77fd750d54a1251889e585f640f79d719a810454f502c9269154fb6`; direct rehash agrees with the manifest.
- The 623-pin manifest includes the body03 source/build/list receipts, original input (`4b324b0a9c136f2203e8073ca260db5be9e7dd07722a37aec60f2cd528be2d54`), continuous fixture, and the module's reference inputs. The supplied body03 receipt binds the same detached source, Nix environment, target directory, and jobs setting, and records a successful build/list in `47.07535186` seconds.
- The exact physical command is bounded to 180 seconds:

  ```text
  nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo nextest run -p openwepp-hillslope-orchestrator --lib -E 'test(=land_surface_energy_shadow::m1_coupled_expected_red::m1_original_sixty_second_solve_has_original_geometry_active_cold_shade_and_real_closure)' --no-fail-fast --success-output immediate --failure-output immediate
  ```

## Non-blocking follow-up

This command has not run. Its result will be evidence for one existing physical original60 solver and closure check only. It is not provider-admission, receiver, staging/commit, or package-acceptance evidence. The provider control HOLD, fmt, Clippy with warnings denied, broader tests, deny, and final parent acceptance remain missing evidence.

## QA disposition

**PASS — release the exact source-and-binary-bound original60 physical prelaunch command.**
