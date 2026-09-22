# COLD-CANOPY-M1 unchanged original60 — prelaunch correctness 01

**Verdict: RELEASED for the single bounded original60 physical selector, subject to its exact-source/binary/support launch custody.** This is constitutive-solve and reservoir-closure evidence only; it supplies no native receiver, owner staging, complete-parent, cycle, or M1 acceptance.

**Evidence class: Static.** Source aggregate `67b684300cb32e8a317559d6282bd2e6bd3f91139df667e12c96611b1936c971`; selected preserved executable SHA-256 `a9d8f98ee77fd750d54a1251889e585f640f79d719a810454f502c9269154fb6`. I did not execute the selector or edit source/configuration.

## Findings

No prelaunch numerical-correctness blocker was found for the exact selector `land_surface_energy_shadow::m1_coupled_expected_red::m1_original_sixty_second_solve_has_original_geometry_active_cold_shade_and_real_closure` under its existing 180-second physical bound.

## Static assessment

The complete `m1_coupled_expected_red.rs` file is byte-identical to the authorized retained reconstruction, SHA-256 `c9b0d76baa113575606692f717869345dfc9e9573fc1fef0a458e7e08d44a6d2`. The selected test at `:829-862` parses the frozen original input, passes it through the actual public `solve_m1_coupled_column`, supplies the two declared cycle-zero reservoirs and 60-second conditions, and asserts:

- exact 60-second support;
- active cold shade gas behavior (`ExactZeroPar`, negative net assimilation, positive vapor flux) for both occupancies;
- exact beginning M/H, finite and changed ending M/H, and nonzero vapor/non-vapor heat operands;
- independent reservoir mass and enthalpy reconstruction through `independently_reconstruct_reservoirs` at `:539-560`, including agreement with reported residuals and absolute closure limits `1e-9 kg m-2` and `1e-6 J m-2`.

`covered_input` at `:139-143` deserializes the actual frozen `CoveredColumnInputs`; conditions and topology at `:144-177` bind the authorized two occupancies and six soil layers. The original input remains SHA-256 `4b324b0a9c136f2203e8073ca260db5be9e7dd07722a37aec60f2cd528be2d54`.

A directory comparison against the authorized reconstruction found no land-surface-energy source difference except the read-only controller-policy accessor in `numerics.rs` and its hidden export in `lib.rs`. The accessor returns the existing `MAX_NEWTON_ITERATIONS` and `MAX_BACKTRACKING_HALVINGS` constants and is not called by this selector or solver path. No solver, constitutive equation, fixture, tolerance, or original60 predicate changed.

## Residual risk and evidence boundary

This release authorizes one exact invocation only. Runtime PASS/FAIL, source/binary pre/post binding, cwd/environment/support manifest, timeout classification, and actual elapsed time belong to the launch evidence. The concurrently observed provider-control mutation-test failure is a separate nonphysical control discriminator and does not change the unchanged original60 source assessment; launch sequencing remains with root/QA.

Even a passing result establishes only a numerical root and the assertions above. It does not prove accepted support materialization, native receiver behavior, rollback, seven-owner staging, parent completion, 72-hour cycles, restart, or performance acceptance.

## Reviewed identities

- original60 test source: `m1_coupled_expected_red.rs` SHA-256 `c9b0d76baa113575606692f717869345dfc9e9573fc1fef0a458e7e08d44a6d2`
- LSE numerics with read-only accessor: SHA-256 `d143e0987b3605979668a4429b3bb2e71f13a1d203adb71d6a16fb232f51975a`
- LSE library export: SHA-256 `98d64dffa4f8d96fbd1e79223eb1543214d1e90c78eb26ef09c659049ab08f4b`
- source recorder: `provider-parent-body-review-03-source.json` SHA-256 `c6663c1fc40b1b4b05b6b3128f021d806b71a4b12d5439ba3cbb29aab1baf396`
- preserved executable: SHA-256 `a9d8f98ee77fd750d54a1251889e585f640f79d719a810454f502c9269154fb6`
