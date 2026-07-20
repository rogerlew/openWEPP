# CANOPY-PHENOLOGY-02 Native Forest Integration

Status: `ACTIVE — reviewed prototype withdrawn; corrected intake`

Package id: `20260719-canopy-phenology-native-integration-001`

Date: `2026-07-19`

Execution mode: `package-end-to-end`

This ExecPlan is a living document maintained under `docs/codex_exec_plans.md`.

## Purpose / Big Picture

Native deciduous, mixed, and evergreen forest managements must drive daily
canopy, leaf-area, foliar biomass, and litter state from the verified Growing
Season Index (GSI), without fixed Julian leaf-on or leaf-off dates. A completed
run must demonstrate that snow, evapotranspiration, interception, erosion, and
litter/decomposition consumers read the same post-phenology state.

## Objective

Close `CANOPY-PHENOLOGY-02` by ratifying and implementing a native-forest
phenology realization of the existing GSI process kernel. The realization must
carry explicit native YAML operands; distinguish persistent structural cover,
evergreen foliar mass, and deciduous foliar mass; publish daily canopy and LAI;
route leaf-off mass directly to litter; record leaf-on allocation; preserve an
exact daily foliar mass ledger; and remove native forest dependence on the
`jdharv` litter-drop window.

## Authority And Dependencies

- `SC-PLANT-001` revision 21 and the completed
  `20260717-canopy-phenology-gsi-kernel-001` process kernel.
- `SC-RESIDUE-001` dynamic surface-litter and residue-depth authority.
- `SC-INFILE-MANAGEMENT-YAML-001` and the native `ow-lanuse-1` YAML authority.
- Jolly, Nemani, and Running (2005): GSI scales potential LAI and supplies the
  continuous foliar-activity signal.
- Pinned WEPP baseline `grow.for` canopy-cover relation
  `cancov = 1-exp(-bb*live_biomass)` and plant-to-residue transfer lineage at
  commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Operator decision on 2026-07-19: validate Southern Hemisphere symmetry by
  shifting a Northern Hemisphere climate sequence by half a year and negating
  latitude. This is a phase-invariance test, not independent observational
  validation.

## Included Scope

- Contract amendments for the native canopy/LAI/biomass/litter state law,
  typed domains, exact mass ledger, management authority, consumer ordering,
  and phase-shifted SH vector.
- Required native YAML phenology operands and fail-closed parser/runtime
  projection.
- A typed forest-canopy realization in `openwepp-plant-phenology` that maps GSI
  to explicit endpoint-bounded foliar pools, LAI, canopy cover, leaf-on
  allocation, and leaf-off litter transfer.
- Direct-production per-lane GSI state and real use of post-phenology state by
  snow, ET, WB15 interception, litter/decomposition/frost, and erosion-facing
  daily inputs.
- Focused contract, parser, kernel, runner, conservation, chronology, phase,
  and negative tests.
- Assurance impact disposition for held `ASSURE-06`; manuscript refresh remains
  campaign-owned and cannot be represented as increment evidence.

## Excluded Scope

- Compatibility interpretation of legacy cropland-encoded forest inputs.
- Site calibration, fitted thresholds, independent SH observational claims, or
  empirical snow-fidelity promotion.
- Sublimation, longwave radiation, canopy snow interception, melt coefficients,
  frost equations, erosion equations, or public output schema changes.
- A binary GSI `0.5` production switch or any fixed leaf-on/off calendar day.

## Declared Write Set

- `Cargo.lock`
- `crates/openwepp-plant-phenology/**`
- `crates/openwepp-management-schema/src/lib.rs`
- `crates/openwepp-input-contract/src/parsers/management.rs`
- `crates/openwepp-landuse-migrate/src/convert.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/05_projection_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-runner/Cargo.toml`
- `crates/openwepp-runner/src/hillslope/**`
- `gate-policy/v1/impact-map.json`
- `gate-policy/v1/gate-definitions.json`
- `tests/fixtures/infile/management/canonical_forest_nonzero_ow_lanuse_1.man.yaml`
- focused crate and `tests/integration/**` tests required by the intent or
  terminal plan, including gate-policy contract tests
- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md`
- `docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-YAML-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/ROADMAP.md`
- `docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260719-canopy-phenology-native-integration-001/**`

Everything else is read-only. A terminal-plan discovery outside this set
requires a pre-implementation package amendment and review or a truthful hold.

## Scientific State Law To Ratify Before Production Edits

The native management record supplies maximum foliar biomass, maximum LAI,
evergreen foliar fraction, persistent structural cover, and the existing `bb`
canopy coefficient. For daily `g = GSI21`, the foliar activity fraction is
`f = evergreen_fraction + (1-evergreen_fraction)*g`. Evergreen and deciduous
foliar pools are separately observable. Live foliar mass and LAI are their
explicit summer endpoints multiplied by `f`; canopy cover is the greater of
persistent structural cover and the baseline-authoritative
`1-exp(-bb*live_foliar_mass)` relation, bounded by the existing finite canopy
cap. Positive day-to-day foliar mass change is leaf-on allocation; negative
change is leaf-off litter transfer. The exact ledger is
`B_after = B_before + allocation - litter`, with all terms finite and
non-negative. Structural biomass/cover does not transfer seasonally.

This endpoint realization is contract authority only after the canonical
contracts are amended and independently reviewed. Until then it is an intake
proposal and production code must not consume it.

## Conservation / Publication Acceptance

Before production edits, record an operand-lineage table covering every mass,
fraction, LAI, canopy, and litter operand, its units, normalization, authority,
and authoritative/diagnostic status. Tests must separate allocation from litter,
reject a fixed-date transfer and aggregate-live-biomass alias, independently
reconstruct the daily mass ledger, and prove the produced litter reaches the
dynamic residue pool. Exact self-consistency alone is supporting evidence; the
real consumer path and a multi-season no-drift audit are required.

## Phase Plan

1. Freeze required reading, authority map, campaign admission, intent plan,
   operand lineage, and contract amendments.
2. Add contract-derived tests and record the pre-implementation contract gate.
3. Implement typed schema/parser/projection and the standalone realization.
4. Integrate per-lane state into the direct production runner and replace the
   native forest fixed-date litter path.
5. Prove downstream snow/ET/interception/residue/frost/erosion reads, NH
   ordering, phase-flipped SH symmetry, chronology, restart, no drift, and mass
   closure.
6. Reconcile the exact terminal diff, execute every selected critical gate,
   complete dual independent review and finding disposition, rerun invalidated
   gates, and complete dual terminal verification.

## Acceptance Criteria

- Native YAML without a complete phenology block fails closed before runtime.
- Deciduous winter canopy is structural-floor limited, mixed canopy retains an
  evergreen foliar floor, and evergreen canopy is seasonally invariant.
- GSI remains continuous; `0.5` is diagnostic only.
- Every daily foliar mass change satisfies the exact allocation/litter ledger,
  leaf-off litter enters the residue pool that drives residue depth, and no
  `jdharv` window carries the native-forest claim.
- A repeated phase-forcing sequence returns to identical endpoint state without
  year-over-year drift.
- The phase-shifted NH sequence at negated latitude produces the corresponding
  SH leaf-on/leaf-off phase within one-day calendar-transform tolerance.
- Snow, ET, WB15 interception, erosion-facing canopy, and residue/frost paths
  consume the post-phenology daily state; old static/fixed-date paths do not
  carry the closure claim.
- Every selected increment gate passes with direct current evidence; no required
  gate is retroactively deferred.
- Dual independent reviews, finding disposition, and dual verification close
  with no undispositioned finding.

## Gate Intent

This is a `critical` increment because it activates a production consumer,
changes native YAML semantics, changes cross-domain plant/residue state, and
touches conservation-sensitive mass transfer. Use the repository TESTGATE
planner before implementation and reconcile it against the exact terminal
diff. Expect affected contract/parser/plant/runner/integration gates plus
campaign-strength workspace regression, doctests, placeholder scan, dependency
policy, full coverage/global CRAP, documentation checks, and assurance impact.

## Security Impact

No secret, network, authentication, unsafe-code, subprocess-interpolation, or
external authority fixture change is intended. YAML remains
`deny_unknown_fields`; missing or invalid authority fails closed.

## Subagent Authorization And Independence

Subagent authorization: this package explicitly authorizes spawning/delegating
to two independent read-only reviewers, two independent read-only terminal
verifiers, and one read-only `comparator_suite_runner` for TESTGATE-selected
heavy workspace/coverage/CRAP commands. Expected outputs are compact findings,
gate metrics/log paths, and verification verdicts written by the parent into
package artifacts. Review agents must not read each other's initial findings.
Subagents have no production write access.

## Progress

- [x] (2026-07-19) Reconstructed Increment 3 authority and direct consumer path.
- [x] (2026-07-19) Accepted operator-selected phase-flipped NH forcing as the
  SH symmetry test, without observational-validation overclaim.
- [x] (2026-07-19) Froze the base-commit authorization scaffold and began the
  machine intent plan for the declared critical write set.
- [x] (2026-07-19) Withdrew the first implementation attempt in commit
  `0692cec7` after independent review proved that its machine intent plan had
  admitted documentation only. That attempt is prototype evidence, not
  implementation admission or closure evidence.
- [x] (2026-07-20) Expanded the corrected intake before reimplementation to
  cover gate-policy A1 bindings and runtime-value consumer evidence discovered
  by the failed exact-diff terminal attempt.
- [x] (2026-07-20) Admitted corrected critical intent plan `f307138c...` from
  base `a749ed7a` and the exact prospective path manifest before reimplementation.
- [ ] Freeze contracts, tests, and the pre-implementation gate.
- [ ] Implement native YAML, typed realization, runner state, and consumers.
- [ ] Execute selected gates and conservation/consumer audits.
- [ ] Complete independent review, disposition, verification, and closure.

## Surprises & Discoveries

- Observation: direct production already passes one post-growth canopy state to
  snow, ET, interception, and daily publication, while litter is derived from
  live-biomass decline and optionally delayed by `jdharv`.
  Evidence: `00c_day_input_builder_impl.rs` and
  `00d_authority_runtime_impl.rs`; integration can replace one producer and one
  fixed-date branch rather than duplicate consumers.
- Observation: the first TESTGATE terminal attempt failed A0 admission before
  checking per-surface bindings and failed workspace Clippy on exact floating
  comparisons in tests; seven independent nodes passed and three dependent
  nodes were blocked.
  Evidence: `/tmp/canopy02-testgate-53f47dba/receipt.json` for withdrawn
  prototype head `53f47dba`.
- Observation: independent prototype review rejected aggregate `vdmt` as an
  initial foliar boundary, source-only downstream proof, a half-year-only SH
  GSI comparison, zero `bb`, inactive native schedule gaps, and silent negative
  VPD normalization.
  Evidence: `artifacts/review_agent_a.md` and `artifacts/review_agent_b.md`.

## Decision Log

- Decision: treat branch/stem area as a persistent structural-cover floor in
  this package rather than defer it to snow interception.
  Rationale: snow, interception, and erosion already consume a single canopy
  cover fraction; excluding the floor would knowingly publish zero winter
  canopy for leafless deciduous stands.
  Date/Author: 2026-07-19 / Codex.
- Decision: use the operator-selected half-year phase transform for the SH gate.
  Rationale: it directly tests hemisphere/calendar symmetry while preserving
  the forcing sequence and avoids claiming unavailable independent SH data.
  Date/Author: 2026-07-19 / operator and Codex.
- Decision: recover through an auditable reverse commit and corrected
  pre-implementation intake rather than represent a terminal plan as
  retroactive implementation authority.
  Rationale: the canonical gate lifecycle says an absent effective intent plan
  blocks implementation admission. Commit `0692cec7` restores the scaffolded
  source state without rewriting history; corrected implementation starts only
  after a new admitted intent plan.
  Date/Author: 2026-07-20 / Codex.

## Outcomes & Retrospective

Pending execution.

## Idempotence And Recovery

All edits are local and additive until the native consumer cutover. TESTGATE
uses a fresh external artifact directory per run. Failures retain evidence and
the package remains active or held; no destructive rollback is authorized.
