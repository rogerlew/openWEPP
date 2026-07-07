# Review - Claude (Cross-Arc)

Status: COMPLETE. Evidence mode: Static.

Reviewer lane: Claude Code (cross-WP continuity). Scope: the post-approval
hybrid arc `977bdad5..6d43ce07` plus this uncommitted package. All source,
contract, and artifact reads were performed directly; all run numbers quoted
below are from package-recorded Codex runs (attributed evidence, not re-run
here).

## Verdict

GO on this package as executed. The hold is legitimate, the timing diagnosis
is counter-discriminated rather than endpoint-only, and no code/contract
change exists to gate. Nothing below blocks committing this package as-is.

All findings target the named follow-on package
(`20260707-laned-router-d16-hybrid-noharm-selector-solvecost-hold-lift-001`)
before it is scaffolded, plus two record-hygiene items. Disposition of each
finding is left to the follow-on author/executor.

## Findings

### CL-H1 (High, follow-on scope design): the stated success target is
### satisfiable by a selector-only outcome that forfeits the fleet prize

- Evidence: `artifacts/timing-profile-adjudication.md`. The two forest
  members are the generic-map payers (WA `98192634` map evals at
  `196.17`/implicit-step; N Idaho `1364.68`/step). H2637's win rides the
  exact bare-skin evaluator (`0` map evals) — which vegetated surfaces
  cannot use by definition (`SC-OFEROUTE-002` §Algorithm 3 exact bare-skin
  guard: no active form/wave/vegetation/Manning addend). The program is
  forest-first (README "Scientific orientation"), so the fleet weight sits
  on the WA-shaped cases, not the H2637-shaped ones.
- The follow-on success target in `artifacts/worker-handoff.md` (no member
  regresses; aggregate beats plain) is arithmetically satisfiable by
  routing plain everywhere except H2637:
  `33.62 + 0.56 + 0.96 + 15.65 = 50.79 s` vs `57.34 s` plain (about
  `-11.4 %`). That clears the stated gate while forfeiting WA's `-33.11 %`
  step reduction — the largest step-savings in the cohort — and leaves the
  fleet-weighted value mostly unclaimed.
- Finding: as written, the success target under-specifies the objective.
  The follow-on package text should state explicitly whether non-bare
  implicit solve-cost reduction (lever 2) is the primary objective with the
  no-harm selector as the safety net, or whether selector-first staging is
  intended with a declared boundary that WA-class wins remain open. Either
  is defensible; what must not happen is a selector-only pass silently
  reading as "hybrid viability closed."

### CL-H2 (High, determinism guard): constrain the admissible predictor
### input classes before implementation

- `SC-OFEROUTE-002#INV-OFEHYB-003` requires the converged cell state to be
  a pure function of (cell parameters, `rhs`, `Δt/Δx`, branch), independent
  of run history; the D16-family package exclusions additionally ban
  branch-history-dependent seeding. An "adaptive/preflight" selector
  invites clock-derived or measured-cost-derived inputs (wall time,
  iteration counts observed mid-run), which would make routing — and
  therefore published output — machine- and load-dependent.
- Finding: the no-harm predictor must be a pure function of run inputs —
  static cover/friction-class composition per lane, source structure, mesh,
  and counters *predicted* from those; never quantities *measured* during
  the run. The follow-on's contract-first amendment should name the
  admissible input classes before code. Secondary: parameterize any
  thresholds by mesh/counter quantities rather than tuned constants, so
  Tier-2 mesh adjudication re-calibrates the rule instead of invalidating
  it.

### CL-M1 (Medium, governance sequencing): predeclare promotion tolerances
### before tuning evidence runs

- `artifacts/viability-levers.md` lever 3 already states "Define promotion
  tolerances before further tuning"; `artifacts/worker-handoff.md` carries
  predeclaration only as a success target (an end-state), not a sequencing
  constraint.
- Risk: tolerances ratified after selector tuning can be fit to whatever
  the tuned selector produces — the mirror image of the no-pre-filled-
  evidence rule.
- Finding: sequence the follow-on so the `SC-OFEROUTE-002` §Tolerance
  amendment (routed outlet, hydrograph shape/peak, HBP semantic deltas,
  pass-sediment, closure surfaces; ratified cohort-wide, not H2637-only)
  lands as a contract-first phase before any tuning evidence is produced.

### CL-M2 (Medium, unattributed fidelity structure): the delta spread is
### unexplained and shape-signed

- Evidence: `artifacts/timing-profile-adjudication.md` closure table. H2637
  outlet delta `-0.43957 %` vs `-0.014109 / -0.005746 / -0.012918 %` for
  the other three — a roughly 30x spread despite comparable implicit
  engagement (`980804` vs `500560` implicit steps, cooldown active in
  both). The delta is case-structure-dependent, not proportional to
  implicit coverage; no artifact names the mechanism. Separately, H2637's
  `-6.474 %` pass-sediment sum against its `-0.43957 %` outlet indicates
  hydrograph-shape change (sediment integrates peak-sensitive detachment)
  even where volume nearly closes.
- Finding: before tolerance ratification (CL-M1), run a single-run
  first-divergent-day/OFE attribution on H2637 plain-vs-hybrid (primary
  debugging lane; no oracle binary required) and name the mechanism class —
  residual implicit smearing in legitimately-quiet bins, bare-skin
  equilibrium dust accumulation, switching-boundary artifacts, or other.
  Tolerances chosen before the mechanism is named would cover an unnamed
  defect class. This attribution is also where lever 4 (erosion-sensitive
  eligibility) is either justified or ruled out.

### CL-L1 (Low, record hygiene): stale template line in a closed package

- `../20260707-laned-router-gap-ofehyb-001-hold-lift-design-001/artifacts/final-disposition.md`
  ends with "Final outcome is not yet available." — contradicting its
  EXECUTED-COMPLETE status. Delete the line in the next commit touching
  that package.

### CL-I1 (Informational, watch item — no action in this package)

- The canhgt publication package left a recorded consumer split: Lane D
  routing consumes daily post-growth canopy height (`SC-PLANT-001` rev 19;
  `SC-OFEROUTE-001` rev 36), while the broader Wave-1 erosion consumer move
  was attempted, changed `erosion_single_ofe_p61_sediment` materially, and
  was intentionally reverted (tracked in that package's worker-handoff).
  Two consumers of the same physical quantity now read different surfaces.
  This was correctly bounded and recorded; keep it visible when scaffolding
  follow-ons so it is not rediscovered later as an unexplained divergence.

## Review Notes

- The hold decision is well-evidenced: the discriminator is profile-counter
  based (H2637 `0` map evals wins; WA `98.2 M` loses), not endpoint-timing
  inference.
- The six-hold D16 chain is the intended defect-closure shape working: each
  hold named a real boundary, and the chain converted two boundaries into
  durable capabilities (Disturbed native `ow-lanuse-1` route-coefficient
  authority in WEPPpy; row-crop daily `canhgt` publication — the latter
  fixes an active-runtime defect independent of the hybrid).
- The canhgt Lane-D friction-operand re-point was checked for contract
  backing during this review and found clean and correctly bounded
  (`source-authority.md`: baseline `grow.for`/`initgr.for`/`frcfac.for`
  lineage; contract amendments before code; the out-of-scope erosion
  consumer change reverted rather than smuggled in).

## Residual Risk

- Sharpening the Codex lane's cohort-size note: the selected cohort is not
  merely small — it is likely *favorably* skewed for the hybrid, because
  the one member with full bare-skin coverage (H2637) carries the
  aggregate. The forest fleet resembles WA/N Idaho (generic non-bare
  implicit work), so the fleet-weighted expected value of the current
  hybrid at current solve cost is negative-to-neutral. Cohort growth for
  tolerance ratification should weight forest members accordingly.
