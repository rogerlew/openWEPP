# DOCOPT01 Kickoff — frequently-loaded doc deduplication

Scope: local repository documentation mechanical-refactor task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).

Autonomy: execute end-to-end — slim `index.md`, extract the AGENTS.md procedures,
update cross-references, run the full closure loop, reconcile any broken tests,
and record gate evidence through disposition — without asking for direction. Ask
or HOLD only at a declared boundary (see Hard stops).

## What and why

Two of the most-frequently-loaded docs carry duplicated (not load-bearing)
content. `docs/specifications/science-contracts/index.md` (43KB) is in the
mandatory required-reading of every kernel WP and is ~35KB duplicative changelog;
`AGENTS.md` (24KB) loads every session and is ~47% authoring procedures. Remove the
duplication without losing any binding fact, and close with the workspace test
loop + test reconciliation (docs and tests are coupled here, like the refactor
series).

Read `package.md` first — it is the authority for the two deliverables, the
coverage-check anti-loss gate, the closure/test requirement, and the protected
boundaries. **Non-negotiable: no binding/discoverable fact lost; no contract or
kernel-behavior change; this is move/dedup only.**

## Required reading
- `package.md` (this WP)
- `docs/standards/mechanical-refactor-authoring-guide.md` (§1.2 ambient-skip
  override, §6.2 closure loop, §9 artifact set)
- `docs/specifications/science-contracts/index.md` (target D1)
- `AGENTS.md` (target D2)
- `docs/specifications/science-contract-spec.md` (registry field schema; `notes`
  is optional)
- `docs/codex_exec_plans.md`

## Tasks (ordered, concrete end states)
1. **D1 index.md slim.** For each registry row, verify its `notes` amendment
   content is present in that contract's own `## Change log`/invariants
   (`rg <amendment-id> docs/specifications/science-contracts/contracts/SC-<DOMAIN>-001.md`).
   If present → replace `notes` with ≤1-line scope note (or empty). If absent →
   HOLD that row (do not delete). Condense the ADR0017 note to governance +
   invariant-ID pointers. Fix "Entry Order" to keep only the sort rule; relocate
   surviving unique narrative to a changelog/provenance location. Record the
   per-row coverage result in `artifacts/index-notes-coverage-check.md`.
2. **D2 AGENTS.md extraction.** Move "Kernel Work-Package Preparation Procedure
   (Required)" and "Prompt Wording Guidance (Required)" to
   `docs/standards/kernel-work-package-preparation.md` and
   `docs/standards/prompt-wording-guidance.md`; leave a binding pointer for each in
   AGENTS.md; update required-reading lists/cross-references to the new paths.
3. **Doc-path integrity.** Confirm every pointer + required-reading path resolves
   (`rg`/link check). Record in `artifacts/doc-path-integrity.md`.
4. **Closure loop (Ran, recorded with exit codes):**
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - `cargo deny check`
   The ambient "never run tests" instruction is overridden here (§1.2).
5. **Test reconciliation.** If any test references removed `notes` content, a moved
   AGENTS.md section, or a doc path, update it to the new structure (path/structure
   only — never change what a test asserts about behavior). Record breakage +
   reconciliation in `artifacts/test-reconciliation.md`. If the loop is clean,
   record that explicitly.
6. Dual review/disposition/verification; line-count governance disposition.

## Outputs
- Slimmed `index.md`; two new `docs/standards/` procedure docs + AGENTS.md pointers.
- `artifacts/`: index-notes-coverage-check, doc-path-integrity, gate logs
  (fmt/clippy/test/deny with exit codes), test-reconciliation, dual
  review/verification, disposition, worker-handoff, line-count-governance checklist.

## Hard stops
- A registry/Entry-Order/ADR0017 note holds a fact NOT in the owning contract →
  HOLD that row; migrate the fact into the contract changelog first, do not delete.
- A closure-loop failure is a genuine regression not reconcilable as
  path/structure-only → HOLD with command-level evidence.
- Any edit would change contract authority or kernel behavior → out of scope, stop.
