# Independent Review B

Evidence class: `Static` and `Ran`

Disposition: `HOLD`

Review subject: terminal-current working tree based on
`45d49090214b4702d11a04aafe5d5ccade7ba440`. Reviewer B did not read Reviewer
A's review.

## What passes

- The equations, generalized thresholds, product, FAO-56 photoperiod geometry,
  and 21-sample FIFO implementation are consistent with the cited primary
  formulations. No equation-fidelity defect was found.
- Scope containment passes. The new crate has no reverse workspace dependency,
  and source search found no production consumer. The package correctly limits
  its maximum claim to `PASS-PROCESS-KERNEL`; it makes no canopy, biomass,
  litter, snow, ET, erosion, or empirical-validation claim.
- Roadmap and registry placement are broadly correct while the package is
  active: `CANOPY-PHENOLOGY-01` remains active, integration remains separately
  queued as `CANOPY-PHENOLOGY-02`, and `SC-PLANT-001` remains `in_review`/
  `draft`.
- `crates/openwepp-plant-phenology/src/lib.rs` is 583 lines, below the
  2,000-line warning threshold.
- Ran successfully on the reviewed tree: `cargo fmt --check`, strict package
  Clippy, focused Nextest (7/7), the three package-listed Markdown lints, and
  `git diff --check`.

## Findings

### B-01 — High — The contract amendment is not traceability-complete

`INV-PLANT-028..031` are added at
`docs/specifications/science-contracts/contracts/SC-PLANT-001.md:371`, but the
Guard Map ends with `INV-PLANT-027` at line 471. The Symbol Alias Map begins at
line 473 and contains no mapping from the new canonical GSI names to the public
Rust names at `crates/openwepp-plant-phenology/src/lib.rs:24`, line 85, and line
112. This conflicts with the amendment workflow in
`docs/specifications/science-contracts/AGENTS.md:27` and line 29. The package's
`artifacts/contract-amendment.md:5` consequently overstates amendment closure.

Action: add explicit guard-map rows for all four new invariants and deterministic
canonical-to-Rust alias rows for parameters, forcing, indicators, `GSI21`, and
history state; then update the amendment evidence to point to those rows.

### B-02 — High — Startup history semantics are project policy presented as direct published law

Jolly et al. state that daily `iGSI` is integrated/calculated as a 21-day running
or moving average; the paper does not define initialization behavior for the
first 20 simulation days. Nevertheless, `INV-PLANT-029` at
`SC-PLANT-001.md:372` labels the available-sample FIFO rule `[DIRECT][Static]`,
and lines 759-762 prescribe an initially empty FIFO and a one-sample first
result. The implementation follows that prescription at
`crates/openwepp-plant-phenology/src/lib.rs:134`, so this is an authority and
evidence-classification defect, not an implementation-fidelity defect.

Action: retain or revise the startup policy only after explicitly identifying it
as an openWEPP inference/design decision, documenting its rationale and restart
implications, and changing the invariant evidence tag to distinguish the
published 21-day window from inferred warm-up semantics.

### B-03 — High — State chronology is not part of the public contract or restorable state

The contract and API call the state an exact trailing *daily* window, but
`GsiState` retains only values (`lib.rs:128-160`) and `advance` admits every call
without checking chronology (`lib.rs:174-196`). Repeated, skipped, or reversed
ordinal days therefore produce a valid “21-day” result that is actually the
mean of the last 21 calls. `try_from_history` also restores no temporal anchor.
Neither the addendum state table at `SC-PLANT-001.md:713` nor its daily algorithm
at line 759 assigns cadence/continuity responsibility.

Action: contract the chronology boundary before passing the kernel. Either add a
year-aware temporal key to state and enforce consecutive daily admission, or
make the caller's continuity and restart-anchor obligation explicit, typed, and
testable. Ordinal day alone cannot distinguish a valid year rollover from
out-of-order replay.

### B-04 — High — Passing tests do not satisfy two explicit contract vectors

The contract requires a product vector with three nontrivial indicators and an
independently reconstructed product at `SC-PLANT-001.md:766-768`. The current
product test uses `photoperiod == 1.0` at
`crates/openwepp-plant-phenology/src/lib.rs:404-416`, so only two indicators are
nontrivial. The contract also requires independently reconstructed first,
20-to-21 fill, and eviction means at `SC-PLANT-001.md:769-770`; the test at
`lib.rs:451-472` checks the first and post-eviction values but does not capture
or assert the 20th and 21st admissions or independently reconstruct those
means. Thus 7/7 passing tests do not close the contract-derived test obligation.

Action: add a public-path product vector whose temperature, VPD, and
photoperiod indicators are all strictly between zero and one, and independently
calculate the expected product. Add heterogeneous-history assertions at sample
counts 20 and 21 and after eviction, with independent expected means.

### B-05 — Moderate — The backlog contradicts the package's integration boundary

`docs/backlog/20260626-frost-daylength-canopy-decline-hemisphere-robust.md:114-117`
says the fixed-date litter window will be replaced when Increment 2's physical
phenology lands, while lines 92-110 correctly say Increment 2 stops at the GSI
kernel and Increment 3 owns litter/consumer integration. The unresolved
leaf-on-driver and one-index-versus-two-law questions at lines 465-474 also
remain written as undecided even though lines 94-100 record the selected unified
GSI law.

Action: assign fixed-date replacement to Increment 3 and mark the process-law
selection questions resolved, leaving only canopy-mapping/calibration questions
open.

### B-06 — Blocking evidence gap — Terminal, CRAP, and line-count closure are absent

The package requires full workspace gates, `cargo deny`, fresh adjudicated CRAP,
and recorded source line counts at `package.md:152-164`; progress correctly
leaves this work unchecked at lines 189-190. The evidence map expects
`focused-gates.md` and `heavy-gates.md` at `artifacts/README.md:9-10`, but neither
exists. Reviewer B's focused reruns do not substitute for terminal full-profile
Nextest, workspace Clippy, deny, or the canonical CRAP workflow. The 583-line
count is acceptable, but CRAP `<=30` and an empty actionable workspace set are
unproven.

Action: after B-01 through B-05 are dispositioned, run and retain all required
terminal gates and the frozen-base adjudicated CRAP gate on the amended terminal
source. Do not promote beyond `HOLD` until those artifacts exist and pass.

## Review conclusion

The package has a credible, well-contained process-kernel implementation and a
correct non-integration posture. It remains `HOLD` because the canonical
contract, public state boundary, contract-derived tests, planning record, and
terminal evidence do not yet support the requested `PASS-PROCESS-KERNEL`
disposition.
