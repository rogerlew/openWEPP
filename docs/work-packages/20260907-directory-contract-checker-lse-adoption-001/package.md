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
