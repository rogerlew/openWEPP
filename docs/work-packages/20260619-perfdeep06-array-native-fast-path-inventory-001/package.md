# PERFDEEP06 - Array-Native Fast-Path Frame Inventory and Execution Plan

Status: executed 2026-06-19. Disposition: `READY-FOR-PERFDEEP07`.

Package type: performance architecture / ADR-0025 Stage-3 planning gate.

## Objective

Produce an execution-ready array-native fast-path plan before the next
implementation package. Enumerate the H2637 hot-loop working set, publication
operands, persistent lane state, fixed arrays, borrowed forcing, phase-owned
outputs, guard/diagnostic needs, direct-frame phase APIs, layout/type-size risks,
and the follow-on package sequence for direct-frame ports.

This package must decide how the next implementation package removes
`HillslopeKernelRequest`, `KernelWritebackPayload`, `HillslopeWritebackSurface`,
`BoundarySymbol`, `BoundaryValue`, `SymbolRegistry::id_of`, logical fallback
reads, and dense/logical refresh/flush from the migrated normal success path.
It must not implement that production port or activate any opt-in by default.

## Rationale

PERFDEEP05 removed the measured PERFDEEP04 full-sync hotspot and preserved H2637
identity, but the opt-in endpoint still measured `911.11 s` against the
`669.97 s` activation reference and the final default-disabled `701.95 s`
comparison. The default-disabled path is itself a persistent regression:
PERFDEEP05 default-disabled is about `+4.7%` versus `669.97 s`, and PERFDEEP03
default-disabled had already measured in the `697-708 s` band. The accumulated
opt-in plumbing therefore taxes `main` even when the island is off. The named
suspect is always-on dense-first resolution and compatibility plumbing, not
default activation of the failed opt-ins.

The remaining opt-in profile is dominated by dense-edge compatibility costs:
cached daily refresh, logical dense writeback apply, `SymbolRegistry::id_of`,
and dirty flush. The next package must treat zero-cost-when-disabled as a P0
gate before expanding the island: either remove/bypass the dense-first tax on
the default path or stop with `HOLD`/`NO-GO`.

ADR-0025 and the ratified runtime specification now bind the next cut away from
another seam-shaving package. PERFDEEP06 is the planning gate for the complete
direct-frame fast path: identify the typed fields and APIs required to port a
complete migrated phase chain without symbol maps or writeback payloads in the
per-OFE-day loop.

## Scope

In scope:

- map the H2637 hot-loop working set into a proposed typed
  `HillslopeDayFrame`/projection schema;
- classify every required value as lane-persistent, start-of-day seed, fixed
  hourly array, layer struct-of-arrays field, borrowed forcing, phase-local
  output, publication operand, diagnostic-only, replay-only, or true I/O edge;
- define direct-frame phase API shapes for the next hydrology fast-path package;
- map current publication/output operands to typed projection sources and name
  parity fixtures needed before logical hot-path deletion;
- record layout/type-size and allocation risks required by ADR-0025 Amendment 1;
- prove statically which map/symbol/writeback mechanisms must be absent from the
  migrated normal success path;
- define a zero-cost-when-disabled gate for the current default path so
  PERFDEEP02/03/05 plumbing does not continue to tax `main` when opt-ins are
  off;
- produce the follow-on package sequence, starting with the next direct-frame
  implementation package;
- update package artifacts, roadmap/catalog status, and review/verification
  disposition.

Out of scope:

- production Rust implementation of the direct-frame phase path;
- default activation of PERFDEEP02/03/05 opt-ins or any new fast path;
- physics/numeric formula changes;
- output schema changes;
- deleting logical/indexed surfaces globally;
- broad hydrology, erosion, growth, or closure-diagnostics rewrites;
- another package whose only objective is to optimize the current
  dense/logical compatibility edge.

## Required Reading

Core:

- `AGENTS.md`
- `docs/codex_exec_plans.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/package.md`
- `docs/architecture/array-native-runtime-specification.md`
- `docs/decisions/0025-array-native-hillslope-day-frame.md`
- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/package.md`
- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05_disposition.md`
- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05-profile.md`

Conditional:

- `crates/AGENTS.md` before any Rust crate edit; the default PERFDEEP06 write
  set is docs-only.
- `docs/specifications/science-contracts/AGENTS.md` before any
  kernel-authority or runtime-projection edit that controls kernel branches.
- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` if execution discovers a
  canonical contract amendment is required.
- `tests/AGENTS.md` before adding or editing tests.

On demand:

- `docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/package.md`
- `docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/artifacts/perfdeep03_disposition.md`
- `docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/artifacts/perfdeep04-profile-results.md`
- `docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/artifacts/perfdeep04-next-package-recommendation.md`
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-runner/src/hillslope/**`
- output schema crates under `crates/openwepp-hillslope-output/src/**`

## Dependencies

- ADR-0025 and `docs/architecture/array-native-runtime-specification.md` are
  binding design authority.
- PERFDEEP05 is the immediate empirical input; this package must start from its
  profile and no-go disposition.
- The current codebase remains the static source of truth for working-set and
  API inventory.

## Intended Write Set

- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only for small
  consistency corrections discovered during planning.

Code files are read-only for this package unless `package.md` is amended before
implementation with a bounded write set, contract-first gates, and review of the
new acceptance criteria.

## Phase Plan

1. Populate the required-reading map and owned-file manifest. Confirm the
   package is executing under ADR-0025 Stage 3 and not under the older
   lane-dense edge-cost-removal recommendation.
2. Inventory the H2637 hot-loop working set. Classify symbols/fields by source,
   lifetime, unit, array family, mutability, phase ownership, and whether they
   belong in the production direct frame.
3. Build the publication operand ledger. Map HBP, WAT, PASS, manifest, and WB13
   daily publication operands from today's runtime-surface reads to proposed
   typed projection fields, including anti-tautology and identity fixture needs.
4. Design direct-frame phase APIs for the first implementation package. Define
   request/context/view shapes, ownership and borrowing model, guard attribution,
   dirty/validity representation, and mixed-mode boundaries.
5. Produce the layout/type-size and allocation ledger. Include field/slot-count
   estimates, unit-wrapper layout policy, array/SoA choices, validity bitsets,
   and explicit normal-path allocation prohibitions from specification section
   4.7.
6. Produce the no-hot-loop-map proof. Name the mechanisms that must be absent
   from the migrated success path, and classify any remaining logical/indexed
   use as I/O, replay, diagnostic, or non-migrated-boundary only.
7. Author the follow-on package sequence. The first follow-on must be a direct
   `&mut HillslopeDayFrame` hydrology fast-path implementation package with
   shadow identity, endpoint/RSS, and allocation/type-size gates.
8. Run documentation validation, complete dual review and dual verification,
   disposition all findings, update roadmap/catalog text, and close with a
   truthful `READY-FOR-PERFDEEP07`, `HOLD`, or `NO-GO` disposition.

Execution result: phases 1-8 were completed. PERFDEEP06 closes as
`READY-FOR-PERFDEEP07`: the next package should first eliminate or bypass the
default-disabled dense-first tax, then implement a bounded direct-frame
hydrology fast path over typed frame/view APIs, with no symbol/logical/writeback
payload machinery on the migrated success path.

## Acceptance Criteria

- `artifacts/perfdeep06-working-set-inventory.md` exists and classifies the
  H2637 hot-loop state by lifetime, unit, array shape, mutability, phase owner,
  and frame disposition.
- `artifacts/perfdeep06-publication-operand-ledger.md` maps HBP/WAT/PASS/WB13
  publication operands to typed projection sources and identifies parity and
  anti-tautology fixtures required before logical hot-path deletion.
- `artifacts/perfdeep06-direct-frame-api-plan.md` defines direct-frame API
  shapes and mixed-mode boundaries for the next implementation package.
- `artifacts/perfdeep06-layout-allocation-ledger.md` records layout/type-size
  and allocation-risk evidence sufficient for ADR-0025 Amendment 1 planning.
- `artifacts/perfdeep06-no-hot-loop-map-proof.md` states which symbol/map/
  writeback mechanisms must be absent from the migrated success path and which
  logical surfaces remain only at true boundaries.
- `artifacts/perfdeep06-follow-on-package-sequence.md` names the next package
  objective, write set, gates, and stop criteria.
- The follow-on sequence records zero-cost-when-disabled as a P0 gate using the
  PERFDEEP05 `701.95 s` default-disabled regression versus `669.97 s` as the
  current evidence. Its timing gate is predeclared: at least three clean H2637
  no-UI runs with all PERFDEEP opt-ins disabled, recording min/median/max/RSS;
  PASS requires median `<= 676.67 s` (`669.97 s + 1%` noise allowance) and
  static proof that dense/indexed/direct-frame compatibility plumbing is not
  constructed on the disabled path. Candidate and control should run on the
  same machine in the same harness/session where feasible; any candidate above
  `676.67 s` requires hard external-environment attribution and otherwise
  blocks closure.
- No production Rust implementation or default activation occurs unless the
  package is amended before edits.
- Markdown lint passes for the package and touched docs.
- Review findings are dispositioned as `accepted`, `rejected`, `deferred`, or
  `follow-up`; accepted findings are fixed and verified.
- Gate Evidence Non-Deferral is checked explicitly by both reviews and both
  verifications.

## Conservation / Output Acceptance

PERFDEEP06 is a planning package, but it touches publication design. It must
author a publication operand-lineage ledger before recommending any future
production output edit. That ledger must cover units, normalization/denominator,
area or volume basis, source authority, authoritative-vs-diagnostic status,
plausible wrong aliases, required anti-tautology fixtures, independent
reconstruction needs, and metadata/schema alignment.

## Contract-First Rule

No physics or canonical `SC-*` contract change is intended. If execution
discovers that the API plan requires changing invariant authority, guard
semantics, diagnostic attribution policy, output meaning, or process physics,
stop and amend the package before implementation. Any such amendment must follow
the contract-first sequence: canonical contract, contract-derived tests,
pre-implementation contract gate, then production edits.

## Security Impact Gate

No secrets, credentials, external network dependencies, or user data are in
scope. The package may inspect local source, docs, and existing profile
artifacts. It must not weaken fail-closed behavior, typed error handling,
validation gates, output schema contracts, or serialization safeguards.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only reviewer and verifier subagents for package
artifact review, no-hot-loop-map proof review, publication operand ledger
review, and gate-legitimacy verification. Expected outputs are compact review
or verification findings recorded in `artifacts/review_agent_a.md`,
`artifacts/review_agent_b.md`, `artifacts/verification_agent_a.md`, and
`artifacts/verification_agent_b.md`. Write access is limited to those artifact
files unless explicitly amended.

## Deliverables

- `artifacts/required-reading-map.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/perfdeep06-working-set-inventory.md`
- `artifacts/perfdeep06-publication-operand-ledger.md`
- `artifacts/perfdeep06-direct-frame-api-plan.md`
- `artifacts/perfdeep06-layout-allocation-ledger.md`
- `artifacts/perfdeep06-no-hot-loop-map-proof.md`
- `artifacts/perfdeep06-follow-on-package-sequence.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/disposition.md`
- `artifacts/worker-handoff.md`

## Autonomy

Execute end-to-end when triggered. Do not stop at a diagnostic-only inventory.
Do not ask the user for next steps unless a hard blocker prevents a truthful
`READY-FOR-PERFDEEP07`, `HOLD`, or `NO-GO` disposition. Do not activate any
runtime path by default.
