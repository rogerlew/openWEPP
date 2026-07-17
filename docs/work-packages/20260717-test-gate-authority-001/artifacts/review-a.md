# Independent Review A — Scientific Correctness And Gate Philosophy

Evidence class: Static

Disposition: **HOLD**

The campaign-scoped direction is sound: it separates affected-change evidence
from integration and release qualification, preserves exact evidence identity,
and makes deferral visible rather than calling it a pass. The proposed authority
is not yet safe or mechanically complete, however. The findings below include
three paths by which mandatory or affected evidence could be omitted and two
lifecycle contradictions that the implementation package could not resolve
without reopening policy.

## Findings

### 1. High — Mandatory constitutive-authority suites can be deferred past a touched process increment

The correctness-authority model makes applicable A3 constitutive suites
mandatory and blocking whenever a package touches a process family that has
them (`docs/specifications/correctness-authority-model.md:39-47`). ADR-0039
expressly preserves external-authority lane classifications and fixture
provenance
(`docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md:115-124`).

The new standard instead combines empirical and external-authority suites into
one family whose normal earliest boundary may be checkpoint, periodic,
campaign, or release
(`docs/standards/testing-and-gate-strategy.md:164-175`). Its minimum increment
plan does not name applicable A3 suites
(`docs/standards/testing-and-gate-strategy.md:193-217`), while campaign closure
requires only external-authority gates *named by the campaign*
(`docs/standards/testing-and-gate-strategy.md:241-256`). This language can move
a blocking touched-process A3 obligation out of the increment or omit it from a
campaign, contrary to preserved authority. A4 empirical and A5 independent-
solver evidence may remain periodic/manual by default; A3 may not be grouped
with them for timing.

Required resolution: split constitutive A3 from empirical/manual families and
state mechanically that every applicable A1 and A3 obligation for the affected
process family is a non-deferrable increment gate. Require the planner to derive
the suite from the external-authority registry and fail closed when a touched
process family has an incomplete or ambiguous binding.

### 2. High — The bounded-new-crate example is classified as critical by another normative rule

The bounded-component class explicitly uses an isolated new process crate with
no production consumer as its example and says the increment plan plus targeted
coverage/CRAP is normally sufficient
(`docs/standards/testing-and-gate-strategy.md:292-300`). The implementation
handoff repeats that acceptance scenario
(`docs/work-packages/20260717-test-gate-authority-001/artifacts/implementation-handoff.md:109-115`).
But the critical class includes any workspace-membership or dependency-
resolution change
(`docs/standards/testing-and-gate-strategy.md:309-329`). Adding that crate to
the Cargo workspace necessarily changes workspace membership and normally the
lockfile package graph, so the motivating bounded case immediately escalates to
full regression and global CRAP.

Required resolution: distinguish a provably isolated workspace-member addition
from changes that alter existing production resolution, default membership,
shared features, test admission, or downstream dependency selection. Define the
mechanical proof needed for the bounded case and retain critical escalation when
isolation cannot be proven. Add the same distinction to the required reason-code
and acceptance-scenario contract.

### 3. High — Function-diff coverage cannot bound changes to non-function Rust items

Increment coverage is defined over new or changed eligible functions, plus
unchanged functions when their branch behavior or tests changed. The follow-up
is then told to implement a source-diff-to-symbol mapper that fails when it
cannot identify the complete affected function
(`docs/standards/testing-and-gate-strategy.md:541-556`). That is insufficient
for a change to a module constant, coefficient table, type or trait contract,
macro, generated input, build script, feature-controlled item, or shared error
definition. Such a change can alter many unchanged function bodies while a
line-based mapper reports no changed function. Package and reverse-dependency
test selection reduces the risk but does not establish the affected CRAP and
coverage denominator required by this section.

Required resolution: define affected *source items* rather than only changed
functions. For a non-function item, the planner must expand to all eligible
functions whose behavior can depend on that item, use a conservative package-
wide affected surface when exact dependency analysis is unavailable, and
escalate to global measurement when that surface still cannot be bounded. Add
fixtures for constants, shared types/traits, macros/build inputs, and feature
changes so an empty symbol selection is rejected.

### 4. Medium — The sole timing authority does not place two existing required quality gates

The Rust scientific coding standard currently requires both the placeholder/
stub pattern scan and `cargo test --doc` before merge, in addition to formatting,
Clippy, Nextest, cargo-deny, and CRAP
(`docs/standards/rust-scientific-coding-standard.md:338-349`). The proposed
standard declares itself authoritative for gate timing, but neither its bounded
increment minimum nor campaign-closure list assigns affected/full doctests or
the stub scan to a boundary
(`docs/standards/testing-and-gate-strategy.md:193-221` and `:241-259`). Nextest
full-profile execution does not by itself prove that rustdoc tests ran. The
handoff notices doctest parity only as a prototype concern
(`docs/work-packages/20260717-test-gate-authority-001/artifacts/implementation-handoff.md:58-75`),
which does not settle the policy.

Required resolution: explicitly assign affected stub detection and affected
doctests to increment closure when their surfaces can change, and assign their
workspace forms to campaign/release closure. Define their receipt inventory and
reuse rules. If either existing requirement is intentionally retired, say so in
ADR-0039's supersession boundary rather than dropping it implicitly.

### 5. Medium — Campaign admission and newly discovered obligations have no closable state transition

A deferred obligation is defined as assigned before implementation
(`docs/standards/testing-and-gate-strategy.md:118-123`), every campaign is
declared before its first implementation increment (`:504-516`), and an
increment can close with campaign entries only when they are already declared
(`:193-217`). The ledger later says an increment may close with entries deferred
to campaign closure, but not with `PENDING` entries (`:517-533`). The standard
does not define:

- whether every implementation increment must belong to a campaign;
- how a standalone repair or urgent defect is admitted;
- how an increment is admitted under a campaign's bounded admission rule; or
- what happens when replanning discovers a genuine campaign-owned obligation
  after implementation began.

The last case cannot truthfully be backdated as `DEFERRED`, but leaving it
`PENDING` also prevents increment closure under the stated rule. An executor
would have to invent policy or retroactively classify an obligation to proceed.

Required resolution: define campaign membership and admission transitions,
including a one-increment/standalone route. Preserve the rule that a newly
discovered current-increment gate must run or hold. Define a reviewed,
machine-recorded amendment route for a newly discovered inherently campaign-
boundary obligation, including discovery identity and a prohibition on using
the amendment to reclassify a failed or already-required increment gate.

### 6. High — Assurance impact detection is advisory and lacks a completeness fallback

The standard says relevant changes *should* add an assurance impact entry and
lists the exact and semantic dependency classes
(`docs/standards/testing-and-gate-strategy.md:584-624`). Because an exact-path
graph cannot notice a newly introduced relevant file and a semantic-watch graph
can itself be incomplete, `should` plus no coverage invariant permits an
approved report to remain apparently current for a campaign head without any
impact record. The assurance lifecycle requires a static impact analysis for
software-realization transfer and does not let a producer unilaterally declare
no impact
(`docs/governance/scientific-assurance-dossier-lifecycle.md:153-185` and
`:187-203`).

Required resolution: make impact creation mandatory. Require every selected
report to declare process/domain and contract-level semantic watches in addition
to exact paths; make missing, unknown, or incomplete watch coverage block
`CURRENT_FOR_CAMPAIGN_HEAD` and `CURRENT_FOR_RELEASE`; and define the
conservative impact produced by a newly added production/contract/result path.
Also state how an authorized no-material-impact disposition advances currency
without forcing a manuscript rewrite or claiming fresh reproduction.

### 7. Medium — A long campaign has no mandatory full-regression backstop interval

The periodic lane runs a full regression or expensive families only "according
to cadence" (`docs/standards/testing-and-gate-strategy.md:634-654`), but the
campaign declaration does not include a full-backstop cadence or maximum number
of uncertified increments (`:504-516`). Campaign closure will eventually catch
a regression missed by an incomplete impact edge, but a long-running campaign
can merge and build on that regression indefinitely. This is precisely where
the cited Firefox task-graph practice pairs selected work with explicit
backstop pushes; the research basis currently cites selection/optimization but
does not carry the backstop safeguard into policy.

Required resolution: require each campaign ledger to declare a maximum elapsed
time and/or merged-increment count between full regression backstops, with a
repository default and a mechanical due/overdue state. Define whether an
overdue backstop blocks further increment closure and require a discovered
missed regression to open a selector/impact-map defect as well as the product
defect.

## External-Reference Assessment

The large-project claims are generally supported by the cited primary or
authoritative sources. Rust documents focused PR/local testing, complete
merge-queue testing, and rollups; Chromium documents affected-suite commit-
queue selection with expensive CI-only tests; Firefox documents task-graph
selection, optimization, and backstops; Prow documents distinct presubmit,
post-submit, periodic, and changed-path controls; and Nextest documents
filtersets and coverage execution. These practices support the layered
direction, but they do not justify omitting mandatory A3 evidence or leaving
selector misses without a bounded backstop.

## Conclusion

The strategy should remain campaign-scoped and mechanically selected. No
finding recommends restoring full release-scale gates to every ordinary
increment. Closure should remain on HOLD until the mandatory-authority timing,
risk contradiction, affected-surface mechanics, assurance fallback, and
campaign transition gaps are corrected and independently dispositioned.
