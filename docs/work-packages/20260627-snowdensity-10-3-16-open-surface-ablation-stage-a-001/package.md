# SNOWDENSITY-10.3.16 Open-Surface Ablation Stage A

Status: executed-non-promotion  
Package id: `20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001`  
Owner: Codex  
Execution mode: package-end-to-end

## Objective

Add and adjudicate an opt-in CoE-melt candidate whose only algorithmic delta is
a turbulent latent-heat / sublimation SWE sink. The candidate targets the
cap-limited over-persistence and mass-excess tail concentrated at wind-exposed
open surfaces (`harvard_open`, `sleepers_south_field`) after the activated
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1` bundle.

Stage A only: this package does not add the full SNOBAL two-layer surface
structure. It tests only whether a clean-room Marks-lineage latent mass-loss
term can improve the open
surface mass tail without introducing shallow-pack under-persistence or water
balance leaks.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.2 item 7 and
  section 10.3
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`

Conditional:

- `docs/standards/kernel-work-package-preparation.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `tests/AGENTS.md`

On-demand:

- `references/copyrighted/marks1999.pdf`
- `references/copyrighted/source_pdfs/marks1998.pdf`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `tools/snowfreeze_observed/*`

## Authority Envelope

- Canonical contract authority: `SC-SNOWFREEZE-001`, amended before production
  edits.
- Physical authority: Marks/SNOBAL energy and mass balance lineage. Marks 1999
  confirms input forcing (`ea`, wind, temperature), energy/mass outputs
  (`LvE`, `Es`), exposed-site dry/windy sublimation behavior, and open-surface
  ablation sensitivity. Marks 1998 provides the explicit energy-balance equation
  `DQ = Rn + H + LvE + G + M` and defines `E` as evaporation/condensation mass
  flux from the snow surface.
- PySnobal/libsnobal is equation-reference only; do not read PySnobal/libsnobal C
  source unless a
  non-GPL-family license is confirmed against `deny.toml`; absent local license
  metadata means C source is not implementation authority for this package.
- Turbulent-transfer constants must come from cited paper authority or a
  contract-listed physical constant, never fixture fitting.

## Included Scope

- Add `SC-SNOWFREEZE-001` authority for Stage A:
  `coe_open_sublimation_stage_a_v1`.
- Add contract-derived tests for:
  - opt-in selector and fail-closed handling;
  - default activated bundle remains default;
  - sublimated mass is tracked separately from routed liquid melt;
  - no parser/runfile/user CLI or output schema changes.
- Implement a typed, opt-in direct-production melt selector. The candidate may
  subtract bounded snowpack SWE as vapor and publish the sink to runtime trace
  and internal water-balance operands.
- Add diagnostic tooling to run real coupled direct-production WAT/trace over
  the observed snow-depth fixture ladder and classify:
  - open-surface cap-limited over-persistence tail;
  - under-persistence guardrail;
  - sublimation magnitude;
  - whole-model snow-state conservation.
- Update package artifacts with evidence, reviews, verification, and final
  disposition.

## Excluded Scope

- No default activation.
- No two-layer surface structure, SNOBAL active-layer thermal state, or
  two-layer SNOBAL surface implementation.
- This is a Stage A-only package with no two-layer surface structure.
- No density cap change.
- No new snow-density model.
- No albedo, radiation, canopy, phase, rain-heat, longwave, frost, Qwet/frzftp,
  parser/runfile/user CLI, fixture, compatibility-runtime, or public output
  schema changes.
- No site-specific constants, observed-depth fitting, or coefficient tuning.
- No frost attribution.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `tools/snowfreeze_observed/open_surface_ablation_stage_a.py`
- `tests/integration/snowdensity10_3_16_open_surface_ablation_stage_a.rs`
- `Cargo.toml` if a new integration test target is required

## Phase Plan

1. Scaffold package and evidence placeholders.
2. Amend `SC-SNOWFREEZE-001` contract-first with Stage A authority, obligations,
   acceptance gates, and protected boundaries.
3. Add contract-derived tests and pre-implementation evidence.
4. Implement the selector, sublimation mass sink, trace/internal WB fields, and
   snowbench parsing support.
5. Add and run the coupled diagnostic over real direct-production WAT/trace.
6. Run focused tests, full package gates, source scans, dual reviews,
   verification, line-count governance, and final disposition.

## Exit Criteria

Current-scope gates:

- Contract version is bumped and carries Stage A authority.
- Default no-env path remains `coe_liquid_holding_capacity_v1` plus
  `physics_bulk_density_compaction_v1`.
- Explicit rollback still selects `legacy_coe` and `legacy_wepp`.
- Candidate selector is opt-in only and fails closed on unsupported values.
- Coupled direct-production WAT/trace evidence proves the candidate reached the
  real snow partition.
- Gate 1: candidate reduces the open-surface cap-limited over-persistence tail.
- Gate 2: candidate does not worsen the under-persistence tail.
- Gate 3: sublimation magnitude is in a literature-defensible range and is not
  fixture-tuned.
- Gate 4: whole-model snow-state conservation closes; sublimated mass leaves as
  vapor and does not become routed melt or disappear from the ledger.
- Public output schema, parser/runfile/user surfaces, fixtures, density cap, and
  frost attribution remain unchanged.
- Focused and workspace gates are run or the package closes `HOLD` with the
  missing gate named:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

If any current-scope gate is missing, fails, or worsens, this package closes as
`HOLD` or non-promotion. It must not activate the candidate.
If any gate is missing or worse, close HOLD/non-promotion.

## Closure Disposition

The package closes `NON-PROMOTION-STAGE-A-GATE-NOT-MET`. The opt-in Stage A
candidate reached the real direct-production snow partition, reduced the
open-surface cap-limited tail, stayed within the provisional sublimation
magnitude envelope, and closed snow-state conservation. It also worsened the
open-surface under-persistence tail, so the bidirectional guardrail failed and
the candidate is not activation-eligible.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only review subagents for implementation and evidence review.
Expected outputs are `artifacts/review_agent_a.md` and
`artifacts/review_agent_b.md`; write access is read-only. Local Codex-authored
independent review artifacts may substitute when subagent dispatch is not used.

## Security / Safety Gate

Unknown selector values must fail closed. Sublimation mass must be finite,
non-negative, bounded by available snowpack SWE, and tracked as vapor loss in
trace/internal conservation artifacts. The package must not use PySnobal C
source without confirmed permissive licensing and must not fit any constant to
the observed fixture cohort.
