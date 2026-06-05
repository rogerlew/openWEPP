# Claude Code Review Findings — HPHYS0296

Reviewer: Claude Code (independent review; serving as the de facto independent
review while subagent dual-review delegation is gated).
Verdict: **HELD THE LINE — no acceptance magic was exercised — but the contract
acceptance gate it authored is too permissive and must be strengthened before
any window is accepted.** Honest triage, not yet science.
Evidence mode: static (contracts, classification artifact, diff) + ran (metric
cross-check, no-production-patch confirmation).

## End goal (the bar this work must clear)

Acceptance is **not** the goal. The goal is to **definitively determine, per
divergent window, the root cause of the openWEPP-vs-baseline divergence and which
model is defective.** A residual may only leave the failing set when it carries a
proven defective-model verdict. "Semantic-not-bit divergence, accepted, cause not
definitively root-caused" is the magic outcome and is prohibited. The deliverable
of this thread is a **defect ledger**, not an acceptance bucket.

## What HPHYS0296 did (verified)

Ran / static:
- No production physics change (contracts + one integration test + artifacts).
- No acceptance exercised: full semantic pass remains `0/39`; no rows excluded,
  re-tiered, or removed; `Q` remains closed `39/39`.
- No downstream WB17/WB18/WB19/WB13 compensation.
- Classification produced: `6` corrected-negative-melt **candidates (pending
  review)** and `3` producer-magnitude/timing **holds** (spring-2016 windows
  with no material negative raw melt).
- Refused to wave away the spring-2016 windows — correctly classified them as
  genuine producer debt rather than bulk-accepting the snow/`RM` residual. This
  is the partition working and is the strongest signal in the package.

This is good discipline: the package triaged and stopped short of magic.

## Findings

### CLAUDE-0296-001 [HIGH] — Acceptance gate is correlational, not root-cause proof

The new gate (`SC-SNOWFREEZE-001#INV-SNOWFREEZE-027`, `SC-WATBAL-001#INV-WATBAL-071`,
`SC-RUNOFFPART-001#INV-RUNOFFPART-024`) permits accepting a residual when trace
evidence "shows material negative raw hourly melt … sufficient to explain the
comparator residual while preserving internal snow-state closure, `RM`
publication identity, and non-negative domains."

This is half-science:
- It correctly forces the accept/hold partition and forbids downstream
  compensation in both branches.
- But "sufficient to explain" is asserted by the **presence** of negative melt
  (correlation), not **proven** by reconstruction. And every acceptance
  condition is *internal self-consistency* (snow-state closure, `RM` identity,
  non-negative domains) — none establishes that openWEPP's value is *correct*
  against a non-legacy authority. Internal `RM identity abs = 0.000000` means
  openWEPP agrees with itself, not that it is right.

As written, the contract permits promoting the 6 candidates to "accepted" on
"contains negative melt + internally closed." That does not determine root cause
or which model is defective. It must be tightened (see Acceptance Criteria).

### CLAUDE-0296-002 [HIGH, process] — Governance contract amended unreviewed

`review_agent_a.md`/`review_agent_b.md` = `not-run` ("tool policy requires
explicit user authorization for subagent delegation"). This is the fifth
consecutive package (0292–0296) without dual review, and 0296 is the one that
amended three canonical contracts to create a new **acceptance-authority** gate.
A "you may accept divergence" invariant entering the contracts unreviewed is the
worst case for the gap. Appears to be a harness constraint (subagent delegation
gated), not a Codex choice; restoring the gate likely requires authorizing
subagent delegation. Until then, contract amendments rest on a single author's
judgment.

## Acceptance Criteria (binding before any window leaves the failing set)

Each divergent window must produce a **defective-model verdict** supported by all
of A–F. Windows lacking any item remain `HOLD` and stay in the failing set.

- **A. Mechanistic root cause (both models).** Identify the specific divergence
  origin by `file:line` in *both* openWEPP and the pinned baseline (e.g.,
  "divergence originates where legacy `winter.for:NNN` applies sign-error path S
  and openWEPP applies corrected path S'"). Not "the window contains negative
  melt."
- **B. Reconstruction (controlled experiment).** Reproduce the baseline value to
  named tolerance by injecting the identified legacy path into openWEPP (and/or
  reproduce openWEPP's value by applying the corrected path to legacy inputs).
  This proves the divergence is caused *by and only by* the identified
  mechanism. Falsifiable: if the baseline number cannot be reconstructed from the
  claimed mechanism, the root cause is wrong and the window stays open.
- **C. Independent correctness adjudication (non-legacy authority).** Decide
  which model is correct using mass/energy conservation, the documented WEPP
  reference equation, the corrected `wepp-forest` fix's own derivation/provenance,
  or external data — never "openWEPP differs from baseline." This yields the
  defective-model verdict.
- **D. Per-window disposition — exactly one of:**
  - `LEGACY-DEFECTIVE` → openWEPP correct → re-tier the window (documented,
    still gated by conservation/bounds); residual not counted against openWEPP.
  - `OPENWEPP-DEFECTIVE` → window stays failing; fix the producer.
  - `UNRESOLVED` → remains `HOLD`; **may not be accepted.** No "accepted, cause
    unknown."
- **E. Scope discipline.** Only windows with a complete A–D verdict move. No bulk
  bucket acceptance; the failing set shrinks one root-caused window at a time.
- **F. Auditable re-tiering, not deletion.** `LEGACY-DEFECTIVE` windows are
  demoted to a named "legacy-defective, documented" comparator tier with the B/C
  evidence linked. Semantic-pass accounting must distinguish three states:
  `matches` / `documented-legacy-defective` / `openWEPP-defective-open`. Rows are
  never silently excluded.

### Contract amendment required
Before any acceptance, amend `INV-SNOWFREEZE-027` (and the `WATBAL`/`RUNOFFPART`
companions) so the acceptance precondition is A–D above, replacing "shows
material negative raw melt … sufficient to explain + internal closure." As
currently worded the gate will pass correlational acceptance.

### Legitimate end state
A rigorous outcome may be that the suite **never reaches 39/39 against this
baseline** — ending instead at `X/39 matches`, `Y/39 documented-legacy-defective`
(with reconstruction evidence), `Z/39 openWEPP-defective-open`. That is success,
not failure. If 39/39-against-legacy is treated as the target, the oracle was
never actually demoted, and any jump in semantic pass without A–D evidence per
accepted window should be treated as a regression in discipline.

## Disposition of the 9 classified windows under these criteria
- 3 spring-2016 `producer-magnitude/timing-hold`: correctly `OPENWEPP-DEFECTIVE`
  / `UNRESOLVED` — stay failing, fix the producer. No change needed.
- 6 corrected-negative-melt `candidates`: **not yet acceptable.** They currently
  satisfy only correlation (A partial) + internal closure. They require B
  (reconstruction) and C (independent authority) before any can be dispositioned
  `LEGACY-DEFECTIVE`. Until then they remain in the failing set.

## Bottom line
0296 held the line and produced an honest partition — no magic was exercised. But
it wrote an acceptance gate that would permit correlational acceptance, and did so
unreviewed. Tighten the gate to A–F before the next package, and require that
every window that ever leaves the failing set carries a proven defective-model
root-cause verdict. The end goal is a defect ledger that names, per window, which
model is wrong and why — not a bucket of "accepted divergence."
