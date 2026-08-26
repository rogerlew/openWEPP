# Terminal receipt DAG v7 executable evidence

Ran: package-local canonical hash-construction prototype.

- pass: `True`
- acyclic/no successor references: `True`
- deterministic replay: `True`
- root poison propagates through all nodes: `True`

Order: `BatchRequestCore -> ArmInputCore -> ArmPhysicalResultCore -> EndingJointReceipt -> ArmCustodyReceipt -> LaneEvidenceCore -> BatchResultCore -> EventReceipt -> ParentReceipt`.
