Execute package `20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001`.

Objective: close the D16 selected-cohort `mn_corn_h4` active-run blocker by
publishing daily PL `Hc`/`canhgt` from the legacy WEPP growth equation and
wiring active/shadow Lane D friction operand builders to the post-growth daily
surface.

Required posture:

- Contract-first: amend `SC-PLANT-001` and `SC-OFEROUTE-001` before code.
- No guard relaxation: positive post-growth LAI still requires positive finite
  canopy height.
- No surrogate physics: use the baseline `grow.for` equation
  `canhgt = (1 - exp(-bbb * vdmt)) * hmax`.
- Do not change hybrid default promotion, mesh policy, route coefficients, or
  broad active selector posture.

Subagent authorization: this package explicitly authorizes spawning/delegation
to science-authority review, implementation review, comparator/timing, package
QA, and verification subagents for contract/source review, selected-cohort
rerun verification, gate review, and disposition review. Expected outputs are
package-local `artifacts/review-*.md`, `artifacts/verification-*.md`, and
compact timing/comparator evidence. Write access is read-only unless a worker
is explicitly assigned a bounded package-artifact correction.

Write all evidence under the package-local `artifacts/` directory and finish
with `final-disposition.md`.
