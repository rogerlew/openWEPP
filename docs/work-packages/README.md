# Work Packages

Initiative tracking convention inherited from wepp-palimpsest. Each work package lives in a dated directory under this tree.

## Directory naming
`YYYYMMDD-<short-slug>/`

## Required files
- `package.md` — scope, deliverables, dependencies, exit criteria
- `prompts/` — agent prompts (active and archived)
- `artifacts/` — produced docs, contracts, evidence

## Autonomous execution intent (required)
- A work package is an execution-ready plan, not a lightweight task note.
- Planning must be front-loaded into the package so execution can proceed
  autonomously from kickoff through disposition without user intervention.
- `package.md` and kickoff prompts must define concrete sequencing, explicit
  file targets, gate commands, and expected evidence updates.
- Kickoff prompts must include an explicit `Autonomy:` line requiring
  end-to-end execution for the declared scope without additional user
  intervention unless hard-blocked.
- Kickoff prompts default to `Execution mode: package-end-to-end` and should
  direct execution across all package phases through disposition.
- Single-phase kickoff prompts are exception-only and must declare
  `Execution mode: phase-only (exception)` plus explicit rationale and
  follow-on trigger.
- Kickoff prompts must include a `Required reading` list with explicit path
  references to orientation and authority documents so agents do not need to
  independently search onboarding context.
- Work-package authoring must reference and follow:
  `docs/codex_exec_plans.md`.

## Phase shape (inherited from wepp-palimpsest)
- **Phase 0**: docs-only audit / inventory
- **Phase 1**: architecture decision with operator-signed acceptance
- **Phase 2**: single-mechanism implementation, replay-and-checkpoint between mechanisms
- **Phase 3**: closeout disposition

## Conventions
- Dates are UTC.
- Evidence classification per claim: `[DIRECT]` (read source / contract / output) vs `[INFERENCE]` (reasoned from evidence).
- Evidence mode per assessment: **Static** (read and reasoned) vs **Ran** (commands actually invoked).
- Single-mechanism rule: one landed change per replay checkpoint.
- Correctness over completion: unresolved contract/invariant correctness gaps keep package disposition in `HOLD` until explicitly resolved or risk-accepted.
- Kernel-affecting packages (including runtime projection controlling kernel branches) must list:
  - `docs/specifications/science-contract-authoring-procedure.md`
  - `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  as dependencies, and must include a kernel-profile compliance checklist artifact.
- Code-authoring work packages should use contract-first sequencing when applicable:
  1. implement/ratify canonical contract amendments,
  2. implement contract-derived tests,
  3. record a pre-implementation contract gate, then
  4. modify production code.
- `package.md` dependencies for authored packages should include:
  - `/workdir/openWEPP/docs/codex_exec_plans.md`
- Missing kernel-profile/procedure compliance keeps disposition in `HOLD`.

## Queued packages

Authorized packages:

- `20260531-auth01-correctness-authority-model-formalization-001/`
  - Purpose: formalize canonical correctness authority ranking and adjudication
    policy, including external-authority constitutive suite authority and
    explicit legacy-parity demotion to investigation-only signal.
- `20260531-auth02-external-authority-constitutive-suite-framework-001/`
  - Purpose: scaffold normative external-authority constitutive suite structure
    (layout, schema, naming, fixture conventions, citation requirements, and
    contract linkage templates) for deterministic implementation.
- `20260531-auth03-level4-constitutive-gate-bootstrap-001/`
  - Purpose: implement first Level-4 constitutive suites and contract-derived
    gates for FC/WP and relax-to-FC behavior as primary correctness adjudicators
    independent of legacy parity matching.
- `20260531-auth04-release-gate-authority-stack-integration-001/`
  - Purpose: integrate authority-stack suite classes into CI/release gates with
    explicit lane and fail-class policy (`required`, `periodic`, `manual`;
    `hard-fail`, `investigation`) and updated release runbook governance.
- `20260531-auth05-level4-constitutive-authority-hardening-001/`
  - Purpose: harden AUTH03 Level-4 constitutive gates by adding runtime
    model-to-authority FC/WP checks on real soil fixtures, removing
    legacy-as-authority citation posture, and making relax-to-FC branch
    assertions non-optional.
- `20260531-auth06-fixture-provenance-hash-enforcement-001/`
  - Purpose: enforce deterministic fixture reproducibility by requiring
    per-suite fixture lockfiles/provenance sidecars, wiring blocking
    release-gate fixture integrity checks, and backfilling active Level-4 suite
    fixture provenance hashes.
- `20260531-auth07-fc-authority-cohort-suite-bootstrap-001/`
  - Purpose: bootstrap an independent direct `theta_fc(-33kPa)` profile-store
    cohort suite with explicit relative-threshold classification and
    rock-fragment-bucket reporting, backed by reproducible fixture
    lock/provenance sidecars.
- `20260531-auth08a-solwpv-branch-gate-authority-retiering-001/`
  - Purpose: re-tier WB19 `solwpv` branch-law suite governance from
    constitutive required/hard-fail posture to periodic/investigation
    legacy-conformance posture, aligned with correctness re-anchoring policy.
- `20260531-auth09-legacy-sanity-tier-normalization-001/`
  - Purpose: establish a canonical Level-3 legacy/sanity external-authority
    tier, re-tier the WB19 `solwpv` branch-conformance suite into that tier,
    and align authority schema/model/registry/contracts/tests so suite ID,
    authority level, and gate posture are coherent.
- `20260531-auth10-fc-authority-gate-and-suite-consistency-001/`
  - Purpose: close AUTH09 review follow-on gaps by (a) completing Level-3
    rename/provenance consistency for the WB19 branch suite, and (b) enforcing
    non-inverted FC direct-theta authority behavior by strengthening Level-4 FC
    gate coverage and demoting discrepancy pinning to non-blocking monitoring.
- `20260531-auth11-required-suite-obligation-and-antievasion-guards-001/`
  - Purpose: add source-level anti-evasion guardrails for suite posture
    changes by enforcing machine-checked required-case anchor obligations,
    diff-based review guards, red/fix/green promotion protocol controls, and
    in-test anchor binding assertions.
- `20260531-auth12-fc-rocky-soil-closure-and-promotion-001/`
  - Purpose: close the direct-theta rocky-soil FC physics discrepancy with
    contract-first kernel remediation and promote the Level-4 cohort from
    `periodic`/`investigation` back to `required`/`hard-fail` only after
    red/fix/green closure evidence is complete.
- `20260601-soilauth01-soil-producer-contract-conformance-audit-001/`
  - Purpose: execute datver-complete (`7778/9002/9003/9005`) `.sol`
    producer-contract conformance audit across openWEPP spec/contract surfaces,
    openWEPP parser/runtime seams, and canonical `wepppy` producer behavior,
    then publish a prioritized closure queue.
- `20260601-soilauth02-soil-producer-contract-correctness-reconciliation-001/`
  - Purpose: reconcile SOILAUTH01 P0/P1 mismatches by applying
    contract-first corrections to openWEPP contract/parser surfaces and
    producer-owned corrections in `wepppy` where required, with fixture hash
    regeneration and provenance evidence.
- `20260601-soilauth03-soil-producer-contract-anti-drift-guards-001/`
  - Purpose: implement machine-checkable anti-drift guards for required `.sol`
    symbol/arity/order obligations and fixture provenance/hash integrity, with
    explicit hard-fail release-lane posture for required checks.
- Execution order for soil producer-contract closure stream is:
  `soilauth01 -> soilauth02 -> soilauth03`.
- Execution order for correctness-authority stream is:
  `auth01 -> auth02 -> auth03 -> auth04 -> auth05 -> auth06 -> auth07 -> auth08a -> auth09 -> auth10 -> auth11 -> auth12`.
- `20260531-hphys0216c-profilefc-normalized-tail-delta-analysis-001/`
  - Purpose: execute immediate HPHYS0216 follow-up diagnostics to explain the
    `ProfileFCStore` regression (`27/39 -> 39/39`) by isolating layer-aggregated
    publication deltas versus normalized-profile seed lineage and publishing a
    concrete remediation handoff package.
- `20260531-hphys0216d-profilefc-normalized-tail-authority-reconciliation-001/`
  - Purpose: execute contract-first FC authority reconciliation by publishing
    explicit normalized-tail contribution (`wb13_profile_fc_tail_mm`) and
    consuming it in WB13 `ProfileFCStore` publication
    (`Σ(thetfc_i*dg_i)*1000 + tail`) with fail-closed guards and workspace
    gate evidence.
- `20260531-hphys0217-post-0216d-coupled-family-rerun-readjudication-001/`
  - Purpose: execute post-HPHYS0216D integrated rerun/adjudication by running a
    fresh `unpalatable-rind` 39-hillslope lane, recomputing residual-family
    diagnostics (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, control
    `ProfileFCStore`), and publishing the next hold-lift disposition plus
    follow-on implementation queue.
- `20260531-hphys0218-wb19-cpm-adjusted-lateral-drain-threshold-closure-001/`
  - Purpose: execute contract-first WB19 lateral/drain remediation by enforcing
    baseline-authoritative `drfc`-equivalent threshold lineage
    (`wb18_perc_fc_#### + (1-cpm_####)*dg_####`) in lateral/drain
    saturated-zone and withdrawal paths, then rerun `unpalatable-rind`
    39-hillslope diagnostics focused on `latqcc` and coupled `Dp`.
- `20260531-hphys0219-wb19-coca-threshold-authority-correction-001/`
  - Purpose: execute contract-first WB19 threshold-authority correction by
    restoring baseline `drfc` coefficient lineage to `coca_####`
    (`wb18_perc_fc_#### + (1-coca_####)*dg_####`), then rerun
    `unpalatable-rind` 39-hillslope diagnostics to re-adjudicate coupled
    `Dp`/`latqcc` residual posture.
- `20260531-hphys0220-wb19-coupled-flux-partition-diagnostics-001/`
  - Purpose: execute post-HPHYS0219 coupled diagnostics to classify
    deterministic cross-family tradeoffs (`Dp` vs `latqcc`/total-soil), audit
    missing baseline WB19 coupling lineage, and hand off contract-first
    remediation scope.
- `20260531-hphys0221-wb19-water-yield-fcdep-coupling-implementation-001/`
  - Purpose: implement baseline-authoritative WB19 water-yield and
    saturated-depth coupling (`solwpv` branch semantics,
    `avpora/avfca/avcoca`, `watyld`, `fcdep`, `unsdep`) with contract-first
    sequencing and 39-hillslope rerun adjudication.
- `20260531-hphys0222-wb19-solwpv-branch-authority-dp-regression-closure-001/`
  - Purpose: correct WB19 saturated-depth mutation authority so `fcdep/unsdep`
    updates apply only for `solwpv < 2006`, add external-authority constitutive
    gating for the branch law, and revalidate workspace/kernel gates.
- `20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/`
  - Purpose: close HPHYS0222 follow-on measurement gap by rerunning
    `unpalatable-rind` 39-hillslope diagnostics and publishing post-change
    residual-family readjudication.
- `20260601-hphys0224-cam-wb19-soilwater-authority-closure-001/`
  - Purpose: resume HPHYS remediation under the Correctness Authority Model by
    closing A0/A1/A3 authority and gate gaps for remaining WB19/soil-water
    residual families (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`,
    `ProfileFCStore`) with contract-first sequencing and post-change cohort
    readjudication.
- `20260601-hphys0225-wb19-available-pool-authority-closure-001/`
  - Purpose: execute follow-on Correctness Authority remediation by removing
    WB19 legacy available-pool max-reconciliation (`max(layer_pool, legacy_term)`),
    codifying layer-derived available-cap authority in canonical contracts, and
    adding required Level-4 external-authority guard coverage for this surface.
- `20260601-hphys0226-residual-family-constitutive-rederive-bootstrap-001/`
  - Purpose: execute immediate follow-on from HPHYS0225 by bootstrapping
    constitutive re-derivation authority for remaining coupled residual
    families via required Level-4 WB19 behavioral gate
    (`cas_l4_subhyd_lateral_saturated_thickness_response_001`) and contract
    linkage under Correctness Authority Model sequencing.
- `20260601-hphys0227-wb19-fcwp-coca-water-yield-authority-closure-001/`
  - Purpose: execute immediate follow-on from HPHYS0226 by closing WB19 FC/WP
    + COCA water-yield coupling authority (`avfca` theta lineage and per-layer
    FC-store consistency) with required Level-4 constitutive gating under
    contract-first sequencing.
- `20260601-hphys0228-wb14-ksatadj-success-lane-restoration-001/`
  - Purpose: execute immediate follow-on from HPHYS0227 by restoring WB14
    disturbed-soil conductivity-adjustment (`ksatadj`) successful-lane
    contract coverage (`solwpv=9001/9002/9003`) under WB19 indexed FC/WP
    prerequisites with typed fail-closed guard posture preserved.
- `20260601-hphys0229-post-0228-cohort-rerun-readjudication-001/`
  - Purpose: execute immediate follow-on from HPHYS0228 by rerunning
    `unpalatable-rind` (`H1..H39`) and publishing current monitored-family
    readjudication deltas (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`,
    `ProfileFCStore`) versus HPHYS0224 baseline before selecting the next
    production remediation slice.
- `20260601-hphys0230-wb18-overdrainage-authority-closure-001/`
  - Purpose: execute immediate follow-on from HPHYS0229 by closing WB18
    percolation over-drainage authority (`Dp` early-transient burst) through
    contract-first migration of baseline-authoritative dynamic `Bi` damping
    and post-change cohort readjudication evidence.
- `20260601-hphys0231-wb18-h7-guard-recovery-and-rerun-001/`
  - Purpose: execute immediate next actions from HPHYS0230 by triaging WB18
    `H7` domain-guard failure with symbol/value diagnostics, reconciling
    guard-placement authority to baseline branch behavior, and rerunning
    `unpalatable-rind` (`H1..H39`) for updated readjudication evidence.
- `20260601-hphys0232-wb18-hourly-lane-percolation-alignment-001/`
  - Purpose: execute immediate next actions from HPHYS0231 by reconciling
    WB18 hourly-lane seepage attenuation authority from legacy
    `watbal_hourly.for`/`purk.for` (`ui_LFtstp`) into the production WB18
    kernel path, then rerunning `unpalatable-rind` (`H1..H39`) for updated
    `Dp` transient readjudication evidence.
- `20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001/`
  - Purpose: execute immediate next actions from HPHYS0232 by migrating the
    baseline-authoritative WB18 daily restrictive-layer conductivity branch
    (`slflag`/`kslast`) and hardening WB13 `D` publication lineage against
    stale state shadowing, then rerunning `unpalatable-rind` (`H1..H39`) for
    updated `Dp` transient readjudication evidence.
- `20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001/`
  - Purpose: execute immediate next actions from HPHYS0233 by hardening WB13
    subsurface publication lineage to flux-authoritative WB19 symbols
    (`q`/`Qdd`/`Qd`) under state/flux conflicts, then rerunning
    `unpalatable-rind` (`H1..H39`) for coupled residual readjudication.
- `20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/`
  - Purpose: execute immediate next actions from HPHYS0234 by isolating and
    evidencing the persistent `Dp` early-transient `~7x` legacy mismatch on
    `H1` (`ui_run=1`) via lane-semantic A/B probes, contract authority
    reanchoring, and implementation-ready hold handoff.
- `20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/`
  - Purpose: execute immediate next actions from HPHYS0235 by landing WB18
    hourly iterative (`24`-substep) production execution semantics, adding a
    contract-derived regression guard against divisor-only single-pass
    behavior, rerunning `unpalatable-rind` (`H1..H39`), and publishing
    readjudication with hold disposition and next-slice handoff.
- `20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/`
  - Purpose: execute immediate next actions from HPHYS0236 by running a
    bulk hourly-lane authority discovery pass that inventories every
    baseline-vs-openWEPP routine requiring iterative substep migration
    (`WB18/WB19/WB14/WB12 sequencing surfaces`) and produces a dispatch-ready
    implementation queue.
- `20260601-hphys0238-wb19-hourly-iterative-lateral-drainage-closure-001/`
  - Purpose: execute immediate next actions from HPHYS0237 by implementing
    WB19 hourly iterative substep execution for lateral/drainage production
    routines (`run_lateral_transfer`, `run_drainage`) with lane-authoritative
    runtime seeding, contract-derived guard coverage, and workspace gate
    evidence.
- `20260601-hphys0239-wb19-wb12-hourly-ordering-handoff-closure-001/`
  - Purpose: execute immediate next actions from HPHYS0238 by closing
    Dispatch-Group-B handoff authority for WB19 -> WB12 -> WB13 through
    canonical ordering contract amendments, contract-derived ordering/anti-shadow
    vectors, and flux-authoritative WB13 publication closure for remaining
    hydrology families (`Q`, `Ep`, `Es`, `Er`).
- `20260601-hphys0240-hourly-runoff-carryover-authority-closure-001/`
  - Purpose: close the HPHYS0239 Dispatch-Group-B residual by implementing
    baseline-authoritative hourly runoff carryover authority and required
    scheduler dependency reconciliation for WB19 -> WB14/WB12 same-pass
    carryover surfaces.
  - Status: completed; Dispatch Group B carryover residual closed, HPHYS stream
    remains in `HOLD` for Dispatch Groups C/D (`hphys0241`, `hphys0242`).
- `20260601-hphys0241-mofe-hourly-carry-arrays-routing-continuity-001/`
  - Purpose: close Dispatch Group C by implementing explicit MOFE hourly
    upstream/runon/lateral carry-array runtime surfaces and routing-continuity
    handoffs for hourly lane mode.
  - Status: executed; explicit array carry, lateral copy-forward, and
    watershed manifest gates implemented. HPHYS stream remains in `HOLD` for
    HPHYS0242 cadence-dependent positive saturation-carry closure and Group D.
- `20260601-hphys0242-wb14-wb12-cadence-ordering-closure-001/`
  - Purpose: close Dispatch Group D by reconciling WB14/WB12 cadence and
    infiltration/ET/runoff/storage observation ordering under hourly lane mode,
    then publishing final HPHYS0239 follow-up HOLD/GO posture.
  - Status: completed; Dispatch Group D closed and HPHYS0239 follow-up
    Dispatch Groups B/C/D are `GO` for the declared hourly cadence/order scope.
- `20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001/`
  - Purpose: execute a fresh post-HPHYS0242 `unpalatable-rind` 39-hillslope
    plus watershed rerun, assess semantic parity evidence, and review prior
    HPHYS residuals to recommend the next focus area.
- `20260602-hphys0244-h1-h7-h39-storage-wb18-lineage-diagnostics-001/`
  - Purpose: execute the first focused post-HPHYS0243 diagnostic work-package
    for `H1`, `H7`, and `H39`, assessing layer state availability
    (`st`/`theta`), `Total-Soil`, `SoilWaterTotal`, and WB18 `Dp`/`Pe`
    lineage evidence before implementation scoping.
- `20260531-hphys0216-profilefc-layer-authority-realignment-001/`
  - Purpose: execute `ProfileFCStore` remediation by realigning WB13 FC
    publication authority to baseline-authoritative layer aggregation
    (`Σ(thetfc_i * dg_i)`), with contract-first amendments, guard-preserving
    runner implementation updates, and 39-hillslope semantic rerun evidence.
- `20260530-hphys0215-coupled-family-remediation-planning-001/`
  - Purpose: decompose integrated HPHYS0214 hold blockers
    (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`) into
    contract-first, implementation-bounded remediation streams with explicit
    ownership, closure measures, and evidence gates for HPHYS0216+ execution.
- `20260530-hphys0214-integrated-hold-lift-readjudication-001/`
  - Purpose: execute integrated post-HPHYS0211/0212/0213 hold-lift
    readjudication by combining process-authority closure evidence, workspace
    gates, and full 39-hillslope semantic diagnostics into a final `HOLD`/`GO`
    decision with explicit ownership for any remaining residual families.
- `20260530-hphys0213-wb12-storage-and-aggregate-reconciliation-closure-001/`
  - Purpose: remediate HPHYS0212 follow-on WB12 storage reconciliation domain
    violations and WB11 aggregate-soil-water continuity defects by enforcing
    physically realizable WB19 withdrawals/flux publication and restoring
    deterministic aggregate updates for `Total-Soil` / `SoilWaterTotal`
    lineage, then rerun gates + unpalatable-rind 39-hillslope diagnostics.
- `20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/`
  - Purpose: remediate HPHYS0211-rooted WB11/WB18 lifecycle reseed defects and
    WB19 control-source defects, restore WB13 `latqcc`/`Tile`/`Qd` coupling
    visibility, and rerun gates + targeted parity diagnostics for `Dp`/`latqcc`
    closure progression.
- `20260530-hphys0211-coupled-threshold-root-cause-ledger-001/`
  - Purpose: execute coupled-threshold residual root-cause decomposition for
    `ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, and `SoilWaterTotal`,
    producing a concrete symbol-path defect ledger and scoped remediation queue
    for HPHYS0212+.
- `20260530-hphys0210-integrated-hold-lift-adjudication-001/`
  - Purpose: execute integrated post-closure hold-lift adjudication by
    combining HPHYS0208/0209 evidence, workspace gate results, and
    confidence-tier diagnostics into a final process-authority-first
    `HOLD`/`GO` decision with explicit ownership for any remaining residuals.
- `20260530-hphys0209-profilewp-near-closed-adjudication-001/`
  - Purpose: isolate and adjudicate the near-closed `ProfileWPStore` residual
    (`1/39`) to determine whether it is unresolved migration defect lineage or
    an expected process-correct delta with authority-backed acceptance.
- `20260530-hphys0208-fc-threshold-coupled-residual-closure-001/`
  - Purpose: close coupled FC-threshold residual families by migrating and
    verifying shared threshold lineage for `ProfileFCStore`, `Dp`, `latqcc`,
    `Total-Soil`, and `SoilWaterTotal` under contract-first sequencing.
- `20260530-hphys0207-fcwp-depth-authority-tail-closure-001/`
  - Purpose: resolve FC/WP depth-authority mismatch after HPHYS0206 by making
    normalized-profile vs parser-layer aggregation depth authority explicit,
    codifying normalized-tail consumption policy, and closing regressions with
    contract-first implementation and rerun deltas.
- `20260530-hphys0206-fcwp-layer-normalization-mapping-closure-001/`
  - Purpose: close remaining FC/WP semantic residual after HPHYS0205 by
    aligning authoritative corrected-layer publication symbols with
    baseline-authoritative layer normalization and deterministic layer mapping
    semantics, with typed fail-closed authority behavior and rerun deltas.
- `20260529-hphys0205-layer-authoritative-fcwp-correction-closure-001/`
  - Purpose: resolve HPHYS0202 residual FC/WP deviations by preserving
    layer-authoritative WB13 publication (`thetfc_####`/`thetdr_####`) while
    migrating baseline-authoritative correction lineage into those
    layer symbols; no rollback to seed-authoritative publication.
- `20260529-hphys0204-disposition-and-diagnostics-001/`
  - Purpose: execute integrated HPHYS follow-up disposition by combining
    process-authority closure evidence, workspace gate results, and semantic
    comparator diagnostics into a HOLD/GO decision where parity is
    investigation signal, not primary gate.
- `20260529-hphys0203-physics-robustness-test-suite-001/`
  - Purpose: implement contract-derived robustness validation for hillslope
    water-balance publication families (conservation-consistent behavior,
    domain guards, perturbation stability, targeted regressions) with parity
    retained as diagnostic evidence.
- `20260529-hphys0202-profile-fc-wp-lineage-closure-001/`
  - Purpose: close `ProfileFCStore`/`ProfileWPStore` follow-up work using
    baseline-authoritative process lineage and contract-derived tests as the
    promotability gate, with semantic parity reruns treated as diagnostics.
- `20260529-hphys0201-physics-first-gate-reframe-001/`
  - Purpose: reframe HPARITY02+ follow-up packages to a physics-first closure
    posture where contract-authoritative process correctness is primary and
    comparator parity is explicitly diagnostic.
- Execution order for active hillslope follow-on is:
  `hphys0201 -> hphys0202 -> hphys0205 -> hphys0206 -> hphys0207 -> hphys0203 -> hphys0204 -> hphys0208 -> hphys0209 -> hphys0210 -> hphys0211 -> hphys0212 -> hphys0213 -> hphys0214 -> hphys0215 -> hphys0216 -> hphys0216c -> hphys0216d -> hphys0217 -> hphys0218 -> hphys0219 -> hphys0220 -> hphys0221 -> hphys0222 -> hphys0223 -> hphys0224 -> hphys0225 -> hphys0226 -> hphys0227 -> hphys0228 -> hphys0229 -> hphys0230 -> hphys0231 -> hphys0232 -> hphys0233 -> hphys0234 -> hphys0235 -> hphys0236 -> hphys0237 -> hphys0238 -> hphys0239 -> hphys0240 -> hphys0241 -> hphys0242 -> hphys0243 -> hphys0244`.
- Legacy parity-centric follow-on packages (`hparity03`/`hparity04`/`hparity05`)
  are retained for historical traceability and are not the default execution
  path unless explicitly re-authorized.
- `20260529-hparity05-unpalatable-rind-column-parity-closeout-001/`
  - Purpose: execute final always-fail-column closeout for `unpalatable-rind`
    by rerunning full 39-hillslope + watershed integration, verifying all 12
    prior always-fail `H.wat` columns are closed, and publishing hold-lift
    disposition evidence.
- `20260529-hparity04-percolation-lateralflow-soilwater-closure-001/`
  - Purpose: close `Dp`, `latqcc`, `SoilWaterTotal`, and `Total-Soil`
    semantic parity deviations using contract-first percolation/subsurface
    lineage closure and cohort rerun evidence.
- `20260529-hparity03-rainmelt-energy-snow-column-closure-001/`
  - Purpose: close `RM`, `Ep`, `Es`, and `Snow-Water` semantic parity
    deviations using baseline-authoritative climate/ET/snow process lineage.
- `20260529-hparity02-profile-capacity-storage-lineage-closure-001/`
  - Purpose: close profile-capacity semantic parity deviations
    (`ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`)
    before downstream flux-family closure.
- `20260529-hparity01-hillslope-wat-always-fail-gap-mapping-001/`
  - Purpose: establish canonical contract provenance, gap matrix mapping, and
    closure-measure scaffolding for the 12 hillslope `H.wat` always-fail
    columns observed in `unpalatable-rind`.
- `20260529-relproc03-release-gate-ci-automation-001/`
  - Purpose: implement and disposition release-gate CI automation for runbook
    gates (`fmt`, `clippy`, `test`, `deny`, release lint, stability cohort)
    with repository-local scripts and workflow wiring.
- `20260529-relproc02-runner-sidecar-emission-cli-001/`
  - Purpose: implement and disposition a dedicated
    `open_wepp_runner release sidecar` command surface for explicit binary
    path/role sidecar emission, and align release contracts/runbook to that
    automation.
- `20260529-relproc01-openwepp-release-procedure-draft-001/`
  - Purpose: draft and disposition a canonical openWEPP release procedure
    documenting candidate assembly, required validation gates, sidecar/lint
    checks, and stability-regression evidence expectations.
- `20260529-hillstab06-wb16-peak-closure-and-p24-climate-triage-001/`
  - Purpose: carry out HILLSTAB05 immediate next actions by closing/reducing
    dominant `HKERNEL-WB16-PEAK-E-003` residuals and triaging/remediating the
    watchlist `p24` climate residual (`HS-SIMPIPE-E-001` `tmax<tmin`), then
    rerun broad hillslope cohorts with delta and disposition reporting.
- `20260528-hillstab05-slope-residual-family-closure-001/`
  - Purpose: close residual slope parser/runtime failure families surfaced by
    HILLSTAB02 (`line 7 col 3` slope token parse, endpoint tolerance,
    cross-OFE boundary mismatch, `HS-RUNTIME-E-023`), then rerun broad
    hillslope cohorts with delta and disposition reporting.
- `20260528-hillstab04-erod14-wave2-domain-closure-001/`
  - Purpose: close residual `HKERNEL-EROD14-WAVE2-E-003` runtime-domain
    failures surfaced by HILLSTAB02 using contract-first sequencing and
    cohort rerun/delta evidence.
- `20260528-hillstab03-wb16-peak-domain-closure-001/`
  - Purpose: close residual `HKERNEL-WB16-PEAK-E-003` runtime-domain failures
    surfaced by HILLSTAB02 using contract-first sequencing and cohort
    rerun/delta evidence.
- `20260528-hillstab02-parser-failure-remediation-and-stability-rerun-001/`
  - Purpose: remediate dominant parser failure families identified in
    HILLSTAB01 (`SOL-E-006`, `MAN-E-009`), run full workspace validation gates,
    and rerun broad hillslope stability cohorts with quantified delta reporting
    and hold-lift disposition.
- `20260528-hillstab01-hillslope-cli-broad-stability-cohorts-001/`
  - Purpose: execute broad release stability validation for `openwepp-cli-hill`
    using the 1166-seed `wepp-forest` cohort and the single-hillslope
    release-gate watchlist, with reproducible per-case evidence and
    GO/HOLD disposition.
- `20260528-hillbench01-hillslope-cli-release-benchmark-optimization-001/`
  - Purpose: benchmark `openwepp-cli-hill` release performance for single-OFE
    and multi-OFE lanes, compare runtime against
    `wepp_260430_baseline/release/wepp_260430_hill`, and land scoped
    hillslope-CLI/runtime hot-path optimizations with repeatable before/after
    evidence.
- `20260511-openwepp-runner-bootstrap/`
  - Purpose: establish runner boundary, release-sidecar contract, and release
    lint gates before kernel implementation.
- `20260520-arch01-subsystem-map-and-contract-spine/`
  - Purpose: architecture discovery for subsystem boundaries, state-surface
    ownership, top-down invariant cataloging, legacy `.run`/sidecar
    compatibility bridge definition, and comparator confidence-tier policy.
- `20260520-sci01-50201000-process-contract-mapping/`
  - Purpose: map `references/50201000` chapters to process-based science
    contract domains and seed invariant families for top-down contract
    authoring.
- `20260520-sci02-author-sc-plant-001/`
  - Purpose: author and disposition `SC-PLANT-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci03-author-sc-climate-001/`
  - Purpose: author and disposition `SC-CLIMATE-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci04-author-sc-watbal-001/`
  - Purpose: author and disposition `SC-WATBAL-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci05-author-sc-snowfreeze-001/`
  - Purpose: author and disposition `SC-SNOWFREEZE-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci06-author-sc-runoffpart-001/`
  - Purpose: author and disposition `SC-RUNOFFPART-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci07-author-sc-evap-001/`
  - Purpose: author and disposition `SC-EVAP-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci08-author-sc-perc-001/`
  - Purpose: author and disposition `SC-PERC-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci09-author-sc-subhyd-001/`
  - Purpose: author and disposition `SC-SUBHYD-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci10-author-sc-soil-001/`
  - Purpose: author and disposition `SC-SOIL-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci11-author-sc-residue-001/`
  - Purpose: author and disposition `SC-RESIDUE-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci12-author-sc-hydraulics-001/`
  - Purpose: author and disposition `SC-HYDRAULICS-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci13-author-sc-sed-001/`
  - Purpose: author and disposition `SC-SED-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci14-author-sc-irrig-001/`
  - Purpose: author and disposition `SC-IRRIG-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci15-author-sc-route-001/`
  - Purpose: author and disposition `SC-ROUTE-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci16-author-sc-impound-001/`
  - Purpose: author and disposition `SC-IMPOUND-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-sci17-author-sc-system-001/`
  - Purpose: author and disposition `SC-SYSTEM-001` using the required
    dual-agent review and fix-verification workflow.
- `20260520-obs01-observability-subsystem-foundation/`
  - Purpose: define first-class observability subsystem architecture for
    kernel stimulation, structured traces, replay windows, and migration away
    from ad-hoc `wepp_observe*` debug sidecars.
- `20260520-infile01-author-sc-infile-climate-001/`
  - Purpose: author and disposition `SC-INFILE-CLIMATE-001` and canonical
    climate input specification (`.cli`).
- `20260520-infile02-author-sc-infile-soil-001/`
  - Purpose: author and disposition `SC-INFILE-SOIL-001` and canonical soil
    input specification (`.sol`).
- `20260520-infile03-author-sc-infile-management-001/`
  - Purpose: author and disposition `SC-INFILE-MANAGEMENT-001` and canonical
    management input specification (`.man`).
- `20260520-infile04-author-sc-infile-slope-001/`
  - Purpose: author and disposition `SC-INFILE-SLOPE-001` and canonical slope
    input specification (`.slp`).
- `20260520-infile05-author-sc-infile-watershed-structure-001/`
  - Purpose: author and disposition `SC-INFILE-WATERSHED-STRUCTURE-001` and
    canonical watershed structure specification (`.str`).
- `20260520-infile06-author-sc-infile-watershed-channel-001/`
  - Purpose: author and disposition `SC-INFILE-WATERSHED-CHANNEL-001` and
    canonical watershed channel specification (`.chn`).
- `20260520-infile07-author-sc-infile-watershed-impoundment-001/`
  - Purpose: author and disposition `SC-INFILE-WATERSHED-IMPOUNDMENT-001` and
    canonical watershed impoundment specification (`.imp`).
- `20260520-infile08-author-sc-infile-irrigation-depletion-001/`
  - Purpose: author and disposition `SC-INFILE-IRRIGATION-DEPLETION-001` and
    canonical depletion-irrigation sidecar specification.
- `20260520-infile09-author-sc-infile-irrigation-fixeddate-001/`
  - Purpose: author and disposition `SC-INFILE-IRRIGATION-FIXEDDATE-001` and
    canonical fixed-date irrigation sidecar specification.
- `20260520-infile10-author-sc-infile-pmetpara-001/`
  - Purpose: author and disposition `SC-INFILE-PMETPARA-001` and canonical
    `pmetpara.txt` specification.
- `20260520-infile11-author-sc-infile-snow-001/`
  - Purpose: author and disposition `SC-INFILE-SNOW-001` and canonical
    `snow.txt` specification.
- `20260520-infile12-author-sc-infile-frost-001/`
  - Purpose: author and disposition `SC-INFILE-FROST-001` and canonical
    `frost.txt` specification.
- `20260520-infile13-author-sc-infile-gwcoeff-001/`
  - Purpose: author and disposition `SC-INFILE-GWCOEFF-001` and canonical
    `gwcoeff.txt` specification.
- `20260520-infile14-author-sc-infile-phosphorus-001/`
  - Purpose: author and disposition `SC-INFILE-PHOSPHORUS-001` and canonical
    `phosphorus.txt` specification.
- `20260520-infile15-author-sc-infile-weppui-001/`
  - Purpose: author and disposition `SC-INFILE-WEPPUI-001` and canonical
    `wepp_ui.txt` specification.
- `20260520-infile16-author-sc-infile-tc-001/`
  - Purpose: author and disposition `SC-INFILE-TC-001` and canonical
    `tc.txt` specification.
- `20260520-infile17-author-sc-infile-tcr-001/`
  - Purpose: author and disposition `SC-INFILE-TCR-001` and canonical
    `tcr.txt` specification.
- `20260520-infile18-author-sc-infile-lcwb-001/`
  - Purpose: author and disposition `SC-INFILE-LCWB-001` and canonical
    `lcwb.txt` specification.
- `20260520-infile19-author-sc-infile-chaninp-001/`
  - Purpose: author and disposition `SC-INFILE-CHANINP-001` and canonical
    `chan.inp` specification.
- `20260521-inimpl01-prioritize-parser-implementation-order/`
  - Purpose: prioritize implementation order for all active `SC-INFILE-*`
    parser surfaces and produce dependency-aware implementation waves plus
    follow-on implementation work-package queue proposals.
- `20260521-inimpl02-wave1-worktree-orchestration-001/`
  - Purpose: establish Wave 1 shared scaffold governance for parallel agent
    worktrees, including ownership manifests and integration sequencing rules.
- `20260521-inimpl03-implement-sc-infile-slope-parser-001/`
  - Purpose: implement `SC-INFILE-SLOPE-001` parser surface in a dedicated
    worker worktree.
- `20260521-inimpl04-implement-sc-infile-soil-parser-001/`
  - Purpose: implement `SC-INFILE-SOIL-001` parser surface in a dedicated
    worker worktree.
- `20260521-inimpl05-implement-sc-infile-climate-parser-001/`
  - Purpose: implement `SC-INFILE-CLIMATE-001` parser surface in a dedicated
    worker worktree.
- `20260521-inimpl06-implement-sc-infile-management-parser-001/`
  - Purpose: implement `SC-INFILE-MANAGEMENT-001` parser surface in a
    dedicated worker worktree.
- `20260521-inimpl07-wave1-core-parser-integration-001/`
  - Purpose: integrate Wave 1 worker outputs and close global Wave 1
    validation gates.
- `20260521-inimpl09-management-full-typed-datamodel-001/`
  - Purpose: close `SC-INFILE-MANAGEMENT-001` execution HOLDs by implementing a
    full typed `.man` datamodel across spec, parser contract, parser code, and
    fixtures/tests.
- `20260521-inimpl10-wave2-worktree-orchestration-001/`
  - Purpose: establish Wave 2 concurrent worktree governance, ownership
    manifests, and integration sequencing for sidecar parser surfaces.
- `20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/`
  - Purpose: implement `SC-INFILE-PMETPARA-001` parser surface in a dedicated
    worker worktree.
- `20260521-inimpl12-implement-sc-infile-irrigation-depletion-parser-001/`
  - Purpose: implement `SC-INFILE-IRRIGATION-DEPLETION-001` parser surface in a
    dedicated worker worktree.
- `20260521-inimpl13-implement-sc-infile-irrigation-fixeddate-parser-001/`
  - Purpose: implement `SC-INFILE-IRRIGATION-FIXEDDATE-001` parser surface in a
    dedicated worker worktree.
- `20260521-inimpl14-implement-sc-infile-frost-parser-001/`
  - Purpose: implement `SC-INFILE-FROST-001` parser surface in a dedicated
    worker worktree.
- `20260521-inimpl15-implement-sc-infile-snow-parser-001/`
  - Purpose: implement `SC-INFILE-SNOW-001` parser surface in a dedicated
    worker worktree.
- `20260521-inimpl16-implement-sc-infile-weppui-parser-001/`
  - Purpose: implement `SC-INFILE-WEPPUI-001` parser surface in a dedicated
    worker worktree.
- `20260521-inimpl17-wave2-sidecar-parser-integration-001/`
  - Purpose: integrate Wave 2 worker outputs and close global Wave 2
    validation gates.
- `20260521-inimpl18-wave3-worktree-orchestration-001/`
  - Purpose: establish Wave 3 concurrent worktree governance, ownership
    manifests, and integration sequencing for watershed-core parser surfaces.
- `20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/`
  - Purpose: implement `SC-INFILE-WATERSHED-STRUCTURE-001` parser surface in a
    dedicated worker worktree.
- `20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/`
  - Purpose: implement `SC-INFILE-WATERSHED-CHANNEL-001` parser surface in a
    dedicated worker worktree.
- `20260521-inimpl21-implement-sc-infile-watershed-impoundment-parser-001/`
  - Purpose: implement `SC-INFILE-WATERSHED-IMPOUNDMENT-001` parser surface in a
    dedicated worker worktree.
- `20260521-inimpl22-wave3-core-parser-integration-001/`
  - Purpose: integrate Wave 3 worker outputs and close global Wave 3
    validation gates.
- `20260522-inimpl23-wave4-worktree-orchestration-001/`
  - Purpose: establish Wave 4 concurrent worktree governance, ownership
    manifests, and integration sequencing for watershed-sidecar parser
    surfaces (`chan.inp`, `tc`, `gwcoeff`, `phosphorus`, `tcr`, `lcwb`).
- `20260522-inimpl24-implement-sc-infile-chaninp-parser-001/`
  - Purpose: implement `SC-INFILE-CHANINP-001` parser surface in a dedicated
    worker worktree.
- `20260522-inimpl25-implement-sc-infile-tc-parser-001/`
  - Purpose: implement `SC-INFILE-TC-001` parser surface in a dedicated worker
    worktree.
- `20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/`
  - Purpose: implement `SC-INFILE-GWCOEFF-001` parser surface in a dedicated
    worker worktree.
- `20260522-inimpl27-implement-sc-infile-tcr-parser-001/`
  - Purpose: implement `SC-INFILE-TCR-001` parser surface in a dedicated
    worker worktree.
- `20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/`
  - Purpose: implement `SC-INFILE-PHOSPHORUS-001` parser surface in a
    dedicated worker worktree.
- `20260522-inimpl29-implement-sc-infile-lcwb-parser-001/`
  - Purpose: implement `SC-INFILE-LCWB-001` parser surface in a dedicated
    worker worktree.
- `20260522-inimpl30-wave4-sidecar-parser-integration-001/`
  - Purpose: integrate Wave 4 worker outputs and close global Wave 4
    validation gates plus `W4DR-001..012` closure reporting.
- `20260522-inimpl31-implement-sc-infile-hbp-parser-001/`
  - Purpose: implement `SC-INFILE-HBP-001` parser surface, including owned HBP
    specification/contract, parser wiring, and integration tests aligned with
    existing `SC-INFILE-*` execution patterns.
- `20260521-arch02-simulation-subsystem-kernel-architecture-discovery/`
  - Purpose: investigate simulation/subsystem/kernel architecture requirements
    via `wepp-forest` pattern extraction, `/workdir/rancor` architecture
    assessment, and Rust exemplar comparison; publish an openWEPP ownership and
    orchestration proposal before Wave 4 ratification.
- `20260521-arch03-sim-contract-crate-and-status-taxonomy-001/`
  - Purpose: implement the foundational simulation contract crate and typed
    status taxonomy (including closure primitives and canonical symbol alias
    registry) required to unblock downstream ARCH04+ implementation packages.
- `20260521-arch04-topology-graph-and-validation-gate-001/`
  - Purpose: implement typed watershed/hillslope topology graph modeling and a
    deterministic pre-execution validation gate wired to ARCH03 status/closure
    contracts.
- `20260521-arch05-hillslope-phase-scheduler-graph-001/`
  - Purpose: implement deterministic hillslope phase scheduler graph
    orchestration with typed precondition enforcement using ARCH03/ARCH04
    contract surfaces.
- `20260521-arch06-watershed-dispatch-scheduler-graph-001/`
  - Purpose: implement deterministic watershed dispatch scheduler graph
    orchestration with typed precondition enforcement using ARCH03/ARCH04
    contract surfaces.
- `20260521-arch07-kernel-trait-boundary-and-writeback-contract-001/`
  - Purpose: implement shared kernel trait boundaries and orchestrator-owned
    writeback contracts for hillslope/watershed execution surfaces.
- `20260521-arch08-sidecar-and-legacy-bridge-adapter-isolation-001/`
  - Purpose: isolate legacy sidecar/HBP compatibility behavior into dedicated
    edge adapter modules so core kernels/orchestrators remain process-focused.
- `20260521-arch09-unit-safe-boundary-types-001/`
  - Purpose: introduce unit-safe boundary type wrappers for critical
    runoff/flow/storage/rate interfaces used across kernel/orchestrator seams.
- `20260521-arch10-summary-accumulator-kernelization-001/`
  - Purpose: implement typed daily/monthly/yearly/EOS summary accumulation as
    a standalone kernelized subsystem.
- `20260522-arch11-comparator-tier-routing-metadata-integration-001/`
  - Purpose: implement comparator confidence-tier metadata propagation through
    reporting/comparator outputs aligned to ADR-0011 governance tiers.
- `20260522-arch12-wave4-readiness-closeout-001/`
  - Purpose: execute Wave 4 architecture readiness closeout and ratification
    by validating ARCH03..ARCH11 gate/disposition closure and issuing GO/HOLD.
- `20260522-arch13-wave4-hold-ratification-checklist-001/`
  - Purpose: ratify outstanding Wave 4 parser/sidecar HOLD decisions with
    explicit decision records and kickoff acceptance criteria.
- `20260522-arch14-claude-architecture-review-disposition-001/`
  - Purpose: normalize/disposition external architecture review findings
    (`CRF-001..010`) with dual review/verification gates and publish a
    dependency-ordered remediation package queue.
- `20260522-arch15-typed-kernel-state-and-unit-boundary-seam-001/`
  - Purpose: implement `CRF-001`/`CRF-002` closure by replacing stringly
    kernel seam maps with typed symbol/value boundaries and wiring
    `openwepp-unit-boundary` into hillslope/watershed kernel seam surfaces.
- `20260522-arch16-scheduler-hot-path-surface-optimization-001/`
  - Purpose: implement `CRF-003` hot-path optimization by reducing
    scheduler clone/allocation pressure while preserving typed seam and
    deterministic writeback/status semantics.
- `20260522-arch17-parser-to-simulation-seam-ownership-and-integration-closure-001/`
  - Purpose: implement `CRF-005`/`CRF-010` closure via explicit
    parser-to-simulation seam ownership contracts and runtime integration
    acceptance evidence.
- `20260522-arch18-hbp-authority-and-convergence-closure-001/`
  - Purpose: implement `CRF-006` closure by defining HBP authority split,
    convergence constraints, and provenance pin evidence.
- `20260522-arch19-run-and-parquet-boundary-contracts-001/`
  - Purpose: implement `CRF-007` by authoring canonical top-level `.run` and
    parquet boundary contracts with explicit schema authority and cross-file
    parser/runtime closure mapping.
- `20260522-arch20-governance-throughput-and-build-hygiene-controls-001/`
  - Purpose: implement `CRF-008`/`CRF-009` by defining governance throughput
    rubric, WIP/closure policy, and workspace build-discipline controls.
- `20260522-arch21-architecture-review-re-closeout-001/`
  - Purpose: re-close ARCH14 by reconciling `CRF-001..010` closure evidence,
    replaying workspace gates, and issuing explicit ARCH14 hold-release
    disposition (`GO_ARCH14_RELEASED` or `HOLD_ARCH14_PENDING`).
- `20260522-clim01-wepp-forest-climate-model-discovery-and-specification-001/`
  - Purpose: reconstruct exact legacy `wepp-forest` climate model behavior
    for continuous-daily and breakfile flows, then author an openWEPP-owned
    detailed climate spec, consumer requirements, and parser/architecture
    integration mapping (single-storm explicitly excluded).
- `20260522-clim02-climate-parser-to-runtime-seam-adapters-001/`
  - Purpose: implement `HS-CLIM-SEAM-001`/`WS-CLIM-SEAM-001` climate
    parser-to-runtime adapters with typed `CLIM-RUNTIME-E-*` errors,
    `datver=0.0` override + `datver>=4.0` policy guards, and
    integration-test closure evidence.
- `20260522-clim03-continuous-daily-climate-runtime-kernel-port-001/`
  - Purpose: port continuous-daily (`ibrkpt=0`) legacy climate runtime
    behavior (including disaggregation/event-shape semantics and versioned
    `iclig` branch policy) into typed openWEPP runtime forcing with
    `/wc1/runs/**/wepp/runs/*.cli` fixture-backed parity evidence.
- `20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/`
  - Purpose: port breakpoint (`ibrkpt=1`) runtime semantics (`stmstr`,
    elapsed-time normalization, interval intensities) and reconcile ratified
    `1500` cardinality + strict `dtime>0` interval-timing policy with explicit
    compatibility controls and `/wc1/runs/**/wepp/runs/*.cli` fixture evidence.
- `20260522-clim11-climate-ownership-boundary-reconciliation-001/`
  - Purpose: reconcile climate forcing ownership boundary between hillslope and
    watershed orchestration layers and publish explicit routing authority.
- `20260522-clim12-shared-climate-runtime-adapter-extraction-001/`
  - Purpose: remove duplicated climate runtime seam logic by extracting a
    shared single-owner adapter surface consumed by both orchestrators.
- `20260522-clim13-typed-climate-forcing-surface-closure-001/`
  - Purpose: close typed-state drift by replacing dynamic breakpoint forcing
    key synthesis with explicit typed climate forcing surfaces.
- `20260522-clim14-runtime-breakpoint-cardinality-policy-closure-001/`
  - Purpose: align runtime breakpoint cardinality behavior with the ratified
    `1500` policy and codify strict vs override compatibility semantics.
- `20260522-clim15-climate-runtime-error-taxonomy-reachability-reconciliation-001/`
  - Purpose: reconcile climate runtime error taxonomy with reachable guard
    paths, including cleanup of unreachable/misnamed variants.
- `20260522-clim16-climate-governance-register-normalization-001/`
  - Purpose: normalize CLIM disposition/register vocabulary and reconcile stale
    status drift after CLIM11..15 closure, including corrected `0.70`
    governance framing and explicit `datver>=4.0` branch-policy confirmation.
- `20260522-sr02-slope-runtime-seam-contract-and-builder-001/`
  - Purpose: implement SR01 follow-on `SR02` by defining and building the
    typed slope parser-to-runtime seam for hillslope orchestration, including
    explicit symbol projection guards and integration-test closure evidence.
- `20260522-sr03-soil-runtime-seam-expansion-001/`
  - Purpose: implement SR01 follow-on `SR03` by expanding the soil
    parser-to-runtime seam from minimal seed symbols to contracted
    layer/profile runtime surfaces required by soil and hydrology consumers.
- `20260522-sr04-symbol-alias-continuity-completion-001/`
  - Purpose: implement SR01 follow-on `SR04` by expanding canonical slope+soil
    symbol alias continuity tables and `openwepp-sim-contract` registry
    coverage after SR02/SR03 seam delivery.
- `20260522-sr05-parser-to-runtime-integration-closure-001/`
  - Purpose: implement SR01 follow-on `SR05` by adding integration closure
    tests proving slope+soil parser outputs propagate into runtime scheduler
    surfaces with typed failures and no silent defaults.
- `20260522-sr06-consumer-ownership-wiring-hillslope-kernels-001/`
  - Purpose: implement SR01 follow-on `SR06` by wiring slope+soil runtime
    surfaces into hillslope consumer boundaries (runoff/soil/watbal/perc)
    with typed error propagation only.
- `20260522-sr07-comparator-confidence-tier-delta-review-001/`
  - Purpose: implement SR01 follow-on `SR07` by running Tier-A
    single-OFE daily water-balance comparator delta review after SR06 to
    validate semantic-parity direction under confidence-tier policy.
- `20260522-pl01-plant-landuse-growth-decomposition-model-representation-discovery-001/`
  - Purpose: discover and map plant/landuse/growth/decomposition
    representations downstream of `.man` surfaces, including consumer
    ownership boundaries, architecture-fit analysis, and follow-on queue
    sequencing.
- `20260522-pl02-plant-runtime-boundary-contract-001/`
  - Purpose: implement PL01 follow-on `PL02` by authoring the typed
    plant/landuse/growth/decomposition runtime boundary contract, ownership
    matrix, canonical symbol alias requirements, and strict parser-to-runtime
    seam requirements for PL03+ execution.
- `20260522-pl03-management-to-runtime-adapter-001/`
  - Purpose: implement PL01/PL02 follow-on `PL03` by building the strict
    typed management-to-runtime adapter (`PL-MAN-SEAM-001`) that projects
    parser outputs into scheduler-facing PL runtime surfaces with typed errors
    and no silent defaults.
- `20260522-pl04-pl-symbol-alias-completion-001/`
  - Purpose: implement PL01/PL02 follow-on `PL04` by expanding
    `openwepp-sim-contract` canonical alias registry coverage for PL runtime
    symbols and validating deterministic alias resolution behavior.
- `20260522-pl05-growth-kernel-surface-scaffolding-001/`
  - Purpose: implement PL01/PL02/PL03/PL04 follow-on `PL05` by adding typed
    growth-kernel interfaces and placeholder annual/perennial scheduler phases
    for deterministic growth state transitions.
- `20260522-pl06-decomposition-resup-kernel-surface-scaffolding-001/`
  - Purpose: implement PL01/PL02/PL03/PL04 follow-on `PL06` by adding typed
    decomposition/resup kernel interfaces and scheduler scaffolding for residue
    partition transitions while preserving baseline phase order.
- `20260522-pl07-parser-to-runtime-integration-tests-001/`
  - Purpose: implement PL01/PL03/PL04 follow-on `PL07` by adding fixture-backed
    integration tests that assert full PL runtime surface projection from `.man`
    inputs, including typed reject paths.
- `20260522-pl08-comparator-confidence-tier-review-001/`
  - Purpose: implement PL01/PL05/PL06/PL07 follow-on `PL08` by running
    single-OFE daily water-balance and plant/residue comparator parity review
    with confidence-tier disposition semantics.
- `20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/`
  - Purpose: assess total implemented openWEPP PL-relevant surfaces and perform
    baseline representation/discovery decomposition to produce a dependency-ordered
    hold-lift work-package queue for resolving `PL08` hold.
- `20260523-pl09a-pre-execution-preconditions-clearance-001/`
  - Purpose: clear Claude pre-execution preconditions (structure-diff
    diagnosis, symbol-wiring clarification, typed-surface strategy decision)
    and acknowledge secondary findings validity before PL10/WB10 queue start.
- `20260523-pl10-active-slot-authority-001/`
  - Purpose: replace hard-coded `slot_0001/crop_0001` dispatch coupling with
    deterministic day-aware active slot/crop authority and typed failure
    behavior for PL growth/decomposition routing.
- `20260523-pl10b-contract-blind-authority-and-conformance-001/`
  - Purpose: run a contract-first blind-authoring gate (implementation-blind
    contract authority, contract-derived tests, conformance run, and gap
    reconciliation) before PL11 implementation work.
- `20260523-pl11-pl-event-runtime-projection-001/`
  - Purpose: project annual/perennial transition-control payload families into
    deterministic PL runtime symbol surfaces with typed guards and mandatory
    kernel-process contract-profile compliance.
- `20260523-pl12-decomp-resup-transition-kernel-001/`
  - Purpose: implement production decomposition/residue transition kernel
    execution with contract-first authority, pre-implementation contract-test
    gating, and typed invariant/guard enforcement.
- `20260523-pl13-growth-transition-kernel-001/`
  - Purpose: implement production annual/perennial growth transition kernel
    execution with contract-first authority, pre-implementation contract-test
    gating, and typed transition/invariant enforcement.
- `20260523-pl13a-alias-continuity-closure-001/`
  - Purpose: close or explicitly disposition canonical PL symbol alias
    continuity (`PL09-GAP-007`) with registry/contract authority updates in a
    parallel governance lane.
- `20260523-wb10-hydrology-phase-kernel-skeleton-001/`
  - Purpose: add production hydrology phase-kernel skeleton entry routing
    (ET/perc/lateral/drainage/runoff/storage classes) with contract-first
    authority and pre-implementation contract-test gating.
- `20260523-wb11-et-perc-lateral-drain-kernels-001/`
  - Purpose: implement ET/percolation/lateral/drainage production kernels with
    typed invariant checks, plus required kernel-contract and contract-test
    implementation evidence.
- `20260523-wb12-runoff-storage-reconciliation-kernels-001/`
  - Purpose: implement runoff/storage reconciliation production kernels with
    explicit closure diagnostics, plus required kernel-contract and
    contract-test implementation evidence.
- `20260523-wb13-daily-water-balance-output-surface-001/`
  - Purpose: emit comparator-ready daily water-balance output surface
    (`H5.wat.dat` equivalent) with required contract and contract-test
    implementation evidence.
- `20260523-int10-plant-water-coupling-validation-001/`
  - Purpose: validate coupled daily execution ordering and state coupling
    (`decomp -> growth -> watbal`) with required contract and contract-test
    implementation evidence.
- `20260523-pl14-tier-a-candidate-emission-and-replay-001/`
  - Purpose: execute strict Tier-A direct openWEPP-vs-legacy comparator replay
    with required comparator JSON artifacts, command trace, provenance hashes,
    and contract/contract-test implementation evidence.
- `20260523-pl15-tier-a-delta-closeout-and-hold-lift-001/`
  - Purpose: disposition residual Tier-A deltas and issue the PL08 hold-lift
    verdict with explicit risk-acceptance references when blockers remain,
    plus required contract/contract-test implementation evidence.
- `20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/`
  - Purpose: re-run strict Tier-A direct openWEPP-vs-legacy comparator replay
    after post-PL15 closure-wave completion with reproducible provenance and
    contract-first sequencing (contract updates, contract tests,
    pre-implementation gate, then replay/harness code).
- `20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001/`
  - Purpose: re-disposition Tier-A deltas from PL14R rerun evidence and issue
    refreshed PL08 hold-lift verdict with contract-first sequencing (contract
    updates, contract tests, pre-implementation gate, then closeout
    decision-surface code).
- `20260524-pl14s-tier-a-openwepp-candidate-emission-and-replay-001/`
  - Purpose: execute semantic-parity Tier-A openWEPP-vs-legacy hillslope
    water-balance replay (erosion excluded) and stand up reusable
    investigation-grade legacy comparison suite tooling for recurring parity
    diagnostics.
- `20260524-simimpl01-hillslope-totality-assessment-and-watbal-consolidation-001/`
  - Purpose: produce a comprehensive legacy-vs-openWEPP hillslope routine gap
    assessment, evaluate watbal implementation source authority
    (`wepp-forest_260430_baseline` vs consolidated `/workdir/wepp-forest`),
    and emit implementation-driving queue deliverable
    `simulation-implementation-wp-queue.md` for the
    `cli -> runner -> simulation -> orchestration` pipeline.
- `20260524-simimpl02-phase-b-full-routine-inventory-and-gap-closure-map-001/`
  - Purpose: execute the full SIMIMPL02 inventory wave by producing a complete
    hillslope routine inventory from baseline legacy sources and an
    evidence-linked owner-surface gap-closure map that drives SIMIMPL03+
    contract-first implementation sequencing.
- `20260524-simimpl03-contract-authority-amendments-for-production-watbal-execution-001/`
  - Purpose: execute SIMIMPL03 contract-first authority closure by amending
    canonical `SC-WATBAL-001`/`SC-SYSTEM-001`/`SC-INFILE-WEPPUI-001` for
    production execution ownership, runtime mode-propagation invariants,
    simulation-owned WB13/H.wat provenance, and consolidated-kernel intake
    guardrails before SIMIMPL04+ code packages.
- `20260524-simimpl04-contract-derived-tests-and-preimplementation-gate-for-runner-kernel-path-001/`
  - Purpose: execute SIMIMPL04 by implementing contract-derived integration
    tests and pre-implementation gate evidence for runner-to-scheduler
    execution closure, `wepp_ui` mode-closure behavior, and
    simulation-owned WB13/H.wat publication requirements before SIMIMPL05
    production edits.
- `20260524-simimpl05-runner-orchestrator-daily-execution-integration-001/`
  - Purpose: execute SIMIMPL05 by integrating production runner daily flow
    with hillslope scheduler/kernel execution, preserving typed error
    propagation and writeback governance to close `GAP-SIMPIPE-001` before
    SIMIMPL06 output-provenance closure work.
- `20260524-simimpl06-simulation-owned-wb13-output-publication-001/`
  - Purpose: execute SIMIMPL06 by replacing projection-first WB13/H.wat
    publication with simulation-owned output assembly and provenance-complete
    reporting surfaces, closing `GAP-SIMOUT-001` and preparing replay
    recloseout in SIMIMPL11.
- `20260524-simimpl07-wepp-ui-hourly-branch-propagation-and-closure-001/`
  - Purpose: execute SIMIMPL07 by propagating parsed `wepp_ui`
    requested/effective mode into runtime lane selection with strict typed
    branch-mismatch closure, closing `GAP-SIMMODE-001` before SIMIMPL09 hourly
    foundation work.
- `20260525-simimpl08-consolidated-kernel-intake-triage-and-provenance-map-001/`
  - Purpose: execute SIMIMPL08 by triaging consolidated watbal candidate
    kernels/adapters/policies (`wbk*` family) against baseline and canonical
    contract authority, producing a bounded `adopt/defer/reject` intake map for
    SIMIMPL09+ implementation planning.
- `20260525-simimpl09-hourly-lane-foundation-and-timestep-policy-surface-001/`
  - Purpose: execute SIMIMPL09 by implementing hourly lane foundation with a
    typed timestep policy surface (`daily`, `hourly`, future sub-hourly
    representable) and adapter-boundary closure using the SIMIMPL08 bounded
    intake allow-list.
- `20260525-simimpl10-winter-soil-frsoil-hydout-coupling-closure-wave-001/`
  - Purpose: execute SIMIMPL10 by closing winter/soil/frsoil/hydout coupling
    gaps in production execution flow with typed invariants, explicit coupling
    boundary provenance, and no silent fallback behavior.
- `20260525-simimpl11-tier-a-semantic-replay-recloseout-and-residual-classification-001/`
  - Purpose: execute SIMIMPL11 by re-running strict + semantic Tier-A replay
    after SIMOUT/SIMCOUP closure and classifying residuals into explicit
    promote/hold posture using confidence-tier governance.
- `20260525-simimpl14-runner-wb13-timeseries-span-and-row-key-closure-001/`
  - Purpose: execute SIMIMPL14 by implementing continuous day-indexed runner
    execution, replay-length WB13/H.wat publication continuity, and candidate
    row-key policy alignment to close SIMIMPL13 continuous-run blockers and
    prepare promotable replay overlap evidence for SIMIMPL15+.
- `20260525-simimpl15-replay-comparator-tooling-alignment-001/`
  - Purpose: execute SIMIMPL15 by aligning replay comparator/tooling behavior
    across strict/semantic lanes, closing parquet alias and provenance
    classification drift, and producing deterministic cross-format parity
    evidence surfaces for SIMIMPL16/SIMIMPL17 closeout waves.
- `20260525-simimpl16-replay-contract-derived-test-coverage-closure-001/`
  - Purpose: execute SIMIMPL16 by closing replay contract-derived test blind
    spots (`SIMIMPL13-TEST-001..005`) with enforceable span/key/alias/
    provenance governance tests so SIMIMPL17 reruns operate with deterministic
    promotion gates.
- `20260525-simimpl17-tier-a-replay-rerun-and-hold-lift-disposition-001/`
  - Purpose: execute SIMIMPL17 by running final Tier-A strict/semantic replay
    reruns after SIMIMPL14/15/16 closure and publishing evidence-backed
    hold-lift disposition against `SIMIMPL13-CRIT-001..008`.
- `20260525-simimpl18-rain-snow-partition-and-storage-state-mutation-closure-001/`
  - Purpose: execute SIMIMPL18 by closing first-day rain/snow partition
    divergence and restoring dynamic winter/soil storage-state mutation under
    identical baseline/candidate inputs, including static-parameter publication
    leak closure, baseline-year span policy closure, and 1095-row precipitation
    parity evidence, using contract-first sequencing and replay rerun evidence.
- `20260525-simimpl19-rm-snow-water-baseline-authoritative-closure-001/`
  - Purpose: execute SIMIMPL19 by implementing baseline-authoritative `RM` and
    `Snow-Water` closure under identical baseline/candidate inputs, including
    rain/snow partition parity, runtime-SWE publication (no static-parameter
    leak), contract-derived test closure, and Tier-A rerun disposition
    evidence.
- `20260525-simimpl20-wb11-soil-water-et-baseline-authority-assessment-and-planning-001/`
  - Purpose: execute SIMIMPL20 by producing a baseline-authoritative
    assessment and follow-on implementation queue for `wb11_soil_water` and
    full-fidelity `Ep`/`Es`/`Er` migration, including dependency/consumer
    landmine analysis (plant/runtime coupling), required canonical `SC-*`
    amendment map, and contract-first execution sequencing for downstream
    physics implementation packages.
- `20260525-simimpl21-wb11-et-soil-water-contract-authority-closure-001/`
  - Purpose: execute SIMIMPL21 by implementing canonical `SC-*` contract
    authority closure for WB11 ET and soil-water migration scope (stage-memory
    state, root-uptake semantics, ordering authority, alias lineage), with
    baseline-provenance citations and downstream SIMIMPL22 handoff readiness.
- `20260525-simimpl22-wb11-et-soil-water-contract-derived-tests-and-gate-001/`
  - Purpose: execute SIMIMPL22 by implementing contract-derived tests and
    pre-implementation gate evidence for SIMIMPL21 WB11 ET/soil-water
    authority closures (stage-memory, root-uptake/stress lineage, ordering,
    WB13 publication aliases) before SIMIMPL23 runtime migration.
- `20260525-simimpl23-wb11-et-full-fidelity-kernel-migration-001/`
  - Purpose: execute SIMIMPL23 by implementing baseline-authoritative WB11 ET
    runtime migration (`evap` + `swu` semantics) with contract-derived closure
    evidence for stage-memory and uptake-lineage behavior under typed guards,
    preparing downstream WB13/publication lineage closeout scope.
- `20260525-simimpl24-wb11-soil-water-lineage-and-publication-closure-001/`
  - Purpose: execute SIMIMPL24 by closing `wb11_soil_water` aggregate lineage
    and WB13 publication semantics (`Total-Soil`, `SoilWaterTotal`, ET
    components) using simulation-owned runtime surfaces only.
- `20260525-simimpl25-wb11-tier-a-rerun-and-hold-lift-disposition-001/`
  - Purpose: execute SIMIMPL25 by running Tier-A replay/comparator reruns after
    SIMIMPL24 closure, recording semantic closure evidence, and producing an
    explicit hold-lift disposition recommendation.
- `20260525-simimpl26-soil-dat-comparator-baseline-candidate-assessment-001/`
  - Purpose: prepare and execute SIMIMPL26 by comparing baseline vs candidate
    `soil.dat`/`.sol` artifacts with reproducible provenance, explicit delta
    classification, and evidence-backed follow-on disposition guidance.
- `20260525-simimpl27-snowfreeze-contract-boundary-closure-for-hourly-energy-balance-001/`
  - Purpose: execute SIMIMPL27 by closing canonical `SC-SNOWFREEZE-001`
    boundary/API authority gaps for hourly snow/freeze migration scope,
    ratifying concrete alias mappings and downstream contract-derived test
    requirements before SIMIMPL28 forcing/kernel implementation work.
- `20260525-simimpl28-hourly-winter-forcing-synthesis-port-001/`
  - Purpose: execute SIMIMPL28 by porting baseline-authoritative hourly
    winter forcing synthesis (`sunmap`, `radcur`, `hr_tmp`, `stmtim`) into
    openWEPP runtime seams with typed guards, reserved hourly alias emission,
    and contract-derived test closure for downstream snow/frost kernel
    migration waves.
- `20260525-simimpl29-snowd-melt-energy-balance-kernel-port-and-coupling-001/`
  - Purpose: execute SIMIMPL29 by porting baseline-authoritative snow-kernel
    hourly state/melt lineage (`snowd`, `melt`) into hydrology runtime
    coupling, publishing `snow.hourly.depth_*`, `snow.hourly.density_*`, and
    `snow.hourly.melt_m` families with typed active-winter guard posture and
    runtime snow carry-state closure.
- `20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/`
  - Purpose: execute SIMIMPL30 by running winter-hourly semantic parity reruns,
    classifying residuals by confidence tier and contract guards, and
    publishing explicit GO/HOLD hold-lift disposition for the SNOWPLAN01 wave.
- `20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/`
  - Purpose: assess feasibility for baseline-authoritative hourly
    energy-balance snow closure and publish a dependency-ordered four-package
    implementation queue with mandatory contract-first sequencing constraints.
- `20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/`
  - Purpose: review openWEPP vs baseline frost-process implementations and
    publish `frost-energy-solver-wp-queue.md`, a dependency-ordered
    contract-first implementation queue to close frost process-parity gaps and
    unblock SIMIMPL30 hold-lift prerequisites.
- `20260526-routeplan01-hillslope-sediment-routing-assessment-and-queue-001/`
  - Purpose: assess baseline `route.for` sediment-routing branch parity versus
    openWEPP runtime implementation, classify implementation gaps (including
    `rtpart.for` provenance mismatch), and publish a dependency-ordered queue
    to migrate full segment routing and eliminate sediment-routing magic
    numbers.
- `20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/`
  - Purpose: assess the work needed to implement watershed channel routing and
    watershed orchestration to a fully scaffolded runtime path that can produce
    non-placeholder watershed parquet interchange outputs, then publish a
    dependency-ordered contract-first implementation queue.
- `20260527-wshedimpl01-watershed-contract-authority-closure-and-gap-normalization-001/`
  - Purpose: execute WSHED02 by normalizing canonical watershed contract
    authority (`SC-ROUTE-001`, `SC-IMPOUND-001`, `SC-SED-001`, `SC-SYSTEM-001`)
    and publishing explicit residual-gap mapping before WSHED03
    contract-derived test and pre-implementation gate work.
- `20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/`
  - Purpose: execute WSHED03 by authoring contract-derived watershed
    routing/impoundment/system vectors (including expected-failure closure for
    partial runtime seams) and recording pre-implementation gate evidence
    before WSHED04+ production migration packages.
- `20260527-wshedimpl04-watershed-runtime-seam-closure-001/`
  - Purpose: execute WSHED04 by closing parser-to-runtime impoundment
    coefficient projection seams (`a,b,c,d,e,ha,ht,hlm,a0,a1,a2,l0,l1,l2`),
    removing synthetic/manual WS12 coefficient seeding from contract vectors,
    and preserving fail-closed typed seam guard posture before WSHED05/06/07.
- `20260527-wshedimpl05-watershed-wave-routing-state-family-migration-001/`
  - Purpose: execute WSHED05 by migrating WS11 `ipeak > 2` wave-routing
    lineage state-family publication (`q1/qin/qlat/c0..c4`), promoting the
    matching WSHED03 expected-failure vector to active conformance, and
    preserving typed fail-closed guard posture before WSHED06/07/08.
- `20260527-wshedimpl06-watershed-channel-sediment-routing-foundation-001/`
  - Purpose: execute WSHED06 publication-family closure by migrating WS11
    channel sediment output symbols (`qsed`, `tc`) into production channel
    writeback, promoting the matching WSHED03 expected-failure vector, and
    preserving typed fail-closed guard posture while documenting residual
    baseline `chnero/chnrt/detach` parity blockers for follow-on closure.
- `20260527-wshedimpl07-watershed-impoundment-continuity-migration-001/`
  - Purpose: execute WSHED07 by migrating WS12 impoundment continuity behavior
    (RK4 integration, adaptive timestep retry, and regime-transition retry
    controls) into watershed production execution, promoting the matching
    WSHED03 expected-failure timestep-stability vector to active conformance,
    and preserving fail-closed typed guard continuity while retaining explicit
    active-structure projection blockers for follow-on closure.
- `20260527-wshedimpl08-watershed-output-row-model-and-parquet-writer-activation-001/`
  - Purpose: execute WSHED08 by replacing watershed output placeholder refusal
    (`OWSOUT-E-004`) with real row-model parquet emission for all required
    watershed outputs, promoting the WSHED03 non-stub parquet vector to active
    conformance, and preserving typed fail-closed writer behavior.
- `20260527-wshedimpl09-end-to-end-validation-comparator-rerun-and-hold-lift-disposition-001/`
  - Purpose: execute WSHED09 by rerunning watershed validation lanes and
    confidence-tier comparator evidence, then publishing explicit GO/HOLD
    disposition with residual ownership for unresolved watershed closure
    blockers.
- `20260527-wshedimpl10-active-structure-impoundment-parser-payload-export-001/`
  - Purpose: execute WSHED10 by exporting active impoundment outlet-structure
    branch payloads from `.imp` parser outputs (drop/culvert/rockfill/
    emergency/filter/riser), adding contract-derived parser coverage, and
    narrowing residual HOLD posture to runtime active-coefficient projection
    closure.
- `20260527-wshedimpl11-runtime-active-structure-coefficient-projection-001/`
  - Purpose: execute WSHED11 by implementing runtime projection of exported
    active impoundment branch payloads into WS12 coefficient families
    (`a,b,c,d,e,ha,ht,hlm`), promoting active-lane conformance vectors, and
    updating canonical gap posture for `GAP-IMPOUND-006` /
    `GAP-SYSTEM-007`.
- `20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001/`
  - Purpose: execute WSHEDIMPL12 by closing WSHEDIMPL11 worker-handoff
    immediate actions operationally: author execution-ready follow-on package
    specs for residual blockers (`GAP-SYSTEM-007`, `GAP-SYSTEM-005`,
    `GAP-SYSTEM-008`) and publish dependency-ordered downstream execution
    ownership.
- `20260527-wshedimpl13-active-lane-15-function-parity-migration-001/`
  - Purpose: execute WSHEDIMPL13 by migrating WS12 active-lane runtime/kernel
    behavior from reduced-family projection to full legacy-authoritative
    15-function structure-family projection + min-controller composition,
    closing `GAP-IMPOUND-006` and `GAP-SYSTEM-007` while preserving program
    HOLD on residual watershed blockers (`GAP-SYSTEM-005`, `GAP-SYSTEM-008`).
- `20260527-wshedimpl14-watershed-baseline-authoritative-end-to-end-comparator-lane-001/`
  - Purpose: execute WSHEDIMPL14 by implementing a baseline-authoritative
    end-to-end `openwepp-cli-watershed` comparator lane that validates topology
    dispatch, branch execution, and publication-boundary signature continuity,
    closing `GAP-SYSTEM-005` while preserving program HOLD on residual
    watershed sediment parity blocker `GAP-SYSTEM-008`.
- `20260527-wshedimpl15-watershed-channel-sediment-process-parity-migration-001/`
  - Purpose: execute WSHEDIMPL15 by implementing WS15 channel-sediment runtime
    projection and fail-closed kernel scaffold closure (`ishape/ienslp/chnz/
    chnnbr/chntcr/chnedm/chneds/ctlz/ctln` plus baseline conversion state
    scaffolds `crsh/depmid/depsid`) as required precursor for full
    `chnero/chnrt/detach` parity migration while preserving explicit non-
    promotable HOLD posture on remaining full process-parity blockers.
- `20260527-wshedimpl16-channel-sediment-payload-seam-closure-and-vector-promotion-001/`
  - Purpose: execute WSHEDIMPL16 immediate next actions from WSHEDIMPL15 by
    projecting fail-closed contributor `particle_diameter_m` payload symbols
    into WS10 runtime ingress, promoting WS11 channel sediment vectors from
    publication checks to equation checks, and rerunning watershed
    baseline-authoritative comparator-lane evidence while preserving explicit
    non-promotable HOLD posture on remaining full `chnero/chnrt/detach`
    process-parity migration blockers.
- `20260527-wshedimpl17-channel-segment-geometry-hydraulic-seam-intake-001/`
  - Purpose: execute WSHEDIMPL17 immediate next actions from WSHEDIMPL16 by
    projecting fail-closed WS10 channel segment/hydraulic scaffold families
    (`nslpts`, per-segment `x/slope/depa/depb/wida/widb`) from slope+channel
    parser payloads, wiring watershed CLI slope-driven seam seeding, and
    preserving explicit non-promotable HOLD posture on remaining full
    `chnero/chnrt/detach` process-parity migration blockers.
- `20260527-wshedimpl18-channel-sediment-transport-capacity-authority-migration-001/`
  - Purpose: execute WSHEDIMPL18 immediate next actions from WSHEDIMPL17 by
    migrating WS10 channel sediment publication from surrogate `tc=qsed` to
    baseline-lineage transport-capacity authority (`shield`/`trncap` +
    hydraulic coupling), promoting WS11 vectors for `tc` process behavior, and
    preserving explicit non-promotable HOLD posture on remaining
    `chnero/chnrt/detach` segment-loop parity blockers.
- `20260527-wshedimpl19-channel-sediment-branch-payload-export-and-upstream-ingress-001/`
  - Purpose: execute WSHEDIMPL19 immediate next actions from WSHEDIMPL18 by
    adding fail-closed WS10 channel sediment branch payload export
    (`particle_class_count`, `particle_flow_fraction_{class:04}`,
    `particle_diameter_m_{class:04}`), ingesting upstream channel-dependency
    payloads into class-aware aggregation, promoting WS11 vectors for payload
    export/ingress continuity, and preserving explicit non-promotable HOLD
    posture on remaining `chnero/chnrt/detach` segment-loop parity blockers.
- `20260527-wshedimpl20-channel-segment-case12-routing-and-diagnostics-001/`
  - Purpose: execute WSHEDIMPL20 immediate next actions from WSHEDIMPL19 by
    adding opt-in WS20 channel segment-loop `case12` routing scaffolding,
    publishing explicit unresolved-detachment diagnostics
    (`ws20_case1_segment_count`, `ws20_case2_segment_count`,
    `ws20_detachment_unmigrated_segment_count`), promoting WS11 vectors for
    default-off/opt-in continuity, and preserving explicit non-promotable HOLD
    posture on remaining baseline-authoritative `chnero/chnrt/detach`
    segment-loop parity blockers.
- `20260527-wshedimpl21-channel-case34-enddet-routing-and-detach-gate-001/`
  - Purpose: execute WSHEDIMPL21 immediate next actions from WSHEDIMPL20 by
    adding WS21 opt-in case34 branch diagnostics scaffolding
    (`ws10_channel_{id}_ws21_case34_enable`) and explicit WS21 diagnostics
    publication (`ws21_case3_segment_count`, `ws21_case4_segment_count`,
    `ws21_enddet_segment_count`, `ws21_detach_unmigrated_segment_count`) while
    preserving non-promotable HOLD posture for remaining baseline-authoritative
    `detach/dcap` migration and full `chnero/chnrt` parity closure.
- `20260527-wshedimpl22-channel-detach-dcap-case34-enddet-migration-001/`
  - Purpose: execute WSHEDIMPL22 immediate next actions from WSHEDIMPL21 by
    replacing WS21 opt-in unresolved-detachment fallback scaffolding with
    baseline-lineage `dcap` + `case34/enddet` execution, adding required
    fail-closed `crfrac` projection gating
    (`ws10_channel_{id}_crfrac_{class:04}`), and preserving explicit
    non-promotable HOLD posture for residual baseline-authoritative WS21
    `case4 -> detach` iterative closure (`nt < cnpart`).
- `20260527-wshedimpl23-channel-detach-case4-iterative-closure-001/`
  - Purpose: execute WSHEDIMPL23 immediate next actions from WSHEDIMPL22 by
    migrating baseline-authoritative `detach.for` iterative closure behavior
    for WS21 `case4` rows (`nt < cnpart`) in WS20+WS21 opt-in routing,
    removing residual WS21 unresolved-detachment fallback diagnostics for that
    branch, and preserving explicit non-promotable HOLD posture for remaining
    full `chnero/chnrt/detach` parity closure families.
- `20260527-wshedimpl24-case12-deposition-detach-transition-migration-001/`
  - Purpose: execute WSHEDIMPL24 immediate next actions from WSHEDIMPL23 by
    migrating baseline-authoritative `case12.for` deposition-to-detachment
    transition continuation (`xdemax < x(i)` into `detach.for`) in WS20
    segment-loop routing, publishing explicit transition diagnostics
    (`ws24_case2_detach_segment_count`), and preserving explicit
    non-promotable HOLD posture for remaining full `chnero/chnrt/detach`
    parity closure families.
- `20260527-wshedimpl25-ws20-opt-in-ws21-auto-activation-closure-001/`
  - Purpose: execute WSHEDIMPL25 immediate next actions from WSHEDIMPL24 by
    closing residual WS20 opt-in unresolved-detachment fallback behavior through
    WS21 auto-activation under WS20 routing, adding contract-derived vectors for
    missing-`crfrac` fail-closed behavior in WS20-only opt-in lanes, and
    preserving explicit non-promotable HOLD posture for remaining full
    `chnero/chnrt/detach` parity closure families.
- `20260527-wshedimpl26-detach-dcap-flagm2-iterative-closure-001/`
  - Purpose: execute WSHEDIMPL26 immediate next actions from WSHEDIMPL25 by
    migrating baseline-authoritative `dcap(flagm=2)` max-detachment limiter
    semantics for WS23 iterative detach closure lanes, adding WS26
    contract-derived vectors, and preserving explicit non-promotable HOLD
    posture for remaining full `chnero/chnrt/detach` parity closure families.
- `20260527-wshedimpl27-enddet-bracket-closure-001/`
  - Purpose: execute WSHEDIMPL27 immediate next actions from WSHEDIMPL26 by
    migrating baseline-authoritative `enddet.for` bracket progression
    semantics (`xdbig/xdsmal`) for WS21 case4 enddet closure lanes, adding
    WS27 contract-derived vectors, and preserving explicit non-promotable HOLD
    posture for remaining full `chnero/chnrt/detach` parity closure families.
- `20260527-wshedimpl28-channel-width-boundary-semantics-001/`
  - Purpose: execute WSHEDIMPL28 immediate next actions from WSHEDIMPL27 by
    migrating baseline-authoritative `chnrt.for` width-boundary routing
    semantics (`widb(i-1)` upper boundary, `wida(i)` lower boundary) in
    WS20/WS21 segment loops, adding WS28 contract-derived vectors, and
    preserving explicit non-promotable HOLD posture for remaining full
    `chnero/chnrt/detach` parity closure families.
- `20260527-wshedimpl29-channel-rectangular-width-mutation-closure-001/`
  - Purpose: execute WSHEDIMPL29 immediate next actions from WSHEDIMPL28 by
    migrating baseline-authoritative rectangular-channel width mutation
    semantics (`dcap` `werb` -> `widb(i-1)`) into WS20 routing/writeback
    surfaces, adding WS29 contract-derived vectors, and preserving explicit
    non-promotable HOLD posture for remaining full `chnero/chnrt/detach`
    parity closure families.
- `20260528-wshedimpl30-erodible-shape-transition-fallback-mapping-001/`
  - Purpose: execute WSHEDIMPL30 immediate next actions from WSHEDIMPL29 by
    migrating baseline-authoritative erodible-lane shape-transition continuity
    (`ishape=3` pathways with `depa/depb`-driven rectangular fallback mapping)
    into WS20/WS21 routing lanes, adding WS30 contract-derived vectors, and
    preserving explicit non-promotable HOLD posture for remaining full
    `chnero/chnrt/detach` parity closure families.
- `20260528-wshedimpl31-detach-lower-boundary-width-mutation-closure-001/`
  - Purpose: execute WSHEDIMPL31 immediate next actions from WSHEDIMPL30 by
    migrating baseline-authoritative lower-boundary width-mutation continuity
    (`flagc=2`, `wera>wfl`, `wida(i)=wera(i)`) through WS23/WS24 detach
    closures and WS10 state writeback, adding WS31 contract-derived vectors,
    and preserving explicit non-promotable HOLD posture for remaining full
    `chnero/chnrt/detach` parity closure families.
- `20260528-wshedimpl32-parser-runtime-ishape-lineage-closure-001/`
  - Purpose: execute WSHEDIMPL32 immediate next actions from WSHEDIMPL31 by
    reconciling parser/runtime naturally eroded channel shape-class lineage
    (`ishape=3`) across watershed channel input parsing and WS10 runtime
    symbol projection/consumption, adding contract-derived parser/runtime
    vectors, and preserving explicit non-promotable HOLD posture for remaining
    full `chnero/chnrt/detach` parity closure families.
- `20260528-wshedimpl33-parser-runtime-ienslp-lineage-closure-001/`
  - Purpose: execute WSHEDIMPL33 immediate next actions from WSHEDIMPL32 by
    reconciling parser/runtime channel `ienslp` lineage across watershed
    channel input parsing and WS10 runtime seed validation (`ienslp` domain
    `1..=2` fail-closed), adding contract-derived parser/runtime vectors, and
    preserving explicit non-promotable HOLD posture for remaining full
    `chnero/chnrt/detach` parity closure families.
- `20260528-wshedimpl34-parser-runtime-chnn-chnnbr-lineage-closure-001/`
  - Purpose: execute WSHEDIMPL34 immediate next actions from WSHEDIMPL33 by
    reconciling parser/runtime watershed-channel Manning relation lineage
    across watershed channel input parsing and WS10 runtime seed validation
    (`chnn >= chnnbr` fail-closed), adding contract-derived parser/runtime
    vectors, and preserving explicit non-promotable HOLD posture for remaining
    full `chnero/chnrt/detach` parity closure families.
- `20260528-wshedimpl35-parser-runtime-icntrl-flgout-lineage-closure-001/`
  - Purpose: execute WSHEDIMPL35 immediate next actions from WSHEDIMPL34 by
    reconciling parser/runtime watershed-channel control lineage for
    `icntrl`/`flgout` across watershed channel input parsing and WS10 runtime
    seed projection/validation (`icntrl in [0,4]`, `flgout in [0,1]`
    fail-closed), adding contract-derived parser/runtime vectors, and
    preserving explicit non-promotable HOLD posture for remaining full
    `chnero/chnrt/detach` parity closure families.
- `20260528-wshedimpl36-parser-runtime-rating-curve-lineage-closure-001/`
  - Purpose: execute WSHEDIMPL36 immediate next actions from WSHEDIMPL35 by
    reconciling parser/runtime rating-curve control lineage (`rccoef`,
    `rcexp`, `rcoset`) for `icntrl==4` channel lanes across watershed channel
    parsing and WS10 runtime seed projection/validation with explicit
    fail-closed payload-presence and domain semantics.
- `20260528-wshedimpl37-ws11-route-chain-wshcqi-wshirs-wshrun-closure-001/`
  - Purpose: execute WSHEDIMPL37 by migrating baseline-authoritative WS11
    hydrology routine-chain behavior (`wshcqi/wshirs/wshrun`) into production
    watershed runtime lanes and closing/narrowing residual `GAP-ROUTE-008`
    parity blockers with contract-derived route-chain vectors.
- `20260528-wshedimpl38-channel-sediment-symbol-burndown-hold-lift-closure-001/`
  - Purpose: execute WSHEDIMPL38 as final symbol/process burndown wave to
    close residual channel sediment parity blockers
    (`GAP-ROUTE-009`/`GAP-SED-006`/`GAP-SYSTEM-008`), run hold-lift evidence
    gates, and publish disposition-grade GO/HOLD outcome with explicit gap
    closure ownership.
- `20260528-wshedimpl39-out-of-scope-gap-closure-001/`
  - Purpose: execute WSHEDIMPL39 to close WSHEDIMPL38 out-of-scope hold
    blockers by binding Chapter-13 applicability limits to concrete watershed
    runfile validator selectors (`GAP-ROUTE-005`) and dispositioning residual
    system-level governance/alias posture (`GAP-SYSTEM-001`,
    `GAP-SYSTEM-002`) with contract-first evidence.
- `20260528-wshedimpl40-muskingum-cunge-baseline-parity-closure-001/`
  - Purpose: execute WSHEDIMPL40 to identify baseline-authoritative
    Muskingum-Cunge implementation gaps (`wshchr` lineage, `ipeak >= 4`) in
    openWEPP watershed routing, implement contract-first parity closure, and
    publish dual-review/dual-verification disposition evidence.
- `20260528-wshedimpl41-ipeak5-mvpmc3-dynamic-coeff-refresh-parity-001/`
  - Purpose: execute WSHEDIMPL41 immediate follow-on closure for WSHEDIMPL40
    by migrating baseline-authoritative `ipeak = 5` MVPMC3 dynamic-coefficient
    refresh routing behavior into WS11 production runtime lanes, adding
    contract-derived parity vectors, and dispositioning
    `GAP-ROUTE-011` / `GAP-SYSTEM-010` with contract-first evidence.
- `20260529-wshedimpl42-wb14-runoff-guard-unpalatable-rind-closure-001/`
  - Purpose: execute WSHEDIMPL42 to close the active
    `HKERNEL-WB14-RUNOFF-E-003` hillslope blocker on
    `/wc1/runs/un/unpalatable-rind`, rerun full watershed orchestration, and
    verify closure by successful watershed parquet emission under contract-first
    sequencing.
- `20260529-wshedimpl43-hbp-binary-only-ascii-pass-removal-001/`
  - Purpose: execute WSHEDIMPL43 as immediate follow-on to WSHEDIMPL42 by
    removing ASCII pass support completely, replacing hillslope pass publication
    with binary HBP-only output, enforcing strict `.hbp` ingestion policy
    (no `.pass.dat` derivation/fallback), and rerunning
    `/wc1/runs/un/unpalatable-rind` to watershed parquet closure.
- `20260529-hillstab07-wb16-peak-flow-input-provenance-parity-001/`
  - Purpose: execute HILLSTAB07 to address WB16 peak-flow input provenance
    parity gaps identified by `docs/audits/20260529_peak_flow_implementation_audit.md`
    using contract-first sequencing: codify canonical `m`/`ealpha` authority,
    eliminate silent parity drift by publishing explicit compatibility-seed
    provenance, add contract-derived tests, and publish GO/HOLD disposition
    with follow-on closure requirements for full baseline-authoritative
    `ealpha` producer migration.
- `20260529-hillstab08-wb16-ealpha-producer-chain-runtime-migration-001/`
  - Purpose: execute HILLSTAB08 immediate next actions from HILLSTAB07 by
    migrating WB16 runtime `ealpha` producer-chain surfaces
    (`frcfac -> rdat(alpha) -> alphay -> eplane`) into production state
    projection and kernel ingress lanes, adding contract-derived lineage/parity
    vectors for single-OFE and multi-OFE fixtures, and publishing GO/HOLD
    disposition for residual baseline-authoritative closure gaps.
- `20260528-clim17-breakpoint-climate-baseline-parity-burndown-001/`
  - Purpose: execute CLIM17 to identify and close implementation gaps in
    openWEPP breakpoint climate behavior versus
    `wepp-forest_260430_baseline`, anchored to
    `/wc1/runs/un/unpalatable-rind`, with contract-first sequencing and
    dual-review/dual-verification gate evidence through disposition.
- `20260526-erod16-route-branch-contract-authority-and-routine-map-001/`
  - Purpose: execute EROD16 by amending canonical `SC-SED-001` and
    `SC-ROUTE-001` with baseline-authoritative `route.for` segment-branch
    routine mapping, explicit `mshear`/deposition invariants and alias
    continuity requirements, and corrected `rtpart.for` provenance
    classification before EROD17 contract-derived tests.
- `20260526-erod17-route-branch-contract-derived-tests-and-preimplementation-gate-001/`
  - Purpose: execute EROD17 by implementing route-branch contract-derived test
    vectors (`mshear`, deposition-end branching, `ndep` follow-up, and
    `qostar` threshold behavior), recording expected pre-migration failures,
    and publishing pre-implementation gate evidence before EROD18/EROD19
    runtime migration work.
- `20260526-erod18-route-runtime-segment-state-topology-and-ingress-closure-001/`
  - Purpose: execute EROD18 by implementing typed route segment-state topology
    and runner-to-kernel ingress closure (`nslpts`, `xu/xl`,
    `ainf/binf/cinf`, `ainftc/binftc/cinftc`, route seam publication family)
    with typed guard enforcement before EROD19 full `route.for` branch-family
    migration.
- `20260526-erod19-route-mshear-segment-kernel-migration-001/`
  - Purpose: execute EROD19 by migrating baseline-derived `route.for`
    segment-branch behavior (`xcrit` `mshear` classification, upper-boundary
    `dl` branch, `depc/depend` deposition-end publication, and route branch
    follow-up state surfaces) and activating EROD17 route vectors before
    EROD20 literal/symbol cleanup.
- `20260526-erod20-sediment-routing-constant-symbolization-and-magic-number-elimination-001/`
  - Purpose: execute EROD20 by replacing remaining sediment-routing literals
    in EROD14/EROD19 production paths with named constants (case bounds,
    vector sizing, attenuation/enrichment thresholds, and route solver
    thresholds), preserving behavior and closing queue item 5 before EROD21.
- `20260526-erod21-route-parity-rerun-and-hold-lift-disposition-001/`
  - Purpose: execute EROD21 by rerunning route-focused branch-family parity
    lanes (EROD17/EROD18 vectors plus MOFE03 runner continuity), executing
    validation gates, and publishing explicit GO/HOLD disposition to close the
    sediment-routing queue.
- `20260526-simimpl31-frost-energy-contract-authority-and-routine-map-001/`
  - Purpose: execute SIMIMPL31 by closing canonical frost routine-chain
    authority gaps in `SC-SNOWFREEZE-001`, ratifying explicit baseline
    routine-to-alias/invariant mapping, and defining downstream
    contract-derived test requirements for SIMIMPL32.
- `20260526-simimpl32-frost-hourly-contract-derived-tests-and-preimplementation-gate-001/`
  - Purpose: execute SIMIMPL32 by implementing frost-hourly contract-derived
    tests and pre-implementation gate evidence for SIMIMPL31 authority
    closure, recording expected reductive-path failures before
    SIMIMPL33/SIMIMPL34 runtime migration edits.
- `20260526-simimpl33-frost-energy-runtime-state-topology-and-seam-closure-001/`
  - Purpose: execute SIMIMPL33 by implementing runtime frost state topology and
    typed seam wiring (fine-layer indexing/count lineage, conductivity
    lineage, and `frost.hourly.*` seam families) required before SIMIMPL34
    baseline-authoritative frost solver migration.
- `20260526-simimpl34-frost-energy-solver-kernel-migration-and-coupling-001/`
  - Purpose: execute SIMIMPL34 by replacing reductive active-frost coupling
    with baseline-authoritative frost solver migration (`frostN` family with
    `frwatc`/`frzng`/`frznw`/`frsoil` + `getFreezeCond`) and coupling closure
    in runoff/infiltration/water-balance runtime outputs.
- `20260526-simimpl35-winter-hourly-frost-parity-rerun-and-hold-lift-disposition-001/`
  - Purpose: execute SIMIMPL35 by rerunning winter-hourly frost parity lanes
    after SIMIMPL34 migration and publishing explicit GO/HOLD hold-lift
    disposition with residual ownership.
- `20260526-simimpl36-frost-hold-blocker-closure-and-rerun-001/`
  - Purpose: execute SIMIMPL36 by closing SIMIMPL35 blockers
    (`KWRITEBACK-E-DOMAIN-VIOLATION`, `SOL-E-006`, and multi-hillslope
    parquet duplicate-key admissibility), rerunning fresh post-fix lanes, and
    publishing updated GO/HOLD hold-lift disposition evidence.
- `20260525-refactor001-openwepp-runner-lib-mechanical-modularization-001/`
  - Purpose: execute REFACTOR001 by mechanically modularizing
    `openwepp-runner/src/lib.rs`, preserving public API/runtime behavior, and
    updating layout-coupled tests to remain contract-accurate under the new
    module tree.
- `20260523-wb14-infiltration-and-subdaily-hyetograph-kernel-001/`
  - Purpose: implement production infiltration kernel authority and within-day
    hyetograph integration with contract-first sequencing (contract amendments,
    contract tests, pre-implementation gate, then kernel code).
- `20260523-pl16-growth-physics-kernelization-001/`
  - Purpose: replace PL13 growth plumbing-only behavior with equation-driven
    production growth physics and contract-first sequencing (contract
    amendments, contract tests, pre-implementation gate, then kernel code).
- `20260523-pl17-decomposition-physics-kernelization-001/`
  - Purpose: replace PL12 decomposition plumbing-only behavior with
    equation-driven decomposition/residue kinetics and contract-first
    sequencing (contract amendments, contract tests, pre-implementation gate,
    then kernel code).
- `20260523-clim05-snow-runtime-kernel-port-001/`
  - Purpose: implement runtime snow accumulation/melt kernel coupling into
    hydrology boundary surfaces with contract-first sequencing (contract
    amendments, contract tests, pre-implementation gate, then kernel code).
- `20260523-clim06-frost-frozen-soil-kernel-port-001/`
  - Purpose: implement frozen-soil/frost runtime kernel behavior and
    infiltration/runoff coupling with contract-first sequencing (contract
    amendments, contract tests, pre-implementation gate, then kernel code).
- `20260523-wb15-canopy-interception-kernel-coupling-001/`
  - Purpose: implement canopy interception coupling from plant runtime state
    into runoff/infiltration/water-balance closure with contract-first
    sequencing (contract amendments, contract tests, pre-implementation gate,
    then kernel code).
- `20260523-irrig10-irrigation-runtime-kernel-port-001/`
  - Purpose: implement irrigation runtime kernels consuming depletion/fixed-date
    parser surfaces with typed scheduling and hydrology coupling, using
    contract-first sequencing (contract amendments, contract tests,
    pre-implementation gate, then kernel code).
- `20260523-wb16-peak-runoff-kernel-001/`
  - Purpose: implement peak runoff kernel outputs for downstream
    sediment/routing coupling with contract-first sequencing (contract
    amendments, contract tests, pre-implementation gate, then kernel code).
- `20260523-wb17-et-physics-equivalence-port-001/`
  - Purpose: replace WB11 ET surrogate behavior with equation-driven ET
    physics parity authority, explicitly authored in canonical science
    contracts (`SC-EVAP-001` + companion `SC-*`) before kernel code updates,
    under contract-first sequencing (contract amendments, contract tests,
    pre-implementation gate, then kernel code).
- `20260523-wb18-percolation-physics-equivalence-port-001/`
  - Purpose: replace WB11 percolation surrogate behavior with layer-aware
    equation-driven percolation physics parity authority, explicitly authored
    in canonical science contracts (`SC-PERC-001` + companion `SC-*`) before
    kernel code updates, under contract-first sequencing (contract amendments,
    contract tests, pre-implementation gate, then kernel code).
- `20260523-wb19-lateral-drainage-physics-equivalence-port-001/`
  - Purpose: replace WB11 lateral/drainage surrogate behavior with
    equation-driven lateral/subsurface drainage physics parity authority,
    explicitly authored in canonical science contracts (`SC-SUBHYD-001` +
    companion `SC-*`) before kernel code updates, under contract-first
    sequencing (contract amendments, contract tests, pre-implementation gate,
    then kernel code).
- `20260523-wb20-forward-water-balance-solver-lane-001/`
  - Purpose: establish a forward-solved water-balance parity lane that
    excludes observed closure targets (`wb12_runoff_observed`,
    `wb12_storage_observed`) from acceptance-driving inputs, with
    contract-first sequencing (contract amendments, contract tests,
    pre-implementation gate, then lane/runtime code).
- `20260523-cli01-open-wepp-runner-and-hillslope-driver-bootstrap-001/`
  - Purpose: implement in-repo `open_wepp_runner` and
    `openwepp-cli-hill` execution path for comparator-ready openWEPP candidate
    outputs, including blind run-directory sidecar discovery, run provenance
    manifests, and release metadata sidecar validation under contract-first
    sequencing (contract amendments/spec updates, contract tests,
    pre-implementation gate, then runner/CLI code).
- `20260524-cli02-hillslope-simulation-and-interchange-emission-001/`
  - Purpose: planning/governance closeout for hillslope `.run` contract
    simplification and runner-boundary realignment, including explicit
    handoff to CLI03 implementation scope.
- `20260524-cli03-hillslope-runner-interchange-implementation-001/`
  - Purpose: implement runner/CLI behavior for schema-versioned hillslope
    `.run` execution, metric-only enforcement, legacy sidecar discovery
    precedence, required pass/loss outputs, optional parquet outputs, and
    manifest/checksum evidence; organize output contracts/serializers/tests in
    dedicated crate `crates/openwepp-hillslope-output/` under contract-first
    sequencing (contract sufficiency check, contract tests,
    pre-implementation gate, then runner/CLI code).
- `20260524-cli04-openwepp-output-hillslope-wat-parquet-architecture-001/`
  - Purpose: define and implement shared output crate architecture for
    hillslope/watershed parquet families and land real `outputs.wat` parquet
    emission (with WEPPpy/WEPPpyo3 schema metadata parity, including
    post-`wepp_260430` `InterceptionStorage` authority) under contract-first
    sequencing (contract/spec amendments, contract tests,
    pre-implementation gate, then runner/output code).
- `20260523-erod10-sediment-kernelization-intake-001/`
  - Purpose: convert erosion-kernel deferral into an executable intake/phase
    plan with explicit package-wave ownership, dependency graph, and
    contract-authority mapping for follow-on sediment kernelization.
- `20260523-erod11-alias-and-boundary-ownership-closure-001/`
  - Purpose: close Wave-0 erosion-lane alias and boundary ownership ambiguity
    by ratifying canonical-to-runtime symbol mappings and cross-contract
    producer/consumer ownership before EROD12/EROD13 code-authoring packages,
    and keep scaffolded/placeholder physics postures in `HOLD`.
- `20260523-erod12-cross-domain-contract-closure-001/`
  - Purpose: close Wave-0 cross-domain erosion companion-contract ownership
    and guard semantics (or explicitly retain blocker `HOLD`s with authority)
    before EROD13 production kernel authoring, under contract-first
    sequencing (contract amendments, contract tests, pre-implementation gate,
    then any code edits if explicitly authorized).
- `20260525-erod13-hillslope-core-erosion-kernel-001/`
  - Purpose: implement Wave-1 core hillslope erosion kernel behavior after
    EROD12 entry authorization, including canonical `SC-*` authority
    amendments, contract-derived tests, pre-implementation gate evidence, and
    typed production runtime integration under contract-first sequencing.
- `20260525-erod14-multiofe-and-enrichment-kernel-001/`
  - Purpose: implement Wave-2 multi-OFE routing and enrichment kernel behavior
    after EROD13 completion, including canonical `SC-*` authority amendments,
    contract-derived conservation-vector tests, pre-implementation gate
    evidence, and typed production runtime integration under contract-first
    sequencing.
- `20260525-erod15-routing-boundary-coupling-001/`
  - Purpose: implement Wave-3 routing-boundary sediment coupling after EROD14
    completion, including canonical `SC-*` authority amendments,
    contract-derived handoff-completeness tests, pre-implementation gate
    evidence, and typed production route-boundary integration under
    contract-first sequencing.
- `20260523-ws10-channel-impoundment-production-kernels-001/`
  - Purpose: replace watershed test/probe posture with production
    channel/impoundment kernels under typed boundary integration using
    contract-first sequencing (contract amendments, contract tests,
    pre-implementation gate, then kernel code).
- `20260524-ws11-channel-routing-physics-equivalence-port-001/`
  - Purpose: replace WS10 channel-routing surrogate authority with
    legacy-equivalent routing physics under contract-first sequencing
    (contract amendments, contract tests, pre-implementation gate, then kernel
    code).
- `20260524-ws12-impoundment-physics-equivalence-port-001/`
  - Purpose: replace WS10 impoundment surrogate authority with
    legacy-equivalent impoundment hydraulics under contract-first sequencing
    (contract amendments, contract tests, pre-implementation gate, then kernel
    code).
- `20260523-arch22-typed-state-surface-closure-001/`
  - Purpose: close CRF-001 carry-forward by migrating stringly
    `BoundarySymbol(String)` production kernel surfaces to typed state
    interfaces, with contract-first sequencing (contract updates, contract
    tests, pre-implementation gate, then migration code).
- `20260523-clim07-climate-comparator-and-closure-evidence-001/`
  - Purpose: add targeted continuous-daily and breakpoint climate comparator
    vectors, parser-to-kernel seam checks, and confidence-tier closure
    evidence with contract-first sequencing (contract updates, contract tests,
    pre-implementation gate, then comparator/integration code).
- `20260523-clim08-climate-governance-disposition-closeout-001/`
  - Purpose: close remaining climate governance HOLD items (`parser/runtime`
    seam and seam integration-test closure) and update canonical climate
    contracts/specifications to promotable status using CLIM02..CLIM07
    closure evidence.
- `20260520-inspec01-author-wepp-input-spec-slope-001/`
  - Purpose: author and disposition canonical slope input specification
    (`slope-file.spec.md`, `.slp`).
- `20260520-inspec02-author-wepp-input-spec-watershed-structure-001/`
  - Purpose: author and disposition canonical watershed structure specification
    (`watershed-structure-file.spec.md`, `.str`).
- `20260520-inspec03-author-wepp-input-spec-watershed-channel-001/`
  - Purpose: author and disposition canonical watershed channel specification
    (`watershed-channel-file.spec.md`, `.chn`).
- `20260520-inspec04-author-wepp-input-spec-watershed-impoundment-001/`
  - Purpose: author and disposition canonical watershed impoundment
    specification (`watershed-impoundment-file.spec.md`, `.imp`).
- `20260520-inspec05-author-wepp-input-spec-irrigation-depletion-001/`
  - Purpose: author and disposition canonical depletion irrigation sidecar
    specification (`irrigation-depletion-file.spec.md`).
- `20260520-inspec06-author-wepp-input-spec-irrigation-fixeddate-001/`
  - Purpose: author and disposition canonical fixed-date irrigation sidecar
    specification (`irrigation-fixeddate-file.spec.md`).
- `20260520-inspec07-author-wepp-input-spec-pmetpara-001/`
  - Purpose: author and disposition canonical `pmetpara.txt` specification
    (`pmetpara.spec.md`).
- `20260520-inspec08-author-wepp-input-spec-snow-001/`
  - Purpose: author and disposition canonical `snow.txt` specification
    (`snow.spec.md`).
- `20260520-inspec09-author-wepp-input-spec-frost-001/`
  - Purpose: author and disposition canonical `frost.txt` specification
    (`frost.spec.md`).
- `20260520-inspec10-author-wepp-input-spec-gwcoeff-001/`
  - Purpose: author and disposition canonical `gwcoeff.txt` specification
    (`gwcoeff.spec.md`).
- `20260520-inspec11-author-wepp-input-spec-phosphorus-001/`
  - Purpose: author and disposition canonical `phosphorus.txt` specification
    (`phosphorus.spec.md`).
- `20260520-inspec12-author-wepp-input-spec-weppui-001/`
  - Purpose: author and disposition canonical `wepp_ui.txt` specification
    (`wepp-ui.spec.md`).
- `20260520-inspec13-author-wepp-input-spec-tc-001/`
  - Purpose: author and disposition canonical `tc.txt` specification
    (`tc.spec.md`).
- `20260520-inspec14-author-wepp-input-spec-tcr-001/`
  - Purpose: author and disposition canonical `tcr.txt` specification
    (`tcr.spec.md`).
- `20260520-inspec15-author-wepp-input-spec-lcwb-001/`
  - Purpose: author and disposition canonical `lcwb.txt` specification
    (`lcwb.spec.md`).
- `20260520-inspec16-author-wepp-input-spec-chaninp-001/`
  - Purpose: author and disposition canonical `chan.inp` specification
    (`chaninp.spec.md`).
- `20260525-refactor002-openwepp-hillslope-orchestrator-lib-mechanical-modularization-001/`
  - Purpose: execute REFACTOR002 by mechanically modularizing
    `openwepp-hillslope-orchestrator/src/lib.rs`, preserving public
    API/runtime behavior, and updating layout-coupled tests to remain
    contract-accurate under the new module tree.
- `20260525-refactor003-openwepp-hillslope-orchestrator-hydrology-mechanical-modularization-001/`
  - Purpose: execute REFACTOR003 by mechanically modularizing
    `openwepp-hillslope-orchestrator/src/hydrology.rs` into multiple source
    files while preserving exported API/runtime behavior and typed guard
    semantics.
- `20260525-refactor004-openwepp-hillslope-orchestrator-runtime-inputs-mechanical-modularization-001/`
  - Purpose: execute REFACTOR004 by mechanically modularizing
    `openwepp-hillslope-orchestrator/src/runtime_inputs.rs` into multiple
    cohesive source files while preserving exported API/runtime behavior and
    typed guard semantics.
- `20260526-refactor005-openwepp-hillslope-orchestrator-kernel-support-mechanical-modularization-001/`
  - Purpose: execute REFACTOR005 by mechanically modularizing
    `openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
    into multiple section files while preserving exported API/runtime behavior
    and typed guard semantics.
- `20260525-mofe01-multi-ofe-readiness-assessment-and-queue-001/`
  - Purpose: assess MOFE readiness across routing plus slope/landuse/soil
    OFE-cardinality parity and produce a dependency-aware
    `mofe-readiness-wp-queue.md` follow-on package plan.
- `20260525-mofe02-cross-file-ofe-parity-hard-gate-001/`
  - Purpose: implement a hard hillslope intake gate that enforces
    slope/management/soil OFE-count parity before runtime-surface merge,
    using contract-first sequencing and contract-derived mismatch tests.
- `20260525-mofe03-wave2-routing-activation-and-input-synthesis-001/`
  - Purpose: make EROD14 Wave-2 routing executable from production runfile
    intake by defining activation policy and deriving/seeding required
    `erod14_*` symbols from parsed/runtime surfaces with typed guards.
- `20260525-mofe04-output-publication-closure-001/`
  - Purpose: close multi-OFE WB13/WAT publication assumptions by defining
    explicit output policy/provenance semantics and implementing deterministic
    OFE-aware publication geometry behavior for MOFE runs.
- `20260525-mofe05-watershed-contributor-metadata-and-intake-validation-001/`
  - Purpose: add watershed contributor MOFE metadata intake surfaces and typed
    hard-fail validation so malformed or inconsistent contributor metadata is
    rejected at the watershed boundary before routing execution.
- `20260525-mofe06-single-hillslope-semantic-parity-carved-letter-001/`
  - Purpose: execute one carved-letter MOFE hillslope semantic-parity lane by
    selecting a reasonable-closure MOFE hillslope via wepppy audit tooling and
    running openWEPP candidate/comparator execution with typed blocker capture.
- `20260525-mofe07-carved-letter-slope-soil-parser-compatibility-001/`
  - Purpose: close MOFE06 carved-letter parser blockers by implementing
    contract-authorized compatibility-mode slope/soil intake support needed for
    single-hillslope semantic parity execution.
- `20260525-mofe08-cligen-5323-compat-and-h324-parity-rerun-001/`
  - Purpose: add CLIGEN `5.323` compatibility policy for openWEPP climate
    intake (`>=5.3,<5.4 -> 5.3`), rerun carved-letter `H324` MOFE semantic
    parity lane, and align external CLIGEN versioning guidance with openWEPP
    compatibility expectations.
- `20260525-mofe09-hs-runtime-e-003-soil-runtime-fallback-and-h324-parity-rerun-001/`
  - Purpose: close carved-letter `H324` runtime soil seam blocker `HS-RUNTIME-E-003` by contract-authorized measured-theta fallback for legacy `7778` soils and rerun semantic parity.
- `20260525-mofe10-legacy-gddmax-runtime-resolution-and-h324-parity-rerun-001/`
  - Purpose: replicate legacy `gddmax=0` sentinel resolution (yldopt/gdmax monthly-climate behavior) in openWEPP runtime seams and rerun carved-letter `H324` MOFE semantic parity.

- `20260525-mofe11-oratea-zero-domain-compatibility-and-h324-parity-rerun-001/`
  - Purpose: close carved-letter `H324` runtime decomposition-rate blocker by replicating legacy-compatible `oratea/orater=0` (no-decay) behavior, then rerun MOFE semantic parity.
- `20260525-mofe12-h2637-closure-spike-replication-diagnostic-001/`
  - Purpose: run an incident-aligned diagnostic lane for `H2637` to determine
    whether openWEPP reproduces the documented day-44 closure-spike defect
    signature from the uncapped-spectacular ablation package.
- `20260525-mofe13-ksatadj-three-regime-kernel-alignment-001/`
  - Purpose: align openWEPP WB14 conductivity selection with
    `wepp-forest_260430` `ksatadj` regime authority (9001 exponential recovery,
    9002 Saxton-Rawls Brooks-Corey effective K, 9003 burn-severity floor) and
    rerun carved-letter `H324` parity evidence.
- `20260526-hrref01-hairsine-rose-references-intake-001/`
  - Purpose: execute references-intake step for the Hairsine-Rose multi-class
    sediment model concept backlog by adding R-17+ entries to
    `references/annotated_bibliography.md`, performing first-pass rights
    classification, and best-effort vendoring of acquirable artifacts. Docs-only;
    closes the "reference closure" promotion-from-backlog acceptance criterion.
