# SNOWDENSITY-10.3.20 Sublimation Diagnosis and Stage B Unlock

Status: executed-non-promotion  
Package id: `20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001`  
Owner: Codex  
Execution mode: package-end-to-end

## Objective

Diagnose why `coe_open_sublimation_stage_a_v1` scored worse on the
cross-SNOTEL forcing-robust rubric, test the current default plus sublimation
composition, and unlock an opt-in Stage B sublimation candidate with
SNOBAL-lineage active surface-layer temperature/cold-content structure.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.2 item 7 and
  section 10.3 step 8 decisions 4-5
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-050`, `INV-SNOWFREEZE-073`, and the 10.3.19 addendum
- `docs/work-packages/20260627-snowdensity-10-3-16-open-surface-ablation-stage-a-001/`
- `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/`
- `docs/work-packages/20260628-snowdensity-10-3-19-harder-pomeroy-default-activation-001/`
- `references/copyrighted/marks1999.pdf`

## Authority Envelope

- Canonical contract authority: `SC-SNOWFREEZE-001`, amended before production
  edits.
- Primary gate authority: `INV-SNOWFREEZE-050` cross-SNOTEL forcing-robust
  rubric over five SNOTEL climates plus the cancov paired set.
- Stage A authority: `INV-SNOWFREEZE-073`.
- Stage B physical authority: Marks 1999 two-layer SNOBAL snowcover structure,
  surface-layer temperature/cold content, latent mass flux, and active
  surface-layer depth.
- libsnobal provenance: local clone `/home/workdir/pysnobal` commit
  `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`; `setup.py` declares
  `license="CC0 1.0"`. `deny.toml` allow-lists `CC0-1.0` and excludes
  GPL-family licenses by omission.

## Included Scope

- Diagnose Stage A degradation by magnitude, site, signature, and residual
  component.
- Score the current default partition+sublimation composition with Stage A.
- Add an opt-in `coe_open_sublimation_stage_b_v1` candidate whose only delta
  from Stage A is the SNOBAL-lineage active surface-layer temperature/cold
  content gate for the sublimation surface vapor pressure.
- Preserve the current default activated bundle plus Harder-Pomeroy phase and
  explicit rollback selectors.
- Emit per-candidate cross-SNOTEL rubric profiles and conservation evidence.

## Excluded Scope

- No default activation unless the candidate beats the current default on the
  primary cross-SNOTEL rubric and passes conservation.
- No public output-schema, fixture, density-cap, frost, parser/runfile/user CLI,
  Qwet/frzftp, or compatibility-runtime changes.
- No site calibration, observed-row-conditioned runtime behavior, or fixture
  tuning.
- No `.run` disable option.

## Intended Write Set

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/**`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`
- `tools/snowfreeze_observed/sublimation_stage_b_unlock.py`
- `tests/integration/snowdensity10_3_20_sublimation_stage_b_unlock.rs`
- `Cargo.toml`

## Phase Plan

1. Scaffold package and evidence artifacts.
2. Amend `SC-SNOWFREEZE-001` contract-first.
3. Add contract-derived guards and opt-in Stage B implementation.
4. Add and run the cross-SNOTEL diagnostic/composition tool.
5. Record gate results, reviews, verification, line-count governance, and final
   disposition.

## Execution Log

- [x] Scaffolded package and required-reading evidence.
- [x] Amended `SC-SNOWFREEZE-001` v105 with `INV-SNOWFREEZE-076`,
  `OBL-SNOWFREEZE-P-051`, `REF-SNOWFREEZE-SNOWDENSITY1020`, and
  `REF-SNOWFREEZE-LIBSNOBAL-CC0`.
- [x] Added opt-in `coe_open_sublimation_stage_b_v1` selector and Stage B
  surface-layer sublimation branch.
- [x] Ran the real cross-SNOTEL direct-production WAT/trace diagnostic.
- [x] Recorded non-promotion disposition: Stage A composition and Stage B do not
  beat the current default.

## Exit Criteria

- Contract version is bumped and carries 10.3.20 authority.
- Current no-env default and rollback selectors are preserved.
- `coe_open_sublimation_stage_b_v1` is opt-in only and fails closed on
  unsupported selector values.
- Stage A degradation is diagnosed by site, signature, magnitude, and residual
  component.
- Partition plus sublimation composition is scored on the cross-SNOTEL rubric.
- Stage B candidate is scored on the same rubric.
- Any promotion requires candidate robust fail count no worse than current
  default, robust ordinal score better than current default, bidirectional
  guardrail non-worsening, and conservation closure.
- If no candidate beats the current default or conservation fails, close
  non-promotion or `HOLD`.
- Run or record blockers for:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only review subagents for implementation and evidence review.
Expected outputs are `artifacts/review_agent_a.md` and
`artifacts/review_agent_b.md`; write access is read-only. Local Codex-authored
independent review artifacts may substitute when subagent dispatch is not used.

## Security / Safety Gate

Unknown selectors must fail closed. Sublimation must be finite, non-negative,
bounded by available snowpack SWE, tracked as vapor, and excluded from routed
liquid. The package must not fit any coefficient or threshold to the observed
fixture cohort.

## Closure Disposition

`NON-PROMOTION-GATE-NOT-MET`.

Current default remains `coe_liquid_holding_capacity_v1 +
physics_bulk_density_compaction_v1 + harder_pomeroy_hourly` (`15` robust fails /
`179` robust score). Partition plus Stage A sublimation scored worse (`19` /
`168`). Stage B conserved vapor and phase mass but did not beat the default
(`15` / `178`) and worsened three robust cells, so it remains opt-in diagnostic
only. No activation, fixture, output-schema, density-cap, frost,
parser/runfile/user CLI, `.run` disable, Qwet/frzftp, compatibility-runtime, or
site-calibration change is authorized.
