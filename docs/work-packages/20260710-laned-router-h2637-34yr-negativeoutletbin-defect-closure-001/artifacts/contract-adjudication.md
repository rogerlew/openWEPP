# Contract Adjudication

Status: `EXECUTED-CONTRACT-FIRST`

Evidence mode: `Static + Ran`

## Authority Decision

`SC-OFEROUTE-001` rev 51 is the proximate correction authority. Existing
`INV-OFEROUTE-005` already prohibited negative published discharge and
`INV-OFEROUTE-006` already required the ledger to book the scheme's actual
boundary flux. The H2637 attribution exposed a missing implementation detail:
the rev-41 face limiter bounded excessive positive outflow but did not enforce
the downstream boundary's exact physical lower bound.

Rev 51 therefore confirms and makes implementation-authoritative that:

- every scheme-actual stage boundary flux is finite and nonnegative;
- the predictor donor outlet face is
  `max(0, 2 q[n-1] - q[n-2])` before the rev-41 available-water upper cap;
- the same bounded face drives the state update and mass ledger;
- the zero lower bound has units `m²/s`, no tolerance, and no configurable
  threshold;
- it is a one-way boundary-domain constraint, not a post-update depth clamp;
- recorder redistribution and `NegativeOutletBin` remain defensive guards, but
  valid production execution may not rely on future-bin borrowing to repair a
  negative scheme-actual face.

Authority anchors are `REF-OFEROUTE-KWE`,
`REF-OFEROUTE-TVD-MACCORMACK`, and `REF-OFEROUTE-PHYS-BOUNDS`. The amendment
changes no friction law, celerity, CFL target, TVD dissipation, source booking,
mesh policy, closure tolerance, snow physics, or hybrid posture.

## Required Contract Surfaces Updated

- frontmatter version and review date;
- Algorithm items 5 and 6;
- KWE/TVD branch-and-guard row;
- `INV-OFEROUTE-005` and `INV-OFEROUTE-006`;
- invariant guard map;
- tolerance/numeric notes;
- single-OFE test-vector obligation;
- KWE/TVD Binding Exposure Index row;
- revision history;
- lifecycle registry review date.

## Kernel-Profile Check

Rev 51 keeps the required contract schema intact. The touched dimensional
surface is an existing `q`/face-flux surface in `m²/s`; no new unit alias,
conversion, constant, publication field, or scalar exception is introduced.
The typed `NegativeOutletBin` failure remains mapped and live. The production
edit must follow only after a pre-fix-failing contract-derived regression and
the pre-implementation gate are recorded.
