# MOFEFID-DC01 — Runon Re-Infiltration Defect Closure (ExecPlan)

Status: **IN EXECUTION** (2026-07-01)
Shape: **Defect-Closure ExecPlan** (ADR-0018; `docs/defect_closure_execplans.md`).
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md).
Owner: Claude Code (operator-authorized 2026-07-01: "yeah, that is a win.
scaffold and execute"). Reviewer at close: Codex. Worktree: `mofefid-dc01`.

## 1. Objective

Close defect **DC01-RUNON-REINFIL** (= A01 finding F-A2, confirmed
first-order by the A02 probe): openWEPP excludes inter-OFE runon from the
WB14 infiltration supply, opposite to the pinned-baseline source intent
(`wepp-forest_260430_baseline/src/watbal_hourly.for:361-363` daily `fin`,
`:471-473` hourly `xfin += (ui_LfUrf + ui_SUrunf) × area-ratio`). Observed
consequence (Ran, A02): H2637 `runvol_pct_precip` 72.33% vs legacy ~55.5%,
with a lower-bound probe recovering −10.1 pp. This package lands the
contract-first correction as **production default semantics** — not the
probe's opt-in approximation.

## 2. Correction Authority Envelope

**Defect ID / observed violation:** DC01-RUNON-REINFIL — on any multi-OFE
lane receiving runon, WB14 cumulative infiltration is invariant to runon
(observable: constructed 2-OFE test where downstream infiltration is
identical with and without upstream transfer; H2637 magnitude signature
per A02).

**In-scope write-set:**
- Contracts: `SC-RUNOFFPART-001.md` (new invariant + REF anchor; revision
  entry), `SC-WATBAL-001.md` (touchpoint annotation only).
- Source: `crates/openwepp-hillslope-orchestrator/src/direct_runtime/
  {runoff.rs, 03_executor.rs, erosion.rs (boundary-limited, §branch-out)}`;
  runner day-input/transfer plumbing files if the hourly profile requires
  them.
- Deletion: the A02 probe env-flag machinery (superseded by the default
  correction; keeping both paths is the dual-representation smell).
- Tests: orchestrator/runner unit + fixture tests for the new semantics.

**Authorized evidence surfaces:** H2637 (closure + magnitude-as-flag),
single-OFE fixtures/suites (byte-identity gate), constructed 2-OFE unit
fixtures, `tools/owcmp` MOFE suite (comparator **flag**, ADR-0017).

**Allowed production-edit classes:** WB14 supply-side composition (hourly
runon admission incl. dry-runon interval synthesis); transfer publication
of an hourly surface-runoff profile; R4J/R4A wiring consistent with the
unchanged partition identity; erosion `qin` boundary handling (clamp +
counter only). **Not allowed:** changes to the partition identity itself,
frost solve, snow authority, publication schemas, or watershed tier.

**Acceptance criteria (falsifiable, behavior-level):**
1. Constructed 2-OFE test: downstream WB14 infiltration includes the
   area-scaled runon share with exact expected values; dry-runon day
   infiltrates (synthesized intervals).
2. **Single-OFE runs byte-identical** (zero-runon specialization must be
   untouched — the anti-regression anchor now that MOFE identity
   intentionally breaks).
3. H2637: exit 0, **all closure guards green** (anti-tautology: the
   ledger-vs-state guard is state-basis, independent of the flux ledger).
4. `INV-RUNOFFPART-031` (new) evidence lineage: supply composition
   traceable to the baseline REF anchor.
5. Magnitude recorded as **flag, not gate**: H2637 `runvol_pct_precip`
   expected to move toward legacy (~62% per the A02 lower bound; the
   hourly-faithful value will differ); Lane C's observed envelope is the
   eventual magnitude bar, explicitly out of this package.
6. Full workspace suite green; fmt/clippy/deny clean; endpoint wall within
   the ≤5× budget.

**Branch-out / negative boundaries:**
- **Erosion case machinery:** decreasing-flow (`qout < qin`) sediment
  cases (legacy Ch4 cases 3/4) remain in the `INV-RUNOFFPART-030` hold.
  This package only replaces the hard `DomainViolation` with a
  hold-consistent bounded clamp (`qin := min(qin, qout)`) plus a run-level
  occurrence counter surfaced in the manifest, and annotates the hold.
  Full case rework routes to the hold-closure package (with F-A1).
- **Lane C/D:** magnitude adjudication (C) and hydrograph routing (D)
  untouched.
- **wepppy/release coordination:** output values change on MOFE runs;
  operator owns downstream sequencing.

## 3. Conversion rule (restated)

This package converts A01/A02's diagnosis into a landed correction in one
pass. Its terminal states are: (1) landed contract-first correction
validated by the acceptance criteria; (2) validated non-defect (not
plausible — the A02 probe already demonstrated materiality); or (3) an
owned HOLD at a declared boundary below. "More diagnostics" is not a
terminal state.

## 4. HOLD-legitimacy conditions

- **H-1:** an hourly-faithful upstream surface profile proves impossible
  without hourly runoff machinery openWEPP lacks, AND all bounded
  approximations (§5 M2 options) fail conservation or the single-OFE
  byte gate → HOLD; route the profile design to Lane D (which builds
  hydrograph routing anyway); the daily-supply variant may still land if
  it passes all gates.
- **H-2:** the erosion seam cannot accept decreasing flow even under the
  bounded clamp without violating held invariants → HOLD the erosion
  edge inside `INV-RUNOFFPART-030`; the water correction may still land
  (the clamp is the declared boundary).

## 5. Milestones

- **M1 — Contract first.** `REF-RUNOFFPART-BASELINE-FIN-XFIN` (baseline
  source-intent anchor, ADR-0024) + `INV-RUNOFFPART-031`: the WB14 event
  liquid supply on a lane with upstream transfer must include the
  area-scaled upstream surface and lateral carry, distributed over the
  event time base; dry-runon days must still present a time base.
  SC-WATBAL touchpoint note; INV-030 hold annotation for the erosion edge.
- **M2 — Hourly surface profile (shadow).** WB14 emits a per-hour
  infiltration-excess profile; combined with `hourly_saturation_carry`
  and normalized to `q_runoff_m` (mass-exact by construction) it becomes
  the published `surface_carry_m` 24-slot content (replacing the slot-0
  lump). Gate: byte-flat everywhere (R4J consumes only the sum).
- **M3 — Supply admission (identity breaks by design).** WB14 consumes
  `(surface_carry + lateral_carry) × upstream_area_ratio` per hour mapped
  onto its interval basis; synthesize 24×1 h intervals when the local
  hyetograph has no positive duration. Delete the A02 probe flag. Gates:
  acceptance 1–4.
- **M4 — Erosion boundary.** Bounded clamp + manifest counter + hold
  annotation (branch-out above).
- **M5 — Validation sweep.** Acceptance 2/3/5/6 full runs; magnitude memo;
  comparator-flag run on the owcmp MOFE suite; timing.
- **M6 — Review/disposition.** Codex review; dispositions; campaign doc +
  ROADMAP updates.

## 6. Defect-shaped handoff (if any)

If M-stages hold: the handoff names the *defect* ("downslope OFEs do not
re-infiltrate runon; supply composition fails at <boundary>"), never an
inspection step. Expected live handoffs regardless of success: the
erosion decreasing-flow case rework (to the INV-030 hold package) and the
magnitude adjudication (to Lane C).

## Progress

- 2026-07-01: scaffolded; M1 starting.

## Surprises & Discoveries

- (running)

## Decision Log

- Hourly profile normalized to `q_runoff_m` (not raw WB14 excess) so the
  transferred surface water equals today's transferred total exactly —
  M2 stays byte-flat and mass-exact.

## Outcomes & Retrospective

- (at close)
