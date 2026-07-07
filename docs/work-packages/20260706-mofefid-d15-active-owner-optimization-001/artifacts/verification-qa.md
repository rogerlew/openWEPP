# Verification — QA lane (post-fix)

Status: **EXECUTED**. Evidence mode: **Ran** + Static as labeled.

Verifier: in-session verification pass over the QA-lane findings against the
fixed tree and re-run gates.

- **QA-H1**: gate table has NO IN-FLIGHT/FAIL rows (both former in-flight
  gates PASS on the final tree); review/verification/disposition artifacts
  exist and are non-placeholder; handoff tense corrected. The catalog
  documents' claims were re-read against the final state and are accurate.
  The sequencing lesson is recorded in `final-disposition.md`.
- **QA-H2**: the conservation-acceptance bar is now met two ways: (1) the
  enforced SEAM cross-ledger check over independent ledgers (measured
  5.0e-14; proven non-vacuous by the pre-fix ~0.11 % failure it caught);
  (2) independent reconstruction from PRODUCED OUTPUTS — the manifest
  latqcc total reconstructs the published parquet `sbrunv` column sum to
  1 ulp, and the behavioral pass-surface deltas (runvol/sbrunv/tdet) were
  independently extracted from the parquet, not from producer counters.
  Anti-tautology: (b) is explicitly labeled router-internal in contract +
  lineage; the R4B-zero-by-construction note is in both.
- **QA-M1..M4**: Static — grep-verified the rev-27 contract now contains the
  mesh-basis rule, the corrected latqcc booking wording, the D12
  uniform-fallback active disposition, and the three named follow-on gates
  (including erosion water-magnitude).
- **QA-M5**: Static — each consumer section carries all six rubric elements.
- **QA-L1..L3**: Static — measured line counts + WARN acknowledgment;
  `total_source_m3` semantics stated; all three seam-fixed timing logs in
  `logs/`.
- Contract lint: Ran — `check_sc_binding_exposure.py` PASS-DEFERRED
  (unchanged posture); `git diff --check` clean.
