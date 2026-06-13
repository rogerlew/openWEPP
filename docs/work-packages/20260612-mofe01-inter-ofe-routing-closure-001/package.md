# MOFE01 — Inter-OFE Routing Closure (rung 3)

Status: active; M-E0 executed-hold

Package type: staged implementation/closure package (FDHP01 execution shape:
scope-then-staged-increments, per-increment conservation hard stops)

## Objective

Implement and close **inter-OFE run-on/run-off routing** on the vertically
closed, frost-settled per-element balance: runoff leaving OFE *i* becomes
run-on to OFE *i+1*, with per-element and hillslope-total water-balance
closure as the acceptance authority. ROADMAP queue item 1 (rung 3 of the
WB → frost → MOFE → snow ladder).

## Development substrate (operator decision, 2026-06-12)

`/wc1/runs/ar/arboreal-dendrite/wepp` — a graded OFE ladder:

- 36 hillslope runs: 7×1-OFE, 5×2-OFE, 5×3-OFE, 3×4-OFE, 16×5-OFE, with
  36 legacy outputs on disk (`output/H*.wat.dat` etc.).
- `pw0.slp` (15 OFEs) is the **watershed-representative profile, not a
  hillslope run** (operator clarification 2026-06-12) — it is not part of
  this package's cohort or gates.
- The single-OFE subset is the continuity anchor: those 7 hillslopes must
  reproduce rung-1/rung-2 closure behavior unchanged, so routing error is
  never aliased into the settled vertical balance.
- The ladder allows closure to be validated **per OFE count** (1→2→3→4→5),
  localizing any routing defect to the count at which it first appears.

## Comparator posture — stronger distrust than the ADR-0017 default

**Legacy WEPP has known water-balance defects that grow with OFE count,
typically appearing above ~10 OFEs** (operator knowledge, 2026-06-12;
corroborated by the legacy-replay MOFE closure-audit triage at wepppy
`docs/work-packages/20260502_mofe_flagged_hillslope_triage/`, which built a
defect-family taxonomy from legacy's own flagged hillslopes). The 1–5-OFE
development ladder is therefore **expected legacy-clean** — M-A verifies
this rather than assumes it. The >10-OFE defect domain is **not reachable
on this substrate's hillslope cohort** (`pw0` is the watershed profile, not
a run); demonstrating openWEPP closure beyond the legacy ceiling is a
follow-on on a high-OFE substrate or the watershed step. Consequences:

- Legacy is a weak flag at low OFE counts and progressively untrustworthy as
  OFE count rises — in exactly the dimension this package builds.
- **No comparator-match acceptance at any OFE count.** Acceptance is
  openWEPP's own conservation closure: per-element identity, inter-OFE
  transfer accounting (run-on received ≡ run-off sent), and hillslope-total
  identity, all at the noise floor established by the FDHP01 arc.
- The characterization increment must **measure** legacy's per-OFE-count
  closure from the on-disk outputs, so comparator trust is calibrated with
  evidence per count rather than assumed. Expected shape: clean across the
  1–5 ladder (making legacy a usable flag for this package). Where legacy
  fails its own closure, divergence from legacy is *expected* and is not a
  finding against openWEPP.
- **The differentiating target (follow-on, not this package)**: openWEPP
  holding the three identities at >10-OFE counts where legacy demonstrably
  cannot — requires a high-OFE substrate or the watershed step; named in
  the closure handoff, not gated here.
- A legacy-vs-openWEPP divergence may still flag an openWEPP defect — but
  only the conservation identities adjudicate.

## Execution shape

Per the FDHP01 template (`docs/work-packages/20260608-fdhp01-frost-depth-heat-flow-parity-closure-001/artifacts/d3-staged-increment-plan.md`,
agent memory `staged-increment-port-template`): a scoping increment produces
the routing scope artifact (legacy state-machine map, openWEPP seam mapping,
state-shape proposal, red-test definitions) **before any production code**;
implementation lands as separately dispatched, separately committed
increments behind per-increment conservation hard stops; diagnostics are
paired-trajectory, evidence-first; refuted hypotheses get contract pins.
The staged plan artifact governs increments:
`artifacts/mofe-staged-increment-plan.md`.

## Included scope

- Characterization of the substrate: openWEPP current multi-OFE behavior
  (the existing `runon_input` carryover seam in
  `hydrology_phase_runoff_reconciliation.rs` and the WAT `UpStrmQ`/`SubRIn`/
  `QOFE` surfaces), legacy per-OFE-count closure measurement, single-OFE
  continuity verification.
- Scope artifact: legacy inter-OFE routing map (the per-plane loop, runoff →
  run-on hand-off symbols, infiltration-on-lower-OFE coupling, lateral flow
  and subsurface inter-element terms), mapped onto openWEPP's kernel-phase
  and element-orchestration seams.
- Contract authority: amend/author the routing science contract(s)
  (`SC-RUNOFFPART-001` / `SC-WATBAL-001` and any routing-specific contract
  the scoping identifies) with the inter-OFE transfer invariants — including
  the new conservation identities (per-element, transfer, hillslope-total).
- Implementation increments per the staged plan, each behind the
  conservation hard stop.
- Acceptance on the graded ladder per OFE count.

## Excluded scope / protected boundaries

- **Single-OFE physics untouched**: rung-1/rung-2 surfaces (WB closure,
  frost state machine and energetics, snow conservation) are settled; any
  increment that moves single-OFE cohort outputs beyond noise is a hard
  stop. The 7 single-OFE hillslopes are the standing non-regression anchor.
- **No comparator-match tuning at any OFE count** (see posture above).
- Snow physics-magnitude (Stage-2), F4 snow density/depth-split, p2-class
  carried items from FDHP01 — out of scope.
- Watershed-CLI routing/outputs are the *following* step; this package is
  hillslope-internal inter-OFE routing (the watershed seam is mapped in
  scoping but not implemented here unless the scoping shows it is the same
  seam).
- Erosion/sediment inter-OFE coupling: mapped in scoping; in scope only if
  the water-routing seam owns it inseparably — otherwise a follow-on.

## Conservation identities (the acceptance authority)

To be pinned exactly in the contract during scoping, in the spirit of:

1. **Per-element**: the rung-1 identity per OFE, extended with run-on as
   inflow and run-off as outflow.
2. **Transfer**: Σ(run-off sent by OFE i) ≡ Σ(run-on received by OFE i+1) —
   no water created or destroyed in the hand-off.
3. **Hillslope-total**: the whole-hillslope identity (inputs − outputs −
   ΔStorage over all elements) closes at the FDHP01-era noise floor; the
   totalwatsed3 audit consumes the result (this finally enables the full
   end-to-end totalwatsed3 run deferred since WBVAL06/6a).

## Deliverables

- `artifacts/mofe-staged-increment-plan.md` (governs dispatches; authored at
  scaffold).
- `artifacts/mofe-routing-port-scope.md` (produced by the scoping increment).
- Characterization ledgers: openWEPP multi-OFE current behavior; legacy
  per-OFE-count closure measurement (the comparator-trust calibration).
- Contract amendments + red/green tests per increment.
- Per-increment evidence artifacts, the standard DC artifact set (dual
  review with finding disposition, dual verification, gate results,
  line-count governance, owned-file manifest, kernel-profile checklist,
  disposition, worker handoff), and `artifacts/required-reading-map.md`.

## Subagent Requirement

Subagent requirement: REQUIRED, not optional. This package explicitly
authorizes subagent spawning/delegation to:

- `comparator_suite_runner` (gpt-5.3-codex-spark) for all heavy
  batch/closure/comparator runs (`cargo test --workspace`, clippy/deny
  loops, 37-hillslope cohort runs, legacy-output parsing batches). Outputs:
  compact metrics + log/artifact paths only; write access: read-only plus
  package `artifacts/` logs. **Do NOT run heavy batch work on the parent
  model** unless the subagent is unavailable; record command-level evidence
  if so (`docs/standards/prompt-wording-guidance.md` §4a).
- review/verification subagents for the dual review/verification artifacts;
  bounded to package `artifacts/`.

## Security-Impact Gate

No new input-parsing surfaces beyond reading existing legacy output files
for characterization (read-only, bounds-checked parsing of `H*.wat.dat`
class text outputs); no subprocess/argument-construction changes; no
network egress; no `unsafe`. If execution discovers it must touch the
watershed-CLI subprocess seam, stop and record the scope change first.

## Acceptance / exit criteria

- The three conservation identities close at the established noise floor on
  the full 1–5 ladder (36 hillslopes), per OFE count.
- Single-OFE cohort non-regressed (bit-identical or at-noise vs the
  pre-package boundary, per increment gates).
- Legacy per-OFE-count closure defect measured and recorded (the
  comparator-trust calibration deliverable) — divergence-from-legacy
  adjudicated only through the identities.
- Contract-derived red/green tests; truthful evidence labels throughout.
- On closure: ROADMAP item 1 removed, README narrative updated, handoff
  names the next mechanism (watershed outputs / Stage-2 per the queue), and
  the totalwatsed3 end-to-end deferral (WBVAL06/6a note) is resolved or
  explicitly re-stated.

## Dependencies

- `docs/ROADMAP.md` (queue item 1, updated 2026-06-12 with this substrate)
- FDHP01 package (the settled vertical balance + the execution-shape
  template); FROSTVAL01/FQ-4 (activation gates)
- wepppy `docs/work-packages/20260502_mofe_flagged_hillslope_triage/`
  (legacy MOFE defect-family taxonomy — calibration evidence, not authority)
- `AGENTS.md`, `docs/codex_exec_plans.md`, `docs/defect_closure_execplans.md`,
  ADR-0011/0017/0018, `docs/standards/kernel-work-package-preparation.md`,
  `docs/standards/prompt-wording-guidance.md` (§4a)
- Contracts: `SC-RUNOFFPART-001`, `SC-WATBAL-001`, `SC-SYSTEM-001` (+ any
  routing contract the scoping identifies)
- Legacy pinned baseline `/workdir/wepp-forest_260430_baseline/src/` (the
  per-plane loop and run-on lineage — mapped during scoping)
- Substrate `/wc1/runs/ar/arboreal-dendrite/wepp/` (inputs + legacy outputs)

## Autonomy

Each dispatched increment executes end-to-end per the staged plan without
asking for direction on intermediate steps; hard stops are the conservation
gates and the protected boundaries. Operator decisions (certification,
boundary declarations, comparator-posture changes) come back to the operator
per the staged plan's routing rules.
