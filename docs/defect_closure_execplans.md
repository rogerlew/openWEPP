# Authoring Defect-Closure ExecPlans

> A Defect-Closure ExecPlan (DC-ExecPlan) is a constrained
> [ExecPlan](codex_exec_plans.md) whose objective is to **close an observed
> invariant violation within a declared authority envelope** — diagnosing *and*
> correcting it in a single autonomous pass, rather than relaying the diagnosis
> to the next package.

Authorship: rationale and authoring guidance by Claude Code; the package shape
was proposed by Codex in a 2026-06-06 design consultation and ratified by
[ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md). This
document extends `docs/codex_exec_plans.md`; it does not replace it.

## 1. What a Defect-Closure ExecPlan is

A DC-ExecPlan is an ExecPlan first. Everything in `docs/codex_exec_plans.md`
applies: it is self-contained, it is a living document, it uses milestones, it
maintains `Progress` / `Surprises & Discoveries` / `Decision Log` /
`Outcomes & Retrospective`, and it carries the dual-review and disposition gates.

What distinguishes it is its **objective and its terminal states**. A
DC-ExecPlan does not exist to "trace the next variable" or "produce one more
fact." It exists to take a *named, observed defect* — an invariant violation, a
fail-closed event on valid input, a conservation residual — and drive it to one
of exactly three honest ends:

1. a **landed, contract-first correction** that is validated to remove the
   defect;
2. a **validated non-defect** — the symptom dissolves under a complete or correct
   measurement (e.g. a conservation residual that vanishes once the balance
   identity is complete), or the existing behavior is proven correct (a typed
   fail-closed guard that is the right response to invalid upstream input); or
3. an **explicitly owned HOLD at a declared boundary** — the mechanism is proven
   to lie outside the package's authority, or the governing authority is missing
   or contradictory — with the *next defect* named, never the next inspection
   step.

"Produced more diagnostic evidence and deferred" is not a terminal state for a
DC-ExecPlan. That is the failure this shape is built to prevent.

**Autonomous closure presumption.** A DC-ExecPlan starts from the assumption
that the agent will close the defect inside the declared envelope. `HOLD` is an
exception that must be earned by evidence, not a normal stopping point when the
work becomes larger than expected. If source reading, contract amendment,
test-writing, production editing, or validation remains possible inside the
envelope, the package continues.

## 2. Why they are needed

This is the load-bearing section. The shape is a reaction to a concrete,
expensive failure mode, and the rationale is the reason the constraints below are
not negotiable.

### 2.1 The grind

The HPHYS0298→0320 arc ran roughly **twenty-two work packages over about two
days and landed essentially one production fix** — the HPHYS0320 `wnttim`
storm-start floor — at the very end. Everything between was diagnostic-only.
Each package narrowed a single divergence by one step down a call chain
(`hrsnow` → hourly snowfall input → 2013 terminal carry → `stmtim` control
surface → active interval → storm-start time), closed as `executed-hold`, and
handed the *next narrow step* to the next package. Each one carried the full
ceremony — contract authority, contract-derived tests, a pre-implementation
gate, two review agents, two verification agents, a disposition, a worker
handoff, and a full 39-hillslope metric run — for what was, in substance, one
step of a single debugging session.

### 2.2 Exhortation already existed and did not work

`AGENTS.md` already warns against exactly this: work packages "must be
right-sized to amortize their required scaffolding," and it names the
anti-pattern of "progressively smaller diagnostic-only packages that merely
advance a `HOLD`." The grind happened anyway. **A doc that only repeats the
warning will fail the same way.** The cause is not that authors forgot the rule;
it is that the package *templates and default terminal states* made the grind a
locally valid, locally successful path. The fix has to be structural.

### 2.3 The four structural attractors

1. **`HOLD` was a valid terminal success state.** A package could be "complete"
   by producing one more fact and deferring the fix. Diagnosis was never
   *required* to convert into correction inside the same package, so it didn't.
2. **Scope was defined by call-chain location, not by defect mechanism.** "Trace
   `hrsnow` into the snowfall input" is a *step*. "The storm start-time interval
   excludes the snowfall hour" is a *defect*. A package scoped to a location ends
   when it reaches the next location; a package scoped to a defect ends when the
   defect is closed.
3. **The autonomous trigger inherited the previous handoff's first item.** The
   one-line kickoff (`prepare and execute the next package`) scopes the new
   package to whatever the prior worker-handoff named as item 1. When handoffs
   named the next *diagnostic step*, the chain propagated steps indefinitely. A
   strategic decision made elsewhere — an ADR, a roadmap — has no hook into this
   chain; only the handoff's item 1 does.
4. **"Contract-first" was misread as "do not touch production in this package."**
   The intended sequencing is *amend the canonical contract, add contract-derived
   tests, record a pre-implementation gate, then edit production* — all within
   one package. It was instead read as a prohibition on landing any fix now,
   which deferred every correction to a perpetually-future "implementation
   package."

A fifth, enabling factor: **reviews verified artifact completeness, not `HOLD`
legitimacy.** No gate asked, "Has this package identified a fixable, in-scope
defect and deferred it anyway?"

### 2.4 The cost is strategic, not just wall-clock

The grind's price is not merely time. Sixteen packages and a ratified ADR
(ADR-0016) came to rest on a premise — "after the negative-melt fix, surviving
baseline divergences are openWEPP defects" — whose keystone evidence (the
HPHYS0298 `hrsnow` finding) the retrospective identifies as a
depth-versus-water-equivalent **unit artifact** rather than a defect (see the
HPHYS0298 review artifact `review_claude_hrsnow_unit_artifact.md` and ADR-0017's
re-pinning). On that reading the arc spent its motion narrowing a phantom.
ADR-0017 re-pinned the discipline (the comparator is an investigation flag, not a
target; no `OPENWEPP-DEFECTIVE` without like-for-like proof), but the structural
attractor that produced the grind survived it. A long chain of locally-valid
diagnostic-only packages does not just waste motion; it manufactures false
confidence and can encode a wrong premise into strategy before anyone notices.

### 2.5 The principle

**Diagnosis that cannot convert to correction inside a bounded envelope will
relay indefinitely.** The DC-ExecPlan exists to force one of two things at the
package boundary: the conversion (diagnose → root-cause → fix → validate → close)
or an honest, *owned* stop at a named boundary. There is no third option of
"deferred for more study."

## 3. The Correction Authority Envelope

Every DC-ExecPlan declares, up front, a **Correction Authority Envelope**. This
is the device that simultaneously *bounds* the package (so it cannot sprawl into
an unbounded mega-package) and *empowers* it (so it has standing authority to
land a fix inside the boundary without re-litigating that authority mid-flight).

The envelope declares:

- **Defect IDs and observed violations** — each defect the package owns, stated
  as an observable failure (a typed error code on valid input, an invariant
  violation, a conservation residual over a named fixture), not as a location to
  inspect.
- **In-scope write-set** — the exact contracts and source files the package may
  amend or edit.
- **Authorized evidence and test surfaces** — where diagnostics and
  contract-derived tests may be added.
- **Allowed production-edit classes** — the kinds of correction permitted (e.g.
  "SIMIMPL28 radiation projection or its physical-bound guard," not "any climate
  code").
- **Acceptance criteria** — falsifiable, behavior-level: what a human can run and
  observe to confirm the defect is closed.
- **Conservation/output acceptance, when applicable** — the independent
  operands, rejected aliases/formulas, magnitude range, real closure audit, and
  metadata/schema alignment that will prove the fix is not self-restating.
- **Branch-out boundaries** — what is deliberately *out* of scope and exactly
  where an out-of-scope finding routes. These include **negative boundaries**:
  scope the package must *not* touch even if it finds the cause there (see §8).

**Envelope adequacy.** The initial envelope must include the correction surfaces
the evidence most plausibly implicates — not an artificially narrow slice.
Scoping to a single file, proving the cause lies one file over, and holding is a
grind-HOLD wearing an envelope (§6). A surface may be excluded from the envelope
only by a *cited* authority, roadmap, or ownership boundary (for example the
suspended snow route), never by silence. A reviewer must reject an envelope that
excludes the evidence's most likely correction surface without such a citation.

**Implementation adequacy.** The envelope must authorize the correction route
needed to remove the defect from the real production path, not merely enough
surface area to add a wrapper, adapter, skeleton, shadow path, or diagnostic
counter. If the observed defect is on a direct kernel, publication, or consumer
path, the package must move the real consumer to the corrected path and prove
the old path is not used for the claim. A convenience bridge is not a correction
when a direct production implementation is in scope.

**Grouping and splitting.** One DC-ExecPlan may carry multiple symptoms only when
they share the same authority envelope, write-set, and validation surface.
Symptoms that cross process-family authority must be split into separate
defect-closure packages. This is the rule that prevents both the mega-package
(everything in one) and the micro-WP (one step per package): the unit of a
DC-ExecPlan is *one authority envelope*, however many or few symptoms fall inside
it.

## 4. The conversion rule

This is the normative core. State it, or an equivalent, in every DC-ExecPlan:

> If the package establishes a **reproducible root cause inside the declared
> envelope**, and the **expected behavior is supported by canonical `SC-*`
> contract authority, pinned-baseline provenance, or a contract-authorized
> physical invariant**, then the package **must** proceed through contract
> amendment, contract-derived tests, a pre-implementation gate, the production
> correction, validation, and dual-review disposition. It **may not** close as
> `HOLD` on the grounds that further investigation is possible.

**Authority ordering.** Pinned-baseline provenance and physical invariants are
evidence *for the contract*, not direct license for production code. The
production correction is authorized only after the canonical `SC-*` text has been
amended or confirmed to require the corrected behavior; the contract is always
the proximate authority for the edit (ADR-0011).

The conversion rule is what removes the §2.3(1) attractor. It makes "diagnosed an
in-scope, authority-backed defect and deferred the fix" an *invalid* terminal
state.

## 5. The bar that flips HOLD into a fix

A finding moves from `HOLD` to "land the fix" only when **all seven** of these
hold. Stating them up front makes the flip falsifiable rather than a judgement
call:

1. **Reproduction** — the failure is reproduced, or statically and
   unambiguously tied to a named fixture.
2. **Mechanism** — the symptom is reduced to a *named mechanism*, not another
   variable to inspect.
3. **Ownership** — the mechanism lies inside the declared write-set / contract
   authority.
4. **Authority** — the expected (correct) behavior traces to canonical `SC-*`,
   pinned-baseline WEPP provenance, or a contract-authorized physical invariant.
5. **Safety** — the fix does not loosen a fail-closed guard, silently clamp,
   invent physics, or canonicalize a domain violation away.
6. **Testability** — a contract-derived regression can be written that fails
   before the fix and passes after.
7. **Validation** — the package's acceptance target is measurable before and
   after the change.

For conservation residuals and output-publication defects, gates 6 and 7 require
anti-tautology evidence: fixtures must make plausible wrong formulas produce
different values, and validation must reconstruct the target from independent
produced operands. One-sided bounds and exact self-consistency checks can support
the case, but they cannot satisfy the validation gate alone.

This is ADR-0017's burden of proof made to cut *both* ways. Gates 4 and 5 forbid
a premature `OPENWEPP-DEFECTIVE` correction (you may not "fix" toward the
comparator without independent authority). Gates 1–3, 6, 7 forbid eternal
deferral (if a defect is reproduced, named, owned, authorized, testable, and
measurable, you must close it). Both failure modes are blocked by the same list.

For process-physics defects, gate 5 is strict: production code may not contain
surrogate, provisional, proxy, empirical stand-in, or heuristic physics. The
correction must implement canonical `SC-*` authority backed by pinned-baseline
provenance, literature authority encoded in the contract, or a
contract-authorized physical invariant. If that authority is absent or
contradictory, the package may hold for authority. If the authority is present
and the routine is in scope, the package must implement the actual physics.

## 6. Legitimate HOLD versus grind-HOLD

`HOLD` remains a correct outcome — but only at a boundary, never as a rest stop.

**Legitimate HOLD** (each names a *boundary*, and the handoff states it):

- The mechanism is proven to lie **outside the declared envelope**.
- The governing canonical authority is **missing or contradictory** (the package
  cannot know what "correct" is without a contract decision first).
- The input is proven **invalid upstream**, and the correct behavior *is* the
  typed fail-closed report (there is nothing to fix downstream).
- The required evidence **cannot be generated** in the available environment.
- The fix requires a **different process family or contract authority** not
  declared in this envelope.

Before recording `HOLD`, the package must add a **HOLD legitimacy audit** that
names the boundary, cites the evidence proving it, lists the in-envelope
correction route that was available or considered, and explains why that route
cannot close the defect in the current package. A hold without this audit is
undispositioned work, not closure.

**Grind-HOLD** (forbidden — these are the §2 attractor in disguise):

- "The next package should inspect the next function."
- "The next package should trace variable X one level deeper."
- "Root cause is in the declared files, but implementation is deferred."
- "Another package should add the contract test this package already specified."
- "The direct fix is larger than expected, so this package leaves a wrapper or
  compatibility bridge."
- "A surrogate formula keeps the path running while actual physics is left for
  later."

The test: a legitimate branch produces a **new defect-closure target** with its
own acceptance; a grind-HOLD produces a **diagnostic breadcrumb**.

## 7. The handoff and kickoff contract

Because the autonomous trigger inherits the handoff's first item (§2.3.3), the
handoff is the highest-leverage surface. A DC-ExecPlan handoff names a **defect**,
with these fields:

- defect ID;
- observable failure;
- suspected mechanism;
- in-scope authority / write-set;
- required reading;
- the failing fixture or evidence;
- correction authority;
- acceptance target;
- HOLD-legitimacy conditions;
- **Forbidden relay**: no handoff may name *only* a next diagnostic step.

The mechanical rule, stated so it cannot be missed: **the handoff's first
actionable item must be "close defect `<id>`," never "inspect `<function>`."**
The autonomous trigger inherits that first item verbatim (§2.3, attractor 3);
diagnostic steps belong *inside* the package's milestones, never as the handoff's
lead item.

The kickoff is correspondingly defect-shaped:

> Execution mode: package-end-to-end. Close defect `X` end-to-end. Diagnose
> internally until the mechanism is owned or a branch condition is met. If the
> mechanism is owned and contract-supported, amend contracts and tests, record
> the pre-implementation gate, implement the correction, validate, and complete
> dual review and disposition. Do not request a new package for intermediate
> diagnostic steps. Do not stop at `HOLD` while source reading, implementation,
> or validation remains possible inside the declared envelope. Do not introduce
> surrogate physics or compatibility wrappers as substitutes for the direct
> production correction.

## 8. Diagnostic-first defects and protected boundaries

Two cases need explicit handling so they do not become back-doors to the grind.

**Diagnostic-first defects (cause unknown).** Some defects are genuinely
investigation-first — a conservation residual whose cause is not yet known. The
DC-ExecPlan still applies; it gains one internal **attribution milestone** whose
exit condition is "attribute the symptom to a *named mechanism*, then close or
branch *by mechanism*" — not "observe the next surface." **Symptom-existence
gate:** if the *reality* of the symptom is not yet established, the first
milestone must establish it before any attribution.

**Independent-operand acceptance.** When the symptom is a conservation residual
or output magnitude defect, the attribution milestone must identify the operand
lineage before accepting a correction. The package must reject any gate that
uses the same operands as the producer formula unless those operands are
independently produced and explicitly authoritative. Record the tempting wrong
pairings in the package evidence so future regressions cannot reintroduce them.

> Worked guard — the WSHED01 `runvol` crossed-pairing. The wrong arc
> over-scaled as `QOFE * A_hillslope`, then under-scaled as `Q * A_outlet`,
> before closing on `QOFE * A_outlet`; both the one-sided
> `runoff <= precip` bound and a self-restating fixture accepted a wrong
> version.

> Worked guard — the WBVAL01 closure leak. Before attributing the +24–79 mm/yr
> residual to any mechanism, the package must first complete the water-balance
> identity it is measuring against (audit and include `Tile`, any populated
> interception-storage delta, and run-on/run-in terms such as `UpStrmQ` and
> `SubRIn`; note that `SoilWaterTotal` already includes `frozwt`). If the
> residual collapses to ~0 under the complete identity, there is **no defect to
> attribute**, and the package closes that finding as "no residual under the
> complete identity" — not as a target for the next package.

**Protected boundaries (negative scope).** An envelope may declare scope the
package must *not* correct even if it finds the cause there, routing it instead
to a named owner. This is how a DC-ExecPlan respects deliberately-suspended work.
Protected boundaries must be **declared up front and justified by a cited
authority** (an ADR, the roadmap, an ownership boundary); they may not be invented
after a cause is found. A reviewer must reject any protected boundary that merely
shields an in-scope, authority-backed fix from the conversion rule (§4).

> Worked guard — the snow suspension. If a WBVAL01 follow-on attributes a residual
> to snowpack mass-loss, it must **stop and route to the backlog snow-science
> review** (ADR-0017 and the agreed roadmap suspended that route). It may not
> correct snow physics in-package. The envelope's negative boundary is what keeps
> "attribute the leak" from becoming a back-door reopening of the parked route.

## 9. Relationship to existing process

- **ExecPlans.** A DC-ExecPlan *is* an ExecPlan (`docs/codex_exec_plans.md`) with
  added constraints. It relaxes nothing in that document.
- **ADR-0011.** The canonical `SC-*` contract is the correctness authority. Gate
  4 of §5 enforces ADR-0011; a DC-ExecPlan never treats the comparator or a
  producer intermediate as authority.
- **ADR-0017.** The fixed comparator is an investigation flag, not a target.
  §5 carries ADR-0017's like-for-like burden into the fix decision.
- **Dual review and disposition.** Unchanged and required. Four review
  obligations are added: a reviewer must check (a) **`HOLD` legitimacy** — that no
  in-scope, authority-backed defect was diagnosed and then deferred, and that
  any hold includes the required legitimacy audit (§6);
  (b) **envelope adequacy** — that the envelope was not drawn to exclude the
  evidence's most likely correction surface without a cited boundary (§3); and
  (c) **protected-boundary integrity** — that no negative boundary merely shields
  an in-scope fix from the conversion rule (§8); and (d)
  **conservation/output anti-tautology**, when applicable — that validation uses
  independent produced operands, rejects tempting aliases/formulas, and keeps
  metadata/schema lineage aligned with the accepted surface (§5).
- **Production-correction review.** For kernel/process-physics packages, review
  must explicitly check that production code implements actual contract-backed
  physics and contains no surrogate/provisional/proxy/heuristic stand-in. For
  direct-path or publication packages, review must explicitly check that the real
  downstream consumer reads the corrected path and that wrappers/adapters are
  not masking an incomplete correction.
- **Line-count governance disposition.** Required. Review artifacts must
   explicitly evaluate `.rs` file thresholds (2000=`WARN`, 3000=`required
   refactor`) and disposition any exception. Any approved 3000+ generated/fixture
   exception must include owner and sunset plan in package artifacts.
- **Not every package is a DC-ExecPlan.** Pure validation/characterization passes
  (such as WBVAL01), architecture scaffolding, and audits keep their own shapes.
  The DC-ExecPlan is specifically for *closing an observed defect*.

## 10. Authoring skeleton

A DC-ExecPlan's `package.md` / ExecPlan should contain, in addition to the
standard ExecPlan sections:

1. **Objective** — stated as the closure of a named defect, not a step.
2. **Correction Authority Envelope** (§3) — defects, write-set, evidence
   surfaces, allowed edit classes, acceptance, and branch-out / negative
   boundaries.
3. **The conversion rule** (§4), restated for this package.
4. **The seven-gate bar** (§5), with the package's specific authority and
   acceptance plugged into gates 4, 6, and 7, including anti-tautology evidence
   for conservation/output defects when applicable.
5. **Milestones** — the bounded internal loop (reproduce → localize to mechanism
   → classify ownership → amend contract/tests → pre-impl gate → fix → validate →
   review/disposition), plus an attribution milestone if the defect is
   diagnostic-first (§8).
6. **HOLD-legitimacy conditions** (§6) — the specific boundaries at which this
   package may legitimately stop, plus the required hold legitimacy audit fields.
7. **Defect-shaped handoff** (§7) — what the next package, if any, must close.

## 11. Provenance

This document was written following the HPHYS0298→0320 retrospective and the
WBVAL01 validation. The package shape was proposed by Codex; the rationale, the
diagnostic-first and protected-boundary guidance, and the framing here were
authored by Claude Code; and the structural rules in §3–§9 (envelope adequacy,
grouping/splitting, authority ordering, the three-terminal-state model, and the
added review obligations) were refined under Codex review on 2026-06-06.

Reference wiring is in `AGENTS.md` and `docs/codex_exec_plans.md`: validation-
derived invariant violations, fail-closed events on valid input, and
conservation residuals use a DC-ExecPlan unless explicitly classified
characterization-only; handoffs are defect-shaped; and reviews check `HOLD`
legitimacy, envelope adequacy, protected-boundary integrity, and applicable
conservation/output anti-tautology. The conversion
rule (§4) is a governance change, not only an authoring style — once the
like-for-like burden is met inside the declared envelope, deferral is no longer
permitted — and is ratified by
[ADR-0018](decisions/0018-defect-closure-execplans-conversion-rule.md).
