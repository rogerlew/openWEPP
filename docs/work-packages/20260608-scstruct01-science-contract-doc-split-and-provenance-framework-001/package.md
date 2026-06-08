# SCSTRUCT01 — Science-Contract Doc Split + Provenance Framework

Status: queued
Created: 2026-06-08
Series: `scstruct` (science-contract structure / context optimization)
Execution mode: package-end-to-end (phased; declared stop-boundaries per phase)

## Objective

Cut the agent-context cost of the `SC-<DOMAIN>-<NNN>` science contracts — the
heaviest files on the kernel-work required-reading path — **without dropping any
binding obligation**. Two coupled deliverables:

1. **Framework (docs):** split the conflated
   `docs/specifications/science-contract-authoring-procedure.md` into a slim
   workflow doc + a new artifact-schema doc + a new provenance-sidecar doc, and
   define a **Binding Exposure Index** + lint gate so a contract's normative core
   can be consolidated safely (nothing binding can be silently lost).
2. **Reference consolidation (proof):** apply the framework to one contract
   end-to-end (recommended: `SC-WATBAL-001`, the worst offender) to validate it,
   then enumerate the remaining contracts as right-sized follow-on packages.

This is a **binding-semantics-preserving restructure**, not a contract revision:
no `INV-*` / obligation may be added, removed, or weakened — only relocated and
indexed. The framework exists to *prove* that conservation.

## Background

Context-optimization pass (sibling to the work-packages README split,
`7ed6045`). Measured findings (`Ran` sizes, `Static` structure read):

- `SC-WATBAL-001.md` = 308KB / 2,504 lines (~77k tokens — the size of the whole
  pre-split work-packages README). Normative core ≈ lines 22–281 (Purpose →
  Variables → Algorithm → **Invariants table**, the binding surface).
- ~2,240 lines after that are per-package **Addendum** prose (HPHYS0298…0314,
  WB12/CLIM05/06/WB14/15/IRRIG10…). Sampling shows much is explicitly superseded
  ("HPHYS0298's all-window `OPENWEPP-DEFECTIVE` verdict is historical and is
  superseded by HPHYS0299 plus ADR0017"), with the binding residue already
  promoted into numbered invariants (`INV-WATBAL-087/088`).
- Same shape: `SC-SNOWFREEZE-001` (150KB), `SC-SYSTEM-001` (117KB),
  `SC-RUNOFFPART-001` (93KB), `SC-SUBHYD/EVAP/PLANT` (75–88KB each).

Design was consulted with Codex (advisory; Codex could not read files in its
sandbox, so its boundary proposal was validated by Claude against the actual
doc text). Convergent conclusions, with two Codex upgrades adopted: a **third**
doc for the sidecar (don't fold lifecycle into schema), and the **Binding
Exposure Index + lint** as the anti-drop safeguard. One Codex proposal rejected:
renaming the procedure doc — **689** files reference
`science-contract-authoring-procedure.md` by path, so the name is preserved.

## Deliverables

### D1 — Three framework docs

| Doc | Content | Disposition |
|---|---|---|
| `docs/specifications/science-contract-authoring-procedure.md` | **Workflow only:** derivation-authority order, ADR-0017 comparator governance, required per-cycle file layout, dual-review gate, disposition workflow, fix/verification gate, promotion-gate logic, change management, prompt templates, and the *enforcement* halves of Symbol-Alias / Unit-Governance ("apply before promotion", fail-closed on suspicious ratio) | **slim in place — keep filename (689 inbound refs)** |
| `docs/specifications/science-contract-spec.md` | **Artifact schema:** canonical location, required section set + order, the "Contract Draft Requirements" field list, the *structure* halves of Symbol-Alias (required alias-table columns) and Unit-Governance (required unit columns), and the **Binding Exposure Index** section requirement | new |
| `docs/specifications/science-contract-provenance-spec.md` | **Sidecar format + lifecycle:** sidecar container naming, status vocabulary (`active` / `superseded` / `historical`), required provenance fields (`verdict`, `effective_date`, `superseded_by`, `canonical_binding_ids`, `migration_target`), retention rules, and the normative rule "history statements are non-binding unless cross-referenced from the core's Binding Exposure Index" | new |

Boundary heuristic (validated against the 308-line procedure doc): **"the
document MUST contain field X" → schema; "must be reviewed in phase Y / what to
do if it fails" → procedure.**

### D2 — Binding Exposure Index (schema) + lint gate

- Each contract **core** carries a `Binding Exposure Index` enumerating every
  active/historical addendum → its `status` → mapped `INV-*` / `OBL-*` IDs.
- A static lint (Codex implements; e.g. `tools/`-resident or cargo xtask, wired
  as a package-precondition check) **fails** when:
  - any addendum marked `binding`/`active` has no core `INV-*`/`OBL-*` mapping, or
  - the core references an `INV-*` that no longer exists, or
  - a sidecar entry lacks required provenance fields.
- This is the mechanism that makes consolidation safe and prevents
  re-accretion: future addenda land in the sidecar by rule; binding residue must
  be promoted to an invariant.

### D3 — Reference consolidation (recommended target: `SC-WATBAL-001`)

Apply the framework to one contract, contract-first:
1. Build the `SC-WATBAL-001` Binding Exposure Index over every addendum section.
2. Adjudicate each addendum: **map-to-existing-`INV-*`** (relocate, no semantic
   change) **or** promote an unpromoted-but-binding obligation to a new `INV-*`
   (recorded as a binding *addition* requiring the full review gate — flagged,
   not silent) **or** mark `historical`/`superseded` and move the narrative to
   the provenance sidecar.
3. Leave a slim normative core conforming to `science-contract-spec.md`.
4. Run the lint gate green; prove the before/after `INV-*`/`OBL-*` set is
   **conserved** (the crosswalk artifact).

### D4 — Follow-on queue

Enumerate per-contract consolidation follow-ons (SNOWFREEZE, SYSTEM,
RUNOFFPART, SUBHYD, EVAP, PLANT, …) as right-sized `scstruct` packages, ordered
by size × read-frequency, each reusing this framework.

## Authority Envelope

### In-scope files
- `docs/specifications/science-contract-authoring-procedure.md` (split/slim)
- `docs/specifications/science-contract-spec.md` (new)
- `docs/specifications/science-contract-provenance-spec.md` (new)
- the reference contract `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
  and its new provenance sidecar
- the binding-exposure lint tool + its wiring
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  `docs/specifications/science-contracts/README.md`,
  `docs/specifications/science-contracts/index.md`, and required-reading lists —
  **pointer/text updates only** to reference the split docs

### Allowed edit classes
- Author/slim the three framework docs.
- Relocate contract prose between core and sidecar.
- Add the Binding Exposure Index section to the reference contract.
- Promote a genuinely-unpromoted binding obligation to a new `INV-*` **only**
  through the full dual-review/disposition/verification gate, recorded as a
  flagged binding addition (never silent).
- Implement + wire the lint.
- Update inbound pointers to the split docs (text only, no path rename of the
  procedure doc).

### Protected boundaries (do not cross)
- **No binding-semantics change by side effect.** No `INV-*`/`OBL-*` may be
  removed or weakened. Additions are allowed only via the flagged review gate.
- **No production kernel / runtime code edit.** This WP does not touch physics.
  If a consolidation appears to require a kernel change, that is out of scope →
  HOLD + follow-on.
- **No procedure-doc path rename** (689 inbound refs).
- **No comparator re-tiering** — ADR-0017 governance text is relocated verbatim,
  not reinterpreted.

## Acceptance criteria
1. The three framework docs exist, each single-purpose, with the boundary
   heuristic applied; the procedure doc retains its filename and all prior
   workflow obligations.
2. `science-contract-spec.md` defines the Binding Exposure Index; the procedure
   and kernel-profile docs reference the split docs without broken links.
3. The lint exists, runs, and is green on the reference contract; it
   demonstrably **fails** on a seeded violation (red/green proof).
4. `SC-WATBAL-001` is consolidated: slim core + provenance sidecar, with a
   conserved `INV-*`/`OBL-*` crosswalk (before == after, plus any flagged
   additions explicitly dispositioned). Measured core token size recorded
   before/after.
5. Follow-on per-contract queue authored.
6. Dual review + disposition + dual verification complete; no undispositioned
   finding; kernel-profile compliance checklist confirms no kernel-affecting
   change leaked in.

## Milestones / phase shape
- **Phase 0 (docs-only inventory):** enumerate all `SC-*` contracts (size,
  addendum-section count); fully classify every `SC-WATBAL-001` addendum as
  `maps-to-INV` / `unpromoted-binding` / `historical`. Stop-boundary: if any
  addendum's binding status is genuinely undecidable from the contract +
  citations, record it and route to a science-review follow-on rather than
  guessing.
- **Phase 1 (framework, operator-signed):** author/slim the three docs + define
  the Binding Exposure Index + provenance format. Operator acceptance required
  (changes normative authoring authority for all contracts).
- **Phase 2 (reference consolidation, contract-first):** Codex executes the
  `SC-WATBAL-001` consolidation + lint, one move per checkpoint, with the
  conservation crosswalk. Stop-boundary: if the live binding set cannot be
  conserved without a science decision, HOLD + follow-on.
- **Phase 3 (closeout):** dual review/verification/disposition; follow-on queue;
  pointer updates; record core-size before/after.

## Legitimate HOLD conditions
- An addendum carries a binding obligation that is **not** representable as a
  faithful `INV-*` relocation and needs a science decision → HOLD, route to a
  science-review follow-on; do not drop it, do not invent an invariant.
- Consolidation appears to require a kernel/runtime change → HOLD (out of
  envelope).

## Dependencies
- `docs/specifications/science-contract-authoring-procedure.md` (subject of split)
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/specifications/unit-governance.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/codex_exec_plans.md`
- `docs/standards/mechanical-refactor-authoring-guide.md` (the relocation work is
  mechanical; the binding crosswalk is its conservation proof)
- Precedent for procedure/schema separation already in repo:
  `docs/specifications/external-authority/{suite-schema.md,suite-template.md}`
  vs its procedure; `docs/specifications/wepp-input-specification-authoring-procedure.md`
  vs `docs/specifications/wepp-input-files/specs/*.spec.md`

## Autonomy
Execute end-to-end across phases through disposition. Phase 1 requires operator
sign-off before Phase 2 begins (normative authority change). Otherwise proceed
without intermediate direction; ask only if hard-blocked or at a declared
stop-boundary.
