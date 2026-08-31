const STAGE3_V11_PRODUCTION_QUALIFICATION_SCHEMA_V1: u16 = 1;
const STAGE3_V11_QUALIFICATION_ACCUMULATOR_SCHEMA_V1: u16 = 1;

include!("snow_stage3_v11_production_qualification_record_identity.rs");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualificationOrderedRootV1 {
    pub record_count: u64,
    pub ordered_root_sha256: Digest32,
}

impl Default for SnowStage3V11QualificationOrderedRootV1 {
    fn default() -> Self {
        Self {
            record_count: 0,
            ordered_root_sha256: Digest32::zero(),
        }
    }
}

impl SnowStage3V11QualificationOrderedRootV1 {
    fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if (self.record_count == 0) != (self.ordered_root_sha256 == Digest32::zero()) {
            return Err(qualification_error(
                "qualification ordered-root empty identity",
            ));
        }
        Ok(())
    }

    fn append(
        &mut self,
        domain: &'static str,
        record_sha256: Digest32,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if record_sha256 == Digest32::zero() {
            return Err(qualification_error(
                "qualification ordered-root zero record",
            ));
        }
        let ordinal = self.record_count.to_be_bytes();
        self.ordered_root_sha256 = framed_sha256(
            domain,
            &[
                FramedField {
                    tag: "previous_ordered_root_sha256",
                    value: self.ordered_root_sha256.as_bytes(),
                },
                FramedField {
                    tag: "record_ordinal",
                    value: &ordinal,
                },
                FramedField {
                    tag: "record_sha256",
                    value: record_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| qualification_error("qualification ordered-root framing"))?;
        self.record_count = self
            .record_count
            .checked_add(1)
            .ok_or_else(|| qualification_error("qualification ordered-root count overflow"))?;
        Ok(())
    }

    fn append_stream(
        &mut self,
        domain: &'static str,
        records: &[Digest32],
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        for record in records {
            self.append(domain, *record)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedSurfaceRouteV1 {
    pub source_ofe_id: String,
    pub destination_ofe_id: String,
    pub kind: crate::direct_runtime::DirectSurfaceLiquidParcelKind,
    pub disposition: crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedMassEnthalpyTotalV1 {
    pub mass_kg_m2_basis_ofe_ground: f64,
    pub enthalpy_j_m2_basis_ofe_ground: f64,
}

mod qualification_surface_route_map_wire {
    include!("snow_stage3_v11_production_qualification_route_wire.rs");
}

impl SnowStage3V11QualifiedMassEnthalpyTotalV1 {
    fn checked_add_assign(
        &mut self,
        right: Self,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if !right.mass_kg_m2_basis_ofe_ground.is_finite()
            || !right.enthalpy_j_m2_basis_ofe_ground.is_finite()
            || right.mass_kg_m2_basis_ofe_ground < 0.0
        {
            return Err(qualification_error(
                "qualification mass-enthalpy total domain",
            ));
        }
        self.mass_kg_m2_basis_ofe_ground += right.mass_kg_m2_basis_ofe_ground;
        self.enthalpy_j_m2_basis_ofe_ground += right.enthalpy_j_m2_basis_ofe_ground;
        if !self.mass_kg_m2_basis_ofe_ground.is_finite()
            || !self.enthalpy_j_m2_basis_ofe_ground.is_finite()
        {
            return Err(qualification_error(
                "qualification mass-enthalpy total overflow",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualificationOwnerEndpointV1 {
    pub complete_owner_sha256: Option<Digest32>,
    pub coupled_owner_set_sha256: Digest32,
    pub accepted_until_ns: u128,
    pub soil_thermal_owner_sha256: Option<Digest32>,
    pub biogeochemistry_owner_sha256: Option<Digest32>,
}

impl SnowStage3V11QualificationOwnerEndpointV1 {
    fn validate(
        &self,
        require_complete_owner: bool,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.coupled_owner_set_sha256 == Digest32::zero()
            || self.soil_thermal_owner_sha256 == Some(Digest32::zero())
            || self.biogeochemistry_owner_sha256 == Some(Digest32::zero())
            || self.complete_owner_sha256 == Some(Digest32::zero())
            || (require_complete_owner
                && (self.complete_owner_sha256.is_none()
                    || self.soil_thermal_owner_sha256.is_none()
                    || self.biogeochemistry_owner_sha256.is_none()))
        {
            return Err(qualification_error("qualification owner endpoint identity"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualificationDayDeltaV1 {
    pub schema_version: u16,
    pub day_index: usize,
    pub total_parent_support_count: u64,
    pub adaptive_support_receipt_count: u64,
    pub snow_free_successor_receipt_count: u64,
    pub snow_free_parent_support_count: u64,
    pub terminal_event_count: u64,
    pub publication_event_count: u64,
    pub accepted_publication_support_count: u64,
    pub ammonium_resource_use_n: f64,
    pub nitrate_resource_use_n: f64,
    pub material_transfers: SnowStage3V11QualifiedBgcInventoryV1,
    pub accepted_support_receipt_sha256s: Vec<Digest32>,
    pub surface_receipt_occurrences: Vec<SnowStage3V11QualificationSurfaceReceiptOccurrenceV1>,
    pub event_receipt_sha256s: Vec<Digest32>,
    #[serde(with = "qualification_surface_route_map_wire")]
    pub surface_flow_by_route:
        BTreeMap<SnowStage3V11QualifiedSurfaceRouteV1, SnowStage3V11QualifiedMassEnthalpyTotalV1>,
    pub routed_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1,
    pub upstream_runon: SnowStage3V11QualifiedMassEnthalpyTotalV1,
    pub outlet_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1,
    pub beginning_owner: SnowStage3V11QualificationOwnerEndpointV1,
    pub ending_owner: SnowStage3V11QualificationOwnerEndpointV1,
    pub receipt_sha256: Digest32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualificationAccumulatorV1 {
    pub schema_version: u16,
    pub committed_day_count: usize,
    pub total_parent_support_count: u64,
    pub adaptive_support_receipt_count: u64,
    pub snow_free_successor_receipt_count: u64,
    pub snow_free_parent_support_count: u64,
    pub terminal_event_count: u64,
    pub publication_event_count: u64,
    pub accepted_publication_support_count: u64,
    pub ammonium_resource_use_n: f64,
    pub nitrate_resource_use_n: f64,
    pub material_transfers: SnowStage3V11QualifiedBgcInventoryV1,
    pub accepted_support_receipts: SnowStage3V11QualificationOrderedRootV1,
    pub surface_receipts: SnowStage3V11QualificationOrderedRootV1,
    pub events: SnowStage3V11QualificationOrderedRootV1,
    #[serde(with = "qualification_surface_route_map_wire")]
    pub surface_flow_by_route:
        BTreeMap<SnowStage3V11QualifiedSurfaceRouteV1, SnowStage3V11QualifiedMassEnthalpyTotalV1>,
    pub routed_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1,
    pub upstream_runon: SnowStage3V11QualifiedMassEnthalpyTotalV1,
    pub outlet_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1,
    pub beginning_owner: Option<SnowStage3V11QualificationOwnerEndpointV1>,
    pub ending_owner: Option<SnowStage3V11QualificationOwnerEndpointV1>,
    pub receipt_sha256: Digest32,
}

impl Default for SnowStage3V11QualificationAccumulatorV1 {
    fn default() -> Self {
        Self {
            schema_version: STAGE3_V11_QUALIFICATION_ACCUMULATOR_SCHEMA_V1,
            committed_day_count: 0,
            total_parent_support_count: 0,
            adaptive_support_receipt_count: 0,
            snow_free_successor_receipt_count: 0,
            snow_free_parent_support_count: 0,
            terminal_event_count: 0,
            publication_event_count: 0,
            accepted_publication_support_count: 0,
            ammonium_resource_use_n: 0.0,
            nitrate_resource_use_n: 0.0,
            material_transfers: SnowStage3V11QualifiedBgcInventoryV1::default(),
            accepted_support_receipts: SnowStage3V11QualificationOrderedRootV1::default(),
            surface_receipts: SnowStage3V11QualificationOrderedRootV1::default(),
            events: SnowStage3V11QualificationOrderedRootV1::default(),
            surface_flow_by_route: BTreeMap::new(),
            routed_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            upstream_runon: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            outlet_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            beginning_owner: None,
            ending_owner: None,
            receipt_sha256: Digest32::zero(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedLaneV1 {
    pub lane_id: u32,
    pub next_interval_index: u64,
    pub represented_ice_kg_m2: f64,
    pub detached_retained_liquid_kg_m2: f64,
    pub cumulative_snowfall_kg_m2: f64,
    pub cumulative_external_liquid_kg_m2: f64,
    pub cumulative_deposition_kg_m2: f64,
    pub cumulative_sublimation_kg_m2: f64,
    pub cumulative_melt_kg_m2: f64,
    pub frost_depth_m: f64,
    pub thaw_depth_m: f64,
    pub frozen_water_m: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedSoilThermalLayerV1 {
    pub layer_id: String,
    pub temperature_k: f64,
    pub enthalpy_j_m2: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedSoilThermalOfeV1 {
    pub ofe_id: String,
    pub ordered_layers: Vec<SnowStage3V11QualifiedSoilThermalLayerV1>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedBgcInventoryV1 {
    pub ammonium_n: f64,
    pub nitrate_n: f64,
    pub receiver_carbon: f64,
    pub receiver_nitrogen: f64,
    pub receiver_dry_matter: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedSurfaceReceiptV1 {
    pub day_index: usize,
    pub interval_index: usize,
    pub accepted_support_receipt_sha256: Digest32,
    pub source_ofe_id: String,
    pub destination_ofe_id: String,
    pub kind: crate::direct_runtime::DirectSurfaceLiquidParcelKind,
    pub disposition: crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition,
    pub mass_kg_m2_basis_ofe_ground: f64,
    pub enthalpy_j_m2_basis_ofe_ground: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SnowStage3V11QualificationEndingOwnerSourceV1 {
    CommittedPostReceiverPhysicalOwnerWithCoupledSnow,
    PreReceiverPhysicalOwner,
    DerivedPublicationFrame,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SnowStage3V11QualificationPredecessorSourceV1 {
    FinalPositiveSupportOwnerJoin,
    SubstitutedOwnerJoin,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedOwnerPredecessorV1 {
    pub source: SnowStage3V11QualificationPredecessorSourceV1,
    pub support_end_ns: u128,
    pub owner_join_receipt_sha256: Digest32,
    pub ending_complete_owner_set_sha256: Digest32,
    pub soil_thermal_owner_sha256: Digest32,
    pub biogeochemistry_owner_sha256: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11QualifiedEndingOwnerV1 {
    pub source: SnowStage3V11QualificationEndingOwnerSourceV1,
    pub complete_owner_sha256: Digest32,
    pub coupled_owner_set_sha256: Digest32,
    pub accepted_until_ns: u128,
    pub owner_sha256_by_id: BTreeMap<String, Digest32>,
    pub predecessor: SnowStage3V11QualifiedOwnerPredecessorV1,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11ProductionQualificationSnapshotV1 {
    pub schema_version: u16,
    pub next_day_index: usize,
    pub committed_day_count: usize,
    pub total_parent_support_count: u64,
    pub adaptive_support_receipt_count: u64,
    pub snow_free_successor_receipt_count: u64,
    pub snow_free_parent_support_count: u64,
    pub terminal_event_count: u64,
    pub accepted_publication_support_count: u64,
    pub accepted_support_receipt_root_sha256: Digest32,
    pub surface_receipt_count: u64,
    pub surface_receipt_root_sha256: Digest32,
    pub publication_event_count: u64,
    pub publication_event_root_sha256: Digest32,
    pub lanes: Vec<SnowStage3V11QualifiedLaneV1>,
    pub ending_owner: SnowStage3V11QualifiedEndingOwnerV1,
    pub soil_thermal_owner_sha256: Digest32,
    pub soil_thermal_ofes: Vec<SnowStage3V11QualifiedSoilThermalOfeV1>,
    pub biogeochemistry_owner_sha256: Digest32,
    pub beginning_biogeochemistry: SnowStage3V11QualifiedBgcInventoryV1,
    pub ending_biogeochemistry: SnowStage3V11QualifiedBgcInventoryV1,
    pub biogeochemistry_delta: SnowStage3V11QualifiedBgcInventoryV1,
    pub ending_biogeochemistry_last_transaction_id: u128,
    pub accepted_support_receipt_sha256s: Vec<Digest32>,
    pub surface_receipts: Vec<SnowStage3V11QualifiedSurfaceReceiptV1>,
    #[serde(with = "qualification_surface_route_map_wire")]
    pub surface_flow_by_route:
        BTreeMap<SnowStage3V11QualifiedSurfaceRouteV1, SnowStage3V11QualifiedMassEnthalpyTotalV1>,
    pub routed_runoff_mass_kg_m2: f64,
    pub routed_runoff_enthalpy_j_m2: f64,
    pub upstream_runon_mass_kg_m2: f64,
    pub upstream_runon_enthalpy_j_m2: f64,
    pub outlet_runoff_mass_kg_m2: f64,
    pub outlet_runoff_enthalpy_j_m2: f64,
    pub receipt_sha256: Digest32,
}

fn qualification_error(detail: &'static str) -> DirectSnowStage3V11AttachmentError {
    DirectSnowStage3V11AttachmentError::Identity(detail)
}

fn qualification_add(left: &mut u64, right: u64) -> Result<(), DirectSnowStage3V11AttachmentError> {
    *left = left
        .checked_add(right)
        .ok_or_else(|| qualification_error("qualification counter overflow"))?;
    Ok(())
}

fn qualification_add_f64(
    left: &mut f64,
    right: f64,
    allow_negative: bool,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if !right.is_finite() || (!allow_negative && right < 0.0) {
        return Err(qualification_error("qualification numeric delta domain"));
    }
    *left += right;
    if !left.is_finite() {
        return Err(qualification_error("qualification numeric total overflow"));
    }
    Ok(())
}

fn qualification_digest_serialized<T: Serialize>(
    domain: &'static [u8],
    value: &T,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|_| qualification_error("qualification canonical serialization"))?;
    let mut framed = domain.to_vec();
    framed.extend_from_slice(&bytes);
    Ok(digest_bytes(&framed))
}

fn qualification_ordered_root(
    domain: &'static str,
    records: &[Digest32],
) -> Result<SnowStage3V11QualificationOrderedRootV1, DirectSnowStage3V11AttachmentError> {
    let mut root = SnowStage3V11QualificationOrderedRootV1::default();
    root.append_stream(domain, records)?;
    Ok(root)
}

fn validate_qualification_day_endpoint_join(
    beginning_owner: &SnowStage3V11QualificationOwnerEndpointV1,
    ending_owner: &SnowStage3V11QualificationOwnerEndpointV1,
    publication_beginning_owner_sha256: Digest32,
    publication_ending_owner_sha256: Digest32,
    first_support_start_ns: u128,
    last_support_end_ns: u128,
    day_start_ns: u128,
    day_end_ns: u128,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if first_support_start_ns != day_start_ns
        || last_support_end_ns != day_end_ns
        || beginning_owner.coupled_owner_set_sha256 != publication_beginning_owner_sha256
        || beginning_owner.accepted_until_ns != day_start_ns
    {
        return Err(qualification_error(
            "qualification daily beginning endpoint",
        ));
    }
    if ending_owner.coupled_owner_set_sha256 != publication_ending_owner_sha256
        || ending_owner.accepted_until_ns != day_end_ns
    {
        return Err(qualification_error("qualification daily ending endpoint"));
    }
    Ok(())
}

fn validate_qualification_beginning_event_bridge(
    publication_beginning_owner_sha256: Digest32,
    first_support_beginning_owner_sha256: Digest32,
    day_start_ns: u128,
    events: &[AcceptedEventReceiptV1],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let mut owner = publication_beginning_owner_sha256;
    let mut receipts = BTreeSet::new();
    for event in events
        .iter()
        .take_while(|event| event.tick().get() == day_start_ns)
    {
        event
            .validate()
            .map_err(|_| qualification_error("qualification daily beginning event seal"))?;
        if event.beginning_owner_set_digest() != owner
            || event.ending_owner_set_digest() == Digest32::zero()
            || !receipts.insert(event.id().digest())
        {
            return Err(qualification_error(
                "qualification daily beginning event bridge",
            ));
        }
        owner = event.ending_owner_set_digest();
    }
    if owner != first_support_beginning_owner_sha256 {
        return Err(qualification_error(
            "qualification daily beginning event bridge",
        ));
    }
    Ok(())
}

impl SnowStage3V11QualificationDayDeltaV1 {
    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let mut value = self.clone();
        value.receipt_sha256 = Digest32::zero();
        qualification_digest_serialized(
            b"OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_DAY_DELTA_V1\0",
            &value,
        )
    }

    pub fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        self.receipt_sha256 = self.reconstructed_digest()?;
        self.validate()?;
        Ok(self)
    }

    #[must_use]
    pub const fn receipt_sha256(&self) -> Digest32 {
        self.receipt_sha256
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.schema_version != STAGE3_V11_QUALIFICATION_ACCUMULATOR_SCHEMA_V1
            || self.total_parent_support_count != STAGE3_V11_PARENT_SUPPORT_COUNT as u64
            || self
                .adaptive_support_receipt_count
                .checked_add(self.snow_free_parent_support_count)
                != Some(self.total_parent_support_count)
        {
            return Err(qualification_error("qualification daily counter topology"));
        }
        if usize::try_from(self.accepted_publication_support_count).ok()
            != Some(self.accepted_support_receipt_sha256s.len())
            || usize::try_from(self.publication_event_count).ok()
                != Some(self.event_receipt_sha256s.len())
            || self.terminal_event_count > self.publication_event_count
        {
            return Err(qualification_error(
                "qualification daily publication cardinality",
            ));
        }
        validate_qualification_digest_records(
            "accepted_support_receipt_sha256s",
            &self.accepted_support_receipt_sha256s,
            true,
        )?;
        validate_qualification_surface_receipt_occurrences(&self.surface_receipt_occurrences)?;
        validate_qualification_digest_records(
            "event_receipt_sha256s",
            &self.event_receipt_sha256s,
            false,
        )?;
        self.beginning_owner.validate(false)?;
        self.ending_owner.validate(true)?;
        let expected_end = u128::try_from(self.day_index)
            .ok()
            .and_then(|day| day.checked_add(1))
            .and_then(|day| day.checked_mul(STAGE3_V11_DAY_NS));
        if expected_end != Some(self.ending_owner.accepted_until_ns)
            || self
                .beginning_owner
                .accepted_until_ns
                .checked_add(STAGE3_V11_DAY_NS)
                != Some(self.ending_owner.accepted_until_ns)
        {
            return Err(qualification_error("qualification daily owner chronology"));
        }
        let bgc_values = [
            self.ammonium_resource_use_n,
            self.nitrate_resource_use_n,
            self.material_transfers.ammonium_n,
            self.material_transfers.nitrate_n,
            self.material_transfers.receiver_carbon,
            self.material_transfers.receiver_nitrogen,
            self.material_transfers.receiver_dry_matter,
        ];
        if bgc_values
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(qualification_error(
                "qualification daily resource/material totals",
            ));
        }
        let mut reconstructed_routed = SnowStage3V11QualifiedMassEnthalpyTotalV1::default();
        let mut reconstructed_runon = SnowStage3V11QualifiedMassEnthalpyTotalV1::default();
        let mut reconstructed_outlet = SnowStage3V11QualifiedMassEnthalpyTotalV1::default();
        for (route, total) in &self.surface_flow_by_route {
            if route.source_ofe_id.is_empty() || route.destination_ofe_id.is_empty() {
                return Err(qualification_error("qualification daily surface route"));
            }
            let mut checked = SnowStage3V11QualifiedMassEnthalpyTotalV1::default();
            checked.checked_add_assign(*total)?;
            if route.disposition
                == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
            {
                reconstructed_routed.checked_add_assign(*total)?;
            }
            if route.kind == crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon {
                reconstructed_runon.checked_add_assign(*total)?;
            }
            if route.disposition
                == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::OutletRunoff
            {
                reconstructed_outlet.checked_add_assign(*total)?;
            }
        }
        if reconstructed_routed != self.routed_runoff
            || reconstructed_runon != self.upstream_runon
            || reconstructed_outlet != self.outlet_runoff
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(qualification_error(
                "qualification daily totals or receipt seal",
            ));
        }
        Ok(())
    }
}

impl SnowStage3V11QualificationAccumulatorV1 {
    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let mut value = self.clone();
        value.receipt_sha256 = Digest32::zero();
        qualification_digest_serialized(
            b"OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_ACCUMULATOR_V1\0",
            &value,
        )
    }

    #[must_use]
    pub const fn receipt_sha256(&self) -> Digest32 {
        self.receipt_sha256
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.schema_version != STAGE3_V11_QUALIFICATION_ACCUMULATOR_SCHEMA_V1 {
            return Err(qualification_error("qualification accumulator schema"));
        }
        self.accepted_support_receipts.validate()?;
        self.surface_receipts.validate()?;
        self.events.validate()?;
        if self.committed_day_count == 0 {
            let empty = Self::default();
            if self != &empty {
                return Err(qualification_error(
                    "qualification accumulator noncanonical empty state",
                ));
            }
            return Ok(());
        }
        if self.total_parent_support_count
            != (self.committed_day_count as u64) * (STAGE3_V11_PARENT_SUPPORT_COUNT as u64)
            || self.accepted_support_receipts.record_count
                != self.accepted_publication_support_count
            || self.events.record_count != self.publication_event_count
            || self.beginning_owner.is_none()
            || self.ending_owner.is_none()
        {
            return Err(qualification_error("qualification accumulator cardinality"));
        }
        self.beginning_owner
            .as_ref()
            .ok_or_else(|| qualification_error("qualification accumulator beginning owner"))?
            .validate(false)?;
        let ending = self
            .ending_owner
            .as_ref()
            .ok_or_else(|| qualification_error("qualification accumulator ending owner"))?;
        ending.validate(true)?;
        if ending.accepted_until_ns != (self.committed_day_count as u128) * STAGE3_V11_DAY_NS
            || self.receipt_sha256 != self.reconstructed_digest()?
        {
            return Err(qualification_error(
                "qualification accumulator endpoint or seal",
            ));
        }
        Ok(())
    }

    pub fn fold_day(
        &mut self,
        day: &SnowStage3V11QualificationDayDeltaV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        day.validate()?;
        let mut candidate = self.clone();
        candidate.fold_day_inner(day)?;
        candidate.receipt_sha256 = candidate.reconstructed_digest()?;
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    pub fn reconstruct_from_days<'a>(
        days: impl IntoIterator<Item = &'a SnowStage3V11QualificationDayDeltaV1>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut value = Self::default();
        for day in days {
            value.fold_day(day)?;
        }
        Ok(value)
    }

    pub fn validate_stream_reconstruction<'a>(
        &self,
        days: impl IntoIterator<Item = &'a SnowStage3V11QualificationDayDeltaV1>,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let reconstructed = Self::reconstruct_from_days(days)?;
        if &reconstructed != self {
            return Err(qualification_error(
                "qualification accumulator stream reconstruction",
            ));
        }
        Ok(())
    }

    fn fold_day_inner(
        &mut self,
        day: &SnowStage3V11QualificationDayDeltaV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if day.day_index != self.committed_day_count
            || self
                .ending_owner
                .as_ref()
                .is_some_and(|ending| ending != &day.beginning_owner)
        {
            return Err(qualification_error(
                "qualification accumulator day order or owner substitution",
            ));
        }
        if self.beginning_owner.is_none() {
            self.beginning_owner = Some(day.beginning_owner.clone());
        }
        self.ending_owner = Some(day.ending_owner.clone());
        self.committed_day_count = self
            .committed_day_count
            .checked_add(1)
            .ok_or_else(|| qualification_error("qualification day-count overflow"))?;
        for (left, right) in [
            (
                &mut self.total_parent_support_count,
                day.total_parent_support_count,
            ),
            (
                &mut self.adaptive_support_receipt_count,
                day.adaptive_support_receipt_count,
            ),
            (
                &mut self.snow_free_successor_receipt_count,
                day.snow_free_successor_receipt_count,
            ),
            (
                &mut self.snow_free_parent_support_count,
                day.snow_free_parent_support_count,
            ),
            (&mut self.terminal_event_count, day.terminal_event_count),
            (
                &mut self.publication_event_count,
                day.publication_event_count,
            ),
            (
                &mut self.accepted_publication_support_count,
                day.accepted_publication_support_count,
            ),
        ] {
            qualification_add(left, right)?;
        }
        qualification_add_f64(
            &mut self.ammonium_resource_use_n,
            day.ammonium_resource_use_n,
            false,
        )?;
        qualification_add_f64(
            &mut self.nitrate_resource_use_n,
            day.nitrate_resource_use_n,
            false,
        )?;
        for (left, right) in [
            (
                &mut self.material_transfers.ammonium_n,
                day.material_transfers.ammonium_n,
            ),
            (
                &mut self.material_transfers.nitrate_n,
                day.material_transfers.nitrate_n,
            ),
            (
                &mut self.material_transfers.receiver_carbon,
                day.material_transfers.receiver_carbon,
            ),
            (
                &mut self.material_transfers.receiver_nitrogen,
                day.material_transfers.receiver_nitrogen,
            ),
            (
                &mut self.material_transfers.receiver_dry_matter,
                day.material_transfers.receiver_dry_matter,
            ),
        ] {
            qualification_add_f64(left, right, false)?;
        }
        self.accepted_support_receipts.append_stream(
            "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_ACCEPTED_SUPPORT_ROOT_V1",
            &day.accepted_support_receipt_sha256s,
        )?;
        for occurrence in &day.surface_receipt_occurrences {
            self.surface_receipts.append(
                "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_SURFACE_RECEIPT_ROOT_V1",
                occurrence.receipt_sha256,
            )?;
        }
        self.events.append_stream(
            "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_EVENT_ROOT_V1",
            &day.event_receipt_sha256s,
        )?;
        for (route, total) in &day.surface_flow_by_route {
            self.surface_flow_by_route
                .entry(route.clone())
                .or_default()
                .checked_add_assign(*total)?;
        }
        self.routed_runoff.checked_add_assign(day.routed_runoff)?;
        self.upstream_runon.checked_add_assign(day.upstream_runon)?;
        self.outlet_runoff.checked_add_assign(day.outlet_runoff)?;
        Ok(())
    }
}

fn qualification_destination(
    receipt: &crate::direct_runtime::DirectSurfaceLiquidParcelReceipt,
) -> String {
    use crate::direct_runtime::DirectSurfaceLiquidReceiptRecipient;
    match &receipt.recipient {
        DirectSurfaceLiquidReceiptRecipient::SoilInfiltration { ofe_id, .. }
        | DirectSurfaceLiquidReceiptRecipient::Outlet { ofe_id } => ofe_id.as_str().to_owned(),
        DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key } => {
            store_key.ofe_id.as_str().to_owned()
        }
        DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
            destination_ofe_id, ..
        } => destination_ofe_id.as_str().to_owned(),
    }
}

fn qualification_complete_owner_sha256(
    owner_bytes: &BTreeMap<String, Vec<u8>>,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let bytes = serde_json::to_vec(owner_bytes)
        .map_err(|_| qualification_error("qualification complete-owner serialization"))?;
    let mut framed = b"OPENWEPP_SNOW_STAGE3_V11_QUALIFIED_COMPLETE_OWNER_V1\0".to_vec();
    framed.extend_from_slice(&bytes);
    Ok(digest_bytes(&framed))
}

fn qualification_ending_owner_authority(
    mut committed_physical_owner_bytes: BTreeMap<String, Vec<u8>>,
    coupled_owners: &[openwepp_coupled_time::OwnerState],
    expected_complete_owner_bytes: &BTreeMap<String, Vec<u8>>,
    expected_coupled_owner_set_sha256: Digest32,
    accepted_until_ns: u128,
    predecessor: SnowStage3V11QualifiedOwnerPredecessorV1,
) -> Result<SnowStage3V11QualifiedEndingOwnerV1, DirectSnowStage3V11AttachmentError> {
    let coupled_owner_set_sha256 = complete_owner_set_digest(coupled_owners)?;
    bind_parent_receipt_snow_owner_bytes_v1(&mut committed_physical_owner_bytes, coupled_owners)?;
    if coupled_owner_set_sha256 != expected_coupled_owner_set_sha256
        || committed_physical_owner_bytes != *expected_complete_owner_bytes
    {
        return Err(qualification_error(
            "qualification ending owner substitution",
        ));
    }
    let owner_sha256_by_id = committed_physical_owner_bytes
        .iter()
        .map(|(owner_id, bytes)| (owner_id.clone(), digest_bytes(bytes)))
        .collect::<BTreeMap<_, _>>();
    Ok(SnowStage3V11QualifiedEndingOwnerV1 {
        source: SnowStage3V11QualificationEndingOwnerSourceV1::CommittedPostReceiverPhysicalOwnerWithCoupledSnow,
        complete_owner_sha256: qualification_complete_owner_sha256(
            &committed_physical_owner_bytes,
        )?,
        coupled_owner_set_sha256,
        accepted_until_ns,
        owner_sha256_by_id,
        predecessor,
    })
}

impl SnowStage3V11ProductionQualificationSnapshotV1 {
    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let mut value = self.clone();
        value.receipt_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&value)
            .map_err(|_| qualification_error("qualification snapshot serialization"))?;
        let mut framed = b"OPENWEPP_SNOW_STAGE3_V11_PRODUCTION_QUALIFICATION_V1\0".to_vec();
        framed.extend_from_slice(&bytes);
        Ok(digest_bytes(&framed))
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let finite = self
            .lanes
            .iter()
            .flat_map(|lane| {
                [
                    lane.represented_ice_kg_m2,
                    lane.detached_retained_liquid_kg_m2,
                    lane.cumulative_snowfall_kg_m2,
                    lane.cumulative_external_liquid_kg_m2,
                    lane.cumulative_deposition_kg_m2,
                    lane.cumulative_sublimation_kg_m2,
                    lane.cumulative_melt_kg_m2,
                    lane.frost_depth_m,
                    lane.thaw_depth_m,
                    lane.frozen_water_m,
                ]
            })
            .chain(self.surface_receipts.iter().flat_map(|row| {
                [
                    row.mass_kg_m2_basis_ofe_ground,
                    row.enthalpy_j_m2_basis_ofe_ground,
                ]
            }))
            .all(f64::is_finite);
        let accepted = self
            .accepted_support_receipt_sha256s
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let bgc_values = [
            self.beginning_biogeochemistry.ammonium_n,
            self.beginning_biogeochemistry.nitrate_n,
            self.beginning_biogeochemistry.receiver_carbon,
            self.beginning_biogeochemistry.receiver_nitrogen,
            self.beginning_biogeochemistry.receiver_dry_matter,
            self.ending_biogeochemistry.ammonium_n,
            self.ending_biogeochemistry.nitrate_n,
            self.ending_biogeochemistry.receiver_carbon,
            self.ending_biogeochemistry.receiver_nitrogen,
            self.ending_biogeochemistry.receiver_dry_matter,
        ];
        let reconstructed_delta = SnowStage3V11QualifiedBgcInventoryV1 {
            ammonium_n: self.ending_biogeochemistry.ammonium_n
                - self.beginning_biogeochemistry.ammonium_n,
            nitrate_n: self.ending_biogeochemistry.nitrate_n
                - self.beginning_biogeochemistry.nitrate_n,
            receiver_carbon: self.ending_biogeochemistry.receiver_carbon
                - self.beginning_biogeochemistry.receiver_carbon,
            receiver_nitrogen: self.ending_biogeochemistry.receiver_nitrogen
                - self.beginning_biogeochemistry.receiver_nitrogen,
            receiver_dry_matter: self.ending_biogeochemistry.receiver_dry_matter
                - self.beginning_biogeochemistry.receiver_dry_matter,
        };
        let sum = |kind: Option<crate::direct_runtime::DirectSurfaceLiquidParcelKind>,
                   disposition: Option<
            crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition,
        >| {
            self.surface_receipts
                .iter()
                .filter(|row| {
                    kind.map_or(true, |k| row.kind == k)
                        && disposition.map_or(true, |d| row.disposition == d)
                })
                .fold((0.0, 0.0), |acc, row| {
                    (
                        acc.0 + row.mass_kg_m2_basis_ofe_ground,
                        acc.1 + row.enthalpy_j_m2_basis_ofe_ground,
                    )
                })
        };
        let mut routed = sum(
            None,
            Some(crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff),
        );
        let mut runon = sum(
            Some(crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon),
            None,
        );
        let mut outlet = sum(
            None,
            Some(crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::OutletRunoff),
        );
        if !self.surface_flow_by_route.is_empty() {
            routed = self
                .surface_flow_by_route
                .iter()
                .filter(|(route, _)| {
                    route.disposition
                        == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
                })
                .fold((0.0, 0.0), |total, (_, value)| {
                    (
                        total.0 + value.mass_kg_m2_basis_ofe_ground,
                        total.1 + value.enthalpy_j_m2_basis_ofe_ground,
                    )
                });
            runon = self
                .surface_flow_by_route
                .iter()
                .filter(|(route, _)| {
                    route.kind
                        == crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon
                })
                .fold((0.0, 0.0), |total, (_, value)| {
                    (
                        total.0 + value.mass_kg_m2_basis_ofe_ground,
                        total.1 + value.enthalpy_j_m2_basis_ofe_ground,
                    )
                });
            outlet = self
                .surface_flow_by_route
                .iter()
                .filter(|(route, _)| {
                    route.disposition
                        == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::OutletRunoff
                })
                .fold((0.0, 0.0), |total, (_, value)| {
                    (
                        total.0 + value.mass_kg_m2_basis_ofe_ground,
                        total.1 + value.enthalpy_j_m2_basis_ofe_ground,
                    )
                });
        }
        let qualified_owner_digests_are_valid = self.ending_owner.source
            == SnowStage3V11QualificationEndingOwnerSourceV1::CommittedPostReceiverPhysicalOwnerWithCoupledSnow
            && self.ending_owner.complete_owner_sha256 != Digest32::zero()
            && self.ending_owner.coupled_owner_set_sha256 != Digest32::zero()
            && self.ending_owner.accepted_until_ns > 0
            && !self.ending_owner.owner_sha256_by_id.is_empty()
            && self
                .ending_owner
                .owner_sha256_by_id
                .values()
                .all(|digest| *digest != Digest32::zero())
            && self.ending_owner.owner_sha256_by_id.get("soil_thermal")
                == Some(&self.soil_thermal_owner_sha256)
            && self.ending_owner.owner_sha256_by_id.get("bgc")
                == Some(&self.biogeochemistry_owner_sha256)
            && self.ending_owner.owner_sha256_by_id.contains_key("snow")
            && self.ending_owner.predecessor.source
                == SnowStage3V11QualificationPredecessorSourceV1::FinalPositiveSupportOwnerJoin
            && self.ending_owner.predecessor.support_end_ns
                <= self.ending_owner.accepted_until_ns
            && self.ending_owner.predecessor.owner_join_receipt_sha256 != Digest32::zero()
            && self
                .ending_owner
                .predecessor
                .ending_complete_owner_set_sha256
                != Digest32::zero()
            && self.ending_owner.predecessor.soil_thermal_owner_sha256 != Digest32::zero()
            && self
                .ending_owner
                .predecessor
                .biogeochemistry_owner_sha256
                != Digest32::zero();
        if self.schema_version != STAGE3_V11_PRODUCTION_QUALIFICATION_SCHEMA_V1 {
            return Err(qualification_error("qualification snapshot schema"));
        }
        if self.next_day_index == 0 || self.committed_day_count != self.next_day_index {
            return Err(qualification_error("qualification snapshot day chronology"));
        }
        if self.total_parent_support_count
            != (self.committed_day_count as u64) * (STAGE3_V11_PARENT_SUPPORT_COUNT as u64)
            || self
                .adaptive_support_receipt_count
                .checked_add(self.snow_free_parent_support_count)
                != Some(self.total_parent_support_count)
        {
            return Err(qualification_error("qualification snapshot parent counts"));
        }
        let surface_receipt_count = usize::try_from(self.surface_receipt_count)
            .map_err(|_| qualification_error("qualification snapshot surface receipt count"))?;
        if (self.accepted_publication_support_count > 0
            && self.accepted_support_receipt_root_sha256 == Digest32::zero())
            || (self.surface_receipt_count > 0
                && self.surface_receipt_root_sha256 == Digest32::zero())
            || (self.publication_event_count > 0
                && self.publication_event_root_sha256 == Digest32::zero())
            || (!self.accepted_support_receipt_sha256s.is_empty()
                && usize::try_from(self.accepted_publication_support_count).ok()
                    != Some(self.accepted_support_receipt_sha256s.len()))
            || self.surface_receipts.len() > surface_receipt_count
        {
            return Err(qualification_error(
                "qualification snapshot accepted publication support count",
            ));
        }
        if self.lanes.is_empty()
            || self
                .lanes
                .windows(2)
                .any(|pair| pair[0].lane_id >= pair[1].lane_id)
        {
            return Err(qualification_error("qualification snapshot lane order"));
        }
        if !finite {
            return Err(qualification_error("qualification snapshot finite values"));
        }
        if bgc_values
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(qualification_error("qualification snapshot BGC inventory"));
        }
        if reconstructed_delta != self.biogeochemistry_delta {
            return Err(qualification_error("qualification snapshot BGC delta"));
        }
        if !qualified_owner_digests_are_valid {
            return Err(qualification_error("qualification snapshot ending owner"));
        }
        if self.soil_thermal_owner_sha256 == Digest32::zero()
            || self.biogeochemistry_owner_sha256 == Digest32::zero()
        {
            return Err(qualification_error("qualification snapshot owner digests"));
        }
        if (!self.accepted_support_receipt_sha256s.is_empty()
            && accepted.len() != self.accepted_support_receipt_sha256s.len())
            || accepted.contains(&Digest32::zero())
        {
            return Err(qualification_error(
                "qualification snapshot accepted receipt identities",
            ));
        }
        if !self.accepted_support_receipt_sha256s.is_empty()
            && qualification_ordered_root(
                "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_ACCEPTED_SUPPORT_ROOT_V1",
                &self.accepted_support_receipt_sha256s,
            )?
            .ordered_root_sha256
                != self.accepted_support_receipt_root_sha256
        {
            return Err(qualification_error(
                "qualification snapshot accepted receipt root",
            ));
        }
        for (route, total) in &self.surface_flow_by_route {
            if route.source_ofe_id.is_empty() || route.destination_ofe_id.is_empty() {
                return Err(qualification_error("qualification snapshot surface route"));
            }
            let mut checked = SnowStage3V11QualifiedMassEnthalpyTotalV1::default();
            checked.checked_add_assign(*total)?;
        }
        if self.surface_receipts.iter().any(|row| {
            !self.accepted_support_receipt_sha256s.is_empty()
                && !accepted.contains(&row.accepted_support_receipt_sha256)
        }) {
            return Err(qualification_error(
                "qualification snapshot surface receipt membership",
            ));
        }
        if routed.0.to_bits() != self.routed_runoff_mass_kg_m2.to_bits()
            || routed.1.to_bits() != self.routed_runoff_enthalpy_j_m2.to_bits()
        {
            return Err(qualification_error("qualification snapshot routed runoff"));
        }
        if runon.0.to_bits() != self.upstream_runon_mass_kg_m2.to_bits()
            || runon.1.to_bits() != self.upstream_runon_enthalpy_j_m2.to_bits()
        {
            return Err(qualification_error("qualification snapshot upstream runon"));
        }
        if outlet.0.to_bits() != self.outlet_runoff_mass_kg_m2.to_bits()
            || outlet.1.to_bits() != self.outlet_runoff_enthalpy_j_m2.to_bits()
        {
            return Err(qualification_error("qualification snapshot outlet runoff"));
        }
        if self.receipt_sha256 != self.reconstructed_digest()? {
            return Err(qualification_error("qualification snapshot receipt seal"));
        }
        Ok(())
    }
}

impl DirectSnowStage3V11ShadowAttachment {
    pub(crate) fn build_qualification_day_delta_v1(
        &self,
        day_index: usize,
    ) -> Result<SnowStage3V11QualificationDayDeltaV1, DirectSnowStage3V11AttachmentError> {
        if self.pending_candidate.is_some()
            || self.pending_publication_day.is_some()
            || self.in_progress_execution.is_some()
            || self.pending_committed_day_evidence.is_some()
            || self.committed.receipt_chain.len() != 1
            || self.archived_receipt_prefix.archived_day_count != day_index
        {
            return Err(qualification_error(
                "qualification daily delta committed posture",
            ));
        }
        let receipt = self
            .committed
            .receipt_chain
            .last()
            .ok_or_else(|| qualification_error("qualification daily receipt"))?;
        if receipt.day_index != day_index
            || receipt.support_count != STAGE3_V11_PARENT_SUPPORT_COUNT
        {
            return Err(qualification_error("qualification daily chronology"));
        }
        receipt.validate_against_ending(&self.committed)?;
        receipt.validate_adaptive_publication_crossjoin_v1(&self.committed.real_consumer)?;
        let supports = self
            .committed
            .real_consumer
            .accepted_publication_supports_for_day(day_index)?;
        let publication_day = self.committed_publication_day(day_index)?;
        let events = self
            .committed
            .real_consumer
            .accepted_publication_event_handoffs();
        let day_start_ns = (day_index as u128)
            .checked_mul(STAGE3_V11_DAY_NS)
            .ok_or_else(|| qualification_error("qualification daily start overflow"))?;
        let day_end_ns = day_start_ns
            .checked_add(STAGE3_V11_DAY_NS)
            .ok_or_else(|| qualification_error("qualification daily end overflow"))?;
        let first_support = supports
            .first()
            .ok_or_else(|| qualification_error("qualification daily first support"))?;
        let last_support = supports
            .last()
            .ok_or_else(|| qualification_error("qualification daily last support"))?;
        let beginning_owner = if let Some(endpoint) = self
            .archived_receipt_prefix
            .qualification_accumulator
            .ending_owner
            .clone()
        {
            endpoint
        } else {
            SnowStage3V11QualificationOwnerEndpointV1 {
                complete_owner_sha256: None,
                coupled_owner_set_sha256: publication_day.beginning_complete_owner_set_sha256(),
                accepted_until_ns: day_start_ns,
                soil_thermal_owner_sha256: None,
                biogeochemistry_owner_sha256: None,
            }
        };
        let owner_bytes = &receipt.complete_owner_bytes;
        let soil_thermal_owner_sha256 = digest_bytes(
            owner_bytes
                .get("soil_thermal")
                .ok_or_else(|| qualification_error("qualification daily soil owner"))?,
        );
        let biogeochemistry_owner_sha256 = digest_bytes(
            owner_bytes
                .get("bgc")
                .ok_or_else(|| qualification_error("qualification daily BGC owner"))?,
        );
        let ending_owner = SnowStage3V11QualificationOwnerEndpointV1 {
            complete_owner_sha256: Some(qualification_complete_owner_sha256(owner_bytes)?),
            coupled_owner_set_sha256: receipt.ending_coupled_owner_set_sha256,
            accepted_until_ns: receipt.ending_coupled_accepted_until_ns.get(),
            soil_thermal_owner_sha256: Some(soil_thermal_owner_sha256),
            biogeochemistry_owner_sha256: Some(biogeochemistry_owner_sha256),
        };
        validate_qualification_day_endpoint_join(
            &beginning_owner,
            &ending_owner,
            publication_day.beginning_complete_owner_set_sha256(),
            publication_day.ending_complete_owner_set_sha256(),
            first_support.support().start_ns().get(),
            last_support.support().end_ns().get(),
            day_start_ns,
            day_end_ns,
        )?;
        validate_qualification_beginning_event_bridge(
            publication_day.beginning_complete_owner_set_sha256(),
            first_support.beginning_complete_owner_set_sha256(),
            day_start_ns,
            events,
        )?;

        for adaptive in &receipt.adaptive_support_receipts {
            adaptive.validate()?;
        }

        let adaptive_parent_ids = receipt
            .adaptive_support_receipts
            .iter()
            .map(|value| value.parent_transaction_id)
            .collect::<BTreeSet<_>>();
        let snow_free_parent_support_count = receipt
            .snow_free_successor_receipts
            .iter()
            .map(|value| value.parent_transaction_id)
            .filter(|parent| !adaptive_parent_ids.contains(parent))
            .collect::<BTreeSet<_>>()
            .len() as u64;
        let mut ammonium_resource_use_n = 0.0;
        let mut nitrate_resource_use_n = 0.0;
        let mut material_transfers = SnowStage3V11QualifiedBgcInventoryV1::default();
        let mut accepted_support_receipt_sha256s = Vec::with_capacity(supports.len());
        let mut surface_receipt_occurrences = Vec::new();
        let mut surface_flow_by_route = BTreeMap::new();
        for support in supports {
            accepted_support_receipt_sha256s.push(support.receipt_sha256());
            for debit in support.resource_debits() {
                if let openwepp_vegetation::v11::V11ResourceKey::MineralNitrogen(key) =
                    &debit.resource_key
                {
                    match key.species {
                        openwepp_kernel_contract::MineralNitrogenSpecies::Ammonium => {
                            qualification_add_f64(
                                &mut ammonium_resource_use_n,
                                debit.final_use,
                                false,
                            )?;
                        }
                        openwepp_kernel_contract::MineralNitrogenSpecies::Nitrate => {
                            qualification_add_f64(
                                &mut nitrate_resource_use_n,
                                debit.final_use,
                                false,
                            )?;
                        }
                    }
                }
            }
            for transfer in support.material_transfers() {
                qualification_add_f64(
                    &mut material_transfers.receiver_carbon,
                    transfer.carbon,
                    false,
                )?;
                qualification_add_f64(
                    &mut material_transfers.receiver_nitrogen,
                    transfer.nitrogen,
                    false,
                )?;
                qualification_add_f64(
                    &mut material_transfers.receiver_dry_matter,
                    transfer.dry_matter,
                    false,
                )?;
            }
            for (source_receipt_ordinal, row) in support.ingress_receipts().iter().enumerate() {
                let row_sha256 = qualification_digest_serialized(
                    b"OPENWEPP_SNOW_STAGE3_V11_QUALIFIED_SURFACE_RECEIPT_V1\0",
                    row,
                )?;
                surface_receipt_occurrences.push(
                    SnowStage3V11QualificationSurfaceReceiptOccurrenceV1::try_new(
                        support.receipt_sha256(),
                        support.support().start_ns().get(),
                        support.support().end_ns().get(),
                        support.interval_index(),
                        source_receipt_ordinal,
                        row_sha256,
                    )?,
                );
                let route = SnowStage3V11QualifiedSurfaceRouteV1 {
                    source_ofe_id: row.origin_store_key.ofe_id.as_str().to_owned(),
                    destination_ofe_id: qualification_destination(row),
                    kind: row.kind,
                    disposition: row.disposition,
                };
                surface_flow_by_route
                    .entry(route)
                    .or_insert_with(SnowStage3V11QualifiedMassEnthalpyTotalV1::default)
                    .checked_add_assign(SnowStage3V11QualifiedMassEnthalpyTotalV1 {
                        mass_kg_m2_basis_ofe_ground: row.mass_kg_m2_basis_ofe_ground,
                        enthalpy_j_m2_basis_ofe_ground: row.enthalpy_j_m2_basis_ofe_ground,
                    })?;
            }
        }
        let sum_routes = |kind: Option<crate::direct_runtime::DirectSurfaceLiquidParcelKind>,
                          disposition: Option<
            crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition,
        >| {
            surface_flow_by_route
                .iter()
                .filter(|(route, _)| {
                    kind.is_none_or(|value| route.kind == value)
                        && disposition.is_none_or(|value| route.disposition == value)
                })
                .fold(
                    SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
                    |mut total, (_, value)| {
                        total.mass_kg_m2_basis_ofe_ground += value.mass_kg_m2_basis_ofe_ground;
                        total.enthalpy_j_m2_basis_ofe_ground +=
                            value.enthalpy_j_m2_basis_ofe_ground;
                        total
                    },
                )
        };
        let routed_runoff = sum_routes(
            None,
            Some(crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff),
        );
        let upstream_runon = sum_routes(
            Some(crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon),
            None,
        );
        let outlet_runoff = sum_routes(
            None,
            Some(crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::OutletRunoff),
        );
        SnowStage3V11QualificationDayDeltaV1 {
            schema_version: STAGE3_V11_QUALIFICATION_ACCUMULATOR_SCHEMA_V1,
            day_index,
            total_parent_support_count: receipt.support_count as u64,
            adaptive_support_receipt_count: receipt.adaptive_support_receipts.len() as u64,
            snow_free_successor_receipt_count: receipt.snow_free_successor_receipts.len() as u64,
            snow_free_parent_support_count,
            terminal_event_count: receipt.terminal_events.len() as u64,
            publication_event_count: events.len() as u64,
            accepted_publication_support_count: accepted_support_receipt_sha256s.len() as u64,
            ammonium_resource_use_n,
            nitrate_resource_use_n,
            material_transfers,
            accepted_support_receipt_sha256s,
            surface_receipt_occurrences,
            event_receipt_sha256s: events.iter().map(|event| event.id().digest()).collect(),
            surface_flow_by_route,
            routed_runoff,
            upstream_runon,
            outlet_runoff,
            beginning_owner,
            ending_owner,
            receipt_sha256: Digest32::zero(),
        }
        .seal()
    }

    pub(crate) fn production_qualification_snapshot(
        &self,
    ) -> Result<SnowStage3V11ProductionQualificationSnapshotV1, DirectSnowStage3V11AttachmentError>
    {
        if self.archived_receipt_prefix.archived_day_count > 0
            && self.committed.receipt_chain.is_empty()
        {
            return self.production_qualification_snapshot_from_accumulator_v1();
        }
        if self.pending_candidate.is_some()
            || self.pending_publication_day.is_some()
            || self.in_progress_execution.is_some()
            || self.committed.receipt_chain.is_empty()
        {
            return Err(qualification_error(
                "qualification requires a complete committed day",
            ));
        }
        let next_day_index = self.committed.real_consumer.v11_next_day_index();
        if next_day_index != self.committed.receipt_chain.len() {
            return Err(qualification_error("qualification committed chronology"));
        }
        let owner_bytes = self.committed.real_consumer.canonical_owner_state_bytes()?;
        let last = self
            .committed
            .receipt_chain
            .last()
            .ok_or_else(|| qualification_error("qualification missing receipt"))?;
        last.validate_against_ending(&self.committed)?;
        let last_join = &last
            .coupled_subslabs
            .last()
            .ok_or_else(|| qualification_error("qualification missing owner join"))?
            .owner_join;
        let predecessor = SnowStage3V11QualifiedOwnerPredecessorV1 {
            source: SnowStage3V11QualificationPredecessorSourceV1::FinalPositiveSupportOwnerJoin,
            support_end_ns: last_join.support.end_ns().get(),
            owner_join_receipt_sha256: last_join.receipt_sha256,
            ending_complete_owner_set_sha256: last_join.ending_complete_owner_set_sha256,
            soil_thermal_owner_sha256: last_join.soil_thermal_owner_sha256,
            biogeochemistry_owner_sha256: last_join.biogeochemistry_owner_sha256,
        };
        let ending_owner = qualification_ending_owner_authority(
            owner_bytes,
            self.committed.coupled_clock.owners(),
            &last.complete_owner_bytes,
            last.ending_coupled_owner_set_sha256,
            last.ending_coupled_accepted_until_ns.get(),
            predecessor,
        )?;
        let owner_bytes = &last.complete_owner_bytes;
        let soil_bytes = owner_bytes
            .get("soil_thermal")
            .ok_or_else(|| qualification_error("qualification soil-thermal owner"))?;
        let bgc_bytes = owner_bytes
            .get("bgc")
            .ok_or_else(|| qualification_error("qualification biogeochemistry owner"))?;
        let soil_digest = digest_bytes(soil_bytes);
        let bgc_digest = digest_bytes(bgc_bytes);

        let mut accepted_publication_support_count = 0;
        let mut adaptive_support_receipt_count = 0;
        let mut snow_free_successor_receipt_count = 0;
        let mut snow_free_parent_support_count = 0;
        let mut terminal_event_count = 0;
        let mut total_parent_support_count = 0;
        let mut accepted_support_receipt_sha256s = Vec::new();
        let mut surface_receipts = Vec::new();
        let mut ammonium_use = 0.0;
        let mut nitrate_use = 0.0;
        let mut receiver_carbon = 0.0;
        let mut receiver_nitrogen = 0.0;
        let mut receiver_dry_matter = 0.0;

        for (day_index, receipt) in self.committed.receipt_chain.iter().enumerate() {
            let execution_parent_count = receipt
                .adaptive_support_receipts
                .iter()
                .map(|value| value.parent_transaction_id)
                .chain(
                    receipt
                        .snow_free_successor_receipts
                        .iter()
                        .map(|value| value.parent_transaction_id),
                )
                .collect::<BTreeSet<_>>()
                .len();
            if receipt.day_index != day_index
                || receipt.support_count != STAGE3_V11_PARENT_SUPPORT_COUNT
                || execution_parent_count != STAGE3_V11_PARENT_SUPPORT_COUNT
            {
                return Err(qualification_error("qualification partial committed day"));
            }
            receipt.validate_adaptive_publication_crossjoin_v1(&self.committed.real_consumer)?;
            qualification_add(
                &mut total_parent_support_count,
                receipt.support_count as u64,
            )?;
            qualification_add(
                &mut adaptive_support_receipt_count,
                receipt.adaptive_support_receipts.len() as u64,
            )?;
            qualification_add(
                &mut snow_free_successor_receipt_count,
                receipt.snow_free_successor_receipts.len() as u64,
            )?;
            let adaptive_parent_ids = receipt
                .adaptive_support_receipts
                .iter()
                .map(|value| value.parent_transaction_id)
                .collect::<BTreeSet<_>>();
            qualification_add(
                &mut snow_free_parent_support_count,
                receipt
                    .snow_free_successor_receipts
                    .iter()
                    .map(|value| value.parent_transaction_id)
                    .filter(|parent| !adaptive_parent_ids.contains(parent))
                    .collect::<BTreeSet<_>>()
                    .len() as u64,
            )?;
            qualification_add(
                &mut terminal_event_count,
                receipt.terminal_events.len() as u64,
            )?;
            for adaptive in &receipt.adaptive_support_receipts {
                adaptive.validate()?;
            }
            let supports = self
                .committed
                .real_consumer
                .accepted_publication_supports_for_day(day_index)?;
            let day_start = (day_index as u128)
                .checked_mul(STAGE3_V11_DAY_NS)
                .ok_or_else(|| qualification_error("qualification day start overflow"))?;
            let day_end = day_start
                .checked_add(STAGE3_V11_DAY_NS)
                .ok_or_else(|| qualification_error("qualification day end overflow"))?;
            if supports.first().map(|s| s.support().start_ns().get()) != Some(day_start)
                || supports.last().map(|s| s.support().end_ns().get()) != Some(day_end)
            {
                return Err(qualification_error(
                    "qualification accepted support day coverage",
                ));
            }
            qualification_add(
                &mut accepted_publication_support_count,
                u64::try_from(supports.len()).map_err(|_| {
                    qualification_error("qualification publication support count width")
                })?,
            )?;
            for support in supports {
                accepted_support_receipt_sha256s.push(support.receipt_sha256());
                for debit in support.resource_debits() {
                    if let openwepp_vegetation::v11::V11ResourceKey::MineralNitrogen(key) =
                        &debit.resource_key
                    {
                        match key.species {
                            openwepp_kernel_contract::MineralNitrogenSpecies::Ammonium => {
                                ammonium_use += debit.final_use
                            }
                            openwepp_kernel_contract::MineralNitrogenSpecies::Nitrate => {
                                nitrate_use += debit.final_use
                            }
                        }
                    }
                }
                for transfer in support.material_transfers() {
                    receiver_carbon += transfer.carbon;
                    receiver_nitrogen += transfer.nitrogen;
                    receiver_dry_matter += transfer.dry_matter;
                }
                for row in support.ingress_receipts() {
                    if row.disposition == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
                        || row.disposition == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::OutletRunoff
                        || row.kind == crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon
                    {
                        surface_receipts.push(SnowStage3V11QualifiedSurfaceReceiptV1 {
                            day_index,
                            interval_index: support.interval_index(),
                            accepted_support_receipt_sha256: support.receipt_sha256(),
                            source_ofe_id: row.origin_store_key.ofe_id.as_str().to_owned(),
                            destination_ofe_id: qualification_destination(row),
                            kind: row.kind,
                            disposition: row.disposition,
                            mass_kg_m2_basis_ofe_ground: row.mass_kg_m2_basis_ofe_ground,
                            enthalpy_j_m2_basis_ofe_ground: row.enthalpy_j_m2_basis_ofe_ground,
                        });
                    }
                }
            }
        }

        let frame = self.committed.real_consumer.hydrology_frame();
        let mut lanes = Vec::with_capacity(self.committed.stage3_by_lane.len());
        for (lane_id, state) in &self.committed.stage3_by_lane {
            let lane = frame
                .lanes
                .iter()
                .find(|lane| lane.lane_id == *lane_id)
                .ok_or_else(|| qualification_error("qualification hydrology lane"))?;
            lanes.push(SnowStage3V11QualifiedLaneV1 {
                lane_id: *lane_id,
                next_interval_index: state.next_interval_index,
                represented_ice_kg_m2: crate::hydrology::stage3_total_represented_ice_swe_m(state)
                    * 1_000.0,
                detached_retained_liquid_kg_m2: state.detached_retained_liquid_kg_m2,
                cumulative_snowfall_kg_m2: state.cumulative_snowfall_kg_m2,
                cumulative_external_liquid_kg_m2: state.cumulative_external_liquid_kg_m2,
                cumulative_deposition_kg_m2: state.cumulative_deposition_kg_m2,
                cumulative_sublimation_kg_m2: state.cumulative_sublimation_kg_m2,
                cumulative_melt_kg_m2: state.cumulative_melt_kg_m2,
                frost_depth_m: lane.winter_column.frost.dfrost_m,
                thaw_depth_m: lane.winter_column.frost.dthaw_m,
                frozen_water_m: lane
                    .subsurface_layers
                    .iter()
                    .map(|layer| layer.frozen_water_m)
                    .sum(),
            });
        }
        let soil = self.committed.real_consumer.qualification_soil_thermal();
        soil.validate()
            .map_err(|_| qualification_error("qualification soil-thermal validation"))?;
        let soil_thermal_ofes = soil
            .ordered_ofes()
            .into_iter()
            .map(|ofe| SnowStage3V11QualifiedSoilThermalOfeV1 {
                ofe_id: ofe.ofe_id().as_str().to_owned(),
                ordered_layers: ofe
                    .ordered_layers()
                    .into_iter()
                    .map(|layer| SnowStage3V11QualifiedSoilThermalLayerV1 {
                        layer_id: layer.layer_id().as_str().to_owned(),
                        temperature_k: layer.temperature_k(),
                        enthalpy_j_m2: layer.enthalpy_high_j_m2_ofe_ground(),
                    })
                    .collect(),
            })
            .collect();
        let bgc = self.committed.real_consumer.qualification_biogeochemistry();
        let ending = SnowStage3V11QualifiedBgcInventoryV1 {
            ammonium_n: bgc.layers.values().map(|layer| layer.ammonium_n).sum(),
            nitrate_n: bgc.layers.values().map(|layer| layer.nitrate_n).sum(),
            receiver_carbon: bgc.receivers.values().map(|pool| pool.carbon).sum(),
            receiver_nitrogen: bgc.receivers.values().map(|pool| pool.nitrogen).sum(),
            receiver_dry_matter: bgc.receivers.values().map(|pool| pool.dry_matter).sum(),
        };
        let beginning = SnowStage3V11QualifiedBgcInventoryV1 {
            ammonium_n: ending.ammonium_n + ammonium_use,
            nitrate_n: ending.nitrate_n + nitrate_use,
            receiver_carbon: ending.receiver_carbon - receiver_carbon,
            receiver_nitrogen: ending.receiver_nitrogen - receiver_nitrogen,
            receiver_dry_matter: ending.receiver_dry_matter - receiver_dry_matter,
        };
        let delta = SnowStage3V11QualifiedBgcInventoryV1 {
            ammonium_n: ending.ammonium_n - beginning.ammonium_n,
            nitrate_n: ending.nitrate_n - beginning.nitrate_n,
            receiver_carbon: ending.receiver_carbon - beginning.receiver_carbon,
            receiver_nitrogen: ending.receiver_nitrogen - beginning.receiver_nitrogen,
            receiver_dry_matter: ending.receiver_dry_matter - beginning.receiver_dry_matter,
        };
        let sum = |kind: Option<crate::direct_runtime::DirectSurfaceLiquidParcelKind>,
                   disposition: Option<
            crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition,
        >| {
            surface_receipts
                .iter()
                .filter(|row| {
                    kind.map_or(true, |k| row.kind == k)
                        && disposition.map_or(true, |d| row.disposition == d)
                })
                .fold((0.0, 0.0), |acc, row| {
                    (
                        acc.0 + row.mass_kg_m2_basis_ofe_ground,
                        acc.1 + row.enthalpy_j_m2_basis_ofe_ground,
                    )
                })
        };
        let routed = sum(
            None,
            Some(crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff),
        );
        let runon = sum(
            Some(crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon),
            None,
        );
        let outlet = sum(
            None,
            Some(crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::OutletRunoff),
        );
        let accepted_support_root = qualification_ordered_root(
            "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_ACCEPTED_SUPPORT_ROOT_V1",
            &accepted_support_receipt_sha256s,
        )?;
        let surface_receipt_sha256s = surface_receipts
            .iter()
            .map(|row| {
                qualification_digest_serialized(
                    b"OPENWEPP_SNOW_STAGE3_V11_QUALIFIED_SURFACE_RECEIPT_PROJECTION_V1\0",
                    row,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let surface_receipt_root = qualification_ordered_root(
            "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_SURFACE_RECEIPT_ROOT_V1",
            &surface_receipt_sha256s,
        )?;
        let event_receipt_sha256s = self
            .committed
            .real_consumer
            .accepted_publication_event_handoffs()
            .iter()
            .map(|event| event.id().digest())
            .collect::<Vec<_>>();
        let event_root = qualification_ordered_root(
            "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_EVENT_ROOT_V1",
            &event_receipt_sha256s,
        )?;
        let mut snapshot = SnowStage3V11ProductionQualificationSnapshotV1 {
            schema_version: STAGE3_V11_PRODUCTION_QUALIFICATION_SCHEMA_V1,
            next_day_index,
            committed_day_count: self.committed.receipt_chain.len(),
            total_parent_support_count,
            adaptive_support_receipt_count,
            snow_free_successor_receipt_count,
            snow_free_parent_support_count,
            terminal_event_count,
            accepted_publication_support_count,
            accepted_support_receipt_root_sha256: accepted_support_root.ordered_root_sha256,
            surface_receipt_count: surface_receipt_root.record_count,
            surface_receipt_root_sha256: surface_receipt_root.ordered_root_sha256,
            publication_event_count: event_root.record_count,
            publication_event_root_sha256: event_root.ordered_root_sha256,
            lanes,
            ending_owner,
            soil_thermal_owner_sha256: soil_digest,
            soil_thermal_ofes,
            biogeochemistry_owner_sha256: bgc_digest,
            beginning_biogeochemistry: beginning,
            ending_biogeochemistry: ending,
            biogeochemistry_delta: delta,
            ending_biogeochemistry_last_transaction_id: bgc.last_transaction_id,
            accepted_support_receipt_sha256s,
            surface_receipts,
            surface_flow_by_route: BTreeMap::new(),
            routed_runoff_mass_kg_m2: routed.0,
            routed_runoff_enthalpy_j_m2: routed.1,
            upstream_runon_mass_kg_m2: runon.0,
            upstream_runon_enthalpy_j_m2: runon.1,
            outlet_runoff_mass_kg_m2: outlet.0,
            outlet_runoff_enthalpy_j_m2: outlet.1,
            receipt_sha256: Digest32::zero(),
        };
        snapshot.receipt_sha256 = snapshot.reconstructed_digest()?;
        snapshot.validate()?;
        Ok(snapshot)
    }

    fn production_qualification_snapshot_from_accumulator_v1(
        &self,
    ) -> Result<SnowStage3V11ProductionQualificationSnapshotV1, DirectSnowStage3V11AttachmentError>
    {
        if self.pending_candidate.is_some()
            || self.pending_publication_day.is_some()
            || self.pending_committed_day_evidence.is_some()
            || self.in_progress_execution.is_some()
            || !self.committed.receipt_chain.is_empty()
        {
            return Err(qualification_error(
                "qualification bounded snapshot committed posture",
            ));
        }
        let prefix = &self.archived_receipt_prefix;
        prefix.validate()?;
        let accumulator = &prefix.qualification_accumulator;
        accumulator.validate()?;
        let accumulated_ending = accumulator
            .ending_owner
            .as_ref()
            .ok_or_else(|| qualification_error("qualification bounded ending endpoint"))?;
        let mut owner_bytes = self.committed.real_consumer.canonical_owner_state_bytes()?;
        let coupled_owner_set_sha256 =
            complete_owner_set_digest(self.committed.coupled_clock.owners())?;
        bind_parent_receipt_snow_owner_bytes_v1(
            &mut owner_bytes,
            self.committed.coupled_clock.owners(),
        )?;
        let complete_owner_sha256 = qualification_complete_owner_sha256(&owner_bytes)?;
        if coupled_owner_set_sha256 != accumulated_ending.coupled_owner_set_sha256
            || Some(complete_owner_sha256) != accumulated_ending.complete_owner_sha256
            || self.committed.coupled_clock.accepted_until().get()
                != accumulated_ending.accepted_until_ns
        {
            return Err(qualification_error(
                "qualification bounded final owner substitution",
            ));
        }
        let owner_sha256_by_id = owner_bytes
            .iter()
            .map(|(owner_id, bytes)| (owner_id.clone(), digest_bytes(bytes)))
            .collect::<BTreeMap<_, _>>();
        let soil_digest = owner_sha256_by_id
            .get("soil_thermal")
            .copied()
            .ok_or_else(|| qualification_error("qualification bounded soil owner"))?;
        let bgc_digest = owner_sha256_by_id
            .get("bgc")
            .copied()
            .ok_or_else(|| qualification_error("qualification bounded BGC owner"))?;
        if Some(soil_digest) != accumulated_ending.soil_thermal_owner_sha256
            || Some(bgc_digest) != accumulated_ending.biogeochemistry_owner_sha256
        {
            return Err(qualification_error(
                "qualification bounded typed owner endpoint",
            ));
        }
        let ending_owner = SnowStage3V11QualifiedEndingOwnerV1 {
            source: SnowStage3V11QualificationEndingOwnerSourceV1::CommittedPostReceiverPhysicalOwnerWithCoupledSnow,
            complete_owner_sha256,
            coupled_owner_set_sha256,
            accepted_until_ns: accumulated_ending.accepted_until_ns,
            owner_sha256_by_id,
            predecessor: SnowStage3V11QualifiedOwnerPredecessorV1 {
                source: SnowStage3V11QualificationPredecessorSourceV1::FinalPositiveSupportOwnerJoin,
                support_end_ns: prefix.accepted_until_ns,
                owner_join_receipt_sha256: prefix.last_parent_receipt_sha256,
                ending_complete_owner_set_sha256: prefix
                    .ending_owner_set_sha256
                    .ok_or_else(|| qualification_error("qualification bounded prefix owner"))?,
                soil_thermal_owner_sha256: soil_digest,
                biogeochemistry_owner_sha256: bgc_digest,
            },
        };

        let frame = self.committed.real_consumer.hydrology_frame();
        let mut lanes = Vec::with_capacity(self.committed.stage3_by_lane.len());
        for (lane_id, state) in &self.committed.stage3_by_lane {
            let lane = frame
                .lanes
                .iter()
                .find(|lane| lane.lane_id == *lane_id)
                .ok_or_else(|| qualification_error("qualification bounded hydrology lane"))?;
            lanes.push(SnowStage3V11QualifiedLaneV1 {
                lane_id: *lane_id,
                next_interval_index: state.next_interval_index,
                represented_ice_kg_m2: crate::hydrology::stage3_total_represented_ice_swe_m(state)
                    * 1_000.0,
                detached_retained_liquid_kg_m2: state.detached_retained_liquid_kg_m2,
                cumulative_snowfall_kg_m2: state.cumulative_snowfall_kg_m2,
                cumulative_external_liquid_kg_m2: state.cumulative_external_liquid_kg_m2,
                cumulative_deposition_kg_m2: state.cumulative_deposition_kg_m2,
                cumulative_sublimation_kg_m2: state.cumulative_sublimation_kg_m2,
                cumulative_melt_kg_m2: state.cumulative_melt_kg_m2,
                frost_depth_m: lane.winter_column.frost.dfrost_m,
                thaw_depth_m: lane.winter_column.frost.dthaw_m,
                frozen_water_m: lane
                    .subsurface_layers
                    .iter()
                    .map(|layer| layer.frozen_water_m)
                    .sum(),
            });
        }
        let soil = self.committed.real_consumer.qualification_soil_thermal();
        soil.validate()
            .map_err(|_| qualification_error("qualification bounded soil validation"))?;
        let soil_thermal_ofes = soil
            .ordered_ofes()
            .into_iter()
            .map(|ofe| SnowStage3V11QualifiedSoilThermalOfeV1 {
                ofe_id: ofe.ofe_id().as_str().to_owned(),
                ordered_layers: ofe
                    .ordered_layers()
                    .into_iter()
                    .map(|layer| SnowStage3V11QualifiedSoilThermalLayerV1 {
                        layer_id: layer.layer_id().as_str().to_owned(),
                        temperature_k: layer.temperature_k(),
                        enthalpy_j_m2: layer.enthalpy_high_j_m2_ofe_ground(),
                    })
                    .collect(),
            })
            .collect();
        let bgc = self.committed.real_consumer.qualification_biogeochemistry();
        let ending_biogeochemistry = SnowStage3V11QualifiedBgcInventoryV1 {
            ammonium_n: bgc.layers.values().map(|layer| layer.ammonium_n).sum(),
            nitrate_n: bgc.layers.values().map(|layer| layer.nitrate_n).sum(),
            receiver_carbon: bgc.receivers.values().map(|pool| pool.carbon).sum(),
            receiver_nitrogen: bgc.receivers.values().map(|pool| pool.nitrogen).sum(),
            receiver_dry_matter: bgc.receivers.values().map(|pool| pool.dry_matter).sum(),
        };
        let beginning_biogeochemistry = SnowStage3V11QualifiedBgcInventoryV1 {
            ammonium_n: ending_biogeochemistry.ammonium_n + accumulator.ammonium_resource_use_n,
            nitrate_n: ending_biogeochemistry.nitrate_n + accumulator.nitrate_resource_use_n,
            receiver_carbon: ending_biogeochemistry.receiver_carbon
                - accumulator.material_transfers.receiver_carbon,
            receiver_nitrogen: ending_biogeochemistry.receiver_nitrogen
                - accumulator.material_transfers.receiver_nitrogen,
            receiver_dry_matter: ending_biogeochemistry.receiver_dry_matter
                - accumulator.material_transfers.receiver_dry_matter,
        };
        let biogeochemistry_delta = SnowStage3V11QualifiedBgcInventoryV1 {
            ammonium_n: ending_biogeochemistry.ammonium_n - beginning_biogeochemistry.ammonium_n,
            nitrate_n: ending_biogeochemistry.nitrate_n - beginning_biogeochemistry.nitrate_n,
            receiver_carbon: ending_biogeochemistry.receiver_carbon
                - beginning_biogeochemistry.receiver_carbon,
            receiver_nitrogen: ending_biogeochemistry.receiver_nitrogen
                - beginning_biogeochemistry.receiver_nitrogen,
            receiver_dry_matter: ending_biogeochemistry.receiver_dry_matter
                - beginning_biogeochemistry.receiver_dry_matter,
        };
        let mut snapshot = SnowStage3V11ProductionQualificationSnapshotV1 {
            schema_version: STAGE3_V11_PRODUCTION_QUALIFICATION_SCHEMA_V1,
            next_day_index: accumulator.committed_day_count,
            committed_day_count: accumulator.committed_day_count,
            total_parent_support_count: accumulator.total_parent_support_count,
            adaptive_support_receipt_count: accumulator.adaptive_support_receipt_count,
            snow_free_successor_receipt_count: accumulator.snow_free_successor_receipt_count,
            snow_free_parent_support_count: accumulator.snow_free_parent_support_count,
            terminal_event_count: accumulator.terminal_event_count,
            accepted_publication_support_count: accumulator.accepted_publication_support_count,
            accepted_support_receipt_root_sha256: accumulator
                .accepted_support_receipts
                .ordered_root_sha256,
            surface_receipt_count: accumulator.surface_receipts.record_count,
            surface_receipt_root_sha256: accumulator.surface_receipts.ordered_root_sha256,
            publication_event_count: accumulator.events.record_count,
            publication_event_root_sha256: accumulator.events.ordered_root_sha256,
            lanes,
            ending_owner,
            soil_thermal_owner_sha256: soil_digest,
            soil_thermal_ofes,
            biogeochemistry_owner_sha256: bgc_digest,
            beginning_biogeochemistry,
            ending_biogeochemistry,
            biogeochemistry_delta,
            ending_biogeochemistry_last_transaction_id: bgc.last_transaction_id,
            accepted_support_receipt_sha256s: Vec::new(),
            surface_receipts: Vec::new(),
            surface_flow_by_route: accumulator.surface_flow_by_route.clone(),
            routed_runoff_mass_kg_m2: accumulator.routed_runoff.mass_kg_m2_basis_ofe_ground,
            routed_runoff_enthalpy_j_m2: accumulator.routed_runoff.enthalpy_j_m2_basis_ofe_ground,
            upstream_runon_mass_kg_m2: accumulator.upstream_runon.mass_kg_m2_basis_ofe_ground,
            upstream_runon_enthalpy_j_m2: accumulator.upstream_runon.enthalpy_j_m2_basis_ofe_ground,
            outlet_runoff_mass_kg_m2: accumulator.outlet_runoff.mass_kg_m2_basis_ofe_ground,
            outlet_runoff_enthalpy_j_m2: accumulator.outlet_runoff.enthalpy_j_m2_basis_ofe_ground,
            receipt_sha256: Digest32::zero(),
        };
        snapshot.receipt_sha256 = snapshot.reconstructed_digest()?;
        snapshot.validate()?;
        Ok(snapshot)
    }
}

#[cfg(test)]
mod production_qualification_tests {
    use super::*;

    fn digest(byte: u8) -> Digest32 {
        Digest32::from_bytes([byte; 32])
    }

    fn valid_snapshot() -> SnowStage3V11ProductionQualificationSnapshotV1 {
        let accepted_support_receipt_sha256s = (1_u8..=48).map(digest).collect::<Vec<_>>();
        let accepted_root = qualification_ordered_root(
            "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_ACCEPTED_SUPPORT_ROOT_V1",
            &accepted_support_receipt_sha256s,
        )
        .expect("accepted root");
        let inventory = SnowStage3V11QualifiedBgcInventoryV1 {
            ammonium_n: 1.0,
            nitrate_n: 2.0,
            receiver_carbon: 3.0,
            receiver_nitrogen: 4.0,
            receiver_dry_matter: 5.0,
        };
        let mut value = SnowStage3V11ProductionQualificationSnapshotV1 {
            schema_version: STAGE3_V11_PRODUCTION_QUALIFICATION_SCHEMA_V1,
            next_day_index: 1,
            committed_day_count: 1,
            total_parent_support_count: 48,
            adaptive_support_receipt_count: 48,
            snow_free_successor_receipt_count: 0,
            snow_free_parent_support_count: 0,
            terminal_event_count: 0,
            accepted_publication_support_count: 48,
            accepted_support_receipt_root_sha256: accepted_root.ordered_root_sha256,
            surface_receipt_count: 0,
            surface_receipt_root_sha256: Digest32::zero(),
            publication_event_count: 0,
            publication_event_root_sha256: Digest32::zero(),
            lanes: vec![SnowStage3V11QualifiedLaneV1 {
                lane_id: 1,
                next_interval_index: 48,
                represented_ice_kg_m2: 1.0,
                detached_retained_liquid_kg_m2: 0.0,
                cumulative_snowfall_kg_m2: 1.0,
                cumulative_external_liquid_kg_m2: 0.0,
                cumulative_deposition_kg_m2: 0.0,
                cumulative_sublimation_kg_m2: 0.0,
                cumulative_melt_kg_m2: 0.0,
                frost_depth_m: 0.1,
                thaw_depth_m: 0.0,
                frozen_water_m: 0.01,
            }],
            ending_owner: SnowStage3V11QualifiedEndingOwnerV1 {
                source: SnowStage3V11QualificationEndingOwnerSourceV1::CommittedPostReceiverPhysicalOwnerWithCoupledSnow,
                complete_owner_sha256: digest(49),
                coupled_owner_set_sha256: digest(48),
                accepted_until_ns: STAGE3_V11_DAY_NS,
                owner_sha256_by_id: BTreeMap::from([
                    ("bgc".to_owned(), digest(51)),
                    ("snow".to_owned(), digest(52)),
                    ("soil_thermal".to_owned(), digest(50)),
                ]),
                predecessor: SnowStage3V11QualifiedOwnerPredecessorV1 {
                    source:
                        SnowStage3V11QualificationPredecessorSourceV1::FinalPositiveSupportOwnerJoin,
                    support_end_ns: STAGE3_V11_DAY_NS,
                    owner_join_receipt_sha256: digest(47),
                    ending_complete_owner_set_sha256: digest(46),
                    soil_thermal_owner_sha256: digest(45),
                    biogeochemistry_owner_sha256: digest(44),
                },
            },
            soil_thermal_owner_sha256: digest(50),
            soil_thermal_ofes: vec![],
            biogeochemistry_owner_sha256: digest(51),
            beginning_biogeochemistry: inventory,
            ending_biogeochemistry: inventory,
            biogeochemistry_delta: SnowStage3V11QualifiedBgcInventoryV1::default(),
            ending_biogeochemistry_last_transaction_id: 1,
            accepted_support_receipt_sha256s,
            surface_receipts: vec![],
            surface_flow_by_route: BTreeMap::new(),
            routed_runoff_mass_kg_m2: 0.0,
            routed_runoff_enthalpy_j_m2: 0.0,
            upstream_runon_mass_kg_m2: 0.0,
            upstream_runon_enthalpy_j_m2: 0.0,
            outlet_runoff_mass_kg_m2: 0.0,
            outlet_runoff_enthalpy_j_m2: 0.0,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest().expect("seal fixture");
        value
    }

    #[test]
    fn qualification_owner_substitution_poison_fails_closed() {
        let mut value = valid_snapshot();
        value.biogeochemistry_owner_sha256 = digest(99);
        assert!(value.validate().is_err());
    }

    fn ending_owner_fixture(
        bgc: &'static [u8],
        hydrology: &'static [u8],
        snow: &'static [u8],
    ) -> BTreeMap<String, Vec<u8>> {
        BTreeMap::from([
            ("bgc".to_owned(), bgc.to_vec()),
            ("hydrology".to_owned(), hydrology.to_vec()),
            ("snow".to_owned(), snow.to_vec()),
        ])
    }

    fn predecessor_fixture() -> SnowStage3V11QualifiedOwnerPredecessorV1 {
        SnowStage3V11QualifiedOwnerPredecessorV1 {
            source: SnowStage3V11QualificationPredecessorSourceV1::FinalPositiveSupportOwnerJoin,
            support_end_ns: 20,
            owner_join_receipt_sha256: digest(61),
            ending_complete_owner_set_sha256: digest(62),
            soil_thermal_owner_sha256: digest(63),
            biogeochemistry_owner_sha256: digest(64),
        }
    }

    #[test]
    fn qualification_post_receiver_physical_owner_with_coupled_snow_is_authoritative() {
        let coupled = [openwepp_coupled_time::OwnerState::new(
            "snow".to_owned(),
            b"coupled-stage3-snow".to_vec(),
        )
        .expect("coupled snow")];
        let physical = ending_owner_fixture(
            b"post-receiver-bgc",
            b"physical-hydrology",
            b"stale-projection",
        );
        let expected = ending_owner_fixture(
            b"post-receiver-bgc",
            b"physical-hydrology",
            b"coupled-stage3-snow",
        );
        let authority = qualification_ending_owner_authority(
            physical,
            &coupled,
            &expected,
            complete_owner_set_digest(&coupled).expect("coupled digest"),
            20,
            predecessor_fixture(),
        )
        .expect("post-receiver physical owner");
        assert_eq!(
            authority.source,
            SnowStage3V11QualificationEndingOwnerSourceV1::CommittedPostReceiverPhysicalOwnerWithCoupledSnow,
        );
        assert_eq!(
            authority.owner_sha256_by_id.get("snow"),
            Some(&digest_bytes(b"coupled-stage3-snow")),
        );
    }

    #[test]
    fn qualification_pre_receiver_owner_substitution_poison_fails_closed() {
        let coupled = [openwepp_coupled_time::OwnerState::new(
            "snow".to_owned(),
            b"coupled-stage3-snow".to_vec(),
        )
        .expect("coupled snow")];
        let pre_receiver = ending_owner_fixture(
            b"pre-receiver-bgc",
            b"physical-hydrology",
            b"stale-projection",
        );
        let expected = ending_owner_fixture(
            b"post-receiver-bgc",
            b"physical-hydrology",
            b"coupled-stage3-snow",
        );
        assert!(
            qualification_ending_owner_authority(
                pre_receiver,
                &coupled,
                &expected,
                complete_owner_set_digest(&coupled).expect("coupled digest"),
                20,
                predecessor_fixture(),
            )
            .is_err(),
        );
    }

    #[test]
    fn qualification_derived_publication_frame_substitution_poison_fails_closed() {
        let coupled = [openwepp_coupled_time::OwnerState::new(
            "snow".to_owned(),
            b"coupled-stage3-snow".to_vec(),
        )
        .expect("coupled snow")];
        let derived = ending_owner_fixture(
            b"post-receiver-bgc",
            b"derived-publication-hydrology",
            b"stale-projection",
        );
        let expected = ending_owner_fixture(
            b"post-receiver-bgc",
            b"physical-hydrology",
            b"coupled-stage3-snow",
        );
        assert!(
            qualification_ending_owner_authority(
                derived,
                &coupled,
                &expected,
                complete_owner_set_digest(&coupled).expect("coupled digest"),
                20,
                predecessor_fixture(),
            )
            .is_err(),
        );
    }

    #[test]
    fn qualification_ending_owner_source_substitution_poison_fails_closed() {
        for source in [
            SnowStage3V11QualificationEndingOwnerSourceV1::PreReceiverPhysicalOwner,
            SnowStage3V11QualificationEndingOwnerSourceV1::DerivedPublicationFrame,
        ] {
            let mut value = valid_snapshot();
            value.ending_owner.source = source;
            value.receipt_sha256 = value.reconstructed_digest().expect("reseal poison");
            assert!(value.validate().is_err());
        }
    }

    #[test]
    fn qualification_finalization_skew_keeps_subslab_join_as_predecessor_evidence() {
        let mut value = valid_snapshot();
        assert_ne!(
            value.ending_owner.predecessor.soil_thermal_owner_sha256,
            value.soil_thermal_owner_sha256,
        );
        assert_ne!(
            value.ending_owner.predecessor.biogeochemistry_owner_sha256,
            value.biogeochemistry_owner_sha256,
        );
        value.receipt_sha256 = value.reconstructed_digest().expect("reseal fixture");
        value.validate().expect("final endpoint is authoritative");
    }

    #[test]
    fn qualification_predecessor_substitution_and_order_poisons_fail_closed() {
        let mut substituted = valid_snapshot();
        substituted.ending_owner.predecessor.source =
            SnowStage3V11QualificationPredecessorSourceV1::SubstitutedOwnerJoin;
        substituted.receipt_sha256 = substituted
            .reconstructed_digest()
            .expect("reseal substituted predecessor");
        assert!(substituted.validate().is_err());

        let mut reordered = valid_snapshot();
        reordered.ending_owner.predecessor.support_end_ns =
            reordered.ending_owner.accepted_until_ns + 1;
        reordered.receipt_sha256 = reordered
            .reconstructed_digest()
            .expect("reseal reordered predecessor");
        assert!(reordered.validate().is_err());
    }

    #[test]
    fn qualification_trial_receipt_visibility_poison_fails_closed() {
        let mut value = valid_snapshot();
        value
            .surface_receipts
            .push(SnowStage3V11QualifiedSurfaceReceiptV1 {
                day_index: 0,
                interval_index: 0,
                accepted_support_receipt_sha256: digest(99),
                source_ofe_id: "ofe-1".to_owned(),
                destination_ofe_id: "ofe-2".to_owned(),
                kind: crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon,
                disposition:
                    crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
                mass_kg_m2_basis_ofe_ground: 1.0,
                enthalpy_j_m2_basis_ofe_ground: 2.0,
            });
        value.receipt_sha256 = value.reconstructed_digest().expect("reseal poison");
        assert!(value.validate().is_err());
    }

    #[test]
    fn qualification_partial_day_poison_fails_closed() {
        let mut value = valid_snapshot();
        value.total_parent_support_count = 47;
        value.receipt_sha256 = value.reconstructed_digest().expect("reseal poison");
        assert!(value.validate().is_err());
    }

    #[test]
    fn qualification_publication_receipt_count_poison_fails_closed() {
        let mut value = valid_snapshot();
        value.accepted_publication_support_count = 49;
        value.receipt_sha256 = value.reconstructed_digest().expect("reseal poison");
        assert!(value.validate().is_err());
    }

    #[test]
    fn qualification_publication_receipt_vector_is_the_count_authority() {
        let mut value = valid_snapshot();
        value.accepted_publication_support_count = 2;
        value.accepted_support_receipt_sha256s = vec![digest(1), digest(2)];
        value.accepted_support_receipt_root_sha256 = qualification_ordered_root(
            "OPENWEPP_SNOW_STAGE3_V11_QUALIFICATION_ACCEPTED_SUPPORT_ROOT_V1",
            &value.accepted_support_receipt_sha256s,
        )
        .expect("accepted root")
        .ordered_root_sha256;
        value.receipt_sha256 = value.reconstructed_digest().expect("reseal fixture");
        value
            .validate()
            .expect("two authoritative publication receipts bind count and root");
    }

    #[test]
    fn qualification_publication_receipt_omission_fails_closed() {
        let mut omitted_child = valid_snapshot();
        omitted_child.accepted_support_receipt_sha256s.pop();
        omitted_child.receipt_sha256 = omitted_child
            .reconstructed_digest()
            .expect("reseal omitted publication receipt");
        assert!(omitted_child.validate().is_err());
    }

    #[test]
    fn qualification_serialized_surfaces_omit_microstep_diagnostics() {
        let snapshot = valid_snapshot();
        let day = qualification_day_delta(0, qualification_endpoint(0, 201, false), 1);
        let accumulator = SnowStage3V11QualificationAccumulatorV1::reconstruct_from_days([&day])
            .expect("one-day accumulator");
        const DIAGNOSTIC_KEYS: [&str; 11] = [
            "direct_trial_count",
            "split_child_trial_count",
            "accepted_microstep_count",
            "accepted_floor_microstep_count",
            "accepted_composed_microstep_count",
            "rejected_candidate_count",
            "phase_refinement_count",
            "event_refinement_count",
            "minimum_accepted_step_ns",
            "maximum_accepted_step_ns",
            "owner_evaluation_counts",
        ];
        for (surface, value) in [
            ("day delta", serde_json::to_value(&day).expect("day JSON")),
            (
                "accumulator",
                serde_json::to_value(&accumulator).expect("accumulator JSON"),
            ),
            (
                "snapshot",
                serde_json::to_value(&snapshot).expect("snapshot JSON"),
            ),
        ] {
            let object = value.as_object().expect("serialized object");
            for key in DIAGNOSTIC_KEYS {
                assert!(
                    !object.contains_key(key),
                    "{surface} persisted diagnostic key {key}",
                );
            }
        }

        let mut day_poison = serde_json::to_value(day).expect("day poison JSON");
        day_poison["direct_trial_count"] = serde_json::json!(1);
        assert!(
            serde_json::from_value::<SnowStage3V11QualificationDayDeltaV1>(day_poison).is_err()
        );
        let mut accumulator_poison =
            serde_json::to_value(accumulator).expect("accumulator poison JSON");
        accumulator_poison["rejected_candidate_count"] = serde_json::json!(1);
        assert!(
            serde_json::from_value::<SnowStage3V11QualificationAccumulatorV1>(accumulator_poison,)
                .is_err()
        );
        let mut snapshot_poison = serde_json::to_value(snapshot).expect("snapshot poison JSON");
        snapshot_poison["owner_evaluation_counts"] = serde_json::json!({"snow": 1});
        assert!(
            serde_json::from_value::<SnowStage3V11ProductionQualificationSnapshotV1>(
                snapshot_poison,
            )
            .is_err()
        );
    }

    fn qualification_adaptive_support(
        start_ns: u128,
        end_ns: u128,
        effective_ending_owner: u8,
    ) -> QualificationAdaptivePublicationSupportV1 {
        qualification_adaptive_support_with_posture(
            start_ns,
            end_ns,
            effective_ending_owner,
            Stage3AdaptiveEventPostureV1::NoEvent,
        )
    }

    fn qualification_adaptive_support_with_posture(
        start_ns: u128,
        end_ns: u128,
        effective_ending_owner: u8,
        event_posture: Stage3AdaptiveEventPostureV1,
    ) -> QualificationAdaptivePublicationSupportV1 {
        QualificationAdaptivePublicationSupportV1 {
            parent_transaction_sha256: digest(90),
            support: TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(end_ns))
                .expect("positive support"),
            effective_ending_complete_owner_set_sha256: digest(effective_ending_owner),
            event_posture,
        }
    }

    fn qualification_subslab(
        start_ns: u128,
        end_ns: u128,
        physical_ending_owner: u8,
        effective_ending_owner: u8,
    ) -> QualificationRetainedSubslabV1 {
        qualification_subslab_with_terminal(
            start_ns,
            end_ns,
            physical_ending_owner,
            effective_ending_owner,
            false,
        )
    }

    fn qualification_subslab_with_terminal(
        start_ns: u128,
        end_ns: u128,
        physical_ending_owner: u8,
        effective_ending_owner: u8,
        terminal_event_at_support_end: bool,
    ) -> QualificationRetainedSubslabV1 {
        QualificationRetainedSubslabV1 {
            parent_transaction_sha256: digest(90),
            support: TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(end_ns))
                .expect("positive support"),
            physical_ending_complete_owner_set_sha256: digest(physical_ending_owner),
            effective_ending_complete_owner_set_sha256: digest(effective_ending_owner),
            terminal_event_at_support_end,
        }
    }

    fn qualification_snow_free_successor(
        start_ns: u128,
        end_ns: u128,
    ) -> QualificationRetainedSnowFreeSuccessorV1 {
        QualificationRetainedSnowFreeSuccessorV1 {
            parent_transaction_sha256: digest(90),
            support: TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(end_ns))
                .expect("positive support"),
        }
    }

    fn qualification_retained_publication(
        start_ns: u128,
        end_ns: u128,
        physical_ending_owner: u8,
        effective_ending_owner: u8,
    ) -> QualificationRetainedPublicationSupportV1 {
        QualificationRetainedPublicationSupportV1 {
            support: TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(end_ns))
                .expect("positive support"),
            physical_ending_complete_owner_set_sha256: digest(physical_ending_owner),
            ordered_owner_chain_sha256s: if physical_ending_owner == effective_ending_owner {
                vec![digest(physical_ending_owner)]
            } else {
                vec![
                    digest(physical_ending_owner),
                    digest(effective_ending_owner),
                ]
            },
        }
    }

    include!("snow_stage3_v11_qualification_crossjoin_child_tests.rs");

    fn qualification_endpoint(
        accepted_until_ns: u128,
        identity: u8,
        complete: bool,
    ) -> SnowStage3V11QualificationOwnerEndpointV1 {
        SnowStage3V11QualificationOwnerEndpointV1 {
            complete_owner_sha256: complete.then(|| digest(identity)),
            coupled_owner_set_sha256: digest(identity),
            accepted_until_ns,
            soil_thermal_owner_sha256: complete.then(|| digest(identity.wrapping_add(1))),
            biogeochemistry_owner_sha256: complete.then(|| digest(identity.wrapping_add(2))),
        }
    }

    fn qualification_real_pre_support_event(
        ending_state_byte: u8,
    ) -> (Digest32, Digest32, AcceptedEventReceiptV1) {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(STAGE3_V11_DAY_NS))
            .expect("day support");
        let beginning_owner = OwnerState::new("snow".to_owned(), vec![1]).expect("beginning owner");
        let ending_owner =
            OwnerState::new("snow".to_owned(), vec![ending_state_byte]).expect("ending owner");
        let beginning_digest = complete_owner_set_digest(std::slice::from_ref(&beginning_owner))
            .expect("beginning owner digest");
        let ending_digest = complete_owner_set_digest(std::slice::from_ref(&ending_owner))
            .expect("ending owner digest");
        let authority = ParentAuthorityV1::new(
            digest(71),
            digest(72),
            digest(73),
            0,
            support,
            beginning_digest,
        )
        .expect("parent authority");
        let mut clock = CoupledClockStateV1::new(
            authority,
            vec![beginning_owner],
            "snow-covered".to_owned(),
            vec!["snow".to_owned()],
            digest(74),
            Vec::new(),
        )
        .expect("coupled clock");
        let proposal = EventProposalV1::new(
            EventClass::OwnershipTransfer,
            "snow".to_owned(),
            digest(75),
            vec![ending_owner],
            vec!["snow".to_owned()],
            "snow-covered".to_owned(),
            vec!["snow".to_owned()],
            vec![
                LedgerEntryV1::new(
                    "qualification-owner-transfer".to_owned(),
                    "identity".to_owned(),
                    digest(76),
                    digest(76),
                    digest(77),
                )
                .expect("event ledger"),
            ],
        )
        .expect("pre-support event proposal");
        let mut queue = EventQueueV1::new(ModelTimeNs::new(0), vec![proposal])
            .expect("pre-support event queue");
        let receipt = queue
            .apply_next(&mut clock)
            .expect("apply pre-support event")
            .expect("accepted pre-support event");
        (beginning_digest, ending_digest, receipt)
    }

    #[test]
    fn qualification_daily_beginning_traverses_real_pre_support_event() {
        let (publication_beginning, support_beginning, event) =
            qualification_real_pre_support_event(2);
        validate_qualification_beginning_event_bridge(
            publication_beginning,
            support_beginning,
            0,
            &[event],
        )
        .expect("sealed day-start event bridges publication to first support");
    }

    #[test]
    fn qualification_daily_beginning_event_omission_and_substitution_fail_closed() {
        let (publication_beginning, support_beginning, event) =
            qualification_real_pre_support_event(2);
        assert!(
            validate_qualification_beginning_event_bridge(
                publication_beginning,
                support_beginning,
                0,
                &[],
            )
            .is_err(),
            "omitted pre-support event cannot bridge the execution owner",
        );
        let (_, _, substituted) = qualification_real_pre_support_event(3);
        assert!(
            validate_qualification_beginning_event_bridge(
                publication_beginning,
                support_beginning,
                0,
                std::slice::from_ref(&substituted),
            )
            .is_err(),
            "a sealed event with substituted ending custody cannot bridge the first support",
        );
        assert_ne!(event.id(), substituted.id());
    }

    #[test]
    fn qualification_daily_endpoint_uses_committed_publication_event_tail() {
        let beginning = qualification_endpoint(0, 10, false);
        let ending = qualification_endpoint(STAGE3_V11_DAY_NS, 12, true);
        let last_support_pre_event_owner = digest(11);
        assert_ne!(
            last_support_pre_event_owner, ending.coupled_owner_set_sha256,
            "day-end event handoff advances custody after the last support"
        );
        validate_qualification_day_endpoint_join(
            &beginning,
            &ending,
            beginning.coupled_owner_set_sha256,
            ending.coupled_owner_set_sha256,
            0,
            STAGE3_V11_DAY_NS,
            0,
            STAGE3_V11_DAY_NS,
        )
        .expect("committed publication event-tail owner is the ending authority");
    }

    #[test]
    fn qualification_daily_endpoint_event_tail_substitution_fails_closed() {
        let beginning = qualification_endpoint(0, 10, false);
        let ending = qualification_endpoint(STAGE3_V11_DAY_NS, 12, true);
        assert!(
            validate_qualification_day_endpoint_join(
                &beginning,
                &ending,
                beginning.coupled_owner_set_sha256,
                digest(11),
                0,
                STAGE3_V11_DAY_NS,
                0,
                STAGE3_V11_DAY_NS,
            )
            .is_err(),
            "pre-event last-support owner cannot replace the committed publication tail",
        );
    }

    fn qualification_day_delta(
        day_index: usize,
        beginning_owner: SnowStage3V11QualificationOwnerEndpointV1,
        ending_identity: u8,
    ) -> SnowStage3V11QualificationDayDeltaV1 {
        let ending_owner = qualification_endpoint(
            ((day_index + 1) as u128) * STAGE3_V11_DAY_NS,
            ending_identity,
            true,
        );
        SnowStage3V11QualificationDayDeltaV1 {
            schema_version: STAGE3_V11_QUALIFICATION_ACCUMULATOR_SCHEMA_V1,
            day_index,
            total_parent_support_count: 48,
            adaptive_support_receipt_count: 48,
            snow_free_successor_receipt_count: 0,
            snow_free_parent_support_count: 0,
            terminal_event_count: 0,
            publication_event_count: 0,
            accepted_publication_support_count: 48,
            ammonium_resource_use_n: 0.01,
            nitrate_resource_use_n: 0.02,
            material_transfers: SnowStage3V11QualifiedBgcInventoryV1 {
                receiver_carbon: 0.03,
                receiver_nitrogen: 0.04,
                receiver_dry_matter: 0.05,
                ..SnowStage3V11QualifiedBgcInventoryV1::default()
            },
            accepted_support_receipt_sha256s: (1_u8..=48)
                .map(|ordinal| digest(ordinal.wrapping_add(ending_identity)))
                .collect(),
            surface_receipt_occurrences: Vec::new(),
            event_receipt_sha256s: Vec::new(),
            surface_flow_by_route: BTreeMap::new(),
            routed_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            upstream_runon: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            outlet_runoff: SnowStage3V11QualifiedMassEnthalpyTotalV1::default(),
            beginning_owner,
            ending_owner,
            receipt_sha256: Digest32::zero(),
        }
        .seal()
        .expect("seal daily delta")
    }

    include!("snow_stage3_v11_production_qualification_record_identity_tests.rs");
    include!("snow_stage3_v11_production_qualification_route_wire_tests.rs");
    include!("snow_stage3_v11_production_qualification_accumulator_tests.rs");
    include!("snow_stage3_v11_qualification_crossjoin_tests.rs");
}
