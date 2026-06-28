# SNOWDENSITY-10.3.15 Default Activation Under Active Cap

Status: complete  
Package id: `20260627-snowdensity-10-3-15-default-activation-active-cap-001`  
Owner: Codex  
Execution mode: package-end-to-end

## Objective

Activate the validated active-cap snow-depth bundle as the direct-production
default:

- `snow_melt_model = coe_liquid_holding_capacity_v1`
- `snow_density_model = physics_bulk_density_compaction_v1`
- active runtime density cap remains `522 kg m^-3`

The activation must preserve explicit rollback/test selectors, compatibility
rollback, fixture inputs, output schemas, parser/runfile/user CLI surfaces, and
frost-attribution blocking while snow-control residuals remain.

## Rationale

SNOWDENSITY-10.3.14 closed with
`READY-FOR-ACTIVATION-PACKAGE-UNDER-ACTIVE-CAP`: the bundle reduced paired
snow-depth failures from `1147/1415` to `498/1415`, passed full workspace gates
under the bundle selectors, and stayed bounded by the active `522 kg m^-3`
density cap. This package performs the separate ratified default activation and
proves the downstream direct-production path consumes the new default without
requiring selector environment variables.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/work-packages/20260627-snowdensity-10-3-14-policy-b-no-regression-cap-authority-001/artifacts/worker-handoff.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

Conditional:

- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`

On-demand:

- `tools/snowfreeze_observed/bundle_activation_adjudication.py`
- `tools/snowfreeze_observed/policy_b_no_regression_cap_authority.py`
- `tests/integration/snowdensity10_3_14_policy_b_no_regression_cap_authority.rs`

## Included Scope

- Amend `SC-SNOWFREEZE-001` contract-first with default-activation authority.
- Convert the existing package-bound melt/density environment variables into
  explicit rollback/test selectors.
- Make absent selector variables choose the validated bundle by default.
- Reject unsupported default-activation selectors rather than silently retaining
  rejected candidates.
- Add contract-derived tests and a package diagnostic proving no-env default
  selection reaches the real direct-production WAT/trace consumer path.
- Record rollback/default isolation evidence and residual snow-control/frost
  blocking in package artifacts.

## Excluded Scope

- No `550 kg m^-3` density-cap change.
- No new density-rate acceleration or spring-densification promotion.
- No open-surface ablation work.
- No frost attribution or frost default activation.
- No Qwet/frzftp implementation or selector.
- No fixture, observation, parser, runfile, public output-schema, or user CLI
  surface changes.
- No site-specific constants or calibration.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/20260627-snowdensity-10-3-15-default-activation-active-cap-001/**`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
- `tools/snowfreeze_observed/default_activation_active_cap.py`
- `tests/integration/snowdensity10_3_15_default_activation_active_cap.rs`
- targeted integration tests that encode superseded default-isolation wording
- `Cargo.toml`

## Phase Plan

1. Phase 0: scaffold package artifacts, required-reading map, and pre-edit
   contract gate.
2. Phase 1: amend `SC-SNOWFREEZE-001` v101 with activation authority,
   rollback/test selector semantics, protected boundaries, and residual
   disposition.
3. Phase 2: add contract-derived tests and diagnostic tooling for no-env
   default evidence plus explicit rollback evidence.
4. Phase 3: implement default activation in the direct-production selector
   path.
5. Phase 4: run diagnostic, focused tests, full gates, source scans, dual
   reviews, disposition, verification, and closeout.

## Exit Criteria

- Contract version is bumped and records default activation authority.
- No-env direct-production path selects `coe_liquid_holding_capacity_v1` and
  `physics_bulk_density_compaction_v1` by default.
- Explicit rollback/test env values still select `legacy_coe` and
  `legacy_wepp`.
- Rejected spring densification is not accepted by the active default selector.
- Direct trace evidence proves the no-env default reached the real WAT consumer
  path.
- Public parser/runfile/user CLI surfaces and WAT/output schemas are unchanged.
- The active runtime density cap remains `522 kg m^-3`.
- Known residuals are carried forward: `498/1415` paired snow-depth failures
  remain under the active-cap bundle; frost attribution remains blocked.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`, and `cargo deny check` pass.
- Source-level anti-evasion guards are run or explicitly justified as not
  applicable.
- Dual independent reviews, finding disposition, dual verification,
  line-count governance, and worker handoff are complete.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to two read-only review subagents for implementation and
evidence review. Expected outputs are `artifacts/review_agent_a.md` and
`artifacts/review_agent_b.md`; write access is read-only. Local execution may
substitute direct Codex review artifacts when subagent dispatch is unavailable.

## Security / Safety Gate

This package must fail closed on unknown selector values. It must not add new
user-facing selectors, alter fixture inputs, loosen typed guards, hide water or
snow-state conservation residuals, or claim frost attribution while snow-depth
control residuals remain.

## Final Disposition

`COMPLETE-DEFAULT-ACTIVATED-UNDER-ACTIVE-CAP`.

SNOWDENSITY-10.3.15 activates the active-cap bundle by default for the
direct-production no-env path, preserves explicit `legacy_coe`/`legacy_wepp`
rollback/test selectors, rejects unsupported active selector values, and leaves
the active density cap at `522 kg m^-3`. The package diagnostic records
`112,502` no-env trace rows selecting both activated members and `13,880`
rollback trace rows selecting legacy members on the representative rollback
surface.

Known residual: `498/1415` paired snow-depth rows still fail the snow-control
gate; frost attribution remains blocked by `SNOW-CONTROL-RESIDUALS-REMAIN`.
