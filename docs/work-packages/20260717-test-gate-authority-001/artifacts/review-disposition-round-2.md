# Round-2 Review Disposition

Evidence class: `Static`

Reviewers C and D independently returned `HOLD`. All 17 findings are accepted
and remediated in the authority or implementation handoff. The campaign-scoped
decision remains; remediation prevents focused execution and evidence reuse
from becoming correctness, security, or auditability loopholes.

## Reviewer C

### C-001 — Test changes and affected CRAP

**Disposition:** Accepted and remediated.

The global trigger now uses closed coverage-loss/unknown reason codes rather
than “materially changing tests.” Additive and bounded test edits remain
affected-scope when contribution maps prove no out-of-closure loss. Affected
CRAP expands to every mechanically known covering test; uncertainty escalates
to package/reverse-dependent and then global measurement. Four explicit
handoff fixtures cover additive, preserved, reducing, and unknown cases.

### C-002 — Nonblocking authority outcomes

**Disposition:** Accepted and remediated.

Section 5.1 separates execution integrity, scientific outcome, investigation
disposition, and prospective blocking promotion. Valid A2/A4/A5/A6 divergence
creates visible investigation evidence; incomplete execution still fails the
gate; unpromoted divergence does not become false pass or automatic failure.

### C-003 — A0 admission

**Disposition:** Accepted and remediated.

Every kernel/process increment now has a non-deferrable A0 admission check
mapping affected behavior to current canonical contract, index, and obligation
bindings. Missing, ambiguous, provisional, or stale authority holds regardless
of broader test results. The handoff requires positive and negative fixtures.

### C-004 — Assurance under-selection

**Disposition:** Accepted and remediated.

Campaign discovery covers every registered report. Release inclusion is derived
from exact public/catalog/snapshot/export/vendoring/distribution inventories,
and that set must equal current release-transfer identities. Attempted omission
and complete historical exclusion are required fixtures.

### C-005 — Certified-head persistence cycle

**Disposition:** Accepted and remediated with D2-003.

Section 11.1 defines an exact source subject, protected non-source Git evidence
ref, protected-CI attestation bundle, two-phase finalization, atomic compare-and-
swap, crash/idempotency behavior, retention, and fresh-clone verification.

### C-006 — Concurrent increments

**Disposition:** Accepted and remediated with D2-004.

Admissions bind expected parent head and predecessor ledger. Concurrent work is
permitted, but advancement requires exact-current-head terminal replanning,
source ancestry, successful gates, and ledger compare-and-swap. Conflict,
supersession, abandonment, and newly selected obligations are explicit.

### C-007 — Cutover criteria

**Disposition:** Accepted and remediated with D2-008.

The authority fixes the replay corpus/observation minimum, zero-miss safety
criteria, deterministic replay, mapping/inventory requirements, planner p95,
matched median/p95 friction targets, stable status contexts, migration order,
and automatic rollback triggers.

### C-008 — Premature completion

**Disposition:** Accepted and remediated.

The package and README are reopened, the first final disposition is marked
superseded, the initial gate record declares itself historical, and the catalog
no longer claims current completion. Closure awaits renewed verification.

## Reviewer D

### D2-001 — Evidence authentication

**Disposition:** Accepted and remediated.

Receipts now use closed local, repository-reviewed, and protected-CI trust
classes. Release evidence binds issuer/repository/ref/workflow/runner/attempt
and offline attestation. The authority defines principal/role authorization,
rotation/revocation, replay/wrong-target rejection, and trust-escalation tests.

### D2-002 — Hermetic reuse

**Disposition:** Accepted and remediated.

Reuse defaults to `NON_REUSABLE`; `SAME_EXECUTION` and `HERMETIC_CONTENT` are
closed alternatives. Content reuse requires filesystem/environment/tool/network/
clock/random confinement and manifests all observable Cargo, build, data,
workflow, and system inputs. Undeclared access invalidates evidence.

### D2-003 — Certificate storage

**Disposition:** Accepted and remediated with C-005.

The selected protected Git evidence-ref model distinguishes source subject,
evidence commit, ledger fold, and certificate, with certificate calculated last
and excluded from its own identity.

### D2-004 — Ledger reduction and backstop

**Disposition:** Accepted and remediated.

The ledger now has immutable event transitions, deterministic precedence,
idempotent ingestion, exact-predecessor compare-and-swap, concurrent admission
rules, and explicit replan behavior. Backstop age/count anchors to a protected-
CI ancestor execution and has exact `CURRENT`, `DUE`, and `OVERDUE` semantics.

### D2-005 — Gate DAG and acceptance algebra

**Disposition:** Accepted and remediated.

The gate-definition ID is separate from content-derived node ID. Matrix/shard
identity, namespaces, acyclicity, prerequisites, and artifact uniqueness are
required. Acceptance uses a closed predicate algebra. Aggregate precedence is
`INVALID`, `FAIL`, `BLOCKED`, `PASS_WITH_RETRY`, `PASS`, preserving causal
failure and retry debt.

### D2-006 — Git and Cargo normalization

**Disposition:** Accepted and remediated.

Section 8 defines raw NUL-delimited rename-disabled Git change sets; separate
index/worktree/untracked handling; executed-source identity; and rejected
unsupported states. Cargo graphs use pinned isolated base/head snapshots,
locked/offline metadata, supported target/feature/dependency-kind union, and a
normalized graph bound to authority.

### D2-007 — Assurance target and fold

**Disposition:** Accepted and remediated.

All assurance axes bind exact report realization, policy/watch generation,
campaign/head, and release target. Request and currency are separate. Immutable
entry events have deterministic dominance, later-impact reset, withdrawal and
revocation behavior, and exact principal/role authorization.

### D2-008 — CI aggregate and cutover

**Disposition:** Accepted and remediated with C-007.

Stable planner and aggregate execution contexts are distinct. Branch protection
requires the aggregate; missing/canceled/different inventory fails closed. The
authority fixes dual-required migration, provider-side evidence, scorecard, and
rollback behavior.

### D2-009 — Primary references

**Disposition:** Accepted and remediated.

The standard and research basis now cite RFC 7493/8785, Git status/diff/index,
Cargo metadata/resolver/features/configuration, SLSA v1.2 provenance, and GitHub
artifact attestations in addition to the existing primary sources.

## Remediation-Verification Residuals

The first remediation verification remained `HOLD`. All residuals are accepted
and corrected:

- **Outcome exhaustiveness:** Section 5.1 now gives a complete A0–A6 table.
  A1/A3 require exact conformance; nonblocking divergence/inconclusive results
  open investigation; `NOT_EVALUATED` never satisfies a selected suite.
- **Campaign-wide compare-and-swap:** one mutable campaign `head` ref and one
  immutable per-subject alias are created/updated in a single atomic Git ref
  transaction, preventing concurrent distinct subjects from both winning one
  predecessor.
- **Release reuse:** ADR-0039 and Section 12.3 now require release-accepted
  `PROTECTED_CI` trust and `HERMETIC_CONTENT`; default non-reusable evidence
  reruns.
- **Obligation transitions:** `SUPERSEDED` is a defined state, and retry,
  blocker-resolution, invalidation/replan, fail, and block transitions are
  closed.
- **Assurance entry reduction:** entry states and transitions are closed;
  refresh completion binds exact impact IDs and evidence; target folding is
  deterministic.
- **Schema consistency:** rename is delete plus add everywhere; prerequisites
  reference node IDs; node ID hashes the complete canonical node payload.
- **Bootstrap transition:** `LEGACY_UNVERIFIED` now has bootstrap-only creation
  and can clear only through adopted replan/rerun or atomic named replacement;
  direct pass or trust promotion is forbidden and a certification fixture is
  required.

## Current Result

All round-2 review findings are resolved in the documentation candidate. This
is not closure: each reviewer must verify its disposition, two terminal
verifiers must assess the exact remediated tree, and final documentation gates
must pass before the package returns to `EXECUTED-COMPLETE`.
