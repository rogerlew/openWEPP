# SNOWDENSITY-05D Opt-In CoE Melt Implementation

Status: complete.

Package type: contract-first production opt-in implementation package.

Primary contract: `SC-SNOWFREEZE-001`.

Closure target: COMPLETE-05D-OPT-IN-COE-MELT.

Objective: wire `coe_shortwave_albedo_v1` into the existing production CoE
snowmelt path without changing default `legacy_coe` behavior. The only accepted
production physics delta is the shortwave `amelt` operand consuming the typed
05C albedo state. Existing `bmelt`, `cmelt`, `dmelt`, rain-on-snow accounting,
signed raw melt, daily redistribution, density gate, and routed `wmelt`
publication lineage remain unchanged.

## Completion Summary

Closed: COMPLETE-05D-OPT-IN-COE-MELT.

SNOWDENSITY-05D amended `SC-SNOWFREEZE-001` to v79 and implemented the
contracted opt-in CoE melt selector. The runner and compatibility/default path
still choose `legacy_coe`; no parser, CLI, external forcing, output schema,
coefficient fitting, or default activation was added. The opt-in path applies
`(1 - snow_albedo)` only to the `amelt` shortwave operand and carries typed
albedo state plus raw/redistributed/routed melt lineage for independent
reconstruction.

All required gates passed after one honest fix to a stale contract-version guard
in an older SNOWDENSITY-02 test.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` sections 2, 4, 5, 7, 10
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- SNOWDENSITY-05A/05B/05C packages and worker handoffs
- Existing CoE melt implementation in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`

## Scope

- Amend `SC-SNOWFREEZE-001` before production wiring.
- Add typed opt-in fields required to choose `SnowMeltModel` and carry
  `SnowAlbedoState`.
- Use the 05B radiation source as-is. No snow-only scalar, scaling, clipping,
  reinterpretation, or fitted radiation multiplier is allowed.
- Use the 05C albedo state only when
  `snow_melt_model = coe_shortwave_albedo_v1`.
- Keep `legacy_coe` as the default, compatibility comparator, and rollback
  path.
- Expose enough typed melt-lineage totals to reconstruct raw melt,
  redistributed melt, routed `wmelt`, SWE loss, and downstream liquid forcing
  without inferring from SWE/depth/density alias surfaces.

## Non-Scope

- No default activation of `coe_shortwave_albedo_v1`.
- No parser, CLI option, output schema, or external forcing provider change.
- No coefficient fitting or site-specific tuning.
- No promotion of `dense_slow_melt_v1`.
- No changes to density physics except as needed to preserve existing gates.

## Acceptance Gates

- `legacy_coe` default path is identity for targeted snowmelt scenarios.
- Active opt-in snow with missing prior albedo state fails typed when no fresh
  snow reset supplies state.
- Opt-in `amelt` independently reconstructs as
  `0.0607 * hrad_mj_m2 * (1 - snow_albedo) * (1 - cancov)`.
- Signed raw melt reconstructs as
  `0.0254 * (amelt + melt_bmelt_in + cmelt + dmelt)`.
- Redistributed melt, routed `wmelt`, SWE loss, WB12 `S`, and WB13 liquid
  forcing close from typed operands and do not alias runtime snow storage.
- Required Rust gates pass or the package closes `HOLD` with the blocker and
  evidence.

## Evidence Plan

- Focused failing test before implementation where practical.
- Focused integration tests for contract markers, legacy identity, opt-in
  reconstruction, missing-state fail-closed behavior, and direct-runtime carry.
- Static source scan for radiation tuning and negative benchmark promotion.
- Required workspace gates:
  `cargo fmt --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`,
  `cargo deny check`.

## Completion Discipline

Do not close this package merely because the selector compiles. Closure requires
the reconstruction gate to pass with explicit evidence. If any invariant remains
unproven, close `HOLD` and document the exact missing operand or blocker.

## Closeout Artifacts

- `artifacts/contract-amendment-evidence.md`
- `artifacts/implementation-evidence.md`
- `artifacts/reconstruction-evidence.md`
- `artifacts/contract-test-evidence.md`
- `artifacts/gate-results.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/review-disposition.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`
