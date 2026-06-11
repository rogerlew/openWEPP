# SCSTRUCT07 — SC-SUBHYD Binding Exposure Index Science-Review Adjudication

Status: executed-map-in-core
Created: 2026-06-10
Series: `scstruct` (science-contract structure / context optimization)
Closes: defect `SCSTRUCT06-SUBHYD-BEI-SCIENCE-REVIEW` (routed from SCSTRUCT06)
Execution mode: **science-steered**, batched (not autonomous — each call is a
domain decision backed by cited authority, not an agent guess)

## Objective

Adjudicate every `SC-SUBHYD-001` Binding Exposure Index row routed to
`science-review-follow-on` by SCSTRUCT06, and — as rows resolve — relocate
genuinely historical narrative to the provenance sidecar so the core shrinks.
Reclaim SC-SUBHYD context tokens **without dropping, weakening, or silently adding
any binding obligation.** SCSTRUCT03/05-equivalent, reusing the proven method.

Per batch: every routed row resolved to a cited outcome, historical/mapped
narrative relocated, a conserved `INV-*`/`OBL-*` crosswalk, a recorded token
delta, and `--strict` lint advancing `PASS-DEFERRED` → `PASS`.

## Background

SCSTRUCT06 added the SC-SUBHYD Binding Exposure Index and routed unresolved rows
to science review. SC-SUBHYD is the **WB19 subsurface lateral/drainage/water-yield**
contract; its addenda are the HPHYS0218–0267 WB19 family plus `GAP-SUBHYD-001..004`,
with several Level-4 constitutive suite linkages (HPHYS0224–0227). This is the hard
**live-authority** cohort (the WATBAL WB19 pattern): expect more map-in-core and
promote than historical relocation, and a smaller token yield than SC-SYSTEM. The
value here is auditability + the anti-re-accretion gate as much as tokens. Legacy
behavior is **not** authority; calls trace to the contract derivation hierarchy.

## Per-row adjudication (cited outcomes)

For each routed row, choose exactly one and record the citation:
1. **map-to-existing-`INV-*`/`OBL-*`** — residue already stated by an existing
   invariant; record the *precise* obligation match (not a token scrape).
2. **promote** — a real unpromoted binding obligation → author a new
   `INV-SUBHYD-*`/`OBL-SUBHYD-*` through the **full
   dual-review/disposition/verification gate** with a guard map and (where the
   addendum carries one) its Level-4 constitutive suite linkage; flagged binding
   addition in the crosswalk, never silent.
3. **historical** — genuinely superseded/diagnostic; mark with provenance fields
   and relocate narrative to the sidecar.
4. **map-in-core (retain)** — mapped but the section carries active
   constitutive/guard/test-vector obligations beyond the IDs → keep core-resident
   (the WATBAL WB19/WB13 pattern; expected to dominate here).
5. **narrower science-HOLD** — needs external authority not in the repo (e.g. an
   open `GAP-SUBHYD-*`) → name the gap/owner/gate; do not force a call.

## Authority requirement (normative)

Cite per the procedure's derivation order (WEPP refs → literature → physical →
legacy-static-secondary). WB19 lateral/water-yield rows trace to the SUBHYD Eq.
[6.2.4]/[6.2.10]-[6.2.11] authority and cross-domain WB18/WB13 ownership.
Promotions touching external constitutive suites must reference suite IDs per
`external-authority/suite-schema.md` (the HPHYS0224–0227 `cas_l4_subhyd_*` suites).

## Batching (ordered: lowest-risk first)
1. **WB19 lineage/ordering/publication rows** (HPHYS0234/0238–0242/0252/0256–0259)
   — likely have `INV-SUBHYD-*` coverage; map-in-core/historical. Validates the
   method + captures any early reduction.
2. **HPHYS0224–0227 constitutive cap/water-yield** (Level-4 suite-linked) —
   live authority; map-in-core or promote with suite linkage.
3. **`GAP-SUBHYD-001..004` + HPHYS0203/0208 robustness/threshold** — promotable-
   with-risk / narrower-HOLD.

## Test integration & closure (DOCOPT01 lesson — required)

Relocating contract narrative can break **contract-derived tests** that reference
the moved content. Per batch that relocates narrative:
1. Run the closure loop — `cargo fmt --check`; `cargo clippy --workspace
   --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check` —
   Ran, recorded with exit codes. (Note: confirm a failing gate is pre-existing
   and unrelated before attributing it as a blocker — diff scope + cross-reference
   check, per the SCSTRUCT05 precedent.)
2. **Reconcile** any contract-derived test referencing relocated narrative
   (path/structure only; assertions should *strengthen* to verify BEI + mapped INV
   + sidecar provenance, never weaken). Record breakage + fix.
3. **Subagent authorization:** this package explicitly authorizes subagent
   spawning/delegation to the `comparator_suite_runner` subagent
   (gpt-5.3-codex-spark) for heavy closure/comparator runs only; consume only
   its compact metrics + log paths. Write access is limited to package-owned
   comparator artifact summaries/log-path records when explicitly instructed.

## Authority Envelope

### In-scope
- `SC-SUBHYD-001.md` — index rows, invariant/obligation additions (via gate),
  narrative relocation.
- new `docs/specifications/science-contracts/contracts/provenance/SC-SUBHYD-001-provenance.md`.
- contract-derived test files requiring path/structure reconciliation after moves.

### Allowed edit classes
- Resolve index rows to cited outcomes; relocate historical/mapped narrative;
  promote via the full gate (with suite linkage where applicable); author the
  conservation crosswalk + token deltas; reconcile broken contract-derived tests.

### Protected boundaries (do not cross)
- **No binding obligation removed or weakened.** Additions only via the flagged
  gate; the before/after crosswalk proves conservation. Do not drop a Level-4
  constitutive suite linkage.
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
   explicitly-dispositioned flagged additions, each with a guard map + suite linkage.
5. Token/byte reduction recorded (per batch + total).
6. Closure loop **Ran** + recorded; any contract-derived test breakage reconciled
   (strengthening, not weakening); pre-existing/unrelated reds confirmed as such.
7. Dual review + disposition + dual verification for promotions; kernel-profile
   compliance checklist; no undispositioned finding.

## Legitimate HOLD conditions
- A row needs external authority not in the repo (e.g. an open `GAP-SUBHYD-*`) →
  narrower science-HOLD (gap + owner + gate); continue other batches.
- A mapping exposes a contract↔kernel mismatch → HOLD + separate kernel package.

## Dependencies
- SCSTRUCT06 routed queue + the SC-SUBHYD Binding Exposure Index.
- SCSTRUCT03 / SCSTRUCT05 (proven adjudication templates).
- The three framework docs; `correctness-authority-model.md`;
  `external-authority/README.md` + `suite-schema.md` (for promotions / suite linkage).
- `docs/standards/mechanical-refactor-authoring-guide.md` (closure loop).
- `.codex/agents/comparator_suite_runner.toml` (heavy-run dispatch target).

## Autonomy
**Science-steered, not end-to-end autonomous.** Within a batch, execute the
mechanics autonomously (resolve rows to cited outcomes, relocate, run the lint,
dispatch the closure loop to `comparator_suite_runner` under the explicit
subagent authorization above), but stop and surface any row needing an authority
decision. Proceed lowest-risk batch first. Reusable for the remaining SC-*
contracts (RUNOFFPART, …) after SC-SUBHYD closes.
