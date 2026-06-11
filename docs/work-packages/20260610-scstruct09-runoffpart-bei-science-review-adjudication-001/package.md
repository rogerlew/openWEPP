# SCSTRUCT09 — SC-RUNOFFPART Binding Exposure Index Science-Review Adjudication

Status: executed-map-in-core
Created: 2026-06-10
Series: `scstruct` (science-contract structure / context optimization)
Closes: defect `SCSTRUCT08-RUNOFFPART-BEI-SCIENCE-REVIEW` (routed from SCSTRUCT08)
Execution mode: **science-steered**, batched (not autonomous — each call is a
domain decision backed by cited authority, not an agent guess)

## Objective

Adjudicate every `SC-RUNOFFPART-001` Binding Exposure Index row routed to
`science-review-follow-on` by SCSTRUCT08, and — as rows resolve — relocate
genuinely historical narrative to the provenance sidecar so the core shrinks.
Reclaim SC-RUNOFFPART context tokens **without dropping, weakening, or silently
adding any binding obligation.** SCSTRUCT03/05/07-equivalent, reusing the proven
method.

Per batch: every routed row resolved to a cited outcome, historical/mapped
narrative relocated, a conserved `INV-*`/`OBL-*` crosswalk, a recorded token
delta, and `--strict` lint advancing `PASS-DEFERRED` → `PASS`.

## Background

SCSTRUCT08 added the SC-RUNOFFPART Binding Exposure Index and routed unresolved
rows to science review. SC-RUNOFFPART is **mixed-character**: the ADR-0017-retired
snow/`RM` comparator arc (HPHYS0296–0298) is likely `historical`/relocatable (the
WATBAL snow-arc pattern), while WB12/WB14 runoff-carryover + spring snowmelt
runoff/infiltration partition (HPHYS0240–0293, SNOWSCI), WB16 `ealpha` producer
(HILLSTAB06–08), and `GAP-RUNOFFPART-001..005` are live runoff-partition authority
(map-in-core/promote). Expect a token yield **between** SC-SYSTEM and SC-SUBHYD.
Legacy behavior is **not** authority; calls trace to the contract derivation
hierarchy. ADR-0017 governs the snow/`RM` rows: retired *investigation* ≠ no
*obligation* — each addendum's residue needs its own call.

## Per-row adjudication (cited outcomes)

For each routed row, choose exactly one and record the citation:
1. **map-to-existing-`INV-*`/`OBL-*`** — residue already stated by an existing
   invariant; record the *precise* obligation match (not a token scrape).
2. **promote** — a real unpromoted binding obligation → author a new
   `INV-RUNOFFPART-*`/`OBL-RUNOFFPART-*` through the **full
   dual-review/disposition/verification gate** with a guard map (and suite linkage
   where the addendum carries one); flagged binding addition, never silent.
3. **historical** — genuinely superseded/diagnostic; mark with provenance fields
   and relocate narrative to the sidecar.
4. **map-in-core (retain)** — mapped but the section carries active
   guard/test-vector/producer obligations beyond the IDs → keep core-resident.
5. **narrower science-HOLD** — needs external authority not in the repo (e.g. an
   open `GAP-RUNOFFPART-*`) → name the gap/owner/gate; do not force a call.

## Authority requirement (normative)

Cite per the procedure's derivation order (WEPP refs → literature → physical →
legacy-static-secondary). Runoff-partition rows trace to the WB12/WB14
infiltration/runoff reconciliation authority and WB16 peak-runoff/`ealpha`
producer lineage; snow/`RM` rows are governed by ADR-0017 (comparator is a flag,
not a target). Promotions touching external constitutive suites reference suite
IDs per `external-authority/suite-schema.md`.

## Batching (ordered: lowest-risk first)
1. **ADR-0017-retired snow/`RM` arc rows** (HPHYS0296–0298) — likely `historical`;
   validates the method + captures early reduction (the WATBAL snow-arc precedent).
2. **WB12/WB14 runoff-carryover + spring snowmelt partition** (HPHYS0240–0293,
   SNOWSCI) — runoff-partition lineage; map-in-core/historical mixed.
3. **WB16 `ealpha` producer (HILLSTAB06–08) + SIMIMPL36 + `GAP-RUNOFFPART-*`** —
   live authority; map-in-core / promote / narrower-HOLD.

## Test integration & closure (DOCOPT01 lesson — required)

Relocating contract narrative can break **contract-derived tests** that reference
the moved content. Per batch that relocates narrative:
1. Run the closure loop — `cargo fmt --check`; `cargo clippy --workspace
   --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check` —
   Ran, recorded with exit codes. Confirm a failing gate is pre-existing and
   unrelated (diff scope + cross-reference) before attributing it as a blocker
   (the SCSTRUCT05 precedent).
2. **Reconcile** any contract-derived test referencing relocated narrative
   (path/structure only; assertions should *strengthen* to verify BEI + mapped INV
   + sidecar provenance, never weaken). Record breakage + fix.
3. **Subagent authorization (mandatory): this package explicitly authorizes
   subagent spawning/delegation to `comparator_suite_runner`
   (gpt-5.3-codex-spark) for the closure loop and any comparator/population runs.
   Do NOT run them on the parent model unless the subagent is unavailable, in
   which case record command-level evidence.** Consume only its compact metrics +
   log paths. See `docs/standards/prompt-wording-guidance.md` §4a.

## Authority Envelope

### In-scope
- `SC-RUNOFFPART-001.md` — index rows, invariant/obligation additions (via gate),
  narrative relocation.
- new `docs/specifications/science-contracts/contracts/provenance/SC-RUNOFFPART-001-provenance.md`.
- contract-derived test files requiring path/structure reconciliation after moves.

### Allowed edit classes
- Resolve index rows to cited outcomes; relocate historical/mapped narrative;
  promote via the full gate; author the conservation crosswalk + token deltas;
  reconcile broken contract-derived tests (path/structure only).

### Protected boundaries (do not cross)
- **No binding obligation removed or weakened.** Additions only via the flagged
  gate; the before/after crosswalk proves conservation.
- **No production kernel/runtime edit**; a contract↔kernel mismatch → HOLD +
  separate kernel package.
- **No legacy-as-authority calls**; no comparator re-tiering; no forced calls.

## Acceptance criteria
1. Every routed row resolved to a cited outcome; no bare `science-review-follow-on`
   rows without owner/gap/gate.
2. Historical/mapped narrative relocated; core conforms to section order.
3. `--strict` lint reaches `PASS` when all rows resolve; partial closures record
   the falling deferred count.
4. Conservation crosswalk: before/after `INV-*`/`OBL-*` identical except
   explicitly-dispositioned flagged additions, each with a guard map.
5. Token/byte reduction recorded (per batch + total).
6. Closure loop **Ran** (via `comparator_suite_runner`) + recorded; any
   contract-derived test breakage reconciled (strengthening, not weakening);
   pre-existing/unrelated reds confirmed as such.
7. Dual review + disposition + dual verification for promotions; kernel-profile
   compliance checklist; no undispositioned finding.

## Legitimate HOLD conditions
- A row needs external authority not in the repo (e.g. open `GAP-RUNOFFPART-*`) →
  narrower science-HOLD (gap + owner + gate); continue other batches.
- A mapping exposes a contract↔kernel mismatch → HOLD + separate kernel package.

## Dependencies
- SCSTRUCT08 routed queue + the SC-RUNOFFPART Binding Exposure Index.
- SCSTRUCT03 / 05 / 07 (proven adjudication templates).
- The three framework docs; `correctness-authority-model.md`;
  `external-authority/README.md` + `suite-schema.md` (promotions / suite linkage).
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`.
- `docs/standards/mechanical-refactor-authoring-guide.md` (closure loop).
- `docs/standards/prompt-wording-guidance.md` §4a (subagent requirement).
- `.codex/agents/comparator_suite_runner.toml` (required heavy-run dispatch target).

## Autonomy
**Science-steered, not end-to-end autonomous.** Within a batch, execute the
mechanics autonomously (resolve rows to cited outcomes, relocate, run the lint),
**spawn `comparator_suite_runner` for the closure loop (required, not optional)**,
but stop and surface any row needing an authority decision. Proceed lowest-risk
batch first. Reusable for the remaining SC-* contracts after SC-RUNOFFPART closes.
