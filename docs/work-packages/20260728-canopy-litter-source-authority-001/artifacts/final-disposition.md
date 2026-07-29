# Final Disposition

Status: `COMPLETE — EXTERNAL BOUNDARY IMPLEMENTED / PREDICTIVE AUTHORITY HOLD RETAINED`

Evidence class: `Static + Ran`

## Implemented result

The package implements an authenticated identity-only external daily
ground-deposition boundary for independent needle and fine-woody tissue.
Prescribed scenarios may omit supported dates as explicit scenario zeros.
Measured-daily inputs must record every supported date, including measured
zeros.

The schema authenticates functional classification, source, and executable
objects by SHA-256; binds material, dry-mass, horizontal area, support,
site/OFE, diameter, and bark semantics; and fails closed on unsupported
derived/interval objects. `not_represented` and `not_applicable` publish null
source operands rather than numeric zero.

The runtime publishes and independently reconstructs:

```text
Q = L_leaf + N_ext + W_ext
S_next = (S_before + Q) * f
I_next = (I_before + Q) * f
R_next = (R_before + Q) * f
```

plus weighted ground mass, interrill/rill/composite cover, residue depth,
exact active erosion cover operands, and exact frost depth. Static source
guards prove one decomposition handoff and no downstream re-addition.

## Predictive authority remains held

No predictive needle or fine-woody production law was added. Those rows remain:

```text
AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED
```

The implemented exogenous interface is:

```text
IMPLEMENTED / NOT_APPLICABLE / NOT_APPLICABLE
```

It does not validate natural source sufficiency or predictive biology.

## Terminal evidence

- contract-derived boundary suite: 16/16 passed;
- real consumer plus source guard: 2/2 passed;
- warnings-denied workspace Clippy: passed;
- type-size layout guard: passed;
- exact-head full workspace profile: 2,117 passed, 29 profile-declared skips,
  757.402 seconds;
- no touched nonexempt Rust file remains at 3,000 lines or longer.

The initial terminal reviews and verifications failed closed; every finding
was corrected. Both independent terminal re-reviews and both independent
terminal re-verifications pass on the corrected tree. The exact scoped diff,
documentation, and whitespace checks reconcile.

## Acquisition need

`ARTICLE_REQUIRED = 0` for this boundary increment. Lifting predictive holds
still requires new species/site needle retention/deposition evidence and
branch/crown/stand plus attached-dead/in-canopy/ground-deposition dynamics.
