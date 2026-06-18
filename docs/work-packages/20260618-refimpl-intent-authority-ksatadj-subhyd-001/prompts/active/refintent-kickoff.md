# REFIMPL-INTENT-AUTHORITY + ksatadj/SC-SUBHYD-001 Kickoff

Execution mode: authority-model decision (proposed ADR-0024) + `SC-SUBHYD-001` amendment +
`ksatadj` re-adjudication. Codex authors; operator ratifies ADR-0024; Claude reviews.

Autonomy: execute end-to-end (ADR-0024 draft → `ksatadj` intent extraction → `SC-SUBHYD-001`
anchor/invariant → openWEPP-vs-intent re-adjudication → verdict + handoff). **Hold the contract
amendment as binding authority until ADR-0024 is operator-ratified.** Do not fix openWEPP physics
here; do not apply `qdry`/`ksflag`.

## Why you're here

STAGE2-LATQCC closed `CONTRACT-GAP`: H2637's `latqcc` is equation-correct on bound-valid operands,
but the magnitude rides on the **provisional forest `ksatadj` conductivity**, which has **no
external physical authority**. Operator decision: for an empirical model like `ksatadj`, **the
legacy wepp-forest reference-implementation *intent* is the authority** — the algorithm is the spec.
Establish that as a general principle (ADR-0024) and apply it to `ksatadj`/`SC-SUBHYD-001`, closing
the gap.

## ADR-0024 — the principle (general)

**Reference-implementation *intent* as a contract-authority (A0) anchor for empirical models
lacking external physical authority** (forest `ksatadj`, frost `qdry`, `ksflag`). An `SC-*` contract
MAY anchor on the legacy reference **algorithm** (`wepp-forest_260430_baseline/src/*.for`, cite
`file:routine`).

**Distinct from ADR-0017 / authority class A6 — do not conflate:**
- A6 (legacy binary **behavior/output**) stays a **flag**, investigation-only — the binary has bugs;
  its output is never a target.
- The new anchor is the source **intent** (the algorithm the authors meant), a normative A0
  provenance **modulo legacy bugs** (flag any bug as non-authoritative).

Slot into `correctness-authority-model.md` as an A0-anchor provenance basis (not a new rank).
**Numbering:** `docs/decisions/` tops at 0022; "ADR-0023" was the abandoned array proposal
(NO-GO, never committed) — this is **ADR-0024**.

## ksatadj application — close the gap

1. **Extract intent** from `wepp-forest_260430_baseline/src/{infpar,input}.for` (the working
   `wepp-forest/src/` confirms the lineage): the algorithm that forms the sat-fraction equivalent
   conductivity — inputs, the saturation-fraction relation, bounds, the WB19 lateral-conductivity
   lineage. Record as *intent*; flag any legacy bug as **non-authoritative**.
2. **Encode in `SC-SUBHYD-001`:** anchor `REF-SUBHYD-KSATADJ-INTENT` (cite `file:routine`) + an
   equivalent-conductivity invariant (`INV-SUBHYD-NNN`), per `science-contract-authoring-procedure.md`.
3. **Re-adjudicate openWEPP vs the *intended algorithm*** (not the legacy numeric output):
   - **Match → `CORRECT`** → openWEPP realizes the authoritative `ksatadj` model → **close the
     FARPOINT01 71% flag** with the resolution documented.
   - **Divergence → `OPENWEPP-DEFECTIVE`** → **Defect-Closure ExecPlan** item-1 (ADR-0018), citing
     the divergence. No fix here.

## Hard stops

1. **Intent, not behavior** — extract the algorithm; flag legacy bugs as non-authoritative; never
   encode a bug as the spec; compare openWEPP to the intended algorithm, not the binary output.
2. **ADR-0017 preserved** — binary behavior (A6) stays a flag. If ADR-0024 is not ratified, the
   amendment + re-adjudication HOLD.
3. **Contract-first** — the contract is the authority; openWEPP conforms. No physics fix here.
4. **Scope** — `qdry`/`ksflag` are future (the ADR covers them; do not apply). Irrigation deferred.

## Constraints / truthfulness

- No openWEPP physics fix; defects route to a defect-closure follow-on. No legacy-binary parity target.
- Cite legacy source `file:routine`; label evidence Static / Ran. A verdict cites the new invariant +
  the intended algorithm, not the legacy output. Markdown + contract lints clean.

## Required reading

- `docs/work-packages/20260618-refimpl-intent-authority-ksatadj-subhyd-001/package.md`
- `docs/work-packages/20260618-stage2-latqcc-h2637-magnitude-001/artifacts/{latqcc_disposition,latqcc-operand-plausibility,latqcc-handoff}.md`
- `docs/specifications/correctness-authority-model.md` (A0-A6; the placement)
- `docs/decisions/0017-...comparator-is-flag-not-target.md`, `0011-architecture-first-top-down-science-contracts.md`, `0018-defect-closure-execplans-conversion-rule.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contract-authoring-procedure.md`, `science-contract-provenance-spec.md`
- `wepp-forest_260430_baseline/src/infpar.for`, `wepp-forest_260430_baseline/src/input.for`
- `AGENTS.md`, `docs/work-packages/AGENTS.md`
