# Work-package execution
> Canonical prospective package lifecycle and record format.

## One maintained record
Keep work-packages and their authorized scope/cadence. Each package uses
`docs/work-packages/YYYYMMDD-<slug>-001/package.md` as its only maintained
narrative: objective/scope, authority and acceptance, current state/next action,
checks/results, attributable review findings/fixes, and disposition.
Use the sections needed by the task; no fixed length or empty placeholders.

Raw logs, fixtures, patches, numerical outputs and necessary machine-readable
evidence may be separate files. Link them from the record; do not duplicate their
contents in reports. No mandatory kickoff file, prompt archive, reading map,
owned-file manifest, handoff, gate-results, line-count or final-disposition file.
An existing external format/schema may still require a specific evidence file.

## Reading and scope
Read root and applicable write-path instructions, this guide and package.md.
Read only your role procedure and the relevant standard/authority sections.
Combined author/implementer work reads them once. A link is a reference, not an
unconditional recursive reading assignment. Follow dependencies needed to
understand the affected obligation; uncertainty expands the affected reading.
Kernel work follows the science-contract instructions and triggered sections
of science-obligations.md. Specialized package types use specialized-workflows.md.
Solver changes bind docs/standards/numerical-solver-architecture.md.
Do not preload historical catalogs or the author's entire onboarding into reviews.

State intent, authorized boundaries, acceptance and selected checks before edits.
Run tools/agents/find-agents --for <paths> before editing new surfaces.
An intended file list is a planning aid, not a second authorization ceremony:
adjust implementation paths within the authorized objective/boundaries and record
material changes. Explicit user exclusions and package authority envelopes bind.
Freeze result-bearing scientific predicates/protocols before measurement; do not
change them after seeing results to obtain acceptance. Reconcile the final diff.

## Manual Validation Planning And Tool Friction
Validation planning has no prospective executable.
Use docs/standards/testing-and-gate-strategy.md directly. No TESTGATE, gate-planning
receipt or replacement validation-planning engine. Cheap deterministic
checks and focused behavior tests precede expensive selected evidence. Run commands
locally with compact output unless delegation has a concrete independent benefit;
elapsed command time alone is not a reason to create another agent.

## Completion and failures
Continue the full authorized package through correction, checks and disposition
while safe in-scope work remains. Every applicable current-scope requirement
needs direct, current evidence. Record PASS, FAIL, BLOCKED or NOT RUN truthfully;
an unmet required acceptance criterion prevents completion.
Campaign-owned requirements may be assigned prospectively to a named later
boundary with owner, trigger and rationale. Never retrospectively defer a failed
current requirement. Correctly executed negative experiments can be complete;
they do not establish production readiness. Inherited lint treatment follows the
testing strategy, never an ad hoc waiver. Scope-specific failed results stay visible.
For DC work, an in-envelope correction remains implementation work, not a HOLD.

## Independent review and fix verification
Select the review count by consequence, not diff size or package naming:
- Editorial/navigation/routine administration with no behavior or authority
  change: executor checks; no mandatory independent reviewer.
- Bounded implementation, behavior-preserving maintenance, isolated experiments
  and profiling: one independent reviewer.
- Physics/numerics, conservation, state/restart, production activation, public
  semantics, security, authority or validation-policy changes: two independent
  reviewers with complementary correctness and QA/evidence assignments.
- Additional fresh verification is exceptional: name the distinct risk or
  release qualification obligation it addresses before assigning it.

Each reviewer inspects primary evidence and verifies accepted fixes in their own
scope. No automatic second wave of terminal verifiers. Parent self-review is not
independent review. Reuse unchanged review portions and evidence; reopen affected
claims only. A consequential unknown gets the two-review route until bounded.
Applicable scientific independent reconstruction remains mandatory regardless of
review count: independence of the calculation is not just another agent rereading
the producer's formula.

This guide explicitly requires and authorizes subagent delegation for selected
independent reviews, subject to actual tool policy; no magic kickoff wording is
needed. Assign scope and read/write limits. Reviewers write only their named
section of package.md, or return findings for verbatim attributable incorporation.
They do not edit source, authority or the other review. Serialize shared-record
writes. Same reviewers check fixes; unavailable required independence remains unmet.
Use role-review.md; role-verification.md is only for exceptional assignments.

Blocking findings must identify a concrete correctness, authority, security,
unsupported-claim, unmet acceptance or material maintainability defect with evidence.
Formatting preferences and requests for redundant narrative are nonblocking.
Resolve accepted findings, explain rejected ones, and name owners for genuine
out-of-scope defects without turning every note into a new package.

## Continuation, evidence and maintenance
Update package.md after material state changes, before handing work to another
agent, and at disposition. Record the actual source identity (base plus diff or
commit), relevant input identities and evidence needed for the claim. No routine
three-manifest scheme, self-hashes or hashes of signoff prose. Recheck actual state
on continuation; investigate relevant drift, not every historical dependency.
active.md is a locator; README.md is searchable history, not duplicate live status.

File length and coverage/CRAP are maintenance signals, not automatic unrelated
refactor requirements. Explicit size/metric packages retain their acceptance.
Reading/token measurements are optional observations unless requested; never claim
quota savings from byte counts alone. No routine overrun reports or role telemetry.

## Transition
This guide and aligned active standards/templates replace prior generic package
ceremony prospectively, including when resuming older packages. Use package.md
for new state; link prior handoffs/artifacts as historical evidence without
rewriting them. Explicit user-frozen scientific acceptance, specialized executable
schemas and release obligations remain binding. Record any substantive acceptance
change and its authorization; do not retroactively convert historical FAIL/HOLD
to PASS. No bulk historical migration is required.
