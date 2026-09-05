// Small V10-stack identity and provider-owner accessors used by both the
// persistent day executor and the covered terminal probe. Keeping these
// lifecycle helpers separate prevents the real-consumer implementation from
// becoming the owner of terminal chronology details.

fn ordered_subsequence_is_complete_v1<T: PartialEq>(available: &[T], required: &[T]) -> bool {
    let mut next_required = 0_usize;
    for value in available {
        if required.get(next_required) == Some(value) {
            next_required += 1;
        }
    }
    next_required == required.len()
}

fn genesis_pre_support_event_authority_is_exact_v1(
    event_tick: openwepp_coupled_time::ModelTimeNs,
    event_ordinal: u32,
    event_beginning_owner: Digest32,
    expected_beginning_owner: Digest32,
    following_support: TimeSupport,
) -> bool {
    event_tick == following_support.start_ns()
        && event_ordinal == 0
        && event_beginning_owner == expected_beginning_owner
}

impl DirectV10RealConsumerShadow {
    pub(crate) const fn wb14_parent_working_state_v1(
        &self,
    ) -> Option<&crate::direct_runtime::DirectWb14ParentWorkingState> {
        self.inner.wb14_parent_working_state.as_ref()
    }

    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_accepted_publication_active_tail_canonical_bytes_v3(
        &self,
    ) -> Result<Vec<u8>, DirectV10RealConsumerError> {
        let supports = self
            .accepted_publication_history
            .supports()
            .iter()
            .map(|support| support.to_wire())
            .collect::<Vec<_>>();
        let resident_shared = self
            .accepted_publication_history
            .supports()
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        let events = self.accepted_publication_history.event_handoffs();
        let traversed =
            validate_accepted_publication_authority(&resident_shared, events).map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "restart V3 resident publication authority chronology",
                ))
            })?;
        let authority_bytes = serde_json::to_vec(&(&supports, events)).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                "restart V3 resident publication authority projection".to_owned(),
            ))
        })?;
        let mut preimage = b"OPENWEPP_ACCEPTED_PUBLICATION_AUTHORITY_RESTART_V2\0".to_vec();
        preimage.extend_from_slice(&authority_bytes);
        let wire = Stage3AcceptedPublicationSupportSetWireV2 {
            schema_version: 2,
            supports,
            event_handoffs: events.to_vec(),
            traversed_ending_complete_owner_set_sha256: traversed,
            receipt_sha256: digest_bytes(&preimage),
        };
        serde_json::to_vec(&wire).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                "restart V3 resident publication authority encoding".to_owned(),
            ))
        })
    }

    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_restore_accepted_publication_active_tail_canonical_bytes_v3(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), DirectV10RealConsumerError> {
        self.restart_authority_restore_accepted_publication_supports_canonical_bytes(bytes)
    }

    #[cfg(feature = "restart-authority-evidence")]
    pub fn restart_authority_install_hydrology_continuation_inputs_v3(
        &mut self,
        continuation_inputs: &[Vec<crate::direct_runtime::DirectDayConstructorInputs>],
    ) -> Result<(), DirectV10RealConsumerError> {
        let continuation_day_count = continuation_inputs
            .first()
            .map(Vec::len)
            .filter(|count| *count > 0)
            .ok_or_else(|| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "restart V3 hydrology continuation input cardinality/topology",
                ))
            })?;
        if continuation_inputs.len() != self.inner.hydrology_frame.lanes.len()
            || continuation_inputs
                .iter()
                .any(|lane_inputs| lane_inputs.len() != continuation_day_count)
            || self
                .inner
                .hydrology_frame
                .lanes
                .iter()
                .enumerate()
                .any(|(index, lane)| {
                    u32::try_from(index + 1).map_or(true, |expected| lane.lane_id != expected)
                })
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "restart V3 hydrology continuation input cardinality/topology",
                ),
            ));
        }
        let mut candidate = self.clone();
        candidate.inner.hydrology_frame.identity.day_count = continuation_day_count;
        for (lane, inputs) in candidate
            .inner
            .hydrology_frame
            .lanes
            .iter_mut()
            .zip(continuation_inputs)
        {
            lane.day_inputs.clone_from(inputs);
        }
        if candidate.inner.hydrology_frame.identity.day_count != continuation_day_count
            || candidate
                .inner
                .hydrology_frame
                .lanes
                .iter()
                .zip(continuation_inputs)
                .any(|(lane, expected)| &lane.day_inputs != expected)
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "restart V3 hydrology continuation input installation",
                ),
            ));
        }
        *self = candidate;
        Ok(())
    }

    pub(crate) fn accepted_publication_event_handoffs_for_owner_adjacency(
        &self,
    ) -> &[openwepp_coupled_time::AcceptedEventReceiptV1] {
        self.accepted_publication_history.event_handoffs()
    }

    pub(crate) fn effective_surface_liquid_state_for_zero_duration_receiver(
        &self,
    ) -> Result<crate::DirectSurfaceLiquidOwnedState, DirectV11RealConsumerError> {
        if let Some(parent) = &self.inner.wb14_parent_working_state {
            return parent
                .effective_surface_state(&self.inner.surface_configuration)
                .map_err(|error| {
                    DirectV11RealConsumerError::ZeroDurationSnowLiquid(error.to_string())
                });
        }
        self.inner
            .hydrology_frame
            .surface_liquid_shadow
            .as_deref()
            .cloned()
            .ok_or(DirectV11RealConsumerError::Identity(
                "zero-duration receiver surface owner",
            ))
    }

    pub(crate) const fn physical_lse_state_for_zero_duration_receiver(
        &self,
    ) -> &openwepp_land_surface_energy::LandSurfaceEnergyState {
        &self.inner.lse_state
    }

    pub(crate) fn retains_accepted_publication_event_handoff(
        &self,
        event: &openwepp_coupled_time::AcceptedEventReceiptV1,
    ) -> bool {
        self.accepted_publication_history
            .event_handoffs()
            .iter()
            .any(|retained| retained == event)
    }

    pub(crate) fn accepted_publication_event_handoff_is_tail(
        &self,
        event: &openwepp_coupled_time::AcceptedEventReceiptV1,
    ) -> bool {
        self.accepted_publication_history
            .event_handoffs()
            .last()
            .is_some_and(|retained| retained == event)
    }

    pub(crate) fn validate_accepted_publication_final_handoff(
        &self,
        accepted_subslabs: &[(TimeSupport, Digest32, Digest32, Digest32, Digest32)],
        expected_initial_beginning: Digest32,
        expected_final_ending: Digest32,
    ) -> Result<(), DirectV11RealConsumerError> {
        let required_parent_transactions = accepted_subslabs
            .iter()
            .map(|value| value.2)
            .collect::<std::collections::BTreeSet<_>>();
        let accepted_support_records = self
            .accepted_publication_history
            .supports()
            .iter()
            .filter(|support| {
                required_parent_transactions.contains(&support.parent_transaction_id.digest())
            })
            .collect::<Vec<_>>();
        let accepted_supports = accepted_support_records
            .iter()
            .map(|support| {
                (
                    support.support,
                    support.accepted_slab_sha256,
                    support.parent_transaction_id.digest(),
                    support.beginning_complete_owner_set_sha256,
                    support.ending_complete_owner_set_sha256,
                )
            })
            .collect::<Vec<_>>();
        let initial_beginning = accepted_support_records.first().map(|support| {
            self.accepted_publication_history
                .event_handoffs()
                .first()
                .filter(|event| {
                    event.tick() == support.support.start_ns()
                        && event.parent_transaction_id() == support.parent_transaction_id
                })
                .map_or(
                    support.beginning_complete_owner_set_sha256,
                    openwepp_coupled_time::AcceptedEventReceiptV1::beginning_owner_set_digest,
                )
        });
        if initial_beginning != Some(expected_initial_beginning)
            || !ordered_subsequence_is_complete_v1(&accepted_supports, accepted_subslabs)
            || self
                .accepted_publication_history
                .validate_cached_tail_against_full_scan()?
                != Some(expected_final_ending)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication final handoff chronology",
            ));
        }
        Ok(())
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_wb14_parent_canonical_bytes(
        &self,
    ) -> Result<Option<Vec<u8>>, DirectV10RealConsumerError> {
        self.inner
            .wb14_parent_restart_bytes()
            .map_err(DirectV10RealConsumerError::from)
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_restore_wb14_parent_canonical_bytes(
        &mut self,
        bytes: Option<&[u8]>,
    ) -> Result<(), DirectV10RealConsumerError> {
        let mut candidate = self.clone();
        candidate
            .inner
            .restore_wb14_parent_restart_bytes(bytes)
            .map_err(DirectV10RealConsumerError::from)?;
        if candidate
            .inner
            .wb14_parent_restart_bytes()
            .map_err(DirectV10RealConsumerError::from)?
            .as_deref()
            != bytes
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("noncanonical WB14 parent restart bytes"),
            ));
        }
        *self = candidate;
        Ok(())
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_restore_surface_liquid_canonical_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), DirectV10RealConsumerError> {
        let restored = crate::DirectSurfaceLiquidOwnedState::from_canonical_bytes(
            &self.inner.surface_configuration,
            bytes,
        )
        .map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                "surface-liquid restart owner bytes",
            ))
        })?;
        if restored
            .canonical_bytes(&self.inner.surface_configuration)
            .map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                    "surface-liquid restart canonical bytes",
                ))
            })?
            != bytes
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("noncanonical surface-liquid restart bytes"),
            ));
        }
        let mut candidate = self.clone();
        candidate.restart_authority_validate_v9_complete_owner_set_exact()?;
        candidate.inner.hydrology_frame.surface_liquid_shadow = Some(Box::new(restored));
        candidate.restart_authority_validate_v9_complete_owner_set_exact()?;
        *self = candidate;
        Ok(())
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    fn restart_authority_validate_v9_complete_owner_set_exact(
        &self,
    ) -> Result<(), DirectV10RealConsumerError> {
        let inner = &self.inner;
        inner
            .vegetation_state
            .validate(&inner.vegetation_configuration)
            .map_err(DirectV9RealConsumerError::V9)?;
        inner.lse_state.validate(&inner.lse_configuration)?;
        let soil_thermal = inner.soil_thermal.v1()?;
        soil_thermal.validate()?;
        let transaction_id = TransactionId(inner.vegetation_state.0.last_transaction_id);
        if inner
            .lse_state
            .last_accepted_transaction_id
            .is_some_and(|value| value != transaction_id)
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("restart V9 vegetation/LSE lineage"),
            ));
        }
        if soil_thermal
            .last_accepted_transaction_id
            .is_some_and(|value| value != transaction_id)
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("restart V9 vegetation/soil lineage"),
            ));
        }
        if inner.biogeochemistry.last_transaction_id != inner.vegetation_state.0.last_transaction_id
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("restart V9 vegetation/BGC lineage"),
            ));
        }
        if inner.accepted_interval_count != 0
            && (inner.lse_state.last_accepted_transaction_id != Some(transaction_id)
                || soil_thermal.last_accepted_transaction_id != Some(transaction_id))
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("restart V9 accepted physical lineage"),
            ));
        }
        if inner.surface_configuration.ofe_bindings.len() != inner.hydrology_frame.lanes.len() {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("restart V9 surface/lane cardinality"),
            ));
        }
        if inner.layer_maps.len() != inner.hydrology_frame.lanes.len()
            || !inner
                .surface_configuration
                .ofe_bindings
                .iter()
                .zip(&inner.layer_maps)
                .all(|(binding, map)| {
                    binding.production_lane_index == map.ofe_lane.lane_index
                        && binding.production_lane_id == map.ofe_lane.lane_id
                        && binding.ordered_soil_layer_ids == map.layer_ids
                })
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("restart V9 surface/layer mapping"),
            ));
        }
        Ok(())
    }

    /// Restore the exact logical vegetation/BGC owner advancement retained by
    /// a V11 parent-finalization event after the positive-support physical
    /// owners have already accepted. The target may differ from the admitted
    /// predecessor only by one transaction and the derived vegetation seal.
    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_restore_parent_finalization_logical_owners(
        &mut self,
        finalized_vegetation: V10CoupledOwnedState,
        finalized_biogeochemistry: BiogeochemistryState,
    ) -> Result<(), DirectV10RealConsumerError> {
        finalized_vegetation
            .validate(&self.vegetation_configuration)
            .map_err(DirectV10RealConsumerError::V10)?;
        let predecessor_transaction = self.vegetation_state.0.last_transaction_id;
        let finalized_transaction = predecessor_transaction.checked_add(1).ok_or_else(|| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                "restart parent-finalization transaction overflow",
            ))
        })?;
        if finalized_vegetation.0.last_transaction_id != finalized_transaction
            || self.inner.biogeochemistry.last_transaction_id != predecessor_transaction
            || finalized_biogeochemistry.last_transaction_id != finalized_transaction
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "restart parent-finalization logical owner lineage",
                ),
            ));
        }
        let mut normalized_vegetation = finalized_vegetation.clone();
        normalized_vegetation.0.last_transaction_id = predecessor_transaction;
        normalized_vegetation.0.state_sha256 = normalized_vegetation.0.canonical_sha256();
        let mut normalized_biogeochemistry = finalized_biogeochemistry.clone();
        normalized_biogeochemistry.last_transaction_id = predecessor_transaction;
        if normalized_vegetation != self.vegetation_state
            || normalized_biogeochemistry != self.inner.biogeochemistry
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "restart parent-finalization constitutive mutation",
                ),
            ));
        }
        let (v9_configuration, v9_state) =
            project_v10_runtime_to_v9(&self.vegetation_configuration, &finalized_vegetation)
                .map_err(DirectV10RealConsumerError::V10)?;
        if v9_configuration != self.inner.vegetation_configuration {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity("restart parent-finalization V9 configuration"),
            ));
        }
        let mut candidate = self.clone();
        candidate.vegetation_state = finalized_vegetation;
        candidate.inner.vegetation_state = v9_state;
        candidate.inner.biogeochemistry = finalized_biogeochemistry;
        *self = candidate;
        Ok(())
    }

    #[must_use]
    pub const fn vegetation_state(&self) -> &V10CoupledOwnedState {
        &self.vegetation_state
    }

    #[must_use]
    pub const fn vegetation_configuration(&self) -> &VegetationConfiguration {
        &self.vegetation_configuration
    }

    #[must_use]
    pub const fn lse_state(&self) -> &LandSurfaceEnergyV2State {
        &self.lse_state
    }

    #[must_use]
    pub const fn lse_configuration(&self) -> &LandSurfaceEnergyConfiguration {
        &self.lse_configuration
    }

    #[must_use]
    pub const fn gsi_parameters(&self) -> GsiParameters {
        self.gsi_owner_configuration.parameters()
    }

    #[must_use]
    pub const fn gsi_owner_configuration(&self) -> &DirectGsiOwnerConfigurationV1 {
        &self.gsi_owner_configuration
    }

    #[must_use]
    pub const fn root_zone_hydraulic_configuration(&self) -> &DirectRootZoneHydraulicConfiguration {
        &self.root_zone_hydraulic_configuration
    }

    #[must_use]
    pub const fn gsi_state(&self) -> &GsiState {
        &self.gsi_state
    }

    #[must_use]
    pub const fn provider_cursor(&self) -> &SnowFreeHalfHourProviderCursor {
        &self.provider_cursor
    }

    #[must_use]
    pub const fn provider_static_configuration(&self) -> &SnowFreeHalfHourStaticConfiguration {
        &self.provider_static_configuration
    }

    pub(crate) fn accept_zero_duration_terminal_receiver(
        &self,
        parcels: &[DirectSnowStage3V11TerminalParcel],
        output_set_sha256: Digest32,
        predecessor_owner_set_sha256: Digest32,
        receiver_context_sha256: Digest32,
        receiver_ordinal: u32,
    ) -> Result<
        (
            Self,
            Vec<u8>,
            Vec<u8>,
            [u8; 32],
            Vec<crate::direct_runtime::DirectZeroDurationSnowLiquidReceiptV1>,
        ),
        DirectV11RealConsumerError,
    > {
        if parcels.is_empty()
            || parcels.iter().any(|parcel| {
                parcel.posture != DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed
            })
        {
            return Err(DirectV11RealConsumerError::Identity(
                "zero-duration terminal receiver parcel posture",
            ));
        }
        if output_set_sha256 == Digest32::zero()
            || predecessor_owner_set_sha256 == Digest32::zero()
            || receiver_context_sha256 == Digest32::zero()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "zero-duration terminal receiver binding",
            ));
        }
        let inputs =
            parcels
                .iter()
                .flat_map(|parcel| {
                    parcel.receiver_destinations.iter().map(move |destination| {
                        Ok(crate::direct_runtime::DirectZeroDurationSnowLiquidInputV1 {
                            output_receipt_sha256: *parcel.parcel_digest.as_bytes(),
                            output_set_sha256: *output_set_sha256.as_bytes(),
                            predecessor_owner_set_sha256: *predecessor_owner_set_sha256.as_bytes(),
                            receiver_context_sha256: *receiver_context_sha256.as_bytes(),
                            support_start_ns: parcel.support.start_ns().get(),
                            support_end_ns: parcel.support.end_ns().get(),
                            receiver_ordinal,
                            ofe_id: OfeId::try_new(destination.destination_ofe_id.clone())
                                .map_err(|_| {
                                    DirectV11RealConsumerError::Identity(
                                        "zero-duration terminal receiver OFE",
                                    )
                                })?,
                            tile_id: TileId::try_new(destination.destination_tile_id.clone())
                                .map_err(|_| {
                                    DirectV11RealConsumerError::Identity(
                                        "zero-duration terminal receiver tile",
                                    )
                                })?,
                            tile_fraction: destination.destination_fraction,
                            mass_kg_m2_tile_ground: parcel.mass_kg_m2_tile_ground,
                            sensible_enthalpy_j_m2_tile_ground: parcel.mass_kg_m2_tile_ground
                                * parcel.specific_liquid_enthalpy_j_kg,
                        })
                    })
                })
                .collect::<Result<Vec<_>, DirectV11RealConsumerError>>()?;
        let mut candidate = self.clone();
        let beginning = candidate
            .inner
            .hydrology_frame
            .surface_liquid_shadow
            .as_deref()
            .ok_or(DirectV11RealConsumerError::Identity(
                "zero-duration terminal receiver surface owner",
            ))?;
        let outcome = beginning
            .accept_zero_duration_snow_liquid_outputs_v1(
                &candidate.inner.surface_configuration,
                &inputs,
                true,
            )
            .map_err(|error| {
                DirectV11RealConsumerError::ZeroDurationSnowLiquid(error.to_string())
            })?;
        let surface_bytes = outcome
            .ending_state
            .canonical_bytes(&candidate.inner.surface_configuration)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "zero-duration terminal receiver surface bytes",
                )
            })?;
        let mut ending_physical_lse = candidate.inner.lse_state.clone();
        let mut ending_lse = candidate.lse_state.clone();
        for credit in &outcome.retained_enthalpy_credits {
            let matching_v2 = ending_lse
                .0
                .tiles
                .iter_mut()
                .filter(|tile| tile.ofe_id == credit.ofe_id && tile.tile_id == credit.tile_id)
                .collect::<Vec<_>>();
            let matching_physical = ending_physical_lse
                .tiles
                .iter_mut()
                .filter(|tile| tile.ofe_id == credit.ofe_id && tile.tile_id == credit.tile_id)
                .collect::<Vec<_>>();
            if matching_v2.len() != 1 || matching_physical.len() != 1 {
                return Err(DirectV11RealConsumerError::Identity(
                    "zero-duration terminal receiver LSE topology",
                ));
            }
            let tile_v2 =
                matching_v2
                    .into_iter()
                    .next()
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "zero-duration terminal receiver LSE tile",
                    ))?;
            let tile_physical = matching_physical.into_iter().next().ok_or(
                DirectV11RealConsumerError::Identity(
                    "zero-duration terminal receiver physical LSE tile",
                ),
            )?;
            tile_v2.surface_enthalpy_j_m2_tile_ground += credit.enthalpy_j_m2_tile_ground;
            tile_physical.surface_enthalpy_j_m2_tile_ground += credit.enthalpy_j_m2_tile_ground;
            if !tile_v2.surface_enthalpy_j_m2_tile_ground.is_finite()
                || !tile_physical.surface_enthalpy_j_m2_tile_ground.is_finite()
                || tile_v2.surface_enthalpy_j_m2_tile_ground.to_bits()
                    != tile_physical.surface_enthalpy_j_m2_tile_ground.to_bits()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "zero-duration terminal receiver LSE enthalpy",
                ));
            }
        }
        ending_lse.0.state_sha256 = ending_lse.0.canonical_sha256().map_err(|_| {
            DirectV11RealConsumerError::Identity("zero-duration terminal receiver LSE seal")
        })?;
        ending_physical_lse.state_sha256 =
            ending_physical_lse.canonical_sha256().map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "zero-duration terminal receiver physical LSE seal",
                )
            })?;
        ending_physical_lse
            .validate(&candidate.inner.lse_configuration)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "zero-duration terminal receiver physical LSE state",
                )
            })?;
        ending_lse
            .validate(&candidate.lse_configuration)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("zero-duration terminal receiver LSE state")
            })?;
        let lse_bytes = serde_json::to_vec(&ending_physical_lse).map_err(|_| {
            DirectV11RealConsumerError::Identity("zero-duration terminal receiver LSE bytes")
        })?;
        candidate.inner.hydrology_frame.surface_liquid_shadow =
            Some(Box::new(outcome.ending_state));
        candidate.lse_state = ending_lse;
        candidate.inner.lse_state = ending_physical_lse;
        Ok((
            candidate,
            surface_bytes,
            lse_bytes,
            outcome.receipt_set_sha256,
            outcome.receipts,
        ))
    }

    pub(crate) fn accept_zero_duration_stage3_support_liquid_receiver(
        &self,
        outputs: &[Stage3AcceptedSnowLiquidOutputV1],
        output_set_sha256: Digest32,
        predecessor_owner_set_sha256: Digest32,
        receiver_context_sha256: Digest32,
        receiver_ordinal: u32,
    ) -> Result<
        (
            Self,
            Vec<u8>,
            Vec<u8>,
            [u8; 32],
            Vec<crate::direct_runtime::DirectZeroDurationSnowLiquidReceiptV1>,
        ),
        DirectV11RealConsumerError,
    > {
        if outputs.is_empty() {
            return Err(DirectV11RealConsumerError::Identity(
                "zero-duration support-liquid receiver output set",
            ));
        }
        let output_receipts = outputs
            .iter()
            .map(Stage3AcceptedSnowLiquidOutputV1::receipt_sha256)
            .collect::<Vec<_>>();
        let output_fields = output_receipts
            .iter()
            .map(|receipt| openwepp_coupled_time::FramedField {
                tag: "snow_liquid_output",
                value: receipt.as_bytes(),
            })
            .collect::<Vec<_>>();
        let reconstructed_output_set = openwepp_coupled_time::framed_sha256(
            "stage3-v11-positive-support-liquid-output-set",
            &output_fields,
        )
        .map_err(|_| {
            DirectV11RealConsumerError::Identity(
                "zero-duration support-liquid receiver output-set seal",
            )
        })?;
        if output_set_sha256 != reconstructed_output_set
            || predecessor_owner_set_sha256 == Digest32::zero()
            || receiver_context_sha256 == Digest32::zero()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "zero-duration support-liquid receiver binding",
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        let mut inputs = Vec::new();
        for output in outputs {
            output.validate()?;
            if output.mass_kg_m2_ofe_ground() <= 0.0 || !seen.insert(output.receipt_sha256()) {
                return Err(DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver output identity",
                ));
            }
            if output.destinations.is_empty() {
                return Err(DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver topology",
                ));
            }
            for destination in &output.destinations {
                inputs.push(crate::direct_runtime::DirectZeroDurationSnowLiquidInputV1 {
                    output_receipt_sha256: *output.receipt_sha256().as_bytes(),
                    output_set_sha256: *output_set_sha256.as_bytes(),
                    predecessor_owner_set_sha256: *predecessor_owner_set_sha256.as_bytes(),
                    receiver_context_sha256: *receiver_context_sha256.as_bytes(),
                    support_start_ns: output.support.start_ns().get(),
                    support_end_ns: output.support.end_ns().get(),
                    receiver_ordinal,
                    ofe_id: destination.ofe_id.clone(),
                    tile_id: destination.tile_id.clone(),
                    tile_fraction: destination.tile_fraction,
                    mass_kg_m2_tile_ground: destination.mass_kg_m2_tile_ground,
                    sensible_enthalpy_j_m2_tile_ground: destination
                        .sensible_enthalpy_j_m2_tile_ground,
                });
            }
        }
        let mut candidate = self.clone();
        let parent_working = candidate.inner.wb14_parent_working_state.clone();
        let beginning = if let Some(parent) = &parent_working {
            parent.candidate_state()
        } else {
            candidate
                .inner
                .hydrology_frame
                .surface_liquid_shadow
                .as_deref()
                .ok_or(DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver surface owner",
                ))?
        };
        let outcome = beginning
            .accept_zero_duration_snow_liquid_outputs_v1(
                &candidate.inner.surface_configuration,
                &inputs,
                parent_working.is_none(),
            )
            .map_err(|error| {
                DirectV11RealConsumerError::ZeroDurationSnowLiquid(error.to_string())
            })?;
        let surface_bytes = outcome
            .ending_state
            .canonical_bytes(&candidate.inner.surface_configuration)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver surface bytes",
                )
            })?;
        let mut ending_physical_lse = candidate.inner.lse_state.clone();
        let mut ending_lse = candidate.lse_state.clone();
        for credit in &outcome.retained_enthalpy_credits {
            let matching_v2 = ending_lse
                .0
                .tiles
                .iter_mut()
                .filter(|tile| tile.ofe_id == credit.ofe_id && tile.tile_id == credit.tile_id)
                .collect::<Vec<_>>();
            let matching_physical = ending_physical_lse
                .tiles
                .iter_mut()
                .filter(|tile| tile.ofe_id == credit.ofe_id && tile.tile_id == credit.tile_id)
                .collect::<Vec<_>>();
            if matching_v2.len() != 1 || matching_physical.len() != 1 {
                return Err(DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver LSE topology",
                ));
            }
            let tile_v2 =
                matching_v2
                    .into_iter()
                    .next()
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "zero-duration support-liquid receiver LSE tile",
                    ))?;
            let tile_physical = matching_physical.into_iter().next().ok_or(
                DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver physical LSE tile",
                ),
            )?;
            tile_v2.surface_enthalpy_j_m2_tile_ground += credit.enthalpy_j_m2_tile_ground;
            tile_physical.surface_enthalpy_j_m2_tile_ground += credit.enthalpy_j_m2_tile_ground;
            if !tile_v2.surface_enthalpy_j_m2_tile_ground.is_finite()
                || !tile_physical.surface_enthalpy_j_m2_tile_ground.is_finite()
                || tile_v2.surface_enthalpy_j_m2_tile_ground.to_bits()
                    != tile_physical.surface_enthalpy_j_m2_tile_ground.to_bits()
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver LSE enthalpy",
                ));
            }
        }
        // A zero-duration ownership transfer is sequenced by its coupled-time
        // event ordinal and typed receiver receipt chain. It does not consume
        // a positive-duration LSE transaction ordinal.
        ending_lse.0.state_sha256 = ending_lse.0.canonical_sha256().map_err(|_| {
            DirectV11RealConsumerError::Identity("zero-duration support-liquid receiver LSE seal")
        })?;
        ending_physical_lse.state_sha256 =
            ending_physical_lse.canonical_sha256().map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver physical LSE seal",
                )
            })?;
        ending_physical_lse
            .validate(&candidate.inner.lse_configuration)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver physical LSE state",
                )
            })?;
        ending_lse
            .validate(&candidate.lse_configuration)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "zero-duration support-liquid receiver LSE state",
                )
            })?;
        let lse_bytes = serde_json::to_vec(&ending_physical_lse).map_err(|_| {
            DirectV11RealConsumerError::Identity("zero-duration support-liquid receiver LSE bytes")
        })?;
        if let Some(parent) = parent_working {
            candidate.inner.wb14_parent_working_state = Some(
                parent
                    .with_zero_duration_receiver_candidate(
                        &candidate.inner.surface_configuration,
                        outcome.ending_state,
                    )
                    .map_err(|error| {
                        DirectV11RealConsumerError::ZeroDurationSnowLiquid(error.to_string())
                    })?,
            );
        } else {
            candidate.inner.hydrology_frame.surface_liquid_shadow =
                Some(Box::new(outcome.ending_state));
        }
        candidate.lse_state = ending_lse;
        candidate.inner.lse_state = ending_physical_lse;
        Ok((
            candidate,
            surface_bytes,
            lse_bytes,
            outcome.receipt_set_sha256,
            outcome.receipts,
        ))
    }

    pub(crate) fn accepted_snow_liquid_outputs_for_support(
        &self,
        support: TimeSupport,
    ) -> Result<Vec<Stage3AcceptedSnowLiquidOutputV1>, DirectV11RealConsumerError> {
        let retained = self.accepted_publication_history.supports().last().ok_or(
            DirectV11RealConsumerError::Identity("support-liquid receiver publication support"),
        )?;
        if retained.support() != support {
            return Err(DirectV11RealConsumerError::Identity(
                "support-liquid receiver publication chronology",
            ));
        }
        let outputs = retained
            .accepted_snow_liquid_outputs()
            .iter()
            .filter(|output| output.mass_kg_m2_ofe_ground() > 0.0)
            .cloned()
            .collect::<Vec<_>>();
        for output in &outputs {
            output.validate()?;
        }
        Ok(outputs)
    }

    pub(crate) fn retain_accepted_publication_zero_duration_event(
        &mut self,
        event: &openwepp_coupled_time::AcceptedEventReceiptV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        event.validate().map_err(|_| {
            DirectV11RealConsumerError::Identity("zero-duration event publication seal")
        })?;
        let support = self.accepted_publication_history.supports().last().ok_or(
            DirectV11RealConsumerError::Identity(
                "zero-duration event publication predecessor support",
            ),
        )?;
        if support.support.end_ns() != event.tick() {
            return Err(DirectV11RealConsumerError::Identity(
                "zero-duration event publication predecessor join",
            ));
        }
        self.accepted_publication_history
            .push_event_handoff(event.clone())
    }

    pub(crate) fn retain_accepted_publication_zero_duration_event_for_following_support(
        &mut self,
        event: &openwepp_coupled_time::AcceptedEventReceiptV1,
        expected_genesis_beginning_owner: Digest32,
        following_support: TimeSupport,
    ) -> Result<(), DirectV11RealConsumerError> {
        if event.tick() != following_support.start_ns() {
            return Err(DirectV11RealConsumerError::Identity(
                "pre-support event/following support chronology",
            ));
        }
        if !self.accepted_publication_history.supports().is_empty() {
            return self.retain_accepted_publication_zero_duration_event(event);
        }
        event.validate().map_err(|_| {
            DirectV11RealConsumerError::Identity("genesis zero-duration event publication seal")
        })?;
        if !self
            .accepted_publication_history
            .event_handoffs()
            .is_empty()
            || !genesis_pre_support_event_authority_is_exact_v1(
                event.tick(),
                event.ordinal(),
                event.beginning_owner_set_digest(),
                expected_genesis_beginning_owner,
                following_support,
            )
        {
            return Err(DirectV11RealConsumerError::Identity(
                "genesis zero-duration event publication authority",
            ));
        }
        self.accepted_publication_history
            .push_event_handoff(event.clone())
    }

    pub(crate) fn next_lse_transaction_id(
        &self,
    ) -> Result<TransactionId, DirectV10RealConsumerError> {
        Ok(TransactionId(
            self.inner
                .vegetation_state
                .0
                .last_transaction_id
                .checked_add(1)
                .ok_or_else(|| {
                    DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
                        "next LSE transaction overflow",
                    ))
                })?,
        ))
    }

    #[must_use]
    pub const fn v11_next_day_index(&self) -> usize {
        self.inner.next_day_index()
    }

    #[must_use]
    pub const fn hydrology_frame(&self) -> &DirectRunFrame {
        self.inner.hydrology_frame()
    }

    #[must_use]
    pub(crate) fn qualification_soil_thermal(&self) -> DirectSoilThermalReadView<'_> {
        self.inner.soil_thermal.read_view()
    }

    #[must_use]
    pub(crate) const fn qualification_biogeochemistry(
        &self,
    ) -> &openwepp_biogeochemistry::BiogeochemistryState {
        self.inner.biogeochemistry()
    }

    /// Install the provider/GSI owner transition only on a cloned candidate
    /// after all coupled Stage-3/V11 supports have accepted. This keeps the
    /// runner cursor out of the live state on any failed support.
    pub(crate) fn commit_prepared_provider_day(
        &mut self,
        prepared: PreparedSnowFreeGsiDayV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        let accepted_receipt = prepared.gsi_receipt().receipt_sha256.clone();
        let next_day_index = self.inner.next_day_index.checked_add(1).ok_or(
            DirectV11RealConsumerError::Identity("prepared provider day index overflow"),
        )?;
        let mut ending_gsi_state = self.gsi_state.clone();
        let mut ending_provider_cursor = self.provider_cursor.clone();
        prepared
            .commit(&mut ending_gsi_state, &mut ending_provider_cursor)
            .map_err(|error| {
                DirectV11RealConsumerError::Runtime(DirectV10RealConsumerError::ForcingProvider(
                    error,
                ))
            })?;
        ending_provider_cursor
            .validate_for_configuration(&self.provider_static_configuration, next_day_index)
            .map_err(DirectV10RealConsumerError::from)?;
        self.gsi_state = ending_gsi_state;
        self.provider_cursor = ending_provider_cursor;
        self.inner.next_day_index = next_day_index;
        self.inner.provider_gsi_receipt_sha256 = accepted_receipt;
        Ok(())
    }
}

#[cfg(test)]
mod accepted_publication_subsequence_tests {
    use super::{
        genesis_pre_support_event_authority_is_exact_v1, ordered_subsequence_is_complete_v1,
    };
    use openwepp_coupled_time::{Digest32, ModelTimeNs, TimeSupport};

    #[test]
    fn covered_subslabs_may_begin_late_but_owner_substitution_rejects() {
        let accepted = [(0_u8, 1_u8, 2_u8), (1, 2, 3), (2, 4, 5)];
        assert!(ordered_subsequence_is_complete_v1(
            &accepted,
            &accepted[2..],
        ));
        assert!(
            !ordered_subsequence_is_complete_v1(&accepted, &[(2, 3, 5)]),
            "late covered subslab stale beginning owner",
        );
        assert!(
            !ordered_subsequence_is_complete_v1(&accepted, &[(2, 4, 6)]),
            "late covered subslab substituted ending owner",
        );
        assert!(
            !ordered_subsequence_is_complete_v1(&accepted, &[(2, 4, 5), (1, 2, 3)]),
            "covered subslab order substitution",
        );
    }

    #[test]
    fn genesis_event_requires_exact_committed_owner_tick_and_zero_ordinal() {
        let support = TimeSupport::new(ModelTimeNs::new(10), ModelTimeNs::new(20))
            .expect("following support");
        let beginning = Digest32::from_bytes([1; 32]);
        assert!(genesis_pre_support_event_authority_is_exact_v1(
            ModelTimeNs::new(10),
            0,
            beginning,
            beginning,
            support,
        ));
        assert!(!genesis_pre_support_event_authority_is_exact_v1(
            ModelTimeNs::new(11),
            0,
            beginning,
            beginning,
            support,
        ));
        assert!(!genesis_pre_support_event_authority_is_exact_v1(
            ModelTimeNs::new(10),
            1,
            beginning,
            beginning,
            support,
        ));
        assert!(!genesis_pre_support_event_authority_is_exact_v1(
            ModelTimeNs::new(10),
            0,
            Digest32::from_bytes([2; 32]),
            beginning,
            support,
        ));
    }
}
