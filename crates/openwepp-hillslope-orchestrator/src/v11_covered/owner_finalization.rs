//! Final owner construction and V11 resource-lineage joins.

use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredParentOwnerJoinReceiptV1 {
    pub run_identity: Digest32,
    pub parent_interval_sha256: Digest32,
    pub parent_transaction_sha256: Digest32,
    pub segment_sha256: Digest32,
    pub accepted_slab_sha256: Digest32,
    pub forcing_receipt_sha256: Digest32,
    pub beginning_complete_owner_set_sha256: Digest32,
    pub ending_complete_owner_set_sha256: Digest32,
    pub support: openwepp_coupled_time::TimeSupport,
    pub final_boundary_receipt_set_sha256: Digest32,
    pub final_lane_boundary_receipt_set_sha256: Digest32,
    pub component_carrier_receipt_set_sha256: Digest32,
    pub wb14_child_receipt_set_sha256: Digest32,
    pub wb14_parent_receipt_set_sha256: Option<Digest32>,
    pub stage3_physical_state_sha256: Digest32,
    pub vegetation_owner_sha256: Digest32,
    pub snow_owner_sha256: Digest32,
    pub land_surface_energy_owner_sha256: Digest32,
    pub hydrology_owner_sha256: Digest32,
    pub biogeochemistry_owner_sha256: Digest32,
    pub soil_thermal_owner_sha256: Digest32,
    pub surface_liquid_owner_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl CoveredParentOwnerJoinReceiptV1 {
    pub(crate) fn validate_retained_boundary_sets(
        &self,
        final_boundaries: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        final_lane_boundaries: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    ) -> Result<(), DirectV11RealConsumerError> {
        for (destination, receipt) in final_boundaries {
            receipt.validate()?;
            if destination != receipt.destination() {
                return Err(DirectV11RealConsumerError::Identity("covered boundary map key"));
            }
        }
        for (lane_id, receipt) in final_lane_boundaries {
            receipt.validate()?;
            if *lane_id != receipt.lane_id {
                return Err(DirectV11RealConsumerError::Identity("covered lane map key"));
            }
        }
        let boundary_fields = final_boundaries.values().map(|receipt| {
            openwepp_coupled_time::FramedField {
                tag: "final_boundary_receipt",
                value: receipt.receipt_sha256().as_bytes(),
            }
        }).collect::<Vec<_>>();
        let lane_fields = final_lane_boundaries.values().map(|receipt| {
            openwepp_coupled_time::FramedField {
                tag: "final_lane_boundary_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            }
        }).collect::<Vec<_>>();
        if openwepp_coupled_time::framed_sha256(
            "covered-stage3-final-boundary-set-v1", &boundary_fields,
        ).map_err(|_| DirectV11RealConsumerError::Identity("covered boundary receipt set"))?
            != self.final_boundary_receipt_set_sha256
            || openwepp_coupled_time::framed_sha256(
                "covered-stage3-final-lane-boundary-set-v1", &lane_fields,
            ).map_err(|_| DirectV11RealConsumerError::Identity("covered lane receipt set"))?
                != self.final_lane_boundary_receipt_set_sha256
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered retained boundary receipt set",
            ));
        }
        Ok(())
    }

    pub fn validate_seal(&self) -> Result<(), DirectV11RealConsumerError> {
        if self.reconstructed_digest()? != self.receipt_sha256 {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner join seal",
            ));
        }
        Ok(())
    }

    pub(crate) fn try_new(
        run_identity: Digest32,
        parent_interval_sha256: Digest32,
        parent_transaction_sha256: Digest32,
        segment_sha256: Digest32,
        accepted_slab_sha256: Digest32,
        forcing_receipt_sha256: Digest32,
        beginning_complete_owner_set_sha256: Digest32,
        wb14_child_receipt_set_sha256: Digest32,
        wb14_parent_receipt_set_sha256: Option<Digest32>,
        support: openwepp_coupled_time::TimeSupport,
        final_boundaries: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        final_lane_boundaries: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
        component_carriers: &BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        owners: &BTreeMap<String, V11OwnerEnvelope>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        if [
            run_identity,
            parent_interval_sha256,
            parent_transaction_sha256,
            segment_sha256,
            accepted_slab_sha256,
            forcing_receipt_sha256,
            beginning_complete_owner_set_sha256,
            wb14_child_receipt_set_sha256,
        ]
        .contains(&Digest32::zero())
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner transaction lineage",
            ));
        }
        let expected = [
            "vegetation",
            "snow",
            "land_surface_energy",
            "hydrology",
            "bgc",
            "soil_thermal",
            "surface_liquid",
        ];
        if owners.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != expected.into_iter().collect::<BTreeSet<_>>()
            || final_boundaries.is_empty()
            || final_lane_boundaries.is_empty()
            || component_carriers.keys().collect::<BTreeSet<_>>()
                != final_boundaries
                    .iter()
                    .filter_map(|(destination, receipt)| {
                        matches!(receipt, FinalStage3TileBoundaryReceiptV1::V11Canopy(_))
                            .then_some(destination)
                    })
                    .collect::<BTreeSet<_>>()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner join topology",
            ));
        }
        for owner in owners.values() {
            owner.to_owner_state().map_err(|_| {
                DirectV11RealConsumerError::Identity("covered parent-owner envelope")
            })?;
        }
        let ending_owner_states = owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| DirectV11RealConsumerError::Identity("covered ending owner set"))?;
        let ending_complete_owner_set_sha256 =
            openwepp_coupled_time::complete_owner_set_digest(&ending_owner_states).map_err(|_| {
                DirectV11RealConsumerError::Identity("covered ending complete-owner digest")
            })?;
        for (destination, receipt) in final_boundaries {
            receipt.validate()?;
            if destination != receipt.destination() {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered final boundary map key",
                ));
            }
        }
        if !final_lane_boundaries
            .keys()
            .all(|lane_id| stage3_states.contains_key(lane_id))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "active lane receipt outside Stage-3 state set",
            ));
        }
        let mut joined_destinations = BTreeSet::new();
        for (lane_id, receipt) in final_lane_boundaries {
            receipt.validate()?;
            if *lane_id != receipt.lane_id {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered lane receipt map key",
                ));
            }
            for contribution in &receipt.ordered_destinations {
                let destination = (receipt.ofe_id.clone(), contribution.tile_id.clone());
                if !joined_destinations.insert(destination.clone()) {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered lane duplicate destination join",
                    ));
                }
                let boundary = final_boundaries.get(&destination).ok_or(
                    DirectV11RealConsumerError::Identity("covered lane/final boundary destination"),
                )?;
                let sources = boundary.source_digests();
                let physical = boundary.physical_operands();
                let expected_class = match boundary {
                    FinalStage3TileBoundaryReceiptV1::V11Canopy(_) => {
                        Stage3TileBoundaryClassV1::V11CanopyCovered
                    }
                    FinalStage3TileBoundaryReceiptV1::OpenSnow(_) => {
                        Stage3TileBoundaryClassV1::OpenSnow
                    }
                };
                if boundary.destination() != &destination
                    || contribution.boundary_class != expected_class
                    || contribution.final_boundary_receipt_sha256 != sources.3
                    || contribution.provisional_carrier_receipt_sha256 != sources.0
                    || contribution.optical_receipt_sha256 != sources.1
                    || contribution.reciprocal_longwave_receipt_sha256 != sources.2
                    || contribution.beginning_stage3_state_sha256
                        != boundary.beginning_stage3_state_sha256()
                    || contribution.sensible_to_canopy_air_w_m2.to_bits() != physical[0].to_bits()
                    || contribution.vapor_to_canopy_air_kg_m2_s.to_bits() != physical[1].to_bits()
                    || contribution.latent_energy_to_canopy_air_j_m2.to_bits()
                        != physical[2].to_bits()
                    || contribution.snow_absorbed_shortwave_w_m2.to_bits() != physical[3].to_bits()
                    || contribution.snow_net_longwave_w_m2.to_bits() != physical[4].to_bits()
                    || contribution.snow_temperature_k.to_bits() != physical[5].to_bits()
                    || contribution.latent_heat_j_kg.to_bits() != physical[6].to_bits()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "covered lane/final boundary semantic join",
                    ));
                }
            }
        }
        if joined_destinations != final_boundaries.keys().cloned().collect::<BTreeSet<_>>() {
            return Err(DirectV11RealConsumerError::Identity(
                "covered lane complete destination join",
            ));
        }
        for (destination, receipt) in component_carriers {
            let boundary =
                final_boundaries
                    .get(destination)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "component/final boundary destination",
                    ))?;
            let FinalStage3TileBoundaryReceiptV1::V11Canopy(boundary) = boundary else {
                return Err(DirectV11RealConsumerError::Identity(
                    "component carrier attached to open-snow boundary",
                ));
            };
            receipt.validate(boundary)?;
        }
        let expected_snow_bytes = canonical_stage3_snow_owner_bytes_v11_with_receipts(
            stage3_states,
            final_lane_boundaries,
            final_boundaries,
        )?;
        let snow_owner = owners
            .get("snow")
            .ok_or(DirectV11RealConsumerError::Identity(
                "covered parent-owner snow envelope",
            ))?;
        validate_exact_snow_owner_bytes(&expected_snow_bytes, snow_owner)?;
        let boundary_fields = final_boundaries
            .values()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "final_boundary_receipt",
                value: receipt.receipt_sha256().as_bytes(),
            })
            .collect::<Vec<_>>();
        let final_boundary_receipt_set_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-stage3-final-boundary-set-v1",
            &boundary_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered boundary receipt set"))?;
        let component_fields = component_carriers
            .values()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "component_carrier_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            })
            .collect::<Vec<_>>();
        let component_carrier_receipt_set_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-component-carrier-set-v1",
            &component_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("component carrier receipt set"))?;
        let lane_fields = final_lane_boundaries
            .values()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "final_lane_boundary_receipt",
                value: receipt.receipt_sha256.as_bytes(),
            })
            .collect::<Vec<_>>();
        let final_lane_boundary_receipt_set_sha256 = openwepp_coupled_time::framed_sha256(
            "covered-stage3-final-lane-boundary-set-v1",
            &lane_fields,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered lane receipt set"))?;
        let stage3_physical_state_sha256 =
            digest_bytes(&canonical_stage3_snow_owner_bytes_v11(stage3_states)?);
        let owner_digest = |name: &'static str| {
            owners.get(name).map(|owner| owner.state_sha256).ok_or(
                DirectV11RealConsumerError::Identity("covered parent-owner join owner"),
            )
        };
        let mut value = Self {
            run_identity,
            parent_interval_sha256,
            parent_transaction_sha256,
            segment_sha256,
            accepted_slab_sha256,
            forcing_receipt_sha256,
            beginning_complete_owner_set_sha256,
            ending_complete_owner_set_sha256,
            support,
            final_boundary_receipt_set_sha256,
            final_lane_boundary_receipt_set_sha256,
            component_carrier_receipt_set_sha256,
            wb14_child_receipt_set_sha256,
            wb14_parent_receipt_set_sha256,
            stage3_physical_state_sha256,
            vegetation_owner_sha256: owner_digest("vegetation")?,
            snow_owner_sha256: owner_digest("snow")?,
            land_surface_energy_owner_sha256: owner_digest("land_surface_energy")?,
            hydrology_owner_sha256: owner_digest("hydrology")?,
            biogeochemistry_owner_sha256: owner_digest("bgc")?,
            soil_thermal_owner_sha256: owner_digest("soil_thermal")?,
            surface_liquid_owner_sha256: owner_digest("surface_liquid")?,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        Ok(value)
    }

    pub(crate) fn validate(
        &self,
        final_boundaries: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
        final_lane_boundaries: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
        component_carriers: &BTreeMap<(OfeId, TileId), ComponentResolvedCarrierReceiptV1>,
        stage3_states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        owners: &BTreeMap<String, V11OwnerEnvelope>,
    ) -> Result<(), DirectV11RealConsumerError> {
        let expected = Self::try_new(
            self.run_identity,
            self.parent_interval_sha256,
            self.parent_transaction_sha256,
            self.segment_sha256,
            self.accepted_slab_sha256,
            self.forcing_receipt_sha256,
            self.beginning_complete_owner_set_sha256,
            self.wb14_child_receipt_set_sha256,
            self.wb14_parent_receipt_set_sha256,
            self.support,
            final_boundaries,
            final_lane_boundaries,
            component_carriers,
            stage3_states,
            owners,
        )?;
        if &expected != self {
            return Err(DirectV11RealConsumerError::Identity(
                "covered parent-owner join replay",
            ));
        }
        self.validate_seal()?;
        Ok(())
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectV11RealConsumerError> {
        let start = self.support.start_ns().get().to_be_bytes();
        let end = self.support.end_ns().get().to_be_bytes();
        openwepp_coupled_time::framed_sha256(
            "covered-parent-owner-join-v1",
            &[
                openwepp_coupled_time::FramedField {
                    tag: "run_identity",
                    value: self.run_identity.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "parent_interval",
                    value: self.parent_interval_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "parent_transaction",
                    value: self.parent_transaction_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "segment",
                    value: self.segment_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "accepted_slab",
                    value: self.accepted_slab_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "forcing_receipt",
                    value: self.forcing_receipt_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "beginning_complete_owner_set",
                    value: self.beginning_complete_owner_set_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "ending_complete_owner_set",
                    value: self.ending_complete_owner_set_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "support_start_ns",
                    value: &start,
                },
                openwepp_coupled_time::FramedField {
                    tag: "support_end_ns",
                    value: &end,
                },
                openwepp_coupled_time::FramedField {
                    tag: "final_boundary_set",
                    value: self.final_boundary_receipt_set_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "final_lane_boundary_set",
                    value: self.final_lane_boundary_receipt_set_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "component_carrier_set",
                    value: self.component_carrier_receipt_set_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "wb14_child_receipt_set",
                    value: self.wb14_child_receipt_set_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "wb14_parent_receipt_set",
                    value: self
                        .wb14_parent_receipt_set_sha256
                        .unwrap_or(Digest32::zero())
                        .as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "stage3_physical_state",
                    value: self.stage3_physical_state_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "vegetation_owner",
                    value: self.vegetation_owner_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "snow_owner",
                    value: self.snow_owner_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "land_surface_energy_owner",
                    value: self.land_surface_energy_owner_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "hydrology_owner",
                    value: self.hydrology_owner_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "biogeochemistry_owner",
                    value: self.biogeochemistry_owner_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "soil_thermal_owner",
                    value: self.soil_thermal_owner_sha256.as_bytes(),
                },
                openwepp_coupled_time::FramedField {
                    tag: "surface_liquid_owner",
                    value: self.surface_liquid_owner_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("covered parent-owner join digest"))
    }
}

fn validate_exact_snow_owner_bytes(
    expected: &[u8],
    snow_owner: &V11OwnerEnvelope,
) -> Result<(), DirectV11RealConsumerError> {
    snow_owner
        .to_owner_state()
        .map_err(|_| DirectV11RealConsumerError::Identity("covered snow owner envelope"))?;
    if snow_owner.owner_id != "snow" || snow_owner.state_bytes != expected {
        return Err(DirectV11RealConsumerError::Identity(
            "covered parent-owner Stage-3/snow semantic join",
        ));
    }
    Ok(())
}

pub(crate) fn canonical_stage3_snow_owner_bytes_v11(
    states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<Vec<u8>, DirectV11RealConsumerError> {
    #[derive(Serialize)]
    struct CanonicalSnowOwner<'a> {
        schema: &'static str,
        lanes: Vec<(&'a u32, &'a DirectSnowStage3PersistentState)>,
    }
    serde_json::to_vec(&CanonicalSnowOwner {
        schema: "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V1",
        lanes: states.iter().collect(),
    })
    .map_err(|_| DirectV11RealConsumerError::Identity("canonical Stage-3 snow bytes"))
}

pub(crate) fn canonical_stage3_snow_owner_bytes_v11_with_receipts(
    states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    lane_receipts: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    receipts: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
) -> Result<Vec<u8>, DirectV11RealConsumerError> {
    #[derive(Serialize)]
    struct CanonicalSnowOwner<'a> {
        schema: &'static str,
        lanes: Vec<(&'a u32, &'a DirectSnowStage3PersistentState)>,
        final_lane_boundary_receipts: BTreeMap<u32, String>,
        final_boundary_receipts: BTreeMap<String, String>,
    }
    let final_boundary_receipts = receipts
        .iter()
        .map(|(destination, receipt)| {
            (
                format!("{}\0{}", destination.0.as_str(), destination.1.as_str()),
                digest32_hex(receipt.source_digests().3),
            )
        })
        .collect();
    let final_lane_boundary_receipts = lane_receipts
        .iter()
        .map(|(lane_id, receipt)| (*lane_id, digest32_hex(receipt.receipt_sha256)))
        .collect();
    serde_json::to_vec(&CanonicalSnowOwner {
        schema: "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V3",
        lanes: states.iter().collect(),
        final_lane_boundary_receipts,
        final_boundary_receipts,
    })
    .map_err(|_| DirectV11RealConsumerError::Identity("canonical Stage-3 snow bytes"))
}

/// Shared V11 post-boundary transaction assembly. Both lower-boundary
/// adopters use this owner/resource/finalization path; only the envelope
/// construction differs between snow-free and covered segments.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn finalize_v11_imported_segment(
    beginning: &DirectV10RealConsumerShadow,
    input: &V11ImportedV10SegmentInput,
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    ending_snow_owner_bytes: Vec<u8>,
    day_index: usize,
) -> Result<
    (
        V11ImportedV10SegmentOutput,
        DirectV10RealConsumerShadow,
        LseSupportAdmissibilityReceiptV1,
    ),
    DirectV11RealConsumerError,
> {
    let support_receipt = LseSupportAdmissibilityReceiptV1::admit(
        &beginning.inner.lse_configuration,
        &beginning.inner.lse_state,
        digest32_hex(input.parent_transaction_id.digest()),
        digest32_hex(input.accepted_slab_receipt.segment_id().digest()),
        digest32_hex(input.accepted_slab_receipt.slab_id().digest()),
        input.accepted_slab_receipt.slab_ordinal(),
        input.support.start_ns().get(),
        input.support.end_ns().get(),
        input.duration_s_bits,
        beginning.inner.soil_thermal.state_sha256.clone(),
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
    })?;
    envelope.validate().map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    let mut resource_debits = v11_nitrogen_resource_debits(envelope, input)?;
    resource_debits.extend(v11_water_resource_debits(
        envelope,
        &input.configuration,
        input,
    )?);

    let mut candidate = beginning.clone();
    candidate
        .inner
        .accept_envelope(envelope.transaction_id(), envelope)
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
        })?;
    candidate.vegetation_state = project_v9_runtime_to_v10(
        candidate.inner.vegetation_state(),
        &candidate.vegetation_configuration,
    )
    .map_err(|error| DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::V10(error)))?;
    candidate.lse_state = project_validated_v1_runtime_to_v2(
        &candidate.inner.lse_configuration,
        candidate.inner.lse_state(),
        &candidate.lse_configuration,
        &openwepp_land_surface_energy::Sha256Digest::try_new(
            candidate
                .vegetation_configuration
                .configuration_sha256
                .clone(),
        )
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
        })?,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LseV2(error))
    })?;

    let segment_ending = candidate.vegetation_state.clone();
    normalize_v11_staged_parent_lineage(&mut candidate, input.beginning.0.last_transaction_id)?;
    let snow = V11OwnerEnvelope::try_new("snow".to_owned(), ending_snow_owner_bytes)?;
    let surface = candidate
        .inner
        .hydrology_frame
        .surface_liquid_shadow
        .as_ref()
        .ok_or(DirectV11RealConsumerError::Identity(
            "missing staged surface-liquid owner",
        ))?;
    let surface_bytes = surface
        .canonical_bytes(&candidate.inner.surface_configuration)
        .map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Serialization(error.to_string()),
            ))
        })?;
    let beginning_hydrology_adapter = RealHydrologyShadowAdapter::try_from_day_start(
        &beginning.inner.hydrology_frame,
        day_index,
        TransactionId(input.beginning.0.last_transaction_id),
        f64::from_bits(input.duration_s_bits),
        candidate.inner.surface_configuration.owner_id.clone(),
        &candidate.inner.layer_maps,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    let hydrology_adapter = RealHydrologyShadowAdapter::try_from_day_start(
        envelope.hydrology().ending_frame(),
        day_index,
        TransactionId(input.beginning.0.last_transaction_id),
        f64::from_bits(input.duration_s_bits),
        candidate.inner.surface_configuration.owner_id.clone(),
        &candidate.inner.layer_maps,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error.into()))
    })?;
    let ending_resource_owners = [
        ("snow".to_owned(), snow),
        (
            "land_surface_energy".to_owned(),
            v11_owner_envelope("land_surface_energy", &candidate.inner.lse_state)?,
        ),
        (
            "surface_liquid".to_owned(),
            V11OwnerEnvelope::try_new("surface_liquid".to_owned(), surface_bytes)?,
        ),
        (
            "hydrology".to_owned(),
            V11OwnerEnvelope::try_new(
                "hydrology".to_owned(),
                hydrology_adapter.snapshot_bytes().to_vec(),
            )?,
        ),
        (
            "bgc".to_owned(),
            v11_owner_envelope("bgc", &candidate.inner.biogeochemistry)?,
        ),
        (
            "soil_thermal".to_owned(),
            v11_owner_envelope("soil_thermal", &candidate.inner.soil_thermal)?,
        ),
    ]
    .into_iter()
    .collect();
    let shared_resource_transitions = v11_shared_resource_transitions(
        envelope,
        input,
        &resource_debits,
        &ending_resource_owners,
        &beginning_hydrology_adapter,
        &hydrology_adapter,
        &beginning.inner.biogeochemistry,
    )?;
    let output = V11ImportedV10SegmentOutput {
        ending: segment_ending,
        lse_support_receipt: V11LseSupportReceiptEnvelope::from_canonical_json(
            serde_json::to_vec(&support_receipt).map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                    DirectV9RealConsumerError::Serialization(error.to_string()),
                ))
            })?,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("V11 LSE support receipt"))?,
        resource_debits,
        admitted_resource_fluxes: Vec::<V11AdmittedResourceFlux>::new(),
        shared_resource_transitions,
        ending_resource_owners,
        material_transfers: envelope.vegetation().material_proposals().to_vec(),
    };
    Ok((output, candidate, support_receipt))
}

pub(crate) fn digest32_hex(value: Digest32) -> String {
    let mut text = String::with_capacity(64);
    for byte in value.as_bytes() {
        write!(&mut text, "{byte:02x}").expect("writing to String cannot fail");
    }
    text
}

pub(crate) fn digest32_from_lower_hex(value: &str) -> Result<Digest32, DirectV11RealConsumerError> {
    if value.len() != 64
        || value
            .bytes()
            .any(|byte| !byte.is_ascii_hexdigit() || byte.is_ascii_uppercase())
    {
        return Err(DirectV11RealConsumerError::Identity(
            "covered receipt digest encoding",
        ));
    }
    let mut bytes = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let high = match pair[0] {
            byte @ b'0'..=b'9' => byte - b'0',
            byte @ b'a'..=b'f' => byte - b'a' + 10,
            _ => {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered receipt digest encoding",
                ));
            }
        };
        let low = match pair[1] {
            byte @ b'0'..=b'9' => byte - b'0',
            byte @ b'a'..=b'f' => byte - b'a' + 10,
            _ => {
                return Err(DirectV11RealConsumerError::Identity(
                    "covered receipt digest encoding",
                ));
            }
        };
        bytes[index] = (high << 4) | low;
    }
    Ok(Digest32::from_bytes(bytes))
}

pub(crate) fn stage3_albedo_state_digest(
    input: &DirectActiveSnowPartitionInputs,
) -> Result<Sha256Digest, DirectV11RealConsumerError> {
    let mut bytes = Vec::with_capacity(128);
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_SNOW_ALBEDO_STATE_V1\0");
    match input.snow_albedo_state {
        Some(state) => {
            bytes.push(1);
            bytes.extend_from_slice(state.model.id().as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&state.albedo.to_bits().to_le_bytes());
            bytes.extend_from_slice(
                &state
                    .accumulated_positive_temperature_c_day
                    .to_bits()
                    .to_le_bytes(),
            );
        }
        None => bytes.push(0),
    }
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| DirectV11RealConsumerError::Identity("Stage-3 albedo digest"))
}

pub(crate) fn stage3_support_forcing_digest(
    support: DirectSnowStage3SupportInput,
) -> Result<Sha256Digest, DirectV11RealConsumerError> {
    let forcing = support.forcing;
    let mut bytes = Vec::with_capacity(256);
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_SUPPORT_FORCING_V1\0");
    for value in [
        forcing.active_precipitation_m,
        forcing.rain_m,
        forcing.snowfall_m,
        forcing.radiation_mj_m2,
        forcing.air_temperature_c,
        forcing.cloud_fraction,
        forcing.rain_fraction,
        forcing.snow_fraction,
        support.duration_seconds,
    ] {
        bytes.extend_from_slice(&value.to_bits().to_le_bytes());
    }
    bytes.extend_from_slice(forcing.phase_model.id().as_bytes());
    bytes.push(0);
    match forcing.hydrometeor_temperature_c {
        Some(value) => {
            bytes.push(1);
            bytes.extend_from_slice(&value.to_bits().to_le_bytes());
        }
        None => bytes.push(0),
    }
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| DirectV11RealConsumerError::Identity("Stage-3 forcing digest"))
}

pub(crate) fn normalize_v11_staged_parent_lineage(
    staged: &mut DirectV10RealConsumerShadow,
    parent: u128,
) -> Result<(), DirectV11RealConsumerError> {
    normalize_v8_parent_lineage(&mut staged.vegetation_state.0, parent);
    normalize_v8_parent_lineage(&mut staged.inner.vegetation_state.0, parent);
    let transaction = (parent != 0).then_some(TransactionId(parent));
    staged.inner.lse_state.last_accepted_transaction_id = transaction;
    staged.inner.lse_state.state_sha256 =
        staged.inner.lse_state.canonical_sha256().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
        })?;
    staged.lse_state.0.last_accepted_transaction_id = transaction;
    staged.lse_state.0.state_sha256 = staged.lse_state.0.canonical_sha256().map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::LandSurface(error))
    })?;
    let surface = staged
        .inner
        .hydrology_frame
        .surface_liquid_shadow
        .as_mut()
        .ok_or(DirectV11RealConsumerError::Identity(
            "missing staged surface-liquid owner",
        ))?;
    if staged.inner.wb14_parent_working_state.is_none() {
        for record in &mut surface.records {
            record.last_accepted_transaction_id = transaction;
        }
        for continuation in &mut surface.continuations {
            continuation.last_accepted_transaction_id = transaction;
        }
        surface.state_sha256 = surface.recomputed_sha256().map_err(|error| {
            DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Serialization(error.to_string()),
            ))
        })?;
    }
    staged.inner.biogeochemistry.last_transaction_id = parent;
    staged.inner.soil_thermal.last_accepted_transaction_id = transaction;
    let soil_transaction = transaction.ok_or(DirectV11RealConsumerError::Identity(
        "zero parent transaction lineage",
    ))?;
    staged.inner.soil_thermal.state_sha256 = digest_soil_state(
        &staged.inner.soil_thermal.owner_id,
        soil_transaction,
        &staged.inner.soil_thermal.ofes,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
    })?;
    staged.inner.soil_thermal.snapshot_sha256 = digest_soil_snapshot(
        &staged.inner.soil_thermal.owner_id,
        &staged.inner.soil_thermal.configuration_sha256,
        &staged.inner.soil_thermal.state_sha256,
        soil_transaction,
        &staged.inner.soil_thermal.ofes,
    )
    .map_err(|error| {
        DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::Runtime(error))
    })?;
    Ok(())
}

pub(crate) fn normalize_v8_parent_lineage(
    state: &mut openwepp_vegetation::V8CoupledOwnedState,
    parent: u128,
) {
    state.last_transaction_id = parent;
    for stratum in state.strata.values_mut() {
        stratum.last_transaction_id = parent;
    }
    let accepted = (parent != 0).then_some(parent);
    for occupancy in state.occupancies.values_mut() {
        occupancy.last_accepted_transaction_id = accepted;
    }
    state.state_sha256 = state.canonical_sha256();
}

pub(crate) fn v11_nitrogen_resource_debits(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
) -> Result<Vec<V11ResourceDebit>, DirectV11RealConsumerError> {
    let (requests, authorizations, uses) = envelope.vegetation().nitrogen_protocol();
    if validate_v11_nitrogen_protocol_cardinality(
        requests.len(),
        authorizations.len(),
        uses.len(),
    )? {
        return Ok(Vec::new());
    }
    let occupancies = input.configuration.expected_occupancies();
    uses.iter()
        .map(|used| {
            let request = requests
                .iter()
                .find(|row| row.owner_id == used.owner_id && row.key == used.key)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 nitrogen request binding",
                ))?;
            let authorization = authorizations
                .iter()
                .find(|row| row.owner_id == used.owner_id && row.key == used.key)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 nitrogen authorization binding",
                ))?;
            let occupancy = occupancies
                .iter()
                .find(|id| id.stratum_id.as_str() == used.owner_id.as_str())
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 nitrogen occupancy binding",
                ))?;
            let matching_ofes = envelope
                .hydrology()
                .receiver_closure_operands()
                .lse_tiles
                .iter()
                .filter(|row| row.tile_id == occupancy.tile_id)
                .map(|row| row.ofe_id.as_str())
                .collect::<BTreeSet<_>>();
            let ofe_id = matching_ofes
                .iter()
                .next()
                .filter(|_| matching_ofes.len() == 1)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 BGC occupancy/OFE binding",
                ))?;
            V11ResourceDebit::new(V11ResourceDebit {
                receipt_id: Digest32::zero(),
                parent_transaction_id: input.parent_transaction_id,
                segment_id: input.accepted_slab_receipt.segment_id(),
                accepted_slab_id: input.accepted_slab_receipt.slab_id(),
                support: input.support,
                owner_id: "bgc".into(),
                resource_key: V11ResourceKey::MineralNitrogen(used.key.clone()),
                ofe_id: (*ofe_id).to_owned(),
                tile_id: occupancy.tile_id.as_str().to_owned(),
                occupancy_id: format!(
                    "{}::{}",
                    occupancy.stratum_id.as_str(),
                    occupancy.tile_id.as_str()
                ),
                layer_id: used.key.layer_id.as_str().to_owned(),
                source_id: match used.key.species {
                    MineralNitrogenSpecies::Ammonium => "nh4",
                    MineralNitrogenSpecies::Nitrate => "no3",
                }
                .into(),
                amount_basis: "kg_n_m2".into(),
                request: request.amount,
                authorization: authorization.amount,
                final_use: used.amount,
            })
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 nitrogen debit"))
        })
        .collect()
}

fn validate_v11_nitrogen_protocol_cardinality(
    request_count: usize,
    authorization_count: usize,
    use_count: usize,
) -> Result<bool, DirectV11RealConsumerError> {
    if request_count == 0 && authorization_count == 0 && use_count == 0 {
        return Ok(true);
    }
    if request_count == 0 || authorization_count == 0 || use_count == 0 {
        return Err(DirectV11RealConsumerError::Identity(
            "V11 nitrogen protocol cardinality",
        ));
    }
    Ok(false)
}

#[cfg(test)]
mod nitrogen_protocol_cardinality_tests {
    use super::*;

    #[test]
    fn empty_protocol_is_admissible_and_every_partial_empty_protocol_rejects() {
        assert!(validate_v11_nitrogen_protocol_cardinality(0, 0, 0).expect("empty protocol"));
        for counts in [(0, 0, 1), (0, 1, 0), (1, 0, 0), (0, 1, 1), (1, 0, 1), (1, 1, 0)] {
            assert!(
                validate_v11_nitrogen_protocol_cardinality(counts.0, counts.1, counts.2).is_err(),
                "partial-empty poison {counts:?}",
            );
        }
        assert!(
            !validate_v11_nitrogen_protocol_cardinality(1, 1, 1)
                .expect("nonempty protocol")
        );
    }
}

pub(crate) fn v11_shared_resource_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    owners: &BTreeMap<String, V11OwnerEnvelope>,
    beginning_hydrology: &RealHydrologyShadowAdapter,
    ending_hydrology: &RealHydrologyShadowAdapter,
    beginning_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let hydrology_digest = owners
        .get("hydrology")
        .ok_or(DirectV11RealConsumerError::Identity(
            "V11 hydrology candidate",
        ))?
        .state_sha256;
    let mut rows = v11_water_owner_transitions(
        envelope,
        input,
        debits,
        beginning_hydrology,
        ending_hydrology,
        hydrology_digest,
    )?;
    let bgc_digest = owners
        .get("bgc")
        .ok_or(DirectV11RealConsumerError::Identity("V11 BGC candidate"))?
        .state_sha256;
    let ofe_id = envelope
        .hydrology()
        .receiver_closure_operands()
        .production_soil
        .first()
        .ok_or(DirectV11RealConsumerError::Identity("V11 BGC OFE"))?
        .ofe_id
        .as_str();
    rows.extend(v11_bgc_owner_transitions(
        envelope,
        input,
        debits,
        beginning_bgc,
        ofe_id,
        bgc_digest,
    )?);
    Ok(rows)
}

pub(crate) fn v11_water_owner_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    beginning: &RealHydrologyShadowAdapter,
    ending: &RealHydrologyShadowAdapter,
    owner_digest: Digest32,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let mut rows = Vec::new();
    for (ofe_index, ofe) in envelope
        .hydrology()
        .receiver_closure_operands()
        .production_soil
        .iter()
        .enumerate()
    {
        for layer in &ofe.ordered_layers {
            let key = V11SharedResourceKey {
                resource: V11SharedResourceKind::Water,
                owner_id: "hydrology".into(),
                ofe_id: ofe.ofe_id.as_str().to_owned(),
                layer_id: layer.layer_id.as_str().to_owned(),
                source_id: "soil_water".into(),
                amount_basis: "kg_m2_stand_ground".into(),
            };
            let ids = v11_linked_debit_ids(debits, &key, true);
            if ids.is_empty() {
                continue;
            }
            let amount = |owner: &RealHydrologyShadowAdapter, message| {
                owner
                    .layer_facts()
                    .values()
                    .find(|fact| {
                        fact.source.ofe_lane.lane_index == ofe_index
                            && fact.source.layer_id == layer.layer_id
                    })
                    .map(|fact| fact.liquid_supply_kg_m2)
                    .ok_or(DirectV11RealConsumerError::Identity(message))
            };
            rows.push(v11_shared_transition(
                input,
                key,
                amount(beginning, "V11 beginning hydrology layer binding")?,
                amount(ending, "V11 ending hydrology layer binding")?,
                ids,
                owner_digest,
            )?);
        }
    }
    Ok(rows)
}

pub(crate) fn v11_bgc_owner_transitions(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    input: &V11ImportedV10SegmentInput,
    debits: &[V11ResourceDebit],
    beginning_bgc: &openwepp_biogeochemistry::BiogeochemistryState,
    ofe_id: &str,
    owner_digest: Digest32,
) -> Result<Vec<V11SharedResourceOwnerTransition>, DirectV11RealConsumerError> {
    let mut rows = Vec::new();
    for operand in envelope.biogeochemistry().mineral_operands() {
        let source_id = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => "nh4",
            MineralNitrogenSpecies::Nitrate => "no3",
        };
        let resource = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => V11SharedResourceKind::Ammonium,
            MineralNitrogenSpecies::Nitrate => V11SharedResourceKind::Nitrate,
        };
        let key = V11SharedResourceKey {
            resource,
            owner_id: "bgc".into(),
            ofe_id: ofe_id.to_owned(),
            layer_id: operand.key.layer_id.as_str().to_owned(),
            source_id: source_id.into(),
            amount_basis: "kg_n_m2".into(),
        };
        let ids = v11_linked_debit_ids(debits, &key, false);
        if ids.is_empty() {
            continue;
        }
        let beginning_layer = beginning_bgc
            .layers
            .get(operand.key.layer_id.as_str())
            .ok_or(DirectV11RealConsumerError::Identity(
                "V11 beginning BGC layer binding",
            ))?;
        let beginning_amount = match operand.key.species {
            MineralNitrogenSpecies::Ammonium => beginning_layer.ammonium_n,
            MineralNitrogenSpecies::Nitrate => beginning_layer.nitrate_n,
        };
        rows.push(v11_shared_transition(
            input,
            key,
            beginning_amount,
            operand.ending_kg_n_m2,
            ids,
            owner_digest,
        )?);
    }
    Ok(rows)
}

pub(crate) fn v11_linked_debit_ids(
    debits: &[V11ResourceDebit],
    key: &V11SharedResourceKey,
    bind_amount_basis: bool,
) -> Vec<Digest32> {
    let mut ids = debits
        .iter()
        .filter(|debit| {
            debit.owner_id == key.owner_id
                && debit.ofe_id == key.ofe_id
                && debit.layer_id == key.layer_id
                && debit.source_id == key.source_id
                && (!bind_amount_basis || debit.amount_basis == key.amount_basis)
        })
        .map(|debit| debit.receipt_id)
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids
}

pub(crate) fn v11_shared_transition(
    input: &V11ImportedV10SegmentInput,
    key: V11SharedResourceKey,
    beginning_amount: f64,
    ending_amount: f64,
    debit_receipt_ids: Vec<Digest32>,
    owner_candidate_sha256: Digest32,
) -> Result<V11SharedResourceOwnerTransition, DirectV11RealConsumerError> {
    Ok(V11SharedResourceOwnerTransition::new(
        V11SharedResourceOwnerTransition {
            transition_id: Digest32::zero(),
            parent_transaction_id: input.parent_transaction_id,
            segment_id: input.accepted_slab_receipt.segment_id(),
            accepted_slab_id: input.accepted_slab_receipt.slab_id(),
            support: input.support,
            shared_resource_key: key,
            beginning_amount,
            ending_amount,
            debit_receipt_ids,
            admitted_flux_receipt_ids: Vec::new(),
            owner_candidate_sha256,
        },
    )?)
}

pub(crate) fn v11_water_resource_debits(
    envelope: &UncommittedCoveredV8OwnerEnvelope,
    configuration: &VegetationConfiguration,
    input: &V11ImportedV10SegmentInput,
) -> Result<Vec<V11ResourceDebit>, DirectV11RealConsumerError> {
    let occupancies = configuration
        .expected_occupancies()
        .into_iter()
        .map(|id| {
            (
                format!("{}::{}", id.stratum_id.as_str(), id.tile_id.as_str()),
                id,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let requests = &envelope.hydrology().arbitration().requests;
    let authorizations = &envelope.hydrology().arbitration().authorizations;
    envelope
        .hydrology()
        .finalized_uses()
        .iter()
        .filter_map(|value| {
            let component = value.key.occupancy_id.as_ref()?;
            let layer = value.key.soil_layer_id.as_ref()?;
            Some((value, component.as_str(), layer))
        })
        .map(|(value, component, layer)| {
            let occupancy =
                occupancies
                    .get(component)
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "V11 water occupancy binding",
                    ))?;
            let request = requests.iter().find(|row| row.key == value.key).ok_or(
                DirectV11RealConsumerError::Identity("V11 water request binding"),
            )?;
            let authorization = authorizations
                .iter()
                .find(|row| row.key == value.key)
                .ok_or(DirectV11RealConsumerError::Identity(
                    "V11 water authorization binding",
                ))?;
            V11ResourceDebit::new(V11ResourceDebit {
                receipt_id: Digest32::zero(),
                parent_transaction_id: input.parent_transaction_id,
                segment_id: input.accepted_slab_receipt.segment_id(),
                accepted_slab_id: input.accepted_slab_receipt.slab_id(),
                support: input.support,
                owner_id: "hydrology".to_owned(),
                resource_key: V11ResourceKey::Water(openwepp_kernel_contract::WaterResourceKey {
                    occupancy_id: occupancy.clone(),
                    layer_id: layer.clone(),
                }),
                ofe_id: value.key.ofe_id.as_str().to_owned(),
                tile_id: occupancy.tile_id.as_str().to_owned(),
                occupancy_id: component.to_owned(),
                layer_id: layer.as_str().to_owned(),
                source_id: "soil_water".to_owned(),
                amount_basis: "kg_m2_stand_ground".to_owned(),
                request: request.amount_kg_m2_stand_ground,
                authorization: authorization.amount_kg_m2_stand_ground,
                final_use: value.amount_kg_m2_stand_ground,
            })
            .map_err(|_| DirectV11RealConsumerError::Identity("V11 water debit"))
        })
        .collect()
}

#[cfg(test)]
mod owner_join_tests {
    use super::*;
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    #[test]
    fn final_owner_join_seal_rejects_each_owner_digest_substitution() {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("support");
        let mut receipt = CoveredParentOwnerJoinReceiptV1 {
            run_identity: Digest32::from_bytes([21; 32]),
            parent_interval_sha256: Digest32::from_bytes([20; 32]),
            parent_transaction_sha256: Digest32::from_bytes([22; 32]),
            segment_sha256: Digest32::from_bytes([23; 32]),
            accepted_slab_sha256: Digest32::from_bytes([24; 32]),
            forcing_receipt_sha256: Digest32::from_bytes([25; 32]),
            beginning_complete_owner_set_sha256: Digest32::from_bytes([26; 32]),
            ending_complete_owner_set_sha256: Digest32::from_bytes([27; 32]),
            support,
            final_boundary_receipt_set_sha256: Digest32::from_bytes([1; 32]),
            final_lane_boundary_receipt_set_sha256: Digest32::from_bytes([18; 32]),
            component_carrier_receipt_set_sha256: Digest32::from_bytes([2; 32]),
            wb14_child_receipt_set_sha256: Digest32::from_bytes([29; 32]),
            wb14_parent_receipt_set_sha256: None,
            stage3_physical_state_sha256: Digest32::from_bytes([3; 32]),
            vegetation_owner_sha256: Digest32::from_bytes([4; 32]),
            snow_owner_sha256: Digest32::from_bytes([5; 32]),
            land_surface_energy_owner_sha256: Digest32::from_bytes([6; 32]),
            hydrology_owner_sha256: Digest32::from_bytes([7; 32]),
            biogeochemistry_owner_sha256: Digest32::from_bytes([8; 32]),
            soil_thermal_owner_sha256: Digest32::from_bytes([9; 32]),
            surface_liquid_owner_sha256: Digest32::from_bytes([10; 32]),
            receipt_sha256: Digest32::zero(),
        };
        receipt.receipt_sha256 = receipt.reconstructed_digest().expect("join digest");
        receipt.validate_seal().expect("valid join seal");
        for mutate in [
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.ending_complete_owner_set_sha256 = Digest32::from_bytes([28; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.final_lane_boundary_receipt_set_sha256 = Digest32::from_bytes([19; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.vegetation_owner_sha256 = Digest32::from_bytes([11; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.snow_owner_sha256 = Digest32::from_bytes([12; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.land_surface_energy_owner_sha256 = Digest32::from_bytes([13; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.hydrology_owner_sha256 = Digest32::from_bytes([14; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.biogeochemistry_owner_sha256 = Digest32::from_bytes([15; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.soil_thermal_owner_sha256 = Digest32::from_bytes([16; 32]);
            },
            |value: &mut CoveredParentOwnerJoinReceiptV1| {
                value.surface_liquid_owner_sha256 = Digest32::from_bytes([17; 32]);
            },
        ] {
            let mut poisoned = receipt.clone();
            mutate(&mut poisoned);
            assert!(poisoned.validate_seal().is_err());
        }
    }

    #[test]
    fn valid_alternate_snow_owner_rejects_against_unchanged_physical_bytes() {
        let expected = b"canonical-stage3-and-boundaries";
        let expected_owner =
            V11OwnerEnvelope::try_new("snow".into(), expected.to_vec()).expect("snow owner");
        validate_exact_snow_owner_bytes(expected, &expected_owner).expect("exact snow join");

        let alternate = V11OwnerEnvelope::try_new(
            "snow".into(),
            b"different-valid-canonical-snow-owner".to_vec(),
        )
        .expect("alternate valid snow owner");
        assert!(validate_exact_snow_owner_bytes(expected, &alternate).is_err());
    }
}
