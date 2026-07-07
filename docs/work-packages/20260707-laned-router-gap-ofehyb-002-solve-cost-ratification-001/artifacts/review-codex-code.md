# Codex Code Review - GAP-OFEHYB-002 Exact Bare-Skin Evaluator

Evidence: Static + Ran.

Ran:
- `git diff --check` - PASS.
- `cargo test -p openwepp-hillslope-orchestrator bare_skin_direct_equilibrium -- --nocapture` - PASS, 3 tests.
- `cargo test -p openwepp-hillslope-orchestrator branch_ -- --nocapture` - PASS, 10 tests.

## Findings

### High - direct bare-skin path can mask invalid inactive roughness/vegetation operands instead of failing closed

`SC-OFEROUTE-001` requires finite roughness-domain operands before equation evaluation and hard failure for invalid/non-finite/out-of-domain operands (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:210`, `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:230`). `SC-OFEROUTE-002` rev 4 also binds the exact evaluator to the same effective-addend guards as the friction equations (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:189`).

The new predicate classifies a cell as bare using exact zero checks on only the addend-enabling fields (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:250`), and the direct evaluator then checks only `flow_depth_m`, `slope`, `skin_rain_term`, and `k_o` for finiteness (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:278`). In the hybrid implicit path, `route_single_ofe_hybrid` validates width/time/window shape but does not validate every `CellParameters` field before calling `implicit_step_with_discharges` (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:472`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:566`). `implicit_step_with_discharges` checks dimensions, timestep, upstream inflow, and state/source values, but not cell-parameter domain (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:109`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:136`). By contrast, the explicit solver wrapper calls `cell.validate()` (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1338`).

That means an invalid but "inactive" operand can be silently ignored by the exact evaluator. Example class: `roughness_concentration == 0.0` with `element_tip_height_m = NaN`, or `LAI == 0.0` with non-finite canopy/vegetation fields, can satisfy `is_bare_skin_only()` and return a finite algebraic discharge. The contract requires fail-closed domain handling, not canonicalize-and-proceed. Either the implicit production entry needs the same local `CellParameters::validate()` guard as the explicit run path, or the direct evaluator/predicate needs to prove and enforce the complete finite/non-negative domain before it can bypass the generic friction functions.

### High - package evidence is not sufficient for closure, ratification, or promotion claims

The package requires focused tests, Case-4 full-hybrid ladder, H2637 timing/profile, before/after solve-cost counters, fidelity/timing ratification, full Rust closure gates, `cargo deny`, and line-count governance (`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/package.md:217`). Its exit criteria require the solve-cost lever to be tested, Case-4 to remain passing, H2637 before/after counters to be recorded, ratification or explicit no-promotion disposition, and all gates to be PASS or explicitly non-applicable (`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/package.md:244`).

The current gate table still marks every required gate as `NOT RUN` (`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/gate-results.md:7`). The implementation, timing/fidelity, and ratification artifacts are placeholders (`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/implementation.md:1`, `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/timing-and-fidelity.md:1`, `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/ratification-audit.md:1`).

There is raw after-effective H2637 timing evidence showing the intended counter movement (`implicit_equilibrium_map_evaluations: 0`, user `33.37s`) (`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid-after-effective-time.log:1`), but it is not dispositioned against the required gates. It also shows `solver_steps` changed from the baseline `7381407` to `7381405` (`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/baseline-profile.md:50`, `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-active-hybrid-after-effective-time.log:1`) and active-output hashes changed for `H2637.hbp` and `H2637.pass.parquet` (`docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/baseline-profile.md:62`, `docs/work-packages/20260707-laned-router-gap-ofehyb-002-solve-cost-ratification-001/artifacts/h2637-scratch-after-effective/output/openwepp_hillslope_run_manifest.json:39`). That may be a tolerable numeric consequence of replacing tolerance-limited fixed-point iteration with the algebraic fixed point, but it is comparator-sensitive and needs a recorded magnitude/fidelity audit under `SC-OFEROUTE-002#INV-OFEHYB-008` (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:390`) before closure.

## Residual Risk And Missing Tests

- Algebra check: no arithmetic finding. The Shen-Li fixed point `q = 8 g S h^3 / ((rain_term + k_o) nu)` and Hirsch fixed point `q = [sqrt(8 g S / 3.19) * nu^-0.225 * h^1.5]^(1/0.775)` match the current friction formulas (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs:106`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:298`).
- `Re <= 1000` vs `> 1000`: no code finding. The direct validity checks use `q_low <= Q_c` and `q_high > Q_c`, matching the friction dispatch (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/friction.rs:111`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:308`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:320`). Add an exact-boundary regression at `q_low == Q_c` if this package continues.
- Determinism/seed-side behavior: no code finding from static review or focused tests. The branch solver still accepts only finite/positive/on-side warm seeds (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/implicit_recession.rs:234`), and the direct evaluator preserves seed-side preference when both fixed points are in-regime (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:286`, `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:322`).
- Missing validation evidence remains blocking: Case-4 full-hybrid oracle ladder, focused source-memory/hybrid suite, full workspace `nextest`, clippy, fmt, deny, markdown/contract checks, line-count governance, and a ratification/no-promotion audit are not recorded as passing.

## GO / NO-GO

NO-GO.

The core algebra is defensible, and the narrow focused tests I ran pass, but the fail-closed guard gap and absent package closure evidence block acceptance. Do not close `GAP-OFEHYB-002`, ratify `INV-OFEHYB-008`, or promote the selector from this state.
