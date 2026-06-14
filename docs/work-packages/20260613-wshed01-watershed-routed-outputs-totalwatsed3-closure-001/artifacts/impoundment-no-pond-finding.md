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
