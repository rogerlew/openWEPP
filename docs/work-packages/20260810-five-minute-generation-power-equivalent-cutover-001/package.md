# Diagnose, adopt, and cut over five-minute power-equivalent runoff-generation forcing
Status: `REOPENED / EXECUTING — output-integrity and evidence corrections`

This ExecPlan is a living document. Maintain `Progress`, `Surprises & Discoveries`, `Decision Log`, and `Outcomes & Retrospective` as execution proceeds.

This plan is governed by `docs/codex_exec_plans.md`, `docs/work-packages/AGENTS.md`, `docs/standards/testing-and-gate-strategy.md`, and `docs/standards/kernel-work-package-preparation.md`.

Package ID: `20260810-five-minute-generation-power-equivalent-cutover-001`

Plan class: `Critical Kernel Feature / Conditional Adoption and Cutover ExecPlan`

Work item IDs:

- `SUBHOUR-WATER-001`: create an event-resolved five-minute runoff-generation water output.
- `EROSION-PEQ-002`: evaluate and, only if admitted, adopt a power-equivalent within-hour erosion forcing.
- `EROSION-PEQ-003`: prove that the power-equivalent cutover does not alter water authority, WB16 peak authority, HBP interchange, routing, or unrelated process state.
- `TOPANGA-PEQ-004`: execute the frozen Topanga small-mutation design and issue an explicit adoption disposition before cutover.

Expected starting repository head:

    a8a96498ee909c4305fbc0a4db562b72e45efd2b

The executor must record the actual starting `HEAD`. If it differs, inspect the intervening commits and update this plan’s `Decision Log` before editing. Do not reset, discard, or overwrite unrelated work.

## Purpose / Big Picture

After this work, an end user can request a separate event-level Parquet product showing five-minute rainfall and modeled surface-runoff-generation intensity in millimetres per hour. The output is reconstructed from the existing scale-free hyetograph and the existing Green–Ampt implementation without changing daily water balance, hourly runoff volume, WB16 peak flow, HBP payloads, channel routing, or the number of erosion solves.

The package also tests a stronger erosion reduction than either an hourly arithmetic mean or a raw five-minute maximum. It represents the subhourly generation series with a power-equivalent rectangular forcing. The reduction must preserve the authoritative hourly runoff volume and a contract-selected nonlinear power integral. It is not a new peak-flow output and must never be published as discharge.

The package has an explicit adoption branch. Five-minute water output may be accepted even when erosion adoption is rejected. The production erosion default changes only if the mathematical authority, source completeness, numerical stability, Topanga mutation behavior, real-consumer proof, and Critical validation gates all pass.

The package must run end-to-end without asking the operator for incremental direction. It must not force adoption merely because implementation exists.

## Progress

- [x] (2026-08-11) Reopened the package after independent review confirmed a
  High-severity sibling-output clobber path, a missing public depression-storage
  operand, exact-clean-commit evidence debt, and unmeasured WAT5-enabled
  long-run cost. The scientific `NO_ADOPTION` and Topanga outcome embargo remain
  accepted and unchanged.
- [x] (2026-08-11) Confirmed the run file is already the sole user-facing WAT5
  selector: presence of `outputs.wat_subhourly` opts in and supplies the path;
  absence opts out. No second CLI or environment selector will be introduced.
- [x] (2026-08-11) Implemented and focused-tested rollback-safe all-or-nothing run-output publication,
  with the manifest as the last completion marker.
- [x] (2026-08-11) Published per-bin depression-storage retention, renamed the raw
  post-depression generation field, and validate public records at the writer
  boundary under a versioned schema.
- [x] (2026-08-11) Ran focused failure/success, positive-storage, real-consumer, performance,
  authority, formatting, and lint evidence.
- [ ] Commit the corrected implementation tree, run the terminal full workspace
  and doctests against that exact clean commit, refresh all required reviews and
  terminal verifiers against that identity, and reclose only if every finding
  passes.

- [x] (2026-08-10) Recorded actual starting source identity, clean worktree,
  applicable `AGENTS.md` chains, and the blocked prerequisite package status.
- [x] (2026-08-10) Confirmed the package, prompt directories, queued artifacts,
  and work-package catalog entry were scaffolded by `c9f28a7db`.
- [x] (2026-08-10) Froze exact source/toolchain/runner identities, focused
  synthetic water and persistent-rill-state receipts, and byte plus logical-row
  hashes for p61 and p102 WAT/PASS/HBP/manifests before production edits. The
  Topanga erosion pair remains unopened under the binding `NO_ADOPTION` embargo.
- [x] (2026-08-10) Completed a static rate/duration consumer map for the
  current hourly Wave-1 erosion path; retained it as restart evidence without
  treating it as adoption authority.
- [x] (2026-08-10) Implemented and tested package-local diagnostic feasibility
  tools without changing production behavior.
- [x] (2026-08-10) Rejected every fixed exponent after a prospective
  constitutive study; identified the two-moment form as structurally necessary
  but not production-admissible without an exponent.
- [x] (2026-08-10) Froze `NO_ADOPTION`, the outcome embargo, thresholds, and
  Topanga input identities; no candidate implementation identity exists.
- [x] (2026-08-10) Amended diagnostic water-output contracts; the candidate
  contract amendment remained unnecessary after binding `NO_ADOPTION`.
- [x] (2026-08-10) Added contract-derived tests and recorded the required 0/6
  pre-implementation red gate followed by the terminal green receipts.
- [x] (2026-08-10) Implemented the isolated opt-in five-minute generation
  ledger and streaming optional Parquet product.
- [x] (2026-08-10) Dispositioned the non-authoritative erosion arm as
  `NOT_IMPLEMENTED_NO_ADOPTION`; no candidate state or selector was opened.
- [x] (2026-08-10) Proved diagnostics-off and diagnostics-on noninterference
  with frozen p61 and source-complete warm-rain byte identities.
- [x] (2026-08-10) Executed applicable water resolution, boundary, analytic,
  source-completeness, and performance studies; erosion-only phase/power
  studies are not applicable after `NO_ADOPTION`.
- [x] (2026-08-10) Preserved the Topanga outcome embargo; the frozen 1,088
  trial diagnostic/cutover studies are not applicable after `NO_ADOPTION`.
- [x] (2026-08-10) Completed the prospective erosion feasibility disposition
  as `NO_ADOPTION`; package-level water disposition remains blocked.
- [x] (2026-08-10) Did not amend production erosion authority or ADR-0036;
  the conditional `ADOPT` branch was not entered.
- [x] (2026-08-10) Retained the existing hourly-mean production erosion
  consumer; no candidate or production-contract red gate was applicable.
- [x] (2026-08-10) Did not cut over erosion; production remains hourly mean.
- [x] (2026-08-10) Recorded real WAT5 publication-consumer proof; Wave-1
  candidate/cutover equivalence is not applicable.
- [x] (2026-08-10) Did not execute Topanga cutover outcomes under the binding
  outcome embargo.
- [x] (2026-08-10) Ran Critical working-source validation, dual science and
  Rust review, and finding disposition; the final post-authority-repair full
  nextest passed 2,380/2,380 and workspace doctests reported zero failures.
- [x] (2026-08-10) Completed fresh dual terminal verification, reconciled the
  exact 87-path owned diff, and issued the final `DIAGNOSTIC_ONLY` water /
  erosion `NO_ADOPTION` disposition.
- [x] (2026-08-10) Stopped at the declared hard prerequisite boundary and
  recorded a HOLD legitimacy audit; no shared contract or production edit was
  opened.
- [x] (2026-08-10) Closed all review findings, passed dual terminal verification
  for the executed HOLD, and reconciled the 30-path terminal diff.
- [x] (2026-08-10) Formally closed the hourly-peak predecessor after its fresh
  exact-source 2,346/2,346 full-workspace pass and fresh dual terminal
  verification; lifted the prerequisite HOLD and resumed at Milestone 1.

## Surprises & Discoveries

- The original WAT5 no-clobber implementation protected only the WAT5 target;
  ordinary WAT and PASS writers opened their final paths before WAT5 admission.
  The corrected runner stages the complete requested output set and publishes
  the manifest last as the completion marker.
- A source-complete, continuously active 45-year p1-geometry workload emitted
  394,488 rows in 4.30 seconds with WAT5 versus 3.02 seconds without it. The
  opt-in path therefore has material relative overhead but low absolute cost
  for one hillslope; compressed output was 3,526,518 bytes (8.94 bytes/row).
- The exact A1 package selection contains 2,028 tests and enters the same
  multi-minute assurance cases as the full profile. A dirty-tree attempt was
  stopped after 180 passes and is not evidence; the exact-clean terminal full
  campaign is the admitted superseding run.

Known observations at plan authoring time:

- The existing WB14 code already advances Green–Ampt over piecewise-constant hyetograph intervals and only later projects generated excess into hourly bins. Five-minute work is therefore primarily a temporal projection and output problem, not authorization to replace the Green–Ampt equations.
- The current hourly erosion path uses one field named like a peak rate for several purposes: local hydraulic forcing, outgoing discharge, volume-duration transposition, and inter-OFE handoff. A safe power-equivalent implementation may require splitting those meanings rather than replacing one scalar everywhere.
- A conventional power mean preserves a nonlinear integral over a fixed hour but does not also preserve runoff volume. The existing WEPP erosion transposition carries both a representative rate and an effective duration, so feasibility must consider a two-moment rectangular reduction.
- Current rainfall `effint` is a separate legacy `REID` quantity: mean rainfall intensity over rainfall-excess intervals, excluding snowmelt from its numerator. This package does not silently replace that operand with runoff-generation intensity.
- The current WAT interchange is daily. A separate event-subinterval dataset is safer than adding hundreds of fields to the daily WAT schema.
- Topanga’s frozen design is single-OFE and rain dominated. It can support a first production adoption for that domain but cannot by itself validate a new multi-OFE or snowmelt timing rule.

- Execution intake found that the required predecessor was source-aligned but
  not then lifecycle-terminal: its package remained `executing` and its
  disposition remained `reopened`. The current peak-authority contract test
  passed 4/4, so the historical blocker was the explicit terminal prerequisite,
  not a new peak-source contradiction.

- Independent Review A correctly found that the initial whole-package HOLD was
  premature because package-local feasibility remained lawful. The accepted
  correction executed 1,008 prospective constitutive records and rejected all
  fixed exponents without opening Topanga mutation outcomes.

- The predecessor was subsequently terminally closed at the reconciled
  authority identity after the fresh full-workspace and dual-verifier gates.
  The historical HOLD was legitimate when issued and is now lifted.

- Both fresh science reviews found that the first implementation apportioned
  a whole-interval Green-Ampt solve across five-minute bins. The accepted fix
  splits source intervals at exact 300-second boundaries before each
  chronological state advance; the delayed-ponding vector now has exact-zero
  first-bin excess and positive second-bin excess.

- Review also separated exact source/support presence from floating-point
  closure tolerance, made bins day-relative `0..287`, and added raw event,
  composed-hour, and day closure guards. Rust review additionally closed
  bounded-clock, no-clobber, target-alias, typed-conversion/error,
  completed-file validation, and resident-frame-layout defects.

- The post-review real p61 consumer writes 24 rows across bins `0..23`, closes
  both authoritative hours exactly under independent reconstruction, and
  leaves every erosion-candidate field null. Diagnostics-on/off HBP, PASS,
  WAT, and loss outputs are byte-identical. The p102 multi-OFE case fails
  closed with `WAT5-E-001` and no WAT5 publication.

Add concise observations and evidence here during execution.

## Decision Log

- Decision: reopen terminal acceptance while retaining the scientific branch.
  Rationale: commit `689bf3193` preserves the correct erosion `NO_ADOPTION`, but
  WAT/PASS writers truncate final paths before WAT5 construction and close
  before WAT5 validation/publication. The existing evidence also omits a
  positive-storage public reconstruction, exact-clean-commit terminal identity,
  and representative enabled-product cost.
  Date/Author: 2026-08-11 / Codex executor after user-accepted review.

- Decision: make the run file the only WAT5 selection authority and retain
  path-presence semantics.
  Rationale: `outputs.wat_subhourly` already declaratively supplies both opt-in
  state and output location. A separate boolean or CLI flag would allow
  contradictory run specifications.
  Date/Author: 2026-08-11 / operator direction and Codex reconciliation.

- Decision: publish a version-2 WAT5 schema.
  Rationale: independently reconstructible raw closure requires a per-bin
  depression-storage-retention operand, and the existing raw-generation name
  incorrectly suggests pre-storage Green-Ampt excess. Adding and renaming
  serialized columns is an intentional breaking schema correction.
  Date/Author: 2026-08-11 / Codex executor.

- Decision: keep WB16/public peak authority unchanged.
  Rationale: the new quantity is an erosion forcing reduction, not a hydrologic peak. The public peak remains the maximum hourly mean derived from the closing hourly runoff ledger.
  Date/Author: 2026-08-10 / operator-directed plan.

- Decision: keep HBP at 24 hourly water and sediment slots.
  Rationale: five-minute HBP propagation and five-minute erosion solves are explicitly out of scope.
  Date/Author: 2026-08-10 / operator-directed plan.

- Decision: add a separate subhourly water-output family rather than widen daily WAT.
  Rationale: this preserves the existing daily schema and makes the optional storage cost explicit.
  Date/Author: 2026-08-10 / plan author.

- Decision: treat the power-law exponent as fixed model authority, not a user parameter.
  Rationale: fitting an exponent to Topanga sediment outcomes would be hidden calibration and would make the enriched-discovery site both selector and validator.
  Date/Author: 2026-08-10 / plan author.

- Decision: Topanga cannot select the exponent or reduction formula.
  Rationale: the exponent and formula must be frozen from equations, source behavior, and synthetic constitutive analysis before result-bearing Topanga execution.
  Date/Author: 2026-08-10 / plan author.

- Decision: preserve the existing hourly runoff ledger as the mass authority.
  Rationale: five-minute timing may refine within-hour shape but cannot change daily or hourly water balance in this package.
  Date/Author: 2026-08-10 / plan author.

- Decision: the initial cutover domain is single-OFE, rainfall-driven erosion hours.
  Rationale: Topanga supports that domain, while positive routed runon, routed melt, and multi-OFE sediment handoff need additional timing authority. Saturation-return water may participate only as an explicitly documented hour-resolved background component.
  Date/Author: 2026-08-10 / plan author.

- Decision: retain the hourly-mean erosion mode as an explicit rollback/comparator selector for one transition window, never as a silent fallback.
  Rationale: rollback must be observable and typed. Missing required power-equivalent inputs in the admitted domain are errors, not permission to switch algorithms.
  Date/Author: 2026-08-10 / plan author.

- Decision: hold before shared contract and production edits.
  Rationale: the prerequisite package is explicitly reopened, and this plan
  prohibits those edits until it is terminally closed. Editing the prior
  package to lift that state is outside this package's write set and conflicts
  with the prohibition on modifying its historical artifacts.
  Date/Author: 2026-08-10 / Codex executor.

- Decision: lift the prerequisite HOLD and resume the independent water-output
  path.
  Rationale: the predecessor now records terminal Critical PASS with a fresh
  exact-source 2,346/2,346 full-workspace receipt and two fresh terminal
  verifier PASS receipts. The erosion `NO_ADOPTION` remains binding, so resume
  excludes candidate erosion implementation and production cutover.
  Date/Author: 2026-08-10 / Codex executor.

- Decision: reject erosion adoption before Topanga execution.
  Rationale: fixed-hour power means violate volume-duration custody. The
  two-moment candidates close their selected moments, but every exponent fails
  the bounded screened detachment/deposition response thresholds and none has
  a stable equation-level exponent through the production hydraulic/Yalin/continuity
  chain. Topanga cannot repair or select missing authority.
  Date/Author: 2026-08-10 / Codex executor.

- Decision: admit only the independent WAT5 water product as
  `DIAGNOSTIC_ONLY` for source-complete rain-timed inputs.
  Rationale: post-review raw/hour/day closure, typed Parquet publication,
  real-consumer reconstruction, byte noninterference, and source-exclusion
  evidence pass. Positive untimed supply remains a typed failure, saturation
  timing is explicitly hour-resolved, and no peak/discharge/erosion/routing
  claim is made.
  Date/Author: 2026-08-10 / Codex executor.

- Decision: include the science-contract admission checker's contract-ID
  grammar in the owned validation write set.
  Rationale: the package explicitly names `SC-OUTPUT-WAT5-001`, while the
  checker accepted only a single domain token. The grammar now admits one or
  more uppercase domain tokens separated by hyphens; lifecycle, registry, and
  file-set checks remain unchanged.
  Date/Author: 2026-08-10 / Codex executor.

- Decision: admit the exact dirty implementation through explicit worktree
  mode and atomic multi-authority impact bindings.
  Rationale: terminal verification found that base=head admission incorrectly
  reported zero science surfaces. The corrected checker includes untracked
  paths, fingerprints its complete authority input surface, requires at least
  one one-contract binding per science path, rejects malformed/duplicate
  bindings, and validates every binding on shared files. Seventeen WAT5 paths
  are now bound to the approved output contract; four shared runtime files
  conservatively retain their independent Plant binding as well. The draft
  `SC-WATBAL-001` amendment was removed, and all new diagnostic authority and
  tolerance now live in approved, active `SC-OUTPUT-WAT5-001`.
  Date/Author: 2026-08-10 / Codex executor after terminal finding.

## Outcomes & Retrospective

The five-minute water product is accepted as `DIAGNOSTIC_ONLY` for
source-complete rain-timed inputs. It advances the unchanged WB14 Green-Ampt
model on exact 300-second boundary pieces, closes raw/hour/day ledgers, labels
hourly saturation as a zero-order hold, and publishes a sparse version-2
Parquet dataset. Positive untimed supply fails typed rather than acquiring an
invented clock.

No fixed erosion exponent was admitted. The only structurally plausible
two-moment reduction failed the preregistered response screen and lacked
equation-level exponent authority. Consequently candidate/cutover sediment
effects, power/phase thresholds, and Topanga mutation continuity are not
applicable; no outcome was opened. Production erosion remains hourly mean.

Post-review p61 evidence closes the two active hours independently, retains
all power fields null, and makes HBP/PASS/WAT/loss byte-identical with WAT5 on
and off. Resolution vectors cover 60 minutes through 1 minute, exact and hour
boundaries, and delayed ponding. Multi-OFE runon, routed melt, snowmelt,
frost-release, HBP, channel, and watershed timing remain outside the admitted
domain. The p102 control proves this boundary with `WAT5-E-001`.

The largest execution lesson was that total-preserving proportional binning
is not a temporal physics replay. Independent reviews caught that distinction
and drove exact-boundary advancement, exact-positive branch semantics, and
explicit raw/hour/day guards before acceptance.

Terminal verification also exposed that a base=head admission receipt can be
formally green while observing none of a dirty implementation. Exact worktree
admission now sees all 17 science surfaces and preserves both authorities on
the four shared Plant/WAT5 files instead of silently replacing either one.

## Context and Orientation

The current water path reconstructs or receives a breakpoint hyetograph in `DirectWb14HyetographInterval` records. `compute_wb14_infiltration_depression_with_profile` advances Green–Ampt over those intervals, accumulates infiltration, forms excess, applies depression storage, and projects the remaining excess into a 24-hour array. Hourly saturation-return water is then added to form the closing hourly runoff ledger.

The current WB16/public peak is the maximum closing hourly depth divided by 3,600 seconds. It is a depth rate internally and becomes volumetric flow only at public publication through one area conversion. This authority is protected and is not changed by this package.

ADR-0036 currently runs Wave-1 erosion once for each hydraulically active hour. The hourly quantum receives its runoff depth and a representative rate equal to that depth divided by 3,600 seconds. The same rate is presently threaded through several hydraulic and continuity operands. Before implementation, the executor must map every use and separate:

- authoritative hourly runoff depth;
- hourly arithmetic-mean runoff rate;
- candidate power-equivalent hydraulic rate;
- effective rectangular duration;
- inter-OFE water-discharge rate;
- rainfall-excess duration;
- rainfall effective intensity;
- public WB16 peak;
- HBP hourly volume.

No field may retain an ambiguous `peakro` name after it carries a power-equivalent erosion-only quantity.

## Mathematical Definitions

### Five-minute clock

Use model-clock-aligned intervals:

    Δt = 300 s
    12 intervals per model hour
    288 intervals per model day

Integrate every original hyetograph interval against five-minute interval boundaries. Never sample rainfall intensity at interval centres.

For an original interval `[a,b)` with constant intensity `i`, its contribution to five-minute bin `[u,v)` is:

    depth = i * max(0, min(b,v) - max(a,u))

Partial event-edge overlap uses the actual overlap duration. Model-hour boundaries and day boundaries are exact.

### Raw diagnostic Green–Ampt ledger

Let:

    r_raw[h,k] = raw Green–Ampt-generated excess depth in hour h, subinterval k
    f_raw[h,k] = raw Green–Ampt infiltration depth
    p[h,k]     = rainfall plus explicitly represented supply depth
    Δs_raw     = diagnostic depression-storage change

The isolated diagnostic calculation must satisfy its own event closure:

    Σ p = Σ f_raw + Σ r_raw + Δs_raw

within a named, contract-defined floating-point tolerance.

The calculation must carry cumulative Green–Ampt infiltration continuously through the event. It must not reset at five-minute or hourly boundaries.

The equations, conductivity, matric potential, storage capacity, and depression-storage capacity are unchanged. Refactoring the existing interval solver into a shared pure helper is allowed; changing its equations or parameters is not.

### Hourly mass closing

The existing hourly WB14 post-depression runoff depth remains authoritative:

    R_wb14[h]

The raw diagnostic five-minute shape is reconciled within each hour, without changing `R_wb14[h]`.

When `R_wb14[h] = 0`, all closed WB14 five-minute generation depths are exactly zero.

When `R_wb14[h] > 0` and `Σ_k r_raw[h,k] > 0`:

    r_closed[h,k] =
        R_wb14[h] * r_raw[h,k] / Σ_j r_raw[h,j]

When `R_wb14[h] > 0` and the raw diagnostic shape has zero support, return a typed source-completeness failure. Do not distribute the depth uniformly and do not use rainfall intensity as an undocumented fallback.

Hourly saturation return remains in its modeled hour. For the subhourly erosion reduction it is represented as an explicitly labeled zero-order hold at its producer resolution:

    s_closed[h,k] = S_sat[h] * Δt / 3600

This is not a claim that the actual return flow was constant. It states only that no timing finer than the modeled hour exists for that source. The water output must expose the source separately.

The closing five-minute surface-generation depth is:

    q5[h,k] = r_closed[h,k] + s_closed[h,k]

and must satisfy:

    Σ_k q5[h,k] = R_wb14[h] + S_sat[h] = Q_hour[h]

within the named subhourly closure tolerance.

Positive routed runon, routed melt, or another hourly-only additional supply may be shown in the diagnostic water output, but it is outside the V1 erosion-cutover domain unless the package establishes a contract-backed subhourly representation. No silent zero-order-hold adoption is permitted for these sources.

### Five-minute generation intensity

For each five-minute interval:

    g[h,k] = q5[h,k] / Δt

Internal units are metres per second. The water output publishes:

    generation_intensity_mm_h = g[h,k] * 1000 * 3600

The output description must call this an interval-average surface-runoff-generation intensity. It is not routed discharge, instantaneous peak flow, or a value sustained for the whole hour.

### Candidate power reductions

Let:

    T = 3600 s
    V = Σ_k g[h,k] * Δt = Q_hour[h]
    E_p = Σ_k g[h,k]^p * Δt

Evaluate two distinct reductions during feasibility.

The fixed-hour power mean is:

    g_power_mean(p) = (E_p / T)^(1/p)

It preserves the p-th power integral when applied over the full hour:

    g_power_mean(p)^p * T = E_p

It does not generally satisfy `g_power_mean * T = V`.

The power-and-volume-equivalent rectangular rate is:

    g_power_volume(p) = (E_p / V)^(1/(p-1)), for p > 1 and V > 0

with equivalent duration:

    d_power_volume = V / g_power_volume

It preserves both:

    g_power_volume * d_power_volume = V

and:

    g_power_volume^p * d_power_volume = E_p

For zero runoff:

    g = 0
    duration = 0

For constant generation over the complete hour, either reduction must reproduce the original rate; the volume-equivalent duration must be 3,600 seconds.

The consumer audit decides which reduction is admissible:

- use the fixed-hour power mean only if the actual hourly erosion consumer uses a fixed one-hour support and independently consumes volume without requiring `rate * duration = volume`;
- use the power-and-volume-equivalent rectangular reduction when the consumer retains WEPP’s representative-rate plus equivalent-duration transposition;
- reject erosion adoption if neither reduction can be mapped to the actual solver without contradicting its water, sediment, or inter-OFE identities.

The raw maximum five-minute intensity is diagnostic only. Source and contract guards must prohibit it from becoming production forcing.

### Exponent authority

The exponent is not assumed merely from the phrase “power-equivalent.”

The feasibility study begins with these preregistered candidates:

    p = 1.0        arithmetic-mean no-op control
    p = 4/3
    p = 3/2        explicit hypothesis motivated by the transport-capacity shear power
    p = 2.0

The executor must not infer that a `3/2` power on shear is automatically a `3/2` power on generation rate. Rill width, Chezy depth, shear, thresholds, transport capacity, and deposition branches intervene.

A different candidate may be added only before Topanga outcome files are opened, with equation-level rationale recorded in `artifacts/exponent-authority.md`.

A production exponent must:

- be one fixed finite constant;
- be greater than one for the two-moment rectangular form;
- trace to an explicit contract-backed response being preserved;
- be stable across the declared V1 domain;
- not be selected by minimizing Topanga sediment differences;
- not vary by event, hillslope, soil, cover, slope, burn severity, or mutation;
- not be exposed as a calibration parameter.

If no fixed exponent meets these conditions, erosion adoption is `NO_ADOPTION`. The five-minute water output may still proceed.

## Implementation Intent and Risk

Intent:

    science implementation
    diagnostic feasibility
    optional conservation-sensitive output
    independent mutation-cohort validation
    conditional production cutover

Risk:

    Critical

The risk is Critical because the work can affect production erosion magnitude, persistent rill-width state, sediment handoff, public output semantics, and default model selection.

Calibration posture:

    science_implementation_status = IMPLEMENTED or NOT_IMPLEMENTED
    calibration_evidence_status = NOT_APPLICABLE
    identifiability_status = NOT_APPLICABLE

The exponent is a fixed numerical/model-reduction authority. It is not empirically calibrated in this package.

## Prerequisite Gate

Before production edits:

1. Confirm `20260809-hourly-peak-runoff-authority-closure-001` is terminal `PASS` at the actual base identity.
2. Confirm ADR-0036, `SC-WATBAL-001`, `SC-SED-001`, `SC-INFILE-HBP-001`, and `tests/integration/peak_hourly_authority_contract.rs` agree that:
   - WB16 peak is the maximum hourly mean;
   - public peak applies area exactly once;
   - HBP minor-1 peak is derived from hourly water;
   - no independent analytical peak is native authority.
3. Run the existing peak-authority contract test.
4. Record the exact result in `artifacts/prerequisite-authority-gate.md`.

If the prerequisite package is not terminally closed, diagnostics may proceed only in package-local tools. No shared contract or production source edit may begin.

## Authority and Write-Set Envelope

### Documentation and contract write set

- `docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- a new `docs/specifications/science-contracts/contracts/SC-OUTPUT-WAT5-001.md`, unless intake proves an existing canonical output contract is the correct owner
- `docs/specifications/science-contracts/index.md`
- `docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`, only after `ADOPT`
- decision indexes or lifecycle records directly required by that amendment
- unit and output-schema governance documents directly required by the new dataset

### Production source write set

Expected files include:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- a new focused module such as `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_operands.rs`
- directly affected direct-runtime tests
- `crates/openwepp-runner/src/hillslope/` output configuration, execution, and publication files
- a new output module under `crates/openwepp-hillslope-output/src/`
- directly affected schemas, row writers, and round-trip tests
- focused integration tests under `tests/integration/`
- `Cargo.toml` files only when a new test target, module, or binary requires them

Before first edit, use:

    tools/agents/find-agents --for <every intended write path>

Record the complete instruction chain in `artifacts/required-reading-map.md`.

Adjacent files in the same authority domain may be added before first edit with a `Decision Log` entry and owned-file manifest update. Do not expand into HBP, watershed routing, climate generation, or multi-OFE cutover code.

### Diagnostic tooling write set

- package-local Rust or Python tools
- package-local tests for those tools
- immutable input plans and compact metadata
- external evidence roots outside the repository for large Parquet files and run directories

Do not modify historical artifacts in the prior hourly-peak package.

## Protected Boundaries

The following are hard boundaries:

- Do not change Green–Ampt equations, coefficients, ponding conditions, storage capacity, or conductivity.
- Do not change daily infiltration, runoff, soil water, ET, percolation, frost, or saturation-return state.
- Do not change the authoritative 24-bin runoff ledger.
- Do not change WB16 peak or rectangular event-duration authority.
- Do not claim an instantaneous or routed five-minute peak.
- Do not add five-minute fields to HBP.
- Do not change HBP major or minor version.
- Do not change watershed or channel routing.
- Do not run Wave-1 erosion at five-minute intervals.
- Do not change the number or ordering of hourly erosion solves.
- Do not replace rainfall `effint` or rainfall-excess `effdrr` in this package.
- Do not fit the exponent to Topanga.
- Do not change Ksat, cover, erodibility, transport coefficients, or calibration parameters.
- Do not adopt raw maximum five-minute generation intensity.
- Do not weaken, delete, rename away, or bypass existing peak-authority tests.
- Do not silently fall back to the hourly mean when an eligible power-equivalent input is missing.
- Do not claim multi-OFE, snowmelt, runon, HBP, routed-watershed, observed-flow, calibration, or universal validation from Topanga.

Any change to one of these boundaries requires a separately authorized package, not an in-flight scope expansion.

## Required Reading

### Core

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- this package’s `package.md`
- `/workdir/openWEPP/docs/standards/testing-and-gate-strategy.md`
- `/workdir/openWEPP/docs/standards/kernel-work-package-preparation.md`
- `/workdir/openWEPP/docs/standards/local-ci-gate-selection.md`

### Conditional, required before contract or kernel edits

- `/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md`
- `/workdir/openWEPP/docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`

### On demand, required for the named mechanism

- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/package.md`
- its operand-lineage, implementation-evidence, mutation-study, review, and verification artifacts
- `tests/integration/peak_hourly_authority_contract.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_seed.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_operands.rs`
- `crates/openwepp-runner/src/hillslope/04_direct_publication.rs`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
- the Topanga LaTeX report at the pinned wepppy identity
- the frozen Topanga plan and harness from the prior package

Record local byte totals and the required-reading budget disposition.

## Required Package Structure and Artifacts

Create:

    docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/
      package.md
      prompts/README.md
      prompts/active/README.md
      prompts/active/20260810-five-minute-generation-power-equivalent-cutover-001_kickoff_agent_prompt.md
      prompts/archived/README.md
      artifacts/README.md
      tools/

Pre-create queued artifacts:

- `artifacts/required-reading-map.md`
- `artifacts/intent-and-base.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/prerequisite-authority-gate.md`
- `artifacts/current-consumer-map.md`
- `artifacts/operand-lineage.md`
- `artifacts/baseline-output-hash-manifest.json`
- `artifacts/baseline-state-identity.md`
- `artifacts/feasibility-protocol.md`
- `artifacts/constitutive-response-study.md`
- `artifacts/exponent-authority.md`
- `artifacts/reduction-selection.md`
- `artifacts/adoption-criteria-preregistration.md`
- `artifacts/diagnostic-contract-evidence.md`
- `artifacts/diagnostic-contract-test-evidence.md`
- `artifacts/pre-implementation-diagnostic-contract-gate.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/no-coupling-byte-identity.md`
- `artifacts/five-minute-water-closure.md`
- `artifacts/resolution-and-phase-sensitivity.md`
- `artifacts/source-completeness.md`
- `artifacts/performance-and-output-size.md`
- `artifacts/topanga-plan-identity.json`
- `artifacts/topanga-diagnostic-study.md`
- `artifacts/topanga-diagnostic-summary.json`
- `artifacts/adoption-disposition.md`
- `artifacts/cutover-contract-evidence.md`
- `artifacts/pre-cutover-contract-gate.md`
- `artifacts/real-consumer-proof.md`
- `artifacts/old-path-negative-proof.md`
- `artifacts/topanga-cutover-study.md`
- `artifacts/topanga-cutover-summary.json`
- `artifacts/kernel-profile-compliance.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/rust_code_review.md`
- `artifacts/rust_qa_review.md`
- `artifacts/finding-disposition.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`
- `artifacts/calibration-readiness-matrix.md`

Large cohort Parquet, binaries, and run directories belong in an external immutable evidence root. Commit only compact manifests, hashes, summaries, and logs.

## Milestone 1: Freeze the Existing Authority and Outputs

Record:

    git rev-parse HEAD
    git status --short
    rustc --version
    cargo --version
    cargo nextest --version
    sha256sum of the production runner binary
    exact feature flags and environment

Run the current peak-authority contract and focused erosion tests before editing.

Generate baseline results for:

- one dry fixture;
- one constant-intensity nonponding fixture;
- one delayed-ponding fixture;
- one high-intensity ponded fixture;
- one saturation-return fixture;
- p61 single-OFE erosion;
- one multi-OFE fixture, protected as a non-adoption control;
- one frozen Topanga baseline and its paired `Ksat +1%` mutation.

Hash or canonically serialize:

- WAT Parquet schema, metadata, row values, and bytes when deterministic;
- PASS Parquet;
- HBP binary;
- public peak fields;
- hourly runoff volumes;
- daily water rows;
- erosion rows;
- persistent erosion carry, especially rill width;
- run manifest and method metadata;
- stderr/stdout when semantically relevant.

These identities are not magnitude targets for the eventual erosion cutover. They are strict noninterference targets for diagnostics and all protected water/HBP surfaces.

## Milestone 2: Audit Rate, Duration, and Consumer Semantics

Before choosing a formula, trace every consumer of:

- `peakro_m_s`;
- `peak_runoff_rate_m_s`;
- hourly runoff depth;
- `effdrn_s`;
- `effdrr_s`;
- `qout_m2_s`;
- `qin_m2_s`;
- `qshear_m2_s`;
- rill width;
- shear;
- transport capacity;
- deposition coefficient;
- sediment discharge;
- HBP hourly water and sediment;
- public peak.

For each operand, record:

- units;
- time support;
- spatial basis;
- source;
- whether it is water authority, erosion forcing, publication only, or diagnostic;
- whether `rate * duration = volume` is required;
- whether it crosses an OFE or serialization boundary;
- whether replacing it with a power-equivalent quantity is admissible.

The map must explicitly answer:

1. Can a distinct erosion hydraulic rate be introduced while water discharge remains the hourly mean?
2. Does the Wave-1 solver require its representative rate and effective duration to reconstruct hourly runoff depth?
3. Which terms depend nonlinearly on representative rate?
4. Which terms must continue to use arithmetic-mean water discharge?
5. Would a variable equivalent duration break inter-OFE sediment-discharge handoff?
6. Can V1 be safely limited to single-OFE operation?

No contract or production decision proceeds with an unresolved consumer.

## Milestone 3: Diagnostic Feasibility and Exponent Authority

Use package-local tools and existing public pure functions. Do not edit production code during the first pass.

Construct a deterministic shape library including:

- constant intensity;
- one five-minute pulse;
- two separated pulses;
- rising limb;
- falling limb;
- symmetric triangular pulse;
- early ponding;
- late ponding;
- a pulse spanning a five-minute boundary;
- a pulse spanning an hourly boundary;
- rainfall plus an hourly saturation-return background;
- near-zero material-floor cases.

Construct a deterministic constitutive-state library spanning:

- Topanga burned and unburned representative soils and covers;
- low, median, and high slope;
- zero and developed rill width;
- low and high roughness;
- shear below, near, and above critical shear;
- detachment- and deposition-dominated conditions.

This library may use Topanga input states to span the operating domain. It must not inspect Topanga mutation sediment outcomes while selecting the exponent.

For every candidate exponent and reduction:

1. Compute five-minute generation rates.
2. Compute the candidate representative rate and duration.
3. Evaluate existing pure rate-dependent hydraulic and transport operand functions at each five-minute rate.
4. Time-integrate the specific constitutive response selected for preservation.
5. Evaluate the same response at the candidate representative rate.
6. Report relative error, branch changes, threshold proximity, rill-width changes, and invalid domains.

This is not permission to run the full erosion continuity solver at five-minute intervals. The reference is limited to pure constitutive/hydraulic operand evaluation.

The study must separately report:

- shear;
- transport capacity;
- rill-width growth;
- detachment-capacity driver;
- deposition coefficient;
- any other rate-dependent operand found by the consumer audit.

A fixed exponent is admissible only if:

- an explicit target response and equation are named;
- the rate exponent is justified after the hydraulic transformation, not copied from a shear exponent;
- median and 95th-percentile approximation errors are at most 5% and 15%, respectively, across the admitted domain;
- no unexplained material error exceeds 30%;
- threshold-neighbourhood failures are identified and bounded;
- both independent hydrology reviewers accept the authority;
- the exponent is frozen before Topanga execution.

These numerical percentages are adoption safety gates, not calibration targets. Failing them means no fixed-exponent erosion cutover.

Write the selected formula, exponent, validity domain, and rejected alternatives to:

    artifacts/exponent-authority.md
    artifacts/reduction-selection.md

Hash both artifacts and the candidate configuration.

## Milestone 4: Preregister Adoption Before Opening Topanga Outcomes

Write `artifacts/adoption-criteria-preregistration.md` before running a result-bearing Topanga candidate cohort.

Freeze:

- exponent;
- fixed-hour versus two-moment reduction;
- V1 applicability predicate;
- material floors;
- resolution and phase thresholds;
- mutation-screen rules;
- sediment attribution thresholds;
- performance thresholds;
- binary identity;
- Topanga input plan and selection identities;
- expected trial count;
- output schemas;
- all candidate method codes.

The V1 production applicability predicate is initially:

- single-OFE run;
- positive rain-driven local generation;
- no positive routed runon;
- no positive routed melt or another unresolved additional-supply source;
- complete five-minute shape;
- finite positive admitted exponent;
- no typed closure or source-completeness failure.

Hourly saturation return is allowed only through the explicit hour-resolved background representation defined above.

Dry hours produce exact zero.

All other hours continue under the existing hourly-mean method with explicit `not_applicable` provenance. This is a process-domain split, not a fallback after failure.

## Milestone 5: Author Diagnostic Contracts and Red Tests

Before production code edits:

1. Amend `SC-WATBAL-001` to define:
   - the optional five-minute diagnostic ledger;
   - exact temporal and unit basis;
   - raw diagnostic versus hourly-closed fields;
   - event and hourly closure;
   - saturation-return composition;
   - source-completeness failure;
   - the fact that the 24-bin ledger remains authoritative.

2. Create or amend the canonical water-output contract to define:
   - the separate subhourly dataset;
   - keys;
   - sparse-event semantics;
   - units;
   - metadata;
   - raw versus closed authority;
   - omitted-zero semantics;
   - versioning;
   - no HBP coupling.

3. Amend `SC-SED-001` only far enough to authorize:
   - a non-authoritative power-equivalent diagnostic candidate;
   - the distinction between arithmetic-mean water rate and candidate erosion rate;
   - the frozen exponent and formula;
   - the V1 applicability domain;
   - no production cutover yet.

4. Add contract-derived tests that require these diagnostics but also assert:
   - WB16/public peak remains maximum hourly mean;
   - HBP remains 24 slots;
   - raw five-minute maximum is diagnostic only;
   - rainfall `effint` remains unchanged;
   - the existing hourly-mean production consumer remains active before adoption.

Run the tests and record the expected red failure because implementation does not yet exist.

The existing `peak_hourly_authority_contract` test must remain green throughout. Do not weaken it to make the new test pass.

## Milestone 6: Implement the Isolated Five-Minute Water Ledger

Refactor the existing Green–Ampt interval implementation only as needed to expose a shared pure kernel. Prove the refactor is bitwise or bit-for-bit numerically identical for all existing callers.

Implement a typed structure such as:

    DirectFiveMinuteGenerationInterval
    DirectFiveMinuteGenerationHour
    DirectFiveMinuteGenerationEvent
    DirectPowerEquivalentHour

Use names that distinguish:

- raw diagnostic infiltration and generation;
- hourly-closed WB14 generation;
- saturation-return contribution;
- closing surface-generation depth;
- arithmetic hourly mean;
- power-equivalent rate;
- power-equivalent duration;
- selected exponent;
- source-completeness and applicability provenance.

The five-minute calculation must occur after the authoritative hourly WB14 and saturation-return ledgers are available. It must not mutate those ledgers.

Avoid recomputing the isolated calculation more than once per day/OFE. Publication and candidate erosion diagnostics consume one produced structure.

### Subhourly output family

Add a separate optional Parquet output, tentatively named:

    hillslope_wat_subhourly.parquet

Use the declarative hillslope run-file entry
`outputs.wat_subhourly = "<path>"` as the sole user-facing selector and path.
Do not add a competing CLI flag or environment selector.

The corrected public dataset version is `2.0`.

Required row keys and fields:

- `wepp_id`
- `ofe_id`
- `year`
- `sim_day_index`
- `julian`
- `event_ordinal`
- `hour_index`
- `subinterval_index`
- `interval_start_s`
- `interval_duration_s`
- `rainfall_depth_mm`
- `additional_supply_depth_mm`
- `raw_green_ampt_infiltration_depth_mm`
- `depression_storage_retention_depth_mm`
- `raw_wb14_post_depression_generation_depth_mm`
- `closed_wb14_generation_depth_mm`
- `saturation_return_depth_mm`
- `closing_surface_generation_depth_mm`
- `closing_surface_generation_intensity_mm_h`
- `hourly_authoritative_runoff_depth_mm`
- `hourly_mean_generation_intensity_mm_h`
- `hourly_power_equivalent_generation_intensity_mm_h`
- `hourly_power_equivalent_duration_s`
- `power_exponent`
- `method_code`
- `source_completeness_code`
- `hourly_closure_residual_mm`

Use the existing event identity. Do not invent gap-based storms. When runtime has one WB14 event/day, use `event_ordinal = 0` and document it.

The dataset may be sparse. Emit rows from the first through last nonzero forcing or closing-generation bin for the existing event, retaining zero bins inside that support. Metadata must state that omitted bins outside event support are exact zero.

The output metadata must distinguish:

- `raw_green_ampt_*`: isolated diagnostic calculation;
- `closed_wb14_*`: shape normalized to the authoritative hourly WB14 total;
- `closing_surface_*`: WB14 generation plus hourly saturation return;
- `power_equivalent_*`: erosion-reduction diagnostic, not flow publication.

## Milestone 7: Implement the Non-Authoritative Candidate Erosion Arm

Add an explicit diagnostic selector. It must default off.

The diagnostic arm:

- uses the selected power-equivalent rate and duration only in the admitted V1 domain;
- runs the ordinary hourly Wave-1 solve, never a five-minute solve;
- clones every persistent or mutable erosion state before execution;
- cannot change production rill width, sediment carry, publication, HBP, or later-day state;
- emits candidate results only to a package-owned diagnostic sidecar or explicitly requested diagnostic output;
- records applicability and method provenance;
- hard-fails its own diagnostic result on missing eligible inputs rather than silently using the hourly mean.

Add a state-identity test proving the candidate arm leaves every production field bitwise unchanged.

Run the current production and candidate arms on the same hourly input and retain:

- input identity;
- production result;
- candidate result;
- persistent-state before/after;
- closure results;
- branch and guard counters.

Shadow evidence supports adoption but cannot close cutover.

## Milestone 8: Prove Noninterference Before Topanga

Run three modes on the frozen fixtures:

1. baseline source identity;
2. new source with diagnostics disabled;
3. new source with five-minute output and candidate arm enabled.

Modes 1 and 2 must match exactly on all existing outputs and state.

Modes 2 and 3 must match exactly on:

- daily WAT rows;
- PASS rows;
- HBP bytes;
- public peak;
- hourly runoff volumes;
- daily water state;
- production erosion;
- sediment carry;
- rill-width carry;
- routing input;
- manifest science fields.

The only allowed additions in mode 3 are:

- the new subhourly output;
- additive output-manifest references to that file;
- explicitly requested diagnostic sidecars;
- diagnostic counters that cannot affect model selection.

Record exact SHA-256 comparisons where formats are deterministic. For Parquet surfaces, also compare canonical schemas, metadata, row counts, key order, nulls, and f64 bit patterns.

Any production difference blocks Topanga adoption execution until fixed.

## Milestone 9: Validate the Five-Minute Calculation

Required analytic/property tests:

- zero rainfall;
- intensity below conductivity;
- immediate ponding;
- delayed ponding inside a five-minute interval;
- constant intensity split into 60-, 30-, 10-, 5-, 2.5-, and 1-minute intervals;
- pulse beginning and ending inside a bin;
- event crossing an hour boundary;
- event ending exactly on a boundary;
- multiple rainfall intervals inside one five-minute bin;
- saturation-return-only hour;
- positive authoritative runoff with missing raw shape;
- non-finite and negative inputs;
- scaling all generation rates by a constant;
- `p=1` control;
- constant-rate power equivalence;
- single-pulse closed-form power equivalence;
- monotonicity with exponent;
- arithmetic-mean and raw-maximum bounds;
- exact two-moment volume and power reconstruction.

For the production 5-minute interval:

- compare 5 minutes against 2.5 minutes on every frozen Topanga baseline event;
- compare 5 minutes against 1 and 10 minutes on a preregistered stratified subset;
- shift the five-minute grid origin by 0, 60, 120, 180, and 240 seconds in diagnostics only.

Use the material floor:

    1e-7 m/s

For material hours, adoption requires:

- 5-minute versus 2.5-minute relative difference in the selected equivalent rate:
  - 99th percentile no greater than 5%;
  - no unexplained value greater than 25%.
- phase-origin relative range:
  - 99th percentile no greater than 5%;
  - no unexplained value greater than 25%.

Every exceedance must be retained and source-traced. A few explained boundary cases may be accepted by both science reviewers, but unresolved cases block adoption.

## Milestone 10: Execute the Frozen Topanga Diagnostic Adoption Study

Reuse the exact Topanga design:

- 560 first-horizon Ksat trials at baseline ×0.99 and ×1.01;
- 528 paired interrill/rill cover trials at ±0.01 after the frozen symmetry exclusions;
- 1,088 total trials;
- burned and unburned strata;
- full 1980–2024 antecedent history;
- one changed hillslope and parameter family per trial;
- the same event-pairing keys.

Do not modify historical package artifacts. Create a new package-local harness derived from the prior harness and pin:

- source commit;
- binary SHA-256;
- mutation plan SHA-256;
- selection SHA-256;
- input-tree identities;
- schema identities;
- selected exponent;
- reduction formula;
- diagnostic mode;
- output root.

The diagnostic cohort must produce both the current production result and candidate shadow result.

### Required event-level fields

- baseline and mutant authoritative hourly runoff depth;
- baseline and mutant public peak;
- five-minute raw generation shape;
- five-minute closed generation shape;
- arithmetic hourly mean;
- raw five-minute maximum;
- selected power-equivalent rate;
- selected equivalent duration;
- source-completeness status;
- production sediment;
- candidate sediment;
- production and candidate branch/guard counters;
- production and candidate rill-width carry.

### Required screens

Retain the report’s material floors and small-mutation philosophy.

Flag:

- event presence change;
- source-completeness or applicability change;
- any protected water/public/HBP difference;
- equivalent-rate change greater than 25% while hourly runoff changes less than 5%;
- equivalent-rate ratio at least 2 or at most 0.5;
- equivalent duration ratio at least 2 or at most 0.5;
- candidate sediment ratio outside 0.5–2 for material paired-positive events;
- candidate sediment ratio outside 0.2–5;
- production-versus-candidate sediment ratio outside 0.5–2;
- any new typed error, refusal, or branch-topology change;
- any exact-zero/nonzero topology change;
- any mutation-direction reversal not explained by the closed five-minute shape.

Every material flag must decompose through:

    hourly runoff ratio
    five-minute shape power ratio
    selected equivalent-rate ratio
    equivalent-duration ratio
    hydraulic/erosion branch ratio
    sediment ratio

Do not classify legacy parity as correctness.

### Claim boundary

The Topanga study supports only:

- deterministic operation;
- water noninterference;
- numerical and small-mutation continuity;
- behavior on the frozen Topanga single-OFE rain-driven domain;
- an adoption decision for that bounded domain.

It does not establish:

- observed hydrologic accuracy;
- universal prevalence;
- multi-OFE validity;
- snowmelt validity;
- runon validity;
- routed watershed behavior;
- channel behavior;
- return-period accuracy;
- empirical calibration.

## Adoption Disposition

Write `artifacts/adoption-disposition.md` with exactly one of:

### `ADOPT`

Proceed to production contracts and cutover only when all are true:

- the fixed exponent and reduction have equation-level authority accepted by both hydrology reviewers;
- no exponent or formula was selected using Topanga outcome magnitude;
- the five-minute raw and hourly-closed ledgers pass every closure gate;
- diagnostics leave every protected output and persistent state unchanged;
- source completeness is 100% for every hour in the proposed V1 production domain;
- no raw maximum is used by production code;
- resolution and phase gates pass;
- the 1,088-trial cohort completes with complete pairing and provenance;
- no unexplained equivalent-rate discontinuity remains;
- no new production guard/refusal event appears in candidate execution;
- every material sediment change is source-traced;
- any candidate/current sediment ratio outside 0.2–5 is resolved and accepted by both science reviewers;
- candidate runtime and memory are acceptable;
- no HBP, routing, public-peak, or daily-water change exists.

### `DIAGNOSTIC_ONLY`

Accept the five-minute water output but do not change production erosion when:

- the water diagnostic and output contracts pass;
- water noninterference passes;
- but no fixed exponent/reduction has adequate authority or stability;
- or Topanga exposes unresolved erosion sensitivity;
- or the candidate cannot be composed with current duration or state semantics safely.

This is a successful non-adoption disposition, not a request for another narrow diagnostic package.

### `REJECT`

Do not publish or adopt the new output when:

- five-minute water closure cannot be established;
- the isolated calculation changes authoritative water state;
- output semantics cannot distinguish raw diagnostic and hourly-closed generation;
- or the event identity is not reconstructible without inventing storms.

No production cutover occurs under `DIAGNOSTIC_ONLY` or `REJECT`.

## Milestone 11: Production Contract Amendment After `ADOPT`

Only after the signed adoption disposition:

1. Amend `SC-SED-001` to define the admitted production forcing:
   - selected exponent;
   - selected formula;
   - equivalent duration;
   - V1 applicability domain;
   - exact operand ownership;
   - dry behavior;
   - typed failure behavior;
   - relationship to hourly runoff depth;
   - relationship to interrill `effint` and `effdrr`;
   - explicit non-authority for WB16/public peak and HBP.

2. Amend ADR-0036 D1 and consequences:
   - the erosion hourly quantum may use the admitted power-equivalent rectangular forcing in the V1 domain;
   - the hourly water surface and public peak remain unchanged;
   - no five-minute erosion solves are introduced;
   - multi-OFE, runon, melt, HBP, and routing remain on current authority;
   - raw maximum remains rejected.

3. Do not weaken D2–D5 or Alternative 4.

4. Add stronger contract/source tests before changing the default consumer.

The new tests must fail because production still reads the hourly-mean forcing. Record this as the pre-cutover red gate.

## Milestone 12: Production Cutover

Introduce explicit method types, for example:

    ErosionHourlyReductionMethod::HourlyMeanV0
    ErosionHourlyReductionMethod::PowerVolumeEquivalentV1

Use the final names selected by the contracts.

Do not expose the exponent as a user-settable float.

For an eligible V1 hour:

- the real hourly Wave-1 consumer reads the admitted equivalent rate;
- the real consumer reads the admitted equivalent duration where required;
- authoritative hourly runoff depth remains unchanged;
- rainfall `effint` and `effdrr` remain unchanged;
- the number and chronological order of hourly solves remain unchanged;
- public peak remains the maximum hourly mean;
- HBP continues to serialize 24 hourly volumes and masses;
- production metadata identifies `PowerVolumeEquivalentV1`;
- missing required five-minute shape is a typed failure.

For a non-applicable process domain:

- use `HourlyMeanV0`;
- record an explicit not-applicable reason;
- do not call it a fallback;
- do not emit a warning for ordinary declared non-applicability.

The retained V0 selector is a bounded rollback/comparator mode for one transition window. It must not silently activate after a V1 error.

### Real consumer proof

Create an end-to-end fixture in which:

- hourly runoff depth is identical;
- the five-minute shape differs;
- arithmetic mean is identical;
- the admitted equivalent rate differs materially;
- the Wave-1 production output differs in the predicted direction;
- WB16/public peak and HBP remain identical;
- the diagnostic candidate and cutover production results match exactly.

Document:

    hyetograph
      -> five-minute isolated Green–Ampt shape
      -> hourly mass closure
      -> power-equivalent producer
      -> day/frame handoff
      -> real Wave-1 production consumer
      -> production erosion publication

Also prove the old hourly-mean scalar does not carry the eligible production claim.

## Anti-Regression Guards

Add or strengthen tests that enforce:

1. WB16 peak remains `max(hourly depth / 3600)`.
2. Public peak remains exactly one area conversion.
3. HBP remains a 24-slot hourly surface.
4. No five-minute output field enters HBP serialization or parsing.
5. Watershed routing does not read five-minute generation fields.
6. Raw maximum five-minute generation cannot be selected by production.
7. The fixed exponent is a named contract constant, not runtime input.
8. Rainfall `erosion_effective_intensity` retains its current semantics.
9. Five-minute diagnostics off produce exact baseline outputs.
10. Five-minute diagnostics on change only the requested additive output.
11. Candidate shadow execution cannot mutate persistent production state.
12. V1 missing-shape behavior is typed failure, not V0 fallback.
13. Multi-OFE and positive additional-supply cases remain explicitly non-applicable.
14. Existing peak-authority assertions are preserved.
15. No test, fixture, or authority lane was deleted or weakened to admit cutover.

Add a source-level exclusion test that searches HBP and watershed-routing production paths for the new subhourly forcing symbols. It supplements, but does not replace, byte and consumer tests.

## Performance and Storage Gates

Measure a fixed Topanga baseline set with:

- old source;
- new source, diagnostics disabled;
- five-minute output enabled;
- candidate shadow enabled;
- final cutover, output disabled;
- final cutover, output enabled.

Required:

- diagnostics-disabled median runtime overhead no greater than 1%;
- diagnostics-disabled peak RSS overhead no greater than 1%;
- final V1 cutover median runtime overhead no greater than 15%;
- final V1 cutover peak RSS overhead no greater than 10%;
- no unbounded per-day accumulation;
- streaming output with bounded row-group buffers;
- projected output rows and compressed bytes reported for one 45-year hillslope and the complete 140-hillslope Topanga set.

If performance exceeds these limits, optimize data movement and allocation. Do not approximate or weaken physics to meet the gate.

## Validation Commands

Use an absolute external scratch directory:

    export TMPDIR=/home/workdir/openwepp-task-tmp
    mkdir -p "$TMPDIR"

Run applicable focused commands as files and targets are created. At minimum:

    cargo fmt --all -- --check

    cargo clippy \
      -p openwepp-hillslope-orchestrator \
      -p openwepp-hillslope-output \
      -p openwepp-runner \
      --all-targets --all-features -- -D warnings

    cargo nextest run --test peak_hourly_authority_contract

    cargo nextest run --test subhourly_generation_contract

    cargo nextest run --test subhourly_generation_properties

    cargo nextest run --test subhourly_water_output_roundtrip

    cargo nextest run --test power_equivalent_erosion_contract

    cargo nextest run --test power_equivalent_real_consumer

    cargo nextest run --test hbp_subhourly_exclusion_contract

    cargo nextest run --workspace --profile quick

At exact terminal cutover head:

    cargo nextest run --workspace --profile full

    cargo test --doc --workspace

Run `cargo deny check` if any manifest, lockfile, dependency, source policy, or workspace membership changes.

Run:

    git diff --check

Run the repository’s canonical Markdown, path, contract-index, schema, and placeholder/stub checks discovered from the current adjacent packages.

If authority-suite bindings, required-case registries, or anti-evasion posture change, also run:

    bash tools/release/check_authority_suite_antievasion.sh

    cargo nextest run --test auth11_required_suite_obligation_guards_contract

Run package-local Python tooling with:

    .venv/bin/python -m pytest \
      docs/work-packages/20260810-five-minute-generation-power-equivalent-cutover-001/tools

Every evidence artifact must record:

- exact command;
- working directory;
- source commit and dirty-state identity;
- relevant input/config hashes;
- start/end or duration for expensive work;
- exit status;
- log path;
- requirement satisfied.

## Topanga Cutover Re-Execution

Shadow results cannot close cutover.

After the production selector changes, rebuild at the exact candidate head and rerun all 1,088 Topanga trials through the real production consumer.

Required cutover checks:

- 1,088 of 1,088 terminal trials;
- exact input and mutation-plan identity;
- protected water, public peak, and HBP equality to the pre-cutover production arm;
- production V1 result equals the prior diagnostic-candidate result for every eligible event;
- explicit V0 not-applicable provenance for every ineligible event;
- no silent fallback;
- no new unresolved mutation discontinuity;
- all sediment and rill-width differences attributable to the admitted forcing;
- no routed-watershed or observed-flow claim.

Write compact summaries to:

    artifacts/topanga-cutover-study.md
    artifacts/topanga-cutover-summary.json

## Review and Finding Disposition

Required independent reviews:

1. Hydrology/science reviewer A:
   - five-minute water closure;
   - Green–Ampt state continuity;
   - hourly mass authority;
   - exponent and reduction authority;
   - source-completeness;
   - Topanga claim boundary.

2. Hydrology/science reviewer B:
   - independent derivation of the reduction;
   - fixed-hour versus two-moment choice;
   - rate/duration semantics;
   - mutation and phase sensitivity;
   - adoption legitimacy.

3. `rust_code_reviewer`:
   - production correctness;
   - state isolation;
   - typed errors;
   - no fallback;
   - real consumer;
   - line count.

4. `rust_qa_reviewer`:
   - test independence;
   - anti-tautology;
   - protected output identity;
   - source guards;
   - full-suite legitimacy.

Every finding is dispositioned as:

    accepted
    rejected
    deferred
    follow-up

Accepted findings must be fixed and reverified. Rejected findings require rationale. No finding may remain undispositioned.

A deferred or follow-up finding cannot concern a current required gate. Current-gate findings block completion.

## Verification

Two independent terminal verifiers must inspect the exact final source identity.

Verifier A checks:

- package requirements;
- adoption legitimacy;
- contract-first sequence;
- exact-head commands;
- Topanga identities;
- protected boundaries;
- real-consumer proof;
- no hidden fallback.

Verifier B independently reconstructs:

- five-minute rainfall depth;
- raw diagnostic water closure;
- hourly closed generation;
- power integral;
- equivalent rate and duration;
- public peak invariance;
- selected fixture sediment result;
- Topanga summary metrics.

Both must return `PASS` for an adopted cutover.

## Line-Count Governance

Record line counts for every changed `.rs` file.

- 2,000 or more lines: `WARN`; record decomposition rationale and a named split intent.
- 3,000 or more lines: closure-blocking unless generated/fixture status is explicitly approved with owner and sunset.
- Prefer a new focused subhourly module over adding hundreds of lines to `runoff.rs`.

Reviews and verification must check this artifact.

## Security Impact

Expected security impact is low, but explicitly verify:

- no new network access;
- no secret or credential handling;
- output paths use existing path-safety conventions;
- no environment variable silently changes production method;
- malformed output paths return typed errors;
- output creation does not overwrite unrelated files unexpectedly;
- no unbounded memory growth from subhourly rows.

Record the result in the final disposition.

## Idempotence and Recovery

All diagnostic tools must be resumable and content-addressed.

- Do not overwrite an evidence root with a different source or configuration identity.
- Refuse mixed binary, plan, exponent, schema, or input hashes.
- Write temporary Parquet files and atomically rename after successful close.
- Resume only terminally incomplete trials.
- Reaggregation of immutable trial outputs must produce identical summary hashes.
- A failed cutover is recovered by restoring the explicit production selector to `HourlyMeanV0`; do not delete diagnostic evidence.
- Do not rewrite historical prior-package evidence.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes spawning/delegating to bounded read-only hydrology/source investigators, two independent hydrology/science reviewers, `rust_code_reviewer`, `rust_qa_reviewer`, two independent terminal verifiers, and `comparator_suite_runner`.

Expected outputs are compact package-local findings, review artifacts, verification artifacts, command logs, and immutable external cohort evidence.

Write limits:

- source investigators: read-only;
- science reviewers: only their named package artifacts;
- Rust reviewers: only their named package artifacts;
- terminal verifiers: only their named package artifacts;
- comparator suite runner: package logs and external evidence roots only;
- parent executor: contracts, production source, tests, package plan, and disposition.

Use `comparator_suite_runner` for full Topanga execution, full-workspace nextest, doctests, and other heavy batch gates. If that role is unavailable, record the reason and execute an equivalent local command only when package governance permits it.

## Exact-Diff Reconciliation

Before final disposition:

1. Compare the exact base to final head.
2. Classify every changed file against the owned write set.
3. Identify every executable-semantic change.
4. Identify every changed public or serialized schema.
5. Confirm no HBP or routing file changed.
6. Confirm no Green–Ampt equation or coefficient changed.
7. Confirm no peak-authority assertion was weakened.
8. Confirm every production change has current evidence.
9. Confirm the final adoption disposition agrees with the actual selector.
10. Confirm no untracked result-bearing artifact remains.

Unknown impact is Critical and requires additional validation or removal before closure.

## Terminal Outcomes

### `PASS — ADOPTED AND CUT OVER`

Allowed only when:

- five-minute water output passes;
- `ADOPT` disposition is recorded;
- production contracts and ADR are amended;
- the real eligible consumer reads V1;
- protected water/public/HBP outputs remain unchanged;
- full Topanga cutover execution passes;
- Critical exact-head gates pass;
- all reviews and verifiers pass;
- no findings remain undispositioned.

### `PASS — DIAGNOSTIC WATER OUTPUT, EROSION NOT ADOPTED`

Allowed when:

- the five-minute water output and noninterference gates pass;
- adoption criteria reject the erosion reduction;
- production erosion selector remains unchanged;
- contracts and metadata truthfully describe the diagnostic-only status;
- no shadow or candidate result is published as production.

### `REJECTED`

Required when the five-minute water product itself cannot satisfy closure, provenance, or noninterference.

### `HOLD`

Permitted only for unavailable required evidence, contradictory canonical authority, or a proven out-of-envelope process-family requirement. Effort, runtime, a difficult implementation, or unresolved in-scope analysis are not valid hold boundaries.

Before `HOLD`, write `artifacts/hold-legitimacy-audit.md` naming:

- the boundary;
- evidence proving it;
- the in-scope route considered;
- why that route cannot close;
- the next defect-shaped action.

## Final Acceptance Summary

The package is complete only when a user can run one explicitly documented command and obtain a five-minute event water dataset whose rainfall and generation depths independently reconstruct the model’s authoritative hourly water, while all pre-existing water, peak, HBP, and routing products remain unchanged.

An erosion cutover additionally requires proof that the selected power-equivalent rate and duration are mathematically authorized, prospectively frozen, numerically stable, mutation-continuous on the complete Topanga design, read by the real hourly Wave-1 consumer, and incapable of silently altering non-applicable process domains.
