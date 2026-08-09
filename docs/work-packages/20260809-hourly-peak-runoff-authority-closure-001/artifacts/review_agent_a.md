# Independent Hydrology / Science Review A

Status: `executed`

Evidence class: `Static exact-commit review + Ran artifact inspection`

Reviewed identity: `c7dbfefe7c7c67137101ddd2c63cd4c4c2e062fa`

Verdict: `HOLD`

Reviewer B output was not consulted. Review covered `SC-WATBAL-001` version
166, `SC-INFILE-HBP-001` version 0.2.4, the direct-runtime runoff producer,
erosion and publication consumers, focused tests, package evidence, and the
in-progress Topanga run.

## Findings

### `REVIEW-A-CRITICAL-001` — source depths do not independently close to event runoff

- Evidence:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1393-1427,
  1430-1458,1490-1539`;
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:47-75`;
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:1254-1259`.
- Finding: the producer sums WB14 excess, hourly saturation return, and routed
  melt only to prove the source total is positive, normalizes those operands to
  weights, then multiplies the normalized weights by `q_runoff_m`. It never
  checks that the independently produced source-depth total reconstructs
  `q_runoff_m`. The melt-only test makes the defect explicit: `0.004 m` of
  source depths is accepted for `0.002 m` event runoff and silently scaled to
  weights `0.75/0.25`. Thus the asserted hourly/event closure is true by
  construction, not independent mass closure, and an arbitrarily inconsistent
  source ledger can determine the peak hour and magnitude.
- Impact: violates the package's anti-tautology acceptance, `INV-WATBAL-102`,
  the operand-lineage claim that the normalized hourly runoff depth is
  authoritative after closure, and HBP's claim that hourly volumes reconstruct
  the event runoff. It also leaves the physical role of `hourly_routed_melt_m`
  ambiguous: if it is runoff, scaling changes mass; if it is pre-infiltration
  supply, it is not an authoritative runoff-depth limb.
- Proposed disposition: `accepted`. Define each source limb at one common
  post-partition runoff lineage, require `sum(source limbs) ~= Q` under an
  explicit scale-aware tolerance, and compute the peak directly from those
  closing hourly depths. If routed melt is supply rather than runoff, route it
  through the hourly partition before peak assembly. Add a negative vector for
  the current `0.004 m` source / `0.002 m` event mismatch.

### `REVIEW-A-MAJOR-002` — “source-free” positive-roundoff rule does not inspect every source limb

- Evidence:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:740-760,
  1362-1375`;
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:141-168`;
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:296`.
- Finding: positive partition runoff at or below `1e-12 m` is changed to exact
  zero whenever the WB14 hourly-excess sum is zero. The decision function is
  not given hourly routed melt, runon provenance, or any other positive source
  lineage. Saturation return is added later and therefore survives, but the
  implementation cannot prove the partition residual is source-free across the
  full source inventory required by the contract. The focused test proves only
  preservation of a WB14-excess-backed value.
- Impact: a small positive source-backed runoff can be erased even though
  `INV-WATBAL-102` states that any positive hourly source must preserve its
  runoff, however small. This is a source-custody and mass-loss defect, not a
  harmless peak floor.
- Proposed disposition: `accepted`. Make canonicalization consume and verify
  the complete source ledger (or a producer-certified exact source-free flag),
  and add sub-tolerance routed-melt-only and runon-only vectors. Do not infer
  source absence from WB14 excess alone.

### `REVIEW-A-MAJOR-003` — inter-OFE runon still has invented uniform timing

- Evidence:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:640-680`;
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_dc01.rs:28-43`;
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md:297,1257-1275`.
- Finding: positive surface or lateral runon with no reconstructible upstream
  hourly shape is distributed uniformly over 24 hours. The code and retained
  test explicitly call this a “uniform fallback.” That invented time base then
  enters WB14 and may carry the production peak, contrary to the new
  fail-closed source-custody rule and the claim that runon-only runoff retains
  its modeled hour.
- Impact: multi-OFE peak timing can still be synthetic even though the package
  claims the same source-complete shape serves transfer, erosion, HBP, and
  peak. A single-OFE Topanga cohort cannot close this multi-OFE source-path
  defect.
- Proposed disposition: `accepted`. Require a closing upstream hourly surface
  and lateral handoff for positive runon; hard-fail when it is absent. Replace
  the uniform-fallback test with missing-shape rejection and explicit in-hour
  runon custody tests through the real multi-OFE handoff.

## Confirmed Correct Boundaries

- The scientific quantity is legitimately the maximum of 24 modeled hourly
  mean depth rates, with earliest-hour tie resolution. It is not described as
  an instantaneous or subhourly peak (`SC-WATBAL-001.md:128-130,1260-1264`).
- WB19 saturation-return depths are read from
  `hourly_saturation_carry_m` and retain their produced hour in peak assembly
  (`direct_runtime/runoff.rs:803-818`; saturation-only vector at
  `direct_runtime_dc01.rs:79-95`).
- Erosion consumes the internal `m s^-1` depth rate, while public hillslope
  output applies area once to produce `m^3 s^-1`
  (`direct_runtime/erosion.rs:520-543`;
  `direct_runtime/01_publication.rs:582-614`). The HBP minor-1 contract then
  reconstructs the same volumetric maximum from hourly volumes and does not
  apply area again (`SC-INFILE-HBP-001.md:116-123,238-243`).
- Rectangular-equivalent duration is correctly distinguished from rainfall
  duration, hydrograph duration, and time to peak.
- The package and Topanga artifacts remain hillslope-only, diagnostic/cohort
  evidence with no coefficient fitting, observed-flow validation, legacy
  parity, watershed-routing, or instantaneous-peak claim. At review time the
  full 1,088-trial log was still running, and the retained summary truthfully
  labels only the bounded probe as passed; no premature complete-cohort claim
  was found.

The three material findings prevent acceptance of the mass/source-custody
claims and therefore require `HOLD` until corrected and independently tested.
