# SCSTRUCT05 — SC-SYSTEM Binding Exposure Index Science-Review Adjudication

Status: queued
Created: 2026-06-10
Series: `scstruct` (science-contract structure / context optimization)
Closes: defect `SCSTRUCT04-SYSTEM-BEI-SCIENCE-REVIEW` (routed from SCSTRUCT04)
Execution mode: **science-steered**, batched (not autonomous — each call is a
domain decision backed by cited authority, not an agent guess)

## Objective

Adjudicate every `SC-SYSTEM-001` Binding Exposure Index row routed to
`science-review-follow-on` by SCSTRUCT04, and — as rows resolve — relocate
genuinely historical narrative to the provenance sidecar so the contract core
shrinks. Reclaim SC-SYSTEM context tokens **without dropping, weakening, or
silently adding any binding obligation.** This is the SCSTRUCT03-equivalent for
SC-SYSTEM, reusing that package's proven method.

Per batch: every routed row resolved to a cited outcome, historical/mapped
narrative relocated, a conserved `INV-*`/`OBL-*` crosswalk, a recorded token
delta, and `tools/check_sc_binding_exposure.py --strict` advancing
`PASS-DEFERRED` → `PASS` as the deferred count falls.

## Background

SCSTRUCT04 added the SC-SYSTEM Binding Exposure Index and routed unresolved rows
to science review. SC-SYSTEM is the System Integration Boundary + Watershed
Assembly contract; its addenda are mostly WSHEDIMPL channel/sediment/routing
integration migrations (each closed a `GAP-SYSTEM-*`/`GAP-ROUTE-*`, so most likely
**already-promoted** with matching `INV-SYSTEM-*` — the WATBAL snow-arc pattern),
plus HPARITY02 profile-capacity governance (`INV-SYSTEM-027`) and HPHYS0202–0209
FC/WP publication governance (likely live-authority, map-in-core or promote).
Legacy behavior is **not** authority; calls are grounded in the contract
derivation hierarchy.

## Per-row adjudication (cited outcomes)

For each routed row, choose exactly one and record the citation:
1. **map-to-existing-`INV-*`/`OBL-*`** — residue already stated by an existing
   invariant; record the *precise* obligation match (not a token scrape).
2. **promote** — a real unpromoted binding obligation → author a new
   `INV-SYSTEM-*`/`OBL-SYSTEM-*` through the **full
   dual-review/disposition/verification gate** with a guard map; flagged binding
   addition in the crosswalk, never silent.
3. **historical** — genuinely superseded/diagnostic; mark with provenance fields
   and relocate narrative to the sidecar.
4. **map-in-core (retain)** — mapped but the section carries active
   schema/guard/test-vector/integration obligations beyond the IDs → keep
   core-resident (the WATBAL WB13 pattern).
5. **narrower science-HOLD** — needs external authority not in the repo → name the
   gap/owner/gate; do not force a call.

## Authority requirement (normative)

Cite per the procedure's derivation order (WEPP refs → literature → physical →
legacy-static-secondary). Watershed/system-integration rows trace to
`SC-ROUTE-001`/`SC-SED-001` cross-domain ownership and the watershed dispatch/
topology contracts where applicable. Promotions touching external constitutive
suites reference suite IDs per `external-authority/suite-schema.md`.

## Batching (ordered: lowest-risk first)
1. **WSHEDIMPL GAP-closure migrations** (WSHEDIMPL14–41) — each closed a
   `GAP-SYSTEM-*`/`GAP-ROUTE-*` and likely has `INV-SYSTEM-*` coverage; most
   likely map-in-core/historical. Validates the method + captures early reduction.
2. **HPHYS0202–0209 FC/WP publication governance** — diagnostic/governance, mixed.
3. **HPARITY02 profile-capacity (`INV-SYSTEM-027`)** + any `GAP-SYSTEM-001`
   promotable-with-risk rows — live authority; map-in-core or promote.

## Test integration & closure (DOCOPT01 lesson — required)

Relocating contract narrative can break **contract-derived tests** that reference
the moved content (DOCOPT01 reconciled erod/hphys contract tests after similar
moves). Therefore, per batch that relocates narrative:
1. Run the closure loop — `cargo fmt --check`; `cargo clippy --workspace
   --all-targets -- -D warnings`; `cargo test --workspace`; `cargo deny check` —
   Ran, recorded with exit codes.
2. **Reconcile** any contract-derived test referencing relocated narrative
   (path/structure only; never change a behavior assertion); record breakage + fix.
3. **Dispatch the heavy closure/comparator runs to the `comparator_suite_runner`
   subagent** (gpt-5.3-codex-spark) so this package's reasoning agent does not load
   the corpus or re-run suites itself — return only compact metrics + log paths.

## Authority Envelope

### In-scope
- `SC-SYSTEM-001.md` — index rows, invariant/obligation additions (via gate),
  narrative relocation.
- new `docs/specifications/science-contracts/contracts/provenance/SC-SYSTEM-001-provenance.md`.
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
6. Closure loop **Ran** + recorded; any contract-derived test breakage reconciled.
7. Dual review + disposition + dual verification for promotions; kernel-profile
   compliance checklist; no undispositioned finding.

## Legitimate HOLD conditions
- A row needs external authority not in the repo → narrower science-HOLD (gap +
  owner + gate); continue other batches.
- A mapping exposes a contract↔kernel mismatch → HOLD + separate kernel package.

## Dependencies
- SCSTRUCT04 routed queue + the SC-SYSTEM Binding Exposure Index.
- SCSTRUCT03 (the proven WATBAL adjudication template).
- The three framework docs; `correctness-authority-model.md`;
  `external-authority/README.md` (for promotions).
- `docs/standards/mechanical-refactor-authoring-guide.md` (closure loop).
- `.codex/agents/comparator_suite_runner.toml` (heavy-run dispatch target).

## Autonomy
**Science-steered, not end-to-end autonomous.** Within a batch, execute the
mechanics autonomously (resolve rows to cited outcomes, relocate, run the lint,
dispatch the closure loop to `comparator_suite_runner`), but stop and surface any
row needing an authority decision. Proceed lowest-risk batch first; capture early
reduction before the live-authority cohort. Reusable for the remaining SC-*
contracts (SUBHYD, RUNOFFPART, …) after SC-SYSTEM closes.
