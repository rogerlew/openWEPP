# Prospective Stage 3 mechanism experiments

Status: EXECUTED HOLD — results complete; dual review and verification accepted.
Execution mode: package-end-to-end. Owner authorization: prompts/active/kickoff.md.
Implementation intent: controlled mechanism evaluation + minimal measurement/coverage correction + re-architecture decision support.

## Purpose and scope

Freeze A from checkpoint e89befa4678eadec039b3e7f7fe0a176af8e9dc5 plus common observation-only harness; compare F = A + typed feed-forward carrier, then R = A + same-sweep component-temperature dependency replay. Local source, builds, serial controlled processes, evidence and local commits only. Retained production remains HOLD. No production promotion, solver redesign, network, push, named branch changes, or historical reconstruction.

The kickoff is the full executable specification and binding sampling/decision protocol. Historical FAIL/HOLD conclusions remain historical; the owner's new authorization admits these prospective experiments without requiring historical binary/index custody.

## Progress

- [x] 2026-09-08 PDT: owner lifted the pause through
  `prompts/active/resume-20260908.md`; the original pause remains historical.
- [x] Bounded resumption reconciliation started from A-source-04,
  F-source-05, and R-source-05 without rebasing the kits.

- [x] 2026-09-06 PDT: kickoff and core guidance read; no equivalent package; checkout equals accepted checkpoint, branch main, existing untracked tmp/ preserved.
- [x] Resolve exact source paths and prospective authority/test amendments.
- [x] Dual independent protocol/authority and concrete F/R test/API reviews before behavior implementation (named artifacts retain reviewed hashes).
- [x] Common harness frozen as A commit2b56e6ebc9c8e8073fb68b89626d4d4e2b4602e3; source reconstruction10,131rows passes; release real one-OFE runs and independent trace audit execute. Full admission and exclusive measurement identity remain open below.
- [ ] Implement/admit/measure F; independently implement/admit/measure R.
- [ ] Full applicable correctness, parity, architecture handoff, dual reviews and terminal verifications.
- [ ] Reconcile terminal diff, findings, catalog and disposition.
- [x] 2026-09-06 PDT: explicit pause checkpoint recorded after A/F/R correctness/parity work and before comparative timing/memory; see `artifacts/pause-checkpoint.md`.

## Write set and sequencing

Package-owned documents, scripts, raw evidence, source snapshots and patches live in this package. Catalog: docs/work-packages/README.md. Roadmap: docs/planning/snow-surface-energy-balance-roadmap.md. Prior package conclusions are preserved with additive forward references.

Prospective common harness paths: crates/openwepp-runner/src/hillslope/03_tests.rs; crates/openwepp-runner/src/hillslope/tests03/stage3_runner_qualification.rs; new crates/openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs. Treatment and authority exact paths must be entered in artifacts/required-reading-map.md before their edits. Experimental Rust is isolated in detached source cuts and retained as complete reproducible deltas; no hidden retained production selector.

Contract-first sequence: applicable canonical experimental authority; derived tests; dual review and preimplementation evidence; behavior changes. No changes to equations, domains, tolerances, stencils, stopping/adaptive/event policy, ownership, restart/publication or fail-closed guards.

## Delegation

Subagent authorization: this package explicitly authorizes subagent spawning/delegation to source investigators, two independent reviewers, two independent verifiers and comparator_suite_runner for bounded source/protocol review, implementation review, verification and frozen execution. Investigators are read-only; reviewers/verifiers may write only assigned artifacts; comparator writes execution results, never source. Implementation delegation requires exact ownership assignment. Subagent requirement: REQUIRED comparator_suite_runner for heavy builds/gates/comparisons; one genuine service failure permits documented local fallback. All measurement windows are serial without other heavy jobs.

## Validation and acceptance

Use docs/standards/testing-and-gate-strategy.md directly. Each critical experimental runtime cut requires full correctness strength plus touched authority, guards, ownership/error/rollback/restart, LSE/orchestrator/runner, real consumer and output parity, independent closure reconstruction, formatting/scoped warnings-denied Clippy, applicable anti-evasion and line-count checks. Historical expected-red seams are recorded, never suppressed. No mandatory gate may be relabeled deferred to close the package.

Prospective corrected-full configuration: `cargo nextest run --release --workspace
--profile full`, CARGO_PROFILE_RELEASE_LTO=false, RUST_MIN_STACK=67108864, offline,
no external wrapper timeout. Canonical full filters/groups and case inventory
remain unchanged; retain each variant's explicit inventory. ReviewB authority
assessment permits optimized full coverage (strategy requires strength, not
default-debug specifically). Focused default-debug affected LSE/orchestrator/
runner/vegetation/kernel invariant tests remain required. This does not relabel
the interrupted initial default-debug full, claim debug-full equivalence, or
qualify normal thin-LTO production binaries. Ignored mechanism/consumer gates
run separately. Both independent assessments approve this variant (artifacts/
validation-posture-review-a.md and full-validation-variant-review-b.md).

Protocol freezes balanced 12 fresh-process timing pairs (two warmups per executable), six separate memory pairs, three processes of ten teardown runs per arm, and three scale pairs at 10/19 OFEs for competitive candidates. At most one timing extension to 24 total pairs and one performance refinement. Full detail, stop rules and required delivery are in kickoff. Measurement admission and protocol require two independent reviews before comparison.

Use exact output/control identity except intended lower-level work counts; independent operand/source/units and closure evidence, explicit volatile field allowlist. Rust >=2000 lines requires rationale/split intent; nonexempt >=3000 requires refactor. Security: no secrets, remote actions or weakened source/authority guards. External authority binding edits trigger both anti-evasion commands.

## Decisions, discoveries and outcome

Decision 2026-09-06: new identities, no historical-custody recovery. Both mechanism source seams currently absent; accepted historical replay was nonzero. New treatment results are pending, not negative measurements. No larger architecture implementation is authorized here. Required handoff ranks at most three directions with source entry points, authority, invariants, comparator and decisive prototype/kill criterion.
