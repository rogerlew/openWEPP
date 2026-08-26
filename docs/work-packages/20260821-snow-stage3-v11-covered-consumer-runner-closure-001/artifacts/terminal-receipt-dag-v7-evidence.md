# Terminal receipt DAG v7 executable evidence

Ran: package-local Rust construction through `openwepp_coupled_time::framed_sha256`.

- pass: `true`
- acyclic/no successor references: `true`
- deterministic replay: `true`
- root poison propagates through all nodes: `true`

Order: `BatchRequestCore -> ArmInputCore -> ArmPhysicalResultCore -> EndingJointReceipt -> ArmCustodyReceipt -> LaneEvidenceCore -> BatchResultCore -> EventReceipt -> ParentReceipt`.
