# WB14 configuration identity map

Evidence mode: Static

| Field | Source | Binding |
| --- | --- | --- |
| OFE and topology | `DirectSurfaceLiquidConfiguration.ofe_topology` | ordered parent identity and every child |
| production lane | `DirectSurfaceLiquidOfeBinding.production_lane_id` | exact OFE-to-lane join |
| surface configuration | `DirectSurfaceLiquidConfiguration.configuration_sha256` | immutable parent identity |
| conductivity | `DirectOfeWb14Parameters.effective_conductivity_m_s` | exact binary64 bits |
| matric potential | `DirectOfeWb14Parameters.matric_potential_m` | exact binary64 bits |
| storage capacity | `DirectOfeWb14Parameters.infiltration_storage_capacity_m` | exact binary64 bits |
| WB14 configuration | ordered identity plus parameter bits | framed SHA-256 |
| WB14 model definition | explicit schema constant/digest | framed SHA-256, never Rust layout/debug text |
