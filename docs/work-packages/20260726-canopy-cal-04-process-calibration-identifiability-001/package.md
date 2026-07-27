# CANOPY-CAL-04 Process Calibration and Identifiability

Package ID: `20260726-canopy-cal-04-process-calibration-identifiability-001`

Status: `EXECUTED / HOLD`

Date opened: `2026-07-26`

Execution mode: `package-end-to-end`

Package type: observational calibration, deterministic identifiability, and
independent holdout evaluation.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`. Keep `Progress`, `Surprises & Discoveries`,
`Decision Log`, and `Outcomes & Retrospective` current during execution.

## Purpose / Big Picture

Estimate defensible ranges for native canopy-process operands in their declared
process order, quantify nonidentifiability and boundary hits, and evaluate the
frozen result against an independent Harvard Forest timing holdout. The
package must leave a reproducible ensemble and complete failure ledger rather
than only a preferred vector.

CAL-04 consumes the authority admitted by
`20260726-canopy-cal-04-05-authority-evidence-admission-001`. It does not
reopen observation roles, invent probability priors, fit decomposition, alter
process physics, or use downstream hydrology and erosion as parameter-selection
authority.

## Progress

- [x] (2026-07-26) Scaffolded the execution contract, declared write set,
  queued artifacts, active kickoff prompt, review authorization, and catalog
  links.
- [x] (2026-07-26) Read and checksum the complete required-reading and
  authority set.
- [x] (2026-07-26) Authenticated the prospective hold intent and exact
  scientific plus closure execution inventory.
- [x] (2026-07-26) Rebuilt the admitted timing windows byte-identically and
  verified protected native member bindings.
- [x] (2026-07-26) Froze aggregation, failure, ordering, advancement, and
  embargo behavior; independent review confirmed that finite GSI search bounds
  and a lawful initial grid cannot be frozen from admitted authority.
- [x] (2026-07-26) Held GSI timing before candidate execution because the typed
  validation domain is not a finite scientific search domain.
- [x] (2026-07-26) Held later ordered stages before execution; retained the
  admitted partition-sum and mature-LAI roles while recording their
  non-separating or upstream-conditional limitations.
- [x] (2026-07-26) Recorded zero candidates, zero model failures, no empirical
  profile, and no accepted or equifinal range.
- [x] (2026-07-26) Preserved the Harvard seal; no holdout result was opened.
- [x] (2026-07-26) Did not run downstream evaluation because no accepted
  ensemble exists.
- [x] (2026-07-26) Completed selected gates, dual scientific review, finding
  disposition, dual terminal verification, post-verification reconciliation,
  and truthful final hold disposition.

## Surprises & Discoveries

- The contracts and validators define process-input admissibility but do not
  define finite calibration search bounds for the complete six-operand GSI
  vector.
- The frozen CAL-03 ledger retains biomass partition-sum and mature-LAI
  calibration roles that the initial execution draft overlooked. Prospective
  review caught the omission before result-bearing work; the corrected
  artifacts retain both roles without overstating their separating power.

## Decision Log

- Decision: Bind timing calibration to Hubbard Brook P3 and holdout evaluation
  to Harvard Forest 50% leaf fall.
  Rationale: these are the independently reviewed, directly scoreable timing
  endpoints admitted before fitting.
  Date/Author: 2026-07-26 / Codex.
- Decision: Use a deterministic, probability-prior-free search.
  Rationale: contract and schema domains are execution bounds, not scientific
  probability distributions.
  Date/Author: 2026-07-26 / Codex.
- Decision: Fit in process order and freeze accepted upstream ranges.
  Rationale: a joint optimum could conceal timing, biomass-partition, LAI, or
  canopy-cover errors through compensation.
  Date/Author: 2026-07-26 / Codex.
- Decision: Stop before the first result-bearing candidate and close `HOLD`.
  Rationale: a bounded deterministic GSI search would require invented numeric
  authority, and current/default/Bill/legacy values are explicitly barred from
  supplying it.
  Date/Author: 2026-07-26 / Codex.

## Outcomes & Retrospective

Executed to a prospective authority hold. Intake checks and the deterministic
timing-window rebuild passed, but no lawful finite GSI grid could be frozen.
No candidate, parameter range, model failure, accepted ensemble, Harvard
result, or downstream result is established. A follow-on requires
prospectively admitted finite GSI bounds/grid and sufficient separating
authority for later stages.

## Context and Orientation

CAL-03 installed protected native members and the daily research trace. The
authority-admission package subsequently retained 932 Hubbard Brook spring P3
calibration intervals through 2024 and 319 Harvard Forest 50% leaf-fall
holdout intervals, excluding Harvard fall 1992.

The frozen timing observation operator reads `/gsi/gsi21` from
`openwepp-canopy-research-daily-v1`. Hubbard P3 uses the first upward daily 0.5
crossing; Harvard leaf fall uses the first downward daily 0.5 crossing.
Missing required crossings invalidate a candidate with infinite objective and
remain counted.

The protected members are:

- Hubbard calibration:
  `tests/fixtures/cancov_forest/hubbardbrook_deciduous_nh/p10.native.run.toml`,
  SHA-256
  `7938bd6fa16614230bf38b02d746ba3d8d2af9ad185bc71f78c659d49c4e1498`;
- Harvard holdout:
  `tests/fixtures/cancov_forest/harvard_deciduous_ma/p6.native.run.toml`,
  SHA-256
  `1ddc0833a914f04153bb149b1c7535de2f7588fc53cdd66e695fa0fa60a98997`.

Each member emits one composite deciduous crossing per calendar year. Within
each year, squared interval distances are averaged over eligible observations;
the objective is the square root of the unweighted mean of those annual means.
This equal-year rule prevents years with more trees or subplots from receiving
extra weight.

## Included Scope

The executor may create package-local analysis and orchestration tools, copy
protected fixtures into package-local or temporary run directories, execute
the native production runner with the research trace enabled, and retain
configuration, command, result, failure, profile, ensemble, and provenance
artifacts under this package.

The executor may evaluate the following native operands in order:

1. GSI minimum-temperature, vapor-pressure-deficit, and photoperiod thresholds;
2. summer foliar biomass `Bf,max` and persistent structural biomass `Bs`;
3. evergreen fraction `fe`;
4. peak LAI control `xmxlai`;
5. canopy-cover coefficient `Cs` and structural cover floor `bb`.

The executor may use only already admitted observations assigned to the
current stage. It may report snow, interception, ET, runoff, frost, and erosion
responses after the canopy ensemble is frozen.

## Excluded Scope

Do not alter Rust production code, process equations, science contracts,
protected fixtures, source observations, immutable calibration/holdout roles,
or retained source bytes. Do not fit litter-source or decomposition operands.
Do not use Bill Elliot values, current defaults, legacy agreement, qualitative
calendar phrases, Harvard holdout results, or downstream hydrology/erosion to
select parameters, domains, weights, tolerances, refinements, or stopping
rules.

Do not collapse an equifinal range to a single vector, discard failed members,
widen a domain after a boundary hit, or reopen an upstream stage without a
recorded finding and prospectively reviewed joint-fit amendment. Do not expose
Harvard results until the accepted calibration ensemble and analysis code are
checksum-frozen.

## Declared Write Set

- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- `docs/work-packages/20260726-canopy-cal-04-process-calibration-identifiability-001/**`

All production Rust, science contracts, source observations, retained research
objects, protected managements, runfiles, climates, soils, slopes, and prior
package artifacts are read-only.

## Dependencies

- `20260726-canopy-cal-03-observation-native-research-001`: `COMPLETE`
- `20260726-canopy-cal-04-05-authority-evidence-admission-001`:
  `EXECUTED / HOLD`, with both CAL-04 authority gaps `LIFTED`
- CAL-03 protected native fixture and research-output hashes unchanged
- Authority source and canopy corpus checksum manifests passing
- Production native runner capable of emitting the frozen research schema

The unresolved CAL-05 operator request does not block CAL-04.

## Required Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/intent-plan.md`
- `artifacts/input-and-authority-manifest.csv`
- `artifacts/search-domain-and-stage-plan.csv`
- `artifacts/objective-and-observation-operator.md`
- two independent prospective-freeze review artifacts
- `artifacts/execution-inventory.csv`
- `artifacts/command-log.csv`
- `artifacts/candidate-ledger.csv`
- `artifacts/failure-ledger.csv`
- `artifacts/stage-disposition.csv`
- `artifacts/identifiability-and-equifinality.md`
- `artifacts/accepted-calibration-ensemble.csv`
- `artifacts/holdout-opening-record.md`
- `artifacts/harvard-holdout-results.csv`
- `artifacts/downstream-evaluation.md`
- `artifacts/gate-evidence.md`
- two independent terminal review artifacts
- `artifacts/finding-disposition.md`
- two independent verification artifacts
- `artifacts/final-disposition.md`

Large daily traces and model outputs may be retained through checksum-bound
manifests when direct repository retention is impractical. Every relied-upon
result must remain reproducible from retained commands, inputs, tools, and
configuration.

## Plan of Work

### Phase 1: intake and prospective freeze

Read the required packages, protocol, admitted sources, trace schema,
controlling contracts, and current runner interface. Rebuild timing windows
byte-identically and verify protected member hashes.

Before affected execution, freeze:

- exact parameter names, units, contract/schema domains, and initial grid;
- deterministic enumeration order and any seed;
- stage-specific observations and objective components;
- equal-year aggregation and missing-crossing invalidation;
- refinement triggers, maximum refinements, stopping rules, and boundary-hit
  behavior;
- failed-run retention and retry policy;
- stage advancement and reopening rules;
- holdout embargo and one-time opening procedure.

No result-bearing run may occur until two independent reviewers approve this
prospective freeze.

### Phase 2: ordered calibration and identifiability

Execute GSI timing first using only Hubbard calibration intervals. Retain every
candidate, component loss, annual score, failure, and boundary hit. Profile
each operand and retain materially equifinal ranges.

Proceed through foliar/structural biomass, evergreen fraction, peak LAI, and
canopy floor/closure only with observations eligible for that stage. Freeze
accepted upstream ranges before the next stage. A stage that lacks adequate
authority or identifiability closes with its full range or a truthful hold; it
does not borrow authority from a downstream response.

### Phase 3: ensemble freeze and independent holdout

Freeze the accepted calibration ensemble, analysis tool, exact hashes, and
holdout command before reading Harvard results. Then open the Harvard member
once and compute the identical equal-year timing statistics. A holdout failure
is retained and cannot trigger refitting, remapping, reweighting, domain
changes, or tolerance changes.

### Phase 4: downstream evaluation and closure

Run downstream canopy consumers only after the ensemble is frozen. Report
effects and ordering without using them to select parameters. Reconcile the
terminal diff, complete dual scientific review and finding disposition, then
obtain two independent terminal verifications.

## Validation and Acceptance

CAL-04 may close `COMPLETE / PASS_BOUNDED` only if:

- all relied-upon source, fixture, runner, configuration, tool, and result
  identities are checksum-bound;
- the admitted Hubbard/Harvard roles and exact observation operator remain
  unchanged;
- every attempted candidate and failure is retained;
- no probability prior or retrospective domain/refinement is introduced;
- each stage has an explicit disposition and upstream freeze;
- parameter profiles, correlations, boundary hits, and equifinal ranges are
  reported;
- the accepted ensemble is frozen before Harvard is opened;
- Harvard is evaluated once and never influences fitting choices;
- downstream consumers are evaluation-only;
- two reviews, finding disposition, selected gates, and two terminal
  verifications pass.

`NONIDENTIFIABLE`, `BOUNDARY_HIT`, `CALIBRATION_FAIL`, `HOLDOUT_FAIL`, and
`MODEL_FAILED` are legitimate scientific outcomes, not permission to select a
convenient vector. The package closes `HOLD` when a required stage lacks
authority or executable evidence. It may close `COMPLETE` with a negative
scientific result only when the complete prespecified workflow and gates pass.

## Selected Gates

- authority and protected-input checksum verification;
- exact timing-window deterministic rebuild;
- authenticated intent and terminal execution inventories;
- observation-role immutability and calibration/holdout disjointness;
- prospective search-domain, refinement, aggregation, and stopping-rule freeze;
- deterministic candidate enumeration and rebuild;
- candidate/failure ledger completeness;
- independent objective reconstruction from retained daily traces;
- stage-order and upstream-freeze enforcement;
- holdout embargo and one-time-open evidence;
- no holdout or downstream parameter-selection contamination;
- no probability-prior mislabeling;
- documentation lint, full diff hygiene, prompt state, and write-set
  reconciliation;
- two independent scientific reviews, finding disposition, and two independent
  terminal verifications.

No Rust production edit is authorized, so Rust correctness and comparator
campaigns are not selected by this scaffold. If execution discovers a
production defect, stop affected scoring, retain the failure, and scaffold a
separate defect-closure package rather than repairing it here.

## Review and Delegation Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two prospective scientific reviewers, bounded execution
workers, two terminal scientific reviewers, and two terminal verifiers for the
scopes declared in this package; expected outputs are prospective-freeze
findings, execution evidence, two terminal review artifacts, and two terminal
verification artifacts; write access is read-only for reviewers/verifiers and
bounded to assigned package-local artifacts or run directories for execution
workers.

At least one prospective reviewer must challenge the model-to-observation
operator, equal-year aggregation, and holdout embargo. At least one must
challenge search domains, refinement, stopping, process ordering, and failure
handling. Terminal reviewers must inspect raw retained execution evidence, not
only summaries. Verifiers must independently enumerate the exact terminal
inventory and confirm Harvard and downstream results did not influence
selection.

## Concrete Steps

Work from `/home/workdir/openWEPP`. Begin with:

    tools/agents/find-agents --for \
      docs/work-packages/20260726-canopy-cal-04-process-calibration-identifiability-001/package.md \
      docs/planning/canopy-phenology-assurance-roadmap.md

Use `.venv/bin/python` for package-local Python. Use fresh temporary or
package-local run directories; never execute into protected fixture
directories. Record the exact native runner binary identity and every command.

Before disposition, run at minimum:

    sha256sum -c references/canopy_phenology/authority_admission/SHA256SUMS
    sha256sum -c tests/fixtures/cancov_forest/observations/canopy_phenology/SHA256SUMS
    markdown-doc lint --path \
      docs/work-packages/20260726-canopy-cal-04-process-calibration-identifiability-001
    git diff --check

Also run package-local validators for manifest joins, candidate completeness,
stage order, objective reconstruction, deterministic rebuild, and holdout
embargo. Record commands, exit status, and evidence paths in
`artifacts/gate-evidence.md`.

## Security and External-Action Gate

Do not commit credentials, tokens, cookies, private URLs, or unauthorized
source material. This package does not authorize purchases, account creation,
access-control bypass, external contact, production deployment, remote job
dispatch, or publication. External execution requiring new authority must stop
and request operator direction.
