# Directory-contract checker and LSE adoption
Status: executing; legacy LSE remains canonical until all adoption gates pass.
Execution mode: package-end-to-end.
Implementation intent: directory-v1 checker implementation + meaning-preserving LSE contract migration + validated selective reading.
Baseline: b932db101cce07d0860b45b5ecaa8ddb7f455b58 on main.

## Objective, authorization and frozen acceptance
The complete owner kickoff is retained in prompts/active/execute.md and is the
frozen substantive execution/acceptance contract, including Phases 0–5 and all
conformance, preservation, usability, review and terminal verification gates.
No gate is deferred. Implement a usable modular LSE contract and the bounded
checker; preserve all scientific meaning and qualification/activation limits.
No production/runtime/solver changes, other-contract migration, historical package
rewrites, branch changes, remote publication or push. Owned local commits authorized.
Preserve unrelated untracked $pkg/ and tmp/.

## Initial exact write set
- tools/check_sc_binding_exposure.py
- tools/sc_contract_directory.py
- tools/release/check_sc_unit_compliance.py
- tests/python/test_check_sc_binding_exposure.py
- tests/python/test_sc_contract_directory.py
- docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
- docs/work-packages/active.md
- docs/work-packages/README.md
- This package: package.md, prompts/active/execute.md, prompts/archived/README.md;
  artifacts/required-reading-map.md, worker-handoff.md, source-manifest.json,
  clause-preservation-map.md, checker-consumer-conformance.md,
  context-usability.md, review_agent_a.md, review_agent_b.md,
  finding-disposition.md, verification_agent_a.md, verification_agent_b.md,
  gate-results.md, line-count-governance.md, final-disposition.md, logs/.
Exact chapter and consumer paths must be added before their edits, after source inventory.

## Phases and assurance
Execute kickoff phases sequentially. Freeze source clauses and reading rubric
before migration; checker conformance precedes LSE relocation. Reviewers inspect
complete original authority. Two independent reviewers and two independent terminal
verifiers are explicitly authorized, restricted to assigned artifacts. Fresh
non-forked read-only exercise agents are explicitly authorized for four tasks.
Correctness high, QA medium, interpretive verification medium; effective runtime
metadata UNOBSERVED unless supplied. Selected heavy checks require comparator_suite_runner.

## Validation and security impact
Substantive bounded parser/path-safety and authority-preserving document migration.
Run actual pytest/CLI conformance, strict real LSE lint, schema/profile/reference/
provenance/unit consumers, affected Rust authority tests and applicable formatting.
Independent corruption tests must retain protected scientific checks. No physics
campaign solely for relocation; escalate any semantic authority or test weakening.
No tolerance, fixture, required-suite or scientific expectation changes authorized.

## Dependencies and continuation
Authority: complete directory-format revision 1 at baseline; parent schema,
provenance, authoring, kernel profile, unit governance and testing strategy.
Source identities: artifacts/source-manifest.json; reading map records expansion.
Progress, discoveries, decisions, outcomes: artifacts/worker-handoff.md.
Recovery: preserve/reinstate baseline canonical LSE and whole-contract routing if
an in-envelope resolution cannot satisfy adoption; report exact unmet gates.

## Frozen chapter and consumer write set (before relocation)
Canonical chapter directory: docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001/.
Exact chapters: interface.md, surface-energy.md, soil-coupling.md, water-vapor.md,
nonlinear-solve.md, terminal-support.md, litter-phase.md, soil-custody.md,
surface-custody.md, map-custody.md, dependency-replay.md, qualification.md,
binding-index.md, history.md.
Additional exact consumer files under tests/integration/:
land_surface_energy_balance_authority_contract.rs,
solver_architecture_authority_contract.rs,
surface_liquid_hydrology_custody_authority_contract.rs,
snow_stage3_terminal_receiver_authority_contract.rs,
snow_stage3_shared_carrier_authority_contract.rs,
snow_stage3_terminal_chronology_v19_contract.rs,
snow_stage3_terminal_batch_temporal_v20_contract.rs,
snow_stage3_terminal_batch_temporal_v21_contract.rs,
vegetation_boundary_authority_contract.rs,
stage3_native_vegetation_laned_throughput_recovery.rs.
Shared test-only bridge: tests/integration/support/sc_contract_text.rs.
Package evidence additions: artifacts/build_candidate.py, clause-map.json,
candidate-manifest.json, source-reading-ranges.json and exercise_01.md through exercise_04.md.
Minimal adoption-status text: docs/specifications/science-contract-directory-format.md;
registry routing: docs/specifications/science-contracts/index.md. Existing guides
already condition selective reading on adoption and require no rule weakening.
No neighboring contract edits planned; existing section fragments resolve by the
reviewed bounded compatibility interpretation. Frozen JSON/model versions untouched.

Boundary assignment: interface retains scope/signs/units/status and baseline
conservation requirements; current constitutive sections divide into radiation/
turbulence, soil transfer and humidity/thermal state, water/vapor chronology,
and complete numerical solve. Terminal support includes represented-snow receiver
boundary; litter phase owns spill and mixed-resource joins. Soil and surface
exact custody retain their own arithmetic, receipts and chronology. Map custody
owns private validation/finalization; dependency replay owns evaluator graph and
error proof; qualification retains every historical/prospective acceptance limit.
History contains only original Change Log; no binding rule is made historical.
All original clause spans remain normative unless explicitly mapped to that log.
Repeated ID rows become nondefining references; one complete marked definition per
ID is placed in its owning chapter. Original detailed requirements remain binding.

## Prospective HOLD recovery write set
Both independent reviewers confirmed inherited required Rust FAIL cannot be
reclassified as a passing adoption gate. No forbidden production seam, historical
oracle/runtime predicate, or unrelated authority will be modified to green it.
After fresh exercises, preserve exact candidate under artifacts/candidate-tree/
with repository-relative paths for the 15 contract-set files and seven Rust
consumer files plus shared bridge. Add artifacts/candidate-recovery.json and
artifacts/restore_candidate.py: bounded isolated-tree reconstruction only, explicit
output argument, no edits to canonical checkout. Retain exact external dependency
identities in candidate-manifest.json; reconstruction uses original Git baseline
plus owned checker/consumer overlay. Original canonical LSE v31 and affected Rust
consumers will be restored from baseline using only owned changes. Checker and
unit consumer support remain. No selective route activation or spec status claim.
This is the kickoff-authorized fallback, not deferred/complete adoption. Final
assurance still required on corrected checker, retained candidate and recovery.

## Newly discovered A0 reader (before edit)
Exact additional write: tools/release/check_science_contract_admission.sh.
Actual candidate invocation rejects binding-index.md as independently unapproved
contract because changed-file logic uses every nested filename stem. Reconcile
chapter paths to validated parent inventory/version via existing shared loader;
retain parent approved/active gate and include chapter bytes in authority digest.
No changes to frozen impact-map/gate definitions or validation policy. Add focused
CLI regression in existing test_sc_contract_directory.py. Retain initial FAIL log.

## Recovery evidence exact additions
Archive checker cut alongside candidate: candidate-tree/tools/{check_sc_binding_exposure.py,
sc_contract_directory.py}, candidate-tree/tools/release/{check_sc_unit_compliance.py,
check_science_contract_admission.sh}, candidate-tree/tests/python/{test_check_sc_binding_exposure.py,
test_sc_contract_directory.py}. These are immutable evidence snapshots, not maintained
parallel authority; canonical implementations remain tools/. Recovery reconstructs an
isolated structural/unit fixture from exact archive and Git-bound external dependencies,
not a full Cargo workspace or claimed executable reproduction. Full Rust reconstruction
requires full baseline checkout plus archived owned overlay; actual original/candidate
Rust runs and helper corruption evidence remain preserved.
Additional package-only machine evidence: artifacts/context-selection.json,
context-report-before.json, context-report-after.json and logs/rust-failure-set-comparison.json.
