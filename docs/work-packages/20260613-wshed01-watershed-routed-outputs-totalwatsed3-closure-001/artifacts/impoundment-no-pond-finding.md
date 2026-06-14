# Impoundment No-Pond Finding

Status: W-A executed

Evidence mode: Ran + Static

## Finding

`jpond=0` on the arboreal-dendrite watershed is valid no-impoundment input.
The current openWEPP impoundment parser rejects it as `IMP-E-004`, so the
classification is **parser defect**, not invalid substrate.

## Current openWEPP Behavior

Ran:

- CLI exits with `CLIWAT-E-010`.
- Wrapped parser error:
  `IMP-E-004: line 2 invalid domain value '0' for jpond; expected >= 1`.
- No watershed output files are written.

Static:

- `watershed_impoundment.rs:526-557` reads explicit `datver` and then the
  single-token `jpond` line.
- `watershed_impoundment.rs:581-588` rejects `declared_count == 0` with
  `DomainError { field: "jpond", allowed: ">= 1" }`.
- `watershed_impoundment.rs:598-628` contains structural-count reconciliation,
  but the zero-count rejection happens before that reconciliation.
- `openwepp-cli-watershed.rs:239-254` passes
  `expected_structural_count: Some(structure.summary.impoundment_count)` and
  wraps the parser error as `CLIWAT-E-010`.

## Fixture Validity

Ran:

- `pw0.imp` is:

```text
99.1
0
```

- `pw0.str` has `15` channel rows and no `elmt=3` impoundment rows.

Static fixture/legacy alignment:

- Legacy `wshinp.for:228-253` initializes `npond = 0` and increments it only
  when a watershed structure element has `elmt(i).eq.3`.
- Legacy `wshini.for:321-345` initializes impoundment routines only when
  `npond.gt.0`; only then does it read `jpond` and compare structure count to
  impoundment-file count.
- Legacy `impint.for:523-525` loops `ipond = 1, npond`; with `npond=0`, the
  per-impoundment payload loop is naturally skipped.
- Legacy `wshdrv.for:1228-1296` guards daily, monthly, yearly, and end-of-run
  impoundment outputs with `if (npond.gt.0)`.

## Required W-B Contract

W-B should accept `jpond=0` as a typed empty impoundment set when the expected
structural impoundment count is zero. This is not a silent default:

- Preserve typed parse errors for non-numeric and negative `jpond`.
- Preserve max-count validation.
- Preserve mismatch errors when structure expects one or more impoundments but
  the `.imp` file declares zero.
- Preserve active-impoundment fixtures and payload validation.
- Ensure both strict and compatibility modes are explicit about zero-count
  behavior under `expected_structural_count`.

## Claude review (2026-06-14) — classification independently confirmed; W-B contract endorsed

Evidence mode: Ran (legacy + openWEPP source read).

**The parser-defect classification is correct and independently verified:**

- Legacy **ran arboreal-dendrite successfully** (its watershed outputs are on
  disk), with this exact `pw0.imp` (`jpond=0`). openWEPP fail-closing on input
  legacy accepts is, by definition, a defect (the WBVAL02 inverse: there the
  "guard too strict" hypothesis was *invalid input*; here legacy proves the
  input is *valid*).
- Mechanism confirmed: legacy `wshini.for:319` gates the entire impoundment
  read behind `if (npond.gt.0)` — for `npond=0` it **never reads `pw0.imp`**,
  so `jpond=0` is never validated. openWEPP
  (`openwepp-cli-watershed.rs:~246`) parses the `.imp` **unconditionally**
  despite already holding `structure.summary.impoundment_count` (its analog of
  `npond`), and the parser rejects `jpond=0` via
  `DomainError{field:"jpond", allowed:">= 1"}`.

**W-B contract endorsed** as contract-first-faithful: gating the `jpond=0`
acceptance on `expected_structural_count == 0` (and preserving negative/
non-numeric/max-count/structure-mismatch guards) ties the leniency to the
legacy `npond>0` semantic rather than loosening the bare `jpond >= 1` check —
the right instinct (cf. the WBVAL05/SC-PERC lesson: fix the consumption gate,
not the typed guard).

**One W-B robustness note (not a blocker):** legacy reads the `.imp` file
*not at all* when `npond=0`, so the file's presence/content is irrelevant in
the no-impoundment case. Codex's W-B (read it, accept `jpond=0`) handles the
present-with-`jpond=0` case (sufficient for arboreal-dendrite), but the fully
legacy-faithful no-impoundment semantic also covers an **absent `.imp`** on a
zero-impoundment watershed. W-B should decide explicitly whether a missing
`.imp` when `impoundment_count == 0` is accepted (legacy: yes, never read) or
still required — and pin it in the contract, so the no-pond handling is
complete, not just the present-file variant.

## W-B Disposition (2026-06-14)

W-B chose the narrow schema-v1 correction:

- `inputs.pw0_imp` remains required by
  `openwepp-watershed-runfile-contract.md`.
- A supported explicit `.imp` file with `jpond=0` is valid only when the
  watershed structure declares zero impoundments.
- Missing `.imp` acceptance for zero-impoundment watersheds is a possible
  future runfile-contract change, not part of W-B. This avoids silently
  changing required input-file binding semantics while clearing the
  arboreal-dendrite blocker.
