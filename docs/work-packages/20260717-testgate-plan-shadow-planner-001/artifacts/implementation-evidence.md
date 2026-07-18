# Implementation Evidence

Evidence class: `Ran` and `Static`

## Delivered Shadow Behavior

- `crates/openwepp-gate-planner/src/canonical.rs` rejects duplicate JSON keys
  and floating-point policy values, orders object names by UTF-16 code units,
  emits compact RFC-8785-compatible v1 bytes, and derives SHA-256 identities
  with self-ID exclusion.
- `repository.rs` observes rename-disabled raw Git changes, binds index,
  worktree-content, and untracked-content identities separately, rejects sparse,
  unmerged, intent-to-add, submodule, unsupported-mode, and non-UTF-8 states,
  and unions locked/offline base and head Cargo graphs before conservative
  reverse-dependency expansion.
- `policy.rs` schema-validates and cross-checks the SHADOW impact map and typed
  gate registry, binds the current policy digest, rejects missing/cyclic gate
  definitions, and implements component-safe path matching.
- `planner.rs` selects monotonic risk, escalates unknown inputs to `CRITICAL`,
  enumerates exact Nextest inventories without running tests, creates stable
  topological typed nodes, derives node/execution/plan identities, binds
  execution/authority/documentation/assurance roots, schema-validates output,
  and independently reconstructs terminal plans before monotonic
  reconciliation or package-to-workspace supersession.
- `verifier.rs` reconstructs plan, receipt, source/root, DAG, inventory,
  attempt/retry, aggregate result, authority outcome, artifact, and
  source-mutation claims. Unsigned receipts remain `LOCAL_UNTRUSTED` regardless
  of their claimed class. Envelope verification requires exact subjects,
  artifacts, provenance, bundle digest, current issuer/revocation authority,
  and a caller-supplied cryptographic attestation verifier.
- `ledger.rs` checks derived IDs, predecessor/CAS ancestry, immutable event
  chains, closed obligation transitions, closure/certification state, exact
  assurance targets, replacement binding, aggregate impact, and transfer
  currency.
- `main.rs` exposes plan, receipt, ledger, and assurance verification commands.
  On Linux it writes canonical plans through stable-directory-descriptor atomic
  replacement and never executes a planned gate.

## Security And Protected Boundaries

Static: planned commands remain arrays from a schema-checked closed registry;
there is no shell evaluation or gate executor. Paths are repository-relative
and artifact reads reject absolute, empty, dot, and parent components. No
credential, network, CI workflow, protected ref, evidence publication,
campaign mutation, assurance mutation, or gate-reduction path was added.

Static: the attestation interface cannot infer trust from an envelope digest.
Protected trust requires external cryptographic verification plus exact current
issuer, trust-root, policy-generation, and revocation-generation equality.
Hermetic reuse separately requires exact roots, execution identity, protected
attestation, and confinement proof; any `NON_REUSABLE` node rejects reuse.

## Superseded Intent Diagnostic

Ran: an earlier implementation loop planned the dirty tree from frozen base
`0873bdae960f7f8c76401845acb476750fdd020e` without executing tests. It emitted
plan `112a15c4cc5cf28fb634cfc0662497f495b311556797dfa1c72208c7b738a086`
at `artifacts/intent-plan.json`; file SHA-256 is
`e9905a57a25695d47fb2dca2ff1f8c0714c8a63632d00d39f98cea833407da2b`.
That payload predates final authorized-path, execution-context, and terminal
reconstruction remediation. Its identifiers are retained here as diagnostic
provenance, but the obsolete 156 KiB payload is intentionally omitted. The
terminal gate results are authoritative.
