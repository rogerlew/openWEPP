# Work Packages

Initiative tracking convention inherited from wepp-palimpsest. Each work package lives in a dated directory under this tree.

## Directory naming
`YYYYMMDD-<short-slug>/`

## Required files
- `package.md` — scope, deliverables, dependencies, exit criteria
- `prompts/` — agent prompts (active and archived)
- `artifacts/` — produced docs, contracts, evidence

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
