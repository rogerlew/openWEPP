# Independent Review Disposition

Evidence class: `Static`

Reviewer A and Reviewer B independently returned `HOLD`. Every finding was
accepted. The authority remains campaign-scoped; remediation makes the focused
path mechanically conservative rather than restoring full release-scale work
to every increment.

## Reviewer A

### A-1 — Non-deferrable A3 constitutive authority

**Disposition:** Accepted and remediated.

The family table now separates A3 constitutive authority from A4/A5
empirical/independent evidence. Increment closure mechanically derives and runs
all applicable A1 and A3 obligations, and incomplete or ambiguous process-suite
binding blocks closure.

### A-2 — Isolated workspace-member contradiction

**Disposition:** Accepted and remediated.

The bounded class now requires a six-part isolation proof and stable reason
codes. Changes to existing resolution, shared behavior, or unproven isolation
remain critical. The handoff acceptance scenario now requires this proof.

### A-3 — Non-function affected surfaces

**Disposition:** Accepted and remediated.

Increment coverage now begins with affected source items, including constants,
tables, types, traits, macros, generated/build inputs, features, and shared
definitions. Unknown expansion becomes package-wide and then global; an empty
selection for these changes is invalid. Corresponding fixtures are required.

### A-4 — Doctest and stub-scan timing

**Disposition:** Accepted and remediated.

Affected doctests and the placeholder/stub scan are increment gates when their
inputs can change. Workspace doctests and the scan are explicit campaign gates.
Their gate nodes and receipts use normal inventory and reuse contracts.

### A-5 — Campaign admission and discovered obligations

**Disposition:** Accepted and remediated.

The standard now requires an intent plan before edits and terminal exact-diff
reconciliation. Every increment belongs to a campaign, with a one-increment
standalone form. It defines admission, amendments, head chaining, rebase,
overlap, abort, supersession, and bootstrap. Newly discovered increment gates
must run; governed amendments cannot backdate deferral.

### A-6 — Assurance impact completeness

**Disposition:** Accepted and remediated.

Impact creation is mandatory. Selected reports require exact, process/domain,
and contract watches. Closed watch kinds, add/rename/delete/unknown handling,
identity, coalescing, ownership, no-material-impact disposition, and fail-closed
transfer behavior are now normative.

### A-7 — Long-campaign full-regression backstop

**Disposition:** Accepted and remediated.

Campaigns now declare a cadence no looser than 14 elapsed days or 10 merged
increments. An overdue backstop blocks further ordinary increment admission and
closure. A missed regression opens both product and selector/impact-map defects.

## Reviewer B

### B-001 — Isolated workspace-member contradiction

**Disposition:** Accepted and remediated with A-2.

The authority and handoff now distinguish additive proven isolation from a
change to existing workspace resolution or behavior.

### B-002 — Typed executable gate contract

**Disposition:** Accepted and remediated.

Plans now contain a typed, versioned per-gate DAG with stable gate ID, executor,
arguments, working directory, environment allowlist, prerequisites, expected
inventory, acceptance rule, timeout/retry/failure policy, artifacts, blocking
transition, reuse class, and identity-breaking platform fields. The standard
separates `plan_id`, `execution_key`, and runtime `receipt_id`, defines aggregate
status precedence, and confines shell workflows to named hashed adapters.

### B-003 — Reproducible roots and reuse

**Disposition:** Accepted and remediated.

Roots are canonical versioned manifests over independently recomputed transitive
input closures. Record format, stable ordering, SHA-256, RFC 8785 JSON
canonicalization, file mode, symlink, untracked-file, submodule, dirty-tree, and
environment/platform rules are explicit.

### B-004 — Deferral timing and ledger evolution

**Disposition:** Accepted and remediated with A-5.

The two-stage plan and append-only ledger lifecycle settle the implementation
choices rather than delegating them to an agent.

### B-005 — Assurance axes and deterministic watches

**Disposition:** Accepted and remediated with A-6.

Assurance state is now four independent closed axes rather than an extension of
the current single enum. The handoff explicitly scopes the schema and planner
transition.

### B-006 — Fresh campaign/release evidence

**Disposition:** Accepted and remediated.

The ADR and standard now define fresh/current for an exact execution root. A
verified campaign global receipt satisfies release when no bound input changes
and no explicit `rerun_on_release` policy applies.

### B-007 — Transition inventory and staged adoption

**Disposition:** Accepted and remediated.

The implementation handoff now names assurance authorities, schemas, catalogs,
report manifests, planner/tests, transition/export/materialization scripts,
gate runners, and workflow contexts. It requires schema-first guards, shadow
comparison, retained-campaign replay, nonblocking observation, discrepancy
disposition, blocking cutover, and conservative rollback. Existing evidence is
legacy unless independently content-verified.

## Disposition Result

All review findings are resolved in the documentation candidate. Final closure
still depends on two independent terminal verifiers reading the remediated tree
and the final documentation gates.
