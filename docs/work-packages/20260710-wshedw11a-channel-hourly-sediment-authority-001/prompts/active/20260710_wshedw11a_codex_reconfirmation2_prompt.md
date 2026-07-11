# WSHED-W11A Codex Re-Confirmation Prompt (Cycle 2)

Local repository engineering work in `/workdir/openWEPP`, starting from
`main` at the commit applying `SC-ROUTE-001` v53. Read-mostly; write access
limited to one confirmation artifact in this package's `artifacts/`.

## Context

Your re-confirmation (`artifacts/codex_reconfirmation.md`) closed
M1/M2/H2/M3 and left two residuals: H1 (the total-vs-per-unit-length
`qlat` conflation) and L1 (record inconsistencies). Both are dispositioned
in the re-confirmation cycle section of
`artifacts/codex-review-disposition.md` and applied in v53.

## Task

Verify exactly the two residuals against the v53 text:

1. H1 residual — `INV-ROUTE-016` and the addendum now bind `qlat(it)`
   (published wave total, `m^3 s^-1`, partition-only) and the derived
   per-unit-length `qlat_eff(it) := qe(it)/leff(it)` as distinct symbols
   with distinct Variables rows and units; raw-total and total/`lc`
   substitution invalid; storage expression all-total; vectors 1/11
   re-pinned (vector 11 distinguishes both wrong-unit aliases); the
   unit-bridge carries the derived-normalization note; the pre-existing
   grouped Variables row is split so `qlat_eff` is `ft^3 s^-1 ft^-1`.
2. L1 residuals — `final-disposition.md` vector count and
   verification-note-4 wording; `w11-handoff.md` v53 header.

Write `artifacts/codex_reconfirmation2.md` with closure status per
residual and a final verdict: `RATIFIED` / `RATIFIED-WITH-AMENDMENTS` /
`REOPEN`. If RATIFIED, state explicitly that `WSHED-W11-HOLD-001` stands
lifted and W11 may resume at Phase B on the v53 authority.
