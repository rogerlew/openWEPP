# Verification Agent B

Status: `VERIFIED`

Static + ran evidence alignment:

- `SC-OFEROUTE-001` rev 47 authorizes analytic celerity, bounded Newton local
  hydraulics, `h * sqrt(h)`, branch-gap behavior, dust residual floor, and the
  no-approximation posture for Hirsch `Re^0.45`.
- `contract-test-implementation.md` lists tests for each landed rev-47 surface,
  including review-added failure and branch-gap cases.
- `consumer-path-proof.md` names the active H2637 runner/executor route through
  `KinematicWaveSolver`; the ignored H2637 active-owner test proves the real
  active consumer path, not a shadow-only producer path.
- Scoped contract checks are acceptable for this hold scope:
  `.venv/bin/python tools/check_sc_binding_exposure.py ...` returns
  `PASS-DEFERRED` because `7` BEI rows remain science-review follow-ons outside
  this package's Tier 1 numerics envelope, and
  `bash tools/release/check_sc_unit_compliance.sh --path ...` returns PASS.
- Pre-change vs rev-47 H2637 output deltas are measured from a detached
  `46532c28` baseline binary, so the non-bit-identity claim has concrete
  evidence.
- Final verification B found the markdown-doc lint gate missing from
  `gate-results.md`; the scoped lint command is now recorded there as PASS.

Independent read-only verifier rechecked the artifact fix and returned VERIFIED:
`gate-results.md` records the scoped markdown-doc lint row with `32` files,
`0` errors, and `0` warnings, and this artifact now explains the scoped
`PASS-DEFERRED` BEI status.

Verification result: artifact evidence matches the final hold disposition. The
only remaining gap is the intentionally unlanded `Re^0.45` approximation
envelope.
