# LANED-NOB-001 Kickoff Prompt

Repository: `/workdir/openWEPP`. Starting branch: `main` (current head).
Push target: `origin main` on completion of gates, per repo convention.

Scope: local repository science-contract/kernel defect closure; flat-file
reads/edits and local builds/tests only; no external connectivity.

Execution mode: package-end-to-end.

Task: close defect `LANED-NOB-001` end-to-end under the Correction
Authority Envelope in
`docs/work-packages/20260710-laned-router-h2637-34yr-negativeoutletbin-defect-closure-001/package.md`:
the Lane D active production default fail-closes
(`NegativeOutletBin`, lane 8 day 2621) on the canonical 34-year H2637
endpoint. Reproduce from the staging recipe in the package (canonical
wepp-forest WB05A inputs + the Tier-1 management patch; ~50 s to failure),
attribute to a named mechanism, then convert: contract
amendment/confirmation, contract-derived failing tests, pre-implementation
gate, production correction, validation (full 34-year completion, off-path
byte identity, oracle/cohort/workspace gates, timing evidence), dual
review, disposition.

Required reading: use the Core/Conditional tiers in `package.md` and record
the exact byte budget in `artifacts/required-reading-map.md` before edits.
The discovery evidence is
`docs/audits/20260710_h2637_34yr_laned_active_endpoint_audit.md`.

Constraints: contract-first sequencing; positivity by construction inside
the flux update (rev-41 posture) — no clamping, damping, mass
injection/removal, or guard loosening; no hybrid-stepper revival
(ADR-0037); snow/winter physics is a protected boundary (seam booking is in
scope, the physics is not); daily/off path byte identity is an acceptance
obligation. The candidate-mechanism notes in the package are
assessment-class, non-binding — attribution is yours.

HOLD legitimacy audit: a HOLD must name and prove a declared boundary
(missing/contradictory authority, proven out-of-envelope mechanism, or
validated invalid-input non-defect) and why the in-envelope correction
cannot close now. Do not hold while source reading, contract/test work,
implementation, or validation remains possible.

Subagent requirement: REQUIRED. This prompt authorizes subagent delegation
to the package's reviewer/verifier roles and a suite runner for heavy
gates, with writes bounded to package artifacts.

Autonomy: execute all phases through disposition without asking for
direction unless hard-blocked.
