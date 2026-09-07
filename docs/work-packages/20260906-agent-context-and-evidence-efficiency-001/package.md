# Agent context and evidence efficiency

Status: IN-PROGRESS
Execution mode: package-end-to-end.
Implementation intent: agent-workflow governance simplification + bounded administrative tooling.
Base: 033dfe30073bc22aaa30aed877cc301b6745a086 on main.

## Objective and authority
Implement all six deliverables in the owner's 2026-09-06 execution request: role defaults; measured role context; conditional instructions; current-state/search; evidence identity/reuse; recoverable capture. This is an intentional prospective governance change, not editorial work. The request freezes acceptance below; subsequent rules cannot waive it.

## Protected boundaries
No production Rust or canonical SC edits, equations, constants, tolerances, solver/physics, ownership, restart, serialization, publication, guards, or fail-closed changes. Preserve contract-first, applicable A0/A1/A3, consumer proof, independent conservation reconstruction, anti-evasion, exact provenance, dual independent reviews and verifications. No network, deployment, push, branch switch, real Stage-3 experiment, planner, or general orchestration engine. Preserve unrelated untracked `$pkg/` and `tmp/`; index initially empty.

## Intended write set
Exact non-package paths are in artifacts/owned-files.json. This package owns package.md, prompts/{README.md,active/README.md,active/kickoff.md,archived/README.md}, and artifacts/{README.md,owned-files.json,required-reading-map.md,change-map.md,gate-results.md,line-count-governance.md,review_agent_a.md,review_agent_b.md,finding-disposition.md,verification_agent_a.md,verification_agent_b.md,final-disposition.md,worker-handoff.md,context-before.json,context-after.json,context-inputs.json,identity.json,role-settings.md}. Bounded test logs and compact JSON results belong under artifacts/. No other package history may be edited. No Rust tests are presently selected for editing; prospective amendment is required if needed.

## Dependencies
Initial binding revision is base above for AGENTS.md, docs/work-packages/AGENTS.md, docs/standards/AGENTS.md, docs/codex_exec_plans.md, docs/standards/testing-and-gate-strategy.md (sections 6,8-10,17-18), docs/standards/kernel-work-package-preparation.md (reading and preserved scientific obligations), docs/standards/prompt-wording-guidance.md, and active templates in the write set. The owner's request authorizes replacing these prospective instructions; acceptance below remains frozen. Record content identities for the stable review cut, and inspect changed bindings rather than silently accepting drift.

## Frozen acceptance
A: correctness high, QA medium, bounded verification low/medium, runner lowest supported suitable effort; remove reviewer config-write grants; TOML/local support verification; configured/requested/effective observations separate, UNOBSERVED where unavailable.
B: same six representative roles for administrative and science-sensitive tasks before/after, full/range bytes and recursive mandatory references, unique/repeated exposure, bootstrap/expansion/workflow-total separate; target bootstrap 32-64 KiB and shared <=16 KiB without omission. No quota claims.
C: concise root/common contracts, role-routed procedures and versioned explicit dependencies; resolve templates and conflicting entrypoints; preserve applicable scientific obligations and autonomous execution.
D: one current handoff, active index, freshness responsibility, targeted and historical search; preserve Stage-3 FAIL/HOLD/NO-GO and new existing authorization without extending it.
E: experiment, claim/evidence, publication identities with explicit membership; accountable impact matrix and all eight requested mutation classes, accepted-finding verification and exact-clean campaign/release requirements.
F: explicit capture/verify/restore outside checkout with source, distinct staged blobs, modes/symlinks/deletions/renames/untracked, executable, fixture/protocol/results, Git/build/environment/external metadata; reject unsafe/missing/corrupt/racy input; restore standalone miniature repository without original source and run restored fixture independently.
Closure: all selected checks PASS, accepted blockers fixed and focused re-reviewed, two independent reviews then two independent verifications of corrected cut, exact-diff/write-set reconciliation, catalog/handoff/disposition and local scoped commits. Missing independent checks remain unmet, never fabricated PASS.

## Steps and validation
1. Scaffold and commit before substantive edits; record baseline reading and change map.
2. Implement A-C and context report, D handoff/index, E-F bounded utility/tests. No third-party dependency added.
3. Run .venv/bin/python -m unittest discover -s tools/agents -p 'test_*.py', Python syntax compilation, TOML parsing/local strict-config check, targeted Markdown paths/anchors/template checks, git diff --check. Inspect relevant governance Rust assertions and run applicable focused tests; do not run full workspace without demonstrated impact. Security tests include malformed archive paths, symlinks, destination conflicts, missing inputs, corruption, staged/working restoration and mutation classification challenges.
4. Review stable substantive cut, disposition each finding, fix and obtain focused re-review. Freeze corrected cut; independent verifiers recheck instructions/measurement and execute capture restoration/negative tests.
5. Reconcile terminal diff, report limitations, update catalog/roadmap current view, commit scoped completion locally; no push.

## Independent assignments
Subagent authorization: this package explicitly authorizes subagent spawning/delegation to two independent reviewers and two independent verifiers, plus comparator_suite_runner if heavy commands become applicable. Agents get non-forked narrow inputs. Review A: authority, precedence, impact/self-waiver. Review B: usability, context, config, security/restoration. Verify A: rules/templates/context/claims. Verify B: commands, independent restoration, negatives, exact diff. Each may write only its assigned artifacts/review_agent_[ab].md or verification_agent_[ab].md. Runner writes declared compact results/logs only. Parent fixes findings. No heavy run presently selected.

## Recovery and handoff
Use artifacts/worker-handoff.md as the sole continuation view. Package executor updates it after meaningful changes and binds source/evidence identities excluding its own hash. Partial failures retain evidence and never overwrite existing captures. Retention owner: repository owner, through disposition and explicit owner audit release; no automatic deletion.

## Progress
- [x] Intake: actual HEAD/index/dirty state and applicable guidance inspected.
- [ ] Six deliverables, validation, review, verification and local delivery.

## Surprises & Discoveries
Registered reviews request xhigh and grant configuration edits. Historical catalog is nearly 6,000 lines; kernel bootstrap currently requires it in full. Current Stage-3 successor already exists, with queued handoff/disposition.

## Decision Log
2026-09-07: before Rust test edits, add only
tests/integration/advisory_linter_authority_contract.rs for narrow administrative
impact-map document-structure reconciliation. Preserve all assertions and atomic
bindings; increase stale expected WAT5 count 22 to existing 27, with independent
review. No production Rust or canonical science change. This uses the owner's
explicit administrative-test exception; selected checks must all pass.
2026-09-07: add tools/release/authority-policy/impact-map.json prospectively,
only its live policy_sha256 binding. Focused governance test exposed stale hash
after authorized standard change; preserve test and all historical identities.
2026-09-07: prospective write-set addition CLAUDE.md, limited to common workflow
routing so its duplicated onboarding cannot override current package guidance.
2026-09-06: preserve all acceptance from the owner; use existing administrative tooling directory and handoff, no ADR required for this owner-authorized workflow change.

## Outcomes & Retrospective
Pending execution; no completion claim.
