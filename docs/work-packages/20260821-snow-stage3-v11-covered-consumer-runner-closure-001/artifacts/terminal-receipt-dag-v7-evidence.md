# Terminal receipt DAG v7 executable evidence

Ran: package-local Rust construction through `openwepp_coupled_time::framed_sha256`.

- pass: `true`
- acyclic/no forward references: `true`
- deterministic baseline replay: `true`
- node-local poison cases: `9`
- every poison preserves ancestors/unrelated nodes and changes exactly its node plus descendants: `true`

Order: `BatchRequestCore -> ArmInputCore -> ArmPhysicalResultCore -> EndingJointReceipt -> ArmCustodyReceipt -> LaneEvidenceCore -> BatchResultCore -> EventReceipt -> ParentReceipt`.
