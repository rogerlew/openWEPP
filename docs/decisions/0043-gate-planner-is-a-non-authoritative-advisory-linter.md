# ADR-0043: Gate Planner Is a Non-Authoritative Advisory Linter

Status: Accepted

Date: 2026-07-27

Decider: Roger Lew

## Context

ADR-0039 through ADR-0041 turned validation planning into an authenticated
planner/executor/receipt/CI lifecycle. In CAL-04B that control plane consumed
substantially more time and code than the modeling work, created recursive
tooling prerequisites, and failed to preserve the first new scientific failure
as the primary package record. The mechanism intended to reduce agent friction
instead became execution and closeout authority.

The repository still needs explainable validation planning. It does not need a
second authority system between agents and the requirements already established
by work packages, standards, tests, science contracts, and protected-data
controls.

## Decision

1. The gate planner is re-conceptualized as a deterministic, read-only
   **advisory linter**. Its only product is cited information for an agent.
2. The linter may inspect declared intent, repository state, dependency
   metadata, policy mappings, and changed paths. It may report possible
   omissions, inconsistencies, excessive validation, relevant obligations, and
   suggested canonical commands.
3. The linter never executes validation, suggested, package-declared,
   workflow, remote, or user-controlled commands. Its only subprocesses are the
   closed, literal, read-only Git inspection allowlist ratified by Order
   0; package data may supply only validated repository-relative path or commit
   operands after `--`, never a program, subcommand, flag, config, helper, or
   environment value. The linter never writes state, issues permission,
   certifies evidence, changes lifecycle status, dispatches CI, controls
   runners, performs recovery or publication, or owns calibration or protected
   data.
4. A completed analysis exits zero regardless of findings. Nonzero means only
   invocation misuse or unavailable analysis. No output or exit code is a
   package, campaign, evidence, or custody verdict.
5. Agents select and run commands directly, record what actually ran, and
   disposition work against governing requirements. Linter findings are not
   requirements and are not evidence.
6. If the linter is absent, wrong, partial, or unavailable, work continues by
   the documented manual route. A known unmet governing requirement can prevent
   truthful closure; a linter defect cannot.
7. The linter has no CI role and no trusted execution identity. It may be
   invoked interactively, but no workflow may require, certify, or promote its
   result.
8. Underlying correctness, science, security, review, package, and
   protected-data obligations remain binding from their own authorities.
   A0/A1/A3 obligations, typed guards, applicable conservation and direct-
   consumer proofs, external-authority anti-evasion, exact-diff reconciliation,
   and explicit package gates are not weakened.
   Unknown or ambiguous production impact receives agent-owned documented
   conservative escalation or authority clarification and is never silently
   narrowed. Evidence reuse remains bound to its source, execution and
   documentation roots, and every relevant input required by the claim.
   Applicable assurance approval, publication, and campaign/release-transfer
   disposition remain under direct assurance governance.
9. Quality measurement remains optional and observational except where an
   explicitly authorized metric-focused package makes its own targets binding.
10. Harvard custody remains outside the linter. Removing legacy planner
    integration cannot occur until a separate owner preserves the nonempty
    freeze, two independent read-only verifier PASS records, durable
    `OPENED_ONCE` transition before the first content read, digest and lock
    checks, no rerun after a post-open crash, read-only Harvard access, and no
    calibration-output write capability or path in the holdout process.
11. Historical TESTGATE evidence retains its original bytes and meaning.
    Historical verifiers may remain only while a named consumer exists; they
    confer no prospective authority.
12. The retained advisory implementation must prove measured utility and stay
    within the budgets ratified by the roadmap. Missing the utility, safety,
    noise, or complexity thresholds disables the linter path; it never stops
    the originating modeling work.

## Supersession And Preservation

This ADR prospectively supersedes ADR-0039, ADR-0040, and ADR-0041 wherever
they assign selection, execution, admission, transition, receipt, attestation,
certification, recovery, publication, runner, or CI authority to TESTGATE or
the gate planner. It also supersedes rejected-alternative language that treats
agent judgment as inherently invalid: agents own judgment but cannot waive
governing requirements.

It preserves:

- ADR-0039's five useful validation moments, declared intent, terminal-diff
  reconciliation, correctness/science substance, deterministic explanation,
  conservative unknown-impact handling, content/input-bound evidence reuse,
  direct assurance approval and transfer obligations, investigation posture
  for A2/A4/A5/A6, and the binding A0/A1/A3 distinction;
- ADR-0040's historical facts and prohibition on running public untrusted code
  on a trusted host;
- ADR-0041's optional observational quality model, explicit metric-package
  closure, complete-profile measurement semantics, and immutable history; and
- ADR-0042's science-implementation, calibration-readiness, data-authority, and
  identifiability distinctions.

The exact decision-by-decision disposition and operative-document amendment
plan are part of the accepted Order-0 evidence in
`20260727-gate-planner-advisory-linter-roadmap-001/artifacts/`.

## Consequences

- Agents can receive deterministic validation advice without waiting for a
  control-plane transaction.
- A tool failure becomes ordinary tooling debt, not a prerequisite package.
- Evidence is again the result of commands actually run, not a planner-owned
  permission or receipt hierarchy.
- Existing planner/executor/CI surfaces remain historical transition debt until
  the ordered removal packages delete or quarantine them.
- The repository must maintain clear canonical requirements because the linter
  is neither a substitute for readable governance nor an authority.
