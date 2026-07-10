# WSHED-W11A Codex Re-Confirmation Prompt

This is local repository engineering work in `/workdir/openWEPP`, starting
from `main` at the commit that applied the `SC-ROUTE-001` v52 post-hoc
closure amendments. Read-mostly; write access limited to one confirmation
artifact in this package's `artifacts/` directory.

## Context

Your post-hoc review (`artifacts/codex_posthoc_review.md`) returned
`REOPEN` (`WSHED-W11A-POSTHOC-001`) with five required closure amendments.
All were accepted and applied in `SC-ROUTE-001` v52
(`artifacts/codex-review-disposition.md` maps each finding to its
amendment; the v52 revision row summarizes). Both H2 terminal claims were
independently re-verified against pinned `dcap.for:160-262` before being
bound into contract text.

## Task

Verify closure of your five required amendments against the v52 working
text (`git show` the closing commit or read the file):

1. H1 — the `INV-ROUTE-016` hydraulic-profile operand map
   (`qe/qt/qlat := q1(it)/qin(it)/qlat(it)`; storage posture; invalid
   aliases) + vector 11.
2. M1 — the `t_exp(it)`/`t_norm(it)` operand split with the constructive
   closure equation + vector 1 pins.
3. M2 — `d_i` and `rho_soil` definitions (Variables rows +
   `INV-ROUTE-018`/`019` text).
4. H2 — the pinned-`dcap.for`-as-realization binding, the two named
   terminals, `GAP-ROUTE-014`, and vectors 10(b)/(c).
5. M3/L1 — the three anchor narrowings and the package-record
   reconciliations (`contract-disposition.md`, `gate-results.md`,
   `authority-matrix.md` third correction, `final-disposition.md` reopen
   record).

Write `artifacts/codex_reconfirmation.md` with closure status per
amendment (`closed` / `still-open`) and a final verdict: `RATIFIED` /
`RATIFIED-WITH-AMENDMENTS` / `REOPEN`. If RATIFIED, state explicitly that
`WSHED-W11-HOLD-001` may stand lifted and W11 may resume at Phase B.
