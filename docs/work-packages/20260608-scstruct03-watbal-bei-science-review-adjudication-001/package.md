# SCSTRUCT03 — WATBAL Binding Exposure Index Science-Review Adjudication

Status: in_progress_batch3_hold_promotion_required
Created: 2026-06-08
Series: `scstruct` (science-contract structure / context optimization)
Closes: defect `SCSTRUCT02-WATBAL-BEI-SCIENCE-REVIEW` (routed from SCSTRUCT02 HOLD)
Execution mode: **science-steered**, batched (not autonomous — each call is a
domain decision backed by cited authority, not an agent guess)

## Objective

Adjudicate every `SC-WATBAL-001` Binding Exposure Index row currently routed to
`science-review-follow-on` (**69** rows), and — as rows resolve — relocate
genuinely historical narrative to the provenance sidecar so the contract core
actually shrinks. This is the package that finally reclaims the WATBAL context
tokens SCSTRUCT01/02 deliberately deferred, **without dropping, weakening, or
silently adding any binding obligation.**

Success per batch: every routed row in the batch resolved to a cited outcome, the
historical/mapped narrative relocated, a conserved `INV-*`/`OBL-*` crosswalk, a
recorded token delta, and `tools/check_sc_binding_exposure.py --strict` advancing
from `PASS-DEFERRED` toward `PASS` as the deferred count falls.

## Background

SCSTRUCT02 (`20260608-scstruct02-...`) triaged the WATBAL Binding Exposure Index
and held: it routed all 69 unresolved rows to `science-review-follow-on` rather
than guess. Verified state (`Ran`): `PASS-DEFERRED … 75 rows, 69
science-review-follow-on rows not yet consolidated` (exit 0; `--strict` exit 1).
No narrative was relocated; SC-WATBAL-001 is unchanged in size. The 69 rows are
listed in
`../20260608-scstruct02-.../artifacts/science-review-followon-queue.md` and carry
`science-review-follow-on` in the contract's `## Binding Exposure Index`.

Why this needs science, not mechanics: these addenda encode water-balance
obligations (WB12/WB14/WB15/WB19 coupling, storage/ET/Ep, snow-melt comparator
lineage, ProfileFC/WP) accreted across years. Deciding which residue is live vs
superseded is a domain call. Per the project's correctness re-anchoring, **legacy
behavior is not authority** — calls are grounded in the contract derivation
hierarchy, not in what a binary did.

## Per-row adjudication (cited outcomes)

For each routed row, choose exactly one and record the citation:

1. **map-to-existing-`INV-*`/`OBL-*`** — residue is already stated by an existing
   invariant; record the *precise* obligation match (not a token scrape). Row
   becomes relocation-eligible.
2. **promote** — a real unpromoted binding obligation. Author a new
   `INV-WATBAL-*`/`OBL-WATBAL-*` through the **full
   dual-review/disposition/verification gate** with a guard map; record as a
   flagged binding addition in the crosswalk (never silent).
3. **historical** — genuinely superseded/diagnostic, no live obligation. Mark
   `historical`/`superseded` with provenance fields; relocate narrative to the
   sidecar.
4. **narrower science-HOLD** — the call requires external authority not yet in the
   repo. Keep the row routed, but tighten it: name the specific authority gap,
   the owner, and the next evidence gate. Do not force a call.

## Authority requirement (normative)

Every outcome above must cite per the procedure's derivation order:
1. WEPP technical references (incl. `references/50201000`), 2. peer-reviewed
literature, 3. physical/common-sense invariants, 4. legacy static inspection
(secondary provenance only). ADR-0017 governs the snow-comparator rows: comparator
agreement is a flag, not authority. Promotions touching external constitutive
suites must reference suite IDs per `external-authority/suite-schema.md`.

## Batching (ordered: validate the method on lowest-risk first)

Run batch by batch; replay `--strict` after each; checkpoint between.
1. **ProfileFC/WP layer family** (HPHYS0202/0205/0206/0216/0216D) — titled
   `(Historical)`; most likely `historical`. Lowest risk → validates the
   adjudication + relocation method and captures early reduction.
2. **Snow/melt-term comparator arc** (HPHYS0298–0308) — ADR-0017-retired
   investigation; likely `historical`, but each addendum's residue needs its own
   call (retired *investigation* ≠ no *obligation*).
3. **WB13/WB16/WB12 output & reconciliation** addenda.
4. **WB14/WB15/CLIM05/CLIM06/IRRIG10 coupling-surface** addenda.
5. **WB19 lateral/drainage/water-yield family** (HPHYS0218–0259) — the genuinely
   hard cohort; most promote-vs-map ambiguity.
6. **Cross-cutting** (EROD12/13/14, ARCH22, SIMIMPL*, MOFE04).

## Authority Envelope

### In-scope
- `SC-WATBAL-001.md` — index rows, invariant/obligation additions (via gate),
  narrative relocation.
- `provenance/SC-WATBAL-001-provenance.md` — populate per the provenance spec.
- lint consumption (`--strict` as the completion gate); no lint code changes here.

### Allowed edit classes
- Resolve index rows to cited outcomes (map / promote / historical / narrower-HOLD).
- Promote obligations to new `INV-*`/`OBL-*` only via the full review gate.
- Relocate `historical`/`superseded` narrative to the sidecar.
- Author the conservation crosswalk + per-batch token deltas.

### Protected boundaries (do not cross)
- **No binding obligation removed or weakened.** Additions only via the flagged
  gate. Crosswalk proves conservation.
- **No production kernel/runtime edit.** A contract↔kernel mismatch → HOLD +
  separate kernel package.
- **No legacy-as-authority calls**; no comparator re-tiering.
- **No forced calls** — undecidable without authority → narrower science-HOLD.

## Acceptance criteria
1. Every routed row resolved to a cited outcome; zero rows left as bare
   `science-review-follow-on` without owner/gap/gate.
2. Historical/mapped narrative relocated; core conforms to
   `science-contract-spec.md` section order.
3. `tools/check_sc_binding_exposure.py --strict SC-WATBAL-001.md` reaches **`PASS`**
   (exit 0) when all rows are resolved; partial closures record the falling
   deferred count.
4. Conservation crosswalk: before/after `INV-*`/`OBL-*` identical except
   explicitly-dispositioned flagged additions, each with a guard map.
5. Token/byte reduction of the SC-WATBAL-001 core recorded (before/after, per batch
   and total).
6. Dual review + disposition + dual verification for any promotions; kernel-profile
   compliance checklist; no undispositioned finding.

## Legitimate HOLD conditions
- A row needs external authority not yet in the repo → narrower science-HOLD
  (named gap + owner + gate); continue other batches.
- A mapping exposes a contract↔kernel mismatch → HOLD + separate kernel package.

## Dependencies
- SCSTRUCT02 routed queue + the current SC-WATBAL-001 Binding Exposure Index.
- The three framework docs (`science-contract-{authoring-procedure,spec,provenance-spec}.md`).
- `docs/specifications/correctness-authority-model.md`,
  `docs/specifications/external-authority/README.md` (for promotions).
- `docs/decisions/0011-...`, `docs/decisions/0017-...`.
- `docs/codex_exec_plans.md`.

## Autonomy
**Science-steered, not end-to-end autonomous.** Each adjudication is a domain
decision: the operator/scientist (or a cited external-authority pass) makes the
call; the agent records it with citation, executes the resulting relocation/
promotion under the gates, and replays the lint. Proceed batch by batch; stop at
any row that needs an authority decision and surface it. This is the reusable
template for the remaining SC-* contract consolidations (SNOWFREEZE, SYSTEM,
RUNOFFPART, SUBHYD, …) once WATBAL is closed.
