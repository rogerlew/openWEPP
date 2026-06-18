# REFIMPL-INTENT-AUTHORITY + ksatadj/SC-SUBHYD-001 Application

Status: **complete 2026-06-18** — **ADR-0024 ratified** (operator, 2026-06-18);
`SC-SUBHYD-001` v33 source-intent `ksatadj` authority (`INV-SUBHYD-032`,
`REF-SUBHYD-KSATADJ-INTENT`) authored and **independently reviewed**
(`artifacts/review-claude-independent.md` — both sides of the divergence verified
against source); openWEPP re-adjudicated **`OPENWEPP-DEFECTIVE`** (forms
`sat_frac = Σθ/Σul` vs source-intent `avsat/(avpor·avcpm)`). The fix routes to the
defect-closure follow-on **`REFINTENT001-KSATADJ-SATFRAC`**; the FARPOINT01 71%
flag stays open until that lands and is re-run. `qdry`/`ksflag` remain future
applications of ADR-0024.

Package type: **Authority-model decision (proposed ADR-0024) + `SC-SUBHYD-001` authority-anchor
amendment + `ksatadj` re-adjudication.** Three coupled deliverables: a general governance
principle (operator-ratified), its first concrete application to `ksatadj`, and the re-adjudication
that closes the FARPOINT01 lateral-magnitude flag. The proposed ADR is operator-ratified (like
ADR-0022); `qdry`/`ksflag` are *future* applications of the same principle, not done here.

## Why this package exists

STAGE2-LATQCC verified H2637's `latqcc` is equation-correct on bound-valid operands but closed
**`CONTRACT-GAP`**: the absolute lateral magnitude rides on the **provisional forest `ksatadj`
sat-fraction conductivity**, which has **no external physical authority** (no closed-form
derivation; "not standard WEPP, no physical why on record"). Operator decision (2026-06-18): for an
empirical model like `ksatadj`, **the legacy wepp-forest reference-implementation *intent* is the
authority** — *the algorithm is the spec*. This package establishes that as a general principle and
applies it to `ksatadj`/`SC-SUBHYD-001`, closing the gap.

## The principle (proposed ADR-0024) — general

**Reference-implementation *intent* as a contract-authority (A0) anchor for empirical/conceptual
models lacking external physical authority.** For a model with no closed-form or external-authority
derivation (e.g. forest `ksatadj`, frost `qdry` harmonic, `ksflag`), an `SC-*` contract MAY take an
authoritative anchor from the **legacy reference-implementation algorithm** (e.g.
`wepp-forest_260430_baseline/src/*.for`), citing source `file:routine`.

This is **distinct from — and does not weaken — ADR-0017 / authority class A6**:

- **A6 (legacy binary *behavior* / output) stays a flag**, investigation-only. The binary carries
  bugs (non-conservation, disabled routines); its output is **never** a target.
- **The new anchor is the source *intent* (the algorithm the authors meant)** — a normative A0
  provenance, **modulo legacy bugs**, which must be explicitly flagged as non-authoritative when
  the anchor is written (you encode the *intended* algorithm, not a bug).

It slots into `correctness-authority-model.md` as a provenance basis for A0 anchors (not a new
rank), and relates to `[[project_legacy_contract_vs_bug]]` (legacy can encode a contract openWEPP
must replicate). **Numbering note:** `docs/decisions/` tops at 0022; "ADR-0023" was the *abandoned*
array-authoritative proposal (proposed, NO-GO, never committed) — this is **ADR-0024**.

## The `ksatadj` application (first instance — closes the gap)

1. **Extract the `ksatadj` intent** from `wepp-forest_260430_baseline/src/{infpar,input}.for` (the
   working `wepp-forest/src/` confirms the same lineage): the **algorithm** that forms the
   sat-fraction equivalent conductivity — its inputs, the saturation-fraction relation, the bounds,
   the lineage into the WB19 lateral conductivity. Record the algorithm as *intent*; flag any
   legacy bug (non-conservation, disabled branch) as **non-authoritative**.
2. **Encode it in `SC-SUBHYD-001`** — a new authority anchor `REF-SUBHYD-KSATADJ-INTENT` citing the
   source `file:routine`, plus an invariant (`INV-SUBHYD-NNN`) for the equivalent-conductivity
   formation that openWEPP must satisfy. Author per `science-contract-authoring-procedure.md`.
3. **Re-adjudicate openWEPP's `ksatadj`** against the code-intent authority: does openWEPP's
   implementation realize the intended algorithm?
   - **Match → `CORRECT`:** openWEPP faithfully implements the now-authoritative `ksatadj` model →
     the H2637 lateral magnitude is correct *by the authority that now governs it* → **close the
     FARPOINT01 71% flag** (document the resolution).
   - **Divergence → `OPENWEPP-DEFECTIVE`:** openWEPP departs from the intended algorithm → a
     **Defect-Closure ExecPlan** item-1 (ADR-0018), citing the divergence. No fix in this package.

## Hard stops

1. **Intent, not behavior.** Extract the *algorithm* the legacy code expresses; **flag legacy bugs
   as non-authoritative** and do not encode them as the spec. The re-adjudication compares openWEPP
   to the **intended algorithm**, **not** to the legacy binary's numeric output.
2. **ADR-0017 preserved.** Legacy binary behavior (A6) stays a flag; nothing here makes the legacy
   *output* a target. If the operator-ratified ADR-0024 is not approved, the `SC-SUBHYD-001`
   amendment + re-adjudication HOLD.
3. **Contract-first.** The `SC-SUBHYD-001` amendment is the authority; openWEPP conforms to the
   contract, not the reverse. No openWEPP physics fix here (defects → defect-closure follow-on).
4. **Scope discipline.** `qdry`/`ksflag` are *future* applications — the ADR covers them, this
   package does **not** apply them. Irrigation deferred.

## Scope

In scope: the proposed ADR-0024 (general principle + `correctness-authority-model.md` placement);
the `ksatadj` intent extraction; the `SC-SUBHYD-001` `REF-SUBHYD-KSATADJ-INTENT` anchor + invariant;
the openWEPP-vs-intent re-adjudication + verdict + handoff (close flag / defect-closure ExecPlan).

Out of scope:

- **No openWEPP physics fix** (defects route to a defect-closure follow-on).
- **No `qdry`/`ksflag` application** (future packages under the same ADR).
- **No legacy-binary parity target** — intent is the authority, behavior stays a flag.
- No conservation re-litigation (closed by MAGPARITY01); no export/transfer rework.

## Acceptance Criteria

- **Proposed ADR-0024** authored (general principle; A0-anchor provenance for empirical models;
  explicit ADR-0017/A6 distinction; `correctness-authority-model.md` placement), **operator-ratified**
  before the `SC-SUBHYD-001` amendment is treated as authority.
- **`ksatadj` intent extraction** from the pinned legacy source, with `file:routine` citations and
  any legacy bug flagged non-authoritative.
- **`SC-SUBHYD-001` amendment:** `REF-SUBHYD-KSATADJ-INTENT` anchor + an equivalent-conductivity
  invariant, authored per the contract procedure (lint/registry-consistent).
- **Re-adjudication verdict** (`CORRECT` / `OPENWEPP-DEFECTIVE`) with openWEPP traced against the
  *intended algorithm*, citing the new invariant — not the legacy numeric output.
- **Handoff:** FARPOINT01 flag resolution (if `CORRECT`) or a Defect-Closure ExecPlan item-1 (if
  `OPENWEPP-DEFECTIVE`). The `qdry`/`ksflag` follow-ons noted.
- Evidence labeled Static / Ran. Markdown + contract lints clean. (No Rust gates unless code touched
  — none expected; a defect routes to a follow-on.)

## Deliverables

- `artifacts/adr0024-refimpl-intent-authority.md` (the proposed ADR text — general)
- `artifacts/ksatadj-intent-extraction.md` (the legacy algorithm, `file:routine`, bug flags)
- `artifacts/sc-subhyd-ksatadj-anchor.md` (the `REF-SUBHYD-KSATADJ-INTENT` anchor + invariant draft)
- `artifacts/ksatadj-openwepp-vs-intent.md` (the re-adjudication trace + verdict)
- `artifacts/refintent-handoff.md` (flag resolution / defect-closure ExecPlan / qdry-ksflag follow-ons)
- `artifacts/refintent_disposition.md`

## Dependencies

- `docs/work-packages/20260618-stage2-latqcc-h2637-magnitude-001/artifacts/{latqcc_disposition,latqcc-operand-plausibility,latqcc-handoff,review-claude-independent}.md` (the CONTRACT-GAP this closes)
- `docs/specifications/correctness-authority-model.md` (A0-A6 ranking; where the principle slots)
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md` (A6/behavior-as-flag, preserved)
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`; `docs/decisions/0018-defect-closure-execplans-conversion-rule.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md` (the amendment target; INV-SUBHYD-003/012/018)
- `docs/specifications/science-contract-authoring-procedure.md`; `docs/specifications/science-contract-provenance-spec.md`
- `wepp-forest_260430_baseline/src/{infpar,input}.for` (the `ksatadj` intent source; `wepp-forest/src/` confirms lineage)
- The `ksatadj` lineage references ([[reference_wepp_forest_frost_ksflag_ksatadj]] context)
- `AGENTS.md`; `docs/work-packages/AGENTS.md`

## Subagent Requirement

None required. If the operator authorizes subagents, the legacy `ksatadj` source tracing
(`infpar.for`/`input.for`) and the openWEPP-vs-intent comparison are parallelizable. Record evidence.

## Autonomy

Execute end-to-end through the ADR-0024 draft, the `ksatadj` intent extraction, the `SC-SUBHYD-001`
anchor/invariant draft, and the openWEPP-vs-intent re-adjudication. **Hold** the contract amendment
as binding authority until ADR-0024 is operator-ratified. The verdict is the deliverable:
`CORRECT` closes the FARPOINT01 flag; `OPENWEPP-DEFECTIVE` opens a clean defect-closure handoff.
Extract *intent*, flag legacy bugs as non-authoritative, and never make the legacy binary output a
target. Do not fix openWEPP physics here; do not apply `qdry`/`ksflag`.
