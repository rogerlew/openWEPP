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
