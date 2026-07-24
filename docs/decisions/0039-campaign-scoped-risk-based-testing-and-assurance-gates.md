# ADR-0039: Campaign-scoped, risk-based testing and assurance gates

**Status:** Accepted

**Date:** 2026-07-17 UTC

**Deciders:** Roger Lew, Codex

**Decision authority:** Roger Lew's 2026-07-17 direction to author the ADR and
canonical test authority before repository alignment

**Canonical operational authority:**
[Testing And Gate Strategy](../standards/testing-and-gate-strategy.md)

**Builds on:** [ADR-0011](0011-architecture-first-top-down-science-contracts.md),
[ADR-0021](0021-module-coverage-closure-thresholds.md), and
[ADR-0038](0038-manuscript-first-scientific-assurance-publication.md)

**Transition amendment:** [ADR-0040](0040-accelerated-testgate-cutover-on-trusted-self-hosted-runner.md)
replaces the elapsed-time, increment-count, 50%, and dual-required pre-cutover
operands with event-driven acceptance on a trusted self-hosted runner. The core
risk-based testing decision remains accepted.

**Quality amendment:** [ADR-0041](0041-separate-testgate-from-observational-quality-ci.md)
removes coverage/CRAP from increment, critical, campaign, and release
transition gates while preserving the quality model and explicit metric-package
closure.

## Context

openWEPP accumulated strong but fragmented quality rules. Science contracts
define process authority, the correctness-authority model ranks evidence,
ADR-0021 fixes coverage and CRAP thresholds, the Rust scientific coding
standard defines test obligations, and the assurance architecture tracks
report dependencies. No single document governed when each kind of evidence
must be executed.

That gap produced a release-shaped package workflow. Root and work-package
instructions require full workspace Clippy, full workspace Nextest, cargo-deny,
and fresh full-workspace LCOV/CRAP for every implementation package. The release
runner then executes full Nextest and a second workspace test pass under LLVM
coverage. The same runner is invoked on pull requests, pushes to `main`, weekly
schedules, campaign validation, and releases.

The resulting cost is disproportionate during active campaigns. An isolated,
not-yet-consumed process crate can spend more time proving unrelated snow,
runner, publication, and assurance behavior than implementing and testing its
own contract. Repetition also does not create independent evidence when the
same unchanged tests are run several times against the same source.

Large test-intensive projects separate fast affected checks from broader
integration and release lanes. They use dependency selection, explicit risk
escalation, post-submit or periodic backstops, merge/campaign batching, and
content-bound evidence reuse. openWEPP needs the same lifecycle while retaining
its stronger science-contract, conservation, consumer-path, coverage, CRAP,
external-authority, and release obligations.

## Decision

1. Adopt
   [`docs/standards/testing-and-gate-strategy.md`](../standards/testing-and-gate-strategy.md)
   as the sole canonical operational authority for test selection, gate timing,
   campaign deferral, risk escalation, gate evidence receipts, coverage/CRAP
   cadence, assurance impact currency, and CI lane assignment.
2. Separate five execution moments:
   - the non-authoritative edit loop;
   - increment closure;
   - campaign checkpoint;
   - campaign closure; and
   - release qualification.
3. Permit an implementation work package to close as a valid campaign
   increment after all increment-scope gates pass. Campaign-owned gates may be
   deferred only when they were declared in an accepted pre-implementation
   intent plan, are recorded in the campaign gate ledger, have a named trigger
   and owner, and are not represented as passed. The exact terminal diff must
   then reconcile the intent plan. Newly discovered increment obligations must
   run before closure; governed campaign amendments cannot retroactively defer
   failed or increment-required gates. Deferred is not waived.
4. Require full workspace regression at campaign closure, release
   qualification, and immediately after a critical-risk change. Coverage/CRAP
   is optional observational QA under ADR-0041 and is not a transition gate.
5. Select increment gates mechanically from the changed paths, Cargo dependency
   graph, explicit non-Cargo dependency map, contract/test bindings, external
   authority registry, assurance dependency graph, and risk rules. Unknown or
   ambiguous production-impact paths escalate; an agent does not silently
   choose a narrower suite.
6. Retain ADR-0021's coverage thresholds, obligation binding, eligibility
   taxonomy, CRAP threshold 30, and exact adjudication requirements as the
   quality-observation and explicit metric-package authority. ADR-0041 governs
   the non-blocking cadence.
7. Treat Nextest as a test executor, filter, scheduler, and sharder—not as the
   authority for manuscript, result, fixture, contract, publication, or other
   non-Cargo dependency impact.
8. Bind gate evidence to explicit execution and documentation roots. A current
   receipt may be reused only under its closed trust/reuse class, with every
   bound and ambient input identical or hermetically excluded. Review prose,
   disposition records, or unrelated documentation do not require rerunning
   unchanged executable evidence.
9. Separate assurance validity for an assessed realization, campaign-impact
   disposition, campaign-head transfer currency, and release-transfer currency.
   A relevant change mechanically produces a pending impact record without
   rewriting a report, invalidating its historical result, or initiating agent
   analysis. Impact discovery covers every registered report; exact release
   inventories must equal reports with current transfer. A report must resolve
   its state before campaign transfer, review-root advancement, or release
   publication as required by its lifecycle.
10. Require periodic full-regression backstops during long campaigns, with a
    mechanically enforced maximum age or increment count and an impact-planner
    defect whenever a broad gate discovers a missed regression.
11. Use deterministic, explainable selection first. Predictive or learned test
    selection is outside the initial architecture and cannot replace the
    conservative fallback without a separately accepted decision and measured
    miss-rate evidence.
12. Separate authority-suite execution integrity from scientific outcome. A
    valid A2/A4/A5/A6 divergence remains visible investigation evidence unless
    prospectively promoted; it is neither mislabeled pass nor automatically
    converted into blocking execution failure. A0/A1/A3 remain non-deferrable
    as required by the correctness-authority model.
13. Require authenticated evidence provenance. Local hashes alone cannot close
    a boundary; campaign and release certification require protected-CI issuer
    identity and offline-verifiable attestations. Content reuse defaults off
    unless environment/input confinement proves it safe.
14. Certify a frozen source subject through a two-phase, compare-and-swap
    transaction on GitHub-ruleset-protected evidence branch/tag namespaces,
    with a dedicated CI app as the only bypass actor. Evidence storage and later
    documentation never retarget the certified source commit.
15. Permit concurrent increment work only through expected-parent admission,
    exact-head terminal replanning, and compare-and-swap ledger advancement.
    CI cutover requires zero safety misses and a measured friction improvement
    before the focused path becomes blocking authority.

## Supersession And Preservation

This ADR supersedes only these earlier frequency and lifecycle rules:

- ADR-0021 Decision 8's sentence requiring every implementation package to run
  fresh full-workspace adjudicated CRAP;
- the work-package rule that every implementation or mechanical-refactor
  package must run the full workspace closure loop regardless of impact;
- Gate Evidence Non-Deferral interpretations that prevent a package from
  closing as a declared campaign increment solely because campaign-owned gates
  run later; and
- release/CI trigger policy that treats every pull request or push as release-
  scale validation.

Until the follow-up alignment package edits those documents and tools, their
existing commands remain the executable repository behavior and must not be
misrepresented as already conforming.

This ADR does not supersede:

- canonical `SC-*` authority or test-vector obligations;
- ADR-0021 coverage percentages, function floor, eligibility taxonomy, CRAP
  threshold, exception discipline, or explicit metric-package closure;
- fail-closed numerical and domain guards;
- direct consumer-path, conservation, reconstruction, or anti-tautology proof;
- external-authority lane classifications and fixture provenance;
- independent review requirements; or
- assurance approval, publication, and release-transfer requirements.

## Consequences

- Routine feature work receives fast, relevant feedback and can proceed through
  a campaign without repeated workspace qualification.
- Campaign closure becomes a real integration event with an exact certified
  head, rather than the repetition of package-level release gates.
- A critical change still receives immediate full correctness regression.
- Gate planners and receipts become governed interfaces. Missing mappings fail
  conservatively rather than allowing an agent to infer low risk.
- Assurance work can accumulate mechanically identified impact during a
  campaign and be resolved at a scientifically appropriate boundary.
- The repository temporarily contains transition debt: the new authority is
  accepted, but current AGENTS files, standards, workflows, and scripts remain
  to be aligned by a follow-up implementation package.

## Rejected Alternatives

### Continue optimizing the full suite

Scheduling and sharding help campaign and release runs but do not fix the wrong
frequency or the duplicate uninstrumented/instrumented workspace executions.

### Make every package a release candidate

This maximizes repeated evidence without distinguishing isolated work from
production activation. It consumes development time without a corresponding
increase in scientific assurance.

### Trust agent judgment for test selection

Agent analysis is difficult to reproduce and may miss non-obvious consumers.
Selection must be mechanical, explainable, and fail conservatively. Human or
agent operators may escalate but may not silently downgrade a plan.

### Defer all testing until campaign end

This produces slow fault localization and permits avoidable integration debt.
Every increment still requires its mechanically selected focused contract,
affected dependency, authority, consumer, and other correctness gates.

### Mark assurance reports simply stale

That collapses historical scientific validity and current-release transfer into
one misleading state. The adopted multi-axis assurance-impact model preserves
the exact assessed result while making new-realization impact visible.

## Transition

ADR-0041 Order 1 aligns repository governance with the amended authority.
Roadmap Order 2 must remove quality execution from TESTGATE and validate the
typed deferral in the gate-plan, receipt, verifier, and workflow contracts
before the amended selection mechanics become the normal execution path.
