# CANOPY-CAL-04B Calibration Readiness and Ensemble Execution

Package ID:
`20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001`

Status: `COMPLETE / CALIBRATION READINESS PASS / VALIDATION LIMITATION RETAINED`

Date opened: `2026-07-27`

Execution mode: `package-end-to-end`

Package type: empirical timing calibration, deterministic ensemble execution,
calibration-readiness demonstration, identifiability analysis, and independent
holdout evaluation.

This ExecPlan is a living document maintained under
`docs/codex_exec_plans.md`.

Current execution truth: the corrected synthetic design, twelve-case native
consumer proof, full 9,261-candidate Hubbard ensemble, dual reconstruction,
later-stage readiness, 177-member transitive freeze, two independent
freeze-verifier receipts, one-time Harvard scoring, and terminal validator all
pass. The frozen accepted ensemble contains 37 candidates. Harvard scored all
37 without refit. Thirty-four candidates have zero interval coverage; three
have limited positive coverage, with a maximum of 18.81%. Together with
43.48–72.46 day aggregate errors, this shows poor independent transferability.
The limitation is retained and did not alter calibration membership. Incidents
005–007 preserve every fail-closed correction before the one-time opening.

## Purpose

Close canopy roadmap Order 4 without requiring sparse observations to uniquely
identify every operand. Execute CAL-04A's frozen 9,261-vector GSI sensitivity
ensemble against admitted Hubbard calibration intervals, freeze and evaluate
the accepted ensemble once against the independent Harvard timing holdout, and
complete auditable calibration-readiness or empirical-calibration dispositions
for `Bf,max`, `Bs`, `fe`, `xmxlai`, `Cs`, and `bb`.

The package must demonstrate that the authoritative native canopy model and
its calibration machinery can consume suitable data. It must never represent
execution assumptions, sensitivity, synthetic recovery, or equifinal ranges as
unique physiological identification.

## Implementation Intent

Combined intent:

- `implementation`: verify that declared native parameters reach the real
  production canopy/GSI path;
- `calibration-readiness`: prove deterministic configuration, observation
  operators, objectives, sensitivity, recovery, boundary/failure reporting,
  and additional-data needs;
- `empirical-calibration`: fit only operands or combinations supported by
  admitted `CALIBRATION` observations;
- `independent-validation`: use Harvard only after the Hubbard ensemble,
  analysis identities, and command are frozen.

## Authority and Dependencies

- ADR-0042 and the calibration-readiness schema;
- closed CAL-04 objective/operator, role bindings, failure posture, and
  historical evidence;
- CAL-04A source custody, Daymet derivations, evidence ledger, exact
  9,261-vector design, acceptance rule, saturation classes, and no-refinement
  stopping rule;
- CAL-03 protected native members, production research trace, observation
  ledger, and pre-calibration protocol;
- CAL-04/05 authority-admission timing ledger and immutable roles;
- `SC-PLANT-001` and `SC-INFILE-MANAGEMENT-YAML-001`.

CAL-04 and CAL-04A are read-only predecessor evidence. CAL-05 is not a
dependency and remains separately scoped.

## Included Scope

- Rebuild the CAL-04A grid byte-identically.
- Prove parameter injection through the real production consumer.
- Execute synthetic recovery and objective-reconstruction cases before
  empirical scoring.
- Execute all 9,261 complete GSI threshold vectors in frozen order.
- Retain every candidate, annual/component score, failure, boundary,
  saturation, and profile/equifinality result.
- Apply the frozen accepted-ensemble rule:
  finite equal-year interval RMSE no more than `minimum finite RMSE + 1 day`.
- Freeze ensemble/tool/configuration/command identities before Harvard access.
- Open and score the Harvard modeled timing holdout once without refitting.
- Propagate accepted upstream uncertainty through later ordered canopy stages.
- Empirically constrain supported combinations and demonstrate readiness for
  data-limited operands.
- Publish the ADR-0042 readiness matrix, three orthogonal status fields, and
  additional-data inventory for every stage.

## Excluded Scope

- No GSI refinement, domain widening, probability-prior interpretation, or
  physiological-bound claim.
- No Harvard observation, modeled trace, or score may influence calibration,
  tolerance, weights, domains, stopping, or ensemble membership.
- No downstream hydrology, erosion, snow, frost, interception, or ET result may
  select canopy parameters.
- No litter-source or decomposition fitting.
- No production physics, science-contract, protected fixture, admitted source,
  or predecessor artifact edits.
- No collapse of equifinal ranges to one preferred vector.

## Declared Write Set

- `docs/work-packages/README.md`
- `docs/planning/canopy-phenology-assurance-roadmap.md`
- this package subtree

All production code, canonical contracts, tests, protected fixtures, source
observations, retained Daymet objects, CAL-04, and CAL-04A are read-only.
Package-local scripts and temporary/configuration copies are allowed.

## Frozen GSI Design

`artifacts/calibration-forcing-authority-resolution.md` is the binding,
result-blind amendment that resolves CAL-04's older composite-member operator
against CAL-04A's later nine-plot forcing authority. All other objective and
holdout semantics remain unchanged.

- Population: nine Hubbard plots, 1989–2024, spring forcing support.
- Forcing: CAL-04A's checksum-bound plot-specific Daymet daily derivation and
  source-EML plot latitude; the protected `p10.cli` fixture is comparison and
  native-path-proof evidence only, never calibration forcing.
- State/calendar rule: each `(plot_id, year)` is a separate native
  `GsiState` cold start. Admit only real Daymet yday 1–180 in order, with no
  synthetic prefill or cross-year state carry. Days 1–59 are warm-up and are
  ineligible for crossing selection; the first upward crossing is selected
  only within the frozen yday 60–180 spring-support window.
- Threshold levels: q00, q05, q25, q50, q75, q95, q100 for temperature, VPD,
  and photoperiod.
- Family profiles: every strictly ordered pair, 21 per family.
- Complete vectors: `21 × 21 × 21 = 9,261`.
- Enumeration: lexicographic temperature pair, VPD pair, photoperiod pair.
- Seed: none.
- Objective: CAL-04 equal-year interval RMSE.
- Missing required crossing: objective `+infinity`, retained failure evidence.
- Accepted ensemble: all finite vectors within 1.0 day of the finite minimum.
- Refinement: none.
- Boundary widening: forbidden.
- Saturation: CAL-04A's exact `1e-12` factor-range rule.

## Ordered Stage Strategy

1. **GSI timing.** Empirically calibrate the complete vector against Hubbard
   timing while retaining partial/nonidentifiability.
2. **`Bf,max` and `Bs`.** Use admitted total/partition evidence only for the
   supported sum or combinations; retain non-separable pairs.
3. **`fe`.** Use admitted evidence if quantitatively eligible; otherwise
   demonstrate sensitivity and synthetic recovery across an explicit
   `ASSUMED_FOR_EXECUTION` axis.
4. **`xmxlai`.** Apply mature-LAI evidence only after propagating accepted GSI
   and `fe` uncertainty; retain conditional/equifinal ranges.
5. **`Cs` and `bb`.** Demonstrate paired cover-response readiness and synthetic
   recovery. Claim empirical calibration only if quantitative admitted cover
   authority supports the observation operator.
6. Freeze every accepted upstream range before evaluating the next stage.

Data-limited stages complete as calibration-ready when every applicable
readiness row passes. Sparse or non-identifying data alone is not a hold
boundary. A broken real parameter path, unreconstructable objective, failed
required synthetic recovery, or other required readiness defect is a hold.

## Phase Plan

The correction strategy in `artifacts/rework-strategy.md` is binding. No
result-bearing execution may begin until the reworked scaffold receives two
prospective scientific PASS reviews, two independent scaffold-verification PASS
records, and every row in `artifacts/prospective-finding-ledger.csv` is
`CONTROL_ACCEPTED`.

### Phase 1: prospective intake and calibration machinery

Authenticate all predecessor identities and immutable roles. Rebuild the
9,261-vector grid. Freeze later-stage execution-assumption axes, objectives,
enumeration, acceptance, failure, boundary, and stopping behavior before
result-bearing work.

Implement separate native-proof, calibration-producer, independent-
reconstruction, readiness-stage, freeze, and holdout commands. Prove real
parameter-path consumption for representative interior, boundary, saturated,
and invalid cases. The population producer must call the actual Rust GSI kernel;
an independently implemented equation reconstruction is verification evidence
only. Complete dual scaffold review, then dual executor review, before empirical
candidate execution.

### Phase 2: Hubbard GSI execution

Run the full deterministic ensemble across every candidate, all nine plots,
and all 36 plot-years. Retain configurations, plot-keyed daily state,
modeled crossings, observation distances, species diagnostics, annual
components, aggregate losses, failures, flags, and profiles. Rebuild
objectives and membership independently from immutable daily GSI traces,
frozen configurations, and admitted observations. The reconstructor derives
crossings by `(candidate_id, plot_id, year)`, joins each record by
`(candidate_id, plot_id, year, record_id)`, and derives distances, annual
components, objective, and membership without reading producer crossings,
components, aggregates, or scoring code, then freezes the accepted ensemble.

### Phase 3: Harvard one-time independent validation

Checksum-freeze ensemble membership, executable/source, configuration, input
manifest, and exact command in `artifacts/holdout-freeze-manifest.csv`. Two
independent verifiers must PASS the nonempty freeze before a separate holdout
command can transition `SEALED` to `OPENED_ONCE`. Open Harvard modeled results
once, use the first `previous > 0.5 && current <= 0.5` crossing, score the frozen
ensemble, and retain results without any calibration write capability.

### Phase 4: remaining ordered operands

For each later stage, freeze units, axes, observation operator, enumeration,
objective, acceptance, and failure behavior before execution. Propagate
upstream accepted membership, execute supported empirical constraints, and
compute sensitivity and synthetic recovery for unsupported axes. Retain
equifinality rather than selecting convenient values. Hard-coded PASS rows are
not evidence.

### Phase 5: roadmap closure

Complete the readiness matrix, stage status ledger, additional-data inventory,
dual scientific review, finding disposition, selected gates, dual terminal
verification, exact-diff reconciliation, prompt archival, package disposition,
and roadmap Order 4 disposition.

## Required Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/rework-strategy.md`
- `artifacts/execution-control-contract.md`
- `artifacts/later-stage-design.csv`
- `artifacts/prospective-finding-ledger.csv`
- `artifacts/holdout-freeze-manifest.csv`
- `artifacts/freeze-verifier-receipts.csv`
- `artifacts/executor-command-plan.csv`
- `artifacts/executor-schema.md`
- `artifacts/synthetic-gsi-design.csv`
- `artifacts/native-proof-case-plan.csv`
- `artifacts/scaffold-verification-agent-a.md`
- `artifacts/scaffold-verification-agent-b.md`
- `artifacts/intent-plan.md`
- `artifacts/input-and-authority-manifest.csv`
- `artifacts/calibration-readiness-matrix.md`
- `artifacts/stage-status-ledger.csv`
- `artifacts/gsi-domain-grid.csv`
- `artifacts/synthetic-recovery-results.csv`
- `artifacts/candidate-ledger.csv`
- `artifacts/failure-ledger.csv`
- `artifacts/accepted-calibration-ensemble.csv`
- `artifacts/identifiability-and-equifinality.md`
- `artifacts/holdout-opening-record.md`
- `artifacts/harvard-holdout-results.csv`
- `artifacts/later-stage-results.csv`
- `artifacts/additional-data-inventory.csv`
- exact command and execution inventories
- prospective dual reviews and finding disposition
- terminal dual reviews and finding disposition
- gate evidence and dual terminal verification
- `artifacts/final-disposition.md`

Large candidate traces may be retained in checksum-bound external package
objects only when the manifest, command, configuration, and rebuild path are
complete and independently verified.

## Required Status Fields

Report package- and stage-level:

- `science_implementation_status`;
- `calibration_evidence_status`;
- `identifiability_status`.

Empirical calibration may coexist with partial or nonidentifiability. Missing
A4 validation limits external-validation and transferability claims without
changing calibration status.

## Advancement and Acceptance

Roadmap Order 4 may close when:

1. the full GSI ensemble executes deterministically and is independently
   reconstructable;
2. Hubbard calibration and Harvard independent-validation evidence are
   retained separately;
3. every later operand is empirically calibrated where supported or passes all
   applicable calibration-readiness obligations;
4. all three status fields are reported for every stage;
5. execution assumptions and synthetic recovery are never presented as
   observations or empirical calibration;
6. the additional-data inventory identifies measurements needed for stronger
   separation;
7. no holdout or downstream response influenced selection; and
8. all required gates, reviews, dispositions, and verifications pass.

Harvard validation failure is retained and limits validation claims; it cannot
trigger refitting. `HOLD` is required only for failed current-scope
implementation/correctness/readiness gates, missing/contradictory authority
with no valid admission route, or an external dependency that cannot be
resolved inside scope.

## Review and Delegation Authorization

Subagent authorization: this package explicitly authorizes subagent
spawning/delegation to two prospective reviewers, two terminal scientific
reviewers, two terminal verifiers, and the `comparator_suite_runner` for heavy
candidate/population execution; expected outputs are compact review,
verification, metrics, receipt, and artifact-path results; reviewer/verifier
write access is read-only and comparator write access is limited to package-local
results/logs plus the exact checksum-bound external object paths under
`/home/workdir/cal04b-objects/` enumerated by
`artifacts/executor-command-plan.csv`.

## Resume Increment — 2026-07-27

User direction reopened this package after
`20260727-cal04b-native-gsi-canopy-height-coherence-hold-lift-001`, the typed
assurance identity correction, and the TESTGATE historical-root correction all
closed. Exact prerequisite evidence includes the corrected 12-case native
proof, dual terminal verification, and full-workspace run
`7e79049d-0871-4142-a9f7-86ac7ac714be` with 2,301/2,301 tests passing.

The resume increment preserves every frozen design, operator, domain,
acceptance rule, stopping rule, command, and Harvard custody boundary. Before
result-bearing execution it must:

- archive the current `/home/workdir/cal04b-objects/` tree byte-preservingly;
- create a new empty `/home/workdir/cal04b-objects/` attempt root;
- rebind source, binary, command, repository-head, and prerequisite identities;
- rerun the scaffold/executor pre-heavy gates and dual read-only restart review;
- restart the observed DAG from `prepare`, never from an intermediate receipt;
- keep Harvard sealed through complete Hubbard production, dual reconstruction,
  readiness, freeze, and two independent freeze-verifier PASS receipts.

Resume progress:

- [x] production, assurance, and TESTGATE prerequisite chain closed;
- [x] package and byte-identical kickoff prompt reactivated;
- [x] fresh attempt root and identities reviewed;
- [x] observed pre-freeze prefix passed from `prepare`;
- [x] freeze and dual verifier barrier passed;
- [x] one-time Harvard holdout and post-holdout summary completed;
- [x] terminal validation passed;
- [x] dual review, dual verification, and closeout passed.

## Minimum Gates

```text
(cd references/canopy_phenology/daymet_calibration && sha256sum -c SHA256SUMS)
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python <package>/tools/validate_scaffold.py
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python <package>/tools/validate_executor.py
cargo test --manifest-path <package>/tools/executor/Cargo.toml
cargo clippy --manifest-path <package>/tools/executor/Cargo.toml --all-targets -- -D warnings
cargo deny --manifest-path <package>/tools/executor/Cargo.toml check
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python <package>/tools/validate.py
markdown-doc lint --path <package>
git diff --check
```

The scaffold and executor validators are pre-heavy gates. The result validator
is terminal and must authenticate producer identity, dual reconstruction,
membership propagation, freeze receipts, one-shot holdout state, and every
required status artifact. Production code remains read-only; the package-local
executor calls the existing native kernel and the native-proof command checks
the production consumer path.

The package-local `.gitattributes` treats the byte-identical inherited CAL-04A
CRLF grid as binary for diff purposes; all authored text retains normal
whitespace checking.

## Outcome

This result closes roadmap Order 4. The corrected fresh attempt evaluated all
9,261 frozen GSI candidates and independently reconstructed the same candidate,
failure, and accepted-membership ledgers. Of 1,598 finite candidates, 37 are in
the frozen minimum-plus-one-day ensemble; the best Hubbard objective is
4.020496520239 days. Required synthetic recovery, native production-consumer
proof, later-stage propagation, readiness, freeze, dual review, dual
verification, and terminal validation pass.

Stage dispositions are:

- GSI timing: `IMPLEMENTED / EMPIRICALLY_CALIBRATED /
  PARTIALLY_IDENTIFIABLE`;
- foliar/structural partition: `IMPLEMENTED / EMPIRICALLY_CALIBRATED /
  PARTIALLY_IDENTIFIABLE`;
- evergreen fraction: `IMPLEMENTED / CALIBRATION_READY_DATA_LIMITED /
  NONIDENTIFIABLE`;
- peak LAI: `IMPLEMENTED / CALIBRATION_READY_DATA_LIMITED /
  PARTIALLY_IDENTIFIABLE`;
- canopy floor/closure: `IMPLEMENTED / CALIBRATION_READY_DATA_LIMITED /
  NONIDENTIFIABLE`.

The one-time Harvard evaluation scored every frozen candidate without refit or
failure. Thirty-four candidates have zero interval coverage. `GSI-4831`,
`GSI-4832`, and `GSI-4837` have limited positive coverage of 18.81%, 3.76%,
and 3.13%, respectively. Aggregate errors span
43.475807448894–72.457861317228 days. This is contrary
independent-validation evidence: it limits external validity and
transferability and cannot trigger refitting. It does not invalidate the
Hubbard empirical-calibration or data-limited readiness evidence.
