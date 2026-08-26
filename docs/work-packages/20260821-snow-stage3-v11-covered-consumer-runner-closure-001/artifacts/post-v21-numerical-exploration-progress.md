# Post-v21 numerical exploration progress

Status: `EXECUTED / HERMITE ANALYTICAL PASS / REAL CANDIDATE INCOMPLETE`.

Base: `57241a838fb580f25a481ccbd3918f3875872112`.
Last qualified physical implementation remains
`43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.

## Attempt ledger

1. `v21 linear interpolation + static J e=-d`: rejected. Its affine defect is
   identically zero while CN endpoint error is nonzero; no gamma can repair it.
2. `Hermite extension + dynamic Gauss error transport`: executed. The first
   run incorrectly applied direct-magnitude enclosure to the retained
   above-selector Richardson arm and counted expected rootless CN cases as
   estimator failures. Those rows remain recorded, but acceptance was
   corrected to the smooth floor domain named by the candidate.
3. Corrected Hermite run: analytical floor result PASS, 48 evaluated smooth
   floor rows, zero sign/enclosure failures, all constitutive supports at least
   600 ms, gamma neither used nor frozen. Expected rootless nonlinear trials
   retain typed unsupported disposition.

The matrix includes affine, stiff affine, index-1 DAE with independently zero
algebraic residual, conservative two-node generated-transfer pairs, nonlinear
manufactured, computed active-set/event, prescribed cumulative/exact-increment
forcing paths, and the retained real-carrier receipt identity. The real row is
explicitly incomplete because a candidate complete-SCC CN/collocation/AD solve
has not executed.

## Receipt DAG

Ran: canonical framed hashes are constructed in the exact order:

`BatchRequestCore -> ArmInputCore -> ArmPhysicalResultCore -> EndingJointReceipt
-> ArmCustodyReceipt -> LaneEvidenceCore -> BatchResultCore -> EventReceipt ->
ParentReceipt`.

No node references a successor. Deterministic replay and root-poison
propagation through every successor pass.

## SCC inventory

Ran: 12 records each carry units, exact symbolic cardinality, canonical index
order, storage map, residual, active tag, forcing class, vector order,
tolerance, generated-transfer counterpart and follower set. Tarjan evaluation
finds one 11-component implicit physical SCC and one exact BGC follower SCC.

## Disposition

No successor contracts are drafted and no final reviews are requested because
the matrix has not passed its real-carrier obligation. Current production
receipts are not candidate-CN evidence. Production physics, temporal
acceptance, tolerances, floor, controller, root/event behavior, public API,
owners, publication, restart, receiver, runner, selector, default, CoE and
cutover remain unchanged.

Next in-envelope implementation seam: a `cfg(test)` complete-SCC candidate
solver must expose the real carrier's endpoint/collocation residual and exact
AD Jacobian from immutable real-fixture beginnings, produce candidate CN
receipts at 1.875 seconds, and feed those typed results into this matrix. A
snow-only or receipt-relabeling approximation is not acceptable.
