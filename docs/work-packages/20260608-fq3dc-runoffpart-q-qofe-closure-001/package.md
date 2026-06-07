# FQ3-DC RUNOFFPART Q/QOFE Underproduction Closure

Status: complete

Package type: Defect-Closure ExecPlan (DC-ExecPlan)

## Objective

Close defect `FQ3-DC-RUNOFFPART-QQOFE-001` end-to-end: openWEPP produces surface
runoff `Q`=0 (and `QOFE`=0) on `35/42` post-FQ1 `/wc1/runs/al/algebraic-radium`
single-OFE hillslopes where the legacy comparator runs off materially (p8 corn
`Q`=760 mm, p1 tah `Q`=278 mm over 7 yr), with 7 further Corn cases nonzero but
materially below legacy. Make openWEPP's infiltration-excess runoff partition
engage so runoff is produced when rainfall intensity exceeds infiltration capacity,
per `SC-RUNOFFPART-001`, validated on the algebraic-radium population.

This package owns correction inside the runoff-partition envelope. If the root
cause is in-envelope and authority-backed, it must land the contract-first fix.

## Run / sibling context

Parallel to `FQ3-DC-ET-CORN-ENGAGEMENT-001` (annual-crop ET, dispatched
separately). `Q`=0 is **universal across managements** — it hits perennial Tah
(p1) and Corn alike — so it is a **distinct root cause** from the corn-canopy ET
defect (which only affects Corn). Both come from FQ-3
(`docs/work-packages/20260608-fq3-et-runoff-zero-term-characterization-001/`).

## Rationale (FQ-3 evidence, comparator-flag confirmed)

FQ-3 + Claude verification (legacy `wepp_260606` WAT vs openWEPP post-FQ1):

- p8 (Corn): legacy `Q`=760 mm, openWEPP `Q`=0.
- p1 (Tah): legacy `Q`=278 mm, openWEPP `Q`=0.
- 35/42 prefixes `defect-openwepp-zero-legacy-nonzero`; 7 Corn nonzero-but-below.

Legacy produces runoff on the **same daily gridmet climate**, so the runoff
mechanism is reachable on this forcing and openWEPP is under-producing — an
openWEPP defect, not invalid input. This is a magnitude/partition defect: the
water is conserved (rung-1 holds) but routed to infiltration/storage instead of
runoff.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `FQ3-DC-RUNOFFPART-QQOFE-001`
  - Observable: `Q`/`QOFE` = 0 (or << legacy) on 35/42 single-OFE prefixes;
    fixture `/wc1/runs/al/algebraic-radium/wepp/runs/`, post-FQ1 corpus
    `/tmp/fq1_after/outputs`, legacy `/tmp/fq3_exec/legacy/outputs`.
  - Per-prefix classification in FQ-3 `per_prefix_term_classification.csv`.

### In-Scope Contracts and Source Files

- Contracts:
  - `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
    (rainfall-excess partition, runoff onset, depression storage — primary).
  - `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md` **only if**
    Milestone 1 proves the cause is the daily-storm intensity/hyetograph
    disaggregation feeding the partition (then amend/branch accordingly).
  - `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` as the
    downstream `Q` consumer (closure must stay closed after runoff is produced).
- Production/test files:
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
    (rainfall-excess / hyetograph-liquid-input / saturation-runoff partition).
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
    (`q_runoff` computation, ~`:4651`).
  - `crates/openwepp-runner/src/hillslope/mod.rs` (`Q`/`QOFE` publication) only for
    publication-coupling surfaces.
  - SIMIMPL28 hourly forcing (`runtime_inputs/06_simimpl28_hourly_forcing.rs`)
    **only if** the proven cause is storm-intensity disaggregation.
  - `tests/integration/**runoff**.rs`, `**fq3dc**.rs`, `**watbal**.rs`.
  - `docs/work-packages/20260608-fq3dc-runoffpart-q-qofe-closure-001/**`,
    `docs/work-packages/README.md`.

### Allowed Edit Classes

- Amend canonical `SC-RUNOFFPART-001` (or `SC-CLIMATE-001` storm-intensity, if
  proven) for the corrected runoff-generation behavior before production code.
- Correct the infiltration-excess partition / depression-storage / runoff-onset /
  intensity-input path so runoff is produced when intensity exceeds capacity.
- Add contract-derived tests (a known intense-rain day producing nonzero excess;
  the 7 under-producing Corn cases; non-regression on terms that are correct).
- Add bounded diagnostics to localize where excess is lost.

### Protected Boundaries (do not cross)

- **No comparator-match acceptance.** `wepp_260606` is a flag that runoff should be
  nonzero (ADR-0017); acceptance is contract-correct infiltration-excess
  generation, not matching legacy `Q` magnitude.
- Conservation must stay closed — do not create runoff by breaking the water
  balance; the rung-1 closure (incl. interception, snow) must still hold.
- Snow magnitude remains a Stage-2 protected boundary.
- Do not touch annual-crop ET engagement (that is
  `FQ3-DC-ET-CORN-ENGAGEMENT-001`); do not touch `p11` percolation (`FQ1-P11`).
- The 17-OFE hillslope (MOFE) is out of scope (rung-3).

### Acceptance Criteria

- openWEPP produces nonzero infiltration-excess `Q`/`QOFE` on the affected
  single-OFE hillslopes when rainfall intensity exceeds infiltration capacity,
  consistent with `SC-RUNOFFPART-001` (legacy nonzero as a flag, not a target).
- The water-balance closure (rung-1 identity + totalwatsed3 audit) still closes
  after runoff is produced.
- Contract-derived red/green tests; pre-implementation failing evidence; post-fix
  validation over the affected population + non-regression on correct terms.
- No conservation break, comparator-target tuning, silent default, or downstream
  compensation.

### Branch-out Boundaries

- If Milestone 1 proves the cause is upstream daily-storm intensity/hyetograph
  disaggregation (a climate-forcing surface), amend `SC-CLIMATE-001` in-package if
  it is the partition's intensity input, or branch a defect-shaped climate target
  if it is a broader climate-disaggregation defect.
- If a subset of hillslopes legitimately produce ~0 runoff (legacy also ~0),
  exclude them with evidence.

## Conversion Rule

If a reproducible root cause is established inside the declared runoff-partition
envelope and the corrected behavior is supported by canonical `SC-RUNOFFPART-001`
(or proven `SC-CLIMATE-001`) authority, pinned-baseline provenance, or a
contract-authorized physical invariant (infiltration-excess generation), the
package must proceed through contract amendment → tests → pre-implementation gate →
production correction → validation → disposition. It may not close `HOLD` because
more investigation is possible.

## Seven-Gate Bar

All seven true ⇒ `HOLD` invalid, must land the fix: (1) reproduce `Q`=0 on an
affected prefix; (2) named mechanism (intensity disaggregation / Green-Ampt
over-infiltration / depression storage / onset / publication), not "trace deeper";
(3) ownership in the runoff write-set (legacy runs off on same climate); (4)
authority `SC-RUNOFFPART-001`/`SC-CLIMATE-001`/physical excess law, not comparator
match; (5) safety — no conservation break, no silent default; (6) testability —
red/green on an intense-rain day; (7) validation — affected-population runoff
measurable before/after.

## Symptom-Existence + Ownership Gate (Milestone 1, first)

1. Reproduce `Q`=0 on an affected prefix (e.g. p8) and pick a day where legacy
   produces runoff.
2. Localize where excess is lost: compare openWEPP's hyetograph/rainfall-intensity
   input, infiltration capacity, depression storage, and infiltration-excess on
   that day vs legacy. Name the mechanism — gentle storm-intensity disaggregation,
   Green-Ampt over-infiltration, depression storage, runoff onset, or
   computed-but-not-published.
3. Ownership: legacy `wepp_260606_hill` runs off on the same daily climate →
   openWEPP defect.

## Legitimate HOLD Conditions

- Mechanism outside the declared runoff/climate-intensity envelope (branch with a
  defect-shaped target).
- Canonical authority missing/contradictory.
- A subset legitimately produces ~0 runoff (legacy also ~0) — exclude with
  evidence.
- Required evidence cannot be generated in the environment.

Grind-HOLD (forbidden): "inspect the next partition variable," "trace runoff one
step deeper," "root cause in WB12 but implementation deferred."

## Milestones

1. Symptom-existence + ownership gate (above).
2. Contract: amend `SC-RUNOFFPART-001` (or proven `SC-CLIMATE-001`).
3. Contract-derived red tests (intense-rain day → nonzero excess; under-producing
   Corn cases; correct-term non-regression).
4. Pre-implementation gate evidence.
5. Production correction in the proven runoff (or intensity) surface.
6. Validation: affected-population runoff produced; WB closure still holds; correct
   terms non-regressed.
7. Dual review, finding disposition, dual verification, defect-shaped handoff.

## Deliverables

- `artifacts/runoff-underproduction-localization.md` (M1 mechanism + ownership).
- `artifacts/fq3dc-runoff-validation-ledger.md` (before/after runoff + closure
  preservation + non-regression).
- Standard contract, gate, dual-review, verification, disposition, handoff.

## Dependencies

- `/workdir/openWEPP/AGENTS.md`, `docs/codex_exec_plans.md`,
  `docs/defect_closure_execplans.md`
- `docs/decisions/0011-...`, `0017-...`, `0018-...`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`,
  `SC-CLIMATE-001.md`, `SC-WATBAL-001.md`
- FQ-3 package + `artifacts/fq3-defect-handoff.md`,
  `per_prefix_term_classification.csv`
- Comparator: `/home/workdir/wepppy/wepp_runner/bin/wepp_260606_hill`
- Run inputs: `/wc1/runs/al/algebraic-radium/wepp/runs/`; corpora
  `/tmp/fq1_after/outputs`, `/tmp/fq3_exec/legacy/outputs`

## Autonomy

Execute end-to-end for the declared scope — M1 localization + ownership, contract
amendment, red tests, pre-impl gate, production correction, validation, dual
review/verification, disposition, defect-shaped handoff — without asking for
direction on intermediate steps. Ask only if hard-blocked by a proven boundary
(climate-disaggregation outside the partition, missing authority, or a legitimately
zero-runoff subset).
