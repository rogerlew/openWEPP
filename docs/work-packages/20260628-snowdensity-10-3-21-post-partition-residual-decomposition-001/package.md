# SNOWDENSITY-10.3.21 Post-Partition Residual Decomposition

Status: executed-diagnostic  
Package id: `20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001`  
Owner: Codex  
Execution mode: diagnostic-only package

## Objective

Decompose the remaining `15` forcing-robust rubric failures on the current
no-env default (`coe_liquid_holding_capacity_v1 +
physics_bulk_density_compaction_v1 + harder_pomeroy_hourly`) across the
cross-SNOTEL plus `cancov_forest` observed corpus, and produce evidence for the
operator's frost-attribution-threshold decision without deciding or unblocking
frost.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 10.3 step 8
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
  `INV-SNOWFREEZE-050`
- `docs/decisions/0028-observed-data-admission-authority.md`
- `docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/`
- `docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/`
- `docs/work-packages/20260628-snowdensity-10-3-20-sublimation-stage-b-unlock-001/artifacts/claude-review-mechanism-family-exhausted.md`
- `tests/fixtures/snotel_observed/`
- `tests/fixtures/cancov_forest/`

## Authority Envelope

- Canonical evaluation authority: `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-050`.
- Candidate-admission framing for any later new mechanism: ADR-0028.
- Comparator posture: ADR-0017; legacy and PySnobal are flags, not targets.
- No new gate authority is expected or added in this package.

## Included Scope

- Decompose post-partition current-default forcing-robust fail cells by site,
  climate, signature, residual component, and persistence direction.
- Compare the fail set against the 10.3.18 pre-partition activated profile.
- Report mass/SWE, depth, and density residual directions directly from the
  observed corpus quantities.
- Classify residual clusters as forcing-limited/irreducible or as pointing to a
  possible new mechanism class: canopy snow interception/sublimation,
  sub-canopy longwave, or wind redistribution.
- Produce frost-attribution-threshold input only.

## Excluded Scope

- No production/default/cap/schema/fixture/frost change.
- No new runtime selector, parser/runfile/user CLI option, or `.run` option.
- No candidate promotion, activation, frost unblock, or threshold decision.
- No site calibration, fixture fitting, or observed-data-conditioned runtime
  behavior.
- No SC contract version bump unless the diagnostic unexpectedly needs new
  authority; current execution consumes `INV-SNOWFREEZE-050`.

## Intended Write Set

- `docs/work-packages/20260628-snowdensity-10-3-21-post-partition-residual-decomposition-001/**`
- `docs/work-packages/README.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `tools/snowfreeze_observed/post_partition_residual_decomposition.py`
- `tests/integration/snowdensity10_3_21_post_partition_residual_decomposition.rs`
- `Cargo.toml`
- `tests/integration/snowdensity03_physics_bulk_offline_contract.rs`

## Phase Plan

1. Scaffold package and required-reading evidence.
2. Add diagnostic-only residual decomposition analyzer.
3. Execute analyzer against the 10.3.20 current-default real-run artifact and
   the 10.3.18 pre-partition artifact.
4. Record gate results, review artifacts, line-count governance, and final
   disposition.
5. Run focused validation and hygiene gates.

## Execution Log

- [x] Scaffolded package and required-reading artifact.
- [x] Added diagnostic analyzer and integration guard.
- [x] Ran residual decomposition.
- [x] Recorded reviews, gate results, line-count governance, and disposition.

## Exit Criteria

- Diagnostic report answers which signatures and climates still fail, and
  whether failures are concentrated or diffuse.
- Report decomposes mass/SWE, density, and depth residual directions directly
  from the observed corpus.
- Report splits over- vs under-persistence and states whether the
  under-persistence tail remains binding post-partition.
- Each residual cluster is classified as forcing-limited/irreducible or mapped
  to a possible new mechanism class for later ADR-0028 candidate work.
- Frost-attribution-threshold input is produced without deciding the threshold
  or unblocking frost.
- Protected boundaries remain unchanged: no production/default/cap/schema/
  fixture/frost/selector change and no site calibration.
- Run or record blockers for:
  - `.venv/bin/python tools/snowfreeze_observed/post_partition_residual_decomposition.py`
  - `.venv/bin/python -m py_compile tools/snowfreeze_observed/post_partition_residual_decomposition.py`
  - `cargo fmt --check`
  - `cargo test --test snowdensity10_3_21_post_partition_residual_decomposition`
  - `git diff --check`

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating
to two read-only review subagents for diagnostic evidence review. Expected
outputs are `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`;
write access is read-only. Local Codex-authored independent review artifacts may
substitute when subagent dispatch is not used.

## Security / Safety Gate

The package consumes committed observed-data artifacts and must not modify
fixtures, production physics, user-facing selectors, output schemas, cap
constants, or frost code. Any later mechanism recommendation must be framed as a
future opt-in ADR-0028 candidate, not a current promotion.

## Closure Disposition

`DIAGNOSTIC-COMPLETE-NO-PROMOTION-NO-FROST-DECISION`.

The post-partition no-env default remains `15` robust fails / `179` robust
score. Residual failures are signature-concentrated but site-diffuse:
`seasonal_densification_trajectory` accounts for `9/15` fails, humid-New-
England depth-SWE slope geometry accounts for `2/15`, and mountain SNOTEL
timing under-persistence accounts for `4/15`. The under-persistence tail is
still present, but it is not the sole binding constraint; density-structure
fails dominate the residual count and no over-persistence timing tail remains.

Frost-threshold input read: `MIXED-NO-SINGLE-GLOBAL-SNOW-LEVER`. The evidence
supports an operator threshold decision rather than automatic snow promotion or
automatic frost unblock. No production, default, cap, schema, fixture, frost,
selector, user surface, or site-calibration change is authorized.
