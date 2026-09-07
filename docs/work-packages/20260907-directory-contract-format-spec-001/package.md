# Directory-based science contract format specification
Status: complete (specification only)
Execution mode: package-end-to-end
Implementation intent: prospective format specification and bounded guidance integration.
Base: d8249849d6e015070818be7caf6f8caa75485098 on main.

## Objective and authorization
Owner's 2026-09-07 request: start directory-based contracts with a specification,
completed as a work-package. Define an implementable document-set format, safe
reading routes, checker requirements and LSE-first migration acceptance.
This is a complete specification package, not an implementation/migration package.
No SC contract edits, scientific amendments, checker implementation, production
or test edits, experiments, activation, historical-package edits, or network work.
The previous administrative package remains closed. Preserve equations, IDs,
qualifiers, provenance, guards, A0/A1/A3, real-consumer and independent closure
obligations. Current whole-contract reading remains until verified adoption.
Artifact shape belongs in the existing schema/procedure; no new ADR required.

## Exact intended write set
- docs/specifications/science-contract-directory-format.md
- docs/specifications/science-contract-spec.md
- docs/specifications/science-contract-authoring-procedure.md
- docs/specifications/science-contracts/AGENTS.md
- docs/specifications/science-contracts/kernel-process-contract-profile.md
- docs/prompt_templates/required-reading-map-template.md
- docs/work-packages/README.md
- docs/work-packages/active.md
Inside this package only:
- package.md
- prompts/active/execute.md
- prompts/archived/README.md
- artifacts/required-reading-map.md
- artifacts/worker-handoff.md
- artifacts/change-map.md
- artifacts/gate-results.md
- artifacts/line-count-governance.md
- artifacts/finding-disposition.md
- artifacts/review_agent_a.md
- artifacts/review_agent_b.md
- artifacts/verification_agent_a.md
- artifacts/verification_agent_b.md
- artifacts/final-disposition.md

## Frozen acceptance
1. Specify entry metadata, exact membership, one coherent revision, mechanism-complete
   chapters, task/role applicability and explicit direct section dependencies.
2. Preserve unique canonical definitions, full schema/profile coverage, qualifiers,
   citations and supersession proof; layout cannot change scientific authority.
3. Specify BEI extension, definitions versus mentions, anchors, safe paths,
   checker verdicts, legacy compatibility and positive/negative conformance cases.
4. Specify LSE-first preservation audit, fresh-agent tasks, dependency-inclusive
   comparable reading bytes, history/activation preservation and consumer-test
   reconciliation. Do not claim migration, checker support or savings executed.
5. Reconcile schema, guide, procedure, profile and reading template prospectively;
   existing reading requirements remain binding pending verified adoption.
6. Run Markdown links/anchors, exact-write-set/diff checks, existing checker Python
   regressions and strict LSE BEI baseline; disclose inherited limitations.
7. Dual independent reviews, explicit findings disposition, actual fixes and focused
   re-review, corrected freeze, dual independent verification; no blocking findings.
8. Scoped local commits and truthful catalog, handoff and disposition. Preserve
   unrelated untracked $pkg/ and tmp/. No push authorized by this request.

## Dependencies and steps
Existing dependencies bind base above and sections in required-reading-map.md.
New/changed documents bind the stable substantive review commit; changed authority
requires impact review, never silent acceptance drift.
1. Commit scaffold before substantive edits.
2. Author format and narrow integration pointers.
3. Run .venv/bin/python -m pytest -q tests/python/test_check_sc_binding_exposure.py.
4. Run .venv/bin/python tools/check_sc_binding_exposure.py --strict
   docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md.
5. Check Markdown links/anchors, git diff --check, exact paths and protected bytes.
6. Commit stable cut; dual reviews, fix/re-review; freeze corrected cut.
7. Dual verification, terminal reconciliation and local final commit.

## Validation and security classification
Substantive governance specification, not editorial. Testing strategy sections
6.2, 8-10 and 18 select documentation/schema and focused existing tooling checks.
No effective production authority, executable behavior, suite policy, custody or
runtime input changes; no Rust/full workspace or science experiments selected.
Specify path/symlink safety and ambiguous-definition failures; no new executor.
Independent reviewers must challenge this classification and all gate legitimacy.

## Independent assignments
This package explicitly authorizes subagent spawning/delegation for two independent
reviewers and two independent verifiers under common governance. Non-forked narrow
inputs: common/role instructions, package/handoff, changed sources and primary evidence.
Reviewer A: correctness/authority/precedence/self-waiver, high.
Reviewer B: QA/usability/security/testability, medium.
Then verifier A: specification/routes; verifier B: commands/exact diff, medium.
Each writes only its assigned named artifact, never source/configuration.
Effective runtime settings UNOBSERVED without metadata. No heavy runner selected.

## Progress, discoveries, decisions and outcomes
Maintained in artifacts/worker-handoff.md, the sole continuation view.

## Recovery
Compare base/stable cuts and exact diff before continuation. Never reset unrelated
state. Reopen accepted specification findings; actual migration/checker implementation
requires a separately authorized package.
