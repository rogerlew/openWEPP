# STAGE2-LATQCC-H2637-MAGNITUDE - WB19 Lateral/Subsurface Generation Magnitude

Status: scaffolded 2026-06-18 (Stage-2 physics-magnitude; the follow-on MAGPARITY01 routed here)

Package type: **Stage-2 physics-magnitude adjudication — produces a VERDICT, not a fix.** Unlike
MAGPARITY01 (which could rule out defects with *conservation* identities), this is a **magnitude**
question — conservation is magnitude-independent, so it must be judged against the **lateral-flow
physics** (WEPP Chapter 6 / Dun 2009) and `SC-SUBHYD-001`, with legacy a flag (ADR-0017). Output:
`OPENWEPP-DEFECTIVE` → a Defect-Closure ExecPlan (ADR-0018); `CORRECT` → the FARPOINT01 runoff flag
resolves; `UNRESOLVED` / `CONTRACT-GAP` → what authority/evidence would settle it. No fix here.

## Why this package exists (the carry-forward from MAGPARITY01)

MAGPARITY01 proved the runoff-**partition machinery** is correct on H2637 — `INV-RUNOFFPART-028`
transfer, area scaling, conservation closure, and export duality all hold at **machine epsilon**.
It localized the 71% outlet `runvol` to **routed lateral flow**: each OFE generates `latqcc`
~757k m³, Σ(OFE1-18) = 13,987,683 m³ routes to the outlet, OFE19 egresses 884,950 m³ as `sbrunv`.
The *combined* export is **75.5% (openWEPP) vs 72.2% (legacy)** — a surface-vs-lateral **partition**
difference, not gross over-generation. The one open question MAGPARITY01 could not answer with
closure: **is the lateral-flow generation magnitude (`latqcc`) physically correct?** That is this
package.

## The question

Does H2637's per-OFE `latqcc` **follow from** the WB19 lateral-flow physics (Eq [6.2.4] / Dun 2009
Eq 3a-3c) given its operands, and are those operands physically correct for H2637's forest soil?
I.e. is the lateral magnitude an honest output of the contract physics, or is a **driver inflated**
(e.g. the equivalent conductivity, the drainable thickness, or the FC-based drainable threshold)?

## Framing (non-negotiable)

- **Magnitude is judged against physics + external authority, not conservation and not legacy.**
  A magnitude is a **defect** only if it violates `SC-SUBHYD-001` (the flux equation, the operand
  domains) or is physically implausible under the Chapter-6 / Dun-2009 lateral-flow model — **not**
  because it differs from legacy.
- **Legacy is a flag (ADR-0017).** No "match 55.5% / 72.2%" target. Legacy with_ui is non-conserving
  (127.7%/152.6%) and disqualified; the bounded variant is a weak, like-for-like-only flag.
- **Do not re-open MAGPARITY01's closed results:** the transfer (`INV-RUNOFFPART-028`), area
  scaling, and export duality are settled; the surface-vs-lateral *partition* at the outlet is
  downstream of the generation magnitude this package adjudicates.

## Governing authority

- **`SC-SUBHYD-001`** (Subsurface Hydrology and Drainage) — the lateral-flow authority:
  - **`INV-SUBHYD-003`** lateral-flux: `q` per Eq [6.2.4], non-negative, finite conductivity/slope
    domains, no-flow branch at zero drainable thickness.
  - **`INV-SUBHYD-001/002/004`** daily continuity (Eq [6.2.1]), drainable-water state (`θd`, Eq
    [6.2.2-3]), drainable-thickness transition (Eq [6.2.5]).
  - **`INV-SUBHYD-012`** WB19 execution: deterministic `q` from layer-aware conductivity/geometry,
    excess-water withdrawal above `drfc_i = fc_i + (1-coca_i)·dg_i`.
  - **`INV-SUBHYD-018`** saturated-thickness response (the `cas_l4` suite below).
- **WEPP Chapter 6** Eq [6.2.1]-[6.2.5] (`references/50201000/chap6.pdf`) — the lateral-flow
  generating equations and drainable-storage definitions (`S, Ho, θd, θ, θFC, θa`).
- **Dun et al. (2009)** Eq 3a-3c (R-21, `references/annotated_bibliography.md`) — the **forest**
  Darcy lateral flux: equivalent hydraulic conductivity × drainable thickness × slope gradient over
  hillslope length.
- **`cas_l4_subhyd_lateral_saturated_thickness_response_001`** — the Level-4 external-authority
  suite (`q` increases with saturated thickness). Note it tests **response monotonicity, not
  absolute magnitude** — see the contract-gap lead.
- **`SC-PERC-001`** (vertical percolation) — the drainage interplay (lateral vs percolation split).
- The **forest `ksatadj` sat-fraction conductivity model** (not standard WEPP) — the likely
  equivalent-conductivity driver.

## Method — verify the equation, then judge the operands

From the existing closed H2637 run, extracting the **per-OFE WB19/subhyd operands behind `latqcc`**
(not just WAT/PASS aliases):

1. **Equation correctness (kernel):** does emitted `latqcc` equal Eq [6.2.4] / Dun 3a-3c evaluated
   on H2637's operands (equivalent conductivity, drainable thickness `Ho`, slope, length, `drfc`
   threshold, drainable water `θd`)? A mismatch is a kernel defect (`INV-SUBHYD-003/012`). A match
   reduces the magnitude question to the operands.
2. **Operand plausibility (the magnitude drivers):** are the operands physically correct for
   H2637's forest soil?
   - **Equivalent conductivity** via the forest `ksatadj` sat-fraction model — is it inflated vs the
     soil-file `Ksat` / restrictive-layer? (PRIME lead — an inflated equivalent K inflates `latqcc`.)
   - **Drainable thickness / saturated-thickness response** (`Ho`, Eq [6.2.5]; `INV-SUBHYD-018`) —
     computed correctly, bounded by the restrictive layer?
   - **The `drfc = fc + (1-coca)·dg` threshold** — the FC-based drainable threshold; ties to the
     profile FC/WP lineage. Is `fc` correct, so the drainable water `θ - drfc` (and thus `q`) is right?
3. **Same-unit legacy flag (dimensionally careful):** compare openWEPP's WB19 lateral operands and
   `latqcc` vs legacy's, **like-for-like** (prove the cut-points are dimensionally identical — the
   comparator-surface-artifact hazard; a suspiciously round delta is a unit smell). Legacy is a flag,
   not a target.
4. **Verdict** (ADR-0017 taxonomy): `CORRECT` / `OPENWEPP-DEFECTIVE` / `LEGACY-DEFECTIVE` /
   `UNRESOLVED` / `CONTRACT-GAP`, each citing the invariant/equation/authority.

## Leads to test (surface, do not pre-decide)

- The **forest `ksatadj` equivalent conductivity** is the most likely magnitude driver — verify the
  sat-fraction model produces a physically defensible equivalent K, not an inflated one.
- The **`drfc` FC-based threshold** governs drainable water; an FC error propagates into `latqcc`.
  (Note: the earlier "FC 2× too low" sub-diagnosis was **withdrawn/wrong** — do not pre-load it;
  re-derive FC's correctness from `SC-PERC-001`/soil authority if it enters the chain.)
- **Lateral vs percolation split** (`SC-SUBHYD-001` vs `SC-PERC-001`): is water that should percolate
  vertically instead exiting laterally (or vice-versa)? The combined-export closeness (75.5% vs 72.2%)
  suggests total drainage is similar; the open question is the *partition* of drainage into lateral.
- **Contract-gap candidate:** the existing `cas_l4` suite tests lateral-flux *response* (monotonic in
  saturated thickness), not *absolute magnitude*. If `latqcc` is equation-consistent and operand-plausible
  but its absolute magnitude still cannot be judged from existing authority, that is a **contract gap** —
  flag the need for an absolute-magnitude external-authority suite (do not invent a bound).

## Verdict + handoff

- **`CORRECT`** — `latqcc` follows from the physics with plausible operands: the **FARPOINT01 runoff
  flag resolves** (the 71% is correct forest hydrology); document the resolution + close the flag.
- **`OPENWEPP-DEFECTIVE`** — a kernel or operand defect (cite the invariant/equation + the inflated
  driver): author a **Defect-Closure ExecPlan** item-1 (ADR-0018) — no fix here.
- **`UNRESOLVED`** — equation/operands check out but absolute magnitude is unjudgeable from available
  authority: state the specific external authority/data needed.
- **`CONTRACT-GAP`** — flag an `SC-SUBHYD-001` amendment / a new absolute-magnitude external-authority
  suite; do not invent a bound.

## Scope

In scope: the H2637 far-point fixture; per-OFE WB19/subhyd operand extraction; the equation-correctness
+ operand-plausibility adjudication; the verdict + handoff.

Out of scope:

- **No code/physics fix** — defects route to a defect-closure follow-on.
- **No "match legacy" target**; no use of legacy with_ui.
- **No re-opening** `INV-RUNOFFPART-028` / area scaling / export duality (MAGPARITY01-settled).
- No `SC-*` change (a gap is *flagged*). No conservation re-litigation. Irrigation deferred.

## Acceptance Criteria

- **Per-OFE WB19 operand extraction** behind `latqcc` (equivalent conductivity, `Ho`, slope, `drfc`,
  `θd`), from the closed H2637 run — not WAT/PASS aliases.
- **Equation-correctness check:** emitted `latqcc` vs Eq [6.2.4]/Dun 3a-3c on those operands, with
  residual + verdict (`INV-SUBHYD-003/012`).
- **Operand-plausibility verdict** per driver (conductivity / drainable thickness / `drfc`), citing the
  governing invariant + external authority (Chapter 6 / Dun 2009), not the legacy delta.
- **Like-for-like legacy flag** with a dimensional-consistency proof of the cut-points.
- **Verdict** (ADR-0017 taxonomy) + **handoff** (defect-closure ExecPlan / FARPOINT01 flag resolution /
  UNRESOLVED evidence list / contract-gap flag).
- Evidence labeled Static / Ran. Markdown lint clean. (No Rust gates unless code touched — none expected.)

## Deliverables

- `artifacts/latqcc-operand-extraction.md` (per-OFE WB19 operands behind `latqcc`)
- `artifacts/latqcc-equation-correctness.md` (emitted `latqcc` vs Eq [6.2.4]/Dun 3a-3c)
- `artifacts/latqcc-operand-plausibility.md` (conductivity/`ksatadj` / drainable thickness / `drfc`)
- `artifacts/latqcc-legacy-flag.md` (like-for-like, dimensional-consistency proof)
- `artifacts/latqcc-per-driver-verdict.md` (ADR-0017 taxonomy, per driver + overall)
- `artifacts/latqcc-handoff.md` (defect-closure ExecPlan / flag resolution / UNRESOLVED / contract-gap)
- `artifacts/latqcc_disposition.md`

## Dependencies

- `docs/work-packages/20260618-mofe-magparity01-per-ofe-runoff-magnitude-adjudication-001/artifacts/{magparity01-runoff-decomposition,magparity01-external-authority-plausibility,magparity01-handoff}.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` (INV-SUBHYD-001/002/003/004/012/018)
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`; `SC-INFILE-GWCOEFF-001.md`
- `docs/specifications/external-authority/suites/cas_l4_subhyd_lateral_saturated_thickness_response_001.md`
  (+ `cas_l3_subhyd_solwpv_fcdep_branch_001`, the other `cas_l4_subhyd_*` suites)
- `references/50201000/chap6.pdf` (WEPP Ch.6 Eq [6.2.1]-[6.2.5]); `references/annotated_bibliography.md` (R-21 Dun 2009, Srivastava 2013)
- The forest `ksatadj` conductivity model + the FC/WP profile lineage (soil-file authority)
- `docs/decisions/0017-...comparator-is-flag-not-target.md`, `0018-defect-closure-execplans-conversion-rule.md`, `docs/defect_closure_execplans.md`
- `docs/specifications/correctness-authority-model.md`; `docs/ROADMAP.md` (Stage-2 ordering)
- `AGENTS.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the per-OFE operand extraction across the 19 OFEs
and the per-driver plausibility checks are parallelizable. Record command evidence.

## Autonomy

Execute end-to-end through operand extraction, the equation-correctness check, per-driver operand
plausibility, the like-for-like legacy flag, the verdict, and the handoff. The verdict is the
deliverable — render it from the lateral-flow equation + operand authority + external plausibility,
legacy a flag only. `CORRECT` (flag resolves), `OPENWEPP-DEFECTIVE` (clean defect-closure handoff),
`UNRESOLVED`, and `CONTRACT-GAP` are each valid complete outcomes. Do not fix, and do not chase legacy parity.
