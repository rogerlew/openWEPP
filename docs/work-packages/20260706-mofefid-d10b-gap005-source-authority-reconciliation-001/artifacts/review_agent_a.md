# Review Agent A (D10B)

Evidence class: Static + Ran (read-only: log cross-checks verbatim, wc -l,
git status/diff, BEI lint PASS-DEFERRED, independent primary extraction of
Davis 3.16-3.20 / Mingham 28b/31a/31f/31g / Tseng abstract). No cargo runs
(taken from the attributed gate record for verification agents to confirm).

Recommendation: **GO-WITH-AMENDMENTS**.

CRITICAL: none.

MAJOR (all contract-wording reconciliation; none changes physics/tests):
1. Frontmatter `contract_version: 23` lags revision rows 24/25.
2. Rev-25 ratification not reconciled into three binding statements:
   (a) INV-011 still says "monotone error decrease" while the ratified
   shape is within-tolerance-at-every-resolution + non-divergence (the
   evidence is non-monotone 1.3->1.8->2.6%); (b) test-vector row (b) still
   says "TV non-increase" vs the ratified TV-transient bound; (c)
   Algorithm item 4 / INV-007 still describe frozen-alpha celerity while
   production landed TRUE dq/dh celerity.
3. Two production decisions absent from the Algorithm Specification:
   material-interface faces carry zero dissipative flux; boundary-adjacent
   limiter-stencil mirroring (both on the Case-4 acceptance path).
4. Algorithm item 6 says "piecewise-linear flux integral" while the landed
   (stronger) mechanism is the piecewise-constant conservative bin series.
5. Demotion rationale ground 3 ("divergence under refinement" as evidence
   the trace was non-converged) is refuted by the S4 outcome (the
   divergence was the defective solver under `k_o=200`); the demotion
   SURVIVES on clean-room + transcription-error-spec + operand-confound
   grounds and must be requalified, not reversed.

MINOR:
6. gate-results missing the BEI-check row (lint run by reviewer:
   PASS-DEFERRED, 6 binding rows, 5 science-review-follow-on).
7. package.md Progress/Surprises completion-tracking (partially stale
   observation; verify current state at closure).
8. TV monitor scope caveat (homogeneous steps, uniform-material faces,
   pre-commit) should be attached to residual item 1.
9. Disposition/verification artifacts pending (expected mid-S5).

Adversarial probes answered: TV-bound ratification honest (stale vector
text is the defect); trace demotion justified as an authority decision
with ground 3 requalified; bin-mean hydrograph semantics verified safe for
all current consumers (grep: none outside `ofe_routing` reads the shape;
laned_shadow consumes mass surfaces + a count; D13 producer flip not
landed).

Verified sound: verbatim scheme-to-primary match (reviewer's own
extraction); ledger booked-equals-actual and exact face telescoping by
construction; 19-OFE machine-epsilon residuals verbatim in logs;
fail-closed guards preserved; no surrogate physics; line counts OK;
conversion rule honored.
