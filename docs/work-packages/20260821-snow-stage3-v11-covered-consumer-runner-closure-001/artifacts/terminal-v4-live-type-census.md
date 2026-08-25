# Terminal V4 exact live-type census

Generator: `terminal_v4_census_tool` (syn AST). Generated from exact checked-out source; do not edit.

## `openwepp_coupled_time::identity::Digest32`

- source: `crates/openwepp-coupled-time/src/identity.rs`
- git blob SHA: `72b9e79fdda65257c599b86ba330771c9d7c037f`
- normalized declaration SHA-256: `e672179b4c6a5f6d09660ff9a55b5e3b8c70a0b342c8b051b0e7e56f21852224`
- visibility: `pub`
- owner stage: `identity/shared`
- exact fields/variants: `0: [u8 ; 32]`
- nested collection/key types: `[u8 ; 32]`
- native validator/digest candidates: `pub const fn digest(self) -> Digest32 { | pub fn digest_bytes(bytes: &[u8]) -> Digest32 {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_coupled_time::identity::FramedField`

- source: `crates/openwepp-coupled-time/src/identity.rs`
- git blob SHA: `72b9e79fdda65257c599b86ba330771c9d7c037f`
- normalized declaration SHA-256: `2a4d09fb7282880aa46fbd9236418f624771c640956b6740817454a9ce20faf2`
- visibility: `pub`
- owner stage: `identity/shared`
- exact fields/variants: `tag: & 'a str; value: & 'a [u8]`
- nested collection/key types: `& 'a [u8]`
- native validator/digest candidates: `pub const fn digest(self) -> Digest32 {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_coupled_time::support::ModelTimeNs`

- source: `crates/openwepp-coupled-time/src/support.rs`
- git blob SHA: `7fb036835fa2cee94d611fe714fde255d70900f4`
- normalized declaration SHA-256: `2ebba356a98184672423f314ddea01a5f9a0e20208ae9468cc3c341f165ca3b6`
- visibility: `pub`
- owner stage: `identity/shared`
- exact fields/variants: `0: u128`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_coupled_time::support::TimeSupport`

- source: `crates/openwepp-coupled-time/src/support.rs`
- git blob SHA: `7fb036835fa2cee94d611fe714fde255d70900f4`
- normalized declaration SHA-256: `2463f51a99a44e17099c09c0ec0342ae9b4d502d2e14bcc6a194ce5a9a96b15d`
- visibility: `pub`
- owner stage: `identity/shared`
- exact fields/variants: `start_ns: ModelTimeNs; end_ns: ModelTimeNs`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::hydrology::02_guard_errors::DirectSnowStage3EvaluationError`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
- git blob SHA: `0a1b23a4201b288efb43aef4ba895222afe5891e`
- normalized declaration SHA-256: `122d2c709d22786996c990b77ccb17ce60608312d2bc68118d13c7b92c1fb01c`
- visibility: `pub`
- owner stage: `typed-error`
- exact fields/variants: `Kernel(Box < Wb11HydrologyKernelGuardError >); TurbulentTransfer(Box < SnowStage3TurbulentTransferError >); TerminalNumerics(SnowTerminalNumericsFailure); TerminalCustody(& 'static str)`
- nested collection/key types: `Box < SnowStage3TurbulentTransferError > | Box < Wb11HydrologyKernelGuardError >`
- native validator/digest candidates: `none discovered`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::hydrology::02_guard_errors::SnowTerminalNumericsFailure`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/02_guard_errors.rs`
- git blob SHA: `0a1b23a4201b288efb43aef4ba895222afe5891e`
- normalized declaration SHA-256: `669b011451f22d446998e12053a3b76b32b46363812d1ab02e7f89c0d7aa6658`
- visibility: `pub`
- owner stage: `typed-error`
- exact fields/variants: `DomainOrNonFinite(); StepUnderflow(); BelowCarrierDomain(); RejectionLimit(); InvalidEventBracket(); EventIterationLimit(); Closure()`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::CoveredProbeChildIdentityV1`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- git blob SHA: `2d2c29ed599d24fe13683fce98c0fc068fee2df9`
- normalized declaration SHA-256: `e858d368601491b554234bf83e1ebfa8af1a85713d60794b47b5fb3213d6198e`
- visibility: `pub (crate)`
- owner stage: `identity/shared`
- exact fields/variants: `parent_transaction_sha256: Digest32; enclosing_parent_support: TimeSupport; trial_support: TimeSupport; physical_child_ordinal: u32; role: CoveredTerminalTrialRoleV1; attempt_ordinal: u32; beginning_joint_sha256: Digest32; beginning_owner_set_sha256: Digest32; complete_forcing_sha256: Digest32; topology_sha256: Digest32; receipt_sha256: Digest32`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredProbeChildIdentityV1`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::CoveredTerminalEndingSnowHintV1`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- git blob SHA: `2d2c29ed599d24fe13683fce98c0fc068fee2df9`
- normalized declaration SHA-256: `eaf47f820cae32bb73db3ac5ccf4413893ef92ee6a131aa10f7a7b1e95d9a7c6`
- visibility: `pub (crate)`
- owner stage: `coupling-evaluation`
- exact fields/variants: `ice_kg_m2: f64; liquid_kg_m2: f64; cold_content_j_m2: f64; surface_temperature_c: f64`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredTerminalEndingSnowHintV1`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::CoveredTerminalJointTrialStateV1`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- git blob SHA: `2d2c29ed599d24fe13683fce98c0fc068fee2df9`
- normalized declaration SHA-256: `82ee00c49277321fd88e52dd431d965fe392c0915344919996ac50e680ea668a`
- visibility: `pub (crate)`
- owner stage: `identity/shared`
- exact fields/variants: `authority: JointTrialAuthorityV1; owner_bytes: BTreeMap < String , Vec < u8 > >; receipt_sha256: Digest32`
- nested collection/key types: `BTreeMap < String , Vec < u8 > >`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredTerminalJointTrialStateV1`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::CoveredTerminalTrialRequestV1`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- git blob SHA: `2d2c29ed599d24fe13683fce98c0fc068fee2df9`
- normalized declaration SHA-256: `4fc9a2aa31358b8d6b61c3e719ec46f31e02cc5e644a5ce3e8c3afae85de1fcd`
- visibility: `pub (crate)`
- owner stage: `identity/shared`
- exact fields/variants: `lane_id: u32; support: TimeSupport; role: CoveredTerminalTrialRoleV1; attempt_ordinal: u32; coupling_iteration: u32; ice_kg_m2: f64; liquid_kg_m2: f64; cold_content_j_m2: f64; surface_temperature_c: f64; snow_depth_m: f64; snow_density_kg_m3: f64; ending_snow_hint: Option < CoveredTerminalEndingSnowHintV1 >; beginning_joint: CoveredTerminalJointTrialStateV1`
- nested collection/key types: `Option < CoveredTerminalEndingSnowHintV1 >`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredTerminalTrialRequestV1`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::CoveredTerminalTrialRoleV1`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- git blob SHA: `2d2c29ed599d24fe13683fce98c0fc068fee2df9`
- normalized declaration SHA-256: `2f7f5db01698dc293d9b99c32d969478ab1db894edd3182f79fdcee6267c7649`
- visibility: `pub (crate)`
- owner stage: `identity/shared`
- exact fields/variants: `Full(); Half1(); Half2(); Retry(); BracketLower(); BracketUpper(); Root()`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredTerminalTrialRoleV1`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::CoveredTerminalTrialTransitionV1`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- git blob SHA: `2d2c29ed599d24fe13683fce98c0fc068fee2df9`
- normalized declaration SHA-256: `f6e1c5194efe3ea1c0d9c85125a71b53ed7c5f56498443caa9741a7b503878b7`
- visibility: `pub (crate)`
- owner stage: `provider-output-before-hydrology-join`
- exact fields/variants: `boundary: Stage3SnowSurfaceBoundaryReceiptV1; beginning_joint: CoveredTerminalJointTrialStateV1; ending_joint: CoveredTerminalJointTrialStateV1; probe_child_identity: CoveredProbeChildIdentityV1; trial_snow_soil_receipt: Option < crate :: v9_real_consumer_shadow :: TerminalSnowSoilTrialReceiptV1 >`
- nested collection/key types: `Option < crate :: v9_real_consumer_shadow :: TerminalSnowSoilTrialReceiptV1 >`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredTerminalTrialTransitionV1`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::JointTrialAuthorityV1`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- git blob SHA: `2d2c29ed599d24fe13683fce98c0fc068fee2df9`
- normalized declaration SHA-256: `b206caa89d34d6efabbcc9cc274e793aa425b921324d9ef105d9ab42365e8fa5`
- visibility: `pub (crate)`
- owner stage: `identity/shared`
- exact fields/variants: `source_owner_set_sha256: Digest32; lane_id: u32; source_snow_owner_sha256: Digest32; interval_index: u64; state_support: TimeSupport; accepted_predecessors: Vec < Digest32 >`
- nested collection/key types: `Vec < Digest32 >`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for JointTrialAuthorityV1`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::ProbeChildAuthorityV1`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- git blob SHA: `2d2c29ed599d24fe13683fce98c0fc068fee2df9`
- normalized declaration SHA-256: `6a67816511876be9d31cec90c6c6d9513df012410ca937bbe444bac491ec9782`
- visibility: `pub (crate)`
- owner stage: `identity/shared`
- exact fields/variants: `parent_transaction_sha256: Digest32; enclosing_parent_support: TimeSupport; trial_support: TimeSupport; physical_child_ordinal: u32; attempt_ordinal: u32; role: CoveredTerminalTrialRoleV1; beginning_joint_sha256: Digest32; beginning_owner_set_sha256: Digest32; complete_forcing_sha256: Digest32; topology_sha256: Digest32`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for ProbeChildAuthorityV1`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::stage3_solver::terminal_event::TerminalFluxIntegral`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`
- git blob SHA: `711d8ee2b67e41eb3ed6a0de4e3dea5285fe8938`
- normalized declaration SHA-256: `e785336347399eec4ff86000452cb3992425e7f2c7ebaa08b9498d6c6bd2306a`
- visibility: `pub (super)`
- owner stage: `terminal-solver`
- exact fields/variants: `complete_energy_j_m2: f64; vapor_mass_exchange_kg_m2: f64; shortwave_energy_j_m2: f64; longwave_energy_j_m2: f64; sensible_energy_j_m2: f64; latent_energy_j_m2: f64; advected_energy_j_m2: f64; snow_soil_heat_energy_j_m2: f64; external_liquid_kg_m2: f64`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for TerminalFluxIntegral`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::stage3_solver::terminal_event::TerminalLedger`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`
- git blob SHA: `711d8ee2b67e41eb3ed6a0de4e3dea5285fe8938`
- normalized declaration SHA-256: `ecb407d9f0a093773b1b3173b453bd467f05d551afeddae5fe5c55fa9ddab04e`
- visibility: ``
- owner stage: `terminal-solver`
- exact fields/variants: `complete_energy_j_m2: f64; cold_energy_change_j_m2: f64; refrozen_kg_m2: f64; deposition_kg_m2: f64; sublimation_kg_m2: f64; melt_kg_m2: f64; unallocated_energy_j_m2: f64; shortwave_energy_j_m2: f64; longwave_energy_j_m2: f64; sensible_energy_j_m2: f64; latent_energy_j_m2: f64; advected_energy_j_m2: f64; snow_soil_heat_energy_j_m2: f64; external_liquid_kg_m2: f64`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for TerminalLedger`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::stage3_solver::terminal_event::TerminalState`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`
- git blob SHA: `711d8ee2b67e41eb3ed6a0de4e3dea5285fe8938`
- normalized declaration SHA-256: `20924b6f711c9b7945d3626094c098946305a074e3c73313b1b5d65fdadeaaa4`
- visibility: `pub (super)`
- owner stage: `terminal-solver`
- exact fields/variants: `ice_kg_m2: f64; liquid_kg_m2: f64; cold_content_j_m2: f64`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for TerminalState`

## `openwepp_hillslope_orchestrator::hydrology::support_helpers_mod::runoff_reconciliation::stage3_solver::terminal_event::TerminalTrial`

- source: `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_event.rs`
- git blob SHA: `711d8ee2b67e41eb3ed6a0de4e3dea5285fe8938`
- normalized declaration SHA-256: `609196722808e8ff57c7db03d30d8519fb45bab14329ce51a8413cfc9f1cdfad`
- visibility: `pub (super)`
- owner stage: `terminal-solver`
- exact fields/variants: `state: TerminalState; ledger: TerminalLedger`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for TerminalTrial`

## `openwepp_hillslope_orchestrator::land_surface_energy_shadow::covered_v8_owner::CoveredCarrierComponentState`

- source: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs`
- git blob SHA: `bc048c571840075d2d64bc6b8865131e972ec135`
- normalized declaration SHA-256: `1f215aa74b3c3a049b68545b05414bddd3bf2efb8277e3f7e21fde24edbcfc72`
- visibility: `pub (crate)`
- owner stage: `provider-carrier`
- exact fields/variants: `vertical_occupancy_ordinal: u32; occupancy_id: String; component_ordinal: u8; surface_area_m2_m2_tile: f64; emissive_area_m2_m2_tile: f64; heat_conductance_m_s_tile: f64; vapor_conductance_m_s_tile: f64; vapor_authorization_kg_m2_tile_s: Option < f64 >; temperature_k: f64; specific_humidity_kg_kg: f64; sensible_to_canopy_air_w_m2: f64; vapor_to_canopy_air_kg_m2_s: f64`
- nested collection/key types: `Option < f64 >`
- native validator/digest candidates: `pub fn validate(&self) -> Result<(), CoveredV8OwnerEnvelopeError> {`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredCarrierComponentState`

## `openwepp_hillslope_orchestrator::land_surface_energy_shadow::covered_v8_owner::CoveredLseIterationState`

- source: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs`
- git blob SHA: `bc048c571840075d2d64bc6b8865131e972ec135`
- normalized declaration SHA-256: `dc77c8c00f7aabcfc22c2da0dc3cae366d99bc5a156c80b0591e07f585502bed`
- visibility: `pub (crate)`
- owner stage: `identity/shared`
- exact fields/variants: `canopy_air_temperature_k: f64; canopy_air_specific_humidity_kg_kg: f64; snow_temperature_k: f64; snow_sensible_w_m2: f64; snow_vapor_kg_m2_s: f64; snow_latent_w_m2: f64; snow_net_longwave_w_m2: f64; component_temperatures_k: Vec < (String , [f64 ; 4]) >; component_carrier_surfaces: Vec < CoveredCarrierComponentState >; canopy_sensible_w_m2: f64; canopy_vapor_kg_m2_s: f64; sensible_to_reference_air_w_m2: f64; vapor_to_reference_air_kg_m2_s: f64`
- nested collection/key types: `Vec < (String , [f64 ; 4]) > | Vec < CoveredCarrierComponentState >`
- native validator/digest candidates: `pub fn validate(&self) -> Result<(), CoveredV8OwnerEnvelopeError> {`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredLseIterationState`

## `openwepp_hillslope_orchestrator::land_surface_energy_shadow::covered_v8_owner::CoveredV8PhysicalOwner`

- source: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs`
- git blob SHA: `bc048c571840075d2d64bc6b8865131e972ec135`
- normalized declaration SHA-256: `d9a63525086095951f2b56dc3a4f1d1633113b2feb8b3e4938da693e6243cb8c`
- visibility: ``
- owner stage: `identity/shared`
- exact fields/variants: `Legacy(CoveredForestShadowResult); MultiTile(MultiTileRuntimeResult)`
- nested collection/key types: `none`
- native validator/digest candidates: `pub fn validate(&self) -> Result<(), CoveredV8OwnerEnvelopeError> {`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredV8PhysicalOwner`

## `openwepp_hillslope_orchestrator::land_surface_energy_shadow::covered_v8_owner::UncommittedCoveredV8OwnerEnvelope`

- source: `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/covered_v8_owner.rs`
- git blob SHA: `bc048c571840075d2d64bc6b8865131e972ec135`
- normalized declaration SHA-256: `d23b203cc71904dd706cf524278b17c8932fca12ed5d98162aea7baeca75e7f1`
- visibility: `pub`
- owner stage: `identity/shared`
- exact fields/variants: `transaction_id: TransactionId; vegetation: UncommittedV8VegetationCandidate; physical: CoveredV8PhysicalOwner; biogeochemistry: BiogeochemistryOwnerCandidate`
- nested collection/key types: `none`
- native validator/digest candidates: `pub fn validate(&self) -> Result<(), CoveredV8OwnerEnvelopeError> {`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::snow_stage3_v11_precipitation::Stage3PrecipitationDestinationV1`

- source: `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_precipitation.rs`
- git blob SHA: `da346fd0058062d396ef46b4442f0ba42742a01d`
- normalized declaration SHA-256: `f4e57500591a873fdbf3aae3435a680d75d18dd4df58e1c5ea529aa399cc08f2`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `topology_index: u32; ofe_id: OfeId; tile_id: TileId; fraction_of_ofe: f64; canopy_covered: bool; destination_identity_sha256: Digest32`
- nested collection/key types: `none`
- native validator/digest candidates: `pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> { | pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::snow_stage3_v11_precipitation::Stage3PrecipitationEnthalpyProviderV1`

- source: `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_precipitation.rs`
- git blob SHA: `da346fd0058062d396ef46b4442f0ba42742a01d`
- normalized declaration SHA-256: `00bf7265602465ef1833db2850f21f93058712bb11cb08b7291290f524e31ef5`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `Temperature(temperature_k: f64, reference_temperature_k: f64, specific_heat_j_kg_k: f64, provider_receipt_sha256: Digest32); SpecificEnthalpy(specific_enthalpy_j_kg: f64, provider_receipt_sha256: Digest32)`
- nested collection/key types: `none`
- native validator/digest candidates: `pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> { | pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::snow_stage3_v11_precipitation::Stage3PrecipitationPhaseParcelSetV1`

- source: `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_precipitation.rs`
- git blob SHA: `da346fd0058062d396ef46b4442f0ba42742a01d`
- normalized declaration SHA-256: `051da68a90f177fd8cbb4208b83c53aa77231edbec385a8371e8b49624029618`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `schema_version: u16; support: TimeSupport; lane_id: u32; ofe_id: OfeId; ofe_ground_basis: bool; beginning_snow_state_sha256: Digest32; topology_identity_sha256: Digest32; destinations: Vec < Stage3PrecipitationDestinationV1 >; parcels: Vec < Stage3PrecipitationPhaseParcelV1 >; receipt_sha256: Digest32`
- nested collection/key types: `Vec < Stage3PrecipitationDestinationV1 > | Vec < Stage3PrecipitationPhaseParcelV1 >`
- native validator/digest candidates: `pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> { | pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::snow_stage3_v11_precipitation::Stage3PrecipitationPhaseParcelV1`

- source: `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_precipitation.rs`
- git blob SHA: `da346fd0058062d396ef46b4442f0ba42742a01d`
- normalized declaration SHA-256: `69b56d5dfd08d2ec4a2dce92f908960aca7949aac867782c960da8070ac7b0c6`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `support: TimeSupport; lane_id: u32; destination_topology_index: u32; destination_ofe_id: OfeId; destination_tile_id: TileId; phase: Stage3PrecipitationPhaseV1; source: Stage3PrecipitationSourceV1; semantic_receipt_ordinal: u32; mass_kg_m2_tile_ground: f64; enthalpy_provider: Stage3PrecipitationEnthalpyProviderV1; source_identity_sha256: Digest32; producer_beginning_state_sha256: Digest32; receipt_sha256: Digest32`
- nested collection/key types: `none`
- native validator/digest candidates: `pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> { | pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::snow_stage3_v11_precipitation::Stage3PrecipitationPhaseV1`

- source: `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_precipitation.rs`
- git blob SHA: `da346fd0058062d396ef46b4442f0ba42742a01d`
- normalized declaration SHA-256: `6f3089b9ce8d6fe985d013bbd15bc91b167a4a21250016c3a909a94a4cdf6c48`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `Solid(); Liquid()`
- nested collection/key types: `none`
- native validator/digest candidates: `pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> { | pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::snow_stage3_v11_precipitation::Stage3PrecipitationSourceV1`

- source: `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_precipitation.rs`
- git blob SHA: `da346fd0058062d396ef46b4442f0ba42742a01d`
- normalized declaration SHA-256: `b7d6a9d87d77f00a8ce3922b910d20e5bfa9837aad643040551b94502bb2d965`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `AtmosphericGroundSnow(); OpenRawRain(); VegetationTerminalThroughfall(); VegetationTerminalInitialDrainage(); VegetationTerminalSecondDrainage(); VegetationTerminalStemflow()`
- nested collection/key types: `none`
- native validator/digest candidates: `pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> { | pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_hillslope_orchestrator::v11_covered::carrier_phase::CoveredCarrierEphemeralCandidatesV1`

- source: `crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs`
- git blob SHA: `bb0c6477ec0e06b254d05088aaec66103ad9f4d8`
- normalized declaration SHA-256: `86b6321ec181db8d3a1e9b7e89e75621bdbd51e76b5145f4a2e566816a2d31d9`
- visibility: `pub (crate)`
- owner stage: `provider-carrier`
- exact fields/variants: `joint: CoveredTerminalJointTrialStateV1; shadow: DirectV10RealConsumerShadow; stage3_by_lane: BTreeMap < u32 , DirectSnowStage3PersistentState >; terminal_snow_soil_trial_receipt: Option < physical_outcome_ledger :: TerminalSnowSoilTrialReceiptV1 >`
- nested collection/key types: `BTreeMap < u32 , DirectSnowStage3PersistentState > | Option < physical_outcome_ledger :: TerminalSnowSoilTrialReceiptV1 >`
- native validator/digest candidates: `none discovered`
- replay class: `1-native-replay-bytes`
- required test-only access: `none; embed exact native bytes`

## `openwepp_hillslope_orchestrator::v11_covered::carrier_phase::CoveredCarrierPhaseResultV1`

- source: `crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs`
- git blob SHA: `bb0c6477ec0e06b254d05088aaec66103ad9f4d8`
- normalized declaration SHA-256: `0a8bfe655bdf4a9633623e6f3f0dc3eaa89467cf51955b86f732f68c01fd093a`
- visibility: `pub (crate)`
- owner stage: `provider-carrier`
- exact fields/variants: `transition: CoveredTerminalTrialTransitionV1; ending_candidates: CoveredCarrierEphemeralCandidatesV1; precipitation_sets: BTreeMap < u32 , Stage3PrecipitationPhaseParcelSetV1 >; carrier_envelope: UncommittedCoveredV8OwnerEnvelope; complete_lower_boundaries: BTreeMap < (OfeId , TileId) , Stage3SnowCoveredLowerBoundary >; carrier_source_receipts: BTreeMap < (OfeId , TileId) , CoveredCarrierInitialGuessV1 >; covered_lse_states: BTreeMap < (OfeId , TileId) , CoveredLseIterationState >; soil_candidate: SoilThermalSnapshot; soil_top_boundary_credit: SoilThermalTopBoundaryCreditV1; wb14_child_receipt_set_sha256: String; wb14_parent_receipt_set_sha256: Option < String >; wb14_child_replay_bytes: Vec < u8 >; wb14_parent_replay_bytes: Option < Vec < u8 > >`
- nested collection/key types: `BTreeMap < (OfeId , TileId) , CoveredCarrierInitialGuessV1 > | BTreeMap < (OfeId , TileId) , CoveredLseIterationState > | BTreeMap < (OfeId , TileId) , Stage3SnowCoveredLowerBoundary > | BTreeMap < u32 , Stage3PrecipitationPhaseParcelSetV1 > | Option < String > | Option < Vec < u8 > > | Vec < u8 >`
- native validator/digest candidates: `none discovered`
- replay class: `1-native-replay-bytes`
- required test-only access: `none; embed exact native bytes`

## `openwepp_hillslope_orchestrator::v11_covered::physical_outcome_ledger::TerminalSnowSoilHeatReceiptV1`

- source: `crates/openwepp-hillslope-orchestrator/src/v11_covered/physical_outcome_ledger.rs`
- git blob SHA: `0bee1f1c11c1973a1887b68045bfb6345a575b70`
- normalized declaration SHA-256: `475e4316d4a27e924063976bcf8c55dc7b299af55b46bdfa8b34cbfce4b9c641`
- visibility: `pub (crate)`
- owner stage: `accepted-event`
- exact fields/variants: `support: TimeSupport; lane_id: u32; ofe_id: OfeId; beginning_snow_owner_sha256: Digest32; ending_dormant_snow_owner_sha256: Digest32; ending_soil_owner_sha256: Digest32; limiting_boundary_receipt_sha256: Digest32; snow_heat_j_m2: f64; soil_heat_j_m2: f64; receipt_sha256: Digest32`
- nested collection/key types: `none`
- native validator/digest candidates: `fn seal(mut self) -> Result<Self, Stage3PhysicalOutcomeLedgerError> { | pub(crate) fn validate(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> { | pub(crate) fn seal(mut self) -> Result<Self, Stage3PhysicalOutcomeLedgerError> { | pub(crate) fn validate(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> { | fn digest(&self) -> Digest32 { | fn validate_closure(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> { | fn digest(&self) -> Digest32 {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for TerminalSnowSoilHeatReceiptV1`

## `openwepp_hillslope_orchestrator::v11_covered::physical_outcome_ledger::TerminalSnowSoilTrialReceiptV1`

- source: `crates/openwepp-hillslope-orchestrator/src/v11_covered/physical_outcome_ledger.rs`
- git blob SHA: `0bee1f1c11c1973a1887b68045bfb6345a575b70`
- normalized declaration SHA-256: `f1935d32598903e60082d7ae3a58f758c67d54202c2f975ed8ce777708417dc0`
- visibility: `pub (crate)`
- owner stage: `identity/shared`
- exact fields/variants: `support: TimeSupport; lane_id: u32; ofe_id: OfeId; canonical_source_sha256: Digest32; beginning_snow_temperature_k: f64; ending_snow_temperature_k: f64; beginning_soil_temperature_k: f64; ending_soil_temperature_k: f64; snow_heat_j_m2: f64; soil_heat_j_m2: f64; ending_soil_candidate_sha256: Digest32; receipt_sha256: Digest32`
- nested collection/key types: `none`
- native validator/digest candidates: `fn seal(mut self) -> Result<Self, Stage3PhysicalOutcomeLedgerError> { | pub(crate) fn validate(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> { | pub(crate) fn seal(mut self) -> Result<Self, Stage3PhysicalOutcomeLedgerError> { | pub(crate) fn validate(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> { | fn digest(&self) -> Digest32 { | fn validate_closure(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> { | fn digest(&self) -> Digest32 {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for TerminalSnowSoilTrialReceiptV1`

## `openwepp_hillslope_orchestrator::v11_covered::receipt_sets::CoveredCarrierInitialGuessV1`

- source: `crates/openwepp-hillslope-orchestrator/src/v11_covered/receipt_sets.rs`
- git blob SHA: `f3a1e1b0f3b5720bd2f812a86456ecc51efa221c`
- normalized declaration SHA-256: `4a599308bd0e8f58ec00d38595fdf1737d798b3eaa94d8ab24cd18e76fb4de60`
- visibility: `pub (crate)`
- owner stage: `provider-carrier`
- exact fields/variants: `snow_temperature_k: f64; snow_sensible_into_surface_w_m2: f64; snow_vapor_into_surface_kg_m2_s: f64; snow_longwave_net_w_m2: f64; diagnostic_sha256: Digest32`
- nested collection/key types: `none`
- native validator/digest candidates: `none discovered`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for CoveredCarrierInitialGuessV1`

## `openwepp_hillslope_orchestrator::v9_real_consumer_shadow::SoilThermalTopBoundaryCreditV1`

- source: `crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow.rs`
- git blob SHA: `414012123294a02095271c274e64be2e42f219c3`
- normalized declaration SHA-256: `9fc24c55af4e6f61120695908ad7e53ff613be45da560a96568d8bbe719ab825`
- visibility: `pub (crate)`
- owner stage: `provider-carrier`
- exact fields/variants: `lane_id: u32; ofe_id: OfeId; first_layer_id: SoilLayerId; beginning_owner_id: ResourceOwnerId; beginning_configuration_sha256: Sha256Digest; beginning_state_sha256: Sha256Digest; support_start_ns: i64; support_end_ns: i64; accepted_positive_downward_j_m2_ofe_ground: f64; soil_thermal_credit_j_m2_ofe_ground: f64; snow_soil_heat_receipt_sha256: Sha256Digest`
- nested collection/key types: `none`
- native validator/digest candidates: `fn validate_complete_owner_set(&self) -> Result<(), DirectV9RealConsumerError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `private cfg(test) replay/adapter function required in owning module for SoilThermalTopBoundaryCreditV1`

## `openwepp_land_surface_energy::owner_envelope::SoilThermalLayerSnapshot`

- source: `crates/openwepp-land-surface-energy/src/owner_envelope.rs`
- git blob SHA: `e51fb1d7bf871d51e26a1aba3ae05fbd8b85e9bc`
- normalized declaration SHA-256: `b5cc83d46a1e5dcf478a0a154fda817febb8468bd6c23acb2347156058b5e456`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `layer_id: SoilLayerId; temperature_k: f64; enthalpy_j_m2_ofe_ground: f64`
- nested collection/key types: `none`
- native validator/digest candidates: `fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> { | fn validate_owner_set(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate_identity_stage(&self) -> Result<(), LandSurfaceEnergyError> {`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_land_surface_energy::owner_envelope::SoilThermalOfeSnapshot`

- source: `crates/openwepp-land-surface-energy/src/owner_envelope.rs`
- git blob SHA: `e51fb1d7bf871d51e26a1aba3ae05fbd8b85e9bc`
- normalized declaration SHA-256: `99108b34c30e68a4d064c54108bfe1be45bf34291e408b17df0165676f07babc`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `ofe_id: OfeId; ordered_layers: Vec < SoilThermalLayerSnapshot >`
- nested collection/key types: `Vec < SoilThermalLayerSnapshot >`
- native validator/digest candidates: `fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> { | fn validate_owner_set(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate_identity_stage(&self) -> Result<(), LandSurfaceEnergyError> {`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_land_surface_energy::owner_envelope::SoilThermalSnapshot`

- source: `crates/openwepp-land-surface-energy/src/owner_envelope.rs`
- git blob SHA: `e51fb1d7bf871d51e26a1aba3ae05fbd8b85e9bc`
- normalized declaration SHA-256: `1138b80311c659e506ccdf7761d78b704a772217001d80a43c86089f084b50c6`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `owner_id: ResourceOwnerId; configuration_sha256: Sha256Digest; state_sha256: Sha256Digest; snapshot_sha256: Sha256Digest; last_accepted_transaction_id: Option < TransactionId >; ofes: Vec < SoilThermalOfeSnapshot >`
- nested collection/key types: `Option < TransactionId > | Vec < SoilThermalOfeSnapshot >`
- native validator/digest candidates: `fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> { | fn validate_owner_set(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate_identity_stage(&self) -> Result<(), LandSurfaceEnergyError> {`
- replay class: `3-no-native-wire-diagnostic-adapter-required`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_land_surface_energy::solver::Stage3SnowCoveredLowerBoundary`

- source: `crates/openwepp-land-surface-energy/src/solver.rs`
- git blob SHA: `d8506c1ab9685efd1006f0bd457d023cae639524`
- normalized declaration SHA-256: `694bb736030cd31e72906ad0bb805bed6e0fa43bf644c2fd963485312fb16f00`
- visibility: `pub`
- owner stage: `provider-carrier`
- exact fields/variants: `snow_temperature_k: f64; latent_heat_j_kg: f64; sensible_to_canopy_air_w_m2: f64; vapor_to_canopy_air_kg_m2_s: f64; net_longwave_w_m2: f64; shortwave_absorbed_w_m2: f64; precipitation_advection_w_m2: f64; carrier_receipt_id: Sha256Digest; snow_vis_albedo: f64; snow_nir_albedo: f64; stage3_albedo_state_sha256: Sha256Digest; forcing_receipt_sha256: Sha256Digest; optical_receipt_sha256: Option < Sha256Digest >; reciprocal_longwave_receipt_sha256: Option < Sha256Digest >; final_canopy_boundary_receipt_sha256: Option < Sha256Digest >`
- nested collection/key types: `Option < Sha256Digest >`
- native validator/digest candidates: `fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

## `openwepp_land_surface_energy::solver::Stage3SnowOpticalBoundaryReceiptV1`

- source: `crates/openwepp-land-surface-energy/src/solver.rs`
- git blob SHA: `d8506c1ab9685efd1006f0bd457d023cae639524`
- normalized declaration SHA-256: `9df4fde05eda40bfff03650c473623e755f302a295b80b3ba6f1e83b139e6f63`
- visibility: `pub`
- owner stage: `identity/shared`
- exact fields/variants: `ofe_id: OfeId; tile_id: TileId; terminal_w_m2_tile: BandDirectionalFluxes; absorbed_w_m2_tile: BandDirectionalFluxes; reflected_w_m2_tile: BandDirectionalFluxes; snow_vis_albedo: f64; snow_nir_albedo: f64; stage3_albedo_state_sha256: Sha256Digest; forcing_receipt_sha256: Sha256Digest; receipt_sha256: Sha256Digest`
- nested collection/key types: `none`
- native validator/digest candidates: `fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> { | pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {`
- replay class: `2-native-preimage-bytes-discarded`
- required test-only access: `crate-private cfg(test) serializer may read public fields`

