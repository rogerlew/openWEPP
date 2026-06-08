# DOCOPT01 — Frequently-Loaded Doc Deduplication (index.md + AGENTS.md)

Status: queued
Created: 2026-06-08
Series: `docopt` (documentation context optimization)
Execution mode: package-end-to-end (mechanical; full closure loop required)
Discipline: follows `docs/standards/mechanical-refactor-authoring-guide.md`
(move-only/dedup edits + the §6.2 Required closure loop + §1.2 ambient-test-skip
override). This is a **documentation mechanical refactor**: no behavior, formula,
threshold, or contract-authority change.

## Objective

Cut the per-load context cost of the two most-frequently-loaded docs by removing
**duplicated** content — not load-bearing content — while preserving every binding
and discoverable fact, and closing with the full workspace test loop plus explicit
**test reconciliation** (docs and tests are coupled in this repo; the refactor
series has been absorbing test reconciliation, and this package adopts the same
gate discipline).

Audited findings (Static, 2026-06-08):
- `docs/specifications/science-contracts/index.md` = 43KB / 158 lines, of which the
  legitimate registry is ~40 lines; ~35KB is duplicative changelog. It is in the
  **mandatory required-reading of every kernel work-package** (AGENTS.md), so it
  loads constantly.
- `AGENTS.md` = 24KB / 398 lines, loaded **every Codex session**; ~189 lines (~47%)
  are two detailed procedures relevant only during WP/prompt authoring.

## Deliverables

### D1 — `index.md` registry slim (~43KB → ~9KB target)
- **Strip the per-contract `notes` changelog.** The `notes` column has become a
  per-contract amendment log (SC-ROUTE/SC-SED/SC-SYSTEM each ~3–4KB). `notes` is
  **optional** per the file's own Registry Fields schema. Replace each cell with a
  short (≤1 line) scope/lifecycle note or empty.
- **Per-row coverage check (mandatory, the anti-loss gate):** before stripping a
  contract's `notes`, confirm the same amendment history exists in **that
  contract's own `## Change log` / invariant table**. If a fact is in the registry
  note but NOT in the contract, do **not** delete it — HOLD that row and record the
  gap (the note is then the only record and must first be migrated into the
  contract changelog).
- **Condense the "ADR0017 registry note"** (lines ~25–63) to the active governance
  statement + the invariant-ID pointers (the HPHYS0314–0320 narrative duplicates
  the contracts' invariant tables).
- **Fix "Entry Order"** (lines ~101–158): move the HPHYS#### narrative notes out
  (they duplicate invariant authority); keep the actual rule (`Sort rows by
  contract_id`). Relocated narrative, if not already in contracts, goes to a
  provenance/changelog location — never silently dropped.
- Keep all required registry fields, governance pointers, and the field schema.

### D2 — `AGENTS.md` procedure extraction (~24KB → ~13KB target)
- Move **"Kernel Work-Package Preparation Procedure (Required)"** (lines ~125–248)
  and **"Prompt Wording Guidance (Required)"** (lines ~249–315) to dedicated docs
  under `docs/standards/` (e.g. `kernel-work-package-preparation.md`,
  `prompt-wording-guidance.md`).
- Leave a **binding pointer** in AGENTS.md for each ("Required before preparing
  kernel WPs / authoring kickoff prompts: read `docs/standards/...`") so the
  obligation stays discoverable and normative — this is relocation, not deletion.
- Update any **required-reading lists / cross-references** that pointed at the
  inline AGENTS.md sections to the new paths.
- AGENTS.md is Codex's owned file; Codex executes this deliverable.

## Test integration & closure (required — the package's emphasis)

Docs and tests are coupled here (code-side catalogs in
`crates/openwepp-sim-contract/src/units_mod/` mirror documented catalogs;
contract-derived tests reference invariant IDs; doc-path/required-reading lists may
be path-checked). Treat this like a mechanical refactor:

1. **Run the full §6.2 closure loop, in-shell, recorded with exit codes:**
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `cargo deny check`
2. **Test reconciliation:** any test that references removed registry-note content,
   a moved AGENTS.md section, a doc path, or a required-reading list must be
   updated to the new location/structure. Record what broke and how it was
   reconciled (mirror the refactor-series gate logs). If the loop is clean, record
   that explicitly — do not assume.
3. **Doc-path integrity:** every pointer and required-reading path must resolve
   after the moves (`rg`/link check); record the check.
4. The §1.2 ambient "never run tests" instruction is **overridden** by these gates.

## Authority Envelope

### In-scope
- `docs/specifications/science-contracts/index.md`
- `AGENTS.md` + new `docs/standards/` procedure docs
- required-reading lists / cross-references pointing at the moved AGENTS.md content
- any test files requiring reconciliation after the doc changes

### Allowed edit classes
- Remove duplicative optional `notes`/changelog from `index.md` (after coverage check).
- Relocate AGENTS.md procedures + leave binding pointers.
- Update doc cross-references / required-reading paths.
- Reconcile broken tests to the new doc structure (move-only intent; no assertion
  semantics change beyond path/structure).

### Protected boundaries (do not cross)
- **No binding/discoverable fact lost.** Every removed note must be proven present
  in the contract changelog first (coverage check); unproven → HOLD, not delete.
- **No contract / invariant authority change**; `index.md` registry fields,
  governance pointers, and the field schema stay intact.
- **No kernel/runtime behavior change**; test reconciliation is path/structure
  only, never a change to what a test asserts about behavior.
- **No new content authored** beyond pointers + relocated text.

## Acceptance criteria
1. `index.md` reduced toward ~9KB; every stripped `notes` cell verified present in
   the owning contract's changelog (coverage-check artifact), or HOLD-listed.
2. AGENTS.md procedures relocated with binding pointers; required-reading remains
   discoverable; all moved paths resolve.
3. Full §6.2 closure loop **Ran** and recorded with exit codes; any test breakage
   reconciled and documented; clean runs recorded explicitly.
4. Doc-path/required-reading integrity check recorded.
5. Dual review + disposition + dual verification; line-count governance
   disposition (N/A unless reconciliation touches `.rs` files); no undispositioned
   finding.

## Legitimate HOLD conditions
- A registry note (or Entry-Order/ADR0017 note) contains a fact **not** present in
  the owning contract → HOLD that row, migrate the fact into the contract changelog
  first (or route a follow-on), do not delete.
- A closure-loop failure is a real regression that cannot be reconciled as
  path/structure-only → HOLD with command-level evidence.

## Dependencies
- `docs/standards/mechanical-refactor-authoring-guide.md` (closure loop, artifact set)
- `docs/specifications/science-contract-spec.md` (registry field schema authority)
- `docs/codex_exec_plans.md`
- `docs/work-packages/README.md`

## Autonomy
Execute end-to-end, mechanically. Run the closure loop and reconcile tests without
asking for direction. Ask/HOLD only at a declared boundary: a registry note whose
fact is not in the owning contract, or a closure-loop regression that is not
path/structure-reconcilable.
