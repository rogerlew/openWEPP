# ADR-0028: Observed-data admission authority when scientific authority is lacking

**Status:** Proposed (operator-decided 2026-06-28; pending ratification by a Codex package)
**Date:** 2026-06-28
**Deciders:** Roger Lew, Codex
**Author of draft:** Claude Code (drafted at decider Roger Lew's direction)
**Extends:** [ADR-0011](0011-architecture-first-top-down-science-contracts.md) — the top-down science-contract authority model
**Reaffirms:** [ADR-0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) — comparators are flags, not targets
**Relates:** [ADR-0024](0024-reference-implementation-intent-authority.md) — reference-implementation intent as empirical anchor; [ADR-0003](0003-parity-semantic-not-bit.md) — parity is semantic, with named tolerance

## Context

ADR-0011 makes the **science contract** the correctness authority, derived
top-down from established science. ADR-0017 demotes the legacy comparator to a
flag. ADR-0024 allows reference-implementation *intent* to anchor an empirical
model where the literature is thin. Together these assume that *some* derivable
authority exists — a governing equation, a documented constant, a defensible
reference intent — from which a contract can be written.

Some modeling decisions have no such authority. The literature offers several
competing formulations with no consensus; the available reference implementations
are themselves imperfect and contested (no "golden ticket"); and no single
equation or constant can be cited as authoritative. In that situation the existing
framework leaves two bad options: **stall** (there is no authority to derive a
contract from), or **default to porting a reference implementation** that is not
actually authoritative — which re-creates the comparator-as-target failure ADR-0017
exists to prevent.

Meanwhile openWEPP increasingly holds **observed-data corpora** and a
**forcing-robust evaluation rubric**: signature-based, multi-timescale, decomposed
so the failed mode is named, with comparators scored as flags and forcing-limited
absolute magnitudes reported but not verdict-bearing.

This was reached concretely in the snow/frost program: melt, densification,
precipitation-phase partition, and ablation have no uncontested authority; SNOBAL
and legacy WEPP both fail the rubric in places; yet a cross-climate observed corpus
(SNOTEL SWE/depth/density across five climates) discriminates cleanly between
candidate mechanisms — and showed that the mechanism the literature-thin path would
have rejected was, on the representative instrument, the best one. The team needs an
explicit rule for admitting a mechanism on that evidence.

## Decision

When a modeling decision lacks sufficient established scientific authority to
derive a contract top-down, a candidate mechanism **may be admitted** — authored
into a science contract, shipped opt-in, and eventually promoted to default — **on
the basis of measurable improvement against an observed-data evaluation rubric**,
provided **all** of the following hold:

1. **Defensible physics.** The mechanism rests on sound physical reasoning —
   conservation, dimensional consistency, a coherent process abstraction — even
   where no single citation establishes it. It is a physical model, not a
   black-box fit.
2. **Observed-data rubric improvement.** It improves robustness on a rubric built
   from **forcing-robust signatures** (relative/intensive quantities that survive
   the forcing-and-representativeness uncertainty budget), scored by a
   **decomposed** metric that names the failure mode rather than a single aggregate
   tolerance. Forcing-limited absolute magnitudes are reported, never promotion
   verdicts.
3. **No calibration to the evaluation set; generalization required.** Constants
   come from physical reasoning or independent sources, never fitted to the
   validation corpus. The corpus is a held-out test set, not a training set. The
   mechanism must improve **across the corpus's regimes** (e.g., the full climate
   span), not win on a narrow, unrepresentative subset.
4. **Comparators stay flags (ADR-0017).** Reference implementations are scored on
   the same rubric as diagnostic flag profiles, never targets or ceilings. Beating
   the references is admissible evidence; "the reference does not do this" is not
   grounds for rejection, and "match the reference" is not grounds for promotion.
5. **Contract-first and opt-in until validated (ADR-0011).** The mechanism is
   authored into a science contract — process abstraction, invariants, the
   admission evidence, the rollback boundary — before production; it ships opt-in
   with prior behavior preserved as rollback; promotion to default requires the
   contract's stated no-regression gates.
6. **Conservation is non-negotiable.** Mass/energy closure (and any other
   governing conservation law) is a hard gate independent of the observed-data
   tradeoff. The admission authority chooses *among conserving mechanisms*; it
   cannot waive conservation.

This makes the observed-data rubric a **third, bounded tier of correctness
authority**:

- **derivable science contract** — preferred; use it whenever authority exists
  (ADR-0011) →
- **observed-data admission authority** — this ADR; when authority is
  absent/insufficient and a defensible corpus + forcing-robust rubric exist →
- **comparators / reference implementations** — flags only (ADR-0017), never
  authority.

## Scope

- **Applies** only when established scientific authority is genuinely insufficient
  to derive the contract **and** a defensible observed-data corpus + forcing-robust
  rubric exist.
- **Does not apply** where established authority exists (use it — ADR-0011); it
  does not license replacing a settled scientific authority with curve-fitting.
- **Domain-general.** This is a modeling-admission authority across the engine
  (hydrology, erosion, thermal/soil, plant growth, routing, etc.), not specific to
  any subsystem. The snow/frost fidelity rubric is the **first instance**, not the
  scope.

## Consequences

**Positive**

- Unblocks progress where the science is genuinely under-specified, instead of
  stalling or porting a non-authoritative reference.
- Makes the admission criterion explicit and auditable (defensible physics +
  forcing-robust rubric + no-overfit/generalization + conservation + contract-first),
  preventing ad-hoc "it looked better" promotions.
- Reinforces ADR-0017 by giving a principled alternative to comparator-as-target.
- Reusable across domains as the engine matures.

**Negative / risks (with mitigations)**

- *Overfitting / spurious improvement* → mitigated by the held-out-corpus,
  no-calibration, forcing-robust-signature, and cross-regime-generalization
  requirements (conditions 2–3).
- *"Defensible physics" is a soft bar* → mitigated by requiring the contract to
  state the physical reasoning and pass conservation, under the standard dual
  review.
- *Observed data carries its own uncertainty* → mitigated by the forcing-robust
  vs forcing-limited tiering (condition 2): only signatures that survive the
  uncertainty budget carry verdicts.
- *Misread as abandoning the literature* → bounded by Scope: the tier sits
  **below** derivable contracts and applies only when authority is absent.

## First instance

`GAP-SNOWFREEZE-002` / `INV-SNOWFREEZE-050` (the snow/frost forcing-robust
fidelity rubric) and the `tests/fixtures/snotel_observed/` cross-climate corpus
are the first application: they admit and score snow melt, density, partition, and
ablation candidates where SNOBAL and legacy WEPP are flags, not authorities. This
ADR generalizes that posture so later subsystems can reuse it without
re-litigating the principle.
