# Terminal Hydrology Review At `dd8127b04`

Evidence class: `Static exact-commit + Ran exact-commit focused tests`

Disposition: `HOLD / NO-GO`

The reviewer accepted the persistent owner, ingress, WB14 continuation,
receiver topology, independent closure, rollback and production exclusion,
then accepted two findings:

1. independently rounded proportional rows could jointly exceed immutable
   supply by one ULP and make the canonical `F=A` endpoint unusable; and
2. the direct public owner APIs still misclassified finite negative versus
   nonfinite request, finalized-use and condensation operands.

The missing exact-head heavy regression remained an evidence blocker. No
finding was deferred or rejected.
