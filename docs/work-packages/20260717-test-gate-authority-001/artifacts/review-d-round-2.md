# Adversarial Review D, Round 2 — Determinism And Implementation Safety

Evidence class: `Static` plus `Ran` repository/path inspection

Disposition: `HOLD`

## Independence And Scope

I reviewed the current ADR, canonical testing/gate standard, package contract,
first-round reviews and disposition, terminal verifications, implementation
handoff, current release workflow, assurance planner, assurance receipt code,
and release procedure. I did not read or coordinate with round-2 reviewer C.
I treated the earlier `PASS` dispositions as claims to challenge, not as
evidence that the mechanics were complete.

The campaign-scoped direction remains sound. The present authority is much
closer to implementable than the first draft, but it still leaves several
security and state-machine choices to the follow-up. Those choices determine
whether a release certificate can be trusted and whether two independent
implementations would select and reduce the same gates. They therefore belong
in the authority rather than being silently chosen by implementation.

## Findings

### D2-001 — Blocking — Content addressing does not establish who executed or authorized evidence

The receipt contract records hashes, tool identities, results, and artifacts,
but no issuer, trust domain, execution provenance, authenticated subject, or
authorization proof
(`docs/standards/testing-and-gate-strategy.md:587-606`). Campaign ledger
versions similarly bind an unspecified "owner authorization"
(`docs/standards/testing-and-gate-strategy.md:651-656`). A party can therefore
fabricate a passing JSON receipt and matching artifact hashes, calculate a
valid `receipt_id`, and insert both into a content-addressed ledger. The
verifier can prove internal integrity but not that the named commands ran or
that the campaign owner approved the transition. The existing assurance
publication implementation already distinguishes a receipt trust domain
(`crates/openwepp-assurance/src/v2/publication.rs:1415-1433`), so omitting an
equivalent trust boundary from the new general receipt architecture would be a
regression.

Required remedy:

- Define closed receipt trust classes, at minimum untrusted/local evidence,
  repository-reviewed evidence, and protected-CI release-eligible evidence.
- Bind issuer identity, repository, source event/ref, workflow and job identity,
  runner/image identity, attempt, and an authenticated attestation or approved
  repository event into release-eligible receipts. Content hashes remain
  necessary but are not the authentication mechanism.
- Define how offline verification establishes issuer authorization, how keys or
  identities are rotated/revoked, and which trust classes each lifecycle
  boundary accepts. A locally generated receipt must not become release-
  eligible merely by copying it into the repository.
- Replace the free-form ledger "owner authorization" with a versioned
  authorization event tied to the repository principal/role authority and the
  exact predecessor plus proposed ledger transition.
- Add forgery, wrong-repository, wrong-ref, replay, revoked-issuer, and local-to-
  release trust-escalation acceptance fixtures.

### D2-002 — Blocking — Safe receipt reuse assumes a complete input closure without constraining ambient inputs

The standard requires a "complete transitive input closure" and permits reuse
when independently recomputed roots match
(`docs/standards/testing-and-gate-strategy.md:530-585` and `:608-625`). An
environment-variable allowlist is present in the gate-node contract, but there
is no filesystem, network, clock, randomness, process, Cargo-home, Git-config,
or system-tool confinement rule (`:459-476`). A test, `build.rs`, shell adapter,
compiler wrapper, or empirical harness can observe an ignored file, `$HOME`, a
mutable registry/cache, DNS/network response, current time, or an executable
found through `PATH` without that input appearing in any root. Declaring that
an undeclared dependency later "opens an impact-map defect" does not make an
already reused receipt sound.

The current CI workflow illustrates the problem: it installs a moving `stable`
toolchain and downloads tools during the job
(`.github/workflows/release-gates.yml:90-107`), while the proposed root contract
does not say whether downloaded package bytes, action revisions, registry
contents, runner image, or install logs are bound.

Required remedy:

- Define closed reuse classes such as hermetic/content-reusable,
  same-run/same-environment reusable, and non-reusable. Default to non-reusable
  when complete observation or confinement cannot be proven.
- Require a content-reusable executor to confine readable filesystem roots,
  deny or explicitly proxy and digest network inputs, sanitize environment and
  Git/Cargo configuration, bind executable and container/image bytes rather
  than version strings alone, and declare clock/random/locale behavior.
- Make an observed access outside the declared closure `INVALID`. For tools
  that cannot be confined, require new execution at the consuming boundary.
- Define how Cargo registries, Git dependencies, `build.rs`, proc macros,
  external datasets, and generated files enter the manifest. Pin workflow
  actions and tool installation inputs before they can issue reusable
  protected-CI evidence.
- Add acceptance fixtures in which an ignored file, `$HOME` config, `PATH`
  replacement, network response, and clock value attempt to influence a gate.

### D2-003 — High — Campaign certification has an unresolved evidence-storage and self-reference cycle

Campaign closure is the exact clean commit for which all evidence is current
(`docs/standards/testing-and-gate-strategy.md:255-275`). The ledger is
append-only, content-addressed, and updated when the campaign head advances
(`:651-656`), and the terminal certification then binds the complete evidence
set. The handoff asks the follow-up to implement receipts and ledgers but does
not select their authoritative storage or finalization transaction
(`docs/work-packages/20260717-test-gate-authority-001/artifacts/implementation-handoff.md:32-65`).

If receipts, the final ledger version, or the certificate are committed into
this repository—as current work-package and assurance evidence commonly are—the
evidence commit changes the Git head after the tested clean commit. If they are
external, the authority has not defined the durable store, retention, trust, or
link from the repository/release to that store. A certificate also cannot be an
input to the identity that determines its own subject.

Required remedy:

- Distinguish `subject_source_commit`, immutable evidence generation, ledger
  generation, and certificate ID. State explicitly which is the campaign head
  and which can be committed later without changing the certificate subject.
- Select an authoritative storage model: an in-repository evidence commit that
  names a prior subject commit, a protected CI attestation/artifact store, or a
  defined hybrid. Specify retention, lookup, integrity, and release embedding.
- Define the finalization order so the terminal plan and executions bind the
  source subject, ledger reduction consumes their receipts, and the certificate
  is calculated last without self-inclusion.
- Define which later documentation/evidence-only commit may carry the
  certificate and how release tooling resolves it back to the exact certified
  source and assurance roots.
- Add a round-trip fixture that certifies a source commit, persists the evidence,
  verifies it from a fresh clone/environment, and proves that storing the
  certificate neither changes nor ambiguously retargets its subject.

### D2-004 — High — Ledger concurrency, obligation transitions, and backstop reduction remain underspecified

The campaign lifecycle has allowed top-level transitions, but obligation
statuses are only enumerated; their allowed event transitions and deterministic
fold are not defined
(`docs/standards/testing-and-gate-strategy.md:658-704`). There is also no
compare-and-swap rule when two admitted increments start from the same ledger
predecessor, no required ancestry relation between an increment base and the
campaign head, and no resolution rule for concurrently produced ledger
versions. "Owner authorization" alone cannot prevent a lost update.

The backstop algorithm compounds the ambiguity. It is calculated from the last
"current" full-regression receipt (`:699-704`), but exact-root evidence becomes
stale as soon as the next increment changes a bound input. The standard does not
say whether that stale-but-valid-at-execution receipt remains the cadence anchor,
when `DUE` becomes `OVERDUE`, which clock is authoritative, or whether the
increment at the 10-count/14-day boundary may close.

Required remedy:

- Define immutable ledger event records and a closed transition table for every
  obligation state, including `PASS -> STALE`, `DEFERRED -> PENDING`, retry after
  `FAIL`/`BLOCKED`, supersession, and idempotent duplicate receipt ingestion.
  Define the deterministic precedence/fold when several events address one
  obligation.
- Require compare-and-swap on the exact predecessor ledger digest. Define
  conflict behavior, branch/head ancestry, merge or rebase admission, and how a
  completed increment is replanned when another increment advances the campaign
  first. No last-writer-wins update is acceptable.
- Define the backstop anchor as a passing full-regression execution on a named
  ancestor head, rather than requiring it to remain current for the latest head.
  Bind an authoritative timestamp and head-advance counter.
- Give exact threshold semantics for `CURRENT`, `DUE`, and `OVERDUE`, including
  whether admission and closure are checked before or after the next head
  advance and how rebase, clock rollback, abort, and supersession affect the
  counters.
- Add concurrent-admission, stale-predecessor, failed compare-and-swap, rebase,
  exact-threshold, and clock-anomaly fixtures.

### D2-005 — High — The typed gate DAG still permits divergent or unsafe executors

The node contract has a stable `gate_id`, argument array, prerequisites,
inventory/cardinality rule, acceptance rule, retry policy, and output paths
(`docs/standards/testing-and-gate-strategy.md:430-476`). It does not distinguish
a stable gate-definition ID from a unique invocation/node ID. Consequently two
shards, target triples, feature matrices, retries, or repeated gate instances
can share an ID, collide on output paths, and make prerequisite references
ambiguous. Nor does the authority define a closed acceptance-predicate language;
"machine-evaluable acceptance rule" could become arbitrary code or a shell-like
expression that silently reintroduces execution and policy into data.

The aggregate precedence is also wrong for an ordinary failure DAG. It returns
`BLOCKED` before `FAIL` (`:478-482`). If gate A fails and dependent gate B is
therefore blocked, the whole plan renders `BLOCKED` and masks the observed
failure that caused it.

Required remedy:

- Separate versioned `gate_definition_id` from a unique, content-derived
  `node_id` that binds parameters, shard, platform, feature set, and attempt
  policy. Prerequisites reference node IDs. Require unique node IDs and artifact
  namespaces, acyclicity, stable matrix expansion, and complete prerequisite
  validation.
- Define a closed, versioned acceptance-predicate algebra—such as exit-code,
  exact/subset inventory, count, threshold, artifact-presence, and typed
  comparator predicates—with no arbitrary evaluation or shell interpolation.
  Define each executor's normalized outcome mapping.
- Reduce observed `FAIL` ahead of derivative `BLOCKED`, or report both a primary
  aggregate outcome and the complete blocked set. `INVALID` may retain highest
  precedence because integrity failure destroys the evidentiary claim.
- Define how flaky-pass outcomes reduce. A deterministic gate that fails once
  cannot become an undifferentiated `PASS` merely because a permitted retry
  succeeded.
- Add shard/matrix collision, cyclic DAG, duplicate output, failing prerequisite,
  arbitrary-expression rejection, and flaky retry fixtures.

### D2-006 — High — Git and Cargo impact inputs are not canonical enough to produce the same affected set

The planner consumes changed paths/change kinds and `cargo metadata`, then maps
reverse dependencies (`docs/standards/testing-and-gate-strategy.md:371-420`).
No command/configuration contract fixes how changes are derived. Git rename
detection is heuristic and configuration/version dependent; staged and
unstaged content may differ; intent-to-add, file-type changes, ignored inputs,
and non-UTF-8 paths are not assigned planner behavior. The dirty-root manifest
records both index and worktree state, but it does not say which state supplies
the executed source or changed-set semantics (`:539-557`).

Similarly, an unqualified `cargo metadata --format-version 1` does not settle
the target, features, dependency kinds, resolver configuration, lock/offline
behavior, or how base and head graphs are compared. Target-conditional,
build/proc-macro, dev, and optional-feature edges can therefore produce
different reverse-dependent sets under equally plausible implementations. This
also makes the isolated-workspace-member proof at `:315-331` non-reproducible.

Required remedy:

- Define one canonical changed-set algorithm. Prefer add/delete/modify/type-change
  records with rename represented as delete plus add, avoiding heuristic rename
  identity. Define base-to-index and index-to-worktree treatment, executed source
  selection, file modes, submodules, intent-to-add, ignored and untracked files,
  and rejection of unsupported path encodings.
- Define the exact Cargo metadata invocations for base and head, including
  `--locked`/offline policy, Cargo/toolchain/config identities, target matrix,
  resolver, feature sets, and included dependency kinds. State whether impact
  is the union across all release-supported configurations or a versioned
  narrower configuration.
- Bind the resulting normalized dependency graph, not raw unstable metadata
  serialization, into planning identity.
- Add target-conditional dependency, optional feature, proc-macro/build
  dependency, dev-dependency integration test, staged-versus-unstaged,
  add/delete rename, and invalid-path fixtures.

### D2-007 — High — Assurance state lacks target identity and a deterministic multi-impact reduction

The four-axis state table is an important correction, but the required record
does not bind each currency value to a campaign ID/head, release candidate,
report source root, or assurance realization (`docs/standards/testing-and-gate-strategy.md:777-797`).
A bare `CURRENT` can therefore be copied forward or interpreted against a new
head. The impact-entry rules create one entry per changed object and permit many
entries per report (`:799-835`), yet no deterministic fold defines the report's
aggregate campaign disposition when one entry is no-material-impact, another
requires refresh, and a third is unresolved. No transition resets
`REFRESH_COMPLETE` to `IMPACT_PENDING` when a later change arrives, and "blocks
both transfer axes" is inconsistent with retaining `NOT_REQUESTED` as a state.

Required remedy:

- Bind every axis record to report/source realization, campaign and exact head,
  policy/watch generation, and—when requested—exact release identity.
- Define impact-entry lifecycle events and a closed transition table, including
  later impacts after no-material-impact or refresh completion, supersession,
  withdrawn reports, and authorization revocation.
- Define a deterministic aggregate fold over all current entries. Unresolved or
  unknown impact must dominate; refresh-required must dominate authorized
  no-impact; transfer can become `CURRENT` only for the exact target after every
  blocking entry is resolved.
- Clarify whether `NOT_REQUESTED` is orthogonal to blocking or replace each
  transfer axis with separate request and currency fields.
- Bind resolution authority to exact principal/role records and immutable
  lifecycle events rather than the prose alternatives "report lead" or
  "assurance steward."
- Add multiple-entry ordering, later-change reset, new-head replay, release-
  target change, withdrawn report, and unauthorized disposition fixtures.

### D2-008 — High — CI can report planner success before required executions, and cutover criteria are unset

The standard recommends one always-reporting planner job and says it reports
`PASS` for a zero-work or focused plan
(`docs/standards/testing-and-gate-strategy.md:860-880`). A successful planner is
not a successful gate plan execution. If branch protection requires only that
stable planner context, a pull request can merge while dynamically selected
executor jobs are failed, absent, or still running. The handoff mentions
workflow/status contexts but not the external branch protection or required-
check settings that actually enforce them
(`docs/work-packages/20260717-test-gate-authority-001/artifacts/implementation-handoff.md:119-143`).
The current release procedure explicitly excludes those provider settings
(`docs/governance/openwepp-release-procedure-draft.md:43-46`).

The staged adoption also permits blocking cutover "after acceptance thresholds
are met" without defining the thresholds, retained corpus, comparison oracle,
minimum observation duration, or rollback trigger
(`docs/work-packages/20260717-test-gate-authority-001/artifacts/implementation-handoff.md:145-164`).
Those are safety policy choices, not incidental coding details.

Required remedy:

- Separate planner-generation status from one stable, always-reporting aggregate
  execution status. The aggregate becomes successful only after the admitted
  DAG and receipt verification reduce to `PASS`; cancellation, missing dynamic
  jobs, and planner/executor disagreement fail closed.
- Define stable status-context names and the migration sequence for repository
  rulesets/branch protection/merge queues. If external settings require a human
  operation, make that operation and captured evidence a cutover prerequisite.
- Define shadow acceptance numerically: zero false-negative selected
  obligations and zero unsafe receipt reuse across a named retained corpus and
  observation window; all discrepancies dispositioned; performance/false-
  positive targets measured separately and unable to waive safety misses.
- Define automatic and operator rollback triggers, the status-context behavior
  during rollback, treatment of campaigns admitted under the new schema, and
  proof that rollback restores the conservative runner without making required
  checks disappear.
- Add missing-job, canceled-job, planner-pass/executor-fail, dynamic-matrix
  failure, renamed-status, branch-protection cutover, and rollback fixtures.

### D2-009 — Medium — The reference set does not support several newly normative mechanisms

The standard invokes I-JSON, Git dirty/index semantics, authenticated evidence
reuse, and concurrent content-addressed ledger updates, but the external
reference list contains RFC 8785 and a Git ignore-pattern page only
(`docs/standards/testing-and-gate-strategy.md:1002-1018`). It does not cite the
I-JSON RFC, Git status/index/diff authorities, a software-provenance/attestation
standard, or a supply-chain model for trusted CI receipts. The research-basis
artifact likewise supports test selection and dependency execution but not
these evidence-security and transaction claims
(`docs/work-packages/20260717-test-gate-authority-001/artifacts/research-basis.md:8-53`).

Required remedy: add primary or authoritative references adjacent to the
mechanics they support, including RFC 7493 for I-JSON, Git's status porcelain
and index-format/diff documentation for dirty-state construction, Cargo's
feature/resolver and configuration authorities for dependency expansion, and a
versioned provenance/attestation authority such as SLSA provenance plus the
selected CI provider's artifact-attestation identity model. If the project
chooses an in-repository authorization model instead, cite and define that
model explicitly rather than implying that SHA-256 authenticates an issuer.

## Checks Run

Ran on the reviewed tree:

- exact line-numbered inspection of ADR-0039, the complete testing/gate standard,
  package contract, prior reviews/disposition/verifications, and implementation
  handoff;
- path and implementation inspection of `.github/workflows/release-gates.yml`,
  `crates/openwepp-assurance/src/v2/planner.rs`, assurance receipt/publication
  code, assurance schemas/transactions, and the draft release procedure; and
- repository search for status-context, branch-protection, campaign-ledger,
  receipt, and assurance-state implementations.

No Rust or workflow execution was appropriate for this read-only architecture
review. These findings concern missing authority contracts, not claims that the
deferred subsystem already exists.

## Conclusion

The authority should remain campaign-scoped and should not restore full gates
to every increment. Round 1 successfully repaired suite timing, broad risk
selection, source-item coverage, and the basic campaign/assurance model. Round
2 shows that the evidence trust boundary, hermetic reuse contract, certificate
finalization, concurrent ledger reduction, exact executable DAG semantics,
Git/Cargo normalization, assurance aggregation, and CI cutover are not yet
settled enough for the follow-up to be mechanical.

Disposition remains `HOLD` until D2-001 through D2-008 are resolved in the
authority and handoff and D2-009 is addressed with the selected mechanisms'
primary references.

## Remediation Verification

Evidence class: `Static` plus `Ran` scoped documentation checks

Verification disposition: `HOLD`

I re-read the live ADR, complete standard, implementation handoff, reopened
package/final/gate artifacts, and round-2 disposition after remediation. I also
checked the interactions with the C-lane additions for A0 admission, scientific
outcome separation, affected CRAP, registry-wide assurance discovery, concurrent
increments, certified-head persistence, and CI cutover. The amended authority
substantively resolves D2-001, D2-002, D2-008, and D2-009. D2-005 and D2-006 are
substantively resolved but retain small normative inconsistencies described
below. D2-003, D2-004, and D2-007 have residual state/transaction ambiguities
that still permit divergent implementations.

### Verified remedies

- **D2-001 — PASS.** Sections 10.2–10.3 now distinguish content integrity from
  issuer trust; define local, repository-reviewed, and protected-CI trust
  classes; bind issuer/repository/ref/workflow/runner/attempt/attestation; define
  principal-role authorization and current revocation; and reject wrong-target,
  replay, revoked-issuer, and trust promotion. The protected-CI requirement is
  also elevated into ADR-0039 Decision 13.
- **D2-002 — PASS.** Section 10.4 defaults to non-reuse, limits same-execution
  reuse, and permits content reuse only with filesystem, environment, tool,
  network, clock, randomness, locale, process, and kernel confinement. It
  explicitly covers Cargo/build/data/workflow/system inputs and invalidates
  observed undeclared access. This interacts safely with current-evidence reuse:
  a non-hermetic gate reruns rather than borrowing content currency.
- **D2-005 — PASS subject to residual R3.** The authority now separates gate
  definition and invocation IDs, requires unique artifact namespaces and an
  acyclic complete matrix, supplies a closed predicate algebra, fixes aggregate
  precedence to preserve causal failure, and exposes retry debt. It no longer
  permits arbitrary acceptance expressions.
- **D2-006 — PASS subject to residual R3.** Section 8.1 fixes raw, NUL-delimited,
  rename-disabled Git records, dirty index/worktree/untracked treatment,
  executed-source identity, unsupported-state rejection, and isolated locked/
  offline Cargo graphs over a versioned target/feature/dependency-kind matrix.
- **D2-008 — PASS.** The planner-only and aggregate execution contexts are now
  distinct; missing/canceled jobs and inventory mismatch fail the required
  aggregate. The scorecard has a fixed observation/corpus floor, zero safety-
  miss requirements, deterministic replay, measured friction criteria,
  dual-required migration, provider-side evidence, and fail-closed rollback.
- **D2-009 — PASS.** The standard and research basis now cite RFC 7493/8785,
  Git status/diff/index, Cargo metadata/resolver/features/configuration, SLSA
  provenance, and GitHub artifact attestations.

### Residual R1 — High — The chosen per-subject evidence-ref cannot perform the campaign-wide compare-and-swap it promises

Section 11 requires every ledger publication to compare-and-swap the exact
predecessor and reject a stale writer
(`docs/standards/testing-and-gate-strategy.md:797-814`). Section 11.1 then chooses
one ref *per subject commit*:
`refs/openwepp/evidence/campaigns/<campaign-id>/<subject-commit>` (`:892-910`).
Those are different Git refs as the campaign head changes. A newly named ref
does not contain the prior evidence commit, so it cannot be atomically updated
from that prior value. Two concurrent increments can each create their distinct
subject ref successfully from the same predecessor ledger, leaving two
campaign leaves even though each per-ref update was atomic. Conversely, if a
new subject ref is expected to contain the prior commit before certification,
the authority does not define who creates that staging ref or how doing so is
made atomic with admission.

This means the D2-003 persistence cycle is fixed, but the selected storage model
does not yet enforce the D2-004 lost-update rule.

Required remedy: define a stable mutable campaign-head ref, for example
`refs/openwepp/evidence/campaigns/<campaign-id>/head`, and immutable per-subject
refs under a separate namespace. Every ledger/head advance must use one atomic
multi-ref transaction that compares the stable head against the exact
predecessor, advances it to the new evidence commit, and creates the subject ref
only if absent. Define which ref is authoritative if the transaction partially
fails (or require an atomic Git ref transaction), how a certified terminal
subject is frozen, and how release lookup uses the immutable subject ref. Add a
fixture where two different subject commits race from one predecessor and
exactly one transaction wins.

### Residual R2 — High — Two closed state reductions still have missing or ambiguous transitions

The obligation enum has no `SUPERSEDED` state (`docs/standards/testing-and-gate-strategy.md:829-839`),
but the allowed transition table sends any nonterminal state to a “superseded
obligation” (`:850-867`). The fold cannot emit a closed value for that event.
The table also allows a stale obligation to pass through a new passing receipt,
but not to become `FAIL`, `BLOCKED`, or `PENDING` when its required rerun fails,
cannot execute, or is queued. An implementation must invent an intermediate
transition or reject a legitimate rerun outcome.

The assurance fold has the same class of gap. It creates immutable entries and
describes aggregate dominance, but defines no closed per-entry disposition
states. “Any unknown or unresolved entry” yields `IMPACT_PENDING`, while “any
unresolved refresh demand” yields `REFRESH_REQUIRED`
(`docs/standards/testing-and-gate-strategy.md:1083-1095`); without distinct
entry-state definitions, a refresh demand is also plausibly an unresolved entry
and the second branch is unreachable. The authority also does not bind a refresh
completion event to the exact impact-entry IDs it resolves. This matters when C-
004's registry-wide discovery produces several impacts for one report.

Required remedy:

- Add `SUPERSEDED` to the closed obligation states or define supersession as an
  event that removes the old obligation from the active fold while retaining a
  separately named terminal result. Permit `STALE -> PENDING/FAIL/BLOCKED` and
  define the current-receipt event that selects each result.
- Define closed per-impact entry states, for example unassessed,
  no-material-impact-authorized, refresh-required, refresh-complete,
  superseded, and invalidated. A refresh-completion event must bind the exact
  entry IDs, target head, source/result roots, and review authority it closes.
- Rewrite the aggregate precedence only in terms of those closed states and add
  fixtures for stale-rerun failure/block, obligation supersession, one report
  with mixed unassessed/refresh/no-impact entries, and a refresh completion that
  omits one current impact.

### Residual R3 — Medium — Two normative field descriptions contradict their canonical mechanics

The required input list still says changed kinds include “rename”
(`docs/standards/testing-and-gate-strategy.md:400-405`), while the canonical Git
algorithm prohibits rename detection and represents every rename as delete plus
add (`:419-435`). The gate-node fields likewise say prerequisites are “gate IDs”
at `:535`, while the next paragraph requires prerequisites to reference
`node_id` at `:543`. Finally, `node_id` is described as content-derived but only
explicitly binds parameters, target, features, matrix, shard, and retry policy
(`:526-531`); it does not state that the digest covers the complete canonical
node definition, including gate-definition ID, executor/argv, prerequisites,
acceptance rule, and artifact namespace. Reusing a node ID after one of those
fields changes would be unsafe even though the enclosing plan ID changes.

Required remedy: remove `rename` from the closed change-kind input vocabulary;
say prerequisites reference `node_id` consistently; and define `node_id` as the
digest of the entire canonical node payload excluding only its derived ID and
runtime outputs. Add a node-ID mutation fixture for executor, argv,
prerequisite, acceptance, and artifact-namespace changes.

### Verification checks run

Ran after reading the amended tree:

- scoped `markdown-doc lint` and `markdown-doc validate` over ADR-0039, the
  testing/gate standard, the three documentation catalogs, and the complete work
  package;
- `git diff --check` over the repository documentation candidate;
- `uk2us` preview over the authority, handoff, round-2 disposition, and this
  review artifact; and
- focused path/term scans for trust classes, hermeticity, evidence refs,
  compare-and-swap, obligation transitions, backstop thresholds, node identity,
  Git/Cargo normalization, assurance fold, CI contexts, cutover scorecard, and
  added primary references.

The documentation checks are necessary but do not cure R1–R3. Verification
remains `HOLD` until those authority ambiguities are corrected and this reviewer
rechecks the exact amended bytes.

## Final Residual Verification

Evidence class: `Static` plus `Ran` scoped documentation checks

Verification disposition: `HOLD`

The exact amended authority now resolves the prior R1–R3 mechanics:

- the stable campaign `head` ref and immutable subject alias are updated/created
  in one atomic compare-and-swap transaction, so concurrent distinct subjects
  cannot both advance one predecessor;
- `SUPERSEDED`, stale rerun, failure, block, and pending obligation transitions
  are explicit;
- assurance impact entries have closed states, exact refresh-completion bindings,
  and an order-independent target fold;
- release reuse requires both release-accepted `PROTECTED_CI` trust and
  `HERMETIC_CONTENT`, while other reuse classes rerun;
- the A0–A6 table exhaustively separates execution integrity, required
  conformance, nonblocking investigation, promotion, and `NOT_EVALUATED`;
- rename is consistently delete plus add, prerequisites consistently reference
  node IDs, and `node_id` hashes the complete canonical node payload.

The C-lane changes interact coherently with these repairs: registry-wide report
discovery feeds the now-closed assurance fold; A0 admission and A1/A3
conformance remain non-deferrable; nonblocking authority divergence remains a
separate investigation; and the stable evidence head provides the lost-update
guard needed by concurrent increment replanning.

### Final residual — Medium — Bootstrap creates a state that the exhaustive obligation transition table cannot create or clear

The campaign bootstrap rule admits prior evidence as `LEGACY_UNVERIFIED`
(`docs/standards/testing-and-gate-strategy.md:790-797`), and the closed obligation
enum defines that state (`:845-867`). The exhaustive allowed-transition table,
however, permits creation only to `PENDING`, `DEFERRED`, or
`NOT_APPLICABLE`, and has no transition from `LEGACY_UNVERIFIED` to `PENDING`,
`PASS`, or an explicitly named replacement (`:869-888`). Consequently two
implementations can reasonably disagree whether bootstrap is an out-of-table
special creation, whether legacy evidence is a separate non-obligation record,
or whether the campaign can ever rerun that obligation. The handoff explicitly
requires active campaigns to use this bootstrap route, so the ambiguity is
reachable rather than theoretical.

Required remedy:

- Add bootstrap creation to `LEGACY_UNVERIFIED` as an explicit allowed event.
- Require `LEGACY_UNVERIFIED -> PENDING` through a terminal replan/rerun event,
  followed by the ordinary accepted-receipt transitions, or explicitly permit
  it to become `SUPERSEDED` only when the named replacement obligation is
  created atomically.
- State that a content-verified legacy artifact retains its observed trust/reuse
  class and cannot create `PASS`; only a new accepted receipt can satisfy the
  replacement/current obligation.
- Add a fixture that imports an active campaign with legacy evidence, replans
  and reruns the obligation, and reaches certification without backdating a
  pass or deferral.

### Final checks run

Ran on the final reviewed tree:

- scoped `markdown-doc lint` and `markdown-doc validate` for ADR-0039, the
  standard, all three catalogs, and the complete package;
- `git diff --check`; and
- `uk2us` preview for the ADR, standard, handoff, round-2 disposition, and this
  review artifact.

All documentation checks pass. The final verification remains `HOLD` only for
the closed bootstrap transition above. Once that small state-machine gap is
patched, no other D-lane actionable finding remains.

## Terminal Bootstrap Reverification

Evidence class: `Static` plus `Ran` scoped documentation checks

Verification disposition: `PASS`

The bootstrap gap is closed on the exact live tree. The obligation transition
table now permits bootstrap-only creation of `LEGACY_UNVERIFIED`, permits it to
enter the ordinary evidence path only through adopted replan/rerun to `PENDING`,
or permits atomic supersession only with a named replacement obligation. It
explicitly forbids direct transition to `PASS` and trust/reuse promotion of the
imported artifact
(`docs/standards/testing-and-gate-strategy.md:870-888`). Campaign closure still
rejects unsatisfied legacy evidence (`:864-868`), and implementation handoff
scenario 32 requires a complete bootstrap-to-certification fixture without
backdated pass or deferral.

This transition composes correctly with protected-CI receipt trust, the stable
campaign-head compare-and-swap, ordinary `PENDING -> PASS/FAIL/BLOCKED`
execution, and atomic replacement semantics. It creates no route for legacy
content integrity to masquerade as authenticated current evidence.

I reran scoped Markdown lint/validation, `git diff --check`, and American-
English previews after this patch. All pass. No actionable D-lane finding
remains. The final round-2 reviewer D disposition is `PASS`.
