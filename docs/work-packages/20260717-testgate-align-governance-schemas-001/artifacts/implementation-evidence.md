# Implementation Evidence

Evidence class: `Ran` and `Static`

## Governance Alignment

Static: root, crate, test, work-package, ExecPlan, prompt, kernel-preparation,
mechanical-refactor, module-test, CQR, local-CI, and Rust-scientific guidance
now delegate gate timing, lifecycle assignment, deferral, reuse, and escalation
to `docs/standards/testing-and-gate-strategy.md`.

Static: all aligned surfaces explicitly retain the conservative full
workspace/global-CRAP implementation fallback until the mechanical planner,
executor, and receipt path complete shadow acceptance and cutover. This package
therefore changes authority consistency without prematurely reducing executable
gates.

Static: ADR-0021 now distinguishes affected eligible-surface increment CRAP
from critical/campaign/release global CRAP. Its 90% science tier, 85% glue tier,
75% per-function floor, CRAP threshold 30, symbol taxonomy, exception
discipline, and empty-actionable-set objective remain unchanged.

Static: the correctness-authority model retains affected non-deferrable A0,
A1, and A3 gates and the separate execution-integrity/scientific-outcome axes.
It now distinguishes the canonical target lifecycle from the current
conservative release-workflow implementation.

## Machine Contracts

Static: `gate-policy/v1/` contains strict JSON Schema Draft 2020-12 contracts
for:

- non-Cargo impact maps and critical unknown-path fallback;
- intent/terminal gate plans with typed nodes, argument arrays, prerequisites,
  closed executor kinds, inventory, retry, artifact, identity, and typed
  acceptance-predicate contracts;
- unsigned receipts with exact plan/execution/root identities, DAG capture,
  inventories, attempts, authority outcomes, artifacts, mutation checks, and
  unauthenticated provenance claims;
- nonrecursive attestation envelopes with receipt/artifact subjects, closed
  trusted issuer classes, provenance, and offline-verifiable signature bundle;
- append-only campaign ledgers with closed lifecycle/obligation states,
  authorization events, receipt-envelope pairs, and backstop state; and
- target-bound assurance impact records with multi-axis currency, immutable
  entries, lifecycle authority, and no bare target-free `CURRENT` state.

Static: Git object IDs accept SHA-1 or SHA-256 repository formats, while
content identities remain SHA-256. The production impact map binds the exact
canonical strategy digest and is explicitly `SCHEMA_ONLY_NONBLOCKING`.

## Focused Execution

Ran: all JSON files parsed successfully with `jq empty`.

Ran: `cargo nextest run --test testgate_align_authority_contract` passed 6/6
after the guard first exposed and drove correction of two missing canonical
guidance pointers.

Ran: targeted Clippy completed successfully with warnings denied for
`testgate_align_authority_contract`.

Ran: the initial `cargo fmt --check` identified formatting drift only in the
new integration test. `cargo fmt` corrected it, and the immediate
`cargo fmt --check` rerun passed.

## Review Remediation

Ran: two independent read-only reviews returned `HOLD`, with four governance
findings and seven schema/security findings. Every finding was accepted.

Static: governance remediation removed the generic hard-fail risk-acceptance
escape, aligned three missed prospective guides, corrected the remaining
mechanical-refactor universal-gate phrase, expanded instruction-chain evidence,
and added negative source assertions against those contradictions.

Static: schema remediation added missing planner/node identities and rules,
governed zero-work plans, complete receipt DAG-node snapshots, canonical
execution/admission/scientific outcome shapes, result/count/inventory/mutation
consistency, typed nonduplicated receipt subjects, discriminated campaign and
assurance events, anchored backstop/certification states, and fold-aware
assurance currency constraints. Predicate branches and legacy adapters are now
discriminated.

Ran: negative fixtures are now one-mutation descriptors derived from their
positive fixture. The contract checks the intended instance and schema error
paths. The remediated focused suite passes `7/7` (run ID
`52a99094-e807-4622-a7b2-6642bdd12314`).

## Terminal Verification Remediation

Ran: the first terminal-verification pair returned `HOLD`. Governance found one
remaining unconditional watershed closure sentence. Schema verification found
cross-field PASS/certification contradictions, missing assurance request and
transfer-event operands, an unrepresentable empty assurance fold, a path-unsafe
matcher, and incomplete byte binding for untracked deliverables.

Static: the second remediation aligns both remaining watershed sentences and
guards their legacy phrases. Impact-map matchers are discriminated into safe
path, safe glob, and identifier shapes. Gate-plan assurance impacts now carry
both request axes. Assurance records admit the canonical empty fold and carry
typed campaign/release transfer, target-head-change, and role-revocation events;
current transfer requires the applicable event.

Static: locally expressible campaign contradictions now fail schema validation,
including receipt-free PASS transitions and a `CURRENT` backstop with ten or
more head advances. Cross-field receipt and certification equalities that JSON
Schema cannot express are explicitly checked by the source-level semantic
guard, with adversarial structurally valid mutations proving rejection.

Ran: after splitting the expanded adversarial test to satisfy warnings-denied
Clippy, the second-remediation focused suite passed `9/9`, run ID
`1e22b97a-67c8-4c72-9a63-62c08dfb2caf`.
