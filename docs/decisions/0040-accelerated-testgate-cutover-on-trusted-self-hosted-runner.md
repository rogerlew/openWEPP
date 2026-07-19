# ADR-0040: Accelerate TESTGATE cutover on a trusted self-hosted runner

**Status:** Accepted

**Date:** 2026-07-18 UTC

**Deciders:** Roger Lew, Codex

**Decision authority:** Roger Lew's 2026-07-18 direction to cut over as soon as
the runner, CRAP cleanup, adversarial review, patches, and acceptance test pass;
elapsed-time and increment-count observation gates are explicitly rejected.

**Amends:** [ADR-0039](0039-campaign-scoped-risk-based-testing-and-assurance-gates.md)

**Canonical operational authority:**
[Testing And Gate Strategy](../standards/testing-and-gate-strategy.md)

## Context

The first combined-path benchmark preserved the exact 26/26 inventory and
projected a 48.8% ordinary-increment wall-time reduction. The operator accepted
that result as a win. The conservative terminal evidence also showed why delay
is costly: full Nextest required about 11.4 minutes and fresh global CRAP about
47.8 minutes on the accepted TESTGATE blocker-lift tree. Repeating broad gates
at package, push, observation, campaign, and release boundaries can consume
roughly ten compute-hours per development day without producing independent
evidence.

The prior policy required at least 14 consecutive observation days, 20
representative increments, two clean environments, a 50% median reduction, and
a dual-required provider interval before cutover. At the stated operating rate,
the waiting period alone could consume about 140 avoidable compute-hours. It
also required a human or agent to remember and maintain a calendar scorecard.
That conflicts with ADR-0039's purpose: reduce redundant testing while retaining
mechanical safety.

Three consecutive GitHub-hosted shadow runs failed before executing the plan
because the cold runner invoked offline Cargo metadata before all locked
dependencies were present. The repository is public. GitHub warns that
self-hosted runners must not execute untrusted public pull-request code because
the runner can be persistently compromised. The `omarchy` host's exact OS and
isolation posture have not yet been inspected. The runner therefore needs a
supported isolated Linux guest rather than an assumed direct installation into
the desktop host.

## Decision

1. Replace calendar- and count-based pre-cutover observation with an
   event-driven acceptance boundary. Fourteen days, 20 increments, two clean
   environments, and a dual-required interval are not cutover requirements.
2. Accept the demonstrated 48.8% projected reduction as sufficient performance
   evidence. The previous 50% threshold is retired; safety findings still block.
3. Use the `omarchy` NUC as the primary TESTGATE execution host through a
   supported isolated Linux guest, a repository-scoped runner, a dedicated
   unprivileged account, and labels that select only this trusted runner.
4. Never route public `pull_request` code to `omarchy`. Automatic self-hosted
   execution is limited to trusted pushes to `main`; explicit manual dispatches
   must resolve to trusted repository commits. The runner receives read-only
   repository permissions, no host secrets, no privileged container socket,
   and no access to unrelated homelab data.
5. Remove the daily shadow schedule and repeated per-job tool installation.
   Bake the reviewed, pinned toolchain into an immutable runner image. Bootstrap
   the locked dependency set before any offline planner operation, reuse it only
   within that job, purge all writable surfaces afterward, and prove one cold
   writable-surface job.
   Receipt authentication uses a tokenless hosted verification job that
   independently reconstructs selection and exact Nextest/A3 inventory, then a
   separate minimal OIDC-enabled aggregate that runs no candidate code before
   minting and verifying the native attestation.
6. Cutover occurs immediately after all of these event gates pass on one exact
   candidate commit:
   - the labeled runner is online and completes a confined end-to-end job;
   - cold-cache bootstrap and offline planning both succeed in their declared
     order;
   - the post-change actionable CRAP set is empty;
   - adversarial review findings are dispositioned and accepted findings are
     patched;
   - the acceptance matrix proves documentation, bounded, integrated/critical,
     failure-receipt, unknown-impact escalation, and untrusted-event isolation;
   - one conservative full-suite comparison passes on the same candidate; and
   - the rollback command and retained conservative manual lane are proven.
7. The accepted planner/executor aggregate becomes the normal trusted increment
   path at cutover. The old broad runner remains available for critical,
   campaign, release, or explicit rollback execution; it is not dual-required
   and does not continue on every increment.
8. No post-cutover calendar scorecard is assigned to a human or agent. Concrete
   safety failures trigger correction or rollback when they occur. The
   repository does not currently have branch protection or a ruleset, so
   provider-context migration is not a current cutover operand.

## Supersession And Preservation

This ADR supersedes ADR-0039 and the testing/gate strategy only where they
require a 14-day/20-increment scorecard, two clean-environment replays, a 50%
performance threshold, protected-context migration when no such provider rule
exists, or a dual-required interval before TESTGATE cutover.

It preserves fail-closed selection, unknown-impact escalation, receipt
verification, output confinement, coverage and CRAP thresholds, exact
adjudication, science-contract obligations, conservation and consumer-path
proof, external-authority anti-evasion, campaign/release qualification, dual
review, and an executable conservative rollback lane.

## Consequences

- Cutover is blocked only by concrete engineering or safety failures, not the
  passage of time or accumulation of arbitrary increments.
- The self-hosted machine reduces dependence on hosted compute and keeps a warm
  Rust/tool cache, but the project owns runner isolation, maintenance, and
  availability.
- Public pull requests cannot use the trusted homelab runner. If public PR
  validation is later needed, it must use GitHub-hosted isolation or a
  separately authorized ephemeral design.
- A single exact-candidate broad comparison remains necessary because workflow,
  executor, and gate-authority changes are critical. Successful heavy evidence
  is reused rather than rerun for presentation or closure.

## References

- [GitHub: Adding self-hosted runners](https://docs.github.com/en/actions/how-tos/manage-runners/self-hosted-runners/add-runners)
- [GitHub: Self-hosted runners reference](https://docs.github.com/en/actions/reference/runners/self-hosted-runners)
- [GitHub: Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)
- [GitHub: Actions billing and usage](https://docs.github.com/en/actions/concepts/billing-and-usage)
