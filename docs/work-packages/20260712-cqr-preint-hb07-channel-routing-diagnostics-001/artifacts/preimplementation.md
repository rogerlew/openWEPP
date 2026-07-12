# HB-07 Preimplementation Record

Evidence class: **Static**

## Classification

The two fixed rows are coherent `E-SCIENCE` WS11 Muskingum-Cunge diagnostics,
not infrastructure or observability exceptions. Start metrics are taken from
the committed High-B raw-to-actionable ledger:

- geometry: CC 15, 52.308% coverage, CRAP 39.408;
- variable MC state: CC 32, 70.455% coverage, CRAP 58.410.

Both miss the 75% function floor. The first action is therefore cover-first
characterization, not immediate decomposition. The 620-line target is below
the campaign WARN threshold.

## Source And Call Path

`compute_direct_channel_variable_muskingum_peak` in `direct.rs` calls the
target variable-state function. It derives dynamic reference flow from current
and prior `qin/q1`, solves Manning depth, dispatches geometry by `ishape`,
computes `ckref`, `tk`, `cx`, and refreshed MC coefficients, then returns the
WS11 wave-routing state consumed by direct channel publication. The geometry
helper is also consumed by Manning depth solving and the production hourly
wave/storage lane in `hourly.rs`.

The downstream proof is the existing W11C runner integration, which executes
static and variable MC channel paths through watershed routing and consumes
published water, storage, peak, and sediment outputs. Private helper tests are
necessary for branch/failure coverage but cannot replace this consumer.

## Risks To Pin Before Refactor

- geometry formulas and `ishape` dispatch, including naturally eroded `chnz0`;
- depth-bracket and fixed-iteration order;
- exact `qref` averaging and epsilon policy;
- prior-state initialization versus carried-memory behavior;
- signed coefficient provenance versus production admissibility rejection;
- coefficient/lateral-term evaluation order and recurrence closure;
- exact symbol and WS10 typed-error priority;
- absence of coefficient clamps, peak clips, damping, or static fallback.

## Planned Evidence

Fresh focused coverage must enumerate every function in `diagnostics.rs`, not
only the two ledger rows. Tests should distinguish all geometry/`ckref` regimes,
finite/domain failures, carried versus fresh state, admissible versus rejected
grids, and coefficient/storage conservation. Final evidence must include the
same-source floor audit, CRAP JSON, focused test counts, W11C executable result,
source/artifact hashes, two review dispositions, and two verification records.

No production or test edit has been made by this kickoff.
