# Preimplementation Contract Gate: INTVAL-EROSION-TOE-001

Status: `PASS`

Evidence class: **Ran + Static**.

- Canonical authority was amended first: `SC-SED-001` revision 54 adds pinned
  `profil.for` terminal-station normalization and an EROD16 guard/test
  obligation.
- The pinned baseline at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` directly sets
  `slen = xinput(nslpts)` and `xstar = xinput / slen`.
- The contract-derived near-terminal vector was added before production code
  changes and failed with exit 100 because current code divides by declared
  physical length (`logs/04-erod16-terminal-normalization-red.*`).
- The implementation boundary is narrow: use the validated terminal station
  as the normalized coordinate denominator while retaining declared-length
  validation and the existing typed route-toe guard.
- Kernel-profile checklist: canonical contract, algorithm/invariant, guard
  behavior, dimensional-role distinction, test vector, and baseline provenance
  are explicit. No external constitutive suite is defined for this geometry
  normalization seam.
- Production implementation was not edited before these gates passed.

## Candidate-3 stability batch

Status: `PASS`

- `SC-PLANT-001` revision 20 now binds the pinned perennial
  cap-before-increment branch and valid zero cap before growth runtime edits.
- `SC-PERC-001` revision 30 and `SC-INFILE-SOIL-001` revision 0.1.12 now bind
  exact-zero `kslast` as an active impermeable boundary before subsurface
  runtime edits. Existing `SC-PERC-001#INV-PERC-017` already binds every
  positive hourly same-pass ingress; no contract amendment was needed for that
  producer defect.
- Three contract-derived vectors were added before production edits and all
  three failed for their named mechanisms: growth positive-only validation,
  restrictive-conductivity positive-only validation, and the tiny-positive
  infiltration closure residual (`logs/08-three-family-red.*`, exit 100).
- Kernel-profile coverage includes canonical algorithm branches, input/domain
  roles, typed negative/non-finite failures, exact-zero degenerate states,
  guard/test mappings, and pinned baseline provenance.
- No tolerance, fixture, suite posture, or production code was changed before
  these gates passed.

## INTVAL-FROST-THAW-CLEAR-001

Status: `PASS`

- `SC-SNOWFREEZE-001` revision 117 now binds pinned `frostn.for:686` and
  `frwatc.for:80-137` egress authority before runner production edits.
- The amended `INV-SNOWFREEZE-012` requires a material thaw-complete outcome
  to retain its original coarse layer basis until R4W applies the outcome
  projection and post-`frwatc` scalar together exactly once. It prohibits a
  pre-ingress synthetic clear against the pre-exchange scalar while retaining
  the existing stale-clear authority for nonmaterial outcomes.
- `SC-WATBAL-001#INV-WATBAL-095` independently binds the freeze/thaw exchange
  identity, and `SC-PERC-001` binds residual water only over unfrozen depth.
- The contract-derived vector uses zero active theta, `0.05` residual theta,
  `0.4 m` depth, `0.026064975283605526 m` prior frozen depth, and the exact
  `0.0013032487641802763 m` thaw credit. Before production correction it fails
  with the watchlist-family debit signature (`logs/11-frost-thaw-clear-red.*`).
- The implementation boundary is one material-outcome discriminator; no
  tolerance, layer redistribution, residual-domain, percolation, fixture, or
  fallback change is authorized.
- Production implementation was not edited before these gates passed.

## INTVAL-EROSION-CLASS-FRACTION-001

Status: `PASS`

- `SC-SED-001` revision 55 now binds the existing dimensionless `1e-15`
  do-40 class floor and requires the floored vector to be renormalized to the
  authoritative routed `ldbot` before label-50 caps.
- Pinned `enrich.for:341-377` establishes the floor/cap ordering but contains
  a trace-load defect: five absolute floors can exceed `ldbot`, after which
  label 50 redistributes a negative shortfall and creates negative class mass.
- Candidate 5 exposed that exact mechanism in OR-H0081 and OR-H0204. A GDB
  capture for H0081 found `ldbot = 1.5907591421467216e-19` and final class
  loads `[-3.642639932051419e-15, 9.50638048587754e-16, ...]`.
- The contract-derived deterministic trace vector was added before production
  edits and failed with fractions `[3143.153396..., -785.538349..., ...]`
  (`logs/17-enrichment-floor-red.log`, exit 100).
- The implementation boundary is limited to the already-defined floor seam:
  only when at least one class is raised by the floor, restore the floored
  nonnegative vector to `ldbot` before caps. No publication clamp, tolerance,
  fixture, total-load equation, or ordinary no-floor path may change.
- Production implementation was not edited before these gates passed.

## INTVAL-GROUNDWATER-TERMINAL-STORAGE-001

Status: `PASS`

- `SC-GWBASEFLOW-001` revision 0.1.2 now requires enabled-run execution
  provenance to publish authoritative `S_0`, `S_N`, `Qb_N`, and `Qs_N`
  alongside cumulative recharge and exports. Disabled runs retain absent,
  rather than zero-filled, recurrence operands.
- The amendment preserves the canonical daily timing
  `S_i = S_(i-1) + D_i - Qb_(i-1) - Qs_(i-1)` and requires both its terminal
  identity and the post-current-export full-run ledger to reconstruct.
- The restarted frozen-candidate H2637 output independently exposed the gap:
  cumulative recharge is `3668.610172576748 m3`, baseflow is
  `3547.636225849919 m3`, seepage is zero, and the unobservable inferred
  post-export terminal storage is `120.97394672682913 m3`.
- The contract-derived real H2637 regression was added before production
  edits and failed exactly at the missing initial-storage manifest operand
  after three production legs (`logs/22-groundwater-terminal-storage-red.log`,
  exit 100).
- The implementation boundary is observability only: retain existing day
  outputs in the run summary and serialize them in execution provenance. No
  recurrence, coefficient, HBP/pass schema, fixture, tolerance, export, or
  routing change is authorized.
- Production implementation was not edited before these gates passed.
