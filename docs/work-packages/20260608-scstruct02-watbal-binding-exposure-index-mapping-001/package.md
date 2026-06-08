# SCSTRUCT02 — WATBAL Binding Exposure Index Mapping + Consolidation

Status: executed-hold-science-review-follow-on
Created: 2026-06-08
Series: `scstruct` (science-contract structure / context optimization)
Closes: defect `SCSTRUCT01-WATBAL-BEI-MAPPING` (routed from SCSTRUCT01 Phase 2 HOLD)
Execution mode: package-end-to-end (phased; batched; declared stop-boundaries)

## Objective

Realize the WATBAL context-optimization win that SCSTRUCT01 deliberately deferred:
adjudicate every `SC-WATBAL-001` Binding Exposure Index row that currently blocks
consolidation, then relocate genuinely historical narrative to a provenance
sidecar — **without dropping, weakening, or silently adding any binding
obligation.** Success is the lint passing green on a slimmed core plus a
conserved `INV-*`/`OBL-*` crosswalk and a measured token reduction.

This is the contract-authoring work SCSTRUCT01 exposed: ~61 addenda carry binding
language never promoted to a numbered invariant, so the bloat is load-bearing.
SCSTRUCT02 pays that debt.

## Background

SCSTRUCT01 (`20260608-scstruct01-...`) authored the framework (procedure split +
`science-contract-spec.md` + `science-contract-provenance-spec.md` + the
binding-exposure lint) and stopped Phase 2 at a legitimate HOLD: the Binding
Exposure Index it added to `SC-WATBAL-001` showed most addenda lack a precise
canonical mapping. Verified lint state (`Ran`):
`FAIL … 133 issue(s)`. Index row counts: `maps-to-existing-INV` 6,
`undecidable` 8, `unpromoted-binding` 61.

Governing framework (authority, do not re-derive):
- `docs/specifications/science-contract-spec.md` — Binding Exposure Index schema,
  lint contract, contract section order.
- `docs/specifications/science-contract-provenance-spec.md` — sidecar format,
  status vocabulary, lifecycle, retention.
- `docs/specifications/science-contract-authoring-procedure.md` — Binding
  Exposure Workflow, dual-review/disposition/verification gates.

## Per-row adjudication rule (the core mechanic)

For each non-`maps-to-existing-INV` index row, choose exactly one:

1. **map-to-existing-`INV-*`/`OBL-*`** — the addendum's binding residue is already
   stated by an existing invariant. Record the precise mapping (not a token
   scrape — the specific obligation must match). Row becomes relocation-eligible.
2. **promote** — the addendum carries a real, unpromoted binding obligation.
   Author a new `INV-WATBAL-*`/`OBL-WATBAL-*` through the **full
   dual-review/disposition/verification gate**. This is a flagged binding
   **addition**, recorded explicitly in the crosswalk — never silent. The
   promoted invariant must carry a guard mapping per `science-contract-spec.md`.
3. **historical** — the addendum is genuinely superseded/diagnostic with no live
   obligation. Mark `historical`/`superseded` with provenance fields; eligible to
   move to the sidecar.
4. **science-review-follow-on** — binding status is undecidable without a science
   decision (e.g., the `undecidable` rows, WB16, the HPHYS0202 family). Route out;
   do **not** force a call. These keep their narrative in core until resolved.

Only after a row is (1) or (3) may its narrative be relocated.

## Batching

Adjudicate in coherent sub-domain batches rather than one 69-row pass, replaying
the lint between batches. Suggested batches (operator/Codex may re-cut):
1. WB12/WB14/WB15/IRRIG10/CLIM05/CLIM06 coupling-surface addenda.
2. WB19 lateral/drainage/water-yield family (HPHYS0218–0259 cluster).
3. HPHYS storage/ET/Ep family (HPHYS0246–0256 cluster).
4. Snow/melt-term comparator addenda (HPHYS0298–0308) — note ADR-0017 governance;
   many are likely `historical` post-arc-retirement, but each needs the call.
5. ProfileFC/WP layer family (HPHYS0202–0216D) — likely historical.
6. Cross-cutting (EROD12/EROD13/EROD14, ARCH22, SIMIMPL*, MOFE04).

## Authority Envelope

### In-scope
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (index rows,
  invariant/obligation additions via gate, narrative relocation)
- a new `docs/specifications/science-contracts/contracts/provenance/SC-WATBAL-001-provenance.md`
  sidecar
- `tools/check_sc_binding_exposure.py` consumption (run as gate; lint code fixes
  belong to the framework, not this package, unless a blocking bug surfaces)

### Allowed edit classes
- Refine Binding Exposure Index rows with precise mappings/classifications.
- Promote unpromoted binding obligations to new `INV-*`/`OBL-*` **only** through
  the full review gate (flagged additions).
- Relocate `historical`/`superseded` narrative to the sidecar.
- Author the conservation crosswalk + token-reduction measurement.

### Protected boundaries (do not cross)
- **No binding obligation removed or weakened.** Additions only via the flagged
  gate. The before/after crosswalk must prove conservation.
- **No production kernel/runtime edit.** If a mapping reveals a contract↔kernel
  mismatch, that is a separate kernel package → HOLD + follow-on, not an edit here.
- **No comparator re-tiering**; ADR-0017 governance text moves verbatim.
- **No forced calls** on `undecidable` rows — route to science review.

## Acceptance criteria
1. Every Binding Exposure Index row is `maps-to-existing-INV`, `historical` (with
   mapping/provenance), or explicitly routed `science-review-follow-on`. No
   `unpromoted-binding` or `undecidable` rows remain unresolved in core.
2. `python3 tools/check_sc_binding_exposure.py SC-WATBAL-001.md` exits **0**
   (PASS), with science-review-follow-on rows handled per the lint contract.
3. Historical narrative relocated to the sidecar; core conforms to
   `science-contract-spec.md` section order.
4. Conservation crosswalk: before/after `INV-*`/`OBL-*` set identical except
   explicitly-dispositioned flagged additions; each addition carries a guard map.
5. Token/byte reduction of `SC-WATBAL-001` core recorded (before/after).
6. Dual review + disposition + dual verification; kernel-profile compliance
   checklist confirms no kernel-affecting change; no undispositioned finding.

## Legitimate HOLD conditions
- A row's binding status needs a science decision → route `science-review-follow-on`,
  keep narrative in core, continue with the rest.
- A mapping reveals a contract↔kernel mismatch → HOLD, separate kernel package.

## Dependencies
- SCSTRUCT01 package + its Phase 0 inventory, Binding Exposure Index, and the
  three framework docs (authority).
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/codex_exec_plans.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`

## Autonomy
Execute end-to-end, batch by batch, replaying the lint between batches. Promotions
go through the review gate as flagged additions. Ask only if hard-blocked or at a
declared stop-boundary (science-review or kernel-mismatch). This is the reusable
template for the remaining SC-* consolidations (SNOWFREEZE, SYSTEM, RUNOFFPART, …).
