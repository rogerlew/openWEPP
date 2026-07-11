# Authority Assessment

Status: `PASS`

Evidence mode: `Static`

W11C is historical before-correction evidence, not current authority. W11D's
completed contract amendments bind this rerun:

- `SC-ROUTE-001` v56 / INV-ROUTE-021 requires exactly `ntchr` routed terminals,
  branch-specific terminal hydraulic storage, retained dry carry, and daily
  `chvol = volint + sinit - sfnl` without material negative storage or generated
  outlet volume.
- `SC-ROUTE-001` v56 / INV-ROUTE-022 requires finite convex MC coefficients and
  the passive bound. Out-of-envelope active configurations fail typed
  `WKERNEL-WS10-CHANNEL-E-003`; admitted static/dynamic configurations must
  execute separately.
- `SC-SYSTEM-001` v90 / INV-SYSTEM-036 requires topology-terminal event-scalar
  publication and direct authoritative sediment mass, with impoundments as
  explicit ancestry boundaries.
- `SC-INFILE-CHANINP-001` v0.1.4 accepts canonical three-record `nchnum=0`,
  retains its requested timestep, and keeps `ichnum=[]` without default aliasing.

No authority amendment is needed or authorized. A mismatch is a current
regression finding; comparator agreement with W11C is not a target.
