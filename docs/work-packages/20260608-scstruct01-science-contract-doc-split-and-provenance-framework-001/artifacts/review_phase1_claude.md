# SCSTRUCT01 Phase 1 Framework Review — Claude Code (reviewer)

Reviewer: Claude Code
Gate: Phase 1 framework operator sign-off (precedes Phase 2 consolidation)
Evidence mode: **Static** — read the three framework docs, the procedure diff
against `HEAD`, and the Phase 0 inventory artifact. No commands beyond `git
diff`/`grep`/`wc` were run. The binding-exposure lint does not yet exist
(Phase 2), so lint behavior is unverified.

Scope reviewed:
- `docs/specifications/science-contract-authoring-procedure.md` (slimmed)
- `docs/specifications/science-contract-spec.md` (new)
- `docs/specifications/science-contract-provenance-spec.md` (new)
- `artifacts/phase0-watbal-addendum-classification.md` (Phase 0 input to Phase 2)

## Boundary verification (passed)

- `git status` confirms `SC-WATBAL-001.md` is **not** modified — no binding rows
  touched.
- No production kernel/runtime files modified; changes are confined to the three
  spec docs + work-package artifacts.
- Procedure-doc path preserved (no rename), so the 689 inbound references hold.

## Overall assessment

The split is sound and exceeds a mechanical move: the procedure correctly shed
artifact-schema content and **added** a contract-first Authoring Workflow and a
Binding Exposure Workflow; the spec is where the "Contract Draft Requirements"
landed (not lost) and upgrades the former prose alias/unit rules into formal
schema tables plus a precise Binding Exposure Index + lint contract; the
provenance-spec is complete and recursively applies the split principle.

The **framework (Phase 1)** is acceptable with minor amendments. The **Phase 0
classification** is a separate artifact with a material quality/truthfulness
issue that does not gate Phase 1 but **must** be resolved before Phase 2
relocates any contract narrative.

## Findings (severity-ordered)

### F4 — HIGH — Phase 0 "classification" is string-matching, not adjudication
- Artifact: `artifacts/phase0-watbal-addendum-classification.md:58-134`.
- Issue: the classification is keyword/token matching, not a reasoned binding
  call. Direct evidence from the artifact itself: the `maps-to-existing-INV`
  rows for `HPHYS0308` (`:68`) and `HPHYS0260` (`:134`) each list ~94 invariants
  (`INV-WATBAL-001…094`) — i.e. every `INV-*` token scraped from a reproduced
  guard-map table, which is not a mapping. `unpromoted-binding` reduces to "no
  `INV-*` string in the section body"; `historical-or-superseded` reduces to a
  hit on "historical/superseded/retracted".
- Why it matters: the conservative direction (over-marking `unpromoted-binding`)
  is safe because the framework forces those through the review gate. The
  **unsafe** direction is `historical-or-superseded`, which authorizes
  relocation out of binding core — and it is assigned by keyword, including large
  sections: `WB16 Required Coupling Surfaces` (`:87`, 103 lines) and `HPHYS0260`
  (`:134`, 180 lines). Relocating those on a keyword hit risks moving a live
  obligation to a non-binding sidecar.
- Proposed disposition: **amend** — does not block Phase 1. Before Phase 2,
  every `historical-or-superseded` section must be re-adjudicated semantically
  (section text + cited invariants) prior to any relocation. The exact mechanism
  is Codex's call; the requirement is that no narrative leaves binding core on a
  token match.

### F5 — MEDIUM — Phase 0 artifact overstates completeness (truthfulness)
- Artifact: `artifacts/phase0-watbal-addendum-classification.md:4` and `:139`.
- Issue: status is `complete-through-static-classification` and the stop-boundary
  note states "This static pass found no undecidable section," yet the table
  carries ~8 rows classified `undecidable` (e.g. `:63`, `:66`, `:72`, `:75`,
  `:78`, `:81`, `:84`, `:86`). The artifact contradicts its own table and
  presents a mechanical first cut as a completed classification.
- Why it matters: a downstream agent reading "complete / no undecidable" could
  treat the table as adjudicated authority and skip the F4 re-adjudication.
- Proposed disposition: **amend** — re-characterize the artifact as a mechanical
  inventory / first cut, and resolve the `undecidable`-count contradiction.

### F3 — MEDIUM — Spec front matter expands 6 → 11 required fields
- Artifact: `docs/specifications/science-contract-spec.md:32-49`.
- Issue: the prior procedure required 6 front-matter fields
  (`contract_id`, `title`, `status`, `maturity`, `owner`, `contract_version`);
  the spec now marks 11 as required, adding `producer_scope`, `consumer_scope`,
  `evidence_level`, `last_reviewed`, `supersedes`, `superseded_by`.
- Why it matters: if existing `SC-*` files do not all carry the 5 added fields,
  the spec silently renders them non-conformant — converting a
  binding-semantics-**preserving** restructure into a latent mass-noncompliance
  event across all 35 contracts.
- Proposed disposition: **amend** — scope the added fields as required for
  new/migrated contracts with backfill recorded as a tracked migration item, or
  confirm by inventory that all existing contracts already conform.

### F1 — LOW — Disposition `decision` vocabulary changed (`amended` dropped)
- Artifact: `docs/specifications/science-contract-authoring-procedure.md:197`.
- Issue: the disposition `decision` set changed from
  `accepted | amended | rejected` to `accepted | rejected | deferred | follow-up`.
  `amended` was dropped; `deferred`/`follow-up` added. This harmonizes with the
  work-packages README dual-review taxonomy and is plausibly an intentional
  improvement.
- Why it matters: any existing `disposition.md` using `amended` is now
  off-vocabulary; silent vocabulary drift can break downstream disposition lint.
- Proposed disposition: **accept if intentional** — confirm the harmonization is
  deliberate and note the retired value; otherwise restore `amended`.

### F2 — LOW — `complements` list dropped ADR-0003
- Artifact: `docs/specifications/science-contract-authoring-procedure.md:19-23`.
- Issue: the slim procedure's "complements" list dropped
  `docs/decisions/0003-parity-semantic-not-bit.md` (and the science-contracts
  README) while adding ADR-0017 + the two new specs.
- Why it matters: ADR-0003 (semantic-not-bit parity) is governing authority for
  contract tolerance language and is worth retaining as a pointer.
- Proposed disposition: **amend** — re-add the ADR-0003 reference.

## Recommendation

**GO-WITH-AMENDMENTS** for the Phase 1 framework.

- F1–F3 are doc-level amendments to fold into the framework before it is treated
  as ratified.
- F4–F5 do **not** block Phase 1 acceptance but are **hard preconditions for
  Phase 2**: the `historical-or-superseded` set must be semantically
  re-adjudicated, and the Phase 0 artifact's completeness claim corrected, before
  any consolidation relocates contract narrative. The framework's own lint +
  flagged-addition gate already prevents silently dropping a *mapped* obligation,
  so residual risk is concentrated in the small, checkable historical class.

Findings surface issues + evidence; disposition and any architectural choice are
left to Codex per the package's dual-model convention.
