# SNOW-PREPEAK-LIQUID-EVACUATION-PHYSICS-AUDIT

Status: `executed / reviewed / verified / HOLD-EVIDENCE`

Date: `2026-08-03`

Plan class: `Characterization-only science and implementation audit`

This ExecPlan is a living document governed by `docs/codex_exec_plans.md`.
The `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes &
Retrospective` sections must remain current during execution.

## Purpose / Big Picture

Determine whether the roughly `0.48 m` of modeled Snowbird snowpack loss before
the observed annual SWE peak is physically warranted, an implementation defect,
or an interaction among otherwise plausible components. The audit challenges
the complete path from empirical CoE melt generation through thermal state,
liquid retention/refreezing, density evolution, daily redistribution, and
downstream liquid release. It produces a ranked, falsifiable mechanism
disposition without changing production behavior.

## Context And Orientation

The predecessor
`20260803-snowbird-rst-prepeak-flux-diagnostic-001` executed 72 real-consumer
cells. Active Harder-Pomeroy phase partitioning was exactly invariant to `rst`,
and an extreme diagnostic legacy threshold could not recover observed peak SWE.
At Snowbird, the active-model median before the observed peak contained about
`0.670 m` of snowfall accumulation, `0.482 m` of actual pack loss, and
`0.483 m` of liquid release. That establishes the symptom and mass destination,
but it does not identify whether melt production or evacuation is correct.

The frozen predecessor configuration combined:

- `harder_pomeroy_hourly` precipitation phase;
- `coe_liquid_holding_capacity_v1` melt and in-pack liquid capacity;
- `physics_bulk_multilayer_density_v1` density;
- `layered_thermal_liquid_v1` Stage-3 thermal/liquid routing; and
- disabled standalone snow-surface longwave and sublimation selectors.

The existing JSONL trace exposes hourly CoE terms (`amelt`, `bmelt`, `cmelt`,
`dmelt`), signed raw melt, pack loss, retained and released liquid, cold content,
refreezing, layer state, density-process terms, and Stage-3 energy diagnostics.
Those are the primary dynamic evidence. Canonical science contracts, Chapter 3
WEPP documentation, pinned/fixed legacy source, and current Rust call paths are
the primary static evidence.

## Scientific Intent And Evidence Roles

- Intent: `characterization-only mechanism audit`.
- SNOTEL observations retain their prior `CALIBRATION` role and may not be
  represented as independent validation.
- ERA5, PySNOBAL, legacy comparator behavior, and alternate existing-selector
  replays are `DIAGNOSTIC_ONLY` unless an owning authority says otherwise.
- No empirical calibration, parameter recommendation, default promotion,
  production correction, or independent-validation claim is authorized.
- Comparator agreement or disagreement is an investigation signal, not
  standalone correctness authority.

## Included Scope

1. Re-derive the CoE melt terms, units, signs, time bases, caps, and implicit
   surface assumptions from canonical authority and first principles.
2. Trace exact runtime ordering from snowfall and rain partition through snow
   depth/density mutation, raw melt, daily redistribution, Stage-3 thermal and
   liquid routing, refreezing, holding capacity, release, and publication.
3. Independently reconstruct the additive snow-mass identity from primitive
   trace operands and define the valid energy-accounting boundary without
   forcing a false full-energy closure onto an empirical CoE formulation.
4. Test surface-energy interactions involving shortwave, albedo, the SIMIMPL
   cloud proxy, implicit CoE longwave behavior, the optional Stage-3 longwave
   surface, dewpoint, wind, and rain heat.
5. Test pack-structure interactions involving density, the legacy density gate,
   fresh-snow mixing, layer restructuring, cold-content carry, retained-liquid
   capacity, capacity contraction, rain-on-snow, and pack exhaustion.
6. Attribute pre-peak behavior by event and water year for Snowbird and the
   retained Mica Creek, Niwot, and Paradise fixtures. Aggregate medians alone
   cannot carry a mechanism verdict.
7. Use only prospectively frozen, existing-selector controlled replays when a
   replay is necessary to falsify a mechanism. Replay results remain diagnostic
   and cannot be tuned after inspection.

## Excluded Scope

- Production Rust, canonical science contracts, tests, fixtures, observations,
  selectors, defaults, and parameter-domain edits.
- New production instrumentation or trace-schema changes. A missing required
  operand becomes a named instrumentation gap and follow-on, not an audit edit.
- `rst` or hydrometeor-temperature bias correction.
- Coefficient fitting, calibration sweeps, model promotion, or a correction.
- External network access or acquisition of new datasets.

## Intended Write Set

- `docs/work-packages/20260803-snow-prepeak-liquid-evacuation-physics-audit-001/`
- `docs/work-packages/README.md`
- `docs/planning/snow-surface-energy-balance-roadmap.md`
- disposable execution evidence under
  `target/snow_prepeak_liquid_evacuation_physics_audit/`

Terminal write-set amendment (post-review, not prospective): review-required
clean reruns also retained
`target/snow_prepeak_liquid_evacuation_physics_audit_v2/` and
`target/snow_prepeak_liquid_evacuation_physics_audit_v3/`. V2 is rejected
evidence; v3 is the accepted same-binary evidence. Creating sibling namespaces
was necessary to preserve overwrite refusal and rejected-run custody, but the
initial manifest should have authorized versioned disposable namespaces
explicitly.

A second retrospective amendment records the exact recovery namespace
`/tmp/openwepp-snow-prepeak-audit-invalid-rain-label-v1-20260803/`. The
orchestrator moved the invalid rain-label run there before the accepted reruns;
the directory is rejected custody evidence, not a scientific result. The
original manifest should have authorized that bounded recovery path before the
move.

Every other path is read-only. Investigator, reviewer, and verifier subagents
have no filesystem write access; the orchestrator alone integrates their compact
reports into the package write set.

## Authority And Dependencies

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `references/50201000/chap3.pdf`
- `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
- fixed negative-melt comparator authority identified by
  `SC-SNOWFREEZE-001`
- the predecessor package and its checksum-bound trace evidence
- current source under `crates/openwepp-hillslope-orchestrator/`,
  `crates/openwepp-meteorology/`, and `crates/openwepp-runner/`

Package artifacts are evidence and may not replace canonical science authority.

## Investigation Questions

Each mechanism must be challenged for sign, timing, magnitude, cross-site
consistency, and a counter-signature that would falsify it.

1. Which CoE terms generate pre-peak melt, and are their unit conversions,
   implicit albedo/emissivity assumptions, and forcing inputs physically and
   contractually consistent?
2. Does any radiative or thermal input affect melt once empirically and again
   in Stage 3, or is any required cooling/thermal storage pathway omitted?
3. Does operation ordering permit new snow or shallow packs to melt and evacuate
   liquid before physically available cold content or refreezing can act?
4. Do negative raw melt, cold-content carry, daily redistribution, and Stage-3
   refreezing represent compatible physical concepts without omission or
   duplication?
5. Does density or holding-capacity evolution force physically premature liquid
   release, especially during mixed snowfall/melt or rain-on-snow events?
6. Can any proposed mechanism explain an order-one fraction of Snowbird's
   approximately `0.48 m` pre-peak loss and the cross-fixture pattern?

## Subagent Investigation Plan

Subagent authorization: this package explicitly authorizes and requires
spawning/delegating to six independent read-only investigator subagents, two
independent read-only science reviewers, and two read-only terminal verifiers.
The investigators own these bounded scopes:

1. CoE physics, dimensional analysis, handbook equations, and legacy provenance.
2. Stage-3 thermal state, cold content, enthalpy, refreezing, and liquid routing.
3. Runtime call graph, hourly/daily ordering, state mutation, signed melt, and
   publication lineage.
4. Shortwave, cloud, albedo, longwave, temperature, humidity, wind, and rain-heat
   interactions.
5. Density, layers, liquid capacity, capacity contraction, rain-on-snow, and
   pack-exhaustion interactions.
6. Cross-site, cross-year, and event-level trace forensics with magnitude and
   falsification tests.

Expected investigator outputs are compact evidence-classified reports naming
`file:line` support, equations/units, observed signatures, competing
explanations, confidence, and falsification conditions. Reviewers independently
attack the orchestrator synthesis. Verifiers reproduce terminal evidence,
write-set confinement, and finding disposition. Subagent write access is
read-only; the orchestrator writes all package artifacts.

## Deliverables

- `artifacts/audit-freeze.json` and `artifacts/execution-receipt.json`:
  prospective scope/input/operator freeze and exact execution lineage.
- `artifacts/evidence-manifest.json`: source, binary, fixture, observation,
  trace, tool, and replay identities.
- `artifacts/authority-equation-map.md`: equations, variables, units, implicit
  assumptions, contract authority, baseline provenance, and Rust consumers.
- `artifacts/execution-state-map.md`: call/order/state/publication lineage.
- `artifacts/mass-energy-ledger.json`: independent primitive-operand mass
  reconstruction and explicitly bounded energy accounting.
- `artifacts/prepeak-event-attribution.json`: site/year/event term attribution.
- `artifacts/mechanism-matrix.md`: necessary signature, observed signature,
  magnitude, alternatives, falsifier, confidence, and disposition.
- `artifacts/integrated-audit.md`: orchestrator synthesis and ranked next action.
- Direct gate, line-count, review, finding-disposition, verification,
  disposition, and handoff artifacts.

## Phase Plan

### Phase A: Freeze Evidence And Authority

Freeze source and dirty-state identity, predecessor evidence hashes, trace and
fixture inventory, binary provenance, observation roles, intended write set,
questions, replay policy, and allowed claims before result-bearing analysis.
Record a primitive-operand lineage table before accepting any closure result.

### Phase B: Parallel Read-Only Investigation

Spawn all six investigators with non-overlapping primary ownership. The
orchestrator maintains a shared evidence index but does not pre-classify any
mechanism. Contradictory findings remain visible and are not resolved by vote.

### Phase C: Reconstruction And Falsification

Reconstruct daily and observed-peak-window mass identity from primitive trace
operands. Quantify CoE components and thermal/liquid responses by event. Run
only the minimal prospectively frozen existing-selector replays necessary to
separate competing mechanisms. Label non-stateful ablations as bounds rather
than alternate simulations.

### Phase D: Orchestrator Integration

Integrate static and dynamic evidence into the mechanism matrix. A supported
mechanism must have the correct sign, timing, and sufficient magnitude. A
`CONFIRMED_DEFECT` requires mechanistic `file:line` evidence, independent
physical or contract authority, and a reproduced trace signature; comparator
disagreement alone is insufficient.

### Phase E: Review, Verification, And Disposition

Complete two independent science reviews, disposition every finding as
`accepted`, `rejected`, `deferred`, or `follow-up`, remediate accepted
package-evidence defects, and complete two fresh terminal verifications. Close
as characterization only with ranked mechanisms and the smallest justified
next correction or discrimination package.

## Validation And Exit Criteria

- Exact source, binary, configuration, trace, fixture, observation, and tool
  identities are retained for every used result.
- Every melt/thermal/liquid/storage operand has units, sign convention, time
  basis, authority, producer, consumer, and authoritative-versus-diagnostic
  status.
- Daily and pre-peak-window additive mass closure is independently reconstructed
  from primitive outputs with two-sided residuals. Aggregate aliases and
  overlapping diagnostics are explicitly rejected as independent sinks.
- Energy claims state the exact accounting boundary. Empirical CoE melt-depth
  terms are not mislabeled as a complete surface-energy balance.
- Snowbird and all three retained comparison sites receive event/year analysis;
  no pooled or median-only result hides site failure or heterogeneity.
- Each material hypothesis has an order-of-magnitude estimate and explicit
  falsifier, then receives `SUPPORTED`, `EXCLUDED`, `CONFIRMED_DEFECT`, or
  `UNRESOLVED` disposition.
- Production/contract/test/fixture paths remain byte-identical and outside the
  terminal diff. Any needed instrumentation is a concrete follow-up.
- Exact terminal diff is reconciled against declared characterization intent.
- Applicable syntax, JSON, Markdown, source-identity, protected-path,
  overwrite, conservation, reconstruction, and replay-integrity checks pass.
- Dual independent review, complete finding disposition, dual fresh terminal
  verification, and line-count governance pass before `complete`.
- No current-scope gate is deferred after execution; any unmet required gate
  forces truthful `HOLD`.

## Security And Data Impact

Security impact is `none expected`. The package uses local repository files and
existing local evidence only. It must not read or emit credentials, contact
external services, mutate protected observations/fixtures, or place absolute
credential paths in artifacts.

## Progress

- [x] (2026-08-03) User authorized a new read-only adversarial audit using
  investigator subagents with orchestrator integration.
- [x] (2026-08-03) Predecessor package committed and pushed as `073dafe3`.
- [x] (2026-08-03) Scaffolded package, prompt, queued artifacts, catalog, and
  roadmap entry.
- [x] (2026-08-03) Froze evidence, operators, roles, write set, and claims.
- [x] (2026-08-03) Completed six read-only investigations.
- [x] (2026-08-03) Reconstructed mass/energy boundaries and ran three
  prospectively frozen operators across four sites.
- [x] (2026-08-03) Integrated contradictions and ranked mechanisms.
- [x] (2026-08-03) Remediated review findings with a fully frozen v3 analysis,
  rebuilt release binary, and fresh same-binary 16-cell execution.
- [x] (2026-08-03) Completed dual review, finding disposition, dual terminal
  verification, and truthful `HOLD-EVIDENCE` closeout.

## Surprises & Discoveries

- Gross positive hourly CoE melt, not signed daily melt, reconstructs pack loss
  across all four sites. In the accepted uncensored v3 surface, Snowbird median
  gross-positive melt is `0.5379 m` against `0.5296 m` loss; negative-hour
  magnitude is `0.1243 m`.
- The legacy-routing rollback is order-one even though instantaneous liquid
  capacity is only centimetres. This separates melt generation from the export
  policy that turns generated melt into seasonal loss.
- The contract statement that mixed exported melt plus refreeze had not been
  observed is stale: the accepted primary cohort contains 1,031 such days.
- Density wet compaction receives pack loss through two aliases. The duplicate
  data-flow identity is exact, but active multilayer authority is insufficient
  for a physical-defect verdict; the separate CoE boundary excludes it as a
  direct SWE-loss cause.
- Rebuilding the release binary materially changed absolute baselines. V2
  therefore could not compare new operator cells with old predecessor traces;
  accepted v3 added four same-binary baseline cells.
- Repository HEAD advanced after the accepted v3 freeze from `073dafe3` to
  `06dc722c` through an out-of-package documentation commit. The six frozen
  protected tree identities, including `references/50201000`, remain unchanged;
  accepted binary and replay provenance stay bound to `073dafe3`.
- Stage-3 liquid closure is internally guarded but not independently
  reconstructable from the real JSONL consumer because four primitive fields
  are omitted. The current package must hold under its exit criteria.

## Decision Log

- Decision: Separate this audit from the completed RST package and prohibit
  production corrections.
  Rationale: The predecessor established the symptom; determining physical and
  implementation correctness spans multiple interacting authorities and needs
  adversarial characterization before any correction is justified.
  Date/Author: 2026-08-03 / Codex.
- Decision: Permit only minimal, prospectively frozen existing-selector replays.
  Rationale: Controlled probes can falsify interactions without turning the
  audit into parameter calibration or changing production source.
  Date/Author: 2026-08-03 / Codex.
- Decision: Run Stage-3 disabled, legacy CoE routing, and explicit longwave for
  all four sites; do not run the frozen legacy-density operator.
  Rationale: Static mapping excluded physical density as a direct CoE/SWE
  driver, while the other three operators discriminated thermal neutrality,
  routing/gate behavior, and missing explicit cooling. Stage 3 does not compose
  with legacy density under the frozen selected boundary.
  Date/Author: 2026-08-03 / Codex.
- Decision: Classify density wet-liquid double alias as a supported data-flow
  alias and active-authority gap, while excluding it as the direct peak-SWE
  cause.
  Rationale: File-line aliasing and exact trace reproduction agree, but the
  cited apply-once authority governs another selector and does not establish
  the correct complete multilayer driver.
  Date/Author: 2026-08-03 / Codex.
- Decision: Reject v1 and v2 scientific result surfaces and admit v3 only.
  Rationale: V1 did not prospectively freeze the event operator or primary
  censor rule. V2 mixed a rebuilt-binary operator surface with predecessor
  reference traces. V3 froze exact rules and ran baseline plus operators with
  one release binary.
  Date/Author: 2026-08-03 / Codex.
- Decision: Enter `HOLD-EVIDENCE` after completing characterization.
  Rationale: Required independent Stage-3 liquid closure cannot be reconstructed
  from the frozen real-consumer trace, and instrumentation is outside this
  package write set.
  Date/Author: 2026-08-03 / Codex.
- Decision: Amend the disposable target write set after review to retain v2 and
  v3 namespaces.
  Rationale: Review required non-overwriting reruns. V2 is retained as rejected
  custody evidence and v3 as accepted evidence. The amendment is retrospective
  and does not relabel the original manifest as prospective.
  Date/Author: 2026-08-03 / Codex.
- Decision: Amend the write set after verification precheck to disclose the
  exact invalid-run recovery namespace.
  Rationale: The rejected rain-label run was moved to a bounded `/tmp` custody
  path that the original prospective manifest did not authorize. Truthful
  lifecycle evidence requires recording the actual write without relabeling it
  as prospective.
  Date/Author: 2026-08-03 / Codex.

## Outcomes & Retrospective

Characterization, dual review, and dual verification are complete. The leading
modeled interaction is order-one gross-positive CoE generation, positive-parts export
under the modern capacity trajectory, and a downstream snow-neutral thermal
carrier. `B+C` provide the largest Snowbird signed raw-term scale, but forcing
bias versus empirical-formula inadequacy remains unresolved. Current Stage-3,
explicit-longwave, rain-heat, and downstream trace accounting do not explain
the order-one loss. Stateful cloud/shortwave effects, systemic phase/forcing
error, signed-hour physics, and the correct export boundary remain unresolved.
The wet-compaction duplicate alias is supported but lacks active physical
operand authority. Required Stage-3 liquid operands are missing from the real
trace, so the truthful disposition is `HOLD-EVIDENCE`, not complete.
