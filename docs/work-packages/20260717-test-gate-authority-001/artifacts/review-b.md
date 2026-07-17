# Independent Review B: Architecture And Implementation Feasibility

Evidence class: Static

Disposition: `HOLD`

Reviewer B independently inspected the package, ADR-0039, the testing and gate
standard, implementation handoff, current Nextest configuration, release
workflow and runners, ADR-0021, and the implemented assurance planner and
lifecycle surfaces. Reviewer A's artifact was not read. This reviewer changed
only this file.

The proposed separation of increment, campaign, and release gates is the right
architecture. Nextest is correctly limited to execution rather than impact
authority, evidence reuse is correctly content-oriented, and assurance impact
is correctly separated from historical scientific validity. The candidate is
not yet implementation-ready, however: the findings below would either restore
release-scale work during ordinary increments or require the follow-up package
to make new policy decisions that this package says are already settled.

## Findings

### B-001 — Blocking — The critical-risk rule contradicts the motivating bounded-crate case

The standard classifies an isolated new process crate with no production
consumer as a bounded component for which targeted coverage/CRAP is sufficient
(`testing-and-gate-strategy.md:292-299`). Adding that crate to this workspace
necessarily changes workspace membership and normally changes dependency
resolution, both of which are unconditional critical triggers requiring an
immediate campaign-closure-strength workspace regression and global CRAP
(`testing-and-gate-strategy.md:309-324`). The handoff nevertheless requires an
acceptance fixture in which exactly such a crate avoids unrelated snow and
runner suites (`implementation-handoff.md:109-115`). Both rules cannot be
implemented.

Required remediation: distinguish additive, mechanically isolated workspace
membership from changes to existing resolution, global features, toolchains,
or shared workspace behavior. Define the proof for the bounded case (new member,
no incoming production edge, no existing package resolution/feature change,
explicit impact-map ownership, and cargo-deny) and retain critical escalation
when any part of that proof fails. Add the corresponding rule and reason codes
to the standard, not only to a future fixture.

### B-002 — High — A gate plan is a command list, not yet an executable gate contract

The standard requires suites to declare owner, dependencies, duration class,
failure policy, and minimum boundary (`testing-and-gate-strategy.md:176-180`),
but the required plan fields carry only selected checks, targets, filters, and
specialized commands (`testing-and-gate-strategy.md:380-396`). They do not
define stable gate IDs, prerequisite ordering, acceptance expressions, expected
test cardinality/inventory, timeout, retry policy, failure class, required
artifacts, blocking transition, or the reuse contract for each result. The
receipt then records aggregate counts and a final result without specifying how
those values are reduced across gates (`testing-and-gate-strategy.md:466-484`).
The phrase “argument arrays rather than shell strings where practical” at line
474 leaves an uncontrolled execution path even though the current release
executor composes shell scripts and environment-dependent commands
(`tools/release/run_release_candidate_gates.sh:553-570`). Finally, a plan is
asked to contain an expected receipt ID before execution
(`testing-and-gate-strategy.md:396`), while the receipt contains runtime results
and timestamps (`testing-and-gate-strategy.md:470-481`); a content-derived
receipt identity cannot be known at plan time.

Required remediation: normatively define a typed per-gate DAG record with at
least `gate_id`, executor kind, argv/cwd and environment allowlist, prerequisites,
expected inventory/cardinality, acceptance rule, timeout/retry/failure policy,
artifact contract, blocking boundary, and reuse class. Define deterministic
`plan_id` and pre-execution `execution_key` separately from the post-execution
`receipt_id`, and define aggregate result precedence. Constrain shell-backed
legacy executors through a named, hashed adapter rather than an open-ended
“where practical” exception.

### B-003 — High — Root identity and evidence reuse are not mechanically defined

The four root descriptions list categories but do not specify whether a gate's
execution root contains the whole repository category or only its transitive
affected closure (`testing-and-gate-strategy.md:442-464`). Whole-repository
production/test roots would invalidate every focused receipt after any source
change and defeat reuse; selected-only roots would permit an omitted dependency
to remain invisible unless the exact closure construction is authoritative.
The standard also does not define canonical path records, file type/mode and
symlink treatment, untracked files, submodules, hash algorithm, map
canonicalization, environment projection, or platform-compatibility rules.
Those omissions also make the dirty-tree-to-commit equivalence rule
(`testing-and-gate-strategy.md:496-498`) non-reproducible.

Required remediation: define each root as a canonical, versioned manifest over
the plan's complete transitive input closure. Specify record fields, stable
ordering, SHA-256 (or another named algorithm), path/file-mode/symlink handling,
Git and dirty-tree coverage, applicable authority subset, tool/environment
projection, and aggregate-root construction. Receipt verification must first
recompute the impact closure, then compare its manifest; it cannot trust the
receipt's own input list. Define which platform/environment differences are
identity-breaking per gate family.

### B-004 — High — Campaign deferral has an impossible timing contract and no ledger evolution model

ADR-0039 permits deferral only when declared before implementation
(`0039-campaign-scoped-risk-based-testing-and-assurance-gates.md:61-65`), and the
ledger repeats that `DEFERRED` is assigned before implementation
(`testing-and-gate-strategy.md:517-529`). The mechanical planner, however,
derives obligations from the actual base/head changed paths
(`testing-and-gate-strategy.md:336-374`), which ordinarily exist only after the
implementation has been made. An emergent consumer, fixture, or authority edge
cannot satisfy both rules. The campaign declaration also lacks normative
transitions for campaign creation, admission of an increment, ledger amendment,
head advancement, overlapping campaigns, base rebasing, abort/supersession, and
recovery of campaigns already active when the policy lands
(`testing-and-gate-strategy.md:504-533`).

Required remediation: define a two-stage contract: a pre-implementation intent
plan based on the authorized write set and declared surfaces, followed by a
terminal plan over the exact diff. State how newly discovered obligations are
handled without retroactively pretending they were predeclared—for example,
they remain current-increment `PENDING` unless a governed campaign-ledger
amendment is accepted before increment closure. Define ledger version identity,
allowed state transitions, amendment authority, campaign-head chaining, and a
bootstrap rule for existing/standalone work. Do not leave these choices to an
agent or to the planner implementation.

### B-005 — High — Assurance “axes” and semantic watches are not a deterministic schema

The four assurance labels are presented as states, but the standard explicitly
says the first and second may both be true
(`testing-and-gate-strategy.md:590-604`). The current assurance planner has one
mutually exclusive `V2PlanState` enum and one state field per node/report
(`crates/openwepp-assurance/src/v2/planner.rs:20-28,41-61`), so the follow-up
cannot implement the new model as an enum extension. In addition, semantic
watches are required to catch newly introduced relevant files, but no watch
types, match semantics, ownership, default behavior, impact-entry identity,
deduplication, or resolution transition is defined
(`testing-and-gate-strategy.md:606-632`; `implementation-handoff.md:77-90`).
“Relevant” would therefore reintroduce agent judgment at exactly the supposedly
mechanical boundary.

Required remediation: define a multi-axis assurance-impact record. At minimum,
separate assessed-realization integrity, campaign-impact disposition,
campaign-head transfer currency, and release-transfer currency; give each axis
closed states and transitions. Define versioned semantic-watch kinds (exact
path, path-prefix/glob with precise semantics, contract ID, Cargo package,
process/domain tag, result procedure, builder/schema), match behavior for add,
rename, delete, and unknown paths, impact-entry IDs, coalescing, owner, trigger,
and the lifecycle authority permitted to resolve each entry. Preserve the
non-mutating and nonpublic behavior already stated.

### B-006 — High — Release CRAP cadence conflicts with certification reuse

ADR-0039 requires fresh full-workspace CRAP at both campaign closure and release
qualification (`0039-campaign-scoped-risk-based-testing-and-assurance-gates.md:66-68`),
and the standard repeats fresh global evidence at both boundaries
(`testing-and-gate-strategy.md:571-576`). The release section instead consumes a
current campaign certification and reruns only invalidated portions, explicitly
allowing reuse when source, toolchain, configuration, fixtures, policy, and
platform match (`testing-and-gate-strategy.md:261-277`). The handoff likewise
says the release runner should consume the campaign certificate and avoid
duplicate execution (`implementation-handoff.md:63-68`). “Fresh” is not defined
as “newly executed” or “still current,” so implementers must choose between an
unnecessary second global coverage run and violating the stated release gate.

Required remediation: decide this in the authority. Prefer defining
`fresh/current for exact execution root` and allowing the exact campaign global
receipt to satisfy release when its full reuse contract matches; require a new
execution only when release construction changes a bound input or a gate has an
explicit rerun-on-release policy. Amend ADR-0039 wording if “fresh” currently
means newly executed.

### B-007 — Medium — The transition inventory is not exact or migration-complete

The package requires every known policy and automation surface to be named
(`package.md:73-76`), but the handoff uses “extend `openwepp-assurance plan` or
an adjacent planner surface” (`implementation-handoff.md:77-80`) and does not
name the assurance lifecycle/architecture/source-build authorities, assurance
schemas and report manifests, or the release-transition/export/materialization
scripts that currently enforce assurance release behavior. It also gives no
adoption/rollback sequence for changing the always-on PR/main/scheduled release
workflow, which currently invokes the full release runner
(`.github/workflows/release-gates.yml:3-10,81-119`), or migration treatment for
open packages and retained evidence.

Required remediation: expand the handoff to an exact path-level inventory for
assurance authorities, source schemas/manifests, planner code/tests, release
transition/export/materialization scripts, workflow/status contexts, and
current gate runners. Define a staged adoption sequence: schema/governance
guards, shadow planner comparison, retained-campaign replay, nonblocking CI
observation, discrepancy disposition, blocking cutover, and rollback criteria.
Name how active packages/campaigns and retained receipts are imported or
declared legacy rather than leaving migration implicit.

## Recommendation

Hold authority-package closure until B-001 through B-006 are resolved in the
ADR/standard/handoff and B-007 completes the transition inventory. These are
not requests to implement the subsystem in this package. They are policy and
interface decisions needed to keep the implementation mechanical, conservative,
and materially less burdensome than the current workflow.
