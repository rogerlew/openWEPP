# ADR-0018: Defect-Closure ExecPlans — diagnosis must convert to correction

**Status:** Accepted
**Date:** 2026-06-06
**Deciders:** Roger Lew, Codex
**Author of draft:** Claude Code (drafted at decider Roger Lew's direction; ratified by Codex on 2026-06-06)
**Reaffirms:** [ADR-0011](0011-architecture-first-top-down-science-contracts.md), [ADR-0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)
**Authoring authority:** [docs/defect_closure_execplans.md](../defect_closure_execplans.md)

## Context

The HPHYS0298→0320 arc ran roughly twenty-two work packages over about two days
and landed essentially one production fix (the HPHYS0320 `wnttim` storm-start
floor), at the very end. Everything between was diagnostic-only: each package
narrowed a single divergence by one step down a call chain, closed as
`executed-hold`, and handed the next narrow step to the next package — each
carrying full ceremony (contract authority, contract-derived tests, a
pre-implementation gate, dual review, dual verification, disposition, handoff, a
full 39-hillslope metric run).

`AGENTS.md` already warned against exactly this ("progressively smaller
diagnostic-only packages that merely advance a `HOLD`"). The grind happened
anyway, which is the decisive fact: the cause is not author forgetfulness but
**structure** — the package templates and default terminal states made the grind
a locally valid, locally successful path. Four attractors produced it: (1) `HOLD`
was a valid terminal *success* state, so diagnosis never had to convert to
correction; (2) scope was defined by call-chain location, not defect mechanism;
(3) the autonomous one-line trigger inherits the prior handoff's first item, so
handoffs that named the next diagnostic step propagated steps indefinitely, and a
strategic decision made elsewhere (an ADR, a roadmap) has no hook into that
chain; (4) "contract-first" was misread as "do not touch production in this
package" rather than "amend contract, add tests, record the gate, then edit
production, all in one package." An enabling factor: reviews verified artifact
completeness, not `HOLD` legitimacy.

The cost is strategic, not merely wall-clock. Per ADR-0017, sixteen of those
packages plus a ratified ADR (ADR-0016) came to rest on a premise whose keystone
evidence was a unit artifact. A long chain of locally-valid diagnostic-only
packages manufactures false confidence and can encode a wrong premise into
strategy before anyone notices.

ADR-0017 added the principle that "findings must resolve to a decision," but left
it as prose with no executable shape. This ADR gives that principle teeth for the
class of work that closes defects.

The full articulation of the package shape, its rules, and worked examples lives
in `docs/defect_closure_execplans.md`. This ADR ratifies the **governance**
change that document encodes.

## Decision

1. **Adopt the Defect-Closure ExecPlan (DC-ExecPlan) as the required shape for
   defect-closure work** — closing an observed invariant violation, a fail-closed
   event on valid input, or a conservation residual. It is a constrained
   [ExecPlan](../codex_exec_plans.md); `docs/defect_closure_execplans.md` is the
   authoring authority. Validation/characterization, architecture, and audit
   packages keep their own shapes and are unaffected.

2. **The conversion rule is binding governance.** When a package establishes a
   reproducible root cause inside its declared Correction Authority Envelope and
   the corrected behavior is supported by canonical `SC-*` authority, pinned
   baseline provenance, or a contract-authorized physical invariant, the package
   **must** land the contract-first correction (contract amendment → tests →
   pre-implementation gate → production edit → validation → disposition).
   "Diagnosed an in-scope, authority-backed defect and deferred the fix" is an
   **invalid terminal state**. This abolishes `HOLD`-as-success for defect work.

3. **`HOLD` is bounded to declared boundaries.** A DC-ExecPlan may close in `HOLD`
   only at a named boundary: the mechanism is outside the declared envelope; the
   governing authority is missing or contradictory; the input is proven invalid
   upstream and the typed fail-closed guard is the correct behavior; the evidence
   cannot be generated in the environment; or the fix needs a different process
   family or contract not declared. A `HOLD` that names a next inspection step
   rather than a boundary or a next *defect* is non-conforming.

4. **Envelope adequacy and one-envelope-per-package are gates.** The declared
   envelope must include the correction surfaces the evidence most plausibly
   implicates; a surface may be excluded only by a cited authority, roadmap, or
   ownership boundary, never by silence. A single DC-ExecPlan carries multiple
   symptoms only when they share one authority envelope, write-set, and validation
   surface; symptoms that cross process-family authority are split.

5. **The handoff names a defect, mechanically.** A DC-ExecPlan handoff's first
   actionable item must be "close defect `<id>`," never "inspect `<function>`."
   This neutralizes attractor (3): the autonomous trigger then inherits a defect
   to close, not a step to relay.

6. **Review obligations extend.** Reviewers must check, in addition to the
   existing gates: `HOLD` legitimacy (no in-scope authority-backed defect
   deferred), envelope adequacy (the envelope did not exclude the evidence's most
   likely correction surface without a cited boundary), and protected-boundary
   integrity (no negative boundary shields an in-scope fix from the conversion
   rule).

7. **Burden of proof is preserved, both directions.** The seven-gate bar in
   `docs/defect_closure_execplans.md` §5 carries ADR-0017's like-for-like burden:
   it forbids a premature `OPENWEPP-DEFECTIVE` correction *and* forbids eternal
   deferral. Pinned-baseline/physical evidence authorizes a contract amendment,
   not production code directly; the contract is always the proximate authority
   for the edit (ADR-0011).

## Consequences

- The diagnostic-only relay chain (the HPHYS0298→0320 pattern) becomes
  **non-conforming** for defect-closure work. ADR-0017's "findings must resolve to
  a decision" gains an executable shape: the conversion rule, the envelope-adequacy
  gate, and the defect-shaped handoff.
- DC-ExecPlans carry more up-front scoping (the envelope) and more standing
  authority (they may land fixes) than diagnostic-only packages — deliberately, to
  force closure. The risk this introduces (an over-broad envelope or a premature
  fix) is bounded by the seven-gate bar and envelope-adequacy review, not by
  reverting to defer-by-default.
- **Wiring is part of the decision.** `AGENTS.md` and
  `docs/codex_exec_plans.md` carry the binding authoring rules:
  validation-derived invariant violations, fail-closed events on valid input,
  and conservation residuals use a DC-ExecPlan unless explicitly
  characterization-only; handoffs must be defect-shaped; reviewers must check
  `HOLD` legitimacy, envelope adequacy, and protected-boundary integrity. No
  separate central handoff/review template exists in this repo, so those root
  authoring rules are the canonical template guidance.
- Does **not** change: ExecPlan fundamentals, dual review/disposition, contract-first
  sequencing, or the authority model of ADR-0011/ADR-0017. The negative-melt
  collapse, the 260430 anchor (ADR-0012), and the comparator identity are
  untouched.
- First application: WBVAL01 follow-ons that close `CLIM-RUNTIME-E-017`,
  `HKERNEL-WB11-PERC-E-003`, or emitted-ledger conservation residuals are
  authored as DC-ExecPlans. The suspended snow route remains a protected
  boundary behind the backlog science review.

## Citations

- `docs/defect_closure_execplans.md` (full package shape, rules, worked examples).
- ADR-0017 (comparator is a flag not a target; "findings must resolve to a
  decision"; like-for-like burden).
- ADR-0011 (the SC-* contract is the correctness authority).
- `AGENTS.md` work-package right-sizing guidance.
- HPHYS0298→0320 retrospective and the WBVAL01 validation.
