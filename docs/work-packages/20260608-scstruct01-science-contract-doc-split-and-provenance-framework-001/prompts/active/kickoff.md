# SCSTRUCT01 Kickoff — science-contract doc split + provenance framework

Execution mode: package-end-to-end (phased; Phase 1 needs operator sign-off
before Phase 2)

Autonomy: execute end-to-end — Phase 0 inventory, Phase 1 framework docs, then
(after operator sign-off) Phase 2 reference consolidation + lint, Phase 3
closeout — without asking for direction on intermediate steps. Ask only if
hard-blocked or at a declared stop-boundary.

## What and why

Science contracts are the heaviest files on the kernel-work required-reading
path. `SC-WATBAL-001.md` is 308KB / 2,504 lines, of which the normative core is
~260 lines and ~2,240 lines are per-package Addendum prose — much of it
explicitly superseded, with the binding residue already in the `INV-WATBAL-*`
table. This package makes the core consolidatable **without dropping any binding
obligation**, by splitting the authoring meta-doc and adding a Binding Exposure
Index + lint, then proves it on `SC-WATBAL-001`.

Read `package.md` in this directory first — it is the authority for scope,
deliverables, the authority envelope, and the protected boundaries. The
non-negotiable boundary: **binding-semantics-preserving.** No `INV-*`/`OBL-*`
removed or weakened; additions only via the flagged review gate; no kernel code
touched.

## Required reading
- `package.md` (this WP)
- `docs/specifications/science-contract-authoring-procedure.md` (the doc being split)
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` (reference target)
- `docs/specifications/external-authority/suite-schema.md` +
  `docs/specifications/external-authority/suite-template.md` (in-repo precedent
  for procedure/schema/template separation)
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/codex_exec_plans.md`
- `docs/standards/mechanical-refactor-authoring-guide.md`

## Phase 0 — inventory (docs-only)
Enumerate every `SC-*` contract (size, addendum-section count). For
`SC-WATBAL-001`, classify **every** addendum section as one of:
`maps-to-existing-INV` / `unpromoted-binding` / `historical-or-superseded`,
citing the contract text + anchors for each call. Stop-boundary: any addendum
whose binding status is genuinely undecidable from the contract + citations is
recorded as `undecidable` and routed to a science-review follow-on — do not
guess. Emit `artifacts/phase0-watbal-addendum-classification.md`.

## Phase 1 — framework docs (operator sign-off gate)
Author/slim the three docs per `package.md` D1, applying the boundary heuristic
("MUST contain field X" → schema; "reviewed in phase Y / what if it fails" →
procedure):
- slim `science-contract-authoring-procedure.md` (keep filename — 689 inbound refs)
- new `science-contract-spec.md` (artifact schema + Binding Exposure Index section)
- new `science-contract-provenance-spec.md` (sidecar format + lifecycle)

Then **stop for operator acceptance** — this changes normative authoring
authority for all contracts. Record the gate in `artifacts/`.

## Phase 2 — reference consolidation (contract-first, after sign-off)
Per `package.md` D3, on `SC-WATBAL-001`, one move per checkpoint:
1. Build the contract's Binding Exposure Index over every addendum.
2. Relocate `historical`/`superseded` narrative to a provenance sidecar; keep
   only binding residue in `INV-WATBAL-*`/obligation rows.
3. Promote any `unpromoted-binding` obligation to a new `INV-*` **only** through
   the full dual-review/disposition/verification gate — flagged binding
   addition, never silent.
4. Implement the binding-exposure lint; show it **red** on a seeded violation
   then **green** on the consolidated contract.
5. Produce the conservation crosswalk: before/after `INV-*`/`OBL-*` set is
   identical except for explicitly-dispositioned flagged additions. Record core
   token size before/after.

## Phase 3 — closeout
Dual review (`review_agent_a.md` / `review_agent_b.md`), disposition, dual
verification; kernel-profile compliance checklist confirming no kernel-affecting
change leaked in; author the per-contract follow-on queue
(`artifacts/followon-queue.md`); update inbound pointers in the kernel-profile
doc, science-contracts README/index, and required-reading lists to the split
docs (text only — no procedure-doc path rename).

## Hard stops
- Cannot conserve the live binding set without a science decision → HOLD +
  follow-on. Do not drop an obligation; do not invent an invariant.
- Consolidation appears to need a kernel/runtime edit → HOLD (out of envelope).
